use crate::dex::{asset_kind::AssetKind, network::Network};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetOnChain {
    network: Network,
    identifier: AssetKind,
}

impl AssetOnChain {
    pub fn new(network: Network, identifier: AssetKind) -> Self {
        Self {
            network,
            identifier,
        }
    }
}
