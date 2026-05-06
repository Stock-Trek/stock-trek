use crate::{
    error::result::StockTrekResult,
    resolved_context::ResolvedContext,
    scratch::key::{ExchangeName, ScratchKey, TokenName},
    values::value::{
        ExchangeValue, ExchangeValueTrait, FlagValue, FlagValueTrait, NumberValue,
        NumberValueTrait, TokenValue, TokenValueTrait,
    },
};

macro_rules! scratch_pad_value {
    ($trait_name:ident, $raw_type:ident, $getter:ident, $clone_type:ident) => {
        #[typetag::serde]
        impl $trait_name for ScratchKey<$raw_type> {
            fn clone_box(&self) -> $clone_type {
                Box::new(self.clone())
            }
            fn $getter(&self, c: &ResolvedContext) -> StockTrekResult<$raw_type> {
                self.read(c)
            }
        }
    };
}

scratch_pad_value! {ExchangeValueTrait, ExchangeName, exchange, ExchangeValue}
scratch_pad_value! {TokenValueTrait, TokenName, token, TokenValue}
scratch_pad_value! {FlagValueTrait, bool, flag, FlagValue}
scratch_pad_value! {NumberValueTrait, f64, number, NumberValue}
