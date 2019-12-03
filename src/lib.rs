use crate::chain_api::ChainCtx;
pub mod chain_api;
pub mod error;
pub mod executor;
mod linker;
pub mod resolver;
mod trampoline;
pub mod utils;

pub mod disassm;

use crate::executor::FuncArgs;

#[test]
fn test_add_one() {
    let wat = include_str!("../tests/add_one.wast");

    for _i in 0..100 {
        let a: i32 = rand::random();
        let sum: i32 = execute(wat, "add_one", (a,), false).unwrap() as i32;
        assert_eq!(sum, a.wrapping_add(1));
    }
}

#[test]
fn test_add() {
    let wat = include_str!("../tests/add.wast");
    for _i in 0..100 {
        let (a, b): (i32, i32) = rand::random();
        let sum = execute(wat, "add", (a, b), false).unwrap() as i32;
        assert_eq!(sum, a.wrapping_add(b));
    }
}

#[test]
fn test_subtract() {
    let wat = include_str!("../tests/subtract.wast");
    for _i in 0..100 {
        let (a, b): (i32, i32) = rand::random();
        let sub = execute(wat, "sub", (a, b), false).unwrap() as i32;
        assert_eq!(sub, a.wrapping_sub(b));
    }
}

#[test]
fn test_multiply() {
    let wat = include_str!("../tests/multiply.wast");
    for _i in 0..100 {
        let (a, b): (i32, i32) = rand::random();
        let sum = execute(wat, "mul", (a, b), false).unwrap() as i32;
        assert_eq!(sum, a.wrapping_mul(b));
    }
}
/// Simple executor that assert the wasm file has an export function `invoke(a:i32, b:32)-> i32`.
pub fn execute<Args: FuncArgs>(wat: &str, func: &str, args: Args, verbose: bool) -> Option<i64> {
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
    executor::execute(wat, func, args, verbose, chain)
}

#[test]
fn test_div() {
    let wat = include_str!("../tests/div.wast");
    for _i in 0..100 {
        let (a, b): (i32, i32) = rand::random();
        if b == 0 {
            continue;
        }
        let sum = execute(wat, "div", (a, b), false).unwrap() as i32;
        assert_eq!(sum, a.wrapping_div(b));
    }
}

#[test]
fn test_global() {
    let wat = include_str!("../tests/global.wast");
    for _i in 0..100 {
        let a: i32 = rand::random();
        let sum = execute(wat, "get-global", (a,), false).unwrap() as i32;
        assert_eq!(sum, a + 1);
    }
}

#[test]
fn test_br_table() {
    let wat = include_str!("../tests/br_table.wast");
    for i in 0u32..255 {
        execute(wat, "br_table", (i, 3), false);
    }
}

#[test]
fn test_chain2() {
    use chain_api::Address;
    fn excute(method: &str) {
        let wat = include_str!("../tests/chain-api.wast");
        let callers: Vec<Address> = vec![[1u8; 20], [1u8; 20]];
        let witness: Vec<Address> = vec![[1u8; 20]];
        let chain = ChainCtx::new(
            1,
            1u32,
            [1u8; 32],
            [1u8; 32],
            callers,
            witness,
            method.as_bytes().to_vec(),
            Vec::new(),
            0,
        );
        let res: u64 = executor::execute(wat, "invoke", (), false, chain).unwrap() as u64;
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

    excute("sha256");
}
