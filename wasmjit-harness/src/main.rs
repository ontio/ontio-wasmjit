#![feature(test)]
extern crate test;

use std::env;
use std::fs::File;
use std::io::Read;
use std::sync::{Arc, Mutex};

use anyhow::Result;
use glob::glob;
use test::{ShouldPanic, TestDesc, TestDescAndFn, TestFn, TestName, TestType};
use wast::{
    parser::{self, ParseBuffer},
    Expression, Instruction, Wast, WastDirective, WastExecute,
};

use ontio_wasmjit::chain_api::ChainCtx;
use ontio_wasmjit::executor;
use ontio_wasmjit::executor::Instance;

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
    let path = format!("spectests/{}", file);
    println!("ffff: {}", path);

    let mut contents = String::new();
    File::open(path)?.read_to_string(&mut contents)?;
    let buf = ParseBuffer::new(&contents)?;
    let wast: Wast = parser::parse(&buf)?;
    let mut wasm: Option<Arc<_>> = None;
    let mut instance: Option<Arc<Mutex<Instance>>> = None;
    let mut testcases = Vec::new();
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
                executor::execute2(&mut instance.lock().unwrap(), &name, args, false);
            }
            WastDirective::AssertReturn { exec, results, .. } => match exec {
                WastExecute::Invoke(invoke) => {
                    if let Some(code) = wasm.take() {
                        let chain = ChainCtx::default();

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

                    let testfunc = move || {
                        let instance = instance;
                        let res: Vec<_> =
                            executor::execute2(&mut instance.lock().unwrap(), &name, args, false)
                                .into_iter()
                                .collect();
                        assert_eq!(res, results);
                    };

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
                        testfn: TestFn::DynTestFn(Box::new(testfunc)),
                    });
                }
                _ => (),
            },
            _ => (),
        }
    }

    Ok(testcases)
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

    let mut args: Vec<String> = env::args().collect();
    args.push("--test-threads=1".to_string());
    test::test_main(&args, funcs, None);
}
