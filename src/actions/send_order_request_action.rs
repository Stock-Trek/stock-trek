use crate::{
    actions::{
        action::{Action, ActionTrait},
        resolved_action::ResolvedAction,
    },
    cex::capability::HasRequiredCapabilities,
    error::result::StockTrekResult,
    resolveable::Resolvable,
    resolved_context::ResolvedContext,
    values::value::{AssetIdValue, CexIdValue, NumberValue},
};
use serde::{Deserialize, Serialize};
use stock_trek_types::cex::{capability::CexCapability, order_request::OrderRequest};

#[derive(Serialize, Deserialize)]
pub struct SendOrderRequestAction {
    cex_id_value: CexIdValue,
    order_request: OrderRequest<AssetIdValue, NumberValue>,
}

impl SendOrderRequestAction {
    pub fn new(
        cex_id_value: CexIdValue,
        order_request: OrderRequest<AssetIdValue, NumberValue>,
    ) -> Action {
        Box::new(Self {
            cex_id_value,
            order_request,
        })
    }
}

#[typetag::serde]
impl ActionTrait for SendOrderRequestAction {}

impl Resolvable<ResolvedAction> for SendOrderRequestAction {
    fn try_resolve(&self, c: &ResolvedContext) -> StockTrekResult<ResolvedAction> {
        let cex_id = self.cex_id_value.cex_id(c)?;
        let order_request = self.order_request.try_resolve(c)?;
        Ok(ResolvedAction::PlaceOrder {
            cex_id,
            order_request,
        })
    }
}

impl HasRequiredCapabilities for SendOrderRequestAction {
    fn required_capabilities(&self) -> Vec<CexCapability> {
        self.order_request.required_capabilities()
    }
}
