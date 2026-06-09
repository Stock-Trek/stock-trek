use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Network {
    network_id: NetworkId,
    chain_id: Option<ChainId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NetworkId {
    Mainnet,
    Testnet,
    Other(String),
}

impl std::fmt::Display for NetworkId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NetworkId::Mainnet => write!(f, "mainnet"),
            NetworkId::Testnet => write!(f, "testnet"),
            NetworkId::Other(name) => write!(f, "{}", name),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ChainId(u64);

impl ChainId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }
}

impl Network {
    pub fn new(network_id: NetworkId, chain_id: Option<ChainId>) -> Self {
        Self {
            network_id,
            chain_id,
        }
    }
}
