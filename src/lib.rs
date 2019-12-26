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
use parity_wasm::elements::Module;

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
    let wasm = wat::parse_str(wat).unwrap();
    let module = build_module(&wasm).unwrap();

    let mut resolver = ChainResolver;
    let mut instance = module.instantiate(&mut resolver).unwrap();

    instance.execute(chain, func, args)
}

pub fn execute2(
    wasm: Vec<u8>,
    chain: ChainCtx,
    func: &str,
    args: Vec<i64>,
) -> Result<Option<i64>, Error> {
    let module = build_module(&wasm).unwrap();

    let mut resolver = ChainResolver;
    let mut instance = module.instantiate(&mut resolver).unwrap();

    instance.execute(chain, func, args)
}

#[test]
fn test_chain2() {
    use chain_api::Address;
    use ontio_wasmjit_runtime::ExecMetrics;

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
        let exec_metrics = ExecMetrics::new(u64::max_value(), 1, u64::max_value(), 100000u64);

        let chain = ChainCtx::new(
            1,
            1u32,
            [1u8; 32],
            [1u8; 32],
            callers,
            witness,
            method.as_bytes().to_vec(),
            exec_metrics,
            0,
        );
        let res: u64 = execute(wat, chain, "invoke", Vec::new()).unwrap().unwrap() as u64;
        assert_eq!(res, 1);
    }
}

#[test]
fn test_check_gas() {
    let mut module = parity_wasm::deserialize_file("./tests/gas_test.wasm").unwrap();
    for i in 0..100 {
        println!("{}", i);
        check_gas(module.clone());
    }
}

fn check_gas(mut module: Module) {
    use chain_api::Address;
    use ontio_wasmjit_runtime::ExecMetrics;
    use parity_wasm::builder;
    use parity_wasm::elements;
    use parity_wasm::elements::Module;
    use std::sync::atomic::{AtomicU64, Ordering};
    use wasmi::{ImportsBuilder, ModuleInstance, NopExternals, RuntimeValue};

    let mut diff_val = 0;
    let mut sections = module.sections_mut();
    let mut index = rand::random::<usize>() % sections.len();
    if let Some(&mut elements::Section::Code(ref mut code_section)) = sections.get_mut(index) {
        let bodies = code_section.bodies_mut();
        let index = rand::random::<usize>() % bodies.len();
        diff_val = inject_div(bodies.get_mut(index).unwrap().code_mut());
    } else {
        return;
    }

    let module_temp = wasmi::Module::from_parity_wasm_module(module.clone())
        .expect("parity-wasm builder generated invalid module!");
    let interpret_consume = interpret(&module_temp);

    let code = module.clone().code_section_mut().unwrap();
    let exec_metrics = ExecMetrics::new(u64::max_value(), 1, u64::max_value(), 100000u64);

    let callers: Vec<Address> = vec![[1u8; 20], [1u8; 20]];
    let witness: Vec<Address> = vec![[1u8; 20]];
    let chain = ChainCtx::new(
        1,
        1u32,
        [1u8; 32],
        [1u8; 32],
        callers,
        witness,
        "invoke".as_bytes().to_vec(),
        exec_metrics,
        0,
    );
    let mut build = builder::from_module(module.clone());
    let mut temp = parity_wasm::serialize(build.build()).unwrap();
    let exec = chain.get_exec_metrics();
    let mut before = exec.gas_left.load(Ordering::Relaxed);
    execute2(temp, chain, "invoke", vec![1, 2]);
    let mut after = exec.gas_left.load(Ordering::Relaxed);
    let jit_consume = before - after;
    println!("diff_val: {}", diff_val);
    println!("jit consume: {}", jit_consume);
    println!("interpret consume: {}", interpret_consume);
    if interpret_consume != jit_consume + 3 + diff_val as u64 {
        parity_wasm::serialize_to_file("aaaaa.wasm", module);
    }
    assert_eq!(interpret_consume, jit_consume + 3 + diff_val as u64);

    fn inject_div(instructions: &mut elements::Instructions) -> usize {
        use parity_wasm::elements::Instruction::*;
        let instructions = instructions.elements_mut();
        let mut index = rand::random::<usize>() % instructions.len();
        let mut flush_position = 0;
        let mut position = index;
        loop {
            if position == 0 {
                break;
            }
            match &instructions[position] {
                &Block(_)
                | &If(_)
                | &Br(_)
                | &BrIf(_)
                | &BrTable(_)
                | &Unreachable
                | &Loop(_)
                | &Else
                | &Return
                | &End
                | &CallIndirect(_, _)
                | &Call(_) => {
                    flush_position = position;
                    break;
                },
                _ => {}
            };
            position -= 1;
            if position == 0 {
                break;
            }
        }
        if index >= 1 {
            match &instructions[index] {
                End => {
                    index -= 1;
                }
                _ =>{}
            }
        }
        println!("insert index: {}", index);
        println!("flush pos: {:?}", flush_position);
        println!("insert before: {:?}", instructions);
        instructions.insert(index, I32Const(2));
        instructions.insert(index + 1, I32Const(0));
        instructions.insert(index + 2, I32DivS);
        instructions.insert(index + 3, Drop);
        println!("insert after: {:?}", instructions);
        index - flush_position
    }

    fn interpret(module: &wasmi::Module) -> u64 {
        let main = ModuleInstance::new(module, &ImportsBuilder::default())
            .expect("Failed to instantiate module")
            .run_start(&mut NopExternals)
            .expect("Failed to run start function in module");
        match main.invoke_export(
            "invoke",
            &[RuntimeValue::I32(1), RuntimeValue::I32(2)],
            &mut NopExternals,
        ) {
            Ok((_, gas)) => gas,
            Err((_, gas)) => gas,
        }
    }
}
