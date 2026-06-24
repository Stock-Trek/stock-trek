use crate::{
    actions::{
        action::{Action, ActionTrait},
        resolved_action::ResolvedAction,
    },
    cex::capability::HasRequiredCapabilities,
    error::result::StockTrekResult,
    resolveable::Resolvable,
    resolved_context::ResolvedContext,
    values::value::CexIdValue,
};
use serde::{Deserialize, Serialize};
use stock_trek_types::cex::{capability::CexCapability, tag::Tag};

#[derive(Serialize, Deserialize)]
pub struct CancelAllOrdersInCexWithIdAction {
    cex_id_value: CexIdValue,
    tag: Tag,
}

impl CancelAllOrdersInCexWithIdAction {
    pub fn new(cex_id_value: CexIdValue, tag: Tag) -> Action {
        Box::new(Self { cex_id_value, tag })
    }
}

#[typetag::serde]
impl ActionTrait for CancelAllOrdersInCexWithIdAction {}

impl Resolvable<ResolvedAction> for CancelAllOrdersInCexWithIdAction {
    fn try_resolve(&self, c: &ResolvedContext) -> StockTrekResult<ResolvedAction> {
        let cex_id = self.cex_id_value.cex_id(c)?;
        let tag = self.tag.clone();
        Ok(ResolvedAction::CancelAllOrdersInCexWithTag { cex_id, tag })
    }
}

impl HasRequiredCapabilities for CancelAllOrdersInCexWithIdAction {
    fn required_capabilities(&self) -> Vec<CexCapability> {
        vec![]
    }
}
