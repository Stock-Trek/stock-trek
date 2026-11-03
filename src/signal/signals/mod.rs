use crate::{
    error::{
        result::{StockTrekError, StockTrekResult},
        value::ValueError,
    },
    signal::{key::SignalKey, key::SignalKeyType, value::SignalValue},
};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, convert::TryFrom};

/// A collection of named signal values.
///
/// Values are validated when written or deserialized.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(try_from = "SignalsData")]
pub struct Signals {
    signal_values: BTreeMap<String, SignalValue>,
}

#[derive(Deserialize)]
struct SignalsData {
    signal_values: BTreeMap<String, SignalValue>,
}

impl TryFrom<SignalsData> for Signals {
    type Error = StockTrekError;

    fn try_from(data: SignalsData) -> StockTrekResult<Self> {
        for value in data.signal_values.values() {
            value.validate()?;
        }
        Ok(Self {
            signal_values: data.signal_values,
        })
    }
}

impl Signals {
    /// Creates an empty signal collection.
    pub fn new() -> Self {
        Self::default()
    }

    /// Reads a typed value from the collection, falling back to the key's default.
    ///
    /// Returns an error when the value is a non-finite number.
    pub fn read<T>(&self, key: &SignalKey<T>) -> StockTrekResult<T>
    where
        T: Clone + SignalKeyType + Into<SignalValue> + TryFrom<SignalValue, Error = StockTrekError>,
    {
        let key_str = key.key();
        let v = self.signal_values.get(key_str);
        match v {
            None => match key.default() {
                None => Err(StockTrekError::Value(ValueError::NotFound {
                    name: T::KEY_NAME.to_string(),
                    key: key_str.to_string(),
                })),
                Some(d) => {
                    let default_value: SignalValue = d.clone().into();
                    default_value.validate()?;
                    Ok(d)
                }
            },
            Some(v) => {
                v.validate()?;
                let typed = T::try_from(v.clone())?;
                Ok(typed)
            }
        }
    }

    /// Writes a typed value to the collection under the given key.
    ///
    /// Returns an error, leaving the collection unchanged, when a number is
    /// not finite.
    pub fn write<T>(&mut self, key: &SignalKey<T>, value: T) -> StockTrekResult<()>
    where
        T: Clone + SignalKeyType + Into<SignalValue> + TryFrom<SignalValue, Error = StockTrekError>,
    {
        let value: SignalValue = value.into();
        value.validate()?;
        self.signal_values.insert(key.key().to_string(), value);
        Ok(())
    }
}

#[cfg(test)]
mod tests;
