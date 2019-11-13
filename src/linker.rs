use core::ptr::write_unaligned;
use cranelift_codegen::binemit::Reloc;
use cranelift_codegen::ir::JumpTableOffsets;
use cranelift_entity::PrimaryMap;
use cranelift_wasm::DefinedFuncIndex;

use ontio_wasmjit_environ::{Module, RelocationTarget, Relocations};

use ontio_wasmjit_runtime::{builtins, VMFunctionBody};

/// Links a module that has been compiled with `compiled_module` in `wasmtime-environ`.
pub fn link_module(
    module: &Module,
    allocated_functions: &PrimaryMap<DefinedFuncIndex, *const VMFunctionBody>, // acturally *mut VMFunctionBody
    jt_offsets: &PrimaryMap<DefinedFuncIndex, JumpTableOffsets>,
    relocations: &Relocations,
) {
    // Apply relocations, now that we have virtual addresses for everything.
    for (i, function_relocs) in relocations.into_iter() {
        for r in function_relocs {
            let target_func_address: usize = match r.reloc_target {
                RelocationTarget::UserFunc(index) => match module.defined_func_index(index) {
                    Some(f) => allocated_functions[f] as usize,
                    None => panic!("import function has been translated to indirect call"),
                },
                RelocationTarget::Memory32Grow => builtins::wasmjit_memory32_grow as usize,
                RelocationTarget::Memory32Size => builtins::wasmjit_memory32_size as usize,
                RelocationTarget::LibCall(libcall) => {
                    //todo: check when this will happen
                    panic!("unexpected libcall: {}", libcall)
                }
                RelocationTarget::JumpTable(func_index, jt) => {
                    match module.defined_func_index(func_index) {
                        Some(f) => {
                            let offset = *jt_offsets
                                .get(f)
                                .and_then(|ofs| ofs.get(jt))
                                .expect("func jump table");

                            allocated_functions[f] as usize + offset as usize
                        }
                        None => panic!("func index of jump table"),
                    }
                }
            };

            let body = allocated_functions[i];
            match r.reloc {
                #[cfg(target_pointer_width = "64")]
                Reloc::Abs8 => unsafe {
                    let reloc_address = body.add(r.offset as usize) as usize;
                    let reloc_addend = r.addend as isize;
                    let reloc_abs = (target_func_address as u64)
                        .checked_add(reloc_addend as u64)
                        .unwrap();
                    write_unaligned(reloc_address as *mut u64, reloc_abs);
                },
                #[cfg(target_pointer_width = "32")]
                Reloc::X86PCRel4 => unsafe {
                    let reloc_address = body.add(r.offset as usize) as usize;
                    let reloc_addend = r.addend as isize;
                    let reloc_delta_u32 = (target_func_address as u32)
                        .wrapping_sub(reloc_address as u32)
                        .checked_add(reloc_addend as u32)
                        .unwrap();
                    write_unaligned(reloc_address as *mut u32, reloc_delta_u32);
                },
                #[cfg(target_pointer_width = "32")]
                Reloc::X86CallPCRel4 => {
                    // ignore
                }
                Reloc::X86PCRelRodata4 => {
                    // ignore
                }
                //todo: check when this will happen
                _ => panic!("unsupported reloc kind"),
            }
        }
    }
}
