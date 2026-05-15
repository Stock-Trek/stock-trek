use crate::exchange_id::ExchangeId;
use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Capability {
    MultiLeg(MultiLegCapability),
    QuoteQuantity(QuoteQuantityCapability),
}

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MultiLegCapability {
    AllowDifferentSymbol,
    AllowDifferentPricing,
    AllowDifferentTiming,
    OneCancelsOther,
    OneTriggersOther,
    OneTriggersOco,
}

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum QuoteQuantityCapability {
    AllowTriggeredTiming,
    AllowLimitPricing,
}

pub trait Capabilities {
    fn exchange_id(&self) -> ExchangeId;
    fn capabilities(&self) -> Vec<Capability>;
}

pub trait HasRequiredCapabilities {
    fn required_capabilities(&self) -> Vec<Capability>;
}

pub fn combine_capabilities<T: HasRequiredCapabilities + ?Sized>(array: &[&T]) -> Vec<Capability> {
    let mut capabilities = Vec::new();
    for element in array {
        capabilities.extend(element.required_capabilities());
    }
    capabilities
}
