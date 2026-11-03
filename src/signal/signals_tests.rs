use super::signals::*;

use crate::{
    error::{result::StockTrekError, value::ValueError},
    signal::{
        key::{SignalKey, SignalKeyType},
        signals::SignalsData,
        value::SignalValue,
    },
};
use std::collections::{BTreeMap, HashSet};

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
fn signal_keys_compare_by_name_and_default() {
    let a = SignalKey::<f64>::new_required("a");
    let b = SignalKey::<f64>::new_required("a");
    let c = SignalKey::<f64>::new_required("c");

    assert_eq!(a, b);
    assert_ne!(a, c);
    assert_ne!(a, SignalKey::<f64>::new_optional("a", 1.0));
    assert_eq!(
        SignalKey::<f64>::new_optional("a", 1.0),
        SignalKey::<f64>::new_optional("a", 1.0)
    );
    assert_ne!(
        SignalKey::<f64>::new_optional("a", 1.0),
        SignalKey::<f64>::new_optional("a", 2.0)
    );
}

#[test]
fn signal_keys_are_eq_and_hash_for_nan_defaults() {
    let a = SignalKey::<f64>::new_optional("number", f64::NAN);
    let b = SignalKey::<f64>::new_optional("number", f64::NAN);

    assert_eq!(a, b);

    let mut keys = HashSet::new();
    keys.insert(a);
    assert!(keys.contains(&b));
}

#[test]
fn signal_keys_hash_zero_defaults_consistently() {
    let mut keys = HashSet::new();
    keys.insert(SignalKey::<f64>::new_optional("number", 0.0));

    assert!(keys.contains(&SignalKey::<f64>::new_optional("number", -0.0)));
}

#[test]
fn iterates_over_stored_signals_in_key_order() {
    let mut signals = Signals::new();
    let z = SignalKey::<f64>::new_required("z");
    let a = SignalKey::<bool>::new_required("a");
    let m = SignalKey::<f64>::new_optional("m", 3.0);
    signals.write(&z, 2.0).unwrap();
    signals.write(&a, true).unwrap();
    signals.write(&m, 1.0).unwrap();

    assert_eq!(signals.len(), 3);
    assert!(!signals.is_empty());
    assert!(signals.contains_key("a"));
    assert!(!signals.contains_key("b"));
    assert!(signals.get("a").is_some());
    assert!(signals.get("b").is_none());

    let keys: Vec<_> = signals.keys().map(String::as_str).collect();
    assert_eq!(keys, ["a", "m", "z"]);

    let entries: Vec<_> = signals.iter().map(|(key, _)| key.as_str()).collect();
    assert_eq!(entries, ["a", "m", "z"]);

    let borrowed: Vec<_> = (&signals)
        .into_iter()
        .map(|(key, _)| key.as_str())
        .collect();
    assert_eq!(borrowed, ["a", "m", "z"]);

    let owned: Vec<_> = signals.into_iter().map(|(key, _)| key).collect();
    assert_eq!(owned, ["a", "m", "z"]);
}

#[test]
fn new_signals_are_empty() {
    let signals = Signals::new();

    assert_eq!(signals.len(), 0);
    assert!(signals.is_empty());
    assert_eq!(signals.iter().count(), 0);
    assert_eq!(signals.keys().count(), 0);
    assert_eq!(signals.values().count(), 0);
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
fn new_optional_rejects_a_non_finite_default() {
    for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert!(SignalKey::<f64>::new_optional("number", value).is_err());
    }
}

#[test]
fn read_accepts_a_finite_optional_default() {
    let key = SignalKey::<f64>::new_optional("number", 1.0).unwrap();

    assert_eq!(Signals::new().read(&key).unwrap(), 1.0);
}

#[test]
fn deserialization_rejects_non_finite_numbers() {
    let data = SignalsData {
        signal_values: BTreeMap::from([("number".to_string(), SignalValue::Number(f64::INFINITY))]),
    };

    assert!(Signals::try_from(data).is_err());
}
