use ontio_wasmjit_runtime::VMFunctionImport;

pub trait Resolver {
    fn resolve(&mut self, module: &str, field: &str) -> Option<VMFunctionImport>;
}

pub use crate::chain_api::ChainResolver;

pub struct NullResolver;

impl Resolver for NullResolver {
    fn resolve(&mut self, _module: &str, _field: &str) -> Option<VMFunctionImport> {
        None
    }
}
