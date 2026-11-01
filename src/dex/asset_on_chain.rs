//! An asset together with the blockchain and network it is deployed on.

use crate::dex::{asset_kind::AssetKind, blockchain::Blockchain, network::Network};
use serde::{Deserialize, Serialize};

/// An asset deployed on a specific blockchain network.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetOnChain {
    blockchain: Blockchain,
    network: Network,
    identifier: AssetKind,
}

impl AssetOnChain {
    /// Creates an asset on the given blockchain and network.
    pub fn new(blockchain: Blockchain, network: Network, identifier: AssetKind) -> Self {
        Self {
            blockchain,
            network,
            identifier,
        }
    }
}
