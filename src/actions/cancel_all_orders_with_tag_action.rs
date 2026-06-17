use crate::{
    actions::{
        action::{Action, ActionTrait},
        resolved_action::ResolvedAction,
    },
    cex::{
        capability::{CexCapability, HasRequiredCapabilities},
        order_tag::OrderTag,
    },
    error::result::StockTrekResult,
    resolveable::Resolvable,
    resolved_context::ResolvedContext,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CancelAllOrdersWithIdAction {
    order_tag: OrderTag,
}

impl CancelAllOrdersWithIdAction {
    pub fn new(order_tag: OrderTag) -> Action {
        Box::new(Self { order_tag })
    }
}

#[typetag::serde]
impl ActionTrait for CancelAllOrdersWithIdAction {}

impl Resolvable<ResolvedAction> for CancelAllOrdersWithIdAction {
    fn try_resolve(&self, _c: &ResolvedContext) -> StockTrekResult<ResolvedAction> {
        let order_tag = self.order_tag.clone();
        Ok(ResolvedAction::CancelAllOrdersWithTag { order_tag })
    }
}

impl HasRequiredCapabilities for CancelAllOrdersWithIdAction {
    fn required_capabilities(&self) -> Vec<CexCapability> {
        vec![]
    }
}
