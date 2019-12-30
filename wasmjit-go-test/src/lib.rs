#![cfg_attr(
    feature = "cargo-clippy",
    allow(clippy::missing_safety_doc, clippy::new_without_default)
)]

pub use ontio_wasmjit::chain_api::ChainCtx;
pub use ontio_wasmjit::error::Error;
use ontio_wasmjit::executor::build_module;
pub use ontio_wasmjit::resolver::Resolver;
use ontio_wasmjit_runtime::{ExecMetrics, VMFunctionBody, VMFunctionImport};
use std::ptr;
use std::slice;
pub use wasmjit_capi::{
    bytes_from_vec, bytes_null, bytes_to_boxed_slice, resolver_to_impl_repr, wasmjit_bytes_t,
    wasmjit_chain_context_t, wasmjit_instance_destroy, wasmjit_instance_invoke,
    wasmjit_instantiate, wasmjit_resolver_t, wasmjit_result_err_trap, wasmjit_result_success,
    wasmjit_result_t, wasmjit_slice_t, wasmjit_test_read_wasm, wasmjit_test_read_wasm_file,
};

#[repr(C)]
pub struct wasmjit_return {
    v: u32,
    isnone: u32,
    res: wasmjit_result_t,
}

#[repr(C)]
pub struct wasmjit_body_t {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct wasmjit_import_func_t {
    name: wasmjit_bytes_t,
    body: *mut wasmjit_body_t,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct ImportFunc {
    pub name: String,
    pub body: *const VMFunctionBody,
}

#[no_mangle]
pub unsafe extern "C" fn wasmjit_bytes_from_slice(s: wasmjit_slice_t) -> wasmjit_bytes_t {
    let func_name = slice::from_raw_parts(s.data, s.len as usize);
    //let fname = String::from_utf8(func_name.to_vec()).expect("invalid function name");
    //bytes_from_vec(fname.into_bytes())
    bytes_from_vec(func_name.to_vec())
}

pub struct GoChainResolver {
    imports: Vec<ImportFunc>,
}

impl GoChainResolver {
    pub unsafe fn new(imports: *mut wasmjit_import_func_t, num: u32) -> Self {
        if num == 0 {
            return Self {
                imports: Vec::new(),
            };
        }

        let imports: &[wasmjit_import_func_t] = slice::from_raw_parts_mut(imports, num as usize);
        let imports = imports
            .iter()
            .map(|v| {
                // not get the ownership of imports.
                let name = slice::from_raw_parts_mut(v.name.data, v.name.len as usize).to_vec();
                ImportFunc {
                    name: String::from_utf8(name).unwrap(),
                    body: v.body as *const VMFunctionBody,
                }
            })
            .collect();

        Self { imports: imports }
    }
}

impl Resolver for GoChainResolver {
    fn resolve(&mut self, _module: &str, field: &str) -> Option<VMFunctionImport> {
        for v in &self.imports {
            if v.name == field {
                return Some(VMFunctionImport { body: v.body });
            }
        }
        None
    }
}

/// Implementation of wasmjit_resolver_create_cgo
#[no_mangle]
pub unsafe extern "C" fn wasmjit_go_resolver_create(
    imports: *mut wasmjit_import_func_t,
    num: u32,
) -> *mut wasmjit_resolver_t {
    let res = GoChainResolver::new(imports, num);
    let b1 = Box::new(res) as Box<dyn Resolver>;

    Box::into_raw(Box::new(b1)) as *mut wasmjit_resolver_t
}

fn make_chain() -> ChainCtx {
    let exec_metrics = ExecMetrics::new(u64::max_value(), 1, u64::max_value(), 100_000_u64);
    ChainCtx::new(
        1,
        1u32,
        [1u8; 32],
        [1u8; 32],
        Vec::new(),
        Vec::new(),
        Vec::new(),
        exec_metrics,
        0,
    )
}

/// Implementation of wasmjit_invoke
#[no_mangle]
pub unsafe extern "C" fn wasmjit_invoke(
    name: wasmjit_slice_t,
    resolver: *mut wasmjit_resolver_t,
) -> u32 {
    let mut instance = ptr::null_mut();
    let codebytes = wasmjit_test_read_wasm_file(name);
    let code = wasmjit_slice_t {
        data: codebytes.data,
        len: codebytes.len,
    };

    let res = wasmjit_instantiate(&mut instance, resolver, code);
    if res.kind != wasmjit_result_success {
        panic!("instantiate error");
    }
    //let ctx = make_chain();
    let ctx = Box::into_raw(Box::new(make_chain())) as *mut wasmjit_chain_context_t;

    let _res = wasmjit_instance_invoke(instance, ctx);
    wasmjit_instance_destroy(instance);
    2
}

/// Implementation of wasmjit_invoke_args
#[no_mangle]
pub unsafe extern "C" fn wasmjit_invoke_args(
    name: wasmjit_slice_t,
    n: u32,
    resolver: *mut wasmjit_resolver_t,
) -> wasmjit_return {
    let wasm = wasmjit_test_read_wasm(name);

    let module = build_module(&wasm).unwrap();

    let mut resolver = resolver_to_impl_repr(resolver);
    let mut instance = module.instantiate(&mut **resolver).unwrap();

    let res = instance.execute(make_chain(), "invoke", vec![n as i64]);

    match res {
        Ok(v) => match v {
            Some(ret) => wasmjit_return {
                v: ret as u32,
                isnone: 0,
                res: wasmjit_result_t {
                    kind: wasmjit_result_success,
                    msg: bytes_null(),
                },
            },
            None => wasmjit_return {
                v: 0,
                isnone: 1,
                res: wasmjit_result_t {
                    kind: wasmjit_result_success,
                    msg: bytes_null(),
                },
            },
        },
        Err(err) => match err {
            Error::Trap(msg) => wasmjit_return {
                v: 0,
                isnone: 0,
                res: wasmjit_result_t {
                    kind: wasmjit_result_err_trap,
                    msg: bytes_from_vec(msg.into_bytes()),
                },
            },
            _ => panic!("execute get inner trap. check your code."),
        },
    }
}
