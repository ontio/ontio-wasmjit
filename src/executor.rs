use cranelift_codegen::ir::types;
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
use crate::trampoline::make_trampoline;
use cranelift_entity::PrimaryMap;
use ontio_wasmjit_runtime::{
    wasmjit_call, wasmjit_call_trampoline, InstanceHandle, VMFunctionBody,
};

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

/// Simple executor that assert the wasm file has an export function `invoke(a:i32, b:32)-> i32`.
pub fn execute<Args: FuncArgs>(
    wat: &str,
    func: &str,
    args: Args,
    verbose: bool,
    chain: ChainCtx,
) -> Option<i64> {
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
        }
    }
    if invoke.signature.returns.len() == 0 {
        return None;
    } else {
        if invoke.signature.returns[0].value_type == types::I32 {
            return Some(args_vec[0] as i32 as i64);
        }
        return Some(args_vec[0] as i64);
    }
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
