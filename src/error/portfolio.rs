use digdigdig3::{Asset, ExchangeId};
use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
#[repr(u8)]
pub enum PortfolioError {
    #[error("Portfolio has no account in {}", exchange.as_str().to_string())]
    NoAccountInExchange { exchange: ExchangeId },
    #[error("Portfolio does not own any {} in {}", asset, exchange.as_str().to_string())]
    AssetNotOwned { exchange: ExchangeId, asset: Asset },
    #[error("Portfolio has {} {} in {} and cannot sell {}", owned, asset, exchange.as_str().to_string(), quantity)]
    NotEnoughTokens {
        exchange: ExchangeId,
        asset: Asset,
        owned: f64,
        quantity: f64,
    },
}
