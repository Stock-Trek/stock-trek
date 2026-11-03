use crate::{
    error::result::{StockTrekError, StockTrekResult},
    resolved_context::ResolvedContext,
    signal::value::SignalValue,
};
use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    hash::{Hash, Hasher},
    marker::PhantomData,
};
use stock_trek_types::cex::{account_id::AccountId, asset_id::AssetId, cex_id::CexId};

/// A typed key used to read and write a signal value.
///
/// Keys are created required, via [`Self::new_required`], or optional, via
/// [`Self::new_optional`] with a default. Reading an optional key yields the
/// default when the signal was not written this evaluation; reading a
/// required key errors instead.
///
/// Equality and hashing use both the signal name and, when present, the
/// optional default value. Numeric defaults compare equal when they are both
/// NaN, and hash -0.0 and 0.0 identically.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalKey<T> {
    key: String,
    default: Option<T>,
    #[serde(skip)]
    _phantom: PhantomData<T>,
}

impl<T> PartialEq for SignalKey<T>
where
    T: SignalKeyType,
{
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
            && match (&self.default, &other.default) {
                (Some(a), Some(b)) => a.key_eq(b),
                (None, None) => true,
                _ => false,
            }
    }
}

impl<T> Eq for SignalKey<T> where T: SignalKeyType {}

impl<T> Hash for SignalKey<T>
where
    T: SignalKeyType,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.key.hash(state);
        match &self.default {
            Some(value) => {
                true.hash(state);
                value.key_hash(state);
            }
            None => false.hash(state),
        }
    }
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
    ///
    /// # Errors
    ///
    /// Returns an error when a numeric default is not finite.
    pub fn new_optional(key: impl AsRef<str>, default: T) -> StockTrekResult<Self> {
        let default_value: SignalValue = default.clone().into();
        default_value.validate()?;
        Ok(Self {
            key: key.as_ref().to_string(),
            default: Some(default),
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
        c.signals().read(self)
    }
}

mod sealed {
    use std::hash::{Hash, Hasher};
    use stock_trek_types::cex::{account_id::AccountId, asset_id::AssetId, cex_id::CexId};

    pub trait Sealed {
        const KEY_NAME: &str;

        fn key_eq(&self, other: &Self) -> bool;
        fn key_hash<H: Hasher>(&self, state: &mut H);
    }

    macro_rules! impl_sealed {
        ($($ty:ty => $name:literal),+ $(,)?) => {
            $(
                impl Sealed for $ty {
                    const KEY_NAME: &str = $name;

                    fn key_eq(&self, other: &Self) -> bool {
                        self == other
                    }

                    fn key_hash<H: Hasher>(&self, state: &mut H) {
                        self.hash(state);
                    }
                }
            )+
        };
    }

    impl_sealed! {
        CexId => "CexId",
        AccountId => "AccountId",
        AssetId => "AssetId",
        bool => "Flag",
    }

    impl Sealed for f64 {
        const KEY_NAME: &str = "Number";

        fn key_eq(&self, other: &Self) -> bool {
            self == other || (self.is_nan() && other.is_nan())
        }

        fn key_hash<H: Hasher>(&self, state: &mut H) {
            if self.is_nan() {
                f64::NAN.to_bits().hash(state);
            } else if *self == 0.0 {
                0.0_f64.to_bits().hash(state);
            } else {
                self.to_bits().hash(state);
            }
        }
    }
}

/// Types that can be used as the value of a [`SignalKey`].
pub trait SignalKeyType: sealed::Sealed {}

impl SignalKeyType for CexId {}
impl SignalKeyType for AccountId {}
impl SignalKeyType for AssetId {}
impl SignalKeyType for bool {}
impl SignalKeyType for f64 {}
