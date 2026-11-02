//! Asset identifiers within a blockchain.

use serde::{Deserialize, Serialize};

/// The kind of asset on a blockchain.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum AssetKind {
    /// The blockchain's native asset.
    Native,
    /// A contract-based asset, identified by its contract address.
    Contract(String),
}

impl std::fmt::Display for AssetKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AssetKind::Native => write!(f, "native"),
            AssetKind::Contract(name) => write!(f, "contract({})", name),
        }
    }
}
