use crate::{
    error::result::{StockTrekError, StockTrekResult},
    resolved_context::ResolvedContext,
    signal::value::SignalValue,
    validation::Validation,
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
///
/// Keys are validated when created or deserialized: the name must not be empty
/// and an optional default must be a valid signal value.
#[derive(Debug, Clone, Serialize)]
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
    /// Leading and trailing whitespace is trimmed from `key`. Returns an error
    /// when the key is empty or whitespace-only.
    pub fn new_required(key: impl AsRef<str>) -> StockTrekResult<Self> {
        Self::from_parts("SignalKey::new_required", key, None)
    }
    /// Creates an optional signal key with a default value.
    ///
    /// Leading and trailing whitespace is trimmed from `key`. Returns an error
    /// when the key is empty or whitespace-only, or when `default` is not a
    /// valid signal value.
    pub fn new_optional(key: impl AsRef<str>, default: T) -> StockTrekResult<Self> {
        Self::from_parts("SignalKey::new_optional", key, Some(default))
    }
    fn from_parts(
        function: &'static str,
        key: impl AsRef<str>,
        default: Option<T>,
    ) -> StockTrekResult<Self> {
        let key = key.as_ref().trim();
        if key.is_empty() {
            return Err(Validation::error(function, "key must not be empty"));
        }
        if let Some(default) = &default {
            let default_value: SignalValue = default.clone().into();
            default_value.validate()?;
        }
        Ok(Self {
            key: key.to_string(),
            default,
            _phantom: PhantomData,
        })
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

#[derive(Deserialize)]
struct SignalKeyData<T> {
    key: String,
    default: Option<T>,
}

impl<'de, T> Deserialize<'de> for SignalKey<T>
where
    T: Deserialize<'de>
        + Clone
        + SignalKeyType
        + Into<SignalValue>
        + TryFrom<SignalValue, Error = StockTrekError>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let data = SignalKeyData::<T>::deserialize(deserializer)?;
        Self::from_parts("SignalKey::deserialize", data.key, data.default)
            .map_err(serde::de::Error::custom)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_empty_and_whitespace_only_keys() {
        for key in ["", "   ", "\t\n"] {
            assert!(SignalKey::<f64>::new_required(key).is_err());
            assert!(SignalKey::<f64>::new_optional(key, 1.0).is_err());
        }
    }

    #[test]
    fn trims_keys() {
        let key = SignalKey::<f64>::new_required(" number ").unwrap();

        assert_eq!(key.key(), "number");
    }

    #[test]
    fn validates_optional_defaults_when_the_key_is_created() {
        assert!(SignalKey::<f64>::new_optional("number", 1.0).is_ok());
        for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            assert!(SignalKey::<f64>::new_optional("number", value).is_err());
        }
    }

    #[test]
    fn serde_round_trip_preserves_the_key() {
        let key = SignalKey::<f64>::new_optional("number", 1.0).unwrap();
        let json = serde_json::to_string(&key).unwrap();
        let deserialized: SignalKey<f64> = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.key(), "number");
        assert_eq!(deserialized.default(), Some(1.0));
    }

    #[test]
    fn deserialization_rejects_an_empty_key() {
        let json = serde_json::json!({
            "key": "",
            "default": 1.0,
        });

        assert!(serde_json::from_value::<SignalKey<f64>>(json).is_err());
    }
}
