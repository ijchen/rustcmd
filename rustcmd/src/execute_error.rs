use std::io;

// ExecuteError will be used as the E in Result<T, E> often, and I don't want it
// to cause return types to be bloated just to deal with the unhappy path
const _: () = assert!(size_of::<ExecuteError>() <= size_of::<usize>() * 2);

#[derive(Debug, thiserror::Error)]
pub enum ExecuteError {
    #[error("the command to execute was an empty string")]
    EmptyCommand,

    #[error("environment variable was unset: {0}")]
    UnsetEnvVar(Box<String>),

    #[error("io error: {0}")]
    Io(#[from] io::Error),
}
