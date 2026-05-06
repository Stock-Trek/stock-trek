use crate::{
    error::result::{StockTrekError, StockTrekResult},
    resolved_context::ResolvedContext,
    scratch::value::ScratchValue,
};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, marker::PhantomData};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScratchKey<T> {
    key: String,
    default: Option<T>,
    _phantom: PhantomData<T>,
}

impl<T> Display for ScratchKey<T>
where
    T: Clone
        + ScratchPadKeyType
        + Into<ScratchValue>
        + TryFrom<ScratchValue, Error = StockTrekError>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ScratchPadKey::{}({})", T::KEY_NAME, &self.key)
    }
}

impl<T> ScratchKey<T>
where
    T: Clone
        + ScratchPadKeyType
        + Into<ScratchValue>
        + TryFrom<ScratchValue, Error = StockTrekError>,
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
        c.scratch_pad.read(self)
    }
}

mod sealed {
    use crate::scratch::key::{ExchangeName, TokenName};

    pub trait Sealed {
        const KEY_NAME: &str;
    }
    impl Sealed for ExchangeName {
        const KEY_NAME: &str = "Exchange";
    }
    impl Sealed for TokenName {
        const KEY_NAME: &str = "Token";
    }
    impl Sealed for bool {
        const KEY_NAME: &str = "Flag";
    }
    impl Sealed for f64 {
        const KEY_NAME: &str = "Number";
    }
}

pub trait ScratchPadKeyType: sealed::Sealed {}

impl ScratchPadKeyType for ExchangeName {}
impl ScratchPadKeyType for TokenName {}
impl ScratchPadKeyType for bool {}
impl ScratchPadKeyType for f64 {}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ExchangeName(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TokenName(pub String);

macro_rules! impl_string_wrapper_key {
    ($name:ident) => {
        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }
        impl std::ops::Deref for $name {
            type Target = String;
            fn deref(&self) -> &String {
                &self.0
            }
        }
        impl std::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut String {
                &mut self.0
            }
        }
        impl From<String> for $name {
            fn from(s: String) -> Self {
                $name(s)
            }
        }
        impl From<&str> for $name {
            fn from(s: &str) -> Self {
                $name(s.to_string())
            }
        }
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
        impl std::borrow::Borrow<str> for $name {
            fn borrow(&self) -> &str {
                &self.0
            }
        }
    };
}

impl_string_wrapper_key!(ExchangeName);
impl_string_wrapper_key!(TokenName);
