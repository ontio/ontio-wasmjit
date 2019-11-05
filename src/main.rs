use ontio_wasmjit::chain_api::{Address, ChainCtx};
use ontio_wasmjit::execute;
use ontio_wasmjit::executor;
use ontio_wasmjit::executor::FuncArgs;

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
    env_logger::init();
    for i in 0..10 {
        let a = rand::random::<i32>() % 100;
        let b = rand::random::<i32>() % 100 + 1;
        let sum: i32 = execute(ADD, "add", (a, b), i == 0);
        println!("{:3} / {:3} = {:4}", a, b, sum);
    }

    let chain = ChainCtx::new(
        1,
        1u32,
        [1u8; 32],
        [1u8; 32],
        [1u8; 20],
        Vec::new(),
        Vec::new(),
        b"function fib(n) { if (n==0 || n==1) { return n; } else { return fib(n-1) + fib(n-2);}}; fib(8)".to_vec(),
        Vec::new(),
    );
    let wat = include_str!("../tests/jsvm.wast");
    let now = std::time::Instant::now();
    log::error!("begin execute ");
    let _: () = executor::execute(wat, "invoke", (), false, chain);
    log::error!("end execute: {:?}", now.elapsed());
}
