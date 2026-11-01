use crate::{
    errors::{StockTrekError, ValueError},
    market_data::millis::*,
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use strum::{EnumCount, EnumIter, FromRepr};

/// A fixed-length rolling time window.
#[derive(
    Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash, EnumCount, EnumIter, FromRepr,
)]
#[non_exhaustive]
#[repr(u8)]
pub enum RollingWindow {
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
    /// A forty-five-minute window.
    Minutes45,
    /// A sixty-minute window.
    Minutes60,
    /// A ninety-minute window.
    Minutes90,
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
    /// A two-day window.
    Days2,
    /// A three-day window.
    Days3,
    /// A seven-day window.
    Days7,
    /// A fourteen-day window.
    Days14,
    /// A thirty-day window.
    Days30,
    /// A forty-five-day window.
    Days45,
    /// A sixty-day window.
    Days60,
    /// A ninety-day window.
    Days90,
    /// A one-hundred-eighty-day window.
    Days180,
    /// A three-hundred-sixty-day window.
    Days360,
    /// A one-week window.
    Weeks1,
    /// A two-week window.
    Weeks2,
    /// A four-week window.
    Weeks4,
    /// A ten-week window.
    Weeks10,
    /// A twenty-week window.
    Weeks20,
    /// A forty-week window.
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
    pub fn label(self) -> String {
        match self {
            RollingWindow::Seconds1 => "1s".to_string(),
            RollingWindow::Seconds5 => "5s".to_string(),
            RollingWindow::Seconds10 => "10s".to_string(),
            RollingWindow::Seconds15 => "15s".to_string(),
            RollingWindow::Seconds30 => "30s".to_string(),
            RollingWindow::Seconds60 => "60s".to_string(),
            RollingWindow::Minutes1 => "1m".to_string(),
            RollingWindow::Minutes2 => "2m".to_string(),
            RollingWindow::Minutes3 => "3m".to_string(),
            RollingWindow::Minutes5 => "5m".to_string(),
            RollingWindow::Minutes10 => "10m".to_string(),
            RollingWindow::Minutes15 => "15m".to_string(),
            RollingWindow::Minutes30 => "30m".to_string(),
            RollingWindow::Minutes45 => "45m".to_string(),
            RollingWindow::Minutes60 => "60m".to_string(),
            RollingWindow::Minutes90 => "90m".to_string(),
            RollingWindow::Hours1 => "1h".to_string(),
            RollingWindow::Hours2 => "2h".to_string(),
            RollingWindow::Hours3 => "3h".to_string(),
            RollingWindow::Hours4 => "4h".to_string(),
            RollingWindow::Hours6 => "6h".to_string(),
            RollingWindow::Hours8 => "8h".to_string(),
            RollingWindow::Hours12 => "12h".to_string(),
            RollingWindow::Hours24 => "24h".to_string(),
            RollingWindow::Days1 => "1d".to_string(),
            RollingWindow::Days2 => "2d".to_string(),
            RollingWindow::Days3 => "3d".to_string(),
            RollingWindow::Days7 => "7d".to_string(),
            RollingWindow::Days14 => "14d".to_string(),
            RollingWindow::Days30 => "30d".to_string(),
            RollingWindow::Days45 => "45d".to_string(),
            RollingWindow::Days60 => "60d".to_string(),
            RollingWindow::Days90 => "90d".to_string(),
            RollingWindow::Days180 => "180d".to_string(),
            RollingWindow::Days360 => "360d".to_string(),
            RollingWindow::Weeks1 => "1w".to_string(),
            RollingWindow::Weeks2 => "2w".to_string(),
            RollingWindow::Weeks4 => "4w".to_string(),
            RollingWindow::Weeks10 => "10w".to_string(),
            RollingWindow::Weeks20 => "20w".to_string(),
            RollingWindow::Weeks40 => "40w".to_string(),
        }
    }
}

impl std::fmt::Display for RollingWindow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.label())
    }
}

impl Serialize for RollingWindow {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.label())
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

#[cfg(test)]
mod tests;
