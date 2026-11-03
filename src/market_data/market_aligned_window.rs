use crate::{
    error::result::StockTrekResult,
    market_data::{
        aligned_window::AlignedWindow, market_candle::MarketCandle, market_ohlcv::MarketOhlcv,
        timestamp::TimestampMillis,
    },
    validation::Validation,
};
use chrono::{TimeZone, Utc};
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::{BTreeMap, BTreeSet};

/// Generates constructors that take the components of a window's aligned UTC
/// start instead of timestamps.
///
/// Each generated constructor takes the components of the first window's
/// start, followed by an iterator of `(OHLCV, trade count)` tuples, and adds
/// one candle per tuple: the first candle starts at the given start, each
/// following candle starts where the previous one ends, and every created
/// candle is marked closed.
macro_rules! aligned_window_constructor {
    ($($(#[$doc:meta])* $name:ident, $variant:ident, [year]);* $(;)?) => {
        $(
            $(#[$doc])*
            ///
            /// The first candle starts at the start of `year` in UTC, each
            /// following candle starts where the previous one ends, and every
            /// created candle is marked closed. Returns an error when the
            /// start does not begin a window.
            pub fn $name(
                year: i32,
                candles: impl IntoIterator<Item = (MarketOhlcv, u64)>,
            ) -> StockTrekResult<Self> {
                let start = Self::window_start_at(year, 1, 1, 0, 0, 0)?;
                Self::with_consecutive_candles(AlignedWindow::$variant, start, candles)
            }
        )*
    };
    ($($(#[$doc:meta])* $name:ident, $variant:ident, [year, month]);* $(;)?) => {
        $(
            $(#[$doc])*
            ///
            /// The first candle starts at the start of `year`-`month` in UTC,
            /// each following candle starts where the previous one ends, and
            /// every created candle is marked closed. Returns an error when
            /// the start does not begin a window.
            pub fn $name(
                year: i32,
                month: u32,
                candles: impl IntoIterator<Item = (MarketOhlcv, u64)>,
            ) -> StockTrekResult<Self> {
                let start = Self::window_start_at(year, month, 1, 0, 0, 0)?;
                Self::with_consecutive_candles(AlignedWindow::$variant, start, candles)
            }
        )*
    };
    ($($(#[$doc:meta])* $name:ident, $variant:ident, [year, month, day]);* $(;)?) => {
        $(
            $(#[$doc])*
            ///
            /// The first candle starts at the start of `year`-`month`-`day`
            /// in UTC, each following candle starts where the previous one
            /// ends, and every created candle is marked closed. Returns an
            /// error when the start does not begin a window.
            pub fn $name(
                year: i32,
                month: u32,
                day: u32,
                candles: impl IntoIterator<Item = (MarketOhlcv, u64)>,
            ) -> StockTrekResult<Self> {
                let start = Self::window_start_at(year, month, day, 0, 0, 0)?;
                Self::with_consecutive_candles(AlignedWindow::$variant, start, candles)
            }
        )*
    };
    ($($(#[$doc:meta])* $name:ident, $variant:ident, [year, month, day, hour]);* $(;)?) => {
        $(
            $(#[$doc])*
            ///
            /// The first candle starts at `year`-`month`-`day` `hour`:00 in
            /// UTC, each following candle starts where the previous one ends,
            /// and every created candle is marked closed. Returns an error
            /// when the start does not begin a window.
            pub fn $name(
                year: i32,
                month: u32,
                day: u32,
                hour: u32,
                candles: impl IntoIterator<Item = (MarketOhlcv, u64)>,
            ) -> StockTrekResult<Self> {
                let start = Self::window_start_at(year, month, day, hour, 0, 0)?;
                Self::with_consecutive_candles(AlignedWindow::$variant, start, candles)
            }
        )*
    };
    ($($(#[$doc:meta])* $name:ident, $variant:ident, [year, month, day, hour, minute]);* $(;)?) => {
        $(
            $(#[$doc])*
            ///
            /// The first candle starts at `year`-`month`-`day` `hour`:`minute`
            /// in UTC, each following candle starts where the previous one
            /// ends, and every created candle is marked closed. Returns an
            /// error when the start does not begin a window.
            pub fn $name(
                year: i32,
                month: u32,
                day: u32,
                hour: u32,
                minute: u32,
                candles: impl IntoIterator<Item = (MarketOhlcv, u64)>,
            ) -> StockTrekResult<Self> {
                let start = Self::window_start_at(year, month, day, hour, minute, 0)?;
                Self::with_consecutive_candles(AlignedWindow::$variant, start, candles)
            }
        )*
    };
    ($($(#[$doc:meta])* $name:ident, $variant:ident, [year, month, day, hour, minute, second]);* $(;)?) => {
        $(
            $(#[$doc])*
            ///
            /// The first candle starts at `year`-`month`-`day`
            /// `hour`:`minute`:`second` in UTC, each following candle starts
            /// where the previous one ends, and every created candle is marked
            /// closed. Returns an error when the start does not begin a
            /// window.
            pub fn $name(
                year: i32,
                month: u32,
                day: u32,
                hour: u32,
                minute: u32,
                second: u32,
                candles: impl IntoIterator<Item = (MarketOhlcv, u64)>,
            ) -> StockTrekResult<Self> {
                let start = Self::window_start_at(year, month, day, hour, minute, second)?;
                Self::with_consecutive_candles(AlignedWindow::$variant, start, candles)
            }
        )*
    };
}

/// Candles grouped by aligned windows.
///
/// Candles within a window are kept in chronological order and exposed either
/// as whole candles or as per-field series such as `opens` and `closes`, in
/// the order needed for indicator calculations.
#[derive(Debug, Clone, Serialize)]
pub struct MarketAlignedWindow {
    candles: BTreeMap<AlignedWindow, Vec<MarketCandle>>,
}

impl MarketAlignedWindow {
    /// Creates an aligned-window collection from candles grouped by window.
    ///
    /// Candles are sorted chronologically by start time. Returns an error when
    /// the same window appears more than once or a window contains duplicate
    /// candle intervals.
    pub fn new(
        candles: impl IntoIterator<Item = (AlignedWindow, Vec<MarketCandle>)>,
    ) -> StockTrekResult<Self> {
        let mut candles_by_window: BTreeMap<AlignedWindow, Vec<MarketCandle>> = BTreeMap::new();
        for (window, window_candles) in candles {
            Self::validate_intervals(window, &window_candles)?;
            if candles_by_window.insert(window, window_candles).is_some() {
                return Err(Validation::error(
                    "MarketAlignedWindow::new",
                    format!("duplicate candles for aligned window {window:?}"),
                ));
            }
        }
        Ok(Self::from_map(candles_by_window))
    }

    aligned_window_constructor!(
        /// Creates an aligned-window collection with candles for 1-second
        /// windows.
        seconds1, Seconds1, [year, month, day, hour, minute, second];
        /// Creates an aligned-window collection with candles for 5-second
        /// windows. The second must be a multiple of 5.
        seconds5, Seconds5, [year, month, day, hour, minute, second];
        /// Creates an aligned-window collection with candles for 10-second
        /// windows. The second must be a multiple of 10.
        seconds10, Seconds10, [year, month, day, hour, minute, second];
        /// Creates an aligned-window collection with candles for 15-second
        /// windows. The second must be 0, 15, 30, or 45.
        seconds15, Seconds15, [year, month, day, hour, minute, second];
        /// Creates an aligned-window collection with candles for 30-second
        /// windows. The second must be 0 or 30.
        seconds30, Seconds30, [year, month, day, hour, minute, second];
    );

    aligned_window_constructor!(
        /// Creates an aligned-window collection with candles for 1-minute
        /// windows.
        minutes1, Minutes1, [year, month, day, hour, minute];
        /// Creates an aligned-window collection with candles for 2-minute
        /// windows. The minute must be even.
        minutes2, Minutes2, [year, month, day, hour, minute];
        /// Creates an aligned-window collection with candles for 3-minute
        /// windows. The minute must be a multiple of 3.
        minutes3, Minutes3, [year, month, day, hour, minute];
        /// Creates an aligned-window collection with candles for 5-minute
        /// windows. The minute must be a multiple of 5.
        minutes5, Minutes5, [year, month, day, hour, minute];
        /// Creates an aligned-window collection with candles for 10-minute
        /// windows. The minute must be a multiple of 10.
        minutes10, Minutes10, [year, month, day, hour, minute];
        /// Creates an aligned-window collection with candles for 15-minute
        /// windows. The minute must be 0, 15, 30, or 45.
        minutes15, Minutes15, [year, month, day, hour, minute];
        /// Creates an aligned-window collection with candles for 30-minute
        /// windows. The minute must be 0 or 30.
        minutes30, Minutes30, [year, month, day, hour, minute];
    );

    aligned_window_constructor!(
        /// Creates an aligned-window collection with candles for 1-hour
        /// windows.
        hours1, Hours1, [year, month, day, hour];
        /// Creates an aligned-window collection with candles for 2-hour
        /// windows. The hour must be even.
        hours2, Hours2, [year, month, day, hour];
        /// Creates an aligned-window collection with candles for 3-hour
        /// windows. The hour must be a multiple of 3.
        hours3, Hours3, [year, month, day, hour];
        /// Creates an aligned-window collection with candles for 4-hour
        /// windows. The hour must be a multiple of 4.
        hours4, Hours4, [year, month, day, hour];
        /// Creates an aligned-window collection with candles for 6-hour
        /// windows. The hour must be a multiple of 6.
        hours6, Hours6, [year, month, day, hour];
        /// Creates an aligned-window collection with candles for 8-hour
        /// windows. The hour must be a multiple of 8.
        hours8, Hours8, [year, month, day, hour];
        /// Creates an aligned-window collection with candles for 12-hour
        /// windows. The hour must be 0 or 12.
        hours12, Hours12, [year, month, day, hour];
    );

    aligned_window_constructor!(
        /// Creates an aligned-window collection with candles for 1-day
        /// windows. The date identifies the calendar day the first window
        /// covers.
        days1, Days1, [year, month, day];
        /// Creates an aligned-window collection with candles for 1-week
        /// windows. The date identifies the calendar day the first window
        /// covers.
        weeks1, Weeks1, [year, month, day];
        /// Creates an aligned-window collection with candles for 2-week
        /// windows. The date identifies the calendar day the first window
        /// covers.
        weeks2, Weeks2, [year, month, day];
        /// Creates an aligned-window collection with candles for 4-week
        /// windows. The date identifies the calendar day the first window
        /// covers.
        weeks4, Weeks4, [year, month, day];
    );

    aligned_window_constructor!(
        /// Creates an aligned-window collection with candles for 1-month
        /// windows. The month identifies the calendar month the first window
        /// covers.
        months1, Months1, [year, month];
        /// Creates an aligned-window collection with candles for 2-month
        /// windows. The month must start the first window: 2-month windows
        /// start in January, March, May, July, September, and November.
        months2, Months2, [year, month];
        /// Creates an aligned-window collection with candles for 3-month
        /// windows. The month must start the first window: 3-month windows
        /// start in January, April, July, and October.
        months3, Months3, [year, month];
        /// Creates an aligned-window collection with candles for 4-month
        /// windows. The month must start the first window: 4-month windows
        /// start in January, May, and September.
        months4, Months4, [year, month];
        /// Creates an aligned-window collection with candles for 6-month
        /// windows. The month must start the first window: 6-month windows
        /// start in January and July.
        months6, Months6, [year, month];
    );

    aligned_window_constructor!(
        /// Creates an aligned-window collection with candles for 1-year
        /// windows. The year identifies the calendar year the first window
        /// covers.
        years1, Years1, [year];
        /// Creates an aligned-window collection with candles for 2-year
        /// windows. The year must start the first window: 2-year windows
        /// start in even years.
        years2, Years2, [year];
    );

    fn validate_intervals(window: AlignedWindow, candles: &[MarketCandle]) -> StockTrekResult<()> {
        let mut intervals = BTreeSet::new();
        for candle in candles {
            let start = candle.start_time_millis_inc();
            let end = candle.end_time_millis_exc();
            if !intervals.insert((start, end)) {
                return Err(Validation::error(
                    "MarketAlignedWindow::new",
                    format!(
                        "duplicate candle interval {start}..{end} for aligned window {window:?}"
                    ),
                ));
            }
        }
        Ok(())
    }

    fn with_consecutive_candles(
        window: AlignedWindow,
        start: TimestampMillis,
        candles: impl IntoIterator<Item = (MarketOhlcv, u64)>,
    ) -> StockTrekResult<Self> {
        let mut candles_by_window: BTreeMap<AlignedWindow, Vec<MarketCandle>> = BTreeMap::new();
        let mut start = start;
        for (ohlcv, trade_count) in candles {
            let candle = Self::aligned_candle(window, start, ohlcv, trade_count)?;
            start = candle.end_time_millis_exc();
            candles_by_window.entry(window).or_default().push(candle);
        }
        Self::new(candles_by_window)
    }

    fn aligned_candle(
        window: AlignedWindow,
        start: TimestampMillis,
        ohlcv: MarketOhlcv,
        trade_count: u64,
    ) -> StockTrekResult<MarketCandle> {
        let end = match window {
            AlignedWindow::Weeks1 | AlignedWindow::Weeks2 | AlignedWindow::Weeks4 => {
                let millis = window.millis_exact().ok_or_else(|| {
                    Validation::error(
                        "MarketAlignedWindow::aligned_candle",
                        format!("the {window} window has no fixed length"),
                    )
                })?;
                start.checked_add(TimestampMillis::new(millis)).ok_or_else(
                    || {
                        Validation::error(
                            "MarketAlignedWindow::aligned_candle",
                            format!(
                                "the {window} window starting at {start} with duration {millis} ms overflows the timestamp range"
                            ),
                        )
                    },
                )?
            }
            _ => {
                let bounds = window.bounds_for(start).map_err(|error| {
                    Validation::error(
                        "MarketAlignedWindow::aligned_candle",
                        format!(
                            "could not determine the {window} window bounds at {start}: {error}"
                        ),
                    )
                })?;
                if bounds.lower() != start {
                    return Err(Validation::error(
                        "MarketAlignedWindow::aligned_candle",
                        format!(
                            "{start} is not the start of a {window} window: the window containing it starts at {}",
                            bounds.lower()
                        ),
                    ));
                }
                bounds.upper()
            }
        };
        MarketCandle::from_interval(start, end, true, ohlcv, trade_count)
    }

    fn window_start_at(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        second: u32,
    ) -> StockTrekResult<TimestampMillis> {
        let components =
            format!("{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}Z");
        let date_time = Utc
            .with_ymd_and_hms(year, month, day, hour, minute, second)
            .single()
            .ok_or_else(|| {
                Validation::error(
                    "MarketAlignedWindow::window_start_at",
                    format!("{components} is not a valid UTC time"),
                )
            })?;
        TimestampMillis::try_from(date_time.timestamp_millis()).map_err(|_| {
            Validation::error(
                "MarketAlignedWindow::window_start_at",
                format!("{components} is not representable as a timestamp"),
            )
        })
    }

    /// Creates an aligned-window collection from candles already grouped by
    /// window, sorting each window's candles chronologically.
    pub(super) fn from_map(
        mut candles_by_window: BTreeMap<AlignedWindow, Vec<MarketCandle>>,
    ) -> Self {
        for window_candles in candles_by_window.values_mut() {
            window_candles.sort_by_key(|candle| {
                (candle.start_time_millis_inc(), candle.end_time_millis_exc())
            });
        }
        Self {
            candles: candles_by_window,
        }
    }
    /// Returns all candles grouped by aligned window, ordered by window.
    pub fn candles(&self) -> &BTreeMap<AlignedWindow, Vec<MarketCandle>> {
        &self.candles
    }
    /// Returns the candles for an aligned window, ordered chronologically.
    pub fn candles_for(&self, window: AlignedWindow) -> Option<&[MarketCandle]> {
        self.candles.get(&window).map(Vec::as_slice)
    }
    /// The opening price of each candle in the window, chronologically.
    pub fn opens(&self, window: AlignedWindow) -> Option<Vec<f64>> {
        self.values(window, MarketOhlcv::open)
    }
    /// The highest price of each candle in the window, chronologically.
    pub fn highs(&self, window: AlignedWindow) -> Option<Vec<f64>> {
        self.values(window, MarketOhlcv::high)
    }
    /// The lowest price of each candle in the window, chronologically.
    pub fn lows(&self, window: AlignedWindow) -> Option<Vec<f64>> {
        self.values(window, MarketOhlcv::low)
    }
    /// The closing price of each candle in the window, chronologically.
    pub fn closes(&self, window: AlignedWindow) -> Option<Vec<f64>> {
        self.values(window, MarketOhlcv::close)
    }
    /// The base-asset volume of each candle in the window, chronologically.
    pub fn volumes(&self, window: AlignedWindow) -> Option<Vec<f64>> {
        self.values(window, MarketOhlcv::volume)
    }
    /// The quote-asset volume of each candle in the window, chronologically.
    pub fn quote_volumes(&self, window: AlignedWindow) -> Option<Vec<f64>> {
        self.values(window, MarketOhlcv::quote_volume)
    }
    /// The volume-weighted average price of each candle in the window,
    /// chronologically.
    pub fn vwaps(&self, window: AlignedWindow) -> Option<Vec<f64>> {
        self.values(window, MarketOhlcv::vwap)
    }

    fn values(
        &self,
        window: AlignedWindow,
        to_value: impl Fn(&MarketOhlcv) -> f64,
    ) -> Option<Vec<f64>> {
        let candles = self.candles.get(&window)?;
        Some(
            candles
                .iter()
                .map(|candle| to_value(candle.ohlcv()))
                .collect(),
        )
    }
}

impl<'de> Deserialize<'de> for MarketAlignedWindow {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct MarketAlignedWindowData {
            candles: BTreeMap<AlignedWindow, Vec<MarketCandle>>,
        }

        let data = MarketAlignedWindowData::deserialize(deserializer)?;
        Self::new(data.candles).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::market_data::timestamp::TimestampMillis;
    use chrono::{TimeZone, Utc};

    fn millis_at(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        second: u32,
    ) -> TimestampMillis {
        TimestampMillis::try_from(
            Utc.with_ymd_and_hms(year, month, day, hour, minute, second)
                .unwrap()
                .timestamp_millis(),
        )
        .unwrap()
    }

    fn candle() -> MarketCandle {
        candle_at(0, 1.0)
    }

    fn ohlcv_at(open: f64) -> MarketOhlcv {
        MarketOhlcv::new(
            open,
            open + 1.0,
            open - 0.5,
            open + 0.5,
            10.0,
            15.0,
            open + 0.4,
        )
        .unwrap()
    }

    fn candle_at(start_time_millis_inc: u64, open: f64) -> MarketCandle {
        MarketCandle::from_interval(
            TimestampMillis::new(start_time_millis_inc),
            TimestampMillis::new(start_time_millis_inc + 1),
            true,
            ohlcv_at(open),
            3,
        )
        .unwrap()
    }

    #[test]
    fn new_rejects_duplicate_windows() {
        let market = MarketAlignedWindow::new([
            (AlignedWindow::Minutes1, vec![candle_at(2, 2.0)]),
            (AlignedWindow::Minutes1, vec![candle_at(1, 1.0)]),
        ]);

        assert!(market.is_err());
    }

    #[test]
    fn new_rejects_duplicate_candle_intervals() {
        let market = MarketAlignedWindow::new([(
            AlignedWindow::Minutes1,
            vec![candle_at(2, 2.0), candle_at(2, 3.0)],
        )]);

        assert!(market.is_err());
    }

    #[test]
    fn new_sorts_candles_chronologically() {
        let market = MarketAlignedWindow::new(BTreeMap::from([(
            AlignedWindow::Minutes1,
            vec![candle_at(2, 2.0), candle_at(1, 1.0)],
        )]))
        .unwrap();

        assert_eq!(market.opens(AlignedWindow::Minutes1), Some(vec![1.0, 2.0]));
    }

    #[test]
    fn serde_json_round_trip() {
        let market =
            MarketAlignedWindow::new(BTreeMap::from([(AlignedWindow::Minutes1, vec![candle()])]))
                .unwrap();
        let json = serde_json::to_string(&market).unwrap();
        let deserialized: MarketAlignedWindow = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.opens(AlignedWindow::Minutes1), Some(vec![1.0]));
    }

    #[test]
    fn deserialization_sorts_candles_chronologically() {
        let json = serde_json::json!({
            "candles": {
                "1m": [
                    candle_at(2, 2.0),
                    candle_at(1, 1.0),
                ]
            }
        });
        let deserialized: MarketAlignedWindow = serde_json::from_value(json).unwrap();

        assert_eq!(
            deserialized.opens(AlignedWindow::Minutes1),
            Some(vec![1.0, 2.0])
        );
    }

    #[test]
    fn deserialization_rejects_duplicate_candle_intervals() {
        let json = serde_json::json!({
            "candles": {
                "1m": [
                    candle_at(2, 2.0),
                    candle_at(2, 3.0),
                ]
            }
        });

        assert!(serde_json::from_value::<MarketAlignedWindow>(json).is_err());
    }

    #[test]
    fn years1_creates_candles_for_calendar_years() {
        let market = MarketAlignedWindow::years1(2024, [(ohlcv_at(1.0), 3)]).unwrap();
        let candles = market.candles_for(AlignedWindow::Years1).unwrap();

        assert_eq!(candles.len(), 1);
        assert_eq!(
            candles[0].start_time_millis_inc(),
            millis_at(2024, 1, 1, 0, 0, 0)
        );
        assert_eq!(
            candles[0].end_time_millis_exc(),
            millis_at(2025, 1, 1, 0, 0, 0)
        );
        assert!(candles[0].is_candle_closed());
        assert_eq!(candles[0].trade_count(), 3);
    }

    #[test]
    fn years2_rejects_years_that_do_not_start_a_window() {
        assert!(MarketAlignedWindow::years2(2025, [(ohlcv_at(1.0), 3)]).is_err());

        let market = MarketAlignedWindow::years2(2024, [(ohlcv_at(1.0), 3)]).unwrap();
        let candles = market.candles_for(AlignedWindow::Years2).unwrap();
        assert_eq!(
            candles[0].start_time_millis_inc(),
            millis_at(2024, 1, 1, 0, 0, 0)
        );
        assert_eq!(
            candles[0].end_time_millis_exc(),
            millis_at(2026, 1, 1, 0, 0, 0)
        );
    }

    #[test]
    fn months1_creates_candles_for_calendar_months() {
        let market = MarketAlignedWindow::months1(2024, 2, [(ohlcv_at(1.0), 3)]).unwrap();
        let candles = market.candles_for(AlignedWindow::Months1).unwrap();

        assert_eq!(
            candles[0].start_time_millis_inc(),
            millis_at(2024, 2, 1, 0, 0, 0)
        );
        assert_eq!(
            candles[0].end_time_millis_exc(),
            millis_at(2024, 3, 1, 0, 0, 0)
        );
    }

    #[test]
    fn month_constructors_reject_months_that_do_not_start_a_window() {
        assert!(MarketAlignedWindow::months2(2024, 2, [(ohlcv_at(1.0), 3)]).is_err());
        assert!(MarketAlignedWindow::months6(2024, 2, [(ohlcv_at(1.0), 3)]).is_err());

        let market = MarketAlignedWindow::months2(2024, 3, [(ohlcv_at(1.0), 3)]).unwrap();
        let candles = market.candles_for(AlignedWindow::Months2).unwrap();
        assert_eq!(
            candles[0].start_time_millis_inc(),
            millis_at(2024, 3, 1, 0, 0, 0)
        );
        assert_eq!(
            candles[0].end_time_millis_exc(),
            millis_at(2024, 5, 1, 0, 0, 0)
        );

        let market = MarketAlignedWindow::months6(2024, 7, [(ohlcv_at(1.0), 3)]).unwrap();
        let candles = market.candles_for(AlignedWindow::Months6).unwrap();
        assert_eq!(
            candles[0].start_time_millis_inc(),
            millis_at(2024, 7, 1, 0, 0, 0)
        );
        assert_eq!(
            candles[0].end_time_millis_exc(),
            millis_at(2025, 1, 1, 0, 0, 0)
        );
    }

    #[test]
    fn weeks1_accepts_any_start_date() {
        let market =
            MarketAlignedWindow::weeks1(2024, 2, 2, [(ohlcv_at(1.0), 3), (ohlcv_at(2.0), 4)])
                .unwrap();
        let candles = market.candles_for(AlignedWindow::Weeks1).unwrap();

        assert_eq!(candles.len(), 2);
        assert_eq!(
            candles[0].start_time_millis_inc(),
            millis_at(2024, 2, 2, 0, 0, 0)
        );
        assert_eq!(
            candles[0].end_time_millis_exc(),
            millis_at(2024, 2, 9, 0, 0, 0)
        );
        assert_eq!(
            candles[1].start_time_millis_inc(),
            millis_at(2024, 2, 9, 0, 0, 0)
        );
        assert_eq!(
            candles[1].end_time_millis_exc(),
            millis_at(2024, 2, 16, 0, 0, 0)
        );
    }

    #[test]
    fn week_constructors_accept_any_start_date() {
        let market = MarketAlignedWindow::weeks2(2024, 2, 2, [(ohlcv_at(1.0), 3)]).unwrap();
        let candles = market.candles_for(AlignedWindow::Weeks2).unwrap();
        assert_eq!(
            candles[0].start_time_millis_inc(),
            millis_at(2024, 2, 2, 0, 0, 0)
        );
        assert_eq!(
            candles[0].end_time_millis_exc(),
            millis_at(2024, 2, 16, 0, 0, 0)
        );

        let market = MarketAlignedWindow::weeks4(2024, 2, 2, [(ohlcv_at(1.0), 3)]).unwrap();
        let candles = market.candles_for(AlignedWindow::Weeks4).unwrap();
        assert_eq!(
            candles[0].start_time_millis_inc(),
            millis_at(2024, 2, 2, 0, 0, 0)
        );
        assert_eq!(
            candles[0].end_time_millis_exc(),
            millis_at(2024, 3, 1, 0, 0, 0)
        );
    }

    #[test]
    fn days1_creates_candles_for_calendar_days_and_rejects_invalid_dates() {
        assert!(MarketAlignedWindow::days1(2024, 2, 30, [(ohlcv_at(1.0), 3)]).is_err());

        let market = MarketAlignedWindow::days1(2024, 2, 15, [(ohlcv_at(1.0), 3)]).unwrap();
        let candles = market.candles_for(AlignedWindow::Days1).unwrap();
        assert_eq!(
            candles[0].start_time_millis_inc(),
            millis_at(2024, 2, 15, 0, 0, 0)
        );
        assert_eq!(
            candles[0].end_time_millis_exc(),
            millis_at(2024, 2, 16, 0, 0, 0)
        );
    }

    #[test]
    fn hours12_rejects_hours_that_do_not_start_a_window() {
        assert!(MarketAlignedWindow::hours12(2024, 2, 15, 9, [(ohlcv_at(1.0), 3)]).is_err());

        let market = MarketAlignedWindow::hours12(2024, 2, 15, 12, [(ohlcv_at(1.0), 3)]).unwrap();
        let candles = market.candles_for(AlignedWindow::Hours12).unwrap();
        assert_eq!(
            candles[0].start_time_millis_inc(),
            millis_at(2024, 2, 15, 12, 0, 0)
        );
        assert_eq!(
            candles[0].end_time_millis_exc(),
            millis_at(2024, 2, 16, 0, 0, 0)
        );
    }

    #[test]
    fn minutes5_rejects_minutes_that_do_not_start_a_window() {
        assert!(MarketAlignedWindow::minutes5(2024, 2, 15, 12, 7, [(ohlcv_at(1.0), 3)]).is_err());

        let market =
            MarketAlignedWindow::minutes5(2024, 2, 15, 12, 5, [(ohlcv_at(1.0), 3)]).unwrap();
        let candles = market.candles_for(AlignedWindow::Minutes5).unwrap();
        assert_eq!(
            candles[0].start_time_millis_inc(),
            millis_at(2024, 2, 15, 12, 5, 0)
        );
        assert_eq!(
            candles[0].end_time_millis_exc(),
            millis_at(2024, 2, 15, 12, 10, 0)
        );
    }

    #[test]
    fn seconds30_rejects_seconds_that_do_not_start_a_window() {
        assert!(
            MarketAlignedWindow::seconds30(2024, 2, 15, 12, 30, 45, [(ohlcv_at(1.0), 3)]).is_err()
        );

        let market =
            MarketAlignedWindow::seconds30(2024, 2, 15, 12, 30, 30, [(ohlcv_at(1.0), 3)]).unwrap();
        let candles = market.candles_for(AlignedWindow::Seconds30).unwrap();
        assert_eq!(
            candles[0].start_time_millis_inc(),
            millis_at(2024, 2, 15, 12, 30, 30)
        );
        assert_eq!(
            candles[0].end_time_millis_exc(),
            millis_at(2024, 2, 15, 12, 31, 0)
        );
    }

    #[test]
    fn aligned_constructors_add_one_consecutive_window_per_entry() {
        let market =
            MarketAlignedWindow::years1(2024, [(ohlcv_at(1.0), 3), (ohlcv_at(2.0), 4)]).unwrap();
        let candles = market.candles_for(AlignedWindow::Years1).unwrap();

        assert_eq!(candles.len(), 2);
        assert_eq!(
            candles[0].start_time_millis_inc(),
            millis_at(2024, 1, 1, 0, 0, 0)
        );
        assert_eq!(
            candles[0].end_time_millis_exc(),
            millis_at(2025, 1, 1, 0, 0, 0)
        );
        assert_eq!(
            candles[1].start_time_millis_inc(),
            millis_at(2025, 1, 1, 0, 0, 0)
        );
        assert_eq!(
            candles[1].end_time_millis_exc(),
            millis_at(2026, 1, 1, 0, 0, 0)
        );
        assert_eq!(market.opens(AlignedWindow::Years1), Some(vec![1.0, 2.0]));
    }

    #[test]
    fn calendar_window_constructors_follow_calendar_lengths() {
        let market =
            MarketAlignedWindow::months1(2024, 1, [(ohlcv_at(1.0), 3), (ohlcv_at(2.0), 4)])
                .unwrap();
        let candles = market.candles_for(AlignedWindow::Months1).unwrap();

        assert_eq!(
            candles[0].end_time_millis_exc(),
            millis_at(2024, 2, 1, 0, 0, 0)
        );
        assert_eq!(
            candles[1].start_time_millis_inc(),
            millis_at(2024, 2, 1, 0, 0, 0)
        );
        assert_eq!(
            candles[1].end_time_millis_exc(),
            millis_at(2024, 3, 1, 0, 0, 0)
        );
    }

    #[test]
    fn every_aligned_constructor_covers_exactly_its_window() {
        let ohlcv = ohlcv_at(1.0);
        let cases = vec![
            (
                MarketAlignedWindow::seconds1(2024, 2, 15, 12, 30, 45, [(ohlcv.clone(), 3)])
                    .unwrap(),
                AlignedWindow::Seconds1,
            ),
            (
                MarketAlignedWindow::seconds5(2024, 2, 15, 12, 30, 45, [(ohlcv.clone(), 3)])
                    .unwrap(),
                AlignedWindow::Seconds5,
            ),
            (
                MarketAlignedWindow::seconds10(2024, 2, 15, 12, 30, 50, [(ohlcv.clone(), 3)])
                    .unwrap(),
                AlignedWindow::Seconds10,
            ),
            (
                MarketAlignedWindow::seconds15(2024, 2, 15, 12, 30, 45, [(ohlcv.clone(), 3)])
                    .unwrap(),
                AlignedWindow::Seconds15,
            ),
            (
                MarketAlignedWindow::seconds30(2024, 2, 15, 12, 30, 30, [(ohlcv.clone(), 3)])
                    .unwrap(),
                AlignedWindow::Seconds30,
            ),
            (
                MarketAlignedWindow::minutes1(2024, 2, 15, 12, 45, [(ohlcv.clone(), 3)]).unwrap(),
                AlignedWindow::Minutes1,
            ),
            (
                MarketAlignedWindow::minutes2(2024, 2, 15, 12, 44, [(ohlcv.clone(), 3)]).unwrap(),
                AlignedWindow::Minutes2,
            ),
            (
                MarketAlignedWindow::minutes3(2024, 2, 15, 12, 45, [(ohlcv.clone(), 3)]).unwrap(),
                AlignedWindow::Minutes3,
            ),
            (
                MarketAlignedWindow::minutes5(2024, 2, 15, 12, 45, [(ohlcv.clone(), 3)]).unwrap(),
                AlignedWindow::Minutes5,
            ),
            (
                MarketAlignedWindow::minutes10(2024, 2, 15, 12, 50, [(ohlcv.clone(), 3)]).unwrap(),
                AlignedWindow::Minutes10,
            ),
            (
                MarketAlignedWindow::minutes15(2024, 2, 15, 12, 45, [(ohlcv.clone(), 3)]).unwrap(),
                AlignedWindow::Minutes15,
            ),
            (
                MarketAlignedWindow::minutes30(2024, 2, 15, 12, 30, [(ohlcv.clone(), 3)]).unwrap(),
                AlignedWindow::Minutes30,
            ),
            (
                MarketAlignedWindow::hours1(2024, 2, 15, 12, [(ohlcv.clone(), 3)]).unwrap(),
                AlignedWindow::Hours1,
            ),
            (
                MarketAlignedWindow::hours2(2024, 2, 15, 12, [(ohlcv.clone(), 3)]).unwrap(),
                AlignedWindow::Hours2,
            ),
            (
                MarketAlignedWindow::hours3(2024, 2, 15, 12, [(ohlcv.clone(), 3)]).unwrap(),
                AlignedWindow::Hours3,
            ),
            (
                MarketAlignedWindow::hours4(2024, 2, 15, 12, [(ohlcv.clone(), 3)]).unwrap(),
                AlignedWindow::Hours4,
            ),
            (
                MarketAlignedWindow::hours6(2024, 2, 15, 12, [(ohlcv.clone(), 3)]).unwrap(),
                AlignedWindow::Hours6,
            ),
            (
                MarketAlignedWindow::hours8(2024, 2, 15, 16, [(ohlcv.clone(), 3)]).unwrap(),
                AlignedWindow::Hours8,
            ),
            (
                MarketAlignedWindow::hours12(2024, 2, 15, 12, [(ohlcv.clone(), 3)]).unwrap(),
                AlignedWindow::Hours12,
            ),
            (
                MarketAlignedWindow::days1(2024, 2, 15, [(ohlcv.clone(), 3)]).unwrap(),
                AlignedWindow::Days1,
            ),
            (
                MarketAlignedWindow::weeks1(2024, 2, 15, [(ohlcv.clone(), 3)]).unwrap(),
                AlignedWindow::Weeks1,
            ),
            (
                MarketAlignedWindow::weeks2(2024, 2, 15, [(ohlcv.clone(), 3)]).unwrap(),
                AlignedWindow::Weeks2,
            ),
            (
                MarketAlignedWindow::weeks4(2024, 2, 15, [(ohlcv.clone(), 3)]).unwrap(),
                AlignedWindow::Weeks4,
            ),
            (
                MarketAlignedWindow::months1(2024, 2, [(ohlcv.clone(), 3)]).unwrap(),
                AlignedWindow::Months1,
            ),
            (
                MarketAlignedWindow::months2(2024, 3, [(ohlcv.clone(), 3)]).unwrap(),
                AlignedWindow::Months2,
            ),
            (
                MarketAlignedWindow::months3(2024, 4, [(ohlcv.clone(), 3)]).unwrap(),
                AlignedWindow::Months3,
            ),
            (
                MarketAlignedWindow::months4(2024, 5, [(ohlcv.clone(), 3)]).unwrap(),
                AlignedWindow::Months4,
            ),
            (
                MarketAlignedWindow::months6(2024, 7, [(ohlcv.clone(), 3)]).unwrap(),
                AlignedWindow::Months6,
            ),
            (
                MarketAlignedWindow::years1(2024, [(ohlcv.clone(), 3)]).unwrap(),
                AlignedWindow::Years1,
            ),
            (
                MarketAlignedWindow::years2(2024, [(ohlcv.clone(), 3)]).unwrap(),
                AlignedWindow::Years2,
            ),
        ];

        for (market, window) in cases {
            let candles = market.candles_for(window).unwrap();
            assert_eq!(candles.len(), 1, "unexpected candles for {window}");
            if let Some(exact_millis) = window.millis_exact() {
                assert_eq!(
                    candles[0].duration_millis(),
                    exact_millis,
                    "unexpected duration for {window}"
                );
            }
        }
    }
}
