use super::*;
use crate::market_data::window_test_support::{CheckedWindow, window_tests};
use strum::IntoEnumIterator;

fn millis_at(
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
) -> TimestampMillis {
    TimestampMillis::try_from(
        Utc.with_ymd_and_hms(year, month, day, hour, minute, second)
            .unwrap()
            .timestamp_millis(),
    )
    .unwrap()
}

fn bounds_for(
    window: AlignedWindow,
    timestamp: impl Into<TimestampMillis>,
) -> AlignedTimestampBounds {
    window.bounds_for(timestamp.into()).unwrap()
}

fn timestamps(
    lower: impl Into<TimestampMillis>,
    upper: impl Into<TimestampMillis>,
) -> AlignedTimestampBounds {
    AlignedTimestampBounds::new(lower.into(), upper.into())
}

fn is_calendar_window(window: AlignedWindow) -> bool {
    matches!(
        window,
        AlignedWindow::Months1
            | AlignedWindow::Months2
            | AlignedWindow::Months3
            | AlignedWindow::Months4
            | AlignedWindow::Months6
            | AlignedWindow::Years1
            | AlignedWindow::Years2
    )
}

#[test]
fn bounds_for_returns_containing_bounds() {
    let timestamps: [TimestampMillis; 5] = [
        TimestampMillis::new(0),
        TimestampMillis::new(1),
        TimestampMillis::new(999),
        TimestampMillis::new(1_000),
        TimestampMillis::new(1_700_000_000_123),
    ];

    for window in AlignedWindow::iter() {
        for timestamp in timestamps {
            let AlignedTimestampBounds { lower, upper } = bounds_for(window, timestamp);

            assert!(lower <= timestamp);
            assert!(timestamp < upper);

            if !is_calendar_window(window) {
                let window_millis = window.millis_upper_bound().unwrap();
                assert_eq!((upper - lower).as_millis(), window_millis);
                assert_eq!(lower.as_millis() % window_millis, 0);
            }
        }
    }
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
    let last_millis_of_february = millis_at(2024, 2, 29, 23, 59, 59)
        .checked_add(TimestampMillis::new(999))
        .unwrap();
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
    let days_millis = u64::from(24 * MILLIS_PER_HOUR);

    assert_eq!(
        (february_2024.upper - february_2024.lower).as_millis() / days_millis,
        29
    );
    assert_eq!(
        (february_2023.upper - february_2023.lower).as_millis() / days_millis,
        28
    );
}

#[test]
fn millis_exact_returns_lengths_for_fixed_windows_only() {
    assert_eq!(AlignedWindow::Seconds1.millis_exact(), Some(1_000));
    assert_eq!(AlignedWindow::Minutes5.millis_exact(), Some(300_000));
    assert_eq!(
        AlignedWindow::Days1.millis_exact(),
        Some(u64::from(24 * MILLIS_PER_HOUR))
    );
    assert_eq!(
        AlignedWindow::Weeks1.millis_exact(),
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
        AlignedWindow::Years1.millis_upper_bound(),
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
    assert_eq!(AlignedWindow::Years1.months_exact(), Some(12));
    assert_eq!(AlignedWindow::Years2.months_exact(), Some(24));

    for window in AlignedWindow::iter().filter(|window| !is_calendar_window(*window)) {
        assert_eq!(window.months_exact(), None);
    }
}

window_tests!(
    AlignedWindow,
    AlignedWindow::Seconds1 => "1s",
    AlignedWindow::Days1 => "1d",
    AlignedWindow::Months1 => "1mo",
    AlignedWindow::Years2 => "2y",
);
