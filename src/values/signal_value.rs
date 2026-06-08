use crate::{
    cex::{asset_id::AssetId, exchange_id::ExchangeId},
    error::result::StockTrekResult,
    resolved_context::ResolvedContext,
    signal::key::SignalKey,
    values::value::{
        AssetIdValue, AssetIdValueTrait, ExchangeIdValue, ExchangeIdValueTrait, FlagValue,
        FlagValueTrait, NumberValue, NumberValueTrait,
    },
};

macro_rules! signal_value {
    ($trait_name:ident, $raw_type:ident, $getter:ident, $clone_type:ident) => {
        #[typetag::serde]
        impl $trait_name for SignalKey<$raw_type> {
            fn clone_box(&self) -> $clone_type {
                Box::new(self.clone())
            }
            fn $getter(&self, c: &ResolvedContext) -> StockTrekResult<$raw_type> {
                self.read(c)
            }
        }
    };
}

signal_value! {ExchangeIdValueTrait, ExchangeId, exchange_id, ExchangeIdValue}
signal_value! {AssetIdValueTrait, AssetId, asset_id, AssetIdValue}
signal_value! {FlagValueTrait, bool, flag, FlagValue}
signal_value! {NumberValueTrait, f64, number, NumberValue}
