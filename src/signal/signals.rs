use crate::{
    error::{
        result::{StockTrekError, StockTrekResult},
        value::ValueError,
    },
    signal::{key::SignalKey, key::SignalKeyType, value::SignalValue},
};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, btree_map},
    convert::TryFrom,
};

/// A collection of named signal values.
///
/// Values are validated when written or deserialized. Iteration yields entries
/// in ascending key order and only includes values that were written this
/// evaluation; defaults from optional keys are not stored.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(try_from = "SignalsData")]
pub struct Signals {
    pub(super) signal_values: BTreeMap<String, SignalValue>,
}

#[derive(Deserialize)]
pub(super) struct SignalsData {
    pub(super) signal_values: BTreeMap<String, SignalValue>,
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

    /// Returns the number of stored signal values.
    pub fn len(&self) -> usize {
        self.signal_values.len()
    }

    /// Returns whether no signal values are stored.
    pub fn is_empty(&self) -> bool {
        self.signal_values.is_empty()
    }

    /// Iterates over the stored signal names and values in ascending key order.
    pub fn iter(&self) -> impl Iterator<Item = (&String, &SignalValue)> {
        self.signal_values.iter()
    }

    /// Iterates over the stored signal names in ascending order.
    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.signal_values.keys()
    }

    /// Iterates over the stored signal values in ascending key order.
    pub fn values(&self) -> impl Iterator<Item = &SignalValue> {
        self.signal_values.values()
    }

    /// Returns the stored value for a signal name, if it was written.
    pub fn get(&self, key: impl AsRef<str>) -> Option<&SignalValue> {
        self.signal_values.get(key.as_ref())
    }

    /// Returns whether a value is stored for a signal name.
    pub fn contains_key(&self, key: impl AsRef<str>) -> bool {
        self.signal_values.contains_key(key.as_ref())
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

impl<'a> IntoIterator for &'a Signals {
    type Item = (&'a String, &'a SignalValue);
    type IntoIter = btree_map::Iter<'a, String, SignalValue>;

    fn into_iter(self) -> Self::IntoIter {
        self.signal_values.iter()
    }
}

impl IntoIterator for Signals {
    type Item = (String, SignalValue);
    type IntoIter = btree_map::IntoIter<String, SignalValue>;

    fn into_iter(self) -> Self::IntoIter {
        self.signal_values.into_iter()
    }
}
