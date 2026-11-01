use crate::{
    errors::{StatsError, StockTrekError, StockTrekResult, ValueError},
    market_data::millis::*,
};
use chrono::{DateTime, Datelike, Duration, Months, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use strum::{EnumCount, EnumIter, FromRepr};

/// A window aligned to fixed or calendar boundaries.
#[derive(
    Debug,
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
#[non_exhaustive]
#[repr(u8)]
pub enum AlignedWindow {
    /// A one-second window.
    Seconds1,
    /// A five-second window.
    Seconds5,
    /// A ten-second window.
    Seconds10,
    /// A fifteen-second window.
    Seconds15,
    /// A thirty-second window.
    Seconds30,
    /// A sixty-second window.
    Seconds60,
    /// A one-minute window.
    Minutes1,
    /// A two-minute window.
    Minutes2,
    /// A three-minute window.
    Minutes3,
    /// A five-minute window.
    Minutes5,
    /// A ten-minute window.
    Minutes10,
    /// A fifteen-minute window.
    Minutes15,
    /// A thirty-minute window.
    Minutes30,
    /// A sixty-minute window.
    Minutes60,
    /// A one-hour window.
    Hours1,
    /// A two-hour window.
    Hours2,
    /// A three-hour window.
    Hours3,
    /// A four-hour window.
    Hours4,
    /// A six-hour window.
    Hours6,
    /// An eight-hour window.
    Hours8,
    /// A twelve-hour window.
    Hours12,
    /// A twenty-four-hour window.
    Hours24,
    /// A one-day window.
    Days1,
    /// A seven-day window.
    Days7,
    /// A one-week window.
    Weeks1,
    /// A two-week window.
    Weeks2,
    /// A four-week window.
    Weeks4,
    /// A one-month window.
    Months1,
    /// A two-month window.
    Months2,
    /// A three-month window.
    Months3,
    /// A four-month window.
    Months4,
    /// A six-month window.
    Months6,
    /// A twelve-month window.
    Months12,
    /// A one-year window.
    Years1,
    /// A two-year window.
    Years2,
}

/// The bounds of an aligned window, both in milliseconds.
///
/// `lower` is inclusive and `upper` is exclusive.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct AlignedTimestampBounds {
    /// The inclusive lower bound in milliseconds.
    pub lower: i64,
    /// The exclusive upper bound in milliseconds.
    pub upper: i64,
}

impl AlignedTimestampBounds {
    /// Creates bounds from an inclusive lower and exclusive upper timestamp.
    pub fn new(lower: i64, upper: i64) -> Self {
        Self { lower, upper }
    }
}

impl AlignedWindow {
    /// Returns the inclusive lower bound and exclusive upper bound of the
    /// aligned window containing `timestamp`, both in milliseconds.
    ///
    /// Month and year windows follow calendar boundaries, so their lengths vary
    /// with the actual number of days in each month or year.
    pub fn bounds_for(&self, timestamp: i64) -> StockTrekResult<AlignedTimestampBounds> {
        if let Some(exact_millis) = self.millis_exact() {
            self.fixed_bounds(timestamp, exact_millis)
        } else if let Some(exact_months) = self.months_exact() {
            self.calendar_bounds(timestamp, exact_months)
        } else {
            Err(Self::bounds_error(format!(
                "Could not calculate bounds for {self} at timestamp {timestamp}"
            )))
        }
    }

    fn fixed_bounds(
        &self,
        timestamp: i64,
        exact_millis: u32,
    ) -> StockTrekResult<AlignedTimestampBounds> {
        let window_millis_wide = i64::from(exact_millis);
        let lower_millis = timestamp
            .div_euclid(window_millis_wide)
            .checked_mul(window_millis_wide)
            .ok_or_else(|| {
                Self::bounds_error(format!(
                    "lower bound for timestamp {timestamp} could not be represented"
                ))
            })?;
        let upper_millis = lower_millis
            .checked_add(window_millis_wide)
            .ok_or_else(|| {
                Self::bounds_error(format!(
                    "upper bound for timestamp {timestamp} could not be represented"
                ))
            })?;
        Ok(AlignedTimestampBounds::new(lower_millis, upper_millis))
    }

    fn calendar_bounds(
        &self,
        timestamp: i64,
        exact_months: u32,
    ) -> StockTrekResult<AlignedTimestampBounds> {
        let date_time = DateTime::from_timestamp_millis(timestamp).ok_or_else(|| {
            Self::bounds_error(format!(
                "timestamp {timestamp} is not representable as a UTC date"
            ))
        })?;
        let months_since_epoch = date_time.year() * 12 + date_time.month0() as i32;
        let months_per_window = exact_months as i32;
        let window_start_months =
            months_since_epoch.div_euclid(months_per_window) * months_per_window;
        let start_year = window_start_months.div_euclid(12);
        let start_month = window_start_months.rem_euclid(12) as u32 + 1;
        let lower = Utc
            .with_ymd_and_hms(start_year, start_month, 1, 0, 0, 0)
            .single()
            .ok_or_else(|| {
                Self::bounds_error(format!(
                    "calendar window lower bound for {start_year}-{start_month:02} could not be represented"
                ))
            })?;
        let upper = lower.checked_add_months(Months::new(exact_months)).ok_or_else(|| {
            Self::bounds_error(format!(
                "calendar window upper bound for {start_year}-{start_month:02} could not be represented"
            ))
        })?;
        Ok(AlignedTimestampBounds {
            lower: lower.timestamp_millis(),
            upper: upper.timestamp_millis(),
        })
    }

    fn bounds_error(message: String) -> StockTrekError {
        StockTrekError::Stats(StatsError::DomainError {
            function: "AlignedWindow::bounds_for",
            message,
        })
    }

    /// Returns the exact window length in milliseconds for fixed-length windows.
    pub fn millis_exact(&self) -> Option<u32> {
        match self {
            AlignedWindow::Seconds1 => Some(MILLIS_PER_SECOND),
            AlignedWindow::Seconds5 => Some(5 * MILLIS_PER_SECOND),
            AlignedWindow::Seconds10 => Some(10 * MILLIS_PER_SECOND),
            AlignedWindow::Seconds15 => Some(15 * MILLIS_PER_SECOND),
            AlignedWindow::Seconds30 => Some(30 * MILLIS_PER_SECOND),
            AlignedWindow::Seconds60 => Some(60 * MILLIS_PER_SECOND),
            AlignedWindow::Minutes1 => Some(MILLIS_PER_MINUTE),
            AlignedWindow::Minutes2 => Some(2 * MILLIS_PER_MINUTE),
            AlignedWindow::Minutes3 => Some(3 * MILLIS_PER_MINUTE),
            AlignedWindow::Minutes5 => Some(5 * MILLIS_PER_MINUTE),
            AlignedWindow::Minutes10 => Some(10 * MILLIS_PER_MINUTE),
            AlignedWindow::Minutes15 => Some(15 * MILLIS_PER_MINUTE),
            AlignedWindow::Minutes30 => Some(30 * MILLIS_PER_MINUTE),
            AlignedWindow::Minutes60 => Some(60 * MILLIS_PER_MINUTE),
            AlignedWindow::Hours1 => Some(MILLIS_PER_HOUR),
            AlignedWindow::Hours2 => Some(2 * MILLIS_PER_HOUR),
            AlignedWindow::Hours3 => Some(3 * MILLIS_PER_HOUR),
            AlignedWindow::Hours4 => Some(4 * MILLIS_PER_HOUR),
            AlignedWindow::Hours6 => Some(6 * MILLIS_PER_HOUR),
            AlignedWindow::Hours8 => Some(8 * MILLIS_PER_HOUR),
            AlignedWindow::Hours12 => Some(12 * MILLIS_PER_HOUR),
            AlignedWindow::Hours24 => Some(24 * MILLIS_PER_HOUR),
            AlignedWindow::Days1 => Some(MILLIS_PER_DAY),
            AlignedWindow::Days7 => Some(7 * MILLIS_PER_DAY),
            AlignedWindow::Weeks1 => Some(MILLIS_PER_WEEK),
            AlignedWindow::Weeks2 => Some(2 * MILLIS_PER_WEEK),
            AlignedWindow::Weeks4 => Some(4 * MILLIS_PER_WEEK),
            AlignedWindow::Months1
            | AlignedWindow::Months2
            | AlignedWindow::Months3
            | AlignedWindow::Months4
            | AlignedWindow::Months6
            | AlignedWindow::Months12
            | AlignedWindow::Years1
            | AlignedWindow::Years2 => None,
        }
    }

    /// Returns an upper bound on the window length in milliseconds.
    pub fn millis_upper_bound(&self) -> u64 {
        if let Some(millis) = self.millis_exact() {
            return millis as u64;
        }

        match self {
            AlignedWindow::Months1 => MILLIS_PER_MONTH_UPPER_BOUND,
            AlignedWindow::Months2 => 2 * MILLIS_PER_MONTH_UPPER_BOUND,
            AlignedWindow::Months3 => 3 * MILLIS_PER_MONTH_UPPER_BOUND,
            AlignedWindow::Months4 => 4 * MILLIS_PER_MONTH_UPPER_BOUND,
            AlignedWindow::Months6 => 6 * MILLIS_PER_MONTH_UPPER_BOUND,
            AlignedWindow::Months12 => 12 * MILLIS_PER_MONTH_UPPER_BOUND,
            AlignedWindow::Years1 => MILLIS_PER_YEAR_UPPER_BOUND,
            AlignedWindow::Years2 => 2 * MILLIS_PER_YEAR_UPPER_BOUND,
            _ => {
                unreachable!("only calendar based windows lack an exact millisecond length")
            }
        }
    }

    /// Returns the exact number of months for calendar-based windows.
    pub fn months_exact(&self) -> Option<u32> {
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

    /// Adds the window length to a UTC date and time.
    pub fn checked_add(&self, date_time: DateTime<Utc>) -> Option<DateTime<Utc>> {
        if let Some(millis) = self.millis_exact() {
            date_time.checked_add_signed(Duration::milliseconds(i64::from(millis)))
        } else {
            date_time.checked_add_months(Months::new(self.months_exact()?))
        }
    }

    /// Subtracts the window length from a UTC date and time.
    pub fn checked_sub(&self, date_time: DateTime<Utc>) -> Option<DateTime<Utc>> {
        if let Some(millis) = self.millis_exact() {
            date_time.checked_sub_signed(Duration::milliseconds(i64::from(millis)))
        } else {
            date_time.checked_sub_months(Months::new(self.months_exact()?))
        }
    }

    /// Returns the short label for the window, such as `1h` or `7d`.
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

impl std::fmt::Display for AlignedWindow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.label())
    }
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
                    let window_millis = i64::try_from(window.millis_upper_bound()).unwrap();
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
                i64::try_from(window.millis_upper_bound()).unwrap(),
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
            Some(24 * MILLIS_PER_HOUR)
        );
        assert_eq!(
            AlignedWindow::Days7.millis_exact(),
            Some(7 * MILLIS_PER_DAY)
        );
        assert_eq!(
            AlignedWindow::Weeks4.millis_exact(),
            Some(4 * MILLIS_PER_WEEK)
        );

        for window in AlignedWindow::iter().filter(|window| is_calendar_window(*window)) {
            assert_eq!(window.millis_exact(), None);
        }
    }

    #[test]
    fn millis_upper_falls_back_for_calendar_windows() {
        assert_eq!(
            AlignedWindow::Seconds1.millis_upper_bound(),
            AlignedWindow::Seconds1.millis_exact().unwrap() as u64
        );
        assert_eq!(
            AlignedWindow::Months1.millis_upper_bound(),
            MILLIS_PER_MONTH_UPPER_BOUND
        );
        assert_eq!(
            AlignedWindow::Years2.millis_upper_bound(),
            2 * MILLIS_PER_YEAR_UPPER_BOUND
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
}
