use crate::portfolios::portfolio::{Portfolio, PortfolioTrait};
use digdigdig3::{Asset, ExchangeId};

#[derive(Debug, Clone, Default)]
pub struct StubPortfolio;

impl StubPortfolio {
    pub fn new() -> Self {
        Self
    }
}

impl From<StubPortfolio> for Portfolio {
    fn from(value: StubPortfolio) -> Self {
        Box::new(value)
    }
}

impl PortfolioTrait for StubPortfolio {
    fn has_account_in_exchange(&self, _exchange: &ExchangeId) -> bool {
        true
    }
    fn owns_asset(&self, _asset: &Asset) -> bool {
        true
    }
    fn owns_asset_in_exchange(&self, _asset: &Asset, _exchange: &ExchangeId) -> bool {
        true
    }
    fn asset_total(&self, _asset: &Asset) -> f64 {
        1_000_000.0
    }
    fn asset_in_exchange(&self, _asset: &Asset, _exchange: &ExchangeId) -> f64 {
        1_000_000.0
    }
}
