use crate::market_data::{millis::*, window_label::window_label_funcs};
use strum::{EnumCount, EnumIter, FromRepr};

/// A fixed-length rolling time window.
///
/// Variants are named after their length, so `Minutes5` is five minutes and
/// `Weeks1` is one week. The short labels returned by [`Self::label`] are used
/// when the window is displayed or serialized.
#[derive(
    Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash, EnumCount, EnumIter, FromRepr,
)]
#[allow(missing_docs)]
#[non_exhaustive]
#[repr(u8)]
pub enum RollingWindow {
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
    Minutes45,
    Minutes90,
    Hours1,
    Hours2,
    Hours3,
    Hours4,
    Hours6,
    Hours8,
    Hours12,
    Days1,
    Days2,
    Days3,
    Days30,
    Days45,
    Days60,
    Days90,
    Days180,
    Days360,
    Weeks1,
    Weeks2,
    Weeks4,
    Weeks10,
    Weeks20,
    Weeks40,
}

window_label_funcs!(
    RollingWindow,
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
    Minutes45 => "45m",
    Minutes90 => "90m",
    Hours1 => "1h",
    Hours2 => "2h",
    Hours3 => "3h",
    Hours4 => "4h",
    Hours6 => "6h",
    Hours8 => "8h",
    Hours12 => "12h",
    Days1 => "1d",
    Days2 => "2d",
    Days3 => "3d",
    Days30 => "30d",
    Days45 => "45d",
    Days60 => "60d",
    Days90 => "90d",
    Days180 => "180d",
    Days360 => "360d",
    Weeks1 => "1w",
    Weeks2 => "2w",
    Weeks4 => "4w",
    Weeks10 => "10w",
    Weeks20 => "20w",
    Weeks40 => "40w",
);

impl RollingWindow {
    /// Returns the exact window length in milliseconds.
    pub fn millis_exact(&self) -> Option<u64> {
        let millis_per_second = MILLIS_PER_SECOND as u64;
        let millis_per_minute = MILLIS_PER_MINUTE as u64;
        let millis_per_hour = MILLIS_PER_HOUR as u64;
        let millis_per_day = MILLIS_PER_DAY as u64;
        let millis_per_week = MILLIS_PER_WEEK as u64;
        match self {
            RollingWindow::Seconds1 => Some(millis_per_second),
            RollingWindow::Seconds5 => Some(5 * millis_per_second),
            RollingWindow::Seconds10 => Some(10 * millis_per_second),
            RollingWindow::Seconds15 => Some(15 * millis_per_second),
            RollingWindow::Seconds30 => Some(30 * millis_per_second),
            RollingWindow::Minutes1 => Some(millis_per_minute),
            RollingWindow::Minutes2 => Some(2 * millis_per_minute),
            RollingWindow::Minutes3 => Some(3 * millis_per_minute),
            RollingWindow::Minutes5 => Some(5 * millis_per_minute),
            RollingWindow::Minutes10 => Some(10 * millis_per_minute),
            RollingWindow::Minutes15 => Some(15 * millis_per_minute),
            RollingWindow::Minutes30 => Some(30 * millis_per_minute),
            RollingWindow::Minutes45 => Some(45 * millis_per_minute),
            RollingWindow::Minutes90 => Some(90 * millis_per_minute),
            RollingWindow::Hours1 => Some(millis_per_hour),
            RollingWindow::Hours2 => Some(2 * millis_per_hour),
            RollingWindow::Hours3 => Some(3 * millis_per_hour),
            RollingWindow::Hours4 => Some(4 * millis_per_hour),
            RollingWindow::Hours6 => Some(6 * millis_per_hour),
            RollingWindow::Hours8 => Some(8 * millis_per_hour),
            RollingWindow::Hours12 => Some(12 * millis_per_hour),
            RollingWindow::Days1 => Some(millis_per_day),
            RollingWindow::Days2 => Some(2 * millis_per_day),
            RollingWindow::Days3 => Some(3 * millis_per_day),
            RollingWindow::Days30 => Some(30 * millis_per_day),
            RollingWindow::Days45 => Some(45 * millis_per_day),
            RollingWindow::Days60 => Some(60 * millis_per_day),
            RollingWindow::Days90 => Some(90 * millis_per_day),
            RollingWindow::Days180 => Some(180 * millis_per_day),
            RollingWindow::Days360 => Some(360 * millis_per_day),
            RollingWindow::Weeks1 => Some(millis_per_week),
            RollingWindow::Weeks2 => Some(2 * millis_per_week),
            RollingWindow::Weeks4 => Some(4 * millis_per_week),
            RollingWindow::Weeks10 => Some(10 * millis_per_week),
            RollingWindow::Weeks20 => Some(20 * millis_per_week),
            RollingWindow::Weeks40 => Some(40 * millis_per_week),
        }
    }

    /// Returns the window length in milliseconds as an upper bound.
    pub fn millis_upper_bound(&self) -> Option<u64> {
        self.millis_exact()
    }
}

#[cfg(test)]
mod tests;
