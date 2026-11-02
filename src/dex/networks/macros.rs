macro_rules! define_network_name {
    ($($variant:ident => $network_id:expr),+ $(,)?) => {
        #[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
        #[serde(rename_all = "snake_case")]
        #[strum(serialize_all = "snake_case")]
        #[non_exhaustive]
        pub enum NetworkName {
            $($variant,)+
        }

        impl TryFrom<NetworkName> for NetworkId {
            type Error = crate::error::result::StockTrekError;

            fn try_from(value: NetworkName) -> Result<Self, Self::Error> {
                match value {
                    $(NetworkName::$variant => Ok($network_id),)+
                }
            }
        }
    };
}

macro_rules! define_evm_network {
    (
        $module:ident,
        $blockchain:literal,
        {
            $($variant:ident => ($chain_id:literal, $network_id:expr)),+ $(,)?
        }
    ) => {
        pub mod $module {
            use crate::dex::network::NetworkId;
            use crate::error::result::StockTrekResult;
            use serde::{Deserialize, Serialize};
            use strum::Display;

            define_network_name! {
                $($variant => $network_id,)+
            }

            impl From<NetworkName> for u64 {
                fn from(value: NetworkName) -> Self {
                    match value {
                        $(NetworkName::$variant => $chain_id,)+
                    }
                }
            }

            pub fn asset_on_chain(
                network: NetworkName,
                asset_kind: crate::dex::asset_kind::AssetKind,
            ) -> StockTrekResult<crate::dex::asset_on_chain::AssetOnChain> {
                super::AssetOnChain::evm($blockchain, network, asset_kind)
            }
        }
    };
}

macro_rules! define_network {
    (
        $module:ident,
        $blockchain:literal,
        $family_id:ident,
        $native_decimals:literal,
        $address_format:literal,
        {
            $($variant:ident => $network_id:expr),+ $(,)?
        }
    ) => {
        pub mod $module {
            use crate::dex::network::{Network, NetworkId};
            use crate::error::result::StockTrekResult;
            use serde::{Deserialize, Serialize};
            use strum::Display;

            define_network_name! {
                $($variant => $network_id,)+
            }

            pub fn asset_on_chain(
                network: NetworkName,
                asset_kind: crate::dex::asset_kind::AssetKind,
            ) -> StockTrekResult<crate::dex::asset_on_chain::AssetOnChain> {
                let blockchain = super::Blockchain::from_ids(
                    $blockchain,
                    super::$family_id,
                    $native_decimals,
                    $address_format,
                )?;
                let network = Network::new(NetworkId::try_from(network)?, None);
                crate::dex::asset_on_chain::AssetOnChain::new(blockchain, network, asset_kind)
            }
        }
    };
}
