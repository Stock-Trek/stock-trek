use super::*;
use chrono::{DateTime, Datelike, Months, TimeZone, Utc};

impl AlignedWindow {
    pub(super) fn calendar_bounds(
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
