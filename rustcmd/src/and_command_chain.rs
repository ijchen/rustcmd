use crate::command_with_io::CommandWithIo;

pub struct AndCommandChain {
    commands: Vec<CommandWithIo>,
}
