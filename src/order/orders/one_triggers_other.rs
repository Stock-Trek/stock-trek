use crate::{
    asset_id::AssetId,
    capability::{combine_capabilities, Capability, HasRequiredCapabilities, MultiLegCapability},
    commands::resolveable::Resolvable,
    error::result::StockTrekResult,
    order::orders::single::SingleOrderGeneric,
    resolved_context::ResolvedContext,
    values::value::{AssetIdValue, NumberValue},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct OneTriggersOtherOrderGeneric<A, N> {
    pub primary: SingleOrderGeneric<A, N>,
    pub secondary: SingleOrderGeneric<A, N>,
}

pub type OneTriggersOtherOrderRaw = OneTriggersOtherOrderGeneric<AssetIdValue, NumberValue>;
pub type OneTriggersOtherOrder = OneTriggersOtherOrderGeneric<AssetId, f64>;

impl Resolvable<OneTriggersOtherOrder> for OneTriggersOtherOrderRaw {
    fn try_resolve(&self, context: &ResolvedContext) -> StockTrekResult<OneTriggersOtherOrder> {
        Ok(OneTriggersOtherOrder {
            primary: self.primary.try_resolve(context)?,
            secondary: self.secondary.try_resolve(context)?,
        })
    }
}

impl<A, N> HasRequiredCapabilities for OneTriggersOtherOrderGeneric<A, N> {
    fn required_capabilities(&self) -> Vec<Capability> {
        let mut required_capabilities = combine_capabilities(&[&self.primary, &self.secondary]);
        required_capabilities.push(Capability::MultiLeg(MultiLegCapability::OneTriggersOther));
        required_capabilities
    }
}
