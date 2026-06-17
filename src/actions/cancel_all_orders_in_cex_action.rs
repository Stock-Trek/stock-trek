use crate::{
    actions::{
        action::{Action, ActionTrait},
        resolved_action::ResolvedAction,
    },
    cex::capability::{CexCapability, HasRequiredCapabilities},
    error::result::StockTrekResult,
    resolveable::Resolvable,
    resolved_context::ResolvedContext,
    values::value::CexIdValue,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CancelAllOrdersInCexAction {
    cex_id_value: CexIdValue,
}

impl CancelAllOrdersInCexAction {
    pub fn new(cex_id_value: CexIdValue) -> Action {
        Box::new(Self { cex_id_value })
    }
}

#[typetag::serde]
impl ActionTrait for CancelAllOrdersInCexAction {}

impl Resolvable<ResolvedAction> for CancelAllOrdersInCexAction {
    fn try_resolve(&self, c: &ResolvedContext) -> StockTrekResult<ResolvedAction> {
        let cex_id = self.cex_id_value.cex_id(c)?;
        Ok(ResolvedAction::CancelAllOrdersInCex { cex_id })
    }
}

impl HasRequiredCapabilities for CancelAllOrdersInCexAction {
    fn required_capabilities(&self) -> Vec<CexCapability> {
        vec![]
    }
}
