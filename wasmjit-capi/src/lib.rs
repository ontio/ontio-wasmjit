#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]

use std::slice;

use ontio_wasmjit::chain_api::ChainCtx;
use ontio_wasmjit::chain_api::{Address, ChainResolver};
use ontio_wasmjit::resolver::Resolver;
use ontio_wasmjit_runtime::VMContext;

use cranelift_wasm::DefinedMemoryIndex;

pub type wasmjit_result_kind = u32;
const wasmjit_result_success: wasmjit_result_kind = 0;
const wasmjit_result_err_internal: wasmjit_result_kind = 1;
const wasmjit_result_err_compile: wasmjit_result_kind = 2;
const wasmjit_result_err_link: wasmjit_result_kind = 3;
const wasmjit_result_err_trap: wasmjit_result_kind = 4;

#[repr(C)]
pub struct wasmjit_result_t {
    kind: wasmjit_result_kind,
    msg: wasmjit_bytes_t,
}

#[repr(C)]
pub struct wasmjit_bytes_t {
    pub data: *mut u8,
    pub len: u32,
}

fn bytes_null() -> wasmjit_bytes_t {
    wasmjit_bytes_t {
        data: std::ptr::null_mut(),
        len: 0,
    }
}

fn bytes_from_vec(data: Vec<u8>) -> wasmjit_bytes_t {
    let bytes: Box<[u8]> = data.into_boxed_slice();
    let len = bytes.len() as u32;
    let data = Box::into_raw(bytes) as *mut u8;
    wasmjit_bytes_t { data, len }
}

#[no_mangle]
pub extern "C" fn wasmjit_bytes_new(len: u32) -> wasmjit_bytes_t {
    bytes_from_vec(vec![0; len as usize])
}

#[no_mangle]
pub extern "C" fn wasmjit_bytes_destroy(bytes: wasmjit_bytes_t) {
    unsafe {
        let raw = slice::from_raw_parts_mut(bytes.data, bytes.len as usize);
        Box::from_raw(raw);
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct wasmjit_slice_t {
    pub data: *mut u8,
    pub len: u32,
}

#[repr(C)]
pub struct wasmjit_resolver_t {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct wasmjit_instance_t {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct wasmjit_vmctx_t {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct wasmjit_module_t {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct wasmjit_chain_context_t {
    _unused: [u8; 0],
}

pub type h256_t = [u8; 32];

pub type address_t = [u8; 20];

unsafe fn addrs_from_slice(callers: wasmjit_slice_t) -> Vec<Address> {
    let buf = slice::from_raw_parts(callers.data, callers.len as usize);
    let mut callers = Vec::with_capacity(callers.len as usize / 20);

    for addr in buf.chunks_exact(20) {
        let mut caller = [0; 20];
        caller[0..].copy_from_slice(addr);
        callers.push(caller);
    }

    callers
}

unsafe fn convert_vmctx<'a>(ctx: *mut wasmjit_vmctx_t) -> &'a mut VMContext {
    let ctx = &mut *(ctx as *mut VMContext);
    ctx
}

#[no_mangle]
pub extern "C" fn wasmjit_vmctx_memory(
    ctx: *mut wasmjit_vmctx_t,
    result: &mut wasmjit_slice_t,
) -> wasmjit_result_t {
    let ctx = unsafe { convert_vmctx(ctx) };
    let mem = ctx
        .instance()
        .memory_slice_mut(DefinedMemoryIndex::from_u32(0));
    match mem {
        Some(mem) => {
            result.data = mem as *mut [u8] as *mut u8;
            result.len = mem.len() as u32;
            wasmjit_result_t {
                kind: wasmjit_result_success,
                msg: bytes_null(),
            }
        }
        None => wasmjit_result_t {
            kind: wasmjit_result_err_trap,
            msg: bytes_from_vec(b"undefined memory".to_vec()),
        },
    }
}

pub type u8x6 = [u8; 4];

#[no_mangle]
pub extern "C" fn abi_test(a1: u32, a2: u32, a3: u64, a4: u64, a5: u64, a6: u64, a7: &u8x6) {
    println!("args: {:?}", (a1, a2, a3, a4, a5, a6, a7))
}

#[no_mangle]
pub extern "C" fn wasmjit_chain_context_create(
    height: u32,
    blockhash: &mut h256_t,
    timestamp: u64,
    txhash: &mut h256_t,
    self_address: &mut address_t,
    callers_raw: wasmjit_slice_t,
    witness_raw: wasmjit_slice_t,
    input_raw: wasmjit_slice_t,
    gas_left: u64,
    service_index: u64,
) -> *mut wasmjit_chain_context_t {
    println!(
        "args: {:?}",
        (
            height,
            &blockhash,
            timestamp,
            &txhash,
            &self_address,
            &callers_raw,
            &witness_raw,
            &input_raw,
            gas_left,
            service_index
        )
    );

    assert_eq!(callers_raw.len % 20, 0);
    assert_eq!(witness_raw.len % 20, 0);

    let (callers, witness, input) = unsafe {
        (
            addrs_from_slice(callers_raw),
            addrs_from_slice(witness_raw),
            slice::from_raw_parts(input_raw.data, input_raw.len as usize).to_vec(),
        )
    };

    let mut ctx = ChainCtx::new(
        timestamp,
        height,
        *blockhash,
        *txhash,
        *self_address,
        callers,
        witness,
        input,
        Vec::new(),
        service_index,
    );

    ctx.set_gas_left(gas_left);
    Box::into_raw(Box::new(ctx)) as *mut wasmjit_chain_context_t
}

#[no_mangle]
pub extern "C" fn wasmjit_chain_context_push_caller(
    ctx: *mut wasmjit_chain_context_t,
    caller: address_t,
) {
    let ctx = unsafe { convert_chain_ctx(ctx) };
    ctx.push_caller(caller);
}

#[no_mangle]
pub extern "C" fn wasmjit_chain_context_pop_caller(
    ctx: *mut wasmjit_chain_context_t,
    result: &mut address_t,
) {
    unimplemented!()
}

unsafe fn convert_chain_ctx<'a>(ctx: *mut wasmjit_chain_context_t) -> &'a mut ChainCtx {
    let ctx = &mut *(ctx as *mut ChainCtx);
    ctx
}

#[no_mangle]
pub extern "C" fn wasmjit_chain_context_get_gas(ctx: *mut wasmjit_chain_context_t) -> u64 {
    let ctx = unsafe { convert_chain_ctx(ctx) };
    ctx.gas_left()
}

#[no_mangle]
pub extern "C" fn wasmjit_chain_context_set_gas(ctx: *mut wasmjit_chain_context_t, gas: u64) {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn wasmjit_chain_context_take_output(
    ctx: *mut wasmjit_chain_context_t,
) -> wasmjit_bytes_t {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn wasmjit_compile(
    module: *mut *mut wasmjit_module_t,
    wasm: wasmjit_slice_t,
) -> wasmjit_result_t {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn wasmjit_module_destroy(module: *mut wasmjit_module_t) {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn wasmjit_module_instantiate(
    module: *const wasmjit_module_t,
    resolver: *mut wasmjit_resolver_t,
    instance: *mut *mut wasmjit_instance_t,
) -> wasmjit_result_t {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn wasmjit_instantiate(
    instance: *mut *mut wasmjit_instance_t,
    wasm: wasmjit_slice_t,
) -> wasmjit_result_t {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn wasmjit_instance_invoke(
    instance: *mut wasmjit_instance_t,
    ctx: *mut wasmjit_chain_context_t,
) -> wasmjit_result_t {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn wasmjit_instance_destroy(instance: *mut wasmjit_instance_t) {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn wasmjit_resolver_destroy(resolver: *mut wasmjit_resolver_t) {
    unsafe {
        let resolver = resolver as *mut Box<dyn Resolver>;
        let _ = Box::from_raw(resolver);
    }
}

#[no_mangle]
pub extern "C" fn wasmjit_resolver_create_cgo() -> *mut wasmjit_resolver_t {
    let res = ChainResolver;
    let b1 = Box::new(res) as Box<dyn Resolver>;

    Box::into_raw(Box::new(b1)) as *mut wasmjit_resolver_t
}

#[no_mangle]
pub extern "C" fn wasmjit_validate(wasm: wasmjit_slice_t) -> bool {
    unimplemented!()
}
