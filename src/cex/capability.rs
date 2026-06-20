use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CexCapability {
    // MultiLeg(MultiLegCexCapability),
    QuoteQuantity(QuoteQuantityCexCapability),
}

// #[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
// pub enum MultiLegCexCapability {
//     AllowDifferentSymbol,
//     AllowDifferentPricing,
//     AllowDifferentTiming,
//     OneCancelsOther,
//     OneTriggersOther,
//     OneTriggersOco,
// }

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum QuoteQuantityCexCapability {
    AllowTriggeredTiming,
    AllowLimitPricing,
}

pub trait HasRequiredCapabilities {
    fn required_capabilities(&self) -> Vec<CexCapability>;
}

pub fn combine_capabilities<T: HasRequiredCapabilities + ?Sized>(
    array: &[&T],
) -> Vec<CexCapability> {
    let mut capabilities = Vec::new();
    for element in array {
        capabilities.extend(element.required_capabilities());
    }
    capabilities
}
