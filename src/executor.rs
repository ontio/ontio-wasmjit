use cranelift_codegen::isa;
use cranelift_codegen::settings;
use cranelift_codegen::settings::Configurable;
use target_lexicon::PointerWidth;

use ontio_wasmjit_environ::compile_module;
use ontio_wasmjit_environ::ModuleEnvironment;
use ontio_wasmjit_environ::Tunables;

use crate::chain_api::ChainCtx;
use crate::disassm;
use crate::resolver::{ChainResolver, Resolver};
use cranelift_wasm::DefinedMemoryIndex;
use dynasmrt::mmap::MutableBuffer;
use std::mem;

use crate::linker;
use cranelift_entity::PrimaryMap;
use ontio_wasmjit_runtime::{wasmjit_call, InstanceHandle, VMContext, VMFunctionBody};

pub trait FuncParam {}

impl FuncParam for i32 {}
impl FuncParam for i64 {}
impl FuncParam for u32 {}
impl FuncParam for u64 {}

pub trait FuncArgs<Output> {
    type FuncType;
    unsafe fn invoke(self, body: *const VMFunctionBody, vmctx: *mut VMContext) -> Output;
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
        impl<Output, $($item: FuncParam),*> FuncArgs<Output> for ($($item,)*) {
            type FuncType = unsafe extern "sysv64" fn(*mut VMContext $(, $item)*) -> Output;
            unsafe fn invoke(self, func: *const VMFunctionBody, vmctx: *mut VMContext) -> Output {
                let func: Self::FuncType =  mem::transmute(func);
                #[allow(non_snake_case)]
                let ($($item,)*) = self;
                func(vmctx $(,$item)*)
            }
        }
    }
}

/// Simple executor that assert the wasm file has an export function `invoke(a:i32, b:32)-> i32`.
pub fn execute<Output, Args: FuncArgs<Output>>(
    wat: &str,
    func: &str,
    args: Args,
    verbose: bool,
    chain: ChainCtx,
) -> Output {
    let wasm = wast::parse_str(wat).unwrap();
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

    let (compilation, relocations, _address_transform, _value_ranges, _stack_slots, _traps) =
        compile_module(
            &result.module,
            &result.translate_state,
            result.function_body_inputs,
            &*isa,
            verbose,
        )
        .unwrap();

    if verbose {
        println!("compilation result");
        for code in &compilation {
            disassm::print_disassembly(&code.body);
        }

        println!("relocations result");
        for (func, reloc) in relocations.iter() {
            println!("reloc for func {:?}", func);
            for (i, rel) in reloc.iter().enumerate() {
                println!("reloc:{} for func {:?}", i, rel);
            }
        }
    }

    let data_initializers = result.data_initializers;
    let module = std::rc::Rc::new(result.module);

    let total_code_size = (&compilation).into_iter().map(|code| code.body.len()).sum();
    let mut exec = MutableBuffer::new(total_code_size).unwrap();
    let mut finished_functions = PrimaryMap::new();
    for code in &compilation {
        let curr_size = exec.len();
        exec.set_len(curr_size + code.body.len());
        exec[curr_size..].copy_from_slice(&code.body);
        finished_functions.push(&exec[curr_size] as *const u8 as *const VMFunctionBody);
    }

    let jt_offsets = compilation.get_jt_offsets();

    let imports = {
        let mut resolver = ChainResolver;
        let mut imports = PrimaryMap::new();
        for (module, func) in module.imported_funcs.values() {
            imports.push(
                resolver
                    .resolve(module, func)
                    .expect("can not resolve import func"),
            );
        }
        imports.into_boxed_slice()
    };

    linker::link_module(&module, &finished_functions, &jt_offsets, relocations);
    let finished_functions = finished_functions.into_boxed_slice();

    let _exec = exec.make_exec().unwrap();

    let mut instance = InstanceHandle::new(
        module.clone(),
        finished_functions,
        imports,
        &data_initializers,
        chain.gas_left.clone(),
        Box::new(chain),
    )
    .unwrap();

    if module.memory_plans.len() > 0 {
        let memory = instance
            .instance_mut()
            .memory_slice_mut(DefinedMemoryIndex::from_u32(0));
        for i in 0..100 {
            memory[4 * i..4 * (i + 1)].copy_from_slice(&(i as u32).to_le_bytes());
        }
    }
    let invoke = instance
        .lookup(func)
        .expect(&format!("can not find export function:{}", func));

    let result = unsafe { args.invoke(invoke.address, invoke.vmctx) };

    result
}

/// Simple executor that assert the wasm file has an export function `invoke(a:i32, b:32)-> i32`.
pub fn call_invoke(wat: &str, verbose: bool, chain: ChainCtx) {
    let wasm = wast::parse_str(wat).unwrap();
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

    let (compilation, relocations, _address_transform, _value_ranges, _stack_slots, _traps) =
        compile_module(
            &result.module,
            &result.translate_state,
            result.function_body_inputs,
            &*isa,
            verbose,
        )
        .unwrap();

    if verbose {
        println!("compilation result");
        for code in &compilation {
            disassm::print_disassembly(&code.body);
        }

        println!("relocations result");
        for (func, reloc) in relocations.iter() {
            println!("reloc for func {:?}", func);
            for (i, rel) in reloc.iter().enumerate() {
                println!("reloc:{} for func {:?}", i, rel);
            }
        }
    }

    let data_initializers = result.data_initializers;
    let module = std::rc::Rc::new(result.module);

    let total_code_size = (&compilation).into_iter().map(|code| code.body.len()).sum();
    let mut exec = MutableBuffer::new(total_code_size).unwrap();
    let mut finished_functions = PrimaryMap::new();
    for code in &compilation {
        let curr_size = exec.len();
        exec.set_len(curr_size + code.body.len());
        exec[curr_size..].copy_from_slice(&code.body);
        finished_functions.push(&exec[curr_size] as *const u8 as *const VMFunctionBody);
    }

    let jt_offsets = compilation.get_jt_offsets();

    let imports = {
        let mut resolver = ChainResolver;
        let mut imports = PrimaryMap::new();
        for (module, func) in module.imported_funcs.values() {
            imports.push(
                resolver
                    .resolve(module, func)
                    .expect("can not resolve import func"),
            );
        }
        imports.into_boxed_slice()
    };

    linker::link_module(&module, &finished_functions, &jt_offsets, relocations);
    let finished_functions = finished_functions.into_boxed_slice();

    let _exec = exec.make_exec().unwrap();

    let mut instance = InstanceHandle::new(
        module.clone(),
        finished_functions,
        imports,
        &data_initializers,
        chain.gas_left.clone(),
        Box::new(chain),
    )
    .unwrap();

    if module.memory_plans.len() > 0 {
        let memory = instance
            .instance_mut()
            .memory_slice_mut(DefinedMemoryIndex::from_u32(0));
        for i in 0..100 {
            memory[4 * i..4 * (i + 1)].copy_from_slice(&(i as u32).to_le_bytes());
        }
    }

    let invoke = instance
        .lookup("invoke")
        .expect(&format!("can not find export function:{}", "invoke"));

    unsafe {
        if let Err(err) = wasmjit_call(invoke.vmctx, invoke.address) {
            println!("execute paniced: {}", err);
        }
    }
}
