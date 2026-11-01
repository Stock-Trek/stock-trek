use crate::{
    errors::{StatsError, StockTrekError, StockTrekResult, ValueError},
    market_data::millis::*,
};
use chrono::{DateTime, Datelike, Duration, Months, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumIter, FromRepr};

#[derive(
    Debug,
    Display,
    Clone,
    Copy,
    PartialOrd,
    Ord,
    PartialEq,
    Eq,
    Hash,
    EnumCount,
    EnumIter,
    FromRepr,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
#[non_exhaustive]
#[repr(u8)]
pub enum AlignedWindow {
    Seconds1,
    Seconds5,
    Seconds10,
    Seconds15,
    Seconds30,
    Seconds60,
    Minutes1,
    Minutes2,
    Minutes3,
    Minutes5,
    Minutes10,
    Minutes15,
    Minutes30,
    Minutes60,
    Hours1,
    Hours2,
    Hours3,
    Hours4,
    Hours6,
    Hours8,
    Hours12,
    Hours24,
    Days1,
    Days7,
    Weeks1,
    Weeks2,
    Weeks4,
    Months1,
    Months2,
    Months3,
    Months4,
    Months6,
    Months12,
    Years1,
    Years2,
}

impl AlignedWindow {
    /// Returns the inclusive lower bound and exclusive upper bound of the
    /// aligned window containing `timestamp`, both in milliseconds.
    ///
    /// Month and year windows follow calendar boundaries, so their lengths vary
    /// with the actual number of days in each month or year.
    pub fn bounds_for(&self, timestamp: i64) -> StockTrekResult<(i64, i64)> {
        if let Some(months_per_window) = self.calendar_months() {
            return calendar_bounds_for(timestamp, months_per_window);
        }

        let window_millis = self.millis_upper_bound();
        let window_millis_wide = i64::try_from(window_millis).map_err(|_| {
            bounds_error(format!(
                "window size {window_millis} does not fit in an i64"
            ))
        })?;
        let lower_bound_millis = timestamp
            .div_euclid(window_millis_wide)
            .checked_mul(window_millis_wide)
            .ok_or_else(|| {
                bounds_error(format!(
                    "lower bound for timestamp {timestamp} could not be represented"
                ))
            })?;
        let upper_bound_millis = lower_bound_millis
            .checked_add(window_millis_wide)
            .ok_or_else(|| {
                bounds_error(format!(
                    "upper bound for timestamp {timestamp} could not be represented"
                ))
            })?;

        Ok((lower_bound_millis, upper_bound_millis))
    }
    fn calendar_months(&self) -> Option<u32> {
        match self {
            AlignedWindow::Months1 => Some(1),
            AlignedWindow::Months2 => Some(2),
            AlignedWindow::Months3 => Some(3),
            AlignedWindow::Months4 => Some(4),
            AlignedWindow::Months6 => Some(6),
            AlignedWindow::Months12 | AlignedWindow::Years1 => Some(12),
            AlignedWindow::Years2 => Some(24),
            _ => None,
        }
    }
    pub fn millis_upper_bound(&self) -> u64 {
        match self {
            AlignedWindow::Seconds1 => MILLIS_PER_SECOND,
            AlignedWindow::Seconds5 => 5 * MILLIS_PER_SECOND,
            AlignedWindow::Seconds10 => 10 * MILLIS_PER_SECOND,
            AlignedWindow::Seconds15 => 15 * MILLIS_PER_SECOND,
            AlignedWindow::Seconds30 => 30 * MILLIS_PER_SECOND,
            AlignedWindow::Seconds60 => 60 * MILLIS_PER_SECOND,
            AlignedWindow::Minutes1 => MILLIS_PER_MINUTE,
            AlignedWindow::Minutes2 => 2 * MILLIS_PER_MINUTE,
            AlignedWindow::Minutes3 => 3 * MILLIS_PER_MINUTE,
            AlignedWindow::Minutes5 => 5 * MILLIS_PER_MINUTE,
            AlignedWindow::Minutes10 => 10 * MILLIS_PER_MINUTE,
            AlignedWindow::Minutes15 => 15 * MILLIS_PER_MINUTE,
            AlignedWindow::Minutes30 => 30 * MILLIS_PER_MINUTE,
            AlignedWindow::Minutes60 => 60 * MILLIS_PER_MINUTE,
            AlignedWindow::Hours1 => MILLIS_PER_HOUR,
            AlignedWindow::Hours2 => 2 * MILLIS_PER_HOUR,
            AlignedWindow::Hours3 => 3 * MILLIS_PER_HOUR,
            AlignedWindow::Hours4 => 4 * MILLIS_PER_HOUR,
            AlignedWindow::Hours6 => 6 * MILLIS_PER_HOUR,
            AlignedWindow::Hours8 => 8 * MILLIS_PER_HOUR,
            AlignedWindow::Hours12 => 12 * MILLIS_PER_HOUR,
            AlignedWindow::Hours24 => 24 * MILLIS_PER_HOUR,
            AlignedWindow::Days1 => MILLIS_PER_DAY,
            AlignedWindow::Days7 => 7 * MILLIS_PER_DAY,
            AlignedWindow::Weeks1 => MILLIS_PER_WEEK,
            AlignedWindow::Weeks2 => 2 * MILLIS_PER_WEEK,
            AlignedWindow::Weeks4 => 4 * MILLIS_PER_WEEK,
            AlignedWindow::Months1 => MILLIS_PER_MONTH_UPPER_BOUND,
            AlignedWindow::Months2 => 2 * MILLIS_PER_MONTH_UPPER_BOUND,
            AlignedWindow::Months3 => 3 * MILLIS_PER_MONTH_UPPER_BOUND,
            AlignedWindow::Months4 => 4 * MILLIS_PER_MONTH_UPPER_BOUND,
            AlignedWindow::Months6 => 6 * MILLIS_PER_MONTH_UPPER_BOUND,
            AlignedWindow::Months12 => 12 * MILLIS_PER_MONTH_UPPER_BOUND,
            AlignedWindow::Years1 => MILLIS_PER_YEAR_UPPER_BOUND,
            AlignedWindow::Years2 => 2 * MILLIS_PER_YEAR_UPPER_BOUND,
        }
    }
    pub fn checked_add(&self, date_time: DateTime<Utc>) -> Option<DateTime<Utc>> {
        match self {
            AlignedWindow::Seconds1 => Some(date_time + Duration::seconds(1)),
            AlignedWindow::Seconds5 => Some(date_time + Duration::seconds(5)),
            AlignedWindow::Seconds10 => Some(date_time + Duration::seconds(10)),
            AlignedWindow::Seconds15 => Some(date_time + Duration::seconds(15)),
            AlignedWindow::Seconds30 => Some(date_time + Duration::seconds(30)),
            AlignedWindow::Seconds60 => Some(date_time + Duration::seconds(60)),
            AlignedWindow::Minutes1 => Some(date_time + Duration::minutes(1)),
            AlignedWindow::Minutes2 => Some(date_time + Duration::minutes(2)),
            AlignedWindow::Minutes3 => Some(date_time + Duration::minutes(3)),
            AlignedWindow::Minutes5 => Some(date_time + Duration::minutes(5)),
            AlignedWindow::Minutes10 => Some(date_time + Duration::minutes(10)),
            AlignedWindow::Minutes15 => Some(date_time + Duration::minutes(15)),
            AlignedWindow::Minutes30 => Some(date_time + Duration::minutes(30)),
            AlignedWindow::Minutes60 => Some(date_time + Duration::minutes(60)),
            AlignedWindow::Hours1 => Some(date_time + Duration::hours(5)),
            AlignedWindow::Hours2 => Some(date_time + Duration::hours(2)),
            AlignedWindow::Hours3 => Some(date_time + Duration::hours(3)),
            AlignedWindow::Hours4 => Some(date_time + Duration::hours(4)),
            AlignedWindow::Hours6 => Some(date_time + Duration::hours(6)),
            AlignedWindow::Hours8 => Some(date_time + Duration::hours(8)),
            AlignedWindow::Hours12 => Some(date_time + Duration::hours(12)),
            AlignedWindow::Hours24 => Some(date_time + Duration::hours(24)),
            AlignedWindow::Days1 => Some(date_time + Duration::days(1)),
            AlignedWindow::Days7 => Some(date_time + Duration::days(7)),
            AlignedWindow::Weeks1 => Some(date_time + Duration::weeks(1)),
            AlignedWindow::Weeks2 => Some(date_time + Duration::weeks(2)),
            AlignedWindow::Weeks4 => Some(date_time + Duration::weeks(4)),
            AlignedWindow::Months1 => date_time.checked_add_months(Months::new(1)),
            AlignedWindow::Months2 => date_time.checked_add_months(Months::new(2)),
            AlignedWindow::Months3 => date_time.checked_add_months(Months::new(3)),
            AlignedWindow::Months4 => date_time.checked_add_months(Months::new(4)),
            AlignedWindow::Months6 => date_time.checked_add_months(Months::new(6)),
            AlignedWindow::Months12 => date_time.checked_add_months(Months::new(12)),
            AlignedWindow::Years1 => date_time.checked_add_months(Months::new(12)),
            AlignedWindow::Years2 => date_time.checked_add_months(Months::new(24)),
        }
    }
    pub fn checked_sub(&self, date_time: DateTime<Utc>) -> Option<DateTime<Utc>> {
        match self {
            AlignedWindow::Seconds1 => Some(date_time - Duration::seconds(1)),
            AlignedWindow::Seconds5 => Some(date_time - Duration::seconds(5)),
            AlignedWindow::Seconds10 => Some(date_time - Duration::seconds(10)),
            AlignedWindow::Seconds15 => Some(date_time - Duration::seconds(15)),
            AlignedWindow::Seconds30 => Some(date_time - Duration::seconds(30)),
            AlignedWindow::Seconds60 => Some(date_time - Duration::seconds(60)),
            AlignedWindow::Minutes1 => Some(date_time - Duration::minutes(1)),
            AlignedWindow::Minutes2 => Some(date_time - Duration::minutes(2)),
            AlignedWindow::Minutes3 => Some(date_time - Duration::minutes(3)),
            AlignedWindow::Minutes5 => Some(date_time - Duration::minutes(5)),
            AlignedWindow::Minutes10 => Some(date_time - Duration::minutes(10)),
            AlignedWindow::Minutes15 => Some(date_time - Duration::minutes(15)),
            AlignedWindow::Minutes30 => Some(date_time - Duration::minutes(30)),
            AlignedWindow::Minutes60 => Some(date_time - Duration::minutes(60)),
            AlignedWindow::Hours1 => Some(date_time - Duration::hours(5)),
            AlignedWindow::Hours2 => Some(date_time - Duration::hours(2)),
            AlignedWindow::Hours3 => Some(date_time - Duration::hours(3)),
            AlignedWindow::Hours4 => Some(date_time - Duration::hours(4)),
            AlignedWindow::Hours6 => Some(date_time - Duration::hours(6)),
            AlignedWindow::Hours8 => Some(date_time - Duration::hours(8)),
            AlignedWindow::Hours12 => Some(date_time - Duration::hours(12)),
            AlignedWindow::Hours24 => Some(date_time - Duration::hours(24)),
            AlignedWindow::Days1 => Some(date_time - Duration::days(1)),
            AlignedWindow::Days7 => Some(date_time - Duration::days(7)),
            AlignedWindow::Weeks1 => Some(date_time - Duration::weeks(1)),
            AlignedWindow::Weeks2 => Some(date_time - Duration::weeks(2)),
            AlignedWindow::Weeks4 => Some(date_time - Duration::weeks(4)),
            AlignedWindow::Months1 => date_time.checked_sub_months(Months::new(1)),
            AlignedWindow::Months2 => date_time.checked_sub_months(Months::new(2)),
            AlignedWindow::Months3 => date_time.checked_sub_months(Months::new(3)),
            AlignedWindow::Months4 => date_time.checked_sub_months(Months::new(4)),
            AlignedWindow::Months6 => date_time.checked_sub_months(Months::new(6)),
            AlignedWindow::Months12 => date_time.checked_sub_months(Months::new(12)),
            AlignedWindow::Years1 => date_time.checked_sub_months(Months::new(12)),
            AlignedWindow::Years2 => date_time.checked_sub_months(Months::new(24)),
        }
    }
    pub fn label(self) -> String {
        match self {
            AlignedWindow::Seconds1 => "1s".to_string(),
            AlignedWindow::Seconds5 => "5s".to_string(),
            AlignedWindow::Seconds10 => "10s".to_string(),
            AlignedWindow::Seconds15 => "15s".to_string(),
            AlignedWindow::Seconds30 => "30s".to_string(),
            AlignedWindow::Seconds60 => "60s".to_string(),
            AlignedWindow::Minutes1 => "1m".to_string(),
            AlignedWindow::Minutes2 => "2m".to_string(),
            AlignedWindow::Minutes3 => "3m".to_string(),
            AlignedWindow::Minutes5 => "5m".to_string(),
            AlignedWindow::Minutes10 => "10m".to_string(),
            AlignedWindow::Minutes15 => "15m".to_string(),
            AlignedWindow::Minutes30 => "30m".to_string(),
            AlignedWindow::Minutes60 => "60m".to_string(),
            AlignedWindow::Hours1 => "1h".to_string(),
            AlignedWindow::Hours2 => "2h".to_string(),
            AlignedWindow::Hours3 => "3h".to_string(),
            AlignedWindow::Hours4 => "4h".to_string(),
            AlignedWindow::Hours6 => "6h".to_string(),
            AlignedWindow::Hours8 => "8h".to_string(),
            AlignedWindow::Hours12 => "12h".to_string(),
            AlignedWindow::Hours24 => "24h".to_string(),
            AlignedWindow::Days1 => "1d".to_string(),
            AlignedWindow::Days7 => "7d".to_string(),
            AlignedWindow::Weeks1 => "1w".to_string(),
            AlignedWindow::Weeks2 => "2w".to_string(),
            AlignedWindow::Weeks4 => "4w".to_string(),
            AlignedWindow::Months1 => "1mo".to_string(),
            AlignedWindow::Months2 => "2mo".to_string(),
            AlignedWindow::Months3 => "3mo".to_string(),
            AlignedWindow::Months4 => "4mo".to_string(),
            AlignedWindow::Months6 => "6mo".to_string(),
            AlignedWindow::Months12 => "12mo".to_string(),
            AlignedWindow::Years1 => "1y".to_string(),
            AlignedWindow::Years2 => "2y".to_string(),
        }
    }
}

fn bounds_error(message: String) -> StockTrekError {
    StockTrekError::Stats(StatsError::DomainError {
        function: "AlignedWindow::bounds_for",
        message,
    })
}

fn calendar_bounds_for(timestamp: i64, months_per_window: u32) -> StockTrekResult<(i64, i64)> {
    let date_time = DateTime::from_timestamp_millis(timestamp).ok_or_else(|| {
        bounds_error(format!(
            "timestamp {timestamp} is not representable as a UTC date"
        ))
    })?;
    let months_since_epoch = i64::from(date_time.year()) * 12 + i64::from(date_time.month0());
    let months_per_window_wide = i64::from(months_per_window);
    let window_start_months =
        months_since_epoch.div_euclid(months_per_window_wide) * months_per_window_wide;
    let start_year = i32::try_from(window_start_months.div_euclid(12)).map_err(|_| {
        bounds_error(format!(
            "calendar window year {} does not fit in an i32",
            window_start_months.div_euclid(12)
        ))
    })?;
    let start_month = u32::try_from(window_start_months.rem_euclid(12) + 1).map_err(|_| {
        bounds_error(format!(
            "calendar window month {} does not fit in a u32",
            window_start_months.rem_euclid(12) + 1
        ))
    })?;
    let lower_bound = Utc
        .with_ymd_and_hms(start_year, start_month, 1, 0, 0, 0)
        .single()
        .ok_or_else(|| {
            bounds_error(format!(
                "calendar window start {start_year}-{start_month:02} could not be represented"
            ))
        })?;
    let upper_bound = lower_bound
        .checked_add_months(Months::new(months_per_window))
        .ok_or_else(|| {
            bounds_error(format!(
                "calendar window end after {start_year}-{start_month:02} could not be represented"
            ))
        })?;

    Ok((
        lower_bound.timestamp_millis(),
        upper_bound.timestamp_millis(),
    ))
}

impl TryFrom<&str> for AlignedWindow {
    type Error = StockTrekError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "1s" => Ok(AlignedWindow::Seconds1),
            "5s" => Ok(AlignedWindow::Seconds5),
            "10s" => Ok(AlignedWindow::Seconds10),
            "15s" => Ok(AlignedWindow::Seconds15),
            "30s" => Ok(AlignedWindow::Seconds30),
            "60s" => Ok(AlignedWindow::Seconds60),
            "1m" => Ok(AlignedWindow::Minutes1),
            "2m" => Ok(AlignedWindow::Minutes2),
            "3m" => Ok(AlignedWindow::Minutes3),
            "5m" => Ok(AlignedWindow::Minutes5),
            "10m" => Ok(AlignedWindow::Minutes10),
            "15m" => Ok(AlignedWindow::Minutes15),
            "30m" => Ok(AlignedWindow::Minutes30),
            "60m" => Ok(AlignedWindow::Minutes60),
            "1h" => Ok(AlignedWindow::Hours1),
            "2h" => Ok(AlignedWindow::Hours2),
            "3h" => Ok(AlignedWindow::Hours3),
            "4h" => Ok(AlignedWindow::Hours4),
            "6h" => Ok(AlignedWindow::Hours6),
            "8h" => Ok(AlignedWindow::Hours8),
            "12h" => Ok(AlignedWindow::Hours12),
            "24h" => Ok(AlignedWindow::Hours24),
            "1d" => Ok(AlignedWindow::Days1),
            "7d" => Ok(AlignedWindow::Days7),
            "1w" => Ok(AlignedWindow::Weeks1),
            "2w" => Ok(AlignedWindow::Weeks2),
            "4w" => Ok(AlignedWindow::Weeks4),
            "1mo" => Ok(AlignedWindow::Months1),
            "2mo" => Ok(AlignedWindow::Months2),
            "3mo" => Ok(AlignedWindow::Months3),
            "4mo" => Ok(AlignedWindow::Months4),
            "6mo" => Ok(AlignedWindow::Months6),
            "12mo" => Ok(AlignedWindow::Months12),
            "1y" => Ok(AlignedWindow::Years1),
            "2y" => Ok(AlignedWindow::Years2),
            _ => Err(StockTrekError::Value(ValueError::NotFound {
                name: "AlignedWindow".into(),
                key: value.into(),
            })),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    fn millis_at(year: i32, month: u32, day: u32, hour: u32, minute: u32, second: u32) -> i64 {
        Utc.with_ymd_and_hms(year, month, day, hour, minute, second)
            .unwrap()
            .timestamp_millis()
    }

    fn bounds_for(window: AlignedWindow, timestamp: i64) -> (i64, i64) {
        window.bounds_for(timestamp).unwrap()
    }

    #[test]
    fn bounds_for_returns_containing_bounds() {
        let timestamps: [i64; 7] = [-1_700_000_000_123, -1, 0, 1, 999, 1_000, 1_700_000_000_123];

        for window in AlignedWindow::iter() {
            for timestamp in timestamps {
                let (lower_bound, upper_bound) = bounds_for(window, timestamp);

                assert!(lower_bound <= timestamp);
                assert!(timestamp < upper_bound);

                if window.calendar_months().is_none() {
                    let window_millis = i64::try_from(window.millis_upper_bound()).unwrap();
                    assert_eq!(upper_bound - lower_bound, window_millis);
                    assert_eq!(lower_bound % window_millis, 0);
                }
            }
        }
    }

    #[test]
    fn bounds_for_handles_pre_epoch_timestamps() {
        assert_eq!(bounds_for(AlignedWindow::Seconds1, -1), (-1_000, 0));
        assert_eq!(
            bounds_for(AlignedWindow::Months1, millis_at(1969, 12, 15, 12, 0, 0)),
            (
                millis_at(1969, 12, 1, 0, 0, 0),
                millis_at(1970, 1, 1, 0, 0, 0)
            )
        );
    }

    #[test]
    fn bounds_for_aligns_seconds_and_minutes() {
        assert_eq!(bounds_for(AlignedWindow::Seconds1, 1_500), (1_000, 2_000));
        assert_eq!(
            bounds_for(AlignedWindow::Seconds5, 12_345),
            (10_000, 15_000)
        );
        assert_eq!(
            bounds_for(AlignedWindow::Minutes1, 61_234),
            (60_000, 120_000)
        );
        assert_eq!(
            bounds_for(AlignedWindow::Minutes5, 360_123),
            (300_000, 600_000)
        );
    }

    #[test]
    fn bounds_for_month_windows_follow_calendar_boundaries() {
        let timestamp = millis_at(2024, 2, 15, 12, 34, 56);

        assert_eq!(
            bounds_for(AlignedWindow::Months1, timestamp),
            (
                millis_at(2024, 2, 1, 0, 0, 0),
                millis_at(2024, 3, 1, 0, 0, 0)
            )
        );
        assert_eq!(
            bounds_for(AlignedWindow::Months2, timestamp),
            (
                millis_at(2024, 1, 1, 0, 0, 0),
                millis_at(2024, 3, 1, 0, 0, 0)
            )
        );
        assert_eq!(
            bounds_for(AlignedWindow::Months3, timestamp),
            (
                millis_at(2024, 1, 1, 0, 0, 0),
                millis_at(2024, 4, 1, 0, 0, 0)
            )
        );
        assert_eq!(
            bounds_for(AlignedWindow::Months4, timestamp),
            (
                millis_at(2024, 1, 1, 0, 0, 0),
                millis_at(2024, 5, 1, 0, 0, 0)
            )
        );
        assert_eq!(
            bounds_for(AlignedWindow::Months6, timestamp),
            (
                millis_at(2024, 1, 1, 0, 0, 0),
                millis_at(2024, 7, 1, 0, 0, 0)
            )
        );
        assert_eq!(
            bounds_for(AlignedWindow::Months12, timestamp),
            (
                millis_at(2024, 1, 1, 0, 0, 0),
                millis_at(2025, 1, 1, 0, 0, 0)
            )
        );
        assert_eq!(
            bounds_for(AlignedWindow::Years1, timestamp),
            (
                millis_at(2024, 1, 1, 0, 0, 0),
                millis_at(2025, 1, 1, 0, 0, 0)
            )
        );
        assert_eq!(
            bounds_for(AlignedWindow::Years2, timestamp),
            (
                millis_at(2024, 1, 1, 0, 0, 0),
                millis_at(2026, 1, 1, 0, 0, 0)
            )
        );
    }

    #[test]
    fn bounds_for_month_windows_include_window_boundaries() {
        let last_millis_of_february = millis_at(2024, 2, 29, 23, 59, 59) + 999;
        let expected = (
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
            (
                millis_at(2024, 3, 1, 0, 0, 0),
                millis_at(2024, 4, 1, 0, 0, 0)
            )
        );
    }

    #[test]
    fn bounds_for_month_windows_handle_leap_years() {
        let february_2024 = bounds_for(AlignedWindow::Months1, millis_at(2024, 2, 15, 0, 0, 0));
        let february_2023 = bounds_for(AlignedWindow::Months1, millis_at(2023, 2, 15, 0, 0, 0));
        let days_millis = i64::try_from(24 * MILLIS_PER_HOUR).unwrap();

        assert_eq!((february_2024.1 - february_2024.0) / days_millis, 29);
        assert_eq!((february_2023.1 - february_2023.0) / days_millis, 28);
    }
}
