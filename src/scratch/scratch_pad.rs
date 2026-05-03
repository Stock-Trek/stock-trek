use crate::{
    error::{
        result::{StockTrekError, StockTrekResult},
        value::ValueError,
    },
    scratch::{key::ScratchKey, key::ScratchPadKeyType, value::ScratchValue},
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, convert::TryFrom};

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
}

impl Default for ScratchPad {
    fn default() -> Self {
        Self::new()
    }
}

impl ScratchPad {
    pub fn read<T>(&self, key: &ScratchKey<T>) -> StockTrekResult<T>
    where
        T: Clone
            + ScratchPadKeyType
            + Into<ScratchValue>
            + TryFrom<ScratchValue, Error = StockTrekError>,
    {
        let key_str = key.key();
        let v = self.values.get(key_str);
        match v {
            None => match key.default() {
                None => Err(StockTrekError::Value(ValueError::NotFound {
                    name: "Key".to_string(),
                    key: key_str.to_string(),
                })),
                Some(d) => Ok(d),
            },
            Some(v) => {
                let typed = T::try_from(v.clone())?;
                Ok(typed)
            }
        }
    }
    pub fn write<T>(&mut self, key: &ScratchKey<T>, value: T)
    where
        T: Clone
            + ScratchPadKeyType
            + Into<ScratchValue>
            + TryFrom<ScratchValue, Error = StockTrekError>,
    {
        self.values.insert(key.key().to_string(), value.into());
    }
}
