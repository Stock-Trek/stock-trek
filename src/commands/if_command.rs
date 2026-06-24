use crate::{
    cex::capability::{HasRequiredCapabilities, combine_capabilities},
    commands::command::{Command, CommandTrait},
    conditions::condition::Condition,
    error::result::StockTrekResult,
    resolved_context::ResolvedContext,
};
use serde::{Deserialize, Serialize};
use stock_trek_types::cex::capability::CexCapability;

#[derive(Serialize, Deserialize)]
pub struct IfCommand {
    condition: Condition,
    if_true: Command,
    if_false: Command,
}

impl IfCommand {
    pub fn new(condition: Condition, if_true: Command, if_false: Command) -> Command {
        Box::new(Self {
            condition,
            if_true,
            if_false,
        })
    }
}

#[typetag::serde]
impl CommandTrait for IfCommand {
    fn execute(&self, c: &ResolvedContext) -> StockTrekResult<()> {
        let condition = self.condition.test(c)?;
        if condition {
            self.if_true.execute(c)?;
        } else {
            self.if_false.execute(c)?;
        }
        Ok(())
    }
}

impl HasRequiredCapabilities for IfCommand {
    fn required_capabilities(&self) -> Vec<CexCapability> {
        combine_capabilities(&[self.if_false.as_ref(), self.if_true.as_ref()])
    }
}
