//! An asset together with the blockchain and network it is deployed on.

use crate::dex::{asset_kind::AssetKind, blockchain::Blockchain, network::Network};
use crate::error::{
    dex::DexError,
    result::{StockTrekError, StockTrekResult},
};
use serde::{Deserialize, Deserializer, Serialize};

/// An asset deployed on a specific blockchain network.
#[derive(Debug, Clone, Serialize)]
pub struct AssetOnChain {
    blockchain: Blockchain,
    network: Network,
    identifier: AssetKind,
}

impl<'de> Deserialize<'de> for AssetOnChain {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct UncheckedAssetOnChain {
            blockchain: Blockchain,
            network: Network,
            identifier: AssetKind,
        }

        let unchecked = UncheckedAssetOnChain::deserialize(deserializer)?;
        Self::new(
            unchecked.blockchain,
            unchecked.network,
            unchecked.identifier,
        )
        .map_err(serde::de::Error::custom)
    }
}

#[allow(missing_docs)]
impl AssetOnChain {
    /// Creates an asset after validating its network and identifier.
    ///
    /// # Errors
    ///
    /// Returns an error if the network does not belong to the blockchain or a
    /// contract identifier does not match the blockchain's address format.
    pub fn new(
        blockchain: Blockchain,
        network: Network,
        identifier: AssetKind,
    ) -> StockTrekResult<Self> {
        if !blockchain.supports_network(&network) {
            return Err(StockTrekError::Dex(DexError::NetworkMismatch {
                blockchain_id: blockchain.blockchain_id().to_string(),
                network_id: network.network_id().to_string(),
            }));
        }

        if let AssetKind::Contract(address) = &identifier
            && !blockchain.is_valid_address(address)
        {
            return Err(StockTrekError::Dex(DexError::InvalidAddress {
                blockchain_id: blockchain.blockchain_id().to_string(),
                address: address.clone(),
            }));
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dex::network::{ChainId, NetworkId};

    fn ethereum_asset() -> AssetOnChain {
        crate::dex::networks::ethereum::asset_on_chain(
            crate::dex::networks::ethereum::NetworkName::Mainnet,
            AssetKind::Native,
        )
        .unwrap()
    }

    #[test]
    fn contract_identifiers_must_match_the_blockchain_address_format() {
        let asset = ethereum_asset();
        let valid_address = format!("0x{}", "0".repeat(40));

        assert!(
            AssetOnChain::new(
                asset.blockchain().clone(),
                asset.network().clone(),
                AssetKind::Contract(valid_address),
            )
            .is_ok()
        );
        assert!(
            AssetOnChain::new(
                asset.blockchain().clone(),
                asset.network().clone(),
                AssetKind::Contract("not-an-address".into()),
            )
            .is_err()
        );
    }

    #[test]
    fn networks_must_belong_to_the_blockchain() {
        let ethereum = ethereum_asset();
        let bitcoin = crate::dex::networks::bitcoin::asset_on_chain(
            crate::dex::networks::bitcoin::NetworkName::Mainnet,
            AssetKind::Native,
        )
        .unwrap();

        assert!(
            AssetOnChain::new(
                ethereum.blockchain().clone(),
                bitcoin.network().clone(),
                AssetKind::Native,
            )
            .is_err()
        );
    }

    #[test]
    fn registered_blockchains_reject_unknown_networks() {
        let asset = ethereum_asset();
        let unknown_network = Network::new(NetworkId::Other("unknown".into()), None);

        assert!(
            AssetOnChain::new(
                asset.blockchain().clone(),
                unknown_network,
                AssetKind::Native,
            )
            .is_err()
        );
    }

    #[test]
    fn unregistered_blockchains_can_be_paired_with_networks() {
        let blockchain = Blockchain::new(
            crate::dex::blockchain::BlockchainId::new("custom"),
            crate::dex::blockchain::FamilyId::new("custom"),
            18,
            "^[a-z]+$",
        )
        .unwrap();
        let network = Network::new(NetworkId::Mainnet, None);

        assert!(AssetOnChain::new(blockchain, network, AssetKind::Native).is_ok());
    }

    #[test]
    fn deserialization_revalidates_the_asset() {
        let asset = ethereum_asset();
        let mut value = serde_json::to_value(&asset).unwrap();
        value["identifier"] = serde_json::json!({ "contract": "not-an-address" });

        assert!(serde_json::from_value::<AssetOnChain>(value).is_err());
    }

    #[test]
    fn manually_constructed_known_blockchains_validate_chain_ids() {
        let blockchain = Blockchain::new(
            crate::dex::blockchain::BlockchainId::new("ethereum"),
            crate::dex::blockchain::FamilyId::new("evm"),
            18,
            "^0x[a-fA-F0-9]{40}$",
        )
        .unwrap();

        assert!(
            AssetOnChain::new(
                blockchain.clone(),
                Network::new(NetworkId::Mainnet, Some(ChainId::new(1))),
                AssetKind::Native,
            )
            .is_ok()
        );
        assert!(
            AssetOnChain::new(
                blockchain,
                Network::new(NetworkId::Mainnet, Some(ChainId::new(56))),
                AssetKind::Native,
            )
            .is_err()
        );
    }

    #[test]
    fn network_association_is_checked_even_without_registered_networks() {
        let blockchain = Blockchain::new(
            crate::dex::blockchain::BlockchainId::new("ethereum"),
            crate::dex::blockchain::FamilyId::new("evm"),
            18,
            "^0x[a-fA-F0-9]{40}$",
        )
        .unwrap();
        let network = Network::for_blockchain(
            crate::dex::blockchain::BlockchainId::new("bitcoin"),
            NetworkId::Mainnet,
            Some(ChainId::new(1)),
        );

        assert!(AssetOnChain::new(blockchain, network, AssetKind::Native).is_err());
    }
}
