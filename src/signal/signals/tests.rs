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
