use std::fmt;
use stock_trek_types::cex::{asset_id::AssetId, cex_id::CexId};

#[derive(Debug)]
#[non_exhaustive]
#[repr(u8)]
pub enum PortfolioError {
    NoAccountInCex {
        cex_id: CexId,
    },
    AssetNotOwned {
        cex_id: CexId,
        asset_id: AssetId,
    },
    NotEnoughAssetsInCex {
        cex_id: CexId,
        asset_id: AssetId,
        owned: f64,
        quantity: f64,
    },
}

impl fmt::Display for PortfolioError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PortfolioError::NoAccountInCex { cex_id } => {
                write!(f, "No account in {}", cex_id)
            }
            PortfolioError::AssetNotOwned { cex_id, asset_id } => {
                write!(f, "Portfolio does not own {} in {}", asset_id, cex_id)
            }
            PortfolioError::NotEnoughAssetsInCex {
                cex_id,
                asset_id,
                owned,
                quantity,
            } => {
                write!(
                    f,
                    "Portfolio has {} {} in {} and cannot sell {}",
                    owned, asset_id, cex_id, quantity
                )
            }
        }
    }
}

impl std::error::Error for PortfolioError {}
