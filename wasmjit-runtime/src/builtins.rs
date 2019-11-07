//! Runtime library calls. Note that wasm compilers may sometimes perform these
//! inline rather than calling them, particularly when CPUs have special
//! instructions which compute them directly.

use crate::vmcontext::VMContext;
use cranelift_wasm::DefinedMemoryIndex;
use std::sync::atomic::Ordering;

/// Implementation of memory.grow for locally-defined 32-bit memories.
#[no_mangle]
pub unsafe extern "C" fn wasmjit_memory32_grow(
    vmctx: *mut VMContext,
    delta: u32,
    memory_index: u32,
) -> u32 {
    let instance = (&mut *vmctx).instance();
    let memory_index = DefinedMemoryIndex::from_u32(memory_index);

    instance
        .memory_grow(memory_index, delta)
        .unwrap_or(u32::max_value())
}

/// Implementation of memory.size for locally-defined 32-bit memories.
#[no_mangle]
pub unsafe extern "C" fn wasmjit_memory32_size(vmctx: *mut VMContext, memory_index: u32) -> u32 {
    let instance = (&mut *vmctx).instance();
    let memory_index = DefinedMemoryIndex::from_u32(memory_index);

    instance.memory_size(memory_index)
}

/// Implementation of check gas
#[no_mangle]
pub unsafe extern "C" fn wasmjit_check_gas(vmctx: *mut VMContext, costs: u32) {
    let costs = costs as u64;
    let instance = (&mut *vmctx).instance();
    let origin = instance.gas_left.fetch_sub(costs, Ordering::Relaxed);

    if origin < costs {
        instance.gas_left.store(0, Ordering::Relaxed);
        panic!("todo: gas exhausted");
    }
}
