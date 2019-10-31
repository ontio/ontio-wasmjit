use crate::resolver::Resolver;
use cranelift_wasm::DefinedMemoryIndex;
use ontio_wasmjit_runtime::{VMContext, VMFunctionBody, VMFunctionImport};
use std::ptr;

pub type Address = [u8; 20];
pub type H256 = [u8; 32];

pub struct ChainCtx {
    height: u32,
    block_hash: H256,
    timestamp: u64,
    tx_hash: H256,
    callers: Vec<Address>,
    witness: Vec<Address>,
    input: Vec<u8>,
    call_output: Vec<u8>,
}

impl ChainCtx {
    pub fn new(timestamp: u64, height: u32) -> Self {
        Self {
            height,
            block_hash: [0; 32],
            timestamp,
            tx_hash: [0; 32],
            callers: Vec::new(),
            witness: Vec::new(),
            input: Vec::new(),
            call_output: Vec::new(),
        }
    }
}
/// Implementation of ontio_timestamp api
#[no_mangle]
pub unsafe extern "C" fn ontio_timestamp(vmctx: *mut VMContext) -> u64 {
    let host = (&mut *vmctx).host_state();
    let chain = host.downcast_ref::<ChainCtx>().unwrap();
    chain.timestamp
}

/// Implementation of ontio_block_height api
#[no_mangle]
pub unsafe extern "C" fn ontio_block_height(vmctx: *mut VMContext) -> u32 {
    let host = (&mut *vmctx).host_state();
    let chain = host.downcast_ref::<ChainCtx>().unwrap();
    chain.height
}

/// Implementation of ontio_input_length api
#[no_mangle]
pub unsafe extern "C" fn ontio_input_length(vmctx: *mut VMContext) -> u32 {
    let host = (&mut *vmctx).host_state();
    let chain = host.downcast_ref::<ChainCtx>().unwrap();
    chain.input.len() as u32
}

/// Implementation of ontio_get_input api
#[no_mangle]
pub unsafe extern "C" fn ontio_get_input(vmctx: *mut VMContext, input_ptr: u32) {
    let host = (&mut *vmctx).host_state();
    let chain = host.downcast_ref::<ChainCtx>().unwrap();
    let instance = (&mut *vmctx).instance();
    // FIXME: check memory 0 exist
    let memory = instance.memory_slice_mut(DefinedMemoryIndex::from_u32(0));
    // FIXME: check memory bounds
    let start = input_ptr as usize;
    memory[start..start + chain.input.len()].copy_from_slice(&chain.input);
}

/// Implementation of ontio_sha256 api
#[no_mangle]
pub unsafe extern "C" fn ontio_sha256(
    vmctx: *mut VMContext,
    data_ptr: u32,
    len: u32,
    out_ptr: u32,
) {
    let instance = (&mut *vmctx).instance();
    //TODO
}

/*
const SIGNATURES: [(&str, &[ValueType], Option<ValueType>); 24] = [
    ("ontio_call_output_length", &[], Some(ValueType::I32)),
    ("ontio_get_call_output", &[ValueType::I32], None),
    ("ontio_self_address", &[ValueType::I32], None),
    ("ontio_caller_address", &[ValueType::I32], None),
    ("ontio_entry_address", &[ValueType::I32], None),
    ("ontio_check_witness", &[ValueType::I32], Some(ValueType::I32)),
    ("ontio_current_blockhash", &[ValueType::I32], Some(ValueType::I32)),
    ("ontio_current_txhash", &[ValueType::I32], Some(ValueType::I32)),
    ("ontio_return", &[ValueType::I32; 2], None),
    ("ontio_panic", &[ValueType::I32; 2], None),
    ("ontio_notify", &[ValueType::I32; 2], None),
    ("ontio_call_contract", &[ValueType::I32; 3], Some(ValueType::I32)),
    ("ontio_contract_create", &[ValueType::I32; 14], Some(ValueType::I32)),
    ("ontio_contract_migrate", &[ValueType::I32; 14], Some(ValueType::I32)),
    ("ontio_contract_destroy", &[], None),
    ("ontio_storage_read", &[ValueType::I32; 5], Some(ValueType::I32)),
    ("ontio_storage_write", &[ValueType::I32; 4], None),
    ("ontio_storage_delete", &[ValueType::I32; 2], None),
    ("ontio_debug", &[ValueType::I32; 2], None),
    ("ontio_sha256", &[ValueType::I32; 3], None),
];
*/

pub struct ChainResolver;

impl Resolver for ChainResolver {
    fn resolve(&mut self, _module: &str, field: &str) -> Option<VMFunctionImport> {
        match field {
            "ontio_timestamp" => Some(VMFunctionImport {
                body: ontio_timestamp as *const VMFunctionBody,
                vmctx: ptr::null_mut(),
            }),
            "ontio_block_height" => Some(VMFunctionImport {
                body: ontio_block_height as *const VMFunctionBody,
                vmctx: ptr::null_mut(),
            }),
            "ontio_input_length" => Some(VMFunctionImport {
                body: ontio_input_length as *const VMFunctionBody,
                vmctx: ptr::null_mut(),
            }),
            "ontio_get_input" => Some(VMFunctionImport {
                body: ontio_get_input as *const VMFunctionBody,
                vmctx: ptr::null_mut(),
            }),
            _ => None,
        }
    }
}
