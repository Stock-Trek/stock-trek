use crate::{
    actions::{
        action::{Action, ActionTrait},
        resolved_action::ResolvedAction,
    },
    capability::{Capability, HasRequiredCapabilities},
    error::result::StockTrekResult,
    order::order_request::OrderRequest,
    resolveable::Resolvable,
    resolved_context::ResolvedContext,
    values::value::{AssetIdValue, ExchangeIdValue, NumberValue},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PlaceOrderAction {
    exchange_id_value: ExchangeIdValue,
    order_request: OrderRequest<AssetIdValue, NumberValue>,
    stale_out_millis: StaleOutMillis,
}

impl PlaceOrderAction {
    pub fn new(
        exchange_id_value: ExchangeIdValue,
        order_request: OrderRequest<AssetIdValue, NumberValue>,
        stale_out_millis: StaleOutMillis,
    ) -> Action {
        Box::new(Self {
            exchange_id_value,
            order_request,
            stale_out_millis,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StaleOutMillis(pub u64);

#[typetag::serde]
impl ActionTrait for PlaceOrderAction {}

impl Resolvable<ResolvedAction> for PlaceOrderAction {
    fn try_resolve(&self, c: &ResolvedContext) -> StockTrekResult<ResolvedAction> {
        let exchange_id = self.exchange_id_value.exchange_id(c)?;
        let order_request = self.order_request.try_resolve(c)?;
        let stale_out_millis = self.stale_out_millis;
        Ok(ResolvedAction::PlaceOrder {
            exchange_id,
            order_request,
            stale_out_millis,
        })
    }
}

impl HasRequiredCapabilities for PlaceOrderAction {
    fn required_capabilities(&self) -> Vec<Capability> {
        self.order_request.required_capabilities()
    }
}
