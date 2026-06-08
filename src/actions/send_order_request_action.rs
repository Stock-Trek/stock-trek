use crate::{
    actions::{
        action::{Action, ActionTrait},
        resolved_action::ResolvedAction,
    },
    cex::{
        capability::{Capability, HasRequiredCapabilities},
        order_request::OrderRequest,
    },
    error::result::StockTrekResult,
    resolveable::Resolvable,
    resolved_context::ResolvedContext,
    values::value::{AssetIdValue, ExchangeIdValue, NumberValue},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SendOrderRequestAction {
    exchange_id_value: ExchangeIdValue,
    order_request: OrderRequest<AssetIdValue, NumberValue>,
}

impl SendOrderRequestAction {
    pub fn new(
        exchange_id_value: ExchangeIdValue,
        order_request: OrderRequest<AssetIdValue, NumberValue>,
    ) -> Action {
        Box::new(Self {
            exchange_id_value,
            order_request,
        })
    }
}

#[typetag::serde]
impl ActionTrait for SendOrderRequestAction {}

impl Resolvable<ResolvedAction> for SendOrderRequestAction {
    fn try_resolve(&self, c: &ResolvedContext) -> StockTrekResult<ResolvedAction> {
        let exchange_id = self.exchange_id_value.exchange_id(c)?;
        let order_request = self.order_request.try_resolve(c)?;
        Ok(ResolvedAction::PlaceOrder {
            exchange_id,
            order_request,
        })
    }
}

impl HasRequiredCapabilities for SendOrderRequestAction {
    fn required_capabilities(&self) -> Vec<Capability> {
        self.order_request.required_capabilities()
    }
}
