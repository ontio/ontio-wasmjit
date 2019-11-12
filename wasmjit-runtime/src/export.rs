use crate::vmcontext::{VMContext, VMFunctionBody};
use cranelift_codegen::ir;

/// The value of an export passed from one instance to another.
#[derive(Debug, Clone)]
pub struct ExportFunc {
    /// The address of the native-code function.
    pub address: *const VMFunctionBody,
    /// Pointer to the containing `VMContext`.
    pub vmctx: *mut VMContext,
    /// The function signature declaration, used for compatibilty checking.
    pub signature: ir::Signature,
}

impl ExportFunc {
    /// Construct a function export value.
    pub fn new(
        address: *const VMFunctionBody,
        vmctx: *mut VMContext,
        signature: ir::Signature,
    ) -> Self {
        Self {
            address,
            vmctx,
            signature,
        }
    }
}
