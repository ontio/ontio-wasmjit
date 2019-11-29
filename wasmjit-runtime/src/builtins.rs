//! Runtime library calls. Note that wasm compilers may sometimes perform these
//! inline rather than calling them, particularly when CPUs have special
//! instructions which compute them directly.

use crate::vmcontext::VMContext;
use crate::wasmjit_unwind;
use cranelift_wasm::DefinedMemoryIndex;
use std::panic;
use std::sync::atomic::Ordering;

/// catch panic of rust host/builtins function.
pub fn check_host_panic<F, U>(f: F) -> U
where
    F: FnOnce() -> U + panic::UnwindSafe,
{
    panic::catch_unwind(f).unwrap_or_else(|e| {
        let msg = if let Some(err) = e.downcast_ref::<String>() {
            err.to_string()
        } else if let Some(err) = e.downcast_ref::<&str>() {
            err.to_string()
        } else {
            "wasm host function paniced!".to_string()
        };

        unsafe { wasmjit_unwind(msg) }
    })
}

/// Implementation of memory.grow for locally-defined 32-bit memories.
#[no_mangle]
pub unsafe extern "C" fn wasmjit_memory32_grow(
    vmctx: *mut VMContext,
    delta: u32,
    memory_index: u32,
) -> u32 {
    check_host_panic(|| {
        let instance = (&mut *vmctx).instance();
        let memory_index = DefinedMemoryIndex::from_u32(memory_index);

        instance
            .memory_grow(memory_index, delta)
            .unwrap_or(u32::max_value())
    })
}

/// Implementation of memory.size for locally-defined 32-bit memories.
#[no_mangle]
pub unsafe extern "C" fn wasmjit_memory32_size(vmctx: *mut VMContext, memory_index: u32) -> u32 {
    check_host_panic(|| {
        let instance = (&mut *vmctx).instance();
        let memory_index = DefinedMemoryIndex::from_u32(memory_index);

        instance.memory_size(memory_index)
    })
}

/// Implementation of check gas
#[no_mangle]
pub unsafe extern "C" fn wasmjit_check_gas(vmctx: *mut VMContext, costs: u32) {
    check_host_panic(|| {
        let costs = costs as u64;
        let instance = (&mut *vmctx).instance();
        let origin = instance.gas_left.fetch_sub(costs, Ordering::Relaxed);

        if origin < costs {
            instance.gas_left.store(0, Ordering::Relaxed);
            panic!("wasmjit: gas exhausted");
        }
    })
}

/// Implementation of check gas
#[no_mangle]
pub unsafe extern "C" fn wasmjit_check_depth(vmctx: *mut VMContext, step_in: i32) {
    check_host_panic(|| {
        let instance = (&mut *vmctx).instance();
        let mut origin: u64 = 0;
        if step_in > 0 {
            origin = instance.depth_left.fetch_sub(1, Ordering::Relaxed);
        } else {
            origin = instance.depth_left.fetch_add(1, Ordering::Relaxed);
        }
        if origin == 0 {
            instance.depth_left.store(0, Ordering::Relaxed);
            panic!("wasmjit: depth over 3");
        }
    })
}
