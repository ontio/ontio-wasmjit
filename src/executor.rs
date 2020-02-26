use crate::chain_api::ChainCtx;
use crate::resolver::Resolver;
use crate::trampoline::make_trampoline;
use crate::{error::Error, linker, utils};

use cranelift_codegen::ir;
use cranelift_codegen::isa;
use cranelift_codegen::settings;
use cranelift_codegen::settings::Configurable;
use cranelift_entity::PrimaryMap;
use cranelift_wasm::DefinedFuncIndex;
use ontio_wasmjit_environ::{
    compile_module, BuildOption, CompileError, Module as ModuleInfo, ModuleEnvironment,
    OwnedDataInitializer, Relocations, Traps, Tunables,
};
use ontio_wasmjit_runtime::builtins::{wasmjit_result_err_trap, wasmjit_result_kind};
use ontio_wasmjit_runtime::{
    get_mut_trap_registry, wasmjit_call, wasmjit_call_trampoline, InstanceHandle,
    TrapRegistrationGuard, VMFunctionBody,
};

use dynasmrt::mmap::MutableBuffer;
use dynasmrt::ExecutableBuffer;
use lru::LruCache;
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use std::convert::TryFrom;
use std::{mem, sync::Arc, usize};
use target_lexicon::PointerWidth;

static MODULE_CACHE: Lazy<Mutex<LruCache<[u8; 20], Arc<Module>>>> =
    Lazy::new(|| Mutex::new(LruCache::new(20)));

pub struct Instance {
    #[allow(unused)]
    module: Arc<Module>,
    handle: InstanceHandle,
}

unsafe impl Send for Instance {}

impl Instance {
    pub fn execute(
        &mut self,
        chain: ChainCtx,
        func: &str,
        args: Vec<i64>,
    ) -> Result<Option<i64>, Error> {
        let invoke = self
            .handle
            .lookup(func)
            .ok_or_else(|| Error::Internal(format!("can not find export function: {}", func)))?;

        self.set_host_state(Box::new(chain));

        let isa_builder = isa::lookup_by_name("x86_64").unwrap();
        let mut flag_builder = settings::builder();
        let _ = flag_builder.set("probestack_enabled", "false");
        let isa = isa_builder.finish(settings::Flags::new(flag_builder));
        let func = make_trampoline(
            &*isa,
            invoke.address,
            &invoke.signature,
            mem::size_of::<u64>(),
        )
        .map_err(Error::Internal)?;

        let mut trampoline = MutableBuffer::new(func.len()).unwrap();
        trampoline.set_len(func.len());
        trampoline.copy_from_slice(&func);
        let tranpoline = trampoline.make_exec().unwrap();

        let address = &tranpoline[0] as *const u8 as *const VMFunctionBody;
        let mut args_vec = args;
        args_vec.push(0); // place holder for return value
        if let Err(err) = unsafe {
            wasmjit_call_trampoline(invoke.vmctx, address, args_vec.as_mut_ptr() as *mut u8)
        } {
            if !self.host_state().is_from_return() {
                let trap_kind = self.handle.trap_kind();
                if trap_kind == wasmjit_result_err_trap {
                    return Err(Error::Trap(err));
                } else {
                    return Err(Error::Internal(err));
                }
            }
        }

        if invoke.signature.returns.is_empty() {
            return Ok(None);
        }
        if invoke.signature.returns[0].value_type == ir::types::I32 {
            return Ok(Some(args_vec[0] as i32 as i64));
        }
        Ok(Some(args_vec[0] as i64))
    }

    pub fn invoke(&mut self, cctx: Box<ChainCtx>) -> Result<(), Error> {
        let invoke = self
            .handle
            .lookup("invoke")
            .ok_or_else(|| Error::Internal("can not find export function: invoke".to_string()))?;
        let param = invoke.signature.params.len();
        let ret = invoke.signature.returns.len();
        if param != 1
            || ret != 0
            || invoke.signature.params[0].purpose != ir::ArgumentPurpose::VMContext
        {
            return Err(Error::Internal(
                "invalid invoke function signature".to_string(),
            ));
        }

        self.set_host_state(cctx);
        let result = unsafe { wasmjit_call(invoke.vmctx, invoke.address) };

        let trap_kind = self.handle.trap_kind();
        let normal_return = self.host_state().is_from_return();

        match result {
            Ok(_) => Ok(()),
            Err(_) if normal_return => Ok(()),
            Err(message) => {
                if trap_kind == wasmjit_result_err_trap {
                    Err(Error::Trap(message))
                } else {
                    Err(Error::Internal(message))
                }
            }
        }
    }

    pub fn set_host_state(&mut self, host_state: Box<ChainCtx>) {
        let instance = self.handle.instance_mut();
        instance.exec_metrics = host_state.exec_metrics.clone();
        self.handle.set_host_state(host_state);
    }

    pub fn host_state(&mut self) -> &mut ChainCtx {
        self.handle.host_state().downcast_mut::<ChainCtx>().unwrap()
    }

    pub fn trap_kind(&mut self) -> wasmjit_result_kind {
        self.handle.trap_kind()
    }
}

pub fn build_module(wasm: &[u8], build_option: BuildOption) -> Result<Arc<Module>, Error> {
    let address = utils::contract_address(wasm);
    let module = MODULE_CACHE.lock().get(&address).cloned();

    match module {
        Some(module) => Ok(module),
        None => {
            let module = Module::compile(wasm, build_option)?;
            let module = Arc::new(module);
            let mut cache = MODULE_CACHE.lock();
            if !cache.contains(&address) {
                cache.put(address, module.clone());
            }
            Ok(module)
        }
    }
}

/// Compiled module for instantiate
#[allow(unused)]
pub struct Module {
    info: Arc<ModuleInfo>,
    data_initializers: Vec<OwnedDataInitializer>,
    func_offsets: PrimaryMap<DefinedFuncIndex, usize>,
    jt_offsets: PrimaryMap<DefinedFuncIndex, ir::JumpTableOffsets>,
    executable: ExecutableBuffer,
    relocs: Relocations,
    traps: Traps,
    trap_registration_guards: Vec<TrapRegistrationGuard>,
}

impl Module {
    pub fn instantiate(self: Arc<Self>, resolver: &mut dyn Resolver) -> Result<Instance, Error> {
        let module_info = self.info.clone();
        let imports =
            {
                let mut imports = PrimaryMap::new();
                for (module, func) in module_info.imported_funcs.values() {
                    imports.push(resolver.resolve(module, func).unwrap_or_else(|| {
                        panic!("can not resolve import func:{}/{}", module, func)
                    }));
                }

                imports.into_boxed_slice()
            };

        let data_initializers: Vec<_> = self.data_initializers.iter().map(|e| e.into()).collect();
        let functions: PrimaryMap<_, _> = self
            .func_offsets
            .iter()
            .map(|(_index, offset)| &self.executable[*offset] as *const u8 as *const VMFunctionBody)
            .collect();

        let chain = ChainCtx::default();
        let instance = InstanceHandle::new(
            self.info.clone(),
            functions.into_boxed_slice(),
            imports,
            &data_initializers,
            chain.exec_metrics.clone(),
            Box::new(chain),
        )?;

        Ok(Instance {
            module: self.clone(),
            handle: instance,
        })
    }

    pub fn compile(wasm: &[u8], build_option: BuildOption) -> Result<Module, Error> {
        let config = isa::TargetFrontendConfig {
            default_call_conv: isa::CallConv::SystemV,
            pointer_width: PointerWidth::U64,
        };

        let isa_builder = isa::lookup_by_name("x86_64").unwrap();
        let mut flag_builder = settings::builder();
        let _ = flag_builder.set("probestack_enabled", "false");
        let isa = isa_builder.finish(settings::Flags::new(flag_builder));

        let module_environ = ModuleEnvironment::new(config, Tunables::default());
        let result = module_environ
            .translate(&wasm)
            .map_err(|e| Error::Compile(CompileError::Wasm(e)))?;

        let (compilation, relocs, _address_transform, _value_ranges, _stack_slots, traps) =
            compile_module(
                &result.module,
                &result.translate_state,
                result.function_body_inputs,
                &*isa,
                false,
                build_option,
            )
            .map_err(Error::Compile)?;

        let total_code_size = (&compilation).into_iter().map(|code| code.body.len()).sum();
        let mut exec = MutableBuffer::new(total_code_size)
            .map_err(|_| Error::Internal("allocate mmap memory failed".to_string()))?;
        let mut finished_functions = PrimaryMap::new();
        let mut func_offsets = PrimaryMap::with_capacity(compilation.len());
        for code in &compilation {
            let curr_size = exec.len();
            exec.set_len(curr_size + code.body.len());
            exec[curr_size..].copy_from_slice(&code.body);
            func_offsets.push(curr_size);
            finished_functions.push(&exec[curr_size] as *const u8 as *const VMFunctionBody);
        }

        let jt_offsets = compilation.get_jt_offsets();

        linker::link_module(&result.module, &finished_functions, &jt_offsets, &relocs)?;

        let mut trap_registration_guards = vec![];
        register_traps(&finished_functions, &traps, &mut trap_registration_guards);

        let executable = exec
            .make_exec()
            .map_err(|_| Error::Internal("failed to set memory executable".to_string()))?;

        Ok(Module {
            info: Arc::new(result.module),
            data_initializers: result
                .data_initializers
                .into_iter()
                .map(|e| e.into_owned())
                .collect(),
            func_offsets,
            executable,
            jt_offsets,
            relocs,
            traps,
            trap_registration_guards,
        })
    }

    pub fn dump(&self) {
        println!("relocations result");
        for (func, reloc) in self.relocs.iter() {
            println!("reloc for func {:?}", func);
            for (i, rel) in reloc.iter().enumerate() {
                println!("reloc:{} for func {:?}", i, rel);
            }
        }
    }
}

impl Drop for Module {
    fn drop(&mut self) {
        // We must deregister traps before freeing the executable memory.
        self.trap_registration_guards.clear();
    }
}

fn register_traps(
    allocated_functions: &PrimaryMap<DefinedFuncIndex, *const VMFunctionBody>,
    traps: &Traps,
    trap_registration_guards: &mut Vec<TrapRegistrationGuard>,
) {
    let mut trap_registry = get_mut_trap_registry();
    for (func_addr, func_traps) in allocated_functions.values().zip(traps.values()) {
        for trap_desc in func_traps.iter() {
            let func_addr = *func_addr as *const u8 as usize;
            let offset = usize::try_from(trap_desc.code_offset).unwrap();
            let trap_addr = func_addr + offset;
            let guard =
                trap_registry.register_trap(trap_addr, trap_desc.source_loc, trap_desc.trap_code);
            trap_registration_guards.push(guard);
        }
    }
}
