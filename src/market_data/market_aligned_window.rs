use crate::{
    error::result::StockTrekResult,
    market_data::{
        aligned_window::AlignedWindow, market_candle::MarketCandle, market_ohlcv::MarketOhlcv,
        timestamp::TimestampMillis,
    },
    validation::Validation,
};
use chrono::{Month, TimeZone, Utc};
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
                month: Month,
                candles: impl IntoIterator<Item = (MarketOhlcv, u64)>,
            ) -> StockTrekResult<Self> {
                let start = Self::window_start_at(year, month.number_from_month(), 1, 0, 0, 0)?;
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
                month: Month,
                day: u32,
                candles: impl IntoIterator<Item = (MarketOhlcv, u64)>,
            ) -> StockTrekResult<Self> {
                let start = Self::window_start_at(year, month.number_from_month(), day, 0, 0, 0)?;
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
                month: Month,
                day: u32,
                hour: u32,
                candles: impl IntoIterator<Item = (MarketOhlcv, u64)>,
            ) -> StockTrekResult<Self> {
                let start = Self::window_start_at(
                    year,
                    month.number_from_month(),
                    day,
                    hour,
                    0,
                    0,
                )?;
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
                month: Month,
                day: u32,
                hour: u32,
                minute: u32,
                candles: impl IntoIterator<Item = (MarketOhlcv, u64)>,
            ) -> StockTrekResult<Self> {
                let start = Self::window_start_at(
                    year,
                    month.number_from_month(),
                    day,
                    hour,
                    minute,
                    0,
                )?;
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
                month: Month,
                day: u32,
                hour: u32,
                minute: u32,
                second: u32,
                candles: impl IntoIterator<Item = (MarketOhlcv, u64)>,
            ) -> StockTrekResult<Self> {
                let start = Self::window_start_at(
                    year,
                    month.number_from_month(),
                    day,
                    hour,
                    minute,
                    second,
                )?;
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
            Self::validate_intervals("MarketAlignedWindow::new", window, &window_candles)?;
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

    pub(super) fn validate_intervals(
        function: &'static str,
        window: AlignedWindow,
        candles: &[MarketCandle],
    ) -> StockTrekResult<()> {
        let mut intervals = BTreeSet::new();
        for candle in candles {
            let start = candle.start_time_millis_inc();
            let end = candle.end_time_millis_exc();
            if !intervals.insert((start, end)) {
                return Err(Validation::error(
                    function,
                    format!("duplicate candle interval {start}..{end} for the {window} window"),
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
        let bounds = window.bounds_for(start).map_err(|error| {
            Validation::error(
                "MarketAlignedWindow::aligned_candle",
                format!("could not determine the {window} window bounds at {start}: {error}"),
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
        MarketCandle::from_interval(start, bounds.upper(), true, ohlcv, trade_count)
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
