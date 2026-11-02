use crate::{
    error::{
        market_data::MarketDataError,
        result::{StockTrekError, StockTrekResult},
    },
    market_data::{millis::*, timestamp::TimestampMillis, window_label::window_label_funcs},
};
use chrono::{DateTime, Datelike, Months, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use strum::{EnumCount, EnumIter, FromRepr};

/// A window aligned to fixed or calendar boundaries.
///
/// Variants up to and including `Weeks4` have fixed lengths. Month and year
/// variants align to calendar boundaries and vary in length. The short labels
/// returned by [`Self::label`] are used when the window is displayed or
/// serialized.
#[derive(
    Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash, EnumCount, EnumIter, FromRepr,
)]
#[allow(missing_docs)]
#[non_exhaustive]
#[repr(u8)]
pub enum AlignedWindow {
    Seconds1,
    Seconds5,
    Seconds10,
    Seconds15,
    Seconds30,
    Minutes1,
    Minutes2,
    Minutes3,
    Minutes5,
    Minutes10,
    Minutes15,
    Minutes30,
    Hours1,
    Hours2,
    Hours3,
    Hours4,
    Hours6,
    Hours8,
    Hours12,
    Days1,
    Weeks1,
    Weeks2,
    Weeks4,
    Months1,
    Months2,
    Months3,
    Months4,
    Months6,
    Years1,
    Years2,
}

window_label_funcs!(
    AlignedWindow,
    Seconds1 => "1s",
    Seconds5 => "5s",
    Seconds10 => "10s",
    Seconds15 => "15s",
    Seconds30 => "30s",
    Minutes1 => "1m",
    Minutes2 => "2m",
    Minutes3 => "3m",
    Minutes5 => "5m",
    Minutes10 => "10m",
    Minutes15 => "15m",
    Minutes30 => "30m",
    Hours1 => "1h",
    Hours2 => "2h",
    Hours3 => "3h",
    Hours4 => "4h",
    Hours6 => "6h",
    Hours8 => "8h",
    Hours12 => "12h",
    Days1 => "1d",
    Weeks1 => "1w",
    Weeks2 => "2w",
    Weeks4 => "4w",
    Months1 => "1mo",
    Months2 => "2mo",
    Months3 => "3mo",
    Months4 => "4mo",
    Months6 => "6mo",
    Years1 => "1y",
    Years2 => "2y",
);

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
        let timestamp_millis = timestamp.as_millis();
        let lower_millis = timestamp_millis.div_euclid(exact_millis) * exact_millis;
        let upper_millis = lower_millis.checked_add(exact_millis).ok_or_else(|| {
            Self::bounds_error(format!(
                "upper bound for timestamp {timestamp} could not be represented"
            ))
        })?;
        Ok(AlignedTimestampBounds::new(
            TimestampMillis::new(lower_millis),
            TimestampMillis::new(upper_millis),
        ))
    }

    fn calendar_bounds(
        &self,
        timestamp: TimestampMillis,
        exact_months: u32,
    ) -> StockTrekResult<AlignedTimestampBounds> {
        let timestamp_i64 = i64::try_from(timestamp.as_millis()).map_err(|_| {
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
        StockTrekError::MarketData(MarketDataError::DomainError {
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
            AlignedWindow::Minutes1 => MILLIS_PER_MINUTE,
            AlignedWindow::Minutes2 => 2 * MILLIS_PER_MINUTE,
            AlignedWindow::Minutes3 => 3 * MILLIS_PER_MINUTE,
            AlignedWindow::Minutes5 => 5 * MILLIS_PER_MINUTE,
            AlignedWindow::Minutes10 => 10 * MILLIS_PER_MINUTE,
            AlignedWindow::Minutes15 => 15 * MILLIS_PER_MINUTE,
            AlignedWindow::Minutes30 => 30 * MILLIS_PER_MINUTE,
            AlignedWindow::Hours1 => MILLIS_PER_HOUR,
            AlignedWindow::Hours2 => 2 * MILLIS_PER_HOUR,
            AlignedWindow::Hours3 => 3 * MILLIS_PER_HOUR,
            AlignedWindow::Hours4 => 4 * MILLIS_PER_HOUR,
            AlignedWindow::Hours6 => 6 * MILLIS_PER_HOUR,
            AlignedWindow::Hours8 => 8 * MILLIS_PER_HOUR,
            AlignedWindow::Hours12 => 12 * MILLIS_PER_HOUR,
            AlignedWindow::Days1 => MILLIS_PER_DAY,
            AlignedWindow::Weeks1 => MILLIS_PER_WEEK,
            AlignedWindow::Weeks2 => 2 * MILLIS_PER_WEEK,
            AlignedWindow::Weeks4 => 4 * MILLIS_PER_WEEK,
            AlignedWindow::Months1
            | AlignedWindow::Months2
            | AlignedWindow::Months3
            | AlignedWindow::Months4
            | AlignedWindow::Months6
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
            AlignedWindow::Years1 => Some(MILLIS_PER_YEAR_UPPER_BOUND),
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
            AlignedWindow::Years1 => Some(12),
            AlignedWindow::Years2 => Some(24),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests;
