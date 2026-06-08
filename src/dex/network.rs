use crate::dex::{asset_kind::AssetKind, asset_on_chain::AssetOnChain, blockchain::Blockchain};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Network {
    blockchain: Blockchain,
    network_id: NetworkId,
    chain_id: Option<ChainId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NetworkId {
    Mainnet,
    Testnet,
    Other(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ChainId(u64);

impl ChainId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }
}

impl Network {
    pub fn new(blockchain: Blockchain, network_id: NetworkId, chain_id: Option<ChainId>) -> Self {
        Self {
            blockchain,
            network_id,
            chain_id,
        }
    }
    pub fn native_asset(&self) -> AssetOnChain {
        AssetOnChain::new(self.clone(), AssetKind::Native)
    }
    pub fn asset(&self, contract_address: &str) -> AssetOnChain {
        assert!(self.blockchain.is_valid_address(contract_address));
        AssetOnChain::new(
            self.clone(),
            AssetKind::Contract(contract_address.to_string()),
        )
    }
}
