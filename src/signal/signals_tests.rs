use super::signals::*;

use crate::{
    error::{result::StockTrekError, value::ValueError},
    signal::{
        key::{SignalKey, SignalKeyType},
        signals::SignalsData,
        value::SignalValue,
    },
};
use std::collections::BTreeMap;

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
        signal_values: BTreeMap::from([("number".to_string(), SignalValue::Number(f64::INFINITY))]),
    };

    assert!(Signals::try_from(data).is_err());
}
