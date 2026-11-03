use crate::{
    error::result::{StockTrekError, StockTrekResult},
    market_data::{millis::*, timestamp::TimestampMillis, window::window_enum},
    validation::Validation,
};
use chrono::{DateTime, Datelike, Months, TimeZone, Utc};
use serde::{Deserialize, Serialize};

window_enum!(
    /// A window aligned to fixed or calendar boundaries.
    ///
    /// Variants up to and including `Weeks4` have fixed lengths. Month and year
    /// variants align to calendar boundaries and vary in length. The short labels
    /// returned by [`Self::label`] are used when the window is displayed or
    /// serialized.
    AlignedWindow,
    exact {
        Seconds1 => "1s" => MILLIS_PER_SECOND,
        Seconds5 => "5s" => 5 * MILLIS_PER_SECOND,
        Seconds10 => "10s" => 10 * MILLIS_PER_SECOND,
        Seconds15 => "15s" => 15 * MILLIS_PER_SECOND,
        Seconds30 => "30s" => 30 * MILLIS_PER_SECOND,
        Minutes1 => "1m" => MILLIS_PER_MINUTE,
        Minutes2 => "2m" => 2 * MILLIS_PER_MINUTE,
        Minutes3 => "3m" => 3 * MILLIS_PER_MINUTE,
        Minutes5 => "5m" => 5 * MILLIS_PER_MINUTE,
        Minutes10 => "10m" => 10 * MILLIS_PER_MINUTE,
        Minutes15 => "15m" => 15 * MILLIS_PER_MINUTE,
        Minutes30 => "30m" => 30 * MILLIS_PER_MINUTE,
        Hours1 => "1h" => MILLIS_PER_HOUR,
        Hours2 => "2h" => 2 * MILLIS_PER_HOUR,
        Hours3 => "3h" => 3 * MILLIS_PER_HOUR,
        Hours4 => "4h" => 4 * MILLIS_PER_HOUR,
        Hours6 => "6h" => 6 * MILLIS_PER_HOUR,
        Hours8 => "8h" => 8 * MILLIS_PER_HOUR,
        Hours12 => "12h" => 12 * MILLIS_PER_HOUR,
        Days1 => "1d" => MILLIS_PER_DAY,
        Weeks1 => "1w" => MILLIS_PER_WEEK,
        Weeks2 => "2w" => 2 * MILLIS_PER_WEEK,
        Weeks4 => "4w" => 4 * MILLIS_PER_WEEK,
    }
    upper {
        Months1 => "1mo" => MILLIS_PER_MONTH_UPPER_BOUND,
        Months2 => "2mo" => 2 * MILLIS_PER_MONTH_UPPER_BOUND,
        Months3 => "3mo" => 3 * MILLIS_PER_MONTH_UPPER_BOUND,
        Months4 => "4mo" => 4 * MILLIS_PER_MONTH_UPPER_BOUND,
        Months6 => "6mo" => 6 * MILLIS_PER_MONTH_UPPER_BOUND,
        Years1 => "1y" => MILLIS_PER_YEAR_UPPER_BOUND,
        Years2 => "2y" => 2 * MILLIS_PER_YEAR_UPPER_BOUND,
    }
);

/// The bounds of an aligned window, both as [`TimestampMillis`].
///
/// `lower` is inclusive and `upper` is exclusive. The lower bound is validated
/// against the upper bound when the bounds are created or deserialized.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(try_from = "AlignedTimestampBoundsData")]
pub struct AlignedTimestampBounds {
    lower: TimestampMillis,
    upper: TimestampMillis,
}

impl AlignedTimestampBounds {
    /// Creates bounds from an inclusive lower and exclusive upper timestamp.
    ///
    /// Returns an error when `lower` is greater than `upper`.
    pub fn new(lower: TimestampMillis, upper: TimestampMillis) -> StockTrekResult<Self> {
        if lower > upper {
            return Err(Validation::error(
                "AlignedTimestampBounds::new",
                format!("lower bound {lower} must not be greater than upper bound {upper}"),
            ));
        }
        Ok(Self { lower, upper })
    }

    /// Returns the inclusive lower bound.
    pub fn lower(&self) -> TimestampMillis {
        self.lower
    }

    /// Returns the exclusive upper bound.
    pub fn upper(&self) -> TimestampMillis {
        self.upper
    }
}

#[derive(Deserialize)]
struct AlignedTimestampBoundsData {
    lower: TimestampMillis,
    upper: TimestampMillis,
}

impl TryFrom<AlignedTimestampBoundsData> for AlignedTimestampBounds {
    type Error = StockTrekError;

    fn try_from(data: AlignedTimestampBoundsData) -> Result<Self, Self::Error> {
        Self::new(data.lower, data.upper)
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
        if let Some(exact_millis) = self.millis().exact() {
            self.fixed_bounds(timestamp, exact_millis)
        } else if let Some(exact_months) = self.months_exact() {
            self.calendar_bounds(timestamp, exact_months)
        } else {
            Err(Validation::error(
                "AlignedWindow::bounds_for",
                format!("Could not calculate bounds for {self} at timestamp {timestamp}"),
            ))
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
            Validation::error(
                "AlignedWindow::bounds_for",
                format!("upper bound for timestamp {timestamp} could not be represented"),
            )
        })?;
        AlignedTimestampBounds::new(
            TimestampMillis::new(lower_millis),
            TimestampMillis::new(upper_millis),
        )
    }

    fn calendar_bounds(
        &self,
        timestamp: TimestampMillis,
        exact_months: u32,
    ) -> StockTrekResult<AlignedTimestampBounds> {
        let timestamp_i64 = i64::try_from(timestamp.as_millis()).map_err(|_| {
            Validation::error(
                "AlignedWindow::bounds_for",
                format!("timestamp {timestamp} is not representable as a UTC date"),
            )
        })?;
        let date_time = DateTime::from_timestamp_millis(timestamp_i64).ok_or_else(|| {
            Validation::error(
                "AlignedWindow::bounds_for",
                format!("timestamp {timestamp} is not representable as a UTC date"),
            )
        })?;
        let months_since_epoch = i64::from(date_time.year()) * 12 + i64::from(date_time.month0());
        let months_per_window = i64::from(exact_months);
        let window_start_months =
            months_since_epoch.div_euclid(months_per_window) * months_per_window;
        let start_year = window_start_months.div_euclid(12);
        let start_year = i32::try_from(start_year).map_err(|_| {
            Validation::error(
                "AlignedWindow::bounds_for",
                format!(
                    "calendar window year {start_year} for timestamp {timestamp} is not representable"
                ),
            )
        })?;
        let start_month = window_start_months.rem_euclid(12) as u32 + 1;
        let lower = Utc
            .with_ymd_and_hms(start_year, start_month, 1, 0, 0, 0)
            .single()
            .ok_or_else(|| {
                Validation::error(
                    "AlignedWindow::bounds_for",
                    format!(
                        "calendar window lower bound for {start_year}-{start_month:02} could not be represented"
                    ),
                )
            })?;
        let upper = lower.checked_add_months(Months::new(exact_months)).ok_or_else(|| {
            Validation::error(
                "AlignedWindow::bounds_for",
                format!(
                    "calendar window upper bound for {start_year}-{start_month:02} could not be represented"
                ),
            )
        })?;
        let lower = TimestampMillis::try_from(lower.timestamp_millis()).map_err(|_| {
            Validation::error(
                "AlignedWindow::bounds_for",
                format!(
                    "calendar window lower bound for {start_year}-{start_month:02} could not be represented as a timestamp"
                ),
            )
        })?;
        let upper = TimestampMillis::try_from(upper.timestamp_millis()).map_err(|_| {
            Validation::error(
                "AlignedWindow::bounds_for",
                format!(
                    "calendar window upper bound for {start_year}-{start_month:02} could not be represented as a timestamp"
                ),
            )
        })?;
        AlignedTimestampBounds::new(lower, upper)
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
