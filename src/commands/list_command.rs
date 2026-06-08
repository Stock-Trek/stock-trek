use crate::{
    cex::capability::{CexCapability, HasRequiredCapabilities},
    commands::command::{Command, CommandTrait},
    error::result::StockTrekResult,
    resolved_context::ResolvedContext,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ListCommand {
    commands: Vec<Command>,
}

impl ListCommand {
    pub fn new(commands: Vec<Command>) -> Command {
        Box::new(Self { commands })
    }
}

#[typetag::serde]
impl CommandTrait for ListCommand {
    fn execute(&self, c: &ResolvedContext) -> StockTrekResult<()> {
        for command in &self.commands {
            command.execute(c)?;
        }
        Ok(())
    }
}

impl HasRequiredCapabilities for ListCommand {
    fn required_capabilities(&self) -> Vec<CexCapability> {
        let mut capabilities = Vec::new();
        for command in &self.commands {
            capabilities.extend(command.required_capabilities());
        }
        capabilities
    }
}
