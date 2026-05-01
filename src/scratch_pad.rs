use anyhow::{bail, Result};
use digdigdig3::{Asset, ExchangeId};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, convert::TryFrom};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScratchValue {
    Asset(Asset),
    Exchange(ExchangeId),
    Flag(bool),
    Number(f64),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScratchPad {
    values: HashMap<String, ScratchValue>,
}

impl ScratchPad {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }
    pub fn write_asset(&mut self, key: impl AsRef<str>, asset: impl AsRef<str>) -> () {
        self.write(key, ScratchValue::Asset(asset.as_ref().to_string()))
    }
    pub fn write_exchange(&mut self, key: impl AsRef<str>, exchange: ExchangeId) -> () {
        self.write(key, ScratchValue::Exchange(exchange))
    }
    pub fn write_flag(&mut self, key: impl AsRef<str>, flag: bool) -> () {
        self.write(key, ScratchValue::Flag(flag))
    }
    pub fn write_number(&mut self, key: impl AsRef<str>, number: f64) -> () {
        self.write(key, ScratchValue::Number(number))
    }
    pub fn read_asset_required(&self, key: impl AsRef<str>) -> Result<Asset> {
        self.read_required::<Asset>(key)
    }
    pub fn read_exchange_required(&self, key: impl AsRef<str>) -> Result<ExchangeId> {
        self.read_required::<ExchangeId>(key)
    }
    pub fn read_flag_required(&self, key: impl AsRef<str>) -> Result<bool> {
        self.read_required::<bool>(key)
    }
    pub fn read_number_required(&self, key: impl AsRef<str>) -> Result<f64> {
        self.read_required::<f64>(key)
    }
    pub fn read_asset_optional(&self, key: impl AsRef<str>) -> Option<Asset> {
        self.read_optional::<Asset>(key)
    }
    pub fn read_exchange_optional(&self, key: impl AsRef<str>) -> Option<ExchangeId> {
        self.read_optional::<ExchangeId>(key)
    }
    pub fn read_flag_optional(&self, key: impl AsRef<str>) -> Option<bool> {
        self.read_optional::<bool>(key)
    }
    pub fn read_number_optional(&self, key: impl AsRef<str>) -> Option<f64> {
        self.read_optional::<f64>(key)
    }
}

impl ScratchPad {
    pub fn write(&mut self, key: impl AsRef<str>, value: ScratchValue) -> () {
        self.values.insert(key.as_ref().to_string(), value);
        ()
    }
    fn read_required<T>(&self, key: impl AsRef<str>) -> Result<T>
    where
        T: TryFrom<ScratchValue, Error = anyhow::Error>,
    {
        let value = self.values.get(key.as_ref());
        match value {
            None => bail!("Key not found: {}", key.as_ref()),
            Some(v) => T::try_from(v.clone()),
        }
    }
    fn read_optional<T>(&self, key: impl AsRef<str>) -> Option<T>
    where
        T: TryFrom<ScratchValue, Error = anyhow::Error>,
    {
        let value = self.values.get(key.as_ref());
        match value {
            None => None,
            Some(v) => match T::try_from(v.clone()) {
                Err(..) => None,
                Ok(t) => Some(t),
            },
        }
    }
}

impl TryFrom<ScratchValue> for Asset {
    type Error = anyhow::Error;
    fn try_from(value: ScratchValue) -> Result<Self, Self::Error> {
        match value {
            ScratchValue::Asset(a) => Ok(a),
            _ => bail!("Found value but is not an Asset"),
        }
    }
}

impl TryFrom<ScratchValue> for ExchangeId {
    type Error = anyhow::Error;
    fn try_from(value: ScratchValue) -> Result<Self, Self::Error> {
        match value {
            ScratchValue::Exchange(e) => Ok(e),
            _ => bail!("Found value but is not an Exchange"),
        }
    }
}

impl TryFrom<ScratchValue> for bool {
    type Error = anyhow::Error;
    fn try_from(value: ScratchValue) -> Result<Self, Self::Error> {
        match value {
            ScratchValue::Flag(f) => Ok(f),
            _ => bail!("Found value but is not a Flag"),
        }
    }
}

impl TryFrom<ScratchValue> for f64 {
    type Error = anyhow::Error;
    fn try_from(value: ScratchValue) -> Result<Self, Self::Error> {
        match value {
            ScratchValue::Number(n) => Ok(n),
            _ => bail!("Found value but is not a Number"),
        }
    }
}
