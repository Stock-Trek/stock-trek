use crate::cex::{asset_id::AssetId, exchange_id::ExchangeId};
use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
#[repr(u8)]
pub enum PortfolioError {
    #[error("Portfolio has no account in {}", exchange)]
    NoAccountInExchange { exchange: ExchangeId },
    #[error("Portfolio does not own any {} in {}", asset_id, exchange_id)]
    AssetNotOwned {
        exchange_id: ExchangeId,
        asset_id: AssetId,
    },
    #[error(
        "Portfolio has {} {} in {} and cannot sell {}",
        owned,
        asset_id,
        exchange_id,
        quantity
    )]
    NotEnoughAssets {
        exchange_id: ExchangeId,
        asset_id: AssetId,
        owned: f64,
        quantity: f64,
    },
}
