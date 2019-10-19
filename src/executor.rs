use cranelift_codegen::isa;
use cranelift_codegen::settings;
use target_lexicon::PointerWidth;

use ontio_wasmjit_environ::compile_module;
use ontio_wasmjit_environ::ModuleEnvironment;
use ontio_wasmjit_environ::Tunables;

use crate::disassm;
use dynasmrt::mmap::MutableBuffer;

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

    let index = result
        .module
        .invoke_func
        .expect("wasm must has invoke function");
    let code = compilation.get(result.module.defined_func_index(index).unwrap());
    let mut exec = MutableBuffer::new(code.body.len()).unwrap();
    exec.set_len(code.body.len());
    exec.copy_from_slice(&code.body);
    let exec = exec.make_exec().unwrap();

    type FuncType = unsafe extern "sysv64" fn(i64, i32, i32) -> i32;
    let invoke_func: FuncType = unsafe { std::mem::transmute(exec.as_ptr()) };

    unsafe { invoke_func(0, a, b) }
}
