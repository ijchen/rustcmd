use std::process::Output;

use crate::{command_segment::CommandSegment, ExecuteError};

pub struct Command {
    // TODO: just pub for temporary testing
    pub(crate) segments: Vec<CommandSegment>,
}

impl Command {
    pub fn run(&self) -> crate::Result<Output> {
        let segments: Vec<_> = self
            .segments
            .iter()
            .flat_map(CommandSegment::as_strs)
            .collect::<crate::Result<_>>()?;

        // Separate the program from its arguments ("cargo" from ["run", "-r"])
        let [program, args @ ..] = segments.as_slice() else {
            return Err(ExecuteError::EmptyCommand);
        };

        let mut command = std::process::Command::new(&**program);

        for arg in args {
            command.arg(&**arg);
        }

        // TODO: set working directory
        // command.current_dir(dir);

        // TODO: set environment variables
        // command.envs(vars);

        // TODO: set stdin
        // command.stdin(cfg);

        // TODO: allow spawning instead of blocking
        // command.spawn();

        Ok(command.output()?)
    }
}
