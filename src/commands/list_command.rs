use crate::{
    capability::{Capability, HasRequiredCapabilities},
    commands::command::{Command, CommandTrait},
    error::result::StockTrekResult,
    resolved_context::ResolvedContext,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ListCommand {
    resolvers: Vec<Command>,
}

impl ListCommand {
    pub fn new(resolvers: Vec<Command>) -> Command {
        Box::new(Self { resolvers })
    }
}

#[typetag::serde]
impl CommandTrait for ListCommand {
    fn resolve(&self, c: &ResolvedContext) -> StockTrekResult<()> {
        for resolver in &self.resolvers {
            resolver.resolve(c)?;
        }
        Ok(())
    }
}

impl HasRequiredCapabilities for ListCommand {
    fn required_capabilities(&self) -> Vec<Capability> {
        let mut capabilities = Vec::new();
        for resolver in &self.resolvers {
            capabilities.extend(resolver.required_capabilities());
        }
        capabilities
    }
}
