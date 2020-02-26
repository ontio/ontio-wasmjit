use ontio_wasmjit::chain_api::{ChainCtx, ChainResolver};
use ontio_wasmjit::error::Error;
use ontio_wasmjit::execute;
use ontio_wasmjit::executor::build_module;
use ontio_wasmjit_environ::BuildOption;
use ontio_wasmjit_runtime::ExecMetrics;

const ADD: &str = include_str!("../tests/add.wast");

pub fn call_invoke(wat: &str, chain: ChainCtx) -> Result<(), Error> {
    let wasm = wat::parse_str(wat).unwrap();
    let module = build_module(&wasm, BuildOption::new().gas_metering(true)).unwrap();

    let mut resolver = ChainResolver;
    let mut instance = module.instantiate(&mut resolver).unwrap();
    instance.invoke(Box::new(chain))
}

fn make_chain() -> ChainCtx {
    let exec_metrics = ExecMetrics::new(u64::max_value(), 1, u64::max_value(), 100_000_u64);
    ChainCtx::new(
        1,
        1u32,
        [1u8; 32],
        [1u8; 32],
        Vec::new(),
        Vec::new(),
        Vec::new(),
        exec_metrics,
        0,
    )
}

fn main() {
    let wat = include_str!("../tests/divtrap.wast");

    execute(wat, make_chain(), "divtrap", vec![]).expect_err("should trap");

    let wat = include_str!("../tests/panic.wast");
    call_invoke(wat, make_chain()).expect_err("should panic");

    env_logger::init();
    for _i in 0..10 {
        let a = rand::random::<i32>() % 100;
        let b = rand::random::<i32>() % 100 + 1;
        let sum: i64 = execute(ADD, make_chain(), "add", vec![a as i64, b as i64])
            .unwrap()
            .unwrap();
        println!("{:3} / {:3} = {:4}", a, b, sum);
    }

    let wat = include_str!("../tests/chain-api.wast");

    let time = execute(wat, make_chain(), "invoke", vec![])
        .unwrap()
        .unwrap() as u64;
    println!("timestamp: {}", time);

    let wat = include_str!("../tests/br_table.wast");
    for i in 0u32..255 {
        execute(wat, make_chain(), "br_table", vec![i as i64, 3]).unwrap();
    }
}
