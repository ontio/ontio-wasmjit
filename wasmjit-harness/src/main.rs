#![feature(test)]
extern crate test;

use std::env;
use std::fs::File;
use std::io::Read;
use std::sync::{atomic::AtomicU64, atomic::Ordering, Arc, Mutex};

use anyhow::Result;
use glob::glob;
use test::{ShouldPanic, TestDesc, TestDescAndFn, TestFn, TestName, TestType};
use wast::{
    parser::{self, ParseBuffer},
    Expression, Instruction, Wast, WastDirective, WastExecute, WastInvoke,
};

use ontio_wasmjit::chain_api::ChainCtx;
use ontio_wasmjit::executor;
use ontio_wasmjit::executor::Instance;
use std::collections::HashMap;

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

fn run_spec_file(file: &str, test_count: &mut usize) -> Result<Vec<TestDescAndFn>> {
    let mut path = String::new();
    if file == "gas_test.wast" || file == "depth_check.wast" || file == "test.wast" {
        path = format!("tests/{}", file);
    } else {
        path = format!("spectests/{}", file);
    }
    println!("test file path: {}", path);

    let mut contents = String::new();
    File::open(path)?.read_to_string(&mut contents)?;
    let buf = ParseBuffer::new(&contents)?;
    let wast: Wast = parser::parse(&buf)?;
    let mut wasm: Option<Arc<_>> = None;
    let mut instance: Option<Arc<Mutex<Instance>>> = None;
    let mut testcases = Vec::new();

    let mut gas_left = Arc::new(AtomicU64::new(u64::max_value()));
    let gas_cost_map = init_gas_map();

    for iterm in wast.directives {
        match iterm {
            WastDirective::Module(mut module) => {
                wasm = Some(Arc::new(module.encode()?));
            }
            WastDirective::Invoke(invoke) => {
                let args: Vec<_> = invoke.args.iter().map(evaluate_expression).collect();
                if args.iter().any(Option::is_none) {
                    continue;
                }
                let args: Vec<_> = args.into_iter().map(Option::unwrap).collect();

                let name = invoke.name.to_string();

                let instance = instance.clone().unwrap();
                let testfunc = move || {
                    let instance = instance;
                    let res: Vec<_> =
                        executor::execute2(&mut instance.lock().unwrap(), &name, args, false)
                            .into_iter()
                            .collect();
                };
                build_test_cases(&mut testcases, test_count, Box::new(testfunc), invoke, file);
            }
            WastDirective::AssertReturn { exec, results, .. } => match exec {
                WastExecute::Invoke(invoke) => {
                    if let Some(code) = wasm.take() {
                        let mut chain = ChainCtx::default();
                        chain.set_gas_left(u64::max_value());
                        gas_left = chain.get_gas_left();
                        let result = Arc::new(Mutex::new(executor::build_instance(&code, chain)));
                        instance = Some(result.clone());
                    }
                    let instance = instance.clone().unwrap();

                    let args: Vec<_> = invoke.args.iter().map(evaluate_expression).collect();
                    if args.iter().any(Option::is_none) {
                        continue;
                    }
                    let results: Vec<_> = results.iter().map(evaluate_expression).collect();
                    if results.iter().any(Option::is_none) {
                        continue;
                    }
                    let results: Vec<_> = results.into_iter().map(Option::unwrap).collect();
                    let args: Vec<_> = args.into_iter().map(Option::unwrap).collect();

                    let name = invoke.name.to_string();
                    let gas_left = gas_left.clone();
                    let gas_cost_map = gas_cost_map.clone();
                    let test_func = move || {
                        let instance = instance;
                        let gas_left = gas_left;
                        let gas_cost_map = gas_cost_map;
                        let start_gas = gas_left.load(Ordering::Relaxed);

                        let res: Vec<_> =
                            executor::execute2(&mut instance.lock().unwrap(), &name, args, false)
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
                _ => (),
            },
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
    let mut test_name = format!("{:04} {}:{}", *test_count, file, invoke.name);
    if invoke.name.as_bytes().iter().any(|x| *x == 0) {
        test_name = format!("{}:unknown-name", file);
    }

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
    gas_map.insert("gas-add".to_string(), 4);
    gas_map.insert("gas-empty-block".to_string(), 5);
    gas_map.insert("gas-type-i32-value-br".to_string(), 4);
    gas_map.insert("gas-as-br-value".to_string(), 5);
    gas_map.insert("gas-type-i32-value-br-table".to_string(), 5);
    gas_map.insert("gas-set-x".to_string(), 3);
    gas_map.insert("gas-empty-if".to_string(), 3);
    gas_map.insert("gas-type-second-i64".to_string(), 6);
    gas_map.insert("gas-empty-loop".to_string(), 3);
    gas_map.insert("gas-singular-loop".to_string(), 4);
    gas_map.insert("gas-load_at_zero".to_string(), 3);
    gas_map.insert("gas-select_i32".to_string(), 5);
    gas_map.insert("gas-stmt".to_string(), 16);
    gas_map.insert("gas-as-func-first".to_string(), 3);
    gas_map.insert("gas-type-i32".to_string(), 3);
    //    gas_map.insert("depth-type-second-i64".to_string(), 12);
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
            //            let file = "memory_redundancy.wast";
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
