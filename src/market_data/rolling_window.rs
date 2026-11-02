use crate::{
    errors::{StockTrekError, ValueError},
    market_data::millis::*,
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::str::FromStr;
use strum::{EnumCount, EnumIter, FromRepr};

/// A fixed-length rolling time window.
///
/// Variants are named after their length, so `Minutes5` is five minutes and
/// `Days7` is seven days. The short labels returned by [`Self::label`] are used
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
    Seconds60,
    Minutes1,
    Minutes2,
    Minutes3,
    Minutes5,
    Minutes10,
    Minutes15,
    Minutes30,
    Minutes45,
    Minutes60,
    Minutes90,
    Hours1,
    Hours2,
    Hours3,
    Hours4,
    Hours6,
    Hours8,
    Hours12,
    Hours24,
    Days1,
    Days2,
    Days3,
    Days7,
    Days14,
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
            RollingWindow::Seconds60 => Some(60 * millis_per_second),
            RollingWindow::Minutes1 => Some(millis_per_minute),
            RollingWindow::Minutes2 => Some(2 * millis_per_minute),
            RollingWindow::Minutes3 => Some(3 * millis_per_minute),
            RollingWindow::Minutes5 => Some(5 * millis_per_minute),
            RollingWindow::Minutes10 => Some(10 * millis_per_minute),
            RollingWindow::Minutes15 => Some(15 * millis_per_minute),
            RollingWindow::Minutes30 => Some(30 * millis_per_minute),
            RollingWindow::Minutes45 => Some(45 * millis_per_minute),
            RollingWindow::Minutes60 => Some(60 * millis_per_minute),
            RollingWindow::Minutes90 => Some(90 * millis_per_minute),
            RollingWindow::Hours1 => Some(millis_per_hour),
            RollingWindow::Hours2 => Some(2 * millis_per_hour),
            RollingWindow::Hours3 => Some(3 * millis_per_hour),
            RollingWindow::Hours4 => Some(4 * millis_per_hour),
            RollingWindow::Hours6 => Some(6 * millis_per_hour),
            RollingWindow::Hours8 => Some(8 * millis_per_hour),
            RollingWindow::Hours12 => Some(12 * millis_per_hour),
            RollingWindow::Hours24 => Some(24 * millis_per_hour),
            RollingWindow::Days1 => Some(millis_per_day),
            RollingWindow::Days2 => Some(2 * millis_per_day),
            RollingWindow::Days3 => Some(3 * millis_per_day),
            RollingWindow::Days7 => Some(7 * millis_per_day),
            RollingWindow::Days14 => Some(14 * millis_per_day),
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

    /// Returns the short label for the window, such as `1h` or `7d`.
    pub fn label(self) -> &'static str {
        match self {
            RollingWindow::Seconds1 => "1s",
            RollingWindow::Seconds5 => "5s",
            RollingWindow::Seconds10 => "10s",
            RollingWindow::Seconds15 => "15s",
            RollingWindow::Seconds30 => "30s",
            RollingWindow::Seconds60 => "60s",
            RollingWindow::Minutes1 => "1m",
            RollingWindow::Minutes2 => "2m",
            RollingWindow::Minutes3 => "3m",
            RollingWindow::Minutes5 => "5m",
            RollingWindow::Minutes10 => "10m",
            RollingWindow::Minutes15 => "15m",
            RollingWindow::Minutes30 => "30m",
            RollingWindow::Minutes45 => "45m",
            RollingWindow::Minutes60 => "60m",
            RollingWindow::Minutes90 => "90m",
            RollingWindow::Hours1 => "1h",
            RollingWindow::Hours2 => "2h",
            RollingWindow::Hours3 => "3h",
            RollingWindow::Hours4 => "4h",
            RollingWindow::Hours6 => "6h",
            RollingWindow::Hours8 => "8h",
            RollingWindow::Hours12 => "12h",
            RollingWindow::Hours24 => "24h",
            RollingWindow::Days1 => "1d",
            RollingWindow::Days2 => "2d",
            RollingWindow::Days3 => "3d",
            RollingWindow::Days7 => "7d",
            RollingWindow::Days14 => "14d",
            RollingWindow::Days30 => "30d",
            RollingWindow::Days45 => "45d",
            RollingWindow::Days60 => "60d",
            RollingWindow::Days90 => "90d",
            RollingWindow::Days180 => "180d",
            RollingWindow::Days360 => "360d",
            RollingWindow::Weeks1 => "1w",
            RollingWindow::Weeks2 => "2w",
            RollingWindow::Weeks4 => "4w",
            RollingWindow::Weeks10 => "10w",
            RollingWindow::Weeks20 => "20w",
            RollingWindow::Weeks40 => "40w",
        }
    }
}

impl std::fmt::Display for RollingWindow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.label())
    }
}

impl Serialize for RollingWindow {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.label())
    }
}

impl<'de> Deserialize<'de> for RollingWindow {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let label = String::deserialize(deserializer)?;
        Self::try_from(label.as_str()).map_err(serde::de::Error::custom)
    }
}

impl TryFrom<&str> for RollingWindow {
    type Error = StockTrekError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "1s" => Ok(RollingWindow::Seconds1),
            "5s" => Ok(RollingWindow::Seconds5),
            "10s" => Ok(RollingWindow::Seconds10),
            "15s" => Ok(RollingWindow::Seconds15),
            "30s" => Ok(RollingWindow::Seconds30),
            "60s" => Ok(RollingWindow::Seconds60),
            "1m" => Ok(RollingWindow::Minutes1),
            "2m" => Ok(RollingWindow::Minutes2),
            "3m" => Ok(RollingWindow::Minutes3),
            "5m" => Ok(RollingWindow::Minutes5),
            "10m" => Ok(RollingWindow::Minutes10),
            "15m" => Ok(RollingWindow::Minutes15),
            "30m" => Ok(RollingWindow::Minutes30),
            "45m" => Ok(RollingWindow::Minutes45),
            "60m" => Ok(RollingWindow::Minutes60),
            "90m" => Ok(RollingWindow::Minutes90),
            "1h" => Ok(RollingWindow::Hours1),
            "2h" => Ok(RollingWindow::Hours2),
            "3h" => Ok(RollingWindow::Hours3),
            "4h" => Ok(RollingWindow::Hours4),
            "6h" => Ok(RollingWindow::Hours6),
            "8h" => Ok(RollingWindow::Hours8),
            "12h" => Ok(RollingWindow::Hours12),
            "24h" => Ok(RollingWindow::Hours24),
            "1d" => Ok(RollingWindow::Days1),
            "2d" => Ok(RollingWindow::Days2),
            "3d" => Ok(RollingWindow::Days3),
            "7d" => Ok(RollingWindow::Days7),
            "14d" => Ok(RollingWindow::Days14),
            "30d" => Ok(RollingWindow::Days30),
            "45d" => Ok(RollingWindow::Days45),
            "60d" => Ok(RollingWindow::Days60),
            "90d" => Ok(RollingWindow::Days90),
            "180d" => Ok(RollingWindow::Days180),
            "360d" => Ok(RollingWindow::Days360),
            "1w" => Ok(RollingWindow::Weeks1),
            "2w" => Ok(RollingWindow::Weeks2),
            "4w" => Ok(RollingWindow::Weeks4),
            "10w" => Ok(RollingWindow::Weeks10),
            "20w" => Ok(RollingWindow::Weeks20),
            "40w" => Ok(RollingWindow::Weeks40),
            _ => Err(StockTrekError::Value(ValueError::NotFound {
                name: "RollingWindow".into(),
                key: value.into(),
            })),
        }
    }
}

impl FromStr for RollingWindow {
    type Err = StockTrekError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::try_from(value)
    }
}

#[cfg(test)]
mod tests;
