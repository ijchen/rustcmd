mod and_command_chain;
mod command;
mod command_segment;
mod command_sequence;
mod command_with_io;
mod env_unset_handler;
mod execute_error;
mod or_command_chain;
mod redirections;

pub use and_command_chain::AndCommandChain;
pub use command::Command;
pub use command_segment::CommandSegment;
pub use command_sequence::CommandSequence;
pub use command_with_io::CommandWithIo;
pub use env_unset_handler::EnvUnsetHandler;
pub use execute_error::ExecuteError;
pub use or_command_chain::OrCommandChain;
pub use redirections::{RedirectionDestination, Redirections};

pub type Result<T> = std::result::Result<T, ExecuteError>;

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: just a temporary test
    #[test]
    fn test_thingy() {
        let output = Command {
            segments: vec![
                CommandSegment::SplitWhitespace("echo".to_string()),
                CommandSegment::Literally("Hello, world!".to_string()),
            ],
        }
        .run()
        .unwrap();

        assert!(output.status.success());
        assert!(output.stdout == b"Hello, world!\n");
        assert!(output.stderr.is_empty());
    }
}
