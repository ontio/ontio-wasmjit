use failure::Fail;
use ontio_wasmjit_environ::CompileError;
use ontio_wasmjit_runtime::InstantiationError;

#[derive(Fail, Debug)]
pub enum Error {
    /// todo: update comments Insufficient resources available for execution.
    #[fail(display = "internal error: {}", _0)]
    Internal(String),

    #[fail(display = "compile error: {}", _0)]
    Compile(CompileError),

    #[fail(display = "link error: {}", _0)]
    Link(String),
}

impl From<InstantiationError> for Error {
    fn from(err: InstantiationError) -> Self {
        match err {
            InstantiationError::Resource(err) => {
                Error::Internal(format!("Insufficient resources: {}", err))
            }
            InstantiationError::Link(err) => Error::Link(err.0),
            // this case should never happen, so convert to internal
            InstantiationError::StartTrap(err) => Error::Internal(err),
        }
    }
}
