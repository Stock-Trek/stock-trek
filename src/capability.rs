use crate::exchange_id::ExchangeId;

pub enum Capability {
    MultiLeg(MultiLegCapability),
    QuoteQuantity(QuoteQuantityCapability),
}

pub enum MultiLegCapability {
    AllowDifferentSymbol,
    AllowDifferentPricing,
    AllowDifferentTiming,
    OneCancelsOther,
    OneTriggersOther,
    OneTriggersOco,
}

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
