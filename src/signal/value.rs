use crate::error::{
    result::{StockTrekError, StockTrekResult},
    value::ValueError,
};
use serde::{Deserialize, Serialize};
use stock_trek_types::cex::{account_id::AccountId, asset_id::AssetId, cex_id::CexId};
use strum::Display;

#[derive(Debug, Display, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
#[non_exhaustive]
pub enum SignalValue {
    CexId(CexId),
    AccountId(AccountId),
    AssetId(AssetId),
    Flag(bool),
    Number(f64),
}

impl From<CexId> for SignalValue {
    fn from(value: CexId) -> Self {
        SignalValue::CexId(value)
    }
}
impl From<AccountId> for SignalValue {
    fn from(value: AccountId) -> Self {
        SignalValue::AccountId(value)
    }
}
impl From<AssetId> for SignalValue {
    fn from(value: AssetId) -> Self {
        SignalValue::AssetId(value)
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
            SignalValue::AccountId(_) => err("CexId", "AccountId"),
            SignalValue::AssetId(_) => err("CexId", "AssetId"),
            SignalValue::Flag(_) => err("CexId", "Flag"),
            SignalValue::Number(_) => err("CexId", "Number"),
        }
    }
}
impl TryFrom<SignalValue> for AccountId {
    type Error = StockTrekError;
    fn try_from(value: SignalValue) -> StockTrekResult<Self> {
        match value {
            SignalValue::CexId(_) => err("AccountId", "CexId"),
            SignalValue::AccountId(a) => Ok(a),
            SignalValue::AssetId(_) => err("AccountId", "AssetId"),
            SignalValue::Flag(_) => err("AccountId", "Flag"),
            SignalValue::Number(_) => err("AccountId", "Number"),
        }
    }
}
impl TryFrom<SignalValue> for AssetId {
    type Error = StockTrekError;
    fn try_from(value: SignalValue) -> StockTrekResult<Self> {
        match value {
            SignalValue::CexId(_) => err("AssetId", "CexId"),
            SignalValue::AccountId(_) => err("AssetId", "AccountId"),
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
            SignalValue::AccountId(_) => err("Flag", "AccountId"),
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
            SignalValue::AccountId(_) => err("Number", "AccountId"),
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
