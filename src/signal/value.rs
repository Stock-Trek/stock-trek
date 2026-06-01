use crate::{
    asset_id::AssetId,
    error::{
        result::{StockTrekError, StockTrekResult},
        value::ValueError,
    },
    exchange_id::ExchangeId,
};
use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Debug, Display, Clone, Serialize, Deserialize)]
pub enum SignalValue {
    ExchangeId(ExchangeId),
    AssetId(AssetId),
    Flag(bool),
    Number(f64),
}

impl From<AssetId> for SignalValue {
    fn from(value: AssetId) -> Self {
        SignalValue::AssetId(value)
    }
}
impl From<ExchangeId> for SignalValue {
    fn from(value: ExchangeId) -> Self {
        SignalValue::ExchangeId(value)
    }
}
impl From<bool> for SignalValue {
    fn from(value: bool) -> Self {
        SignalValue::Flag(value)
    }
}
impl From<f64> for SignalValue {
    fn from(value: f64) -> Self {
        SignalValue::Number(value)
    }
}

impl TryFrom<SignalValue> for ExchangeId {
    type Error = StockTrekError;
    fn try_from(value: SignalValue) -> StockTrekResult<Self> {
        match value {
            SignalValue::ExchangeId(e) => Ok(e),
            SignalValue::AssetId(_) => err("ExchangeId", "AssetId"),
            SignalValue::Flag(_) => err("ExchangeId", "Flag"),
            SignalValue::Number(_) => err("ExchangeId", "Number"),
        }
    }
}
impl TryFrom<SignalValue> for AssetId {
    type Error = StockTrekError;
    fn try_from(value: SignalValue) -> StockTrekResult<Self> {
        match value {
            SignalValue::ExchangeId(_) => err("AssetId", "ExchangeId"),
            SignalValue::AssetId(a) => Ok(a),
            SignalValue::Flag(_) => err("AssetId", "Flag"),
            SignalValue::Number(_) => err("AssetId", "Number"),
        }
    }
}
impl TryFrom<SignalValue> for bool {
    type Error = StockTrekError;
    fn try_from(value: SignalValue) -> StockTrekResult<Self> {
        match value {
            SignalValue::ExchangeId(_) => err("Flag", "ExchangeId"),
            SignalValue::AssetId(_) => err("Flag", "AssetId"),
            SignalValue::Flag(f) => Ok(f),
            SignalValue::Number(_) => err("Flag", "Number"),
        }
    }
}
impl TryFrom<SignalValue> for f64 {
    type Error = StockTrekError;
    fn try_from(value: SignalValue) -> StockTrekResult<Self> {
        match value {
            SignalValue::ExchangeId(_) => err("Number", "ExchangeId"),
            SignalValue::AssetId(_) => err("Number", "AssetId"),
            SignalValue::Flag(_) => err("Number", "Flag"),
            SignalValue::Number(n) => Ok(n),
        }
    }
}

fn err<T>(expected: impl AsRef<str>, found: impl AsRef<str>) -> StockTrekResult<T> {
    Err(StockTrekError::Value(ValueError::IncorrectType {
        expected: expected.as_ref().to_string(),
        found: found.as_ref().to_string(),
    }))
}
