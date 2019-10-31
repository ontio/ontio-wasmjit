use crate::chain_api::{Address, ChainCtx, H256};
pub mod chain_api;
pub mod executor;
mod linker;
pub mod resolver;

pub mod disassm;

use crate::executor::FuncArgs;

#[test]
fn test_add_one() {
    let wat = include_str!("../tests/add_one.wast");

    for _i in 0..100 {
        let a: i32 = rand::random();
        let sum: i32 = execute(wat, "add_one", (a,), false);
        assert_eq!(sum, a.wrapping_add(1));
    }
}

#[test]
fn test_add() {
    let wat = include_str!("../tests/add.wast");
    for _i in 0..100 {
        let (a, b): (i32, i32) = rand::random();
        let sum: i32 = execute(wat, "add", (a, b), false);
        assert_eq!(sum, a.wrapping_add(b));
    }
}

#[test]
fn test_load_memory() {
    let wat = include_str!("../tests/memory-load.wast");

    let sum: i32 = execute(wat, "load_add", (0, 4), false);
    assert_eq!(sum, 1);
}

#[test]
fn test_sum() {
    let wat = include_str!("../tests/sum.wast");
    let sum: i32 = execute(wat, "sum", (0i32, 100i32), false);
    assert_eq!(sum, 4950);
}

#[test]
fn test_subtract() {
    let wat = include_str!("../tests/subtract.wast");
    for i in 0..100 {
        let (a, b): (i32, i32) = rand::random();
        let sub: i32 = execute(wat, "sub", (a, b), false);
        assert_eq!(sub, a.wrapping_sub(b));
    }
}

#[test]
fn test_load_subtract() {
    let wat = include_str!("../tests/load_sub.wast");
    let sub: i32 = execute(wat, "sub", (4, 0), false);
    assert_eq!(sub, 1)
}

#[test]
fn test_multiply() {
    let wat = include_str!("../tests/multiply.wast");
    for i in 0..100 {
        let (a, b): (i32, i32) = rand::random();
        let sum: i32 = execute(wat, "mul", (a, b), false);
        assert_eq!(sum, a.wrapping_mul(b));
    }
}

#[test]
fn test_load_multiply() {
    let wat = include_str!("../tests/load_mul.wast");
    let mul: i32 = execute(wat, "mul", (4, 8), false);
    assert_eq!(mul, 2);
}

/// Simple executor that assert the wasm file has an export function `invoke(a:i32, b:32)-> i32`.
pub fn execute<Output, Args: FuncArgs<Output>>(
    wat: &str,
    func: &str,
    args: Args,
    verbose: bool,
) -> Output {
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
    executor::execute(wat, func, args, verbose, chain)
}

#[test]
fn test_div() {
    let wat = include_str!("../tests/div.wast");
    for i in 0..100 {
        let (a, b): (i32, i32) = rand::random();
        if b == 0 {
            continue;
        }
        let sum: i32 = execute(wat, "div", (a, b), false);
        assert_eq!(sum, a.wrapping_div(b));
    }
}

#[test]
fn test_fibonacci() {
    fn fib(x: i32) -> i32 {
        if x < 0 {
            return 0;
        } else if x == 1 || x == 2 {
            return 1;
        } else {
            return fib(x - 1) + fib(x - 2);
        }
    }
    let wat = include_str!("../tests/fibonacci.wast");
    for i in 0..30 {
        let sum: i32 = execute(wat, "fib", (i,), false);
        assert_eq!(sum, fib(i));
    }
}

#[test]
fn test_global() {
    let wat = include_str!("../tests/global.wast");
    for i in 0..100 {
        let a: i32 = rand::random();
        let sum: i32 = execute(wat, "get-global", (a,), false);
        assert_eq!(sum, a + 1);
    }
}

#[test]
fn test_br_table() {
    let wat = include_str!("../tests/br_table.wast");
    for i in 0u32..255 {
        let _: i32 = execute(wat, "br_table", (i, 3), false);
    }
}

#[test]
fn test_chain2() {
    fn excute(method: &str) {
        let wat = include_str!("../tests/chain-api.wast");
        let callers: Vec<Address> = vec![[1u8; 20]];
        let witness: Vec<Address> = vec![[1u8; 20]];
        let chain = ChainCtx::new(
            1,
            1u32,
            [1u8; 32],
            [1u8; 32],
            [1u8; 20],
            callers,
            witness,
            method.as_bytes().to_vec(),
            Vec::new(),
        );
        let res: u64 = executor::execute(wat, "invoke", (), false, chain);
        assert_eq!(res, 1);
    }
    excute("get_current_block_hash");

    excute("get_current_tx_hash");

    excute("get_timestamp");

    excute("get_block_height");

    excute("get_block_height");

    excute("caller_address");

    excute("entry_address");

    excute("check_witness");
}
