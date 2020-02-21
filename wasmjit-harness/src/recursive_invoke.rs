#![cfg(test)]
mod tests {
    use std::sync::{Arc, Barrier};
    use std::thread;

    use ontio_wasmjit::error::Error;
    use ontio_wasmjit::executor::build_module;
    use ontio_wasmjit::resolver::Resolver;
    use ontio_wasmjit_runtime::builtins::check_host_panic;
    use ontio_wasmjit_runtime::{VMContext, VMFunctionBody, VMFunctionImport};

    fn recursive_fib(n: u32) -> u32 {
        recursive_fib_inner(n, normal_resolve).unwrap()
    }

    fn recursive_fib_panic(n: u32) -> Result<u32, Error> {
        recursive_fib_inner(n, panic_resolve)
    }

    fn recursive_fib_inner(n: u32, mut resolver: impl Resolver) -> Result<u32, Error> {
        /*
                   extern "C" {
                   fn ontio_invoke(n: u32) -> u32;
                   }

        #[no_mangle]
        pub extern "C" fn invoke(n: u32) -> u32 {
        if n == 0 || n == 1 {
        return 1
        } else  {
        return unsafe {ontio_invoke(n - 1) + ontio_invoke(n - 2)}
        }
        }
                 */

        let wat = r#"
	(module
	 (type $t0 (func (param i32) (result i32)))
	 (import "env" "ontio_invoke" (func $ontio_invoke (type $t0)))
	 (func $invoke (export "invoke") (type $t0) (param $p0 i32) (result i32)
	  (local $l0 i32)
	  i32.const 1
	  set_local $l0
	  block $B0
	  get_local $p0
	  i32.const 2
	  i32.lt_u
	  br_if $B0
	  get_local $p0
	  i32.const -1
	  i32.add
	  call $ontio_invoke
	  get_local $p0
	  i32.const -2
	  i32.add
	  call $ontio_invoke
	  i32.add
	  set_local $l0
	  end
	  get_local $l0))
	  "#;

        let wasm = wat::parse_str(wat).unwrap();
        let module = build_module(&wasm).unwrap();

        let mut instance = module.instantiate(&mut resolver).unwrap();

        instance
            .execute(super::super::make_chain(), "invoke", vec![n as i64])
            .map(|val| val.unwrap() as u32)
    }

    fn normal_resolve(_module: &str, field: &str) -> Option<VMFunctionImport> {
        match field {
            "ontio_invoke" => Some(VMFunctionImport {
                body: ontio_invoke as *const VMFunctionBody,
            }),
            _ => None,
        }
    }

    fn panic_resolve(_module: &str, field: &str) -> Option<VMFunctionImport> {
        match field {
            "ontio_invoke" => Some(VMFunctionImport {
                body: ontio_invoke_host_panic as *const VMFunctionBody,
            }),
            _ => None,
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn ontio_invoke(vmctx: *mut VMContext, n: u32) -> u32 {
        println!("call normal ontio invoke: {}", n);
        check_host_panic((&mut *vmctx).instance(), |_| Ok(recursive_fib(n)))
    }

    #[no_mangle]
    pub unsafe extern "C" fn ontio_invoke_host_panic(vmctx: *mut VMContext, n: u32) -> u32 {
        println!("call panic ontio invoke: {}", n);
        check_host_panic((&mut *vmctx).instance(), |_| {
            if n == 0 || n == 1 {
                panic!("panic at 0")
            }

            Ok(recursive_fib_panic(n).unwrap_or_else(|e| panic!(e)))
        })
    }

    fn fib(n: u32) -> u32 {
        if n == 0 || n == 1 {
            1
        } else {
            fib(n - 1) + fib(n - 2)
        }
    }

    #[test]
    fn test_recursive_invoke() {
        for i in 0..10 {
            assert_eq!(fib(i), recursive_fib(i));
        }
    }

    #[test]
    fn test_invoke_panic() {
        let n = 10;

        recursive_fib_panic(n).expect_err("should panic");
    }

    #[test]
    fn test_concurrent_panic() {
        const N: usize = 10;

        let mut handles = Vec::with_capacity(N);
        let barrier = Arc::new(Barrier::new(N));
        for _ in 0..N {
            let c = barrier.clone();
            handles.push(thread::spawn(move || {
                for _ in 0..100 {
                    c.wait();
                    recursive_fib_panic(10).expect_err("should panic");
                }
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }
}
