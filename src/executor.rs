use cranelift_codegen::isa;
use cranelift_codegen::settings;
use target_lexicon::PointerWidth;

use ontio_wasmjit_environ::compile_module;
use ontio_wasmjit_environ::ModuleEnvironment;
use ontio_wasmjit_environ::Tunables;

use crate::disassm;
use cranelift_wasm::DefinedMemoryIndex;
use dynasmrt::mmap::MutableBuffer;

use cranelift_entity::PrimaryMap;
use ontio_wasmjit_runtime::{InstanceHandle, VMContext, VMFunctionBody};

/// Simple executor that assert the wasm file has an export function `invoke(a:i32, b:32)-> i32`.
pub fn execute(wat: &str, a: i32, b: i32, verbose: bool) -> i32 {
    let wasm = wast::parse_str(wat).unwrap();
    let config = isa::TargetFrontendConfig {
        default_call_conv: isa::CallConv::SystemV,
        pointer_width: PointerWidth::U64,
    };

    let isa_builder = isa::lookup_by_name("x86_64").unwrap();
    let flag_builder = settings::builder();
    let isa = isa_builder.finish(settings::Flags::new(flag_builder));

    let module_environ = ModuleEnvironment::new(config, Tunables::default());
    let result = module_environ.translate(&wasm).unwrap();

    let (compilation, relocations, address_transform, value_ranges, stack_slots, _traps) =
        compile_module(&result.module, result.function_body_inputs, &*isa, verbose).unwrap();

    if verbose {
        println!("compilation result");
        for code in &compilation {
            disassm::print_disassembly(&code.body);
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
    let exec = exec.make_exec().unwrap();

    let finished_functions = finished_functions.into_boxed_slice();
    let imports = PrimaryMap::new().into_boxed_slice();

    let mut instance = InstanceHandle::new(
        module.clone(),
        finished_functions,
        imports,
        &data_initializers,
        Box::new(0),
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
    let invoke = instance.lookup("invoke").unwrap();

    type FuncType = unsafe extern "sysv64" fn(*mut VMContext, i32, i32) -> i32;
    let invoke_func: FuncType = unsafe { std::mem::transmute(invoke.address) };

    unsafe { invoke_func(invoke.vmctx, a, b) }
}
