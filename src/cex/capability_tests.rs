use super::capability::{HasRequiredCapabilities, combine_capabilities};
use stock_trek_types::cex::capability::CexCapability;

struct WithCapabilities(Vec<CexCapability>);

impl HasRequiredCapabilities for WithCapabilities {
    fn required_capabilities(&self) -> Vec<CexCapability> {
        self.0.clone()
    }
}

#[test]
fn combines_empty_capabilities() {
    let value = WithCapabilities(Vec::new());

    assert!(combine_capabilities([&value]).is_empty());
    assert!(combine_capabilities(std::iter::empty::<&WithCapabilities>()).is_empty());
}

#[test]
fn combines_capabilities_from_multiple_values() {
    let first = WithCapabilities(vec![CexCapability::QuoteQuantityOnLimitOrders]);
    let second = WithCapabilities(vec![CexCapability::QuoteQuantityOnLimitOrders]);

    assert_eq!(
        combine_capabilities([&first, &second]),
        vec![CexCapability::QuoteQuantityOnLimitOrders]
    );
}

#[test]
fn removes_duplicate_capabilities_from_a_single_value() {
    let value = WithCapabilities(vec![
        CexCapability::QuoteQuantityOnLimitOrders,
        CexCapability::QuoteQuantityOnLimitOrders,
    ]);

    assert_eq!(
        combine_capabilities([&value]),
        vec![CexCapability::QuoteQuantityOnLimitOrders]
    );
}
