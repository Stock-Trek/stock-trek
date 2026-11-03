//! An asset together with the blockchain and network it is deployed on.

use crate::dex::{asset_kind::AssetKind, blockchain::Blockchain, network::Network};
use crate::error::{dex::DexError, result::StockTrekResult};
use serde::{Deserialize, Serialize};

/// An asset deployed on a specific blockchain network.
#[derive(Debug, Clone, Serialize)]
pub struct AssetOnChain {
    blockchain: Blockchain,
    network: Network,
    identifier: AssetKind,
}

#[derive(Deserialize)]
struct AssetOnChainRepr {
    blockchain: Blockchain,
    network: Network,
    identifier: AssetKind,
}

impl AssetOnChain {
    /// Creates an asset on a blockchain network.
    ///
    /// # Errors
    ///
    /// Returns an error if the identifier is a contract address that does not
    /// match the blockchain's address format.
    pub fn new(
        blockchain: Blockchain,
        network: Network,
        identifier: AssetKind,
    ) -> StockTrekResult<Self> {
        if let AssetKind::Contract(address) = &identifier
            && !blockchain.is_valid_address(address)
        {
            return Err(DexError::InvalidContractAddress {
                blockchain_id: blockchain.blockchain_id().to_string(),
                address: address.clone(),
            }
            .into());
        }

        Ok(Self {
            blockchain,
            network,
            identifier,
        })
    }

    /// The blockchain the asset is deployed on.
    pub fn blockchain(&self) -> &Blockchain {
        &self.blockchain
    }

    /// The network of the blockchain the asset is deployed on.
    pub fn network(&self) -> &Network {
        &self.network
    }

    /// The asset kind: the blockchain's native asset or a contract.
    pub fn identifier(&self) -> &AssetKind {
        &self.identifier
    }
}

impl<'de> Deserialize<'de> for AssetOnChain {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let repr = AssetOnChainRepr::deserialize(deserializer)?;
        Self::new(repr.blockchain, repr.network, repr.identifier).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests;
