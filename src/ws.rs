use thiserror::Error;

/// IPCError enumerates all possible errors returned by this module.
#[derive(Error, Debug)]
pub enum IPCError {
    /// Represents a failure to register to IPC more one time.
    #[error("Already registered to IPC")]
    AlreadyRegistered,

    /// Represents a failure to unregister from IPC more one time.
    #[error("Already unregistered from IPC")]
    AlreadyUnregistered,

    /// Represents a failure to register to IPC by internal library.
    #[error("Failed to register to IPC")]
    RegistrationFailed,

    /// Represents a failure to unregister to IPC by internal library.
    #[error("Failed to unregister from IPC")]
    UnregistrationFailed,

    /// Represents other internal errors
    #[error(transparent)]
    Internal(#[from] anyhow::Error)
}

pub fn test() -> Result<(), IPCError> {
    Err(IPCError::RegistrationFailed)
}