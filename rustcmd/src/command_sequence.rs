use crate::or_command_chain::OrCommandChain;

pub struct CommandSequence {
    commands: Vec<OrCommandChain>,
}

impl CommandSequence {
    pub fn new(commands: Vec<OrCommandChain>) -> Self {
        Self { commands }
    }
}
