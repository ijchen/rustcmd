use std::path::PathBuf;

pub struct Redirections {
    stdout: RedirectionDestination,
    stderr: RedirectionDestination,
}

pub enum RedirectionDestination {
    Stdin,
    Stdout,
    Stderr,
    Null,
    Overwrite(PathBuf),
    AppendTo(PathBuf),
}
