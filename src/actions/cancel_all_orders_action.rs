use crate::{
    actions::{
        action::{Action, ActionTrait},
        resolved_action::ResolvedAction,
    },
    cex::capability::HasRequiredCapabilities,
    error::result::StockTrekResult,
    resolveable::Resolvable,
    resolved_context::ResolvedContext,
};
use serde::{Deserialize, Serialize};
use stock_trek_types::cex::capability::CexCapability;

#[derive(Serialize, Deserialize)]
pub struct CancelAllOrdersAction {}

impl CancelAllOrdersAction {
    pub fn new() -> Action {
        Box::new(Self {})
    }
}

#[typetag::serde]
impl ActionTrait for CancelAllOrdersAction {}

impl Resolvable<ResolvedAction> for CancelAllOrdersAction {
    fn try_resolve(&self, _c: &ResolvedContext) -> StockTrekResult<ResolvedAction> {
        Ok(ResolvedAction::CancelAllOrders)
    }
}

impl HasRequiredCapabilities for CancelAllOrdersAction {
    fn required_capabilities(&self) -> Vec<CexCapability> {
        vec![]
    }
}
