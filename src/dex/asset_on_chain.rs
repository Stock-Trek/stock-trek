use crate::dex::{asset_kind::AssetKind, blockchain::Blockchain, network::Network};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetOnChain {
    blockchain: Blockchain,
    network: Network,
    identifier: AssetKind,
}

impl AssetOnChain {
    pub fn new(blockchain: Blockchain, network: Network, identifier: AssetKind) -> Self {
        Self {
            blockchain,
            network,
            identifier,
        }
    }
}
