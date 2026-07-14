use crate::{
    error::{
        result::{StockTrekError, StockTrekResult},
        value::ValueError,
    },
    signal::{key::SignalKey, key::SignalKeyType, value::SignalValue},
};
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Debug, Serialize, Deserialize)]
pub struct Signals {
    signal_values: HashMap<String, SignalValue>,
}

impl Signals {
    pub fn new() -> Self {
        Self {
            signal_values: HashMap::new(),
        }
    }
}

impl Default for Signals {
    fn default() -> Self {
        Self::new()
    }
}

impl Signals {
    pub fn read<T>(&self, key: &SignalKey<T>) -> StockTrekResult<T>
    where
        T: Clone + SignalKeyType + Into<SignalValue> + TryFrom<SignalValue, Error = StockTrekError>,
    {
        let key_str = key.key();
        let v = self.signal_values.get(key_str);
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
    pub fn write<T>(&mut self, key: &SignalKey<T>, value: T)
    where
        T: Clone + SignalKeyType + Into<SignalValue> + TryFrom<SignalValue, Error = StockTrekError>,
    {
        self.signal_values
            .insert(key.key().to_string(), value.into());
    }
}
