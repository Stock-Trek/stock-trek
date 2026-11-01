use crate::{
    errors::{StatsError, StockTrekError, StockTrekResult, ValueError},
    market_data::{millis::*, timestamp::TimestampMillis},
};
use chrono::{DateTime, Datelike, Months, TimeZone, Utc};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use strum::{EnumCount, EnumIter, FromRepr};

/// A window aligned to fixed or calendar boundaries.
#[derive(
    Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash, EnumCount, EnumIter, FromRepr,
)]
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

/// The bounds of an aligned window, both as [`TimestampMillis`].
///
/// `lower` is inclusive and `upper` is exclusive.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct AlignedTimestampBounds {
    /// The inclusive lower bound.
    pub lower: TimestampMillis,
    /// The exclusive upper bound.
    pub upper: TimestampMillis,
}

impl AlignedTimestampBounds {
    /// Creates bounds from an inclusive lower and exclusive upper timestamp.
    pub fn new(lower: TimestampMillis, upper: TimestampMillis) -> Self {
        Self { lower, upper }
    }
}

impl AlignedWindow {
    /// Returns the inclusive lower bound and exclusive upper bound of the
    /// aligned window containing `timestamp`.
    ///
    /// Month and year windows follow calendar boundaries, so their lengths vary
    /// with the actual number of days in each month or year.
    pub fn bounds_for(
        &self,
        timestamp: TimestampMillis,
    ) -> StockTrekResult<AlignedTimestampBounds> {
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
        timestamp: TimestampMillis,
        exact_millis: u64,
    ) -> StockTrekResult<AlignedTimestampBounds> {
        let lower_millis = timestamp.div_euclid(exact_millis) * exact_millis;
        let upper_millis = lower_millis.checked_add(exact_millis).ok_or_else(|| {
            Self::bounds_error(format!(
                "upper bound for timestamp {timestamp} could not be represented"
            ))
        })?;
        Ok(AlignedTimestampBounds::new(lower_millis, upper_millis))
    }

    fn calendar_bounds(
        &self,
        timestamp: TimestampMillis,
        exact_months: u32,
    ) -> StockTrekResult<AlignedTimestampBounds> {
        let timestamp_i64 = i64::try_from(timestamp).map_err(|_| {
            Self::bounds_error(format!(
                "timestamp {timestamp} is not representable as a UTC date"
            ))
        })?;
        let date_time = DateTime::from_timestamp_millis(timestamp_i64).ok_or_else(|| {
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
        let lower = TimestampMillis::try_from(lower.timestamp_millis()).map_err(|_| {
            Self::bounds_error(format!(
                "calendar window lower bound for {start_year}-{start_month:02} could not be represented as a timestamp"
            ))
        })?;
        let upper = TimestampMillis::try_from(upper.timestamp_millis()).map_err(|_| {
            Self::bounds_error(format!(
                "calendar window upper bound for {start_year}-{start_month:02} could not be represented as a timestamp"
            ))
        })?;
        Ok(AlignedTimestampBounds { lower, upper })
    }

    fn bounds_error(message: String) -> StockTrekError {
        StockTrekError::Stats(StatsError::DomainError {
            function: "AlignedWindow::bounds_for",
            message,
        })
    }

    /// Returns the exact window length in milliseconds for fixed-length windows.
    ///
    /// Calendar windows have no fixed length, so this returns `None` for them.
    pub fn millis_exact(&self) -> Option<u64> {
        let millis = match self {
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
            AlignedWindow::Months1
            | AlignedWindow::Months2
            | AlignedWindow::Months3
            | AlignedWindow::Months4
            | AlignedWindow::Months6
            | AlignedWindow::Months12
            | AlignedWindow::Years1
            | AlignedWindow::Years2 => return None,
        };
        Some(u64::from(millis))
    }

    /// Returns an upper bound on the window length in milliseconds.
    pub fn millis_upper_bound(&self) -> Option<u64> {
        if let Some(millis) = self.millis_exact() {
            return Some(millis);
        }

        match self {
            AlignedWindow::Months1 => Some(MILLIS_PER_MONTH_UPPER_BOUND),
            AlignedWindow::Months2 => Some(2 * MILLIS_PER_MONTH_UPPER_BOUND),
            AlignedWindow::Months3 => Some(3 * MILLIS_PER_MONTH_UPPER_BOUND),
            AlignedWindow::Months4 => Some(4 * MILLIS_PER_MONTH_UPPER_BOUND),
            AlignedWindow::Months6 => Some(6 * MILLIS_PER_MONTH_UPPER_BOUND),
            AlignedWindow::Months12 | AlignedWindow::Years1 => Some(MILLIS_PER_YEAR_UPPER_BOUND),
            AlignedWindow::Years2 => Some(2 * MILLIS_PER_YEAR_UPPER_BOUND),
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

impl Serialize for AlignedWindow {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.label())
    }
}

impl<'de> Deserialize<'de> for AlignedWindow {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let label = String::deserialize(deserializer)?;
        Self::try_from(label.as_str()).map_err(serde::de::Error::custom)
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
mod tests;
