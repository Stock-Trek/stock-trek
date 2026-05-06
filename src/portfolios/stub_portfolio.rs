use crate::{
    portfolios::portfolio::{Portfolio, PortfolioTrait},
    scratch::key::{ExchangeName, TokenName},
};

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
    fn has_account_in_exchange(&self, _exchange: &ExchangeName) -> bool {
        true
    }
    fn owns_token(&self, _token: &TokenName) -> bool {
        true
    }
    fn owns_token_in_exchange(&self, _token: &TokenName, _exchange: &ExchangeName) -> bool {
        true
    }
    fn token_total(&self, _token: &TokenName) -> f64 {
        1_000_000.0
    }
    fn token_in_exchange(&self, _token: &TokenName, _exchange: &ExchangeName) -> f64 {
        1_000_000.0
    }
}
