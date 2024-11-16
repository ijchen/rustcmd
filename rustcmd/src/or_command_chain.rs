use crate::and_command_chain::AndCommandChain;

pub struct OrCommandChain {
    commands: Vec<AndCommandChain>,
}
