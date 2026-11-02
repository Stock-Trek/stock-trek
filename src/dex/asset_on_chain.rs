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

#[allow(missing_docs)]
impl AssetOnChain {
    pub fn new(blockchain: Blockchain, network: Network, identifier: AssetKind) -> Self {
        Self {
            blockchain,
            network,
            identifier,
        }
    }

    pub fn blockchain(&self) -> &Blockchain {
        &self.blockchain
    }

    pub fn network(&self) -> &Network {
        &self.network
    }

    pub fn identifier(&self) -> &AssetKind {
        &self.identifier
    }
}
