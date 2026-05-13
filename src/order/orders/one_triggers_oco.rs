use crate::{
    asset_id::AssetId,
    capability::{Capability, HasRequiredCapabilities},
    error::result::StockTrekResult,
    order::orders::{one_cancels_other::OneCancelsOtherOrderGeneric, single::SingleOrderGeneric},
    resolved_context::ResolvedContext,
    resolvers::resolveable::Resolvable,
    values::value::{AssetIdValue, NumberValue},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct OneTriggersOcoOrderGeneric<A, N> {
    pub primary: SingleOrderGeneric<A, N>,
    pub oco_order: OneCancelsOtherOrderGeneric<A, N>,
}

pub type OneTriggersOcoOrderRaw = OneTriggersOcoOrderGeneric<AssetIdValue, NumberValue>;
pub type OneTriggersOcoOrder = OneTriggersOcoOrderGeneric<AssetId, f64>;

impl Resolvable<OneTriggersOcoOrder> for OneTriggersOcoOrderRaw {
    fn try_resolve(&self, context: &ResolvedContext) -> StockTrekResult<OneTriggersOcoOrder> {
        Ok(OneTriggersOcoOrder {
            primary: self.primary.try_resolve(context)?,
            oco_order: self.oco_order.try_resolve(context)?,
        })
    }
}

impl<A, N> HasRequiredCapabilities for OneTriggersOcoOrderGeneric<A, N> {
    fn required_capabilities(&self) -> Vec<Capability> {
        let mut capabilities = Vec::new();
        capabilities.extend(self.primary.required_capabilities());
        capabilities.extend(self.oco_order.required_capabilities());
        capabilities
    }
}
