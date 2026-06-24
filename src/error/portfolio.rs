use stock_trek_types::cex::{asset_id::AssetId, cex_id::CexId};
use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
#[repr(u8)]
pub enum PortfolioError {
    #[error("Portfolio has no account in {}", cex_id)]
    NoAccountInCex { cex_id: CexId },
    #[error("Portfolio does not own any {} in {}", asset_id, cex_id)]
    AssetNotOwned { cex_id: CexId, asset_id: AssetId },
    #[error(
        "Portfolio has {} {} in {} and cannot sell {}",
        owned,
        asset_id,
        cex_id,
        quantity
    )]
    NotEnoughAssetsInCex {
        cex_id: CexId,
        asset_id: AssetId,
        owned: f64,
        quantity: f64,
    },
}
