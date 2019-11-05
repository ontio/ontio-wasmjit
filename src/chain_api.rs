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
    self_address: Address,
    callers: Vec<Address>,
    witness: Vec<Address>,
    input: Vec<u8>,
    call_output: Vec<u8>,
}

impl ChainCtx {
    pub fn new(
        timestamp: u64,
        height: u32,
        block_hash: H256,
        tx_hash: H256,
        self_address: Address,
        callers: Vec<Address>,
        witness: Vec<Address>,
        input: Vec<u8>,
        call_output: Vec<u8>,
    ) -> Self {
        Self {
            height,
            block_hash,
            timestamp,
            tx_hash,
            self_address,
            callers,
            witness,
            input,
            call_output,
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

/// Implementation of ontio_current_blockhash api
#[no_mangle]
pub unsafe extern "C" fn ontio_current_blockhash(
    vmctx: *mut VMContext,
    block_hash_ptr: u32,
) -> u32 {
    let host = (&mut *vmctx).host_state();
    let chain = host.downcast_ref::<ChainCtx>().unwrap();
    let instance = (&mut *vmctx).instance();
    // FIXME: check memory 0 exist
    let mut memory = instance.memory_slice_mut(DefinedMemoryIndex::from_u32(0));
    let start = block_hash_ptr as usize;
    memory[start..start + chain.block_hash.len()].copy_from_slice(&chain.block_hash);
    32
}

/// Implementation of ontio_current_txhash api
#[no_mangle]
pub unsafe extern "C" fn ontio_current_txhash(vmctx: *mut VMContext, tx_hash_ptr: u32) -> u32 {
    let host = (&mut *vmctx).host_state();
    let chain = host.downcast_ref::<ChainCtx>().unwrap();
    let instance = (&mut *vmctx).instance();
    // FIXME: check memory 0 exist
    let mut memory = instance.memory_slice_mut(DefinedMemoryIndex::from_u32(0));
    let start = tx_hash_ptr as usize;
    memory[start..start + &chain.tx_hash.len()].copy_from_slice(&chain.tx_hash);
    20
}

/// Implementation of ontio_self_address api
#[no_mangle]
pub unsafe extern "C" fn ontio_self_address(vmctx: *mut VMContext, addr_ptr: u32) {
    let host = (&mut *vmctx).host_state();
    let chain = host.downcast_ref::<ChainCtx>().unwrap();
    let instance = (&mut *vmctx).instance();
    // FIXME: check memory 0 exist
    let mut memory = instance.memory_slice_mut(DefinedMemoryIndex::from_u32(0));
    // FIXME: check memory bounds
    let start = addr_ptr as usize;
    memory[start..start + 20].copy_from_slice(&chain.self_address);
}

/// Implementation of ontio_caller_address api
#[no_mangle]
pub unsafe extern "C" fn ontio_caller_address(vmctx: *mut VMContext, caller_ptr: u32) {
    let host = (&mut *vmctx).host_state();
    let chain = host.downcast_ref::<ChainCtx>().unwrap();
    let instance = (&mut *vmctx).instance();
    // FIXME: check memory 0 exist
    let mut memory = instance.memory_slice_mut(DefinedMemoryIndex::from_u32(0));
    // FIXME: check memory bounds
    let start = caller_ptr as usize;
    let addr: Address = chain.callers.last().map(|v| *v).unwrap_or([0; 20]);
    memory[start..start + 20].copy_from_slice(&addr);
}

/// Implementation of ontio_entry_address api
#[no_mangle]
pub unsafe extern "C" fn ontio_entry_address(vmctx: *mut VMContext, entry_ptr: u32) {
    let host = (&mut *vmctx).host_state();
    let chain = host.downcast_ref::<ChainCtx>().unwrap();
    let instance = (&mut *vmctx).instance();
    // FIXME: check memory 0 exist
    let mut memory = instance.memory_slice_mut(DefinedMemoryIndex::from_u32(0));
    // FIXME: check memory bounds
    let start = entry_ptr as usize;
    let addr: Address = chain.callers.first().map(|v| *v).unwrap_or([0; 20]);
    memory[start..start + 20].copy_from_slice(&addr);
}

/// Implementation of ontio_check_witness api
#[no_mangle]
pub unsafe extern "C" fn ontio_check_witness(vmctx: *mut VMContext, addr_ptr: u32) -> u32 {
    let host = (&mut *vmctx).host_state();
    let chain = host.downcast_ref::<ChainCtx>().unwrap();
    let instance = (&mut *vmctx).instance();
    // FIXME: check memory 0 exist
    let memory = instance.memory_slice_mut(DefinedMemoryIndex::from_u32(0));
    // FIXME: check memory bounds
    let start = addr_ptr as usize;
    let mut addr: Address = [0; 20];
    addr.copy_from_slice(&memory[start..start + 20]);
    let res = chain.witness.iter().find(|&&x| x == addr);
    match res {
        Some(_) => 1,
        None => 0,
    }
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
    let mut memory = instance.memory_slice_mut(DefinedMemoryIndex::from_u32(0));
    // FIXME: check memory bounds
    let start = input_ptr as usize;
    memory[start..start + chain.input.len()].copy_from_slice(&chain.input);
}

//TODO
/// Implementation of ontio_panic api
#[no_mangle]
pub unsafe extern "C" fn ontio_panic(vmctx: *mut VMContext, input_ptr: u32, ptr_len: u32) {
    //TODO
    println!("ontio_panic");
}

//TODO
/// Implementation of ontio_debug api
#[no_mangle]
pub unsafe extern "C" fn ontio_debug(vmctx: *mut VMContext, input_ptr: u32, ptr_len: u32) {
    let instance = (&mut *vmctx).instance();
    // FIXME: check memory 0 exist
    let memory = instance.memory_slice_mut(DefinedMemoryIndex::from_u32(0));
    // FIXME: check memory bounds
    let start = input_ptr as usize;
    let msg = &memory[start..start + ptr_len as usize];
    println!("ontio-debug: {}", String::from_utf8_lossy(msg));
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
            "ontio_current_blockhash" => Some(VMFunctionImport {
                body: ontio_current_blockhash as *const VMFunctionBody,
                vmctx: ptr::null_mut(),
            }),
            "ontio_current_txhash" => Some(VMFunctionImport {
                body: ontio_current_txhash as *const VMFunctionBody,
                vmctx: ptr::null_mut(),
            }),
            "ontio_self_address" => Some(VMFunctionImport {
                body: ontio_self_address as *const VMFunctionBody,
                vmctx: ptr::null_mut(),
            }),
            "ontio_caller_address" => Some(VMFunctionImport {
                body: ontio_caller_address as *const VMFunctionBody,
                vmctx: ptr::null_mut(),
            }),
            "ontio_entry_address" => Some(VMFunctionImport {
                body: ontio_entry_address as *const VMFunctionBody,
                vmctx: ptr::null_mut(),
            }),
            "ontio_check_witness" => Some(VMFunctionImport {
                body: ontio_check_witness as *const VMFunctionBody,
                vmctx: ptr::null_mut(),
            }),
            "ontio_panic" => Some(VMFunctionImport {
                body: ontio_panic as *const VMFunctionBody,
                vmctx: ptr::null_mut(),
            }),
            "ontio_debug" => Some(VMFunctionImport {
                body: ontio_debug as *const VMFunctionBody,
                vmctx: ptr::null_mut(),
            }),
            _ => None,
        }
    }
}
