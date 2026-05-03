use crate::{
    error::result::StockTrekError, prelude::StockTrekResult, resolved_context::ResolvedContext,
    scratch::value::ScratchValue,
};
use digdigdig3::{Asset, ExchangeId};
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
    pub fn read(&self, context: &ResolvedContext) -> StockTrekResult<T> {
        context.scratch_pad.read(self)
    }
}

mod sealed {
    use digdigdig3::{Asset, ExchangeId};

    pub trait Sealed {
        const KEY_NAME: &str;
    }
    impl Sealed for Asset {
        const KEY_NAME: &str = "Asset";
    }
    impl Sealed for ExchangeId {
        const KEY_NAME: &str = "Exchange";
    }
    impl Sealed for bool {
        const KEY_NAME: &str = "Flag";
    }
    impl Sealed for f64 {
        const KEY_NAME: &str = "Number";
    }
}

pub trait ScratchPadKeyType: sealed::Sealed {}

impl ScratchPadKeyType for Asset {}
impl ScratchPadKeyType for ExchangeId {}
impl ScratchPadKeyType for bool {}
impl ScratchPadKeyType for f64 {}
