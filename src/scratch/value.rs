use crate::{
    error::{
        result::{StockTrekError, StockTrekResult},
        value::ValueError,
    },
    scratch::key::{ExchangeName, TokenName},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScratchValue {
    Exchange(ExchangeName),
    Token(TokenName),
    Flag(bool),
    Number(f64),
}

impl From<TokenName> for ScratchValue {
    fn from(value: TokenName) -> Self {
        ScratchValue::Token(value)
    }
}
impl From<ExchangeName> for ScratchValue {
    fn from(value: ExchangeName) -> Self {
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

impl TryFrom<ScratchValue> for ExchangeName {
    type Error = StockTrekError;
    fn try_from(value: ScratchValue) -> StockTrekResult<Self> {
        match value {
            ScratchValue::Exchange(e) => Ok(e),
            ScratchValue::Token(_) => err("Exchange", "Token"),
            ScratchValue::Flag(_) => err("Exchange", "Flag"),
            ScratchValue::Number(_) => err("Exchange", "Number"),
        }
    }
}
impl TryFrom<ScratchValue> for TokenName {
    type Error = StockTrekError;
    fn try_from(value: ScratchValue) -> StockTrekResult<Self> {
        match value {
            ScratchValue::Exchange(_) => err("Token", "Exchange"),
            ScratchValue::Token(a) => Ok(a),
            ScratchValue::Flag(_) => err("Token", "Flag"),
            ScratchValue::Number(_) => err("Token", "Number"),
        }
    }
}
impl TryFrom<ScratchValue> for bool {
    type Error = StockTrekError;
    fn try_from(value: ScratchValue) -> StockTrekResult<Self> {
        match value {
            ScratchValue::Exchange(_) => err("Flag", "Exchange"),
            ScratchValue::Token(_) => err("Flag", "Token"),
            ScratchValue::Flag(f) => Ok(f),
            ScratchValue::Number(_) => err("Flag", "Number"),
        }
    }
}
impl TryFrom<ScratchValue> for f64 {
    type Error = StockTrekError;
    fn try_from(value: ScratchValue) -> StockTrekResult<Self> {
        match value {
            ScratchValue::Exchange(_) => err("Number", "Exchange"),
            ScratchValue::Token(_) => err("Number", "Token"),
            ScratchValue::Flag(_) => err("Number", "Flag"),
            ScratchValue::Number(n) => Ok(n),
        }
    }
}

fn err<T>(expected: impl AsRef<str>, found: impl AsRef<str>) -> StockTrekResult<T> {
    Err(StockTrekError::Value(ValueError::IncorrectType {
        expected: expected.as_ref().to_string(),
        found: found.as_ref().to_string(),
    }))
}
