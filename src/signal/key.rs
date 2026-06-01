use crate::{
    asset_id::AssetId,
    error::result::{StockTrekError, StockTrekResult},
    exchange_id::ExchangeId,
    resolved_context::ResolvedContext,
    signal::value::SignalValue,
};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, marker::PhantomData};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalKey<T> {
    key: String,
    default: Option<T>,
    _phantom: PhantomData<T>,
}

impl<T> Display for SignalKey<T>
where
    T: Clone + SignalKeyType + Into<SignalValue> + TryFrom<SignalValue, Error = StockTrekError>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SignalKey::{}({})", T::KEY_NAME, &self.key)
    }
}

impl<T> SignalKey<T>
where
    T: Clone + SignalKeyType + Into<SignalValue> + TryFrom<SignalValue, Error = StockTrekError>,
{
    pub fn new_required(key: impl AsRef<str>) -> Self {
        Self {
            key: key.as_ref().to_string(),
            default: None,
            _phantom: PhantomData,
        }
    }
    pub fn new_optional(key: impl AsRef<str>, default: T) -> Self {
        Self {
            key: key.as_ref().to_string(),
            default: Some(default),
            _phantom: PhantomData,
        }
    }
    pub fn key(&self) -> &str {
        &self.key
    }
    pub fn default(&self) -> Option<T> {
        self.default.clone()
    }
    pub fn read(&self, c: &ResolvedContext) -> StockTrekResult<T> {
        c.signals.read(self)
    }
}

mod sealed {
    use crate::{asset_id::AssetId, signal::key::ExchangeId};

    pub trait Sealed {
        const KEY_NAME: &str;
    }
    impl Sealed for ExchangeId {
        const KEY_NAME: &str = "ExchangeId";
    }
    impl Sealed for AssetId {
        const KEY_NAME: &str = "AssetId";
    }
    impl Sealed for bool {
        const KEY_NAME: &str = "Flag";
    }
    impl Sealed for f64 {
        const KEY_NAME: &str = "Number";
    }
}

pub trait SignalKeyType: sealed::Sealed {}

impl SignalKeyType for ExchangeId {}
impl SignalKeyType for AssetId {}
impl SignalKeyType for bool {}
impl SignalKeyType for f64 {}
