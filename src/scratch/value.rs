use crate::error::{
    result::{StockTrekError, StockTrekResult},
    value::ValueError,
};
use digdigdig3::{Asset, ExchangeId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScratchValue {
    Asset(Asset),
    Exchange(ExchangeId),
    Flag(bool),
    Number(f64),
}

impl From<String> for ScratchValue {
    fn from(value: String) -> Self {
        ScratchValue::Asset(value)
    }
}
impl From<ExchangeId> for ScratchValue {
    fn from(value: ExchangeId) -> Self {
        ScratchValue::Exchange(value)
    }
}
impl From<bool> for ScratchValue {
    fn from(value: bool) -> Self {
        ScratchValue::Flag(value)
    }
}
impl From<f64> for ScratchValue {
    fn from(value: f64) -> Self {
        ScratchValue::Number(value)
    }
}

impl TryFrom<ScratchValue> for ExchangeId {
    type Error = StockTrekError;
    fn try_from(value: ScratchValue) -> StockTrekResult<Self> {
        match value {
            ScratchValue::Asset(_) => err("Exchange", "Asset"),
            ScratchValue::Exchange(e) => Ok(e),
            ScratchValue::Flag(_) => err("Exchange", "Flag"),
            ScratchValue::Number(_) => err("Exchange", "Number"),
        }
    }
}
impl TryFrom<ScratchValue> for bool {
    type Error = StockTrekError;
    fn try_from(value: ScratchValue) -> StockTrekResult<Self> {
        match value {
            ScratchValue::Asset(_) => err("Flag", "Asset"),
            ScratchValue::Exchange(_) => err("Flag", "Exchange"),
            ScratchValue::Flag(f) => Ok(f),
            ScratchValue::Number(_) => err("Flag", "Number"),
        }
    }
}
impl TryFrom<ScratchValue> for f64 {
    type Error = StockTrekError;
    fn try_from(value: ScratchValue) -> StockTrekResult<Self> {
        match value {
            ScratchValue::Asset(_) => err("Number", "Asset"),
            ScratchValue::Exchange(_) => err("Number", "Exchange"),
            ScratchValue::Flag(_) => err("Number", "Flag"),
            ScratchValue::Number(n) => Ok(n),
        }
    }
}
impl TryFrom<ScratchValue> for Asset {
    type Error = StockTrekError;
    fn try_from(value: ScratchValue) -> StockTrekResult<Self> {
        match value {
            ScratchValue::Asset(a) => Ok(a),
            ScratchValue::Exchange(_) => err("Asset", "Exchange"),
            ScratchValue::Flag(_) => err("Asset", "Flag"),
            ScratchValue::Number(_) => err("Asset", "Number"),
        }
    }
}

fn err<T>(expected: impl AsRef<str>, found: impl AsRef<str>) -> StockTrekResult<T> {
    Err(StockTrekError::Value(ValueError::IncorrectType {
        expected: expected.as_ref().to_string(),
        found: found.as_ref().to_string(),
    }))
}
