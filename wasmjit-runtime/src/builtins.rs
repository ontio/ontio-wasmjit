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

        if instance.exec_step.load(Ordering::Relaxed) < costs {
            instance.exec_step.store(0, Ordering::Relaxed);
            panic!("wasmjit: exec step exhausted");
        } else {
            instance.exec_step.fetch_sub(costs, Ordering::Relaxed);
        }

        instance.local_gas_counter += costs;

        let gas_factor = instance.gas_factor.load(Ordering::Relaxed);
        let normalize_costs = instance.local_gas_counter / gas_factor;
        if normalize_costs == 0 {
            return;
        }

        instance.local_gas_counter %= gas_factor;

        if instance.gas_left.load(Ordering::Relaxed) >= normalize_costs {
            instance
                .gas_left
                .fetch_sub(normalize_costs, Ordering::Relaxed);
        } else {
            instance.gas_left.store(0, Ordering::Relaxed);
            panic!("wasmjit: gas exhausted");
        }
    })
}

/// Implementation of check gas
#[no_mangle]
pub unsafe extern "C" fn wasmjit_check_depth(vmctx: *mut VMContext, count: i32) {
    check_host_panic(|| {
        let instance = (&mut *vmctx).instance();
        let origin = if count > 0 {
            instance
                .depth_left
                .fetch_sub(count as u64, Ordering::Relaxed)
        } else {
            instance
                .depth_left
                .fetch_add(-count as u64, Ordering::Relaxed)
        };
        if origin == 0 {
            instance.depth_left.store(0, Ordering::Relaxed);
            panic!("wasmjit: out of function calling depth limitation");
        }
    })
}
