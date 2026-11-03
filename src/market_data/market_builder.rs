use super::market::Market;
use crate::{
    error::result::StockTrekResult,
    market_data::{
        aligned_window::AlignedWindow, market_aligned_window::MarketAlignedWindow,
        market_candle::MarketCandle, market_order_book::MarketOrderBook, market_quote::MarketQuote,
        market_rolling_window::MarketRollingWindow, market_tick::MarketTick,
        market_ticks::MarketTicks, rolling_window::RollingWindow, timestamp::TimestampMillis,
    },
    validation::Validation,
};
use std::collections::BTreeMap;

const INCREMENT_TOLERANCE: f64 = 1e-9;
const INCREMENT_TOLERANCE_ULPS: f64 = 4.0;
const MAX_INCREMENT_TOLERANCE: f64 = 0.25;

/// Builder for constructing a [`Market`].
///
/// New markets start empty. Add ticks, candles, and order book quotes before
/// calling [`MarketBuilder::build`]. All validation happens at build time:
/// [`MarketBuilder::build`] rejects invalid parameters, quotes, candles, and
/// order books.
#[derive(Debug, Clone)]
pub struct MarketBuilder {
    base_increment: f64,
    quote_increment: f64,
    minimum_notional: f64,
    ticks: Vec<(TimestampMillis, MarketQuote, MarketQuote, MarketQuote)>,
    rolling: BTreeMap<RollingWindow, MarketCandle>,
    aligned: BTreeMap<AlignedWindow, Vec<MarketCandle>>,
    order_book_asks: Vec<MarketQuote>,
    order_book_bids: Vec<MarketQuote>,
}

impl MarketBuilder {
    /// Creates a builder with the market's trading increments and minimum notional.
    ///
    /// The values are validated when [`MarketBuilder::build`] is called.
    pub fn new(base_increment: f64, quote_increment: f64, minimum_notional: f64) -> Self {
        Self {
            base_increment,
            quote_increment,
            minimum_notional,
            ticks: Vec::new(),
            rolling: BTreeMap::new(),
            aligned: BTreeMap::new(),
            order_book_bids: Vec::new(),
            order_book_asks: Vec::new(),
        }
    }

    /// Adds a market tick with the given bid, ask, and last quotes.
    ///
    /// The quotes are validated when [`MarketBuilder::build`] is called: each
    /// quote must respect the market's increments, the bid and ask must meet
    /// the minimum notional, and the bid must be below the ask. The last trade
    /// quote is not subject to the order minimum notional.
    pub fn tick(
        &mut self,
        timestamp_millis: TimestampMillis,
        bid: MarketQuote,
        ask: MarketQuote,
        last: MarketQuote,
    ) -> &mut Self {
        self.ticks.push((timestamp_millis, bid, ask, last));
        self
    }

    /// Sets the candle for a fixed-length rolling window.
    ///
    /// The candle is validated when [`MarketBuilder::build`] is called: its
    /// duration must match the window length.
    pub fn rolling_set_candle(&mut self, window: RollingWindow, candle: MarketCandle) -> &mut Self {
        self.rolling.insert(window, candle);
        self
    }

    /// Appends a candle to an aligned window.
    ///
    /// The candle is validated when [`MarketBuilder::build`] is called: it
    /// must exactly cover the window that contains its start time and its
    /// interval must not already be present in the window.
    pub fn aligned_push_candle(
        &mut self,
        window: AlignedWindow,
        candle: MarketCandle,
    ) -> &mut Self {
        self.aligned.entry(window).or_default().push(candle);
        self
    }

    /// Adds a bid quote to the order book.
    ///
    /// The quote and the finished order book are validated when
    /// [`MarketBuilder::build`] is called: the quote must respect the market's
    /// increments and minimum notional, and every bid must be below every ask.
    pub fn order_book_bid(&mut self, quote: MarketQuote) -> &mut Self {
        self.order_book_bids.push(quote);
        self
    }

    /// Adds an ask quote to the order book.
    ///
    /// The quote and the finished order book are validated when
    /// [`MarketBuilder::build`] is called: the quote must respect the market's
    /// increments and minimum notional, and every bid must be below every ask.
    pub fn order_book_ask(&mut self, quote: MarketQuote) -> &mut Self {
        self.order_book_asks.push(quote);
        self
    }

    /// Builds the [`Market`] from the collected data.
    ///
    /// Returns an error unless the market parameters are valid, every quote
    /// respects the market's increments and notional rules, every bid is below
    /// every ask, and every candle matches its window.
    pub fn build(self) -> StockTrekResult<Market> {
        let function = "MarketBuilder::build";
        Market::validate_parameters(
            function,
            self.base_increment,
            self.quote_increment,
            self.minimum_notional,
        )?;

        let mut ticks = Vec::with_capacity(self.ticks.len());
        for (timestamp_millis, bid, ask, last) in &self.ticks {
            Self::validate_quote(
                function,
                "bid",
                bid,
                self.base_increment,
                self.quote_increment,
                Some(self.minimum_notional),
            )?;
            Self::validate_quote(
                function,
                "ask",
                ask,
                self.base_increment,
                self.quote_increment,
                Some(self.minimum_notional),
            )?;
            Self::validate_quote(
                function,
                "last",
                last,
                self.base_increment,
                self.quote_increment,
                None,
            )?;
            ticks.push(MarketTick::new(
                *timestamp_millis,
                bid.clone(),
                ask.clone(),
                last.clone(),
            )?);
        }

        for (window, candle) in &self.rolling {
            MarketRollingWindow::validate_candle_duration(function, *window, candle)?;
        }

        for (window, candles) in &self.aligned {
            for candle in candles {
                let start = candle.start_time_millis_inc();
                let end = candle.end_time_millis_exc();
                let bounds = window.bounds_for(start).map_err(|error| {
                    Validation::error(
                        function,
                        format!(
                            "could not determine the {window} window bounds at {start}: {error}"
                        ),
                    )
                })?;
                if start != bounds.lower() || end != bounds.upper() {
                    return Err(Validation::error(
                        function,
                        format!(
                            "candle interval {start}..{end} does not match the {window} window bounds {}..{}",
                            bounds.lower(),
                            bounds.upper()
                        ),
                    ));
                }
            }
            MarketAlignedWindow::validate_intervals(function, *window, candles)?;
        }

        for quote in &self.order_book_bids {
            Self::validate_quote(
                function,
                "order book bid",
                quote,
                self.base_increment,
                self.quote_increment,
                Some(self.minimum_notional),
            )?;
        }
        for quote in &self.order_book_asks {
            Self::validate_quote(
                function,
                "order book ask",
                quote,
                self.base_increment,
                self.quote_increment,
                Some(self.minimum_notional),
            )?;
        }
        let order_book = MarketOrderBook::new(self.order_book_bids, self.order_book_asks)?;

        Ok(Market {
            base_increment: self.base_increment,
            quote_increment: self.quote_increment,
            minimum_notional: self.minimum_notional,
            ticks: MarketTicks::new(ticks),
            rolling: MarketRollingWindow {
                candles: self.rolling,
            },
            aligned: MarketAlignedWindow::from_map(self.aligned),
            order_book,
        })
    }

    fn validate_quote(
        function: &'static str,
        field: &str,
        quote: &MarketQuote,
        base_increment: f64,
        quote_increment: f64,
        minimum_notional: Option<f64>,
    ) -> StockTrekResult<()> {
        if !Self::is_multiple_of(quote.price(), quote_increment) {
            return Err(Validation::error(
                function,
                format!(
                    "{field} price {} must be a multiple of the quote increment {quote_increment}",
                    quote.price()
                ),
            ));
        }
        if !Self::is_multiple_of(quote.quantity(), base_increment) {
            return Err(Validation::error(
                function,
                format!(
                    "{field} quantity {} must be a multiple of the base increment {base_increment}",
                    quote.quantity()
                ),
            ));
        }

        let notional = quote.price() * quote.quantity();
        if !notional.is_finite() {
            return Err(Validation::error(
                function,
                format!("{field} notional {notional} must be finite"),
            ));
        }
        if let Some(minimum_notional) = minimum_notional
            && notional < minimum_notional
        {
            return Err(Validation::error(
                function,
                format!(
                    "{field} notional {notional} must be at least the minimum notional {minimum_notional}"
                ),
            ));
        }
        Ok(())
    }

    fn is_multiple_of(value: f64, increment: f64) -> bool {
        let steps = value / increment;
        if !steps.is_finite() {
            return false;
        }
        let tolerance = (INCREMENT_TOLERANCE_ULPS * f64::EPSILON * steps.abs())
            .clamp(INCREMENT_TOLERANCE, MAX_INCREMENT_TOLERANCE);
        (steps - steps.round()).abs() <= tolerance
    }
}
