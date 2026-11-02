use stock_trek_types::cex::capability::CexCapability;

/// A type that declares the CEX capabilities it requires.
pub trait HasRequiredCapabilities {
    /// Returns the capabilities required by this value.
    fn required_capabilities(&self) -> Vec<CexCapability>;
}

/// Combines the required capabilities of all values produced by `values`.
pub fn combine_capabilities<'a, T, I>(values: I) -> Vec<CexCapability>
where
    T: HasRequiredCapabilities + ?Sized + 'a,
    I: IntoIterator<Item = &'a T>,
{
    values
        .into_iter()
        .flat_map(|value| value.required_capabilities())
        .collect()
}
