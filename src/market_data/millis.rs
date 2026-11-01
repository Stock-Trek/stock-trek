pub(crate) const MILLIS_PER_SECOND: u32 = 1_000;
pub(crate) const MILLIS_PER_MINUTE: u32 = 60 * MILLIS_PER_SECOND;
pub(crate) const MILLIS_PER_HOUR: u32 = 60 * MILLIS_PER_MINUTE;
pub(crate) const MILLIS_PER_DAY: u32 = 24 * MILLIS_PER_HOUR;
pub(crate) const MILLIS_PER_WEEK: u32 = 7 * MILLIS_PER_DAY;
pub(crate) const MILLIS_PER_MONTH_UPPER_BOUND: u64 = 31 * MILLIS_PER_DAY as u64;
pub(crate) const MILLIS_PER_YEAR_UPPER_BOUND: u64 = 366 * MILLIS_PER_DAY as u64;
