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
use stock_trek_types::cex::{capability::CexCapability, tag::Tag};

#[derive(Serialize, Deserialize)]
pub struct CancelAllOrdersWithIdAction {
    tag: Tag,
}

impl CancelAllOrdersWithIdAction {
    pub fn new(tag: Tag) -> Action {
        Box::new(Self { tag })
    }
}

#[typetag::serde]
impl ActionTrait for CancelAllOrdersWithIdAction {}

impl Resolvable<ResolvedAction> for CancelAllOrdersWithIdAction {
    fn try_resolve(&self, _c: &ResolvedContext) -> StockTrekResult<ResolvedAction> {
        let order_tag = self.tag.clone();
        Ok(ResolvedAction::CancelAllOrdersWithTag { tag: order_tag })
    }
}

impl HasRequiredCapabilities for CancelAllOrdersWithIdAction {
    fn required_capabilities(&self) -> Vec<CexCapability> {
        vec![]
    }
}
