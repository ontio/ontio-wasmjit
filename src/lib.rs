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
use ontio_wasmjit_environ::BuildOption;

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
    execute2(wasm, chain, func, args)
}

pub fn execute2(
    wasm: Vec<u8>,
    chain: ChainCtx,
    func: &str,
    args: Vec<i64>,
) -> Result<Option<i64>, Error> {
    let module = build_module(&wasm, BuildOption::new().gas_metering(true)).unwrap();

    let mut resolver = ChainResolver;
    let mut instance = module.instantiate(&mut resolver).unwrap();

    instance.execute(chain, func, args)
}

#[test]
fn test_memory_grow() {
    use ontio_wasmjit_runtime::ExecMetrics;

    let wat = r#"
        (module
          (type $t0 (func (result i32)))
          (func $add (export "invoke") (type $t0)  (result i32)
                        (memory.grow (i32.const 12))
                        )
          (memory (;0;) 1 1000)
        )"#;
    let exec_metrics = ExecMetrics::new(1, u64::max_value(), 1, u64::max_value(), 100000u64);

    let chain = ChainCtx::new(
        1,
        1u32,
        [1u8; 32],
        [1u8; 32],
        Vec::new(),
        Vec::new(),
        Vec::new(),
        exec_metrics,
        0,
    );

    let wasm = wat::parse_str(wat).unwrap();
    // here not enable_gas_metering
    let module = build_module(&wasm, BuildOption::new().set_mem_gas_factor(2000)).unwrap();

    let mut resolver = ChainResolver;
    let mut instance = module.instantiate(&mut resolver).unwrap();

    let _res: u64 = instance
        .execute(chain, "invoke", Vec::new())
        .unwrap()
        .unwrap() as u64;

    let host = instance.host_state();
    let gas_left = host.exec_metrics.gas_left.load(Ordering::Relaxed);
    use std::sync::atomic::Ordering;
    assert_eq!(gas_left, u64::max_value() - 2000 * 12);
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
        let exec_metrics = ExecMetrics::new(1, u64::max_value(), 1, u64::max_value(), 100000u64);

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
    use parity_wasm::elements::Module;
    let module = parity_wasm::deserialize_file("./tests/gas_test.wasm").unwrap();
    for _ in 0..100 {
        check_gas(module.clone());
    }
    fn check_gas(mut module: Module) {
        use chain_api::Address;
        use ontio_wasmjit_runtime::ExecMetrics;
        use parity_wasm::builder;
        use parity_wasm::elements;
        use std::sync::atomic::Ordering;
        use wasmi::{ImportsBuilder, ModuleInstance, NopExternals, RuntimeValue};

        let sections = module.sections_mut();
        let index = rand::random::<usize>() % sections.len();
        let diff_val = if let Some(&mut elements::Section::Code(ref mut code_section)) =
            sections.get_mut(index)
        {
            let bodies = code_section.bodies_mut();
            let index = rand::random::<usize>() % bodies.len();
            inject_div(bodies.get_mut(index).unwrap().code_mut())
        } else {
            return;
        };

        let module_temp = wasmi::Module::from_parity_wasm_module(module.clone())
            .expect("parity-wasm builder generated invalid module!");
        let interpret_consume = interpret(&module_temp);

        let exec_metrics = ExecMetrics::new(1, u64::max_value(), 1, u64::max_value(), 100000u64);

        let callers: Vec<Address> = vec![[1u8; 20], [1u8; 20]];
        let witness: Vec<Address> = vec![[1u8; 20]];
        let method = "invoke";
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
        let build = builder::from_module(module.clone());
        let temp = parity_wasm::serialize(build.build()).unwrap();
        let exec = chain.get_exec_metrics();
        let before = exec.gas_left.load(Ordering::Relaxed);
        let _ = execute2(temp, chain, "invoke", vec![1, 2]);
        let after = exec.gas_left.load(Ordering::Relaxed);
        let jit_consume = before - after;
        println!("diff_val: {}", diff_val);
        println!("jit consume: {}", jit_consume);
        println!("interpret consume: {}", interpret_consume);
        assert_eq!(interpret_consume, jit_consume + 3 + diff_val as u64);

        fn inject_div(instructions: &mut elements::Instructions) -> usize {
            use parity_wasm::elements::Instruction::*;
            let instructions = instructions.elements_mut();
            let mut index = rand::random::<usize>() % instructions.len();
            loop {
                if index >= 1 {
                    match &instructions[index] {
                        End => {
                            index -= 1;
                        }
                        _ => {
                            break;
                        }
                    }
                } else {
                    break;
                }
            }
            let mut flush_position = 0;
            let mut position = {
                if index > 0 {
                    index - 1
                } else {
                    0
                }
            };
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
                    }
                    _ => {}
                };
                position -= 1;
            }
            instructions.insert(index, I32Const(2));
            instructions.insert(index + 1, I32Const(0));
            instructions.insert(index + 2, I32DivS);
            instructions.insert(index + 3, Drop);
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
}
