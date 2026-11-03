//! Constructor macro shared by aligned-window collections.

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
                let start = Self::window_start_at(year, Month::January, 1, 0, 0, 0)?;
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
                month: Month,
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
                month: Month,
                day: u32,
                hour: u32,
                candles: impl IntoIterator<Item = (MarketOhlcv, u64)>,
            ) -> StockTrekResult<Self> {
                let start = Self::window_start_at(
                    year,
                    month,
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
                    month,
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
                    month,
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

pub(crate) use aligned_window_constructor;

/// Generates constructors that take any date within the first window instead
/// of the window's aligned start.
///
/// Each generated constructor takes a date, followed by an iterator of
/// `(OHLCV, trade count)` tuples, and adds one candle per tuple: the first
/// candle starts at the start of the window containing the given date, each
/// following candle starts where the previous one ends, and every created
/// candle is marked closed.
macro_rules! aligned_window_containing_constructor {
    ($($(#[$doc:meta])* $name:ident, $variant:ident);* $(;)?) => {
        $(
            $(#[$doc])*
            pub fn $name(
                year: i32,
                month: Month,
                day: u32,
                candles: impl IntoIterator<Item = (MarketOhlcv, u64)>,
            ) -> StockTrekResult<Self> {
                let requested = Self::window_start_at(year, month, day, 0, 0, 0)?;
                Self::with_window_containing(AlignedWindow::$variant, requested, candles)
            }
        )*
    };
}

pub(crate) use aligned_window_containing_constructor;
