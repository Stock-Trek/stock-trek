use crate::{
    capability::{Capability, HasRequiredCapabilities},
    commands::{
        command::{Command, CommandTrait},
        resolveable::Resolvable,
    },
    error::result::StockTrekResult,
    order::order_request::OrderRequest,
    resolved_context::ResolvedContext,
    values::value::{AssetIdValue, ExchangeIdValue, NumberValue},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct EnqueueOrderCommand {
    exchange_id_value: ExchangeIdValue,
    order_request: OrderRequest<AssetIdValue, NumberValue>,
}

impl EnqueueOrderCommand {
    pub fn new(
        exchange_id_value: ExchangeIdValue,
        order_request: OrderRequest<AssetIdValue, NumberValue>,
    ) -> Command {
        Box::new(Self {
            exchange_id_value,
            order_request,
        })
    }
}

#[typetag::serde]
impl CommandTrait for EnqueueOrderCommand {
    fn resolve(&self, c: &ResolvedContext) -> StockTrekResult<()> {
        let exchange_id = self.exchange_id_value.exchange_id(c)?;
        let resolved_order_request = self.order_request.try_resolve(c)?;
        (c.enqueue_order)(exchange_id, resolved_order_request)
    }
}

impl HasRequiredCapabilities for EnqueueOrderCommand {
    fn required_capabilities(&self) -> Vec<Capability> {
        self.order_request.required_capabilities()
    }
}
