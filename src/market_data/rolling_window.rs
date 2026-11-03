use crate::market_data::{millis::*, window::window_enum};

window_enum!(
    /// A fixed-length rolling time window.
    ///
    /// Variants are named after their length, so `Minutes5` is five minutes and
    /// `Weeks1` is one week. The short labels returned by [`Self::label`] are used
    /// when the window is displayed or serialized.
    RollingWindow,
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
        Minutes45 => "45m" => 45 * MILLIS_PER_MINUTE,
        Minutes90 => "90m" => 90 * MILLIS_PER_MINUTE,
        Hours1 => "1h" => MILLIS_PER_HOUR,
        Hours2 => "2h" => 2 * MILLIS_PER_HOUR,
        Hours3 => "3h" => 3 * MILLIS_PER_HOUR,
        Hours4 => "4h" => 4 * MILLIS_PER_HOUR,
        Hours6 => "6h" => 6 * MILLIS_PER_HOUR,
        Hours8 => "8h" => 8 * MILLIS_PER_HOUR,
        Hours12 => "12h" => 12 * MILLIS_PER_HOUR,
        Days1 => "1d" => MILLIS_PER_DAY,
        Days2 => "2d" => 2 * MILLIS_PER_DAY,
        Days3 => "3d" => 3 * MILLIS_PER_DAY,
        Days30 => "30d" => 30 * MILLIS_PER_DAY,
        Days45 => "45d" => 45 * MILLIS_PER_DAY,
        Days60 => "60d" => 60 * MILLIS_PER_DAY,
        Days90 => "90d" => 90 * MILLIS_PER_DAY,
        Days180 => "180d" => 180 * MILLIS_PER_DAY,
        Days360 => "360d" => 360 * MILLIS_PER_DAY,
        Weeks1 => "1w" => MILLIS_PER_WEEK,
        Weeks2 => "2w" => 2 * MILLIS_PER_WEEK,
        Weeks4 => "4w" => 4 * MILLIS_PER_WEEK,
        Weeks10 => "10w" => 10 * MILLIS_PER_WEEK,
        Weeks20 => "20w" => 20 * MILLIS_PER_WEEK,
        Weeks40 => "40w" => 40 * MILLIS_PER_WEEK,
    }
);
