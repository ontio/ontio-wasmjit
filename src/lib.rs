#![cfg_attr(
    feature = "cargo-clippy",
    allow(
        clippy::missing_safety_doc,
        clippy::new_without_default,
        clippy::unreadable_literal,
        clippy::too_many_arguments
    )
)]

use crate::chain_api::{ChainCtx, ChainResolver};
use crate::error::Error;
use crate::executor::build_module;

pub mod chain_api;
pub mod error;
pub mod executor;
mod linker;
pub mod resolver;
mod trampoline;
pub mod utils;

pub mod disassm;

pub fn execute(
    wat: &str,
    chain: ChainCtx,
    func: &str,
    args: Vec<i64>,
) -> Result<Option<i64>, Error> {
    let wasm = wast::parse_str(wat).unwrap();
    let module = build_module(&wasm).unwrap();

    let mut resolver = ChainResolver;
    let mut instance = module.instantiate(&mut resolver).unwrap();

    instance.execute(chain, func, args)
}

#[test]
fn test_chain2() {
    use chain_api::Address;
    for method in [
        "get_current_block_hash",
        "get_current_tx_hash",
        "get_timestamp",
        "get_block_height",
        "get_block_height",
        "caller_address",
        "entry_address",
        "check_witness",
        "sha256",
    ]
    .iter()
    {
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
        let res: u64 = execute(wat, chain, "invoke", Vec::new()).unwrap().unwrap() as u64;
        assert_eq!(res, 1);
    }
}
