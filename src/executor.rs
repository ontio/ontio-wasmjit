use crate::chain_api::ChainCtx;
use crate::resolver::{ChainResolver, Resolver};
use crate::trampoline::make_trampoline;
use crate::{disassm, linker, utils};

use cranelift_codegen::ir;
use cranelift_codegen::isa;
use cranelift_codegen::settings;
use cranelift_codegen::settings::Configurable;
use cranelift_entity::PrimaryMap;
use cranelift_wasm::{DefinedFuncIndex, DefinedMemoryIndex};
use ontio_wasmjit_environ::{
    compile_module, CompileError, Module as ModuleInfo, ModuleEnvironment, OwnedDataInitializer,
    Relocations, Traps, Tunables,
};
use ontio_wasmjit_runtime::{
    get_mut_trap_registry, wasmjit_call, wasmjit_call_trampoline, InstanceHandle,
    TrapRegistrationGuard, VMFunctionBody,
};

use dynasmrt::mmap::MutableBuffer;
use dynasmrt::ExecutableBuffer;
use lru::LruCache;
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use std::convert::TryFrom;
use std::{mem, sync::Arc, usize};
use target_lexicon::PointerWidth;

pub trait FuncParam {
    fn into_i64(self) -> i64;
}

impl FuncParam for i32 {
    fn into_i64(self) -> i64 {
        self as i64
    }
}

impl FuncParam for i64 {
    fn into_i64(self) -> i64 {
        self as i64
    }
}
impl FuncParam for u32 {
    fn into_i64(self) -> i64 {
        self as i64
    }
}
impl FuncParam for u64 {
    fn into_i64(self) -> i64 {
        self as i64
    }
}

pub trait FuncArgs {
    fn args_vector(self) -> Vec<i64>;
}

macro_rules! for_each_tuple_ {
    ($m:ident !!) => {
        $m! { }
    };
    ($m:ident !! $h:ident, $($t:ident,)*) => {
        $m! { $h $($t)* }
        for_each_tuple_! { $m !! $($t,)* }
    }
}
macro_rules! for_each_tuple {
    ($($m:tt)*) => {
        macro_rules! m { $($m)* }
        for_each_tuple_! { m !! A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, }
    }
}

//trace_macros!(true);
for_each_tuple! {
    ($($item:ident)*) => {
        impl<$($item: FuncParam),*> FuncArgs for ($($item,)*) {
            fn args_vector(self) -> Vec<i64> {
                #[allow(unused_mut)]
                let mut buf = Vec::new();
                #[allow(non_snake_case)]
                let ($($item,)*) = self;
                $(buf.push($item.into_i64());)*
                buf
            }
        }
    }
}

static MODULE_CACHE: Lazy<Mutex<LruCache<[u8; 20], Arc<Module>>>> =
    Lazy::new(|| Mutex::new(LruCache::new(20)));

pub struct Instance {
    module: Arc<Module>,
    handle: InstanceHandle,
}

impl Instance {
    fn invoke(&mut self) {
        if let Some(memory) = self
            .handle
            .instance_mut()
            .memory_slice_mut(DefinedMemoryIndex::from_u32(0))
        {
            for i in 0..100 {
                memory[4 * i..4 * (i + 1)].copy_from_slice(&(i as u32).to_le_bytes());
            }
        }

        let invoke = self
            .handle
            .lookup("invoke")
            .expect(&format!("can not find export function:{}", "invoke"));

        unsafe {
            if let Err(err) = wasmjit_call(invoke.vmctx, invoke.address) {
                println!("execute paniced: {}", err);
            }
        }
    }
}

/// Compiled module for instantiate
pub struct Module {
    info: Arc<ModuleInfo>,
    data_initializers: Vec<OwnedDataInitializer>,
    func_offsets: PrimaryMap<DefinedFuncIndex, usize>,
    jt_offsets: PrimaryMap<DefinedFuncIndex, ir::JumpTableOffsets>,
    executable: ExecutableBuffer,
    relocs: Relocations,
    traps: Traps,
    trap_registration_guards: Vec<TrapRegistrationGuard>,
}

impl Module {
    fn instantiate(self: Arc<Self>, chain: ChainCtx) -> Instance {
        let module_info = self.info.clone();
        let imports = {
            let mut resolver = ChainResolver;
            let mut imports = PrimaryMap::new();
            for (module, func) in module_info.imported_funcs.values() {
                imports.push(
                    resolver
                        .resolve(module, func)
                        .expect("can not resolve import func"),
                );
            }
            imports.into_boxed_slice()
        };

        let data_initializers: Vec<_> = self.data_initializers.iter().map(|e| e.into()).collect();
        let functions: PrimaryMap<_, _> = self
            .func_offsets
            .iter()
            .map(|(_index, offset)| &self.executable[*offset] as *const u8 as *const VMFunctionBody)
            .collect();

        let mut instance = InstanceHandle::new(
            self.info.clone(),
            functions.into_boxed_slice(),
            imports,
            &data_initializers,
            chain.gas_left.clone(),
            Box::new(chain),
        )
        .unwrap();

        if let Some(memory) = instance
            .instance_mut()
            .memory_slice_mut(DefinedMemoryIndex::from_u32(0))
        {
            for i in 0..100 {
                memory[4 * i..4 * (i + 1)].copy_from_slice(&(i as u32).to_le_bytes());
            }
        }

        Instance {
            module: self.clone(),
            handle: instance,
        }
    }

    fn compile(wasm: &[u8]) -> Result<Module, CompileError> {
        let config = isa::TargetFrontendConfig {
            default_call_conv: isa::CallConv::SystemV,
            pointer_width: PointerWidth::U64,
        };

        let isa_builder = isa::lookup_by_name("x86_64").unwrap();
        let mut flag_builder = settings::builder();
        let _ = flag_builder.set("probestack_enabled", "false");
        let isa = isa_builder.finish(settings::Flags::new(flag_builder));

        let module_environ = ModuleEnvironment::new(config, Tunables::default());
        let result = module_environ.translate(&wasm).unwrap();

        let (compilation, relocs, _address_transform, _value_ranges, _stack_slots, traps) =
            compile_module(
                &result.module,
                &result.translate_state,
                result.function_body_inputs,
                &*isa,
                false,
            )?;

        let total_code_size = (&compilation).into_iter().map(|code| code.body.len()).sum();
        let mut exec = MutableBuffer::new(total_code_size).unwrap();
        let mut finished_functions = PrimaryMap::new();
        let mut func_offsets = PrimaryMap::with_capacity(compilation.len());
        for code in &compilation {
            let curr_size = exec.len();
            exec.set_len(curr_size + code.body.len());
            exec[curr_size..].copy_from_slice(&code.body);
            func_offsets.push(curr_size);
            finished_functions.push(&exec[curr_size] as *const u8 as *const VMFunctionBody);
        }

        let jt_offsets = compilation.get_jt_offsets();

        linker::link_module(&result.module, &finished_functions, &jt_offsets, &relocs);

        let mut trap_registration_guards = vec![];
        register_traps(&finished_functions, &traps, &mut trap_registration_guards);

        let executable = exec.make_exec().unwrap();

        Ok(Module {
            info: Arc::new(result.module),
            data_initializers: result
                .data_initializers
                .into_iter()
                .map(|e| e.into_owned())
                .collect(),
            func_offsets,
            executable,
            jt_offsets,
            relocs,
            traps,
            trap_registration_guards,
        })
    }

    fn dump(&self) {
        //        println!("compilation result");
        //        for code in &self.compilation {
        //            disassm::print_disassembly(&code.body);
        //        }

        println!("relocations result");
        for (func, reloc) in self.relocs.iter() {
            println!("reloc for func {:?}", func);
            for (i, rel) in reloc.iter().enumerate() {
                println!("reloc:{} for func {:?}", i, rel);
            }
        }
    }
}

impl Drop for Module {
    fn drop(&mut self) {
        // We must deregister traps before freeing the executable memory.
        self.trap_registration_guards.clear();
    }
}

fn register_traps(
    allocated_functions: &PrimaryMap<DefinedFuncIndex, *const VMFunctionBody>,
    traps: &Traps,
    trap_registration_guards: &mut Vec<TrapRegistrationGuard>,
) {
    let mut trap_registry = get_mut_trap_registry();
    for (func_addr, func_traps) in allocated_functions.values().zip(traps.values()) {
        for trap_desc in func_traps.iter() {
            let func_addr = *func_addr as *const u8 as usize;
            let offset = usize::try_from(trap_desc.code_offset).unwrap();
            let trap_addr = func_addr + offset;
            let guard =
                trap_registry.register_trap(trap_addr, trap_desc.source_loc, trap_desc.trap_code);
            trap_registration_guards.push(guard);
        }
    }
}

/// Simple executor that assert the wasm file has an export function `invoke(a:i32, b:32)-> i32`.
pub fn execute<Args: FuncArgs>(
    wat: &str,
    func: &str,
    args: Args,
    verbose: bool,
    chain: ChainCtx,
) -> Option<i64> {
    let wasm = wast::parse_str(wat).unwrap();
    let address = utils::contract_address(&wasm);
    let module = MODULE_CACHE.lock().get(&address).cloned();

    let module = module.unwrap_or_else(|| {
        let module = Module::compile(&wasm).unwrap();
        let module = Arc::new(module);
        let mut cache = MODULE_CACHE.lock();
        if !cache.contains(&address) {
            cache.put(address, module.clone());
        }

        module
    });

    if verbose {
        module.dump();
    }

    let mut instance = module.instantiate(chain);
    let invoke = instance
        .handle
        .lookup(func)
        .expect(&format!("can not find export function:{}", func));

    let isa_builder = isa::lookup_by_name("x86_64").unwrap();
    let mut flag_builder = settings::builder();
    let _ = flag_builder.set("probestack_enabled", "false");
    let isa = isa_builder.finish(settings::Flags::new(flag_builder));
    let func = make_trampoline(
        &*isa,
        invoke.address,
        &invoke.signature,
        mem::size_of::<u64>(),
    )
    .unwrap();
    let mut trampoline = MutableBuffer::new(func.len()).unwrap();
    trampoline.set_len(func.len());
    trampoline.copy_from_slice(&func);
    let tranpoline = trampoline.make_exec().unwrap();

    let address = &tranpoline[0] as *const u8 as *const VMFunctionBody;
    let mut args_vec = args.args_vector();
    args_vec.push(0); // place holder for return value
    unsafe {
        if let Err(err) =
            wasmjit_call_trampoline(invoke.vmctx, address, args_vec.as_mut_ptr() as *mut u8)
        {
            println!("execute paniced: {}", err);
            return None;
        }
    }
    if invoke.signature.returns.len() == 0 {
        return None;
    } else {
        if invoke.signature.returns[0].value_type == ir::types::I32 {
            return Some(args_vec[0] as i32 as i64);
        }
        return Some(args_vec[0] as i64);
    }
}

/// Simple executor that assert the wasm file has an export function `invoke(a:i32, b:32)-> i32`.
pub fn call_invoke(wat: &str, verbose: bool, chain: ChainCtx) {
    let wasm = wast::parse_str(wat).unwrap();
    let address = utils::contract_address(&wasm);
    let module = MODULE_CACHE.lock().get(&address).cloned();

    let module = module.unwrap_or_else(|| {
        let module = Module::compile(&wasm).unwrap();
        let module = Arc::new(module);
        let mut cache = MODULE_CACHE.lock();
        if !cache.contains(&address) {
            cache.put(address, module.clone());
        }

        module
    });

    if verbose {
        module.dump();
    }

    let mut instance = module.instantiate(chain);
    instance.invoke();
}
