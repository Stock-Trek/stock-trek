use crate::error::{
    result::{StockTrekError, StockTrekResult},
    value::ValueError,
};
use serde::{Deserialize, Serialize};
use stock_trek_types::cex::{account_id::AccountId, asset_id::AssetId, cex_id::CexId};
use strum::Display;

/// A dynamically typed signal value.
///
/// Signals may hold CEX, account, and asset identifiers, boolean flags, or
/// numbers.
#[derive(Debug, Display, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
#[non_exhaustive]
pub enum SignalValue {
    /// A CEX identifier.
    CexId(CexId),
    /// A CEX account identifier.
    AccountId(AccountId),
    /// An asset identifier.
    AssetId(AssetId),
    /// A boolean flag.
    Flag(bool),
    /// A numeric value.
    Number(f64),
}

macro_rules! signal_value_conversions {
    ($($variant:ident($type:ty)),+ $(,)?) => {
        impl SignalValue {
            fn variant_name(&self) -> &'static str {
                match self {
                    $(SignalValue::$variant(_) => stringify!($variant),)+
                }
            }
        }

        $(
            impl From<$type> for SignalValue {
                fn from(value: $type) -> Self {
                    SignalValue::$variant(value)
                }
            }

            impl TryFrom<SignalValue> for $type {
                type Error = StockTrekError;
                fn try_from(value: SignalValue) -> StockTrekResult<Self> {
                    match value {
                        SignalValue::$variant(value) => Ok(value),
                        other => err(stringify!($variant), other.variant_name()),
                    }
                }
            }
        )+
    };
}

signal_value_conversions! {
    CexId(CexId),
    AccountId(AccountId),
    AssetId(AssetId),
    Flag(bool),
    Number(f64),
}

fn err<T>(expected: impl AsRef<str>, found: impl AsRef<str>) -> StockTrekResult<T> {
    Err(StockTrekError::Value(ValueError::IncorrectType {
        expected: expected.as_ref().to_string(),
        found: found.as_ref().to_string(),
    }))
}
