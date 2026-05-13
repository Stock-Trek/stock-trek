use crate::{
    asset_id::AssetId,
    capability::{Capability, HasRequiredCapabilities},
    error::result::StockTrekResult,
    order::order_request::OrderRequest,
    resolved_context::ResolvedContext,
    resolvers::{
        resolveable::Resolvable,
        resolver::{Resolver, ResolverTrait},
    },
    values::value::{AssetIdValue, ExchangeIdValue, NumberValue},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct EnqueueOrderResolver {
    exchange_id_value: ExchangeIdValue,
    order_request: OrderRequest<AssetIdValue, NumberValue>,
}

impl EnqueueOrderResolver {
    pub fn new(
        exchange_id_value: ExchangeIdValue,
        order_request: OrderRequest<AssetIdValue, NumberValue>,
    ) -> Resolver {
        Box::new(Self {
            exchange_id_value,
            order_request,
        })
    }
}

#[typetag::serde]
impl ResolverTrait for EnqueueOrderResolver {
    fn resolve(
        &self,
        c: &ResolvedContext,
        order_requests: &mut Vec<OrderRequest<AssetId, f64>>,
    ) -> StockTrekResult<()> {
        let resolved_order_request = self.order_request.try_resolve(c)?;
        order_requests.push(resolved_order_request);
        Ok(())
    }
}

impl HasRequiredCapabilities for EnqueueOrderResolver {
    fn required_capabilities(&self) -> Vec<Capability> {
        self.order_request.required_capabilities()
    }
}
