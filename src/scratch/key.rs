use crate::{error::result::StockTrekError, scratch::value::ScratchValue};
use digdigdig3::{Asset, ExchangeId};
use std::{fmt::Display, marker::PhantomData};

#[derive(Debug)]
pub struct ScratchKey<T> {
    key: String,
    _phantom: PhantomData<T>,
}

impl<T> Display for ScratchKey<T>
where
    T: ScratchPadKeyType,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ScratchPadKey::{}({})", T::KEY_NAME, &self.key)
    }
}

impl<T> ScratchKey<T>
where
    T: ScratchPadKeyType + Into<ScratchValue> + TryFrom<ScratchValue, Error = StockTrekError>,
{
    pub fn new(key: impl AsRef<str>) -> Self {
        Self {
            key: key.as_ref().to_string(),
            _phantom: PhantomData,
        }
    }
    pub fn key(&self) -> String {
        self.key.to_string()
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
