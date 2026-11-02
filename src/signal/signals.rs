use crate::{
    error::{
        stock_trek_error::{StockTrekError, StockTrekResult},
        value::ValueError,
    },
    signal::{key::SignalKey, key::SignalKeyType, signal_value::SignalValue},
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
        signals.write(&z, 2.0).unwrap();
        signals.write(&a, 1.0).unwrap();

        let json = serde_json::to_string(&signals).unwrap();

        assert_eq!(
            json,
            r#"{"signal_values":{"a":{"number":1.0},"z":{"number":2.0}}}"#
        );
    }

    #[test]
    fn write_rejects_non_finite_numbers() {
        let mut signals = Signals::new();
        let key = SignalKey::<f64>::new_required("number");
        signals.write(&key, 1.0).unwrap();

        for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            assert!(signals.write(&key, value).is_err());
            assert_eq!(signals.read(&key).unwrap(), 1.0);
        }
    }

    #[test]
    fn read_rejects_a_non_finite_stored_value() {
        let mut signals = Signals::new();
        let key = SignalKey::<f64>::new_required("number");
        signals
            .signal_values
            .insert(key.key().to_string(), SignalValue::Number(f64::NAN));

        assert!(signals.read(&key).is_err());
    }

    #[test]
    fn read_rejects_a_non_finite_optional_default() {
        let key = SignalKey::<f64>::new_optional("number", f64::INFINITY);

        assert!(Signals::new().read(&key).is_err());
    }

    #[test]
    fn read_accepts_a_finite_optional_default() {
        let key = SignalKey::<f64>::new_optional("number", 1.0);

        assert_eq!(Signals::new().read(&key).unwrap(), 1.0);
    }

    #[test]
    fn deserialization_rejects_non_finite_numbers() {
        let data = SignalsData {
            signal_values: BTreeMap::from([(
                "number".to_string(),
                SignalValue::Number(f64::INFINITY),
            )]),
        };

        assert!(Signals::try_from(data).is_err());
    }
}
