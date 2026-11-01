use stock_trek_types::cex::capability::CexCapability;

/// A type that declares the CEX capabilities it requires.
pub trait HasRequiredCapabilities {
    /// Returns the capabilities required by this value.
    fn required_capabilities(&self) -> Vec<CexCapability>;
}

/// Combines the required capabilities of all values in `array`.
pub fn combine_capabilities<T: HasRequiredCapabilities + ?Sized>(
    array: &[&T],
) -> Vec<CexCapability> {
    let mut capabilities = Vec::new();
    for element in array {
        capabilities.extend(element.required_capabilities());
    }
    capabilities
}
