//! Runtime library calls. Note that wasm compilers may sometimes perform these
//! inline rather than calling them, particularly when CPUs have special
//! instructions which compute them directly.

use crate::vmcontext::VMContext;
use cranelift_wasm::DefinedMemoryIndex;

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
