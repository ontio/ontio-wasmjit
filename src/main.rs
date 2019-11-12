use ontio_wasmjit::chain_api::ChainCtx;
use ontio_wasmjit::execute;
use ontio_wasmjit::executor::call_invoke;

const ADD: &str = include_str!("../tests/add.wast");

fn main() {
    let chain = ChainCtx::new(
        1,
        1u32,
        [1u8; 32],
        [1u8; 32],
        [1u8; 20],
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    );
    let wat = include_str!("../tests/panic.wast");
    call_invoke(wat, false, chain);
    return env_logger::init();
    for i in 0..10 {
        let a = rand::random::<i32>() % 100;
        let b = rand::random::<i32>() % 100 + 1;
        let sum: i64 = execute(ADD, "add", (a, b), i == 0).unwrap();
        println!("{:3} / {:3} = {:4}", a, b, sum);
    }

    let wat = include_str!("../tests/chain-api.wast");

    let time = execute(wat, "invoke", (), true).unwrap() as u64;
    println!("timestamp: {}", time);

    let wat = include_str!("../tests/br_table.wast");
    for i in 0u32..255 {
        execute(wat, "br_table", (i, 3), i == 0);
    }
}
