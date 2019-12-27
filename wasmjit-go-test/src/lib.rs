#![cfg_attr(
    feature = "cargo-clippy",
    allow(clippy::missing_safety_doc, clippy::new_without_default)
)]

pub use ontio_wasmjit::resolver::Resolver;
use ontio_wasmjit_runtime::builtins::{check_host_panic, check_internel_panic, wasmjit_trap};
use ontio_wasmjit_runtime::{wasmjit_unwind, VMContext, VMFunctionBody, VMFunctionImport};
use std::ptr;
use std::slice;
pub use wasmjit_capi::{
    bytes_from_vec, bytes_to_boxed_slice, wasmjit_bytes_t, wasmjit_instantiate, wasmjit_resolver_t,
    wasmjit_result_err_trap, wasmjit_result_success, wasmjit_result_t, wasmjit_slice_t,
    wasmjit_test_read_wasm_file,
};

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
    let fname = String::from_utf8(func_name.to_vec()).expect("invalid function name");
    bytes_from_vec(fname.into_bytes())
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
            .map(|v| ImportFunc {
                name: (String::from_utf8_lossy(&bytes_to_boxed_slice(wasmjit_bytes_t {
                    data: v.name.data,
                    len: v.name.len,
                })))
                .to_string(),
                body: v.body as *const VMFunctionBody,
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

/// Implementation of wasmjit_invoke
#[no_mangle]
pub unsafe extern "C" fn wasmjit_invoke(
    name: wasmjit_slice_t,
    resolver: *mut wasmjit_resolver_t,
) -> Result<(), String> {
    let mut instance = ptr::null_mut();
    let codebytes = wasmjit_test_read_wasm_file(name);
    let code = wasmjit_slice_t {
        data: codebytes.data,
        len: codebytes.len,
    };

    wasmjit_instantiate(&mut instance, resolver, code);
    Ok(())
}
