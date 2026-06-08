use crate::{
    cex::{asset_id::AssetId, exchange_id::ExchangeId},
    error::result::StockTrekResult,
    resolved_context::ResolvedContext,
    values::value::{
        AssetIdValue, AssetIdValueTrait, ExchangeIdValue, ExchangeIdValueTrait, FlagValue,
        FlagValueTrait, NumberValue, NumberValueTrait,
    },
};
use serde::{Deserialize, Serialize};

macro_rules! literal_value {
    ($name:ident, $trait_name:ident, $raw_type:ident, $getter:ident, $clone_type:ident) => {
        #[derive(Clone, Serialize, Deserialize)]
        pub struct $name {
            literal: $raw_type,
        }
        impl $name {
            pub fn new(literal: $raw_type) -> $clone_type {
                Box::new(Self { literal })
            }
        }
        #[typetag::serde]
        impl $trait_name for $name {
            fn clone_box(&self) -> $clone_type {
                Box::new(self.clone())
            }
            fn $getter(&self, _: &ResolvedContext) -> StockTrekResult<$raw_type> {
                Ok(self.literal.clone())
            }
        }
    };
}

literal_value! {LiteralExchangeIdValue, ExchangeIdValueTrait, ExchangeId, exchange_id, ExchangeIdValue}
literal_value! {LiteralAssetIdValue, AssetIdValueTrait, AssetId, asset_id, AssetIdValue}
literal_value! {LiteralFlagValue, FlagValueTrait, bool, flag, FlagValue}
literal_value! {LiteralNumberValue, NumberValueTrait, f64, number, NumberValue}
