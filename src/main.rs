
use cranelift_codegen::isa;
use cranelift_wasm::{translate_module, DummyEnvironment, ReturnMode};
use target_lexicon::PointerWidth;


const ADD_ONE: &str = r#"
(module
  (type $t0 (func (param i32) (result i32)))
  (func $add_one (export "add_one") (type $t0) (param $p0 i32) (result i32)
    get_local $p0
    i32.const 1
    i32.add)
  (func $add_two (export "add_two") (type $t0) (param $p0 i32) (result i32)
    get_local $p0
    i32.const 2
    i32.add)
)
"#;

fn main() {

    let wasm = wabt::wat2wasm(ADD_ONE).unwrap();
    let mut runtime = DummyEnvironment::new(
        isa::TargetFrontendConfig {
            default_call_conv: isa::CallConv::Fast,
            pointer_width: PointerWidth::U64,
        },
        ReturnMode::NormalReturns,
        false,
    );

    let result = translate_module(&wasm, &mut runtime);
    println!("Hello, world: {:?}", result);
    for (k, v) in &runtime.info.functions {
        println!("module info: {:?} {:?}", k, v.export_names);
    }

    for (k, v) in &runtime.info.function_bodies {
        println!("function info: {:?} \n {}", k, v.display(None));
    }
}
