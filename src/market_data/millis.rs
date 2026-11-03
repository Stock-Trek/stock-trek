//! Millisecond constants for common time spans.

/// Milliseconds in one second.
pub const MILLIS_PER_SECOND: u64 = 1_000;
/// Milliseconds in one minute.
pub const MILLIS_PER_MINUTE: u64 = 60 * MILLIS_PER_SECOND;
/// Milliseconds in one hour.
pub const MILLIS_PER_HOUR: u64 = 60 * MILLIS_PER_MINUTE;
/// Milliseconds in one day.
pub const MILLIS_PER_DAY: u64 = 24 * MILLIS_PER_HOUR;
/// Milliseconds in one week.
pub const MILLIS_PER_WEEK: u64 = 7 * MILLIS_PER_DAY;
/// Upper bound on the number of milliseconds in one month.
pub const MILLIS_PER_MONTH_UPPER_BOUND: u64 = 31 * MILLIS_PER_DAY;
/// Upper bound on the number of milliseconds in one year.
pub const MILLIS_PER_YEAR_UPPER_BOUND: u64 = 366 * MILLIS_PER_DAY;
