use crate::scratch::key::{ExchangeName, TokenName};
use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
#[repr(u8)]
pub enum PortfolioError {
    #[error("Portfolio has no account in {}", exchange)]
    NoAccountInExchange { exchange: ExchangeName },
    #[error("Portfolio does not own any {} in {}", token, exchange)]
    TokenNotOwned {
        exchange: ExchangeName,
        token: TokenName,
    },
    #[error(
        "Portfolio has {} {} in {} and cannot sell {}",
        owned,
        token,
        exchange,
        quantity
    )]
    NotEnoughTokens {
        exchange: ExchangeName,
        token: TokenName,
        owned: f64,
        quantity: f64,
    },
}
