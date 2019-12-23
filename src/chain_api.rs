use crate::resolver::Resolver;
use cranelift_wasm::DefinedMemoryIndex;
use hmac_sha256::Hash;
use ontio_wasmjit_runtime::builtins::{
    check_host_panic, wasmjit_result_err_internal, wasmjit_trap,
};
use ontio_wasmjit_runtime::{
    wasmjit_unwind, ExecMetrics, Instance, VMContext, VMFunctionBody, VMFunctionImport,
};
use std::any::Any;
use std::panic;
use std::sync::atomic::Ordering;
use std::sync::Arc;

pub type Address = [u8; 20];
pub type H256 = [u8; 32];

pub const TIME_STAMP_GAS: u64 = 1;
pub const BLOCK_HEGHT_GAS: u64 = 1;
pub const SELF_ADDRESS_GAS: u64 = 1;
pub const CALLER_ADDRESS_GAS: u64 = 1;
pub const ENTRY_ADDRESS_GAS: u64 = 1;
pub const CHECKWITNESS_GAS: u64 = 200;
pub const CALL_CONTRACT_GAS: u64 = 10;
pub const CONTRACT_CREATE_GAS: u64 = 20000000;
pub const CONTRACT_MIGRATE_GAS: u64 = 20000000;
pub const NATIVE_INVOKE_GAS: u64 = 1000;

pub const CURRENT_BLOCK_HASH_GAS: u64 = 100;
pub const CURRENT_TX_HASH_GAS: u64 = 100;

pub const STORAGE_GET_GAS: u64 = 200;
pub const STORAGE_PUT_GAS: u64 = 4000;
pub const STORAGE_DELETE_GAS: u64 = 100;
pub const UINT_DEPLOY_CODE_LEN_GAS: u64 = 200000;
pub const PER_UNIT_CODE_LEN: u64 = 1024;

pub const SHA256_GAS: u64 = 10;

#[derive(Default)]
pub struct ChainCtx {
    height: u32,
    block_hash: H256,
    timestamp: u64,
    tx_hash: H256,
    invoke_addrs: Vec<Address>,
    witness: Vec<Address>,
    input: Vec<u8>,
    pub(crate) exec_metrics: Arc<ExecMetrics>,
    call_output: Vec<u8>,
    service_index: u64,
    from_return: bool,
}

impl ChainCtx {
    pub fn push_caller(&mut self, caller: Address) {
        self.invoke_addrs.push(caller);
    }
    pub fn pop_caller(&mut self) -> Option<Address> {
        self.invoke_addrs.pop()
    }

    pub fn exec_step(&self) -> u64 {
        self.exec_metrics.exec_step_left.load(Ordering::Relaxed)
    }

    pub fn set_exec_step(&mut self, exec_step: u64) {
        self.exec_metrics
            .exec_step_left
            .store(exec_step, Ordering::Relaxed)
    }

    pub fn gas_factor(&self) -> u64 {
        self.exec_metrics.gas_factor.load(Ordering::Relaxed)
    }

    pub fn set_gas_factor(&mut self, gas_factor: u64) {
        self.exec_metrics
            .gas_factor
            .store(gas_factor, Ordering::Relaxed);
    }

    pub fn gas_left(&self) -> u64 {
        self.exec_metrics.gas_left.load(Ordering::Relaxed)
    }

    pub fn get_exec_metrics(&self) -> Arc<ExecMetrics> {
        self.exec_metrics.clone()
    }

    pub fn set_gas_left(&mut self, gas: u64) {
        self.exec_metrics.gas_left.store(gas, Ordering::Relaxed);
    }

    pub fn set_depth_left(&mut self, depth_left: u64) {
        self.exec_metrics
            .depth_left
            .store(depth_left, Ordering::Relaxed)
    }

    pub fn take_output(&mut self) -> Vec<u8> {
        let mut result = Vec::new();
        std::mem::swap(&mut self.call_output, &mut result);
        result
    }

    pub fn set_output(&mut self, output: Vec<u8>) {
        self.call_output = output;
    }

    pub fn call_output_len(&self) -> u32 {
        self.call_output.len() as u32
    }

    pub fn set_from_return(&mut self) {
        self.from_return = true;
    }

    pub fn is_from_return(&self) -> bool {
        self.from_return
    }

    pub fn service_index(&self) -> u64 {
        self.service_index
    }

    pub fn new(
        timestamp: u64,
        height: u32,
        block_hash: H256,
        tx_hash: H256,
        invoke_addrs: Vec<Address>,
        witness: Vec<Address>,
        input: Vec<u8>,
        exec_metrics: ExecMetrics,
        service_index: u64,
    ) -> Self {
        Self {
            height,
            block_hash,
            timestamp,
            tx_hash,
            invoke_addrs,
            witness,
            input,
            exec_metrics: Arc::new(exec_metrics),
            call_output: Vec::new(),
            service_index,
            from_return: false,
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn ontio_runtime_check_gas(vmctx: *mut VMContext, costs: u64) {
    check_host_panic((&mut *vmctx).instance(), |instance| {
        if !instance.check_gas(costs) {
            panic!("wasmjit: gas exhausted");
        }
    })
}

pub fn check_gas_and_host_panic<F, U>(instance: &mut Instance, gas: u64, func: F) -> U
where
    F: FnOnce(&mut Instance) -> U + panic::UnwindSafe,
{
    check_host_panic(instance, |instance| {
        if !instance.check_gas(gas) {
            panic!("wasmjit: gas exhausted");
        }
        func(instance)
    })
}

/// Implementation of ontio_timestamp api
#[no_mangle]
pub unsafe extern "C" fn ontio_timestamp(vmctx: *mut VMContext) -> u64 {
    check_gas_and_host_panic((&mut *vmctx).instance(), TIME_STAMP_GAS, |instance| {
        let host = instance.host_state();
        convert_chainctx(host).timestamp
    })
}

/// Implementation of ontio_block_height api
#[no_mangle]
pub unsafe extern "C" fn ontio_block_height(vmctx: *mut VMContext) -> u32 {
    check_gas_and_host_panic((&mut *vmctx).instance(), BLOCK_HEGHT_GAS, |instance| {
        let host = instance.host_state();
        convert_chainctx(host).height
    })
}

/// Implementation of ontio_current_blockhash api
#[no_mangle]
pub unsafe extern "C" fn ontio_current_blockhash(
    vmctx: *mut VMContext,
    block_hash_ptr: u32,
) -> u32 {
    check_gas_and_host_panic(
        (&mut *vmctx).instance(),
        CURRENT_BLOCK_HASH_GAS,
        |instance| {
            let host = instance.host_state();
            let chain = convert_chainctx(host);
            let instance = (&mut *vmctx).instance();
            let memory = instance
                .memory_slice_mut(DefinedMemoryIndex::from_u32(0))
                .unwrap();
            let start = block_hash_ptr as usize;
            memory[start..start + chain.block_hash.len()].copy_from_slice(&chain.block_hash);
            32
        },
    )
}

/// Implementation of ontio_current_txhash api
#[no_mangle]
pub unsafe extern "C" fn ontio_current_txhash(vmctx: *mut VMContext, tx_hash_ptr: u32) -> u32 {
    check_gas_and_host_panic((&mut *vmctx).instance(), CURRENT_TX_HASH_GAS, |instance| {
        let host = instance.host_state();
        let tx_hash = convert_chainctx(host).tx_hash;
        let memory = instance
            .memory_slice_mut(DefinedMemoryIndex::from_u32(0))
            .unwrap();
        let start = tx_hash_ptr as usize;
        memory[start..start + tx_hash.len()].copy_from_slice(&tx_hash);
        32
    })
}

/// Implementation of ontio_self_address api
#[no_mangle]
pub unsafe extern "C" fn ontio_self_address(vmctx: *mut VMContext, addr_ptr: u32) {
    check_gas_and_host_panic((&mut *vmctx).instance(), SELF_ADDRESS_GAS, |instance| {
        let host = instance.host_state();
        let chain = convert_chainctx(host);
        let addr: Address = chain.invoke_addrs.last().copied().unwrap_or([0; 20]);
        let memory = instance
            .memory_slice_mut(DefinedMemoryIndex::from_u32(0))
            .unwrap();
        let start = addr_ptr as usize;
        memory[start..start + 20].copy_from_slice(&addr);
    })
}

/// Implementation of ontio_caller_address api
#[no_mangle]
pub unsafe extern "C" fn ontio_caller_address(vmctx: *mut VMContext, caller_ptr: u32) {
    check_gas_and_host_panic((&mut *vmctx).instance(), CALLER_ADDRESS_GAS, |instance| {
        let host = instance.host_state();
        let chain = convert_chainctx(host);
        let addr = if chain.invoke_addrs.len() >= 2 {
            chain
                .invoke_addrs
                .get(chain.invoke_addrs.len() - 2)
                .copied()
                .unwrap_or([0; 20])
        } else {
            [0; 20]
        };
        let memory = instance
            .memory_slice_mut(DefinedMemoryIndex::from_u32(0))
            .unwrap();
        let start = caller_ptr as usize;
        memory[start..start + 20].copy_from_slice(&addr);
    })
}

/// Implementation of ontio_entry_address api
#[no_mangle]
pub unsafe extern "C" fn ontio_entry_address(vmctx: *mut VMContext, entry_ptr: u32) {
    check_gas_and_host_panic((&mut *vmctx).instance(), ENTRY_ADDRESS_GAS, |instance| {
        let host = instance.host_state();
        let chain = convert_chainctx(host);
        let addr: Address = chain.invoke_addrs.first().copied().unwrap_or([0; 20]);
        let memory = instance
            .memory_slice_mut(DefinedMemoryIndex::from_u32(0))
            .unwrap();
        let start = entry_ptr as usize;
        memory[start..start + 20].copy_from_slice(&addr);
    })
}

/// Implementation of ontio_check_witness api
#[no_mangle]
pub unsafe extern "C" fn ontio_check_witness(vmctx: *mut VMContext, addr_ptr: u32) -> u32 {
    check_gas_and_host_panic((&mut *vmctx).instance(), CHECKWITNESS_GAS, |instance| {
        let memory = instance
            .memory_slice_mut(DefinedMemoryIndex::from_u32(0))
            .unwrap();
        let start = addr_ptr as usize;
        let mut addr: Address = [0; 20];
        addr.copy_from_slice(&memory[start..start + 20]);
        let host = instance.host_state();
        let chain = convert_chainctx(host);
        let res = chain.witness.iter().find(|&&x| x == addr);
        match res {
            Some(_) => 1,
            None => 0,
        }
    })
}

/// Implementation of ontio_input_length api
#[no_mangle]
pub unsafe extern "C" fn ontio_input_length(vmctx: *mut VMContext) -> u32 {
    check_host_panic((&mut *vmctx).instance(), |instance| {
        let host = instance.host_state();
        let chain = convert_chainctx(host);
        chain.input.len() as u32
    })
}

/// Implementation of ontio_output_length api
#[no_mangle]
pub unsafe extern "C" fn ontio_call_output_length(vmctx: *mut VMContext) -> u32 {
    check_host_panic((&mut *vmctx).instance(), |instance| {
        let host = instance.host_state();
        let chain = convert_chainctx(host);
        chain.call_output.len() as u32
    })
}

/// Implementation of ontio_get_input api
#[no_mangle]
pub unsafe extern "C" fn ontio_get_input(vmctx: *mut VMContext, input_ptr: u32) {
    check_host_panic((&mut *vmctx).instance(), |instance| {
        let host = instance.host_state();
        let input = convert_chainctx(host).input.clone();
        let memory = instance
            .memory_slice_mut(DefinedMemoryIndex::from_u32(0))
            .unwrap();
        let start = input_ptr as usize;
        memory[start..start + input.len()].copy_from_slice(&input);
    })
}

/// Implementation of ontio_get_call_out api
#[no_mangle]
pub unsafe extern "C" fn ontio_get_call_output(vmctx: *mut VMContext, dst_ptr: u32) {
    check_host_panic((&mut *vmctx).instance(), |instance| {
        let host = instance.host_state();
        let call_output = convert_chainctx(host).call_output.clone();
        let memory = instance
            .memory_slice_mut(DefinedMemoryIndex::from_u32(0))
            .unwrap();
        let start = dst_ptr as usize;
        memory[start..start + call_output.len()].copy_from_slice(&call_output);
    })
}

/// Implementation of ontio_panic api
#[no_mangle]
pub unsafe extern "C" fn ontio_panic(vmctx: *mut VMContext, input_ptr: u32, ptr_len: u32) {
    let msg = panic::catch_unwind(|| {
        let instance = (&mut *vmctx).instance();
        let memory = instance
            .memory_slice_mut(DefinedMemoryIndex::from_u32(0))
            .unwrap();
        let start = input_ptr as usize;
        let end = start
            .checked_add(ptr_len as usize)
            .expect("out of memory bound");
        String::from_utf8_lossy(&memory[start..end]).to_string()
    })
    .unwrap_or_else(|e| {
        if let Some(err) = e.downcast_ref::<String>() {
            err.to_string()
        } else if let Some(&err) = e.downcast_ref::<&str>() {
            err.to_string()
        } else {
            "wasm host function paniced!".to_string()
        }
    });

    wasmjit_unwind(msg)
}

/// Implementation of ontio_sha256 api
#[no_mangle]
pub unsafe extern "C" fn ontio_sha256(vmctx: *mut VMContext, data_ptr: u32, l: u32, out_ptr: u32) {
    let costs = ((l / 1024) + 1) as u64 * SHA256_GAS;
    check_gas_and_host_panic((&mut *vmctx).instance(), costs, |instance| {
        let memory = instance
            .memory_slice_mut(DefinedMemoryIndex::from_u32(0))
            .unwrap();
        let start = data_ptr as usize;
        let data = &memory[start..start + l as usize];
        let res = Hash::hash(data);
        let start = out_ptr as usize;
        memory[start..start + res.len()].copy_from_slice(&res);
    })
}

/// Implementation of ontio_return api
#[no_mangle]
pub unsafe extern "C" fn ontio_return(vmctx: *mut VMContext, data_ptr: u32, l: u32) -> ! {
    check_host_panic((&mut *vmctx).instance(), |instance| {
        let memory = instance
            .memory_slice_mut(DefinedMemoryIndex::from_u32(0))
            .unwrap();
        // check here to avoid the memory attack in go.
        if memory.len() < (data_ptr + l) as usize {
            panic!("ontio_return access out of bound");
        }

        let mut output_buffer = Vec::new();
        output_buffer.extend_from_slice(&memory[data_ptr as usize..(data_ptr + l) as usize]);

        let host = instance.host_state();
        let chain = convert_chainctx(host);

        chain.set_output(output_buffer);
        chain.set_from_return();
    });

    wasmjit_unwind(String::new());
}

/*
const SIGNATURES: [(&str, &[ValueType], Option<ValueType>); 24] = [
    ("ontio_notify", &[ValueType::I32; 2], None),
    ("ontio_call_contract", &[ValueType::I32; 3], Some(ValueType::I32)),
    ("ontio_contract_create", &[ValueType::I32; 14], Some(ValueType::I32)),
    ("ontio_contract_migrate", &[ValueType::I32; 14], Some(ValueType::I32)),
    ("ontio_contract_destroy", &[], None),
    ("ontio_storage_read", &[ValueType::I32; 5], Some(ValueType::I32)),
    ("ontio_storage_write", &[ValueType::I32; 4], None),
    ("ontio_storage_delete", &[ValueType::I32; 2], None),
    ("ontio_debug", &[ValueType::I32; 2], None),
];
*/

pub struct ChainResolver;

impl Resolver for ChainResolver {
    fn resolve(&mut self, _module: &str, field: &str) -> Option<VMFunctionImport> {
        match field {
            "ontio_timestamp" => Some(VMFunctionImport {
                body: ontio_timestamp as *const VMFunctionBody,
            }),
            "ontio_block_height" => Some(VMFunctionImport {
                body: ontio_block_height as *const VMFunctionBody,
            }),
            "ontio_input_length" => Some(VMFunctionImport {
                body: ontio_input_length as *const VMFunctionBody,
            }),
            "ontio_call_output_length" => Some(VMFunctionImport {
                body: ontio_call_output_length as *const VMFunctionBody,
            }),
            "ontio_get_input" => Some(VMFunctionImport {
                body: ontio_get_input as *const VMFunctionBody,
            }),
            "ontio_current_blockhash" => Some(VMFunctionImport {
                body: ontio_current_blockhash as *const VMFunctionBody,
            }),
            "ontio_current_txhash" => Some(VMFunctionImport {
                body: ontio_current_txhash as *const VMFunctionBody,
            }),
            "ontio_self_address" => Some(VMFunctionImport {
                body: ontio_self_address as *const VMFunctionBody,
            }),
            "ontio_caller_address" => Some(VMFunctionImport {
                body: ontio_caller_address as *const VMFunctionBody,
            }),
            "ontio_entry_address" => Some(VMFunctionImport {
                body: ontio_entry_address as *const VMFunctionBody,
            }),
            "ontio_check_witness" => Some(VMFunctionImport {
                body: ontio_check_witness as *const VMFunctionBody,
            }),
            "ontio_sha256" => Some(VMFunctionImport {
                body: ontio_sha256 as *const VMFunctionBody,
            }),
            "ontio_get_call_output" => Some(VMFunctionImport {
                body: ontio_get_call_output as *const VMFunctionBody,
            }),
            "ontio_panic" => Some(VMFunctionImport {
                body: ontio_panic as *const VMFunctionBody,
            }),
            "ontio_return" => Some(VMFunctionImport {
                body: ontio_return as *const VMFunctionBody,
            }),
            _ => None,
        }
    }
}

pub unsafe fn convert_chainctx(host: &mut dyn Any) -> &mut ChainCtx {
    host.downcast_mut::<ChainCtx>().unwrap_or_else(|| {
        panic!(wasmjit_trap {
            kind: wasmjit_result_err_internal,
            msg: String::from("Get ChainCtx error"),
        })
    })
}
