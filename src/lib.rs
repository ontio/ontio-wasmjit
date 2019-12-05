use crate::chain_api::ChainCtx;
pub mod chain_api;
pub mod error;
pub mod executor;
mod linker;
pub mod resolver;
mod trampoline;
pub mod utils;

pub mod disassm;

/// Simple executor that assert the wasm file has an export function `invoke(a:i32, b:32)-> i32`.
pub fn execute(wat: &str, func: &str, args: Vec<i64>, verbose: bool) -> Option<i64> {
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
        let res: u64 = executor::execute(wat, "invoke", Vec::new(), false, chain).unwrap() as u64;
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
