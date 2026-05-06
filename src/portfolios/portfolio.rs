use crate::scratch::key::{ExchangeName, TokenName};

pub type Portfolio = Box<dyn PortfolioTrait>;

pub trait PortfolioTrait {
    fn has_account_in_exchange(&self, exchange: &ExchangeName) -> bool;
    fn owns_token(&self, token: &TokenName) -> bool;
    fn owns_token_in_exchange(&self, token: &TokenName, exchange: &ExchangeName) -> bool;
    fn token_total(&self, token: &TokenName) -> f64;
    fn token_in_exchange(&self, token: &TokenName, exchange: &ExchangeName) -> f64;
}
