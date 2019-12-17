use ontio_wasmjit::chain_api::{ChainCtx, ChainResolver};
use ontio_wasmjit::execute;
use ontio_wasmjit::executor::build_module;

const ADD: &str = include_str!("../tests/add.wast");

pub fn call_invoke(wat: &str, verbose: bool, chain: ChainCtx) {
    let wasm = wast::parse_str(wat).unwrap();
    let module = build_module(&wasm).unwrap();

    if verbose {
        module.dump();
    }

    let mut resolver = ChainResolver;
    let mut instance = module.instantiate(&mut resolver).unwrap();
    instance.invoke(Box::new(chain)).unwrap();
}

fn main() {
    let wat = include_str!("../tests/divtrap.wast");

    execute(wat, "divtrap", vec![], true);

    let chain = ChainCtx::new(
        1,
        1u32,
        [1u8; 32],
        [1u8; 32],
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        0,
    );
    let wat = include_str!("../tests/panic.wast");
    call_invoke(wat, false, chain);
    return;

    env_logger::init();
    for i in 0..10 {
        let a = rand::random::<i32>() % 100;
        let b = rand::random::<i32>() % 100 + 1;
        let sum: i64 = execute(ADD, "add", vec![a as i64, b as i64], i == 0).unwrap();
        println!("{:3} / {:3} = {:4}", a, b, sum);
    }

    let wat = include_str!("../tests/chain-api.wast");

    let time = execute(wat, "invoke", vec![], true).unwrap() as u64;
    println!("timestamp: {}", time);

    let wat = include_str!("../tests/br_table.wast");
    for i in 0u32..255 {
        execute(wat, "br_table", vec![i as i64, 3], i == 0);
    }
}
