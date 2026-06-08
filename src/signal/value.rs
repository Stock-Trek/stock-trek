use crate::{
    cex::{asset_id::AssetId, cex_id::CexId},
    error::{
        result::{StockTrekError, StockTrekResult},
        value::ValueError,
    },
};
use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Debug, Display, Clone, Serialize, Deserialize)]
pub enum SignalValue {
    CexId(CexId),
    AssetId(AssetId),
    Flag(bool),
    Number(f64),
}

impl From<AssetId> for SignalValue {
    fn from(value: AssetId) -> Self {
        SignalValue::AssetId(value)
    }
}
impl From<CexId> for SignalValue {
    fn from(value: CexId) -> Self {
        SignalValue::CexId(value)
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

impl TryFrom<SignalValue> for CexId {
    type Error = StockTrekError;
    fn try_from(value: SignalValue) -> StockTrekResult<Self> {
        match value {
            SignalValue::CexId(e) => Ok(e),
            SignalValue::AssetId(_) => err("CexId", "AssetId"),
            SignalValue::Flag(_) => err("CexId", "Flag"),
            SignalValue::Number(_) => err("CexId", "Number"),
        }
    }
}
impl TryFrom<SignalValue> for AssetId {
    type Error = StockTrekError;
    fn try_from(value: SignalValue) -> StockTrekResult<Self> {
        match value {
            SignalValue::CexId(_) => err("AssetId", "CexId"),
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
            SignalValue::CexId(_) => err("Flag", "CexId"),
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
            SignalValue::CexId(_) => err("Number", "CexId"),
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
