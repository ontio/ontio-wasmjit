#![feature(test)]
extern crate test;

use std::env;
use std::fs::File;
use std::io::Read;
use std::sync::{atomic::Ordering, Arc, Mutex};

use anyhow::Result;
use glob::glob;
use test::{ShouldPanic, TestDesc, TestDescAndFn, TestFn, TestName, TestType};
use wast::{
    parser::{self, ParseBuffer},
    Expression, Instruction, Wast, WastDirective, WastExecute, WastInvoke,
};

use ontio_wasmjit::chain_api::{ChainCtx, ChainResolver};
use ontio_wasmjit::executor::{build_module, Instance};
use std::collections::HashMap;

fn make_chain() -> ChainCtx {
    let mut chain = ChainCtx::default();
    chain.set_gas_left(u64::max_value());
    chain.set_exec_step(u64::max_value());
    chain.set_gas_factor(1);
    chain.set_depth_left(10000u64);

    chain
}

fn build_instance(wasm: &[u8]) -> Instance {
    let module = build_module(wasm).unwrap();
    let mut resolver = ChainResolver;
    module.instantiate(&mut resolver).unwrap()
}

fn evaluate_expression(exp: &Expression<'_>) -> Option<i64> {
    if exp.instrs.len() == 1 {
        let result = match exp.instrs[0] {
            Instruction::I64Const(val) => val,
            Instruction::I32Const(val) => val as i64,
            _ => return None,
        };

        return Some(result);
    }

    None
}

fn evaluate_expressions(exps: &[Expression<'_>]) -> Option<Vec<i64>> {
    let args: Vec<_> = exps.iter().map(evaluate_expression).collect();
    if args.iter().any(Option::is_none) {
        return None;
    }

    Some(args.into_iter().map(Option::unwrap).collect())
}

fn run_spec_file(file: &str, test_count: &mut usize) -> Result<Vec<TestDescAndFn>> {
    let path = if file == "gas_test.wast" || file == "depth_check.wast" || file == "test.wast" {
        format!("tests/{}", file)
    } else {
        format!("spectests/{}", file)
    };
    println!("test file path: {}", path);

    let mut contents = String::new();
    File::open(path)?.read_to_string(&mut contents)?;
    let buf = ParseBuffer::new(&contents)?;
    let wast: Wast = parser::parse(&buf)?;
    let mut wasm: Option<Arc<_>> = None;
    let mut instance: Option<Arc<Mutex<Instance>>> = None;
    let mut testcases = Vec::new();

    let gas_cost_map = init_gas_map();

    for iterm in wast.directives {
        match iterm {
            WastDirective::Module(mut module) => {
                wasm = Some(Arc::new(module.encode()?));
            }
            WastDirective::Invoke(invoke) => {
                if let Some(args) = evaluate_expressions(&invoke.args) {
                    let name = invoke.name.to_string();
                    let instance = instance.clone().unwrap();

                    let testfunc = move || {
                        instance
                            .lock()
                            .unwrap()
                            .execute(make_chain(), &name, args)
                            .unwrap();
                    };
                    build_test_cases(&mut testcases, test_count, Box::new(testfunc), invoke, file);
                }
            }
            WastDirective::AssertTrap { exec, message, .. } => {
                if let WastExecute::Invoke(invoke) = exec {
                    if let Some(code) = wasm.take() {
                        let result = Arc::new(Mutex::new(build_instance(&code)));
                        instance = Some(result.clone());
                    }
                    if let Some(args) = evaluate_expressions(&invoke.args) {
                        let name = invoke.name.to_string();
                        let instance = instance.clone().unwrap();
                        let trap_msg = message.to_string();

                        let testfunc = move || {
                            instance
                                .lock()
                                .unwrap()
                                .execute(make_chain(), &name, args)
                                .expect_err(&trap_msg);
                        };
                        build_test_cases(
                            &mut testcases,
                            test_count,
                            Box::new(testfunc),
                            invoke,
                            file,
                        );
                    }
                }
            }
            WastDirective::AssertReturn { exec, results, .. } => {
                if let WastExecute::Invoke(invoke) = exec {
                    if let Some(code) = wasm.take() {
                        let result = Arc::new(Mutex::new(build_instance(&code)));
                        instance = Some(result.clone());
                    }

                    if let (Some(args), Some(results)) = (
                        evaluate_expressions(&invoke.args),
                        evaluate_expressions(&results),
                    ) {
                        let instance = instance.clone().unwrap();
                        let name = invoke.name.to_string();
                        let gas_cost_map = gas_cost_map.clone();
                        let test_func = move || {
                            let chain = make_chain();
                            let gas_left = chain.get_gas_left();
                            let start_gas = gas_left.load(Ordering::Relaxed);

                            let res: Vec<_> = instance
                                .lock()
                                .unwrap()
                                .execute(chain, &name, args)
                                .unwrap()
                                .into_iter()
                                .collect();
                            assert_eq!(res, results);
                            let end_gas = gas_left.load(Ordering::Relaxed);
                            if gas_cost_map.contains_key(&name) {
                                let expected = gas_cost_map.get(&name).unwrap_or(&0);
                                assert_eq!(*expected as u64, start_gas - end_gas);
                            }
                        };
                        build_test_cases(
                            &mut testcases,
                            test_count,
                            Box::new(test_func),
                            invoke,
                            file,
                        );
                    }
                };
            }
            _ => (),
        }
    }

    Ok(testcases)
}

fn build_test_cases(
    testcases: &mut Vec<TestDescAndFn>,
    test_count: &mut usize,
    test_func: Box<impl FnOnce() + std::marker::Send + 'static>,
    invoke: WastInvoke,
    file: &str,
) {
    *test_count += 1;
    let test_name = if invoke.name.as_bytes().iter().any(|x| *x == 0) {
        format!("{}:unknown-name", file)
    } else {
        format!("{:04} {}:{}", *test_count, file, invoke.name)
    };

    testcases.push(TestDescAndFn {
        desc: TestDesc {
            name: TestName::DynTestName(test_name),
            ignore: false,
            should_panic: ShouldPanic::No,
            allow_fail: false,
            test_type: TestType::UnitTest,
        },
        testfn: TestFn::DynTestFn(test_func),
    });
}

fn init_gas_map() -> HashMap<String, i32> {
    let mut gas_map = HashMap::new();
    for (func, gas) in [
        ("gas-add", 4),
        ("gas-empty-block", 5),
        ("gas-type-i32-value-br", 4),
        ("gas-as-br-value", 5),
        ("gas-type-i32-value-br-table", 5),
        ("gas-set-x", 3),
        ("gas-empty-if", 3),
        ("gas-type-second-i64", 6),
        ("gas-empty-loop", 3),
        ("gas-singular-loop", 4),
        ("gas-load_at_zero", 3),
        ("gas-select_i32", 5),
        ("gas-stmt", 16),
        ("gas-as-func-first", 3),
        ("gas-type-i32", 3),
    ]
    .iter()
    {
        gas_map.insert((*func).to_string(), *gas);
    }

    gas_map
}

fn main() {
    let mut test_count = 0;
    let mut funcs = Vec::new();
    for entry in glob("spectests/*.wast").unwrap() {
        if let Ok(path) = entry {
            let file = path.file_name().unwrap().to_str().unwrap();
            if [
                "f32",
                "f64",
                "float",
                "func_ptrs",
                "imports",
                "elem",
                "exports",
                "linking",
                "start",
                "names",
            ]
            .iter()
            .any(|e| file.contains(e))
            {
                continue;
            }

            funcs.extend(run_spec_file(file, &mut test_count).unwrap());
        }
    }

    for entry in glob("tests/*.wast").unwrap() {
        if let Ok(path) = entry {
            let file = path.file_name().unwrap().to_str().unwrap();
            funcs.extend(run_spec_file(file, &mut test_count).unwrap());
        }
    }

    let mut args: Vec<String> = env::args().collect();
    args.push("--test-threads=1".to_string());
    test::test_main(&args, funcs, None);
}
