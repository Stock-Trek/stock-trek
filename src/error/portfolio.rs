use std::fmt;
use stock_trek_types::cex::{asset_id::AssetId, cex_id::CexId};

/// Errors related to portfolio state.
#[derive(Debug, Clone)]
pub enum PortfolioError {
    /// The requested CEX account does not exist.
    NoCexAccount {
        /// The CEX containing the missing account.
        cex_id: CexId,
    },
    /// The portfolio does not own the requested asset.
    AssetNotOwned {
        /// The CEX containing the account.
        cex_id: CexId,
        /// The asset that is not owned.
        asset_id: AssetId,
    },
    /// The CEX account does not hold enough of an asset.
    NotEnoughAssetsInCex {
        /// The CEX containing the account.
        cex_id: CexId,
        /// The asset that is in short supply.
        asset_id: AssetId,
        /// The amount currently owned.
        owned: f64,
        /// The amount required.
        quantity: f64,
    },
}

impl fmt::Display for PortfolioError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PortfolioError::NoCexAccount { cex_id } => {
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
