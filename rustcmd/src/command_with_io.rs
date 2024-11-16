use std::path::PathBuf;

use crate::{command::Command, redirections::Redirections};

pub struct CommandWithIo {
    command: Command,
    redirections: Redirections,
    input: Option<PathBuf>,
}
