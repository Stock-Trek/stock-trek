use crate::{
    error::result::StockTrekResult,
    resolved_context::ResolvedContext,
    scratch::key::{ExchangeName, TokenName},
    values::value::{
        ExchangeValue, ExchangeValueTrait, FlagValue, FlagValueTrait, NumberValue,
        NumberValueTrait, TokenValue, TokenValueTrait,
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

literal_value! {LiteralExchangeValue, ExchangeValueTrait, ExchangeName, exchange, ExchangeValue}
literal_value! {LiteralTokenValue, TokenValueTrait, TokenName, token, TokenValue}
literal_value! {LiteralFlagValue, FlagValueTrait, bool, flag, FlagValue}
literal_value! {LiteralNumberValue, NumberValueTrait, f64, number, NumberValue}
