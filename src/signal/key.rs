use crate::{
    error::stock_trek_error::{StockTrekError, StockTrekResult},
    resolved_context::ResolvedContext,
    signal::signal_value::SignalValue,
};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, marker::PhantomData};
use stock_trek_types::cex::{account_id::AccountId, asset_id::AssetId, cex_id::CexId};

/// A typed key used to read and write a signal value.
///
/// Keys are created required, via [`Self::new_required`], or optional, via
/// [`Self::new_optional`] with a default. Reading an optional key yields the
/// default when the signal was not written this evaluation; reading a
/// required key errors instead.
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
    pub fn new_required(key: impl AsRef<str>) -> Self {
        Self {
            key: key.as_ref().to_string(),
            default: None,
            _phantom: PhantomData,
        }
    }
    /// Creates an optional signal key with a default value.
    pub fn new_optional(key: impl AsRef<str>, default: T) -> Self {
        Self {
            key: key.as_ref().to_string(),
            default: Some(default),
            _phantom: PhantomData,
        }
    }
    /// The name the signal is stored under.
    pub fn key(&self) -> &str {
        &self.key
    }
    /// The value returned when the signal was not written, if the key is
    /// optional.
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
