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
    values::value::CexIdValue,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CancelAllOrdersInCexWithIdAction {
    cex_id_value: CexIdValue,
    order_tag: OrderTag,
}

impl CancelAllOrdersInCexWithIdAction {
    pub fn new(cex_id_value: CexIdValue, order_tag: OrderTag) -> Action {
        Box::new(Self {
            cex_id_value,
            order_tag,
        })
    }
}

#[typetag::serde]
impl ActionTrait for CancelAllOrdersInCexWithIdAction {}

impl Resolvable<ResolvedAction> for CancelAllOrdersInCexWithIdAction {
    fn try_resolve(&self, c: &ResolvedContext) -> StockTrekResult<ResolvedAction> {
        let cex_id = self.cex_id_value.cex_id(c)?;
        let order_tag = self.order_tag.clone();
        Ok(ResolvedAction::CancelAllOrdersInCexWithTag { cex_id, order_tag })
    }
}

impl HasRequiredCapabilities for CancelAllOrdersInCexWithIdAction {
    fn required_capabilities(&self) -> Vec<CexCapability> {
        vec![]
    }
}
