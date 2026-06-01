use crate::{
    asset_id::AssetId,
    capability::{Capability, HasRequiredCapabilities},
    commands::resolveable::Resolvable,
    error::result::StockTrekResult,
    order::orders::{
        one_cancels_other::OneCancelsOtherOrderGeneric,
        one_triggers_oco::OneTriggersOcoOrderGeneric,
        one_triggers_other::OneTriggersOtherOrderGeneric, single::SingleOrderGeneric,
    },
    resolved_context::ResolvedContext,
    values::value::{AssetIdValue, NumberValue},
};
use serde::{Deserialize, Serialize};
use std::hash::Hash;
use strum::Display;

#[derive(Debug, Display, Clone, Hash, Serialize, Deserialize)]
pub enum OrderRequest<A, N> {
    Single(SingleOrderGeneric<A, N>),
    OneCancelsOther(OneCancelsOtherOrderGeneric<A, N>),
    OneTriggersOther(OneTriggersOtherOrderGeneric<A, N>),
    OneTriggersOco(OneTriggersOcoOrderGeneric<A, N>),
}

impl Resolvable<OrderRequest<AssetId, f64>> for OrderRequest<AssetIdValue, NumberValue> {
    fn try_resolve(
        &self,
        context: &ResolvedContext,
    ) -> StockTrekResult<OrderRequest<AssetId, f64>> {
        match self {
            Self::Single(order) => Ok(OrderRequest::Single(order.try_resolve(context)?)),
            Self::OneCancelsOther(order) => {
                Ok(OrderRequest::OneCancelsOther(order.try_resolve(context)?))
            }
            Self::OneTriggersOther(order) => {
                Ok(OrderRequest::OneTriggersOther(order.try_resolve(context)?))
            }
            Self::OneTriggersOco(order) => {
                Ok(OrderRequest::OneTriggersOco(order.try_resolve(context)?))
            }
        }
    }
}

impl<A, N> HasRequiredCapabilities for OrderRequest<A, N> {
    fn required_capabilities(&self) -> Vec<Capability> {
        match self {
            Self::Single(order) => order.required_capabilities(),
            Self::OneCancelsOther(order) => order.required_capabilities(),
            Self::OneTriggersOther(order) => order.required_capabilities(),
            Self::OneTriggersOco(order) => order.required_capabilities(),
        }
    }
}
