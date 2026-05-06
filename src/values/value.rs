use crate::{
    error::result::StockTrekResult,
    resolved_context::ResolvedContext,
    scratch::key::{ExchangeName, TokenName},
};
use std::{
    fmt::{Debug, Formatter},
    hash::{Hash, Hasher},
};

macro_rules! value_type {
    ($name:ident, $trait_name:ident, $getter:ident, $value:ident) => {
        pub type $name = Box<dyn $trait_name>;
        impl Debug for $name {
            fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
                f.debug_tuple(stringify!($name)).finish()
            }
        }
        impl Clone for $name {
            fn clone(&self) -> $name {
                (**self).clone_box()
            }
        }
        impl Hash for $name {
            fn hash<H>(&self, state: &mut H)
            where
                H: Hasher,
            {
                stringify!($name).hash(state)
            }
        }
        #[typetag::serde]
        pub trait $trait_name: Send + Sync {
            fn clone_box(&self) -> $name;
            fn $getter(&self, c: &ResolvedContext) -> StockTrekResult<$value>;
        }
    };
}

value_type! {ExchangeValue, ExchangeValueTrait, exchange, ExchangeName}
value_type! {TokenValue, TokenValueTrait, token, TokenName}
value_type! {FlagValue, FlagValueTrait, flag, bool}
value_type! {NumberValue, NumberValueTrait, number, f64}
