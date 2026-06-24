use stock_trek_types::cex::capability::CexCapability;

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
