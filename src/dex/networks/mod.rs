//! Well-known blockchain and network definitions.
//!
//! EVM and non-EVM definitions are kept in separate submodules and re-exported
//! here so callers can keep using paths such as `networks::ethereum` and
//! `networks::bitcoin`.

use crate::dex::{
    asset_on_chain::AssetOnChain,
    blockchain::{Blockchain, BlockchainId, FamilyId},
    network::{ChainId, Network, NetworkId},
};
use crate::error::result::{StockTrekError, StockTrekResult};

pub const FAMILY_ID_BITCOIN: &str = "bitcoin";
pub const FAMILY_ID_BITCOIN_CASH: &str = "bitcoin-cash";
pub const FAMILY_ID_COSMOS: &str = "cosmos";
pub const FAMILY_ID_DOGECOIN: &str = "dogecoin";
pub const FAMILY_ID_EVM: &str = "evm";
pub const FAMILY_ID_LITECOIN: &str = "litecoin";
pub const FAMILY_ID_NEAR: &str = "near";
pub const FAMILY_ID_OSMOSIS: &str = "osmosis";
pub const FAMILY_ID_SOLANA: &str = "solana";
pub const FAMILY_ID_TRON: &str = "tron";

#[macro_use]
mod macros;

mod evm;
mod non_evm;

pub use evm::*;
pub use non_evm::*;

impl Blockchain {
    fn from_ids(
        blockchain_id: &str,
        family_id: &str,
        native_decimals: u8,
        address_format: &str,
    ) -> StockTrekResult<Self> {
        Self::new(
            BlockchainId::new(blockchain_id)?,
            FamilyId::new(family_id)?,
            native_decimals,
            address_format,
        )
    }

    fn evm(blockchain_name: &str) -> StockTrekResult<Self> {
        Self::from_ids(blockchain_name, FAMILY_ID_EVM, 18, "^0x[a-fA-F0-9]{40}$")
    }
}

impl AssetOnChain {
    fn evm(
        blockchain_name: &str,
        network: impl TryInto<NetworkId, Error = StockTrekError> + Into<u64> + Copy,
        asset_kind: crate::dex::asset_kind::AssetKind,
    ) -> StockTrekResult<Self> {
        let blockchain = Blockchain::evm(blockchain_name)?;
        let network_id = network.try_into()?;
        let network = Network::new(network_id, Some(ChainId::new(network.into())));
        Self::new(blockchain, network, asset_kind)
    }
}

#[cfg(test)]
mod tests;
