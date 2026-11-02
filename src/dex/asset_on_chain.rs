//! An asset together with the blockchain and network it is deployed on.

use crate::dex::{asset_kind::AssetKind, blockchain::Blockchain, network::Network, networks};
use crate::error::{
    dex::DexError,
    result::{StockTrekError, StockTrekResult},
};
use serde::{Deserialize, Serialize};

/// An asset deployed on a specific blockchain network.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "UncheckedAssetOnChain")]
pub struct AssetOnChain {
    blockchain: Blockchain,
    network: Network,
    identifier: AssetKind,
}

#[derive(Debug, Deserialize)]
struct UncheckedAssetOnChain {
    blockchain: Blockchain,
    network: Network,
    identifier: AssetKind,
}

impl TryFrom<UncheckedAssetOnChain> for AssetOnChain {
    type Error = StockTrekError;

    fn try_from(value: UncheckedAssetOnChain) -> Result<Self, Self::Error> {
        Self::new(value.blockchain, value.network, value.identifier)
    }
}

#[allow(missing_docs)]
impl AssetOnChain {
    /// Creates an asset and validates that it can exist on the supplied chain.
    ///
    /// # Errors
    ///
    /// Returns an error if the network does not belong to the blockchain or if a
    /// contract identifier does not match the blockchain's address format.
    pub fn new(
        blockchain: Blockchain,
        network: Network,
        identifier: AssetKind,
    ) -> StockTrekResult<Self> {
        let asset = Self {
            blockchain,
            network,
            identifier,
        };
        asset.validate()?;
        Ok(asset)
    }

    /// Validates the asset against its blockchain and network.
    ///
    /// # Errors
    ///
    /// Returns an error if the network does not belong to the blockchain or if a
    /// contract identifier does not match the blockchain's address format.
    pub fn validate(&self) -> StockTrekResult<()> {
        if !self.network_belongs_to_blockchain() {
            return Err(StockTrekError::Dex(DexError::InvalidNetwork {
                blockchain_id: self.blockchain.blockchain_id().to_string(),
                network_id: self.network.network_id().to_string(),
            }));
        }

        match &self.identifier {
            AssetKind::Native => Ok(()),
            AssetKind::Contract(address) => self.blockchain.validate_address(address),
        }
    }

    fn network_belongs_to_blockchain(&self) -> bool {
        if let Some(blockchain_id) = self.network.blockchain_id() {
            return blockchain_id == self.blockchain.blockchain_id();
        }

        networks::network_belongs_to_blockchain(
            self.blockchain.blockchain_id().as_str(),
            self.network.network_id(),
            self.network.chain_id(),
        )
        .unwrap_or(true)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dex::{
        blockchain::{BlockchainId, FamilyId},
        network::{ChainId, NetworkId},
    };

    fn ethereum() -> Blockchain {
        Blockchain::new(
            BlockchainId::new("ethereum"),
            FamilyId::new("evm"),
            18,
            "^0x[a-fA-F0-9]{40}$",
        )
        .unwrap()
    }

    fn ethereum_mainnet() -> Network {
        Network::new(NetworkId::Mainnet, Some(ChainId::new(1)))
    }

    #[test]
    fn accepts_native_assets_on_a_matching_network() {
        let asset = AssetOnChain::new(ethereum(), ethereum_mainnet(), AssetKind::Native).unwrap();

        assert_eq!(asset.blockchain().blockchain_id().as_str(), "ethereum");
        assert_eq!(asset.network().network_id().as_str(), "mainnet");
    }

    #[test]
    fn accepts_contract_identifiers_that_match_the_address_format() {
        let address = "0x0000000000000000000000000000000000000000";
        let asset = AssetOnChain::new(
            ethereum(),
            ethereum_mainnet(),
            AssetKind::Contract(address.to_string()),
        )
        .unwrap();

        assert_eq!(
            asset.identifier(),
            &AssetKind::Contract(address.to_string())
        );
    }

    #[test]
    fn rejects_contract_identifiers_that_do_not_match_the_address_format() {
        let error = AssetOnChain::new(
            ethereum(),
            ethereum_mainnet(),
            AssetKind::Contract("not-an-address".to_string()),
        )
        .unwrap_err();

        assert!(matches!(
            error,
            StockTrekError::Dex(DexError::InvalidAddress { .. })
        ));
    }

    #[test]
    fn rejects_networks_that_do_not_belong_to_the_blockchain() {
        let network = Network::new(NetworkId::Other("solana".to_string()), None);
        let error = AssetOnChain::new(ethereum(), network, AssetKind::Native).unwrap_err();

        assert!(matches!(
            error,
            StockTrekError::Dex(DexError::InvalidNetwork { .. })
        ));
    }

    #[test]
    fn rejects_known_evm_networks_with_the_wrong_chain_id() {
        let network = Network::new(NetworkId::Mainnet, Some(ChainId::new(137)));
        let error = AssetOnChain::new(ethereum(), network, AssetKind::Native).unwrap_err();

        assert!(matches!(
            error,
            StockTrekError::Dex(DexError::InvalidNetwork { .. })
        ));
    }

    #[test]
    fn rejects_explicit_network_associations_for_another_blockchain() {
        let network = Network::for_blockchain("bitcoin", NetworkId::Mainnet, None);
        let error = AssetOnChain::new(ethereum(), network, AssetKind::Native).unwrap_err();

        assert!(matches!(
            error,
            StockTrekError::Dex(DexError::InvalidNetwork { .. })
        ));
    }

    #[test]
    fn validates_assets_when_they_are_deserialized() {
        let asset = AssetOnChain::new(
            ethereum(),
            ethereum_mainnet(),
            AssetKind::Contract("0x0000000000000000000000000000000000000000".to_string()),
        )
        .unwrap();
        let mut json = serde_json::to_value(asset).unwrap();
        json["identifier"] = serde_json::json!({ "contract": "not-an-address" });

        assert!(serde_json::from_value::<AssetOnChain>(json).is_err());
    }
}
