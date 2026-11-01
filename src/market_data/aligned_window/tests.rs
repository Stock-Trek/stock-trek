use super::*;
use strum::IntoEnumIterator;

fn millis_at(year: i32, month: u32, day: u32, hour: u32, minute: u32, second: u32) -> i64 {
    Utc.with_ymd_and_hms(year, month, day, hour, minute, second)
        .unwrap()
        .timestamp_millis()
}

fn bounds_for(window: AlignedWindow, timestamp: i64) -> AlignedTimestampBounds {
    window.bounds_for(timestamp).unwrap()
}

fn timestamps(lower: i64, upper: i64) -> AlignedTimestampBounds {
    AlignedTimestampBounds::new(lower, upper)
}

fn is_calendar_window(window: AlignedWindow) -> bool {
    matches!(
        window,
        AlignedWindow::Months1
            | AlignedWindow::Months2
            | AlignedWindow::Months3
            | AlignedWindow::Months4
            | AlignedWindow::Months6
            | AlignedWindow::Months12
            | AlignedWindow::Years1
            | AlignedWindow::Years2
    )
}

#[test]
fn bounds_for_returns_containing_bounds() {
    let timestamps: [i64; 7] = [-1_700_000_000_123, -1, 0, 1, 999, 1_000, 1_700_000_000_123];

    for window in AlignedWindow::iter() {
        for timestamp in timestamps {
            let AlignedTimestampBounds { lower, upper } = bounds_for(window, timestamp);

            assert!(lower <= timestamp);
            assert!(timestamp < upper);

            if !is_calendar_window(window) {
                let window_millis = i64::try_from(window.millis_upper_bound().unwrap()).unwrap();
                assert_eq!(upper - lower, window_millis);
                assert_eq!(lower % window_millis, 0);
            }
        }
    }
}

#[test]
fn bounds_for_handles_pre_epoch_timestamps() {
    assert_eq!(
        bounds_for(AlignedWindow::Seconds1, -1),
        timestamps(-1_000, 0)
    );
    assert_eq!(
        bounds_for(AlignedWindow::Months1, millis_at(1969, 12, 15, 12, 0, 0)),
        timestamps(
            millis_at(1969, 12, 1, 0, 0, 0),
            millis_at(1970, 1, 1, 0, 0, 0)
        )
    );
}

#[test]
fn bounds_for_aligns_seconds_and_minutes() {
    assert_eq!(
        bounds_for(AlignedWindow::Seconds1, 1_500),
        timestamps(1_000, 2_000)
    );
    assert_eq!(
        bounds_for(AlignedWindow::Seconds5, 12_345),
        timestamps(10_000, 15_000)
    );
    assert_eq!(
        bounds_for(AlignedWindow::Minutes1, 61_234),
        timestamps(60_000, 120_000)
    );
    assert_eq!(
        bounds_for(AlignedWindow::Minutes5, 360_123),
        timestamps(300_000, 600_000)
    );
}

#[test]
fn bounds_for_month_windows_follow_calendar_boundaries() {
    let timestamp = millis_at(2024, 2, 15, 12, 34, 56);

    assert_eq!(
        bounds_for(AlignedWindow::Months1, timestamp),
        timestamps(
            millis_at(2024, 2, 1, 0, 0, 0),
            millis_at(2024, 3, 1, 0, 0, 0)
        )
    );
    assert_eq!(
        bounds_for(AlignedWindow::Months2, timestamp),
        timestamps(
            millis_at(2024, 1, 1, 0, 0, 0),
            millis_at(2024, 3, 1, 0, 0, 0)
        )
    );
    assert_eq!(
        bounds_for(AlignedWindow::Months3, timestamp),
        timestamps(
            millis_at(2024, 1, 1, 0, 0, 0),
            millis_at(2024, 4, 1, 0, 0, 0)
        )
    );
    assert_eq!(
        bounds_for(AlignedWindow::Months4, timestamp),
        timestamps(
            millis_at(2024, 1, 1, 0, 0, 0),
            millis_at(2024, 5, 1, 0, 0, 0)
        )
    );
    assert_eq!(
        bounds_for(AlignedWindow::Months6, timestamp),
        timestamps(
            millis_at(2024, 1, 1, 0, 0, 0),
            millis_at(2024, 7, 1, 0, 0, 0)
        )
    );
    assert_eq!(
        bounds_for(AlignedWindow::Months12, timestamp),
        timestamps(
            millis_at(2024, 1, 1, 0, 0, 0),
            millis_at(2025, 1, 1, 0, 0, 0)
        )
    );
    assert_eq!(
        bounds_for(AlignedWindow::Years1, timestamp),
        timestamps(
            millis_at(2024, 1, 1, 0, 0, 0),
            millis_at(2025, 1, 1, 0, 0, 0)
        )
    );
    assert_eq!(
        bounds_for(AlignedWindow::Years2, timestamp),
        timestamps(
            millis_at(2024, 1, 1, 0, 0, 0),
            millis_at(2026, 1, 1, 0, 0, 0)
        )
    );
}

#[test]
fn bounds_for_month_windows_include_window_boundaries() {
    let last_millis_of_february = millis_at(2024, 2, 29, 23, 59, 59) + 999;
    let expected = timestamps(
        millis_at(2024, 2, 1, 0, 0, 0),
        millis_at(2024, 3, 1, 0, 0, 0),
    );

    assert_eq!(
        bounds_for(AlignedWindow::Months1, millis_at(2024, 2, 1, 0, 0, 0)),
        expected
    );
    assert_eq!(
        bounds_for(AlignedWindow::Months1, last_millis_of_february),
        expected
    );
    assert_eq!(
        bounds_for(AlignedWindow::Months1, millis_at(2024, 3, 1, 0, 0, 0)),
        timestamps(
            millis_at(2024, 3, 1, 0, 0, 0),
            millis_at(2024, 4, 1, 0, 0, 0)
        )
    );
}

#[test]
fn bounds_for_month_windows_handle_leap_years() {
    let february_2024 = bounds_for(AlignedWindow::Months1, millis_at(2024, 2, 15, 0, 0, 0));
    let february_2023 = bounds_for(AlignedWindow::Months1, millis_at(2023, 2, 15, 0, 0, 0));
    let days_millis = i64::from(24 * MILLIS_PER_HOUR);

    assert_eq!(
        (february_2024.upper - february_2024.lower) / days_millis,
        29
    );
    assert_eq!(
        (february_2023.upper - february_2023.lower) / days_millis,
        28
    );
}

#[test]
fn checked_add_and_sub_use_the_window_length() {
    let timestamp = millis_at(2024, 2, 15, 12, 0, 0);
    let date_time = DateTime::from_timestamp_millis(timestamp).unwrap();

    for window in AlignedWindow::iter().filter(|window| !is_calendar_window(*window)) {
        let added = window.checked_add(date_time).unwrap();
        let subtracted = window.checked_sub(date_time).unwrap();
        let forward = added.timestamp_millis() - timestamp;
        let backward = timestamp - subtracted.timestamp_millis();

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
    for window in AlignedWindow::iter() {
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

#[test]
fn millis_exact_returns_lengths_for_fixed_windows_only() {
    assert_eq!(AlignedWindow::Seconds1.millis_exact(), Some(1_000));
    assert_eq!(AlignedWindow::Minutes5.millis_exact(), Some(300_000));
    assert_eq!(
        AlignedWindow::Hours24.millis_exact(),
        Some(u64::from(24 * MILLIS_PER_HOUR))
    );
    assert_eq!(
        AlignedWindow::Days7.millis_exact(),
        Some(u64::from(7 * MILLIS_PER_DAY))
    );
    assert_eq!(
        AlignedWindow::Weeks4.millis_exact(),
        Some(u64::from(4 * MILLIS_PER_WEEK))
    );

    for window in AlignedWindow::iter().filter(|window| is_calendar_window(*window)) {
        assert_eq!(window.millis_exact(), None);
    }
}

#[test]
fn millis_upper_falls_back_for_calendar_windows() {
    assert_eq!(
        AlignedWindow::Seconds1.millis_upper_bound(),
        AlignedWindow::Seconds1.millis_exact()
    );
    assert_eq!(
        AlignedWindow::Months1.millis_upper_bound(),
        Some(MILLIS_PER_MONTH_UPPER_BOUND)
    );
    assert_eq!(
        AlignedWindow::Years2.millis_upper_bound(),
        Some(2 * MILLIS_PER_YEAR_UPPER_BOUND)
    );
    assert_eq!(
        AlignedWindow::Months12.millis_upper_bound(),
        AlignedWindow::Years1.millis_upper_bound()
    );
    assert_eq!(
        AlignedWindow::Months12.millis_upper_bound(),
        Some(MILLIS_PER_YEAR_UPPER_BOUND)
    );
}

#[test]
fn calendar_months_returns_months_for_calendar_windows_only() {
    assert_eq!(AlignedWindow::Months1.months_exact(), Some(1));
    assert_eq!(AlignedWindow::Months2.months_exact(), Some(2));
    assert_eq!(AlignedWindow::Months3.months_exact(), Some(3));
    assert_eq!(AlignedWindow::Months4.months_exact(), Some(4));
    assert_eq!(AlignedWindow::Months6.months_exact(), Some(6));
    assert_eq!(AlignedWindow::Months12.months_exact(), Some(12));
    assert_eq!(AlignedWindow::Years1.months_exact(), Some(12));
    assert_eq!(AlignedWindow::Years2.months_exact(), Some(24));

    for window in AlignedWindow::iter().filter(|window| !is_calendar_window(*window)) {
        assert_eq!(window.months_exact(), None);
    }
}

#[test]
fn display_uses_short_labels() {
    assert_eq!(AlignedWindow::Seconds1.to_string(), "1s");
    assert_eq!(AlignedWindow::Hours24.to_string(), "24h");
    assert_eq!(AlignedWindow::Months1.to_string(), "1mo");
    assert_eq!(AlignedWindow::Years2.to_string(), "2y");
}

#[test]
fn display_round_trips_through_try_from() {
    for window in AlignedWindow::iter() {
        assert_eq!(window.label(), window.to_string());
        assert_eq!(
            AlignedWindow::try_from(window.to_string().as_str()).unwrap(),
            window
        );
    }
}

#[test]
fn serde_round_trips_short_labels() {
    for window in AlignedWindow::iter() {
        let json = serde_json::to_string(&window).unwrap();

        assert_eq!(json, format!("\"{}\"", window.label()));
        assert_eq!(
            serde_json::from_str::<AlignedWindow>(&json).unwrap(),
            window
        );
    }
}

#[test]
fn serde_rejects_names() {
    assert!(serde_json::from_str::<AlignedWindow>("\"seconds1\"").is_err());
}
