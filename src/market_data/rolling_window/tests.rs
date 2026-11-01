use super::*;
use strum::IntoEnumIterator;

#[test]
fn display_uses_short_labels() {
    assert_eq!(RollingWindow::Seconds1.to_string(), "1s");
    assert_eq!(RollingWindow::Hours24.to_string(), "24h");
    assert_eq!(RollingWindow::Days7.to_string(), "7d");
}

#[test]
fn display_round_trips_through_try_from() {
    for window in RollingWindow::iter() {
        assert_eq!(window.label(), window.to_string());
        assert_eq!(
            RollingWindow::try_from(window.to_string().as_str()).unwrap(),
            window
        );
    }
}

#[test]
fn serde_round_trips_short_labels() {
    for window in RollingWindow::iter() {
        let json = serde_json::to_string(&window).unwrap();

        assert_eq!(json, format!("\"{}\"", window.label()));
        assert_eq!(
            serde_json::from_str::<RollingWindow>(&json).unwrap(),
            window
        );
    }
}

#[test]
fn serde_rejects_names() {
    assert!(serde_json::from_str::<RollingWindow>("\"seconds1\"").is_err());
}

#[test]
fn checked_add_and_sub_use_the_window_length() {
    let date_time = DateTime::from_timestamp_millis(1_700_000_000_123).unwrap();

    for window in RollingWindow::iter() {
        let added = window.checked_add(date_time).unwrap();
        let subtracted = window.checked_sub(date_time).unwrap();
        let forward = added.timestamp_millis() - date_time.timestamp_millis();
        let backward = date_time.timestamp_millis() - subtracted.timestamp_millis();

        assert_eq!(forward, backward, "{window} is not symmetric");
        assert_eq!(
            i64::try_from(window.millis_upper_bound().unwrap()).unwrap(),
            forward,
            "{window} does not advance by its own length"
        );
    }
}

#[test]
fn checked_add_and_sub_return_none_when_out_of_range() {
    for window in RollingWindow::iter() {
        assert_eq!(
            window.checked_add(DateTime::<Utc>::MAX_UTC),
            None,
            "{window} should not add past the maximum date"
        );
        assert_eq!(
            window.checked_sub(DateTime::<Utc>::MIN_UTC),
            None,
            "{window} should not subtract past the minimum date"
        );
    }
}
