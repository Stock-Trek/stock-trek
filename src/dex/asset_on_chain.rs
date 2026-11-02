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

#[allow(missing_docs)]
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
mod tests {
    use super::*;
    use crate::dex::{
        blockchain::{BlockchainId, FamilyId},
        network::{ChainId, NetworkId},
    };

    fn blockchain() -> Blockchain {
        Blockchain::new(
            BlockchainId::new("ethereum").unwrap(),
            FamilyId::new("evm").unwrap(),
            18,
            "^0x[a-fA-F0-9]{40}$",
        )
        .unwrap()
    }

    fn network() -> Network {
        Network::new(NetworkId::Mainnet, Some(ChainId::new(1)))
    }

    #[test]
    fn creates_a_native_asset() {
        let asset = AssetOnChain::new(blockchain(), network(), AssetKind::Native).unwrap();

        assert_eq!(asset.identifier(), &AssetKind::Native);
    }

    #[test]
    fn creates_a_contract_asset_with_a_valid_address() {
        let address = "0x0000000000000000000000000000000000000000";
        let asset = AssetOnChain::new(
            blockchain(),
            network(),
            AssetKind::Contract(address.to_string()),
        )
        .unwrap();

        assert_eq!(
            asset.identifier(),
            &AssetKind::Contract(address.to_string())
        );
    }

    #[test]
    fn rejects_a_contract_asset_with_an_invalid_address() {
        let result = AssetOnChain::new(
            blockchain(),
            network(),
            AssetKind::Contract("not-an-address".to_string()),
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_a_contract_asset_with_an_invalid_address_when_deserializing() {
        let json = serde_json::json!({
            "blockchain": {
                "blockchain_id": "ethereum",
                "family_id": "evm",
                "native_decimals": 18,
                "address_regex": "^0x[a-fA-F0-9]{40}$"
            },
            "network": {
                "network_id": "mainnet",
                "chain_id": 1
            },
            "identifier": {
                "contract": "not-an-address"
            }
        });

        assert!(serde_json::from_value::<AssetOnChain>(json).is_err());
    }

    #[test]
    fn round_trips_a_contract_asset_with_a_valid_address() {
        let address = "0x0000000000000000000000000000000000000000";
        let asset = AssetOnChain::new(
            blockchain(),
            network(),
            AssetKind::Contract(address.to_string()),
        )
        .unwrap();

        let json = serde_json::to_string(&asset).unwrap();
        let deserialized: AssetOnChain = serde_json::from_str(&json).unwrap();

        assert_eq!(
            deserialized.identifier(),
            &AssetKind::Contract(address.to_string())
        );
    }
}
