use cranelift_wasm::DefinedMemoryIndex;
use ontio_wasmjit::chain_api::{ChainCtx, ChainResolver};
pub use ontio_wasmjit::resolver::Resolver;
use ontio_wasmjit::utils;
use ontio_wasmjit_runtime::builtins::{catch_host_panic, check_host_panic};
use ontio_wasmjit_runtime::{VMContext, VMFunctionBody, VMFunctionImport};
use std::ptr;
pub use wasmjit_capi::{
    bytes_from_vec, bytes_null, bytes_to_boxed_slice, convert_chain_ctx, convert_vmctx,
    wasmjit_bytes_new, wasmjit_bytes_t, wasmjit_chain_context_t, wasmjit_instance_destroy,
    wasmjit_instance_invoke, wasmjit_instance_t, wasmjit_instantiate, wasmjit_resolver_t,
    wasmjit_result_err_internal, wasmjit_result_err_trap, wasmjit_result_success, wasmjit_result_t,
    wasmjit_slice_t, wasmjit_vmctx_chainctx, wasmjit_vmctx_memory, wasmjit_vmctx_t,
};

#[repr(C)]
pub struct wasmjit_u32 {
    v: u32,
    res: wasmjit_result_t,
}

#[repr(C)]
pub struct wasmjit_u64 {
    v: u64,
    res: wasmjit_result_t,
}

#[repr(C)]
pub struct wasmjit_buffer {
    buffer: wasmjit_bytes_t,
    res: wasmjit_result_t,
}

extern "C" {
    fn ontio_debug_cgo(vmctx: *mut wasmjit_vmctx_t, data_ptr: u32, l: u32) -> wasmjit_result_t;
    fn ontio_notify_cgo(vmctx: *mut wasmjit_vmctx_t, data_ptr: u32, l: u32) -> wasmjit_result_t;
    fn ontio_storage_read_cgo(
        vmctx: *mut wasmjit_vmctx_t,
        key_ptr: u32,
        klen: u32,
        val: u32,
        vlen: u32,
        offset: u32,
    ) -> wasmjit_u32;
    fn ontio_storage_write_cgo(
        vmctx: *mut wasmjit_vmctx_t,
        key_ptr: u32,
        klen: u32,
        val: u32,
        vlen: u32,
    ) -> wasmjit_result_t;
    fn ontio_storage_delete_cgo(
        vmctx: *mut wasmjit_vmctx_t,
        keyPtr: u32,
        klen: u32,
    ) -> wasmjit_result_t;
    fn ontio_contract_create_cgo(
        vmctx: *mut wasmjit_vmctx_t,
        code_ptr: u32,
        code_len: u32,
        vm_type: u32,
        name_ptr: u32,
        name_len: u32,
        ver_ptr: u32,
        ver_len: u32,
        author_ptr: u32,
        author_len: u32,
        email_ptr: u32,
        email_len: u32,
        desc_ptr: u32,
        desc_len: u32,
        newaddress_ptr: u32,
    ) -> wasmjit_u32;
    fn ontio_contract_migrate_cgo(
        vmctx: *mut wasmjit_vmctx_t,
        code_ptr: u32,
        code_len: u32,
        vm_type: u32,
        name_ptr: u32,
        name_len: u32,
        ver_ptr: u32,
        ver_len: u32,
        author_ptr: u32,
        author_len: u32,
        email_ptr: u32,
        email_len: u32,
        desc_ptr: u32,
        desc_len: u32,
        newaddress_ptr: u32,
    ) -> wasmjit_u32;
    fn ontio_contract_destroy_cgo(vmctx: *mut wasmjit_vmctx_t) -> wasmjit_result_t;
    fn ontio_call_contract_cgo(
        vmctx: *mut wasmjit_vmctx_t,
        contract_addr: u32,
        input_ptr: u32,
        input_len: u32,
    ) -> wasmjit_result_t;
}

/// Implementation of ontio_debug api
#[no_mangle]
pub unsafe extern "C" fn ontio_debug(vmctx: *mut VMContext, data_ptr: u32, l: u32) {
    check_host_panic(|| {
        let res = ontio_debug_cgo(vmctx as *mut wasmjit_vmctx_t, data_ptr, l);

        if res.kind != wasmjit_result_success {
            panic!(
                (std::string::String::from_utf8_lossy(&bytes_to_boxed_slice(res.msg))).to_string()
            );
        }
    })
}

/// Implementation of ontio_call_contract
#[no_mangle]
pub unsafe extern "C" fn ontio_call_contract(
    vmctx: *mut VMContext,
    contract_addr: u32,
    input_ptr: u32,
    inputlen: u32,
) -> u32 {
    check_host_panic(|| {
        let res = ontio_call_contract_cgo(
            vmctx as *mut wasmjit_vmctx_t,
            contract_addr,
            input_ptr,
            inputlen,
        );
        if res.kind != wasmjit_result_success {
            panic!(
                (std::string::String::from_utf8_lossy(&bytes_to_boxed_slice(res.msg))).to_string()
            );
        }
        let host = (&mut *vmctx).host_state();
        let chain = host.downcast_ref::<ChainCtx>().unwrap();
        chain.call_output_len()
    })
}

/// Implementation of ontio_notify api
#[no_mangle]
pub unsafe extern "C" fn ontio_notify(vmctx: *mut VMContext, ptr: u32, l: u32) {
    check_host_panic(|| {
        let res = ontio_notify_cgo(vmctx as *mut wasmjit_vmctx_t, ptr, l);
        if res.kind != wasmjit_result_success {
            panic!(
                (std::string::String::from_utf8_lossy(&bytes_to_boxed_slice(res.msg))).to_string()
            );
        }
    })
}

/// Implementation of ontio_storage_read
#[no_mangle]
pub unsafe extern "C" fn ontio_storage_read(
    vmctx: *mut VMContext,
    key_ptr: u32,
    klen: u32,
    val: u32,
    vlen: u32,
    offset: u32,
) -> u32 {
    check_host_panic(|| {
        let jit_u32 = ontio_storage_read_cgo(
            vmctx as *mut wasmjit_vmctx_t,
            key_ptr,
            klen,
            val,
            vlen,
            offset,
        );
        if jit_u32.res.kind != wasmjit_result_success {
            panic!(
                (std::string::String::from_utf8_lossy(&bytes_to_boxed_slice(jit_u32.res.msg)))
                    .to_string()
            );
        }
        jit_u32.v
    })
}

/// Implementation of ontio_storage_write
#[no_mangle]
pub unsafe extern "C" fn ontio_storage_write(
    vmctx: *mut VMContext,
    key_ptr: u32,
    klen: u32,
    val: u32,
    vlen: u32,
) {
    check_host_panic(|| {
        let res = ontio_storage_write_cgo(vmctx as *mut wasmjit_vmctx_t, key_ptr, klen, val, vlen);
        if res.kind != wasmjit_result_success {
            panic!(
                (std::string::String::from_utf8_lossy(&bytes_to_boxed_slice(res.msg))).to_string()
            );
        }
    })
}

/// Implementation of ontio_storage_delete
#[no_mangle]
pub unsafe extern "C" fn ontio_storage_delete(vmctx: *mut VMContext, key_ptr: u32, klen: u32) {
    check_host_panic(|| {
        let res = ontio_storage_delete_cgo(vmctx as *mut wasmjit_vmctx_t, key_ptr, klen);
        if res.kind != wasmjit_result_success {
            panic!(
                (std::string::String::from_utf8_lossy(&bytes_to_boxed_slice(res.msg))).to_string()
            );
        }
    })
}

/// Implementation of ontio_contract_create
#[no_mangle]
pub unsafe extern "C" fn ontio_contract_create(
    vmctx: *mut VMContext,
    code_ptr: u32,
    code_len: u32,
    vm_type: u32,
    name_ptr: u32,
    name_len: u32,
    ver_ptr: u32,
    ver_len: u32,
    author_ptr: u32,
    author_len: u32,
    email_ptr: u32,
    email_len: u32,
    desc_ptr: u32,
    desc_len: u32,
    newaddress_ptr: u32,
) -> u32 {
    check_host_panic(|| {
        let jit_u32 = ontio_contract_create_cgo(
            vmctx as *mut wasmjit_vmctx_t,
            code_ptr,
            code_len,
            vm_type,
            name_ptr,
            name_len,
            ver_ptr,
            ver_len,
            author_ptr,
            author_len,
            email_ptr,
            email_len,
            desc_ptr,
            desc_len,
            newaddress_ptr,
        );

        if jit_u32.res.kind != wasmjit_result_success {
            panic!(
                (std::string::String::from_utf8_lossy(&bytes_to_boxed_slice(jit_u32.res.msg)))
                    .to_string()
            );
        }

        jit_u32.v
    })
}

/// Implementation of ontio_contract_migrate
#[no_mangle]
pub unsafe extern "C" fn ontio_contract_migrate(
    vmctx: *mut VMContext,
    code_ptr: u32,
    code_len: u32,
    vm_type: u32,
    name_ptr: u32,
    name_len: u32,
    ver_ptr: u32,
    ver_len: u32,
    author_ptr: u32,
    author_len: u32,
    email_ptr: u32,
    email_len: u32,
    desc_ptr: u32,
    desc_len: u32,
    newaddress_ptr: u32,
) -> u32 {
    check_host_panic(|| {
        let jit_u32 = ontio_contract_migrate_cgo(
            vmctx as *mut wasmjit_vmctx_t,
            code_ptr,
            code_len,
            vm_type,
            name_ptr,
            name_len,
            ver_ptr,
            ver_len,
            author_ptr,
            author_len,
            email_ptr,
            email_len,
            desc_ptr,
            desc_len,
            newaddress_ptr,
        );
        if jit_u32.res.kind != wasmjit_result_success {
            panic!(
                (std::string::String::from_utf8_lossy(&bytes_to_boxed_slice(jit_u32.res.msg)))
                    .to_string()
            );
        }
        jit_u32.v
    })
}

/// Implementation of ontio_contract_destroy
#[no_mangle]
pub unsafe extern "C" fn ontio_contract_destroy(vmctx: *mut VMContext) {
    check_host_panic(|| {
        let res = ontio_contract_destroy_cgo(vmctx as *mut wasmjit_vmctx_t);
        if res.kind != wasmjit_result_success {
            panic!(
                (std::string::String::from_utf8_lossy(&bytes_to_boxed_slice(res.msg))).to_string()
            );
        }

        let ctx = wasmjit_vmctx_chainctx(vmctx as *mut wasmjit_vmctx_t);
        let ctx_r = convert_chain_ctx(ctx);
        ctx_r.set_from_return();
        panic!("ontio_return_special_sig");
    })
}

pub struct OntoChainResolver {
    inner: ChainResolver,
}

impl OntoChainResolver {
    pub fn new() -> Self {
        let cr = ChainResolver;
        Self { inner: cr }
    }
}

impl Resolver for OntoChainResolver {
    fn resolve(&mut self, module: &str, field: &str) -> Option<VMFunctionImport> {
        match field {
            "ontio_debug" => Some(VMFunctionImport {
                body: ontio_debug as *const VMFunctionBody,
                vmctx: ptr::null_mut(),
            }),
            "ontio_call_contract" => Some(VMFunctionImport {
                body: ontio_call_contract as *const VMFunctionBody,
                vmctx: ptr::null_mut(),
            }),
            "ontio_notify" => Some(VMFunctionImport {
                body: ontio_notify as *const VMFunctionBody,
                vmctx: ptr::null_mut(),
            }),
            "ontio_storage_read" => Some(VMFunctionImport {
                body: ontio_storage_read as *const VMFunctionBody,
                vmctx: ptr::null_mut(),
            }),
            "ontio_storage_write" => Some(VMFunctionImport {
                body: ontio_storage_write as *const VMFunctionBody,
                vmctx: ptr::null_mut(),
            }),
            "ontio_storage_delete" => Some(VMFunctionImport {
                body: ontio_storage_delete as *const VMFunctionBody,
                vmctx: ptr::null_mut(),
            }),
            "ontio_contract_create" => Some(VMFunctionImport {
                body: ontio_contract_create as *const VMFunctionBody,
                vmctx: ptr::null_mut(),
            }),
            "ontio_contract_migrate" => Some(VMFunctionImport {
                body: ontio_contract_migrate as *const VMFunctionBody,
                vmctx: ptr::null_mut(),
            }),
            "ontio_contract_destroy" => Some(VMFunctionImport {
                body: ontio_contract_destroy as *const VMFunctionBody,
                vmctx: ptr::null_mut(),
            }),
            _ => self.inner.resolve(module, field),
        }
    }
}

/// Implementation of wasmjit_resolver_create_cgo
#[no_mangle]
pub extern "C" fn wasmjit_onto_resolver_create() -> *mut wasmjit_resolver_t {
    let res = OntoChainResolver::new();
    let b1 = Box::new(res) as Box<dyn Resolver>;

    Box::into_raw(Box::new(b1)) as *mut wasmjit_resolver_t
}

/// Implementation of wasmjit_read_memory
#[no_mangle]
pub unsafe extern "C" fn wasmjit_memory_len(vmctx: *mut wasmjit_vmctx_t) -> wasmjit_u32 {
    let mut memory = wasmjit_slice_t {
        data: ptr::null_mut(),
        len: 0,
    };
    let result = wasmjit_vmctx_memory(vmctx, &mut memory);
    wasmjit_u32 {
        v: memory.len,
        res: result,
    }
}

/// Implementation of wasmjit_read_memory
#[no_mangle]
pub unsafe extern "C" fn wasmjit_read_memory(
    vmctx: *mut wasmjit_vmctx_t,
    data_buffer: *mut u8,
    data_ptr: u32,
    data_len: u32,
) -> wasmjit_result_t {
    if data_len == 0 {
        return wasmjit_result_t {
            kind: wasmjit_result_success,
            msg: bytes_null(),
        };
    }

    let mut memory = wasmjit_slice_t {
        data: ptr::null_mut(),
        len: 0,
    };

    let result = wasmjit_vmctx_memory(vmctx, &mut memory);
    if result.kind != wasmjit_result_success {
        return result;
    }

    if memory.len < data_ptr + data_len {
        return wasmjit_result_t {
            kind: wasmjit_result_err_trap,
            msg: bytes_from_vec(b"wasmjit_read_memory access out of bound.".to_vec()),
        };
    }

    let memory = std::slice::from_raw_parts_mut(memory.data, memory.len as usize);
    let buff = std::slice::from_raw_parts_mut(data_buffer, data_len as usize);
    buff.copy_from_slice(&memory[data_ptr as usize..(data_ptr + data_len) as usize]);

    wasmjit_result_t {
        kind: wasmjit_result_success,
        msg: bytes_null(),
    }
}

/// Implementation of wasmjit_read_memory
#[no_mangle]
pub unsafe extern "C" fn wasmjit_write_memory(
    vmctx: *mut wasmjit_vmctx_t,
    data_buffer: *mut u8,
    data_ptr: u32,
    data_len: u32,
) -> wasmjit_result_t {
    // here need catch the access out of bound panic.
    if data_len == 0 {
        return wasmjit_result_t {
            kind: wasmjit_result_success,
            msg: bytes_null(),
        };
    }

    let mut memory = wasmjit_slice_t {
        data: ptr::null_mut(),
        len: 0,
    };

    let result = wasmjit_vmctx_memory(vmctx, &mut memory);
    if result.kind != wasmjit_result_success {
        return result;
    }

    if memory.len < data_ptr + data_len {
        return wasmjit_result_t {
            kind: wasmjit_result_err_trap,
            msg: bytes_from_vec(b"wasmjit_write_memory access out of bound.".to_vec()),
        };
    }

    let buff = std::slice::from_raw_parts_mut(data_buffer, data_len as usize);
    let memory = std::slice::from_raw_parts_mut(memory.data, memory.len as usize);
    memory[data_ptr as usize..(data_ptr + data_len) as usize].copy_from_slice(buff);

    wasmjit_result_t {
        kind: wasmjit_result_success,
        msg: bytes_null(),
    }
}

/// Implementation of wasmjit_contruct_result_t
#[no_mangle]
pub unsafe extern "C" fn wasmjit_construct_result_t(
    data_buffer: *mut u8,
    data_len: u32,
    kind_t: u32,
) -> wasmjit_result_t {
    let v = std::slice::from_raw_parts(data_buffer, data_len as usize).to_vec();
    wasmjit_result_t {
        kind: kind_t,
        msg: bytes_from_vec(v),
    }
}

/// Implementation of wasmjit_service_index
#[no_mangle]
pub unsafe extern "C" fn wasmjit_service_index(vmctx: *mut wasmjit_vmctx_t) -> u64 {
    let ctx = wasmjit_vmctx_chainctx(vmctx);
    let ctx_r = convert_chain_ctx(ctx);

    ctx_r.service_index()
}

/// Implementation of wasmjit_invoke
#[no_mangle]
pub unsafe extern "C" fn wasmjit_invoke(
    code: wasmjit_slice_t,
    chainctx: *mut wasmjit_chain_context_t,
) -> wasmjit_result_t {
    let mut instance = ptr::null_mut();
    let resolver = wasmjit_onto_resolver_create();

    let res = wasmjit_instantiate(&mut instance, resolver, code);
    if res.kind != wasmjit_result_success {
        return res;
    }

    let result = wasmjit_instance_invoke(instance, chainctx);
    // should destroy the instance after take output.
    result
}
