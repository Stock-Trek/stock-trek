use crate::{
    actions::recoverable_action::RecoverableAction,
    capability::{Capability, HasRequiredCapabilities},
    commands::command::{Command, CommandTrait},
    error::result::StockTrekResult,
    resolved_context::ResolvedContext,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PlanCommand {
    actions: Vec<RecoverableAction>,
}

impl PlanCommand {
    pub fn new(actions: Vec<RecoverableAction>) -> Command {
        Box::new(Self { actions })
    }
}

#[typetag::serde]
impl CommandTrait for PlanCommand {
    fn execute(&self, c: &ResolvedContext) -> StockTrekResult<()> {
        for action in &self.actions {
            action.enqueue(c)?;
        }
        Ok(())
    }
}

impl HasRequiredCapabilities for PlanCommand {
    fn required_capabilities(&self) -> Vec<Capability> {
        let mut capabilities = Vec::new();
        for action in &self.actions {
            capabilities.extend(action.required_capabilities());
        }
        capabilities
    }
}
