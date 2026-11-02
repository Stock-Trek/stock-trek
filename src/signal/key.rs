use crate::{
    error::result::{StockTrekError, StockTrekResult},
    resolved_context::ResolvedContext,
    signal::value::SignalValue,
};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, marker::PhantomData};
use stock_trek_types::cex::{account_id::AccountId, asset_id::AssetId, cex_id::CexId};

/// A typed key used to read and write a signal value.
///
/// The same key must be used when writing a signal in
/// [`Algorithm::signals`](crate::Algorithm::signals) and when reading it in a
/// strategy. Required keys produce an error when the signal is absent; optional
/// keys fall back to their default.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalKey<T> {
    key: String,
    default: Option<T>,
    #[serde(skip)]
    _phantom: PhantomData<T>,
}

impl<T> Display for SignalKey<T>
where
    T: Clone + SignalKeyType + Into<SignalValue> + TryFrom<SignalValue, Error = StockTrekError>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SignalKey::{}({})", T::KEY_NAME, self.key)
    }
}

impl<T> SignalKey<T>
where
    T: Clone + SignalKeyType + Into<SignalValue> + TryFrom<SignalValue, Error = StockTrekError>,
{
    /// Creates a required signal key that has no default value.
    ///
    /// Reading this key fails with a value-not-found error if the algorithm did
    /// not write it during the current evaluation.
    pub fn new_required(key: impl AsRef<str>) -> Self {
        Self {
            key: key.as_ref().to_string(),
            default: None,
            _phantom: PhantomData,
        }
    }
    /// Creates an optional signal key with a default value.
    ///
    /// Reading this key returns the default when the signal was not written
    /// during the current evaluation.
    pub fn new_optional(key: impl AsRef<str>, default: T) -> Self {
        Self {
            key: key.as_ref().to_string(),
            default: Some(default),
            _phantom: PhantomData,
        }
    }
    /// Returns the string key used to store and look up the signal.
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Returns the key's default value, or `None` for a required key.
    pub fn default(&self) -> Option<T> {
        self.default.clone()
    }
    /// Reads the signal value from the resolved context.
    pub fn read(&self, c: &ResolvedContext) -> StockTrekResult<T> {
        c.signals.read(self)
    }
}

mod sealed {
    use stock_trek_types::cex::{account_id::AccountId, asset_id::AssetId, cex_id::CexId};

    pub trait Sealed {
        const KEY_NAME: &str;
    }
    impl Sealed for CexId {
        const KEY_NAME: &str = "CexId";
    }
    impl Sealed for AccountId {
        const KEY_NAME: &str = "AccountId";
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

/// Types that can be used as the value of a [`SignalKey`].
pub trait SignalKeyType: sealed::Sealed {}

impl SignalKeyType for CexId {}
impl SignalKeyType for AccountId {}
impl SignalKeyType for AssetId {}
impl SignalKeyType for bool {}
impl SignalKeyType for f64 {}
