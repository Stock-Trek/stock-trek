//! Constructors shared by the well-known network definitions.

use super::family_ids::FAMILY_ID_EVM;
use crate::{
    dex::{
        asset_kind::AssetKind,
        asset_on_chain::AssetOnChain,
        blockchain::{Blockchain, BlockchainId, FamilyId},
        network::{ChainId, Network, NetworkId},
    },
    error::result::{StockTrekError, StockTrekResult},
};
use regex::Regex;
use std::sync::LazyLock;

const EVM_ADDRESS_FORMAT: &str = "^0x[a-fA-F0-9]{40}$";

static EVM_ADDRESS_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(EVM_ADDRESS_FORMAT).expect("EVM address format must be a valid regex")
});

impl Blockchain {
    pub(super) fn from_ids(
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

    pub(super) fn evm(blockchain_name: &str) -> StockTrekResult<Self> {
        Ok(Self::from_compiled_regex(
            BlockchainId::new(blockchain_name)?,
            FamilyId::new(FAMILY_ID_EVM)?,
            18,
            EVM_ADDRESS_REGEX.clone(),
        ))
    }
}

impl AssetOnChain {
    pub(super) fn evm(
        blockchain_name: &str,
        network: impl TryInto<NetworkId, Error = StockTrekError> + Copy,
        chain_id: ChainId,
        asset_kind: AssetKind,
    ) -> StockTrekResult<Self> {
        let blockchain = Blockchain::evm(blockchain_name)?;
        let network_id = network.try_into()?;
        let network = Network::new(network_id, Some(chain_id))?;
        Self::new(blockchain, network, asset_kind)
    }
}
