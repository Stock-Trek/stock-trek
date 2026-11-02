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
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Signals {
    signal_values: BTreeMap<String, SignalValue>,
}

impl Signals {
    /// Creates an empty signal collection.
    pub fn new() -> Self {
        Self::default()
    }

    /// Reads a typed value from the collection, falling back to the key's default.
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
                Some(d) => Ok(d),
            },
            Some(v) => {
                let typed = T::try_from(v.clone())?;
                Ok(typed)
            }
        }
    }
    /// Writes a typed value to the collection under the given key.
    pub fn write<T>(&mut self, key: &SignalKey<T>, value: T)
    where
        T: Clone + SignalKeyType + Into<SignalValue> + TryFrom<SignalValue, Error = StockTrekError>,
    {
        self.signal_values
            .insert(key.key().to_string(), value.into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn missing_name<T>(key: &SignalKey<T>) -> String
    where
        T: Clone + SignalKeyType + Into<SignalValue> + TryFrom<SignalValue, Error = StockTrekError>,
    {
        match Signals::new().read(key) {
            Err(StockTrekError::Value(ValueError::NotFound { name, .. })) => name,
            Err(other) => panic!("expected NotFound, got {other:?}"),
            Ok(_) => panic!("expected NotFound, got a value"),
        }
    }

    #[test]
    fn missing_signal_error_uses_key_type_name() {
        let number = SignalKey::<f64>::new_required("missing_number");
        assert_eq!(missing_name(&number), "Number");

        let flag = SignalKey::<bool>::new_required("missing_flag");
        assert_eq!(missing_name(&flag), "Flag");
    }

    #[test]
    fn serializes_signal_values_in_key_order() {
        let mut signals = Signals::new();
        let z = SignalKey::<f64>::new_required("z");
        let a = SignalKey::<f64>::new_required("a");
        signals.write(&z, 2.0);
        signals.write(&a, 1.0);

        let json = serde_json::to_string(&signals).unwrap();

        assert_eq!(
            json,
            r#"{"signal_values":{"a":{"number":1.0},"z":{"number":2.0}}}"#
        );
    }
}
