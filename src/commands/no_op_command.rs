use crate::{
    capability::{Capability, HasRequiredCapabilities},
    commands::command::{Command, CommandTrait},
    error::result::StockTrekResult,
    resolved_context::ResolvedContext,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NoOpCommand;

impl NoOpCommand {
    pub fn new() -> Command {
        Box::new(Self {})
    }
}

#[typetag::serde]
impl CommandTrait for NoOpCommand {
    fn resolve(&self, _: &ResolvedContext) -> StockTrekResult<()> {
        Ok(())
    }
}

impl HasRequiredCapabilities for NoOpCommand {
    fn required_capabilities(&self) -> Vec<Capability> {
        Vec::new()
    }
}
