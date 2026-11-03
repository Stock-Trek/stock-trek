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

/// Builder for constructing a [`Market`].
///
/// New markets start empty. Add ticks, candles, and order book quotes before
/// calling [`MarketBuilder::build`]. Invalid parameters, quotes, candles, and
/// order books are rejected immediately by the method that receives them.
/// Setter methods borrow the builder mutably, so a rejected value can be
/// corrected and retried without rebuilding the collected data.
#[derive(Debug, Clone)]
pub struct MarketBuilder {
    base_increment: f64,
    quote_increment: f64,
    minimum_notional: f64,
    ticks: Vec<MarketTick>,
    rolling: BTreeMap<RollingWindow, MarketCandle>,
    aligned: BTreeMap<AlignedWindow, Vec<MarketCandle>>,
    order_book_asks: Vec<MarketQuote>,
    order_book_bids: Vec<MarketQuote>,
}

impl MarketBuilder {
    /// Creates a builder with the market's trading increments and minimum notional.
    ///
    /// Returns an error unless the increments are finite and positive and the
    /// minimum notional is finite and non-negative.
    pub fn new(
        base_increment: f64,
        quote_increment: f64,
        minimum_notional: f64,
    ) -> StockTrekResult<Self> {
        Market::validate_parameters(
            "MarketBuilder::new",
            base_increment,
            quote_increment,
            minimum_notional,
        )?;
        Ok(Self {
            base_increment,
            quote_increment,
            minimum_notional,
            ticks: Vec::new(),
            rolling: BTreeMap::new(),
            aligned: BTreeMap::new(),
            order_book_bids: Vec::new(),
            order_book_asks: Vec::new(),
        })
    }

    /// Adds a market tick with the given bid, ask, and last quotes.
    ///
    /// Returns an error if a quote does not respect the market's increments, if
    /// the bid or ask is below the minimum notional, or if the bid is not below
    /// the ask. The last trade quote is not subject to the order minimum
    /// notional.
    pub fn tick(
        &mut self,
        timestamp_millis: TimestampMillis,
        bid: MarketQuote,
        ask: MarketQuote,
        last: MarketQuote,
    ) -> StockTrekResult<&mut Self> {
        let function = "MarketBuilder::tick";
        Self::validate_quote(
            function,
            "bid",
            &bid,
            self.base_increment,
            self.quote_increment,
            Some(self.minimum_notional),
        )?;
        Self::validate_quote(
            function,
            "ask",
            &ask,
            self.base_increment,
            self.quote_increment,
            Some(self.minimum_notional),
        )?;
        Self::validate_quote(
            function,
            "last",
            &last,
            self.base_increment,
            self.quote_increment,
            None,
        )?;
        Self::validate_bid_and_ask(function, bid.price(), ask.price())?;

        self.ticks
            .push(MarketTick::new(timestamp_millis, bid, ask, last));
        Ok(self)
    }

    /// Sets the candle for a fixed-length rolling window.
    ///
    /// Returns an error unless the candle duration matches the window length.
    pub fn rolling_set_candle(
        &mut self,
        window: RollingWindow,
        candle: MarketCandle,
    ) -> StockTrekResult<&mut Self> {
        let expected_duration = window.millis_exact();
        let actual_duration = candle.duration_millis();
        if actual_duration != expected_duration {
            return Err(Validation::error(
                "MarketBuilder::rolling_set_candle",
                format!(
                    "candle duration {actual_duration} ms does not match the {window} window duration {expected_duration} ms"
                ),
            ));
        }

        self.rolling.insert(window, candle);
        Ok(self)
    }

    /// Appends a candle to an aligned window.
    ///
    /// Returns an error unless the candle exactly covers the window that
    /// contains its start time.
    pub fn aligned_push_candle(
        &mut self,
        window: AlignedWindow,
        candle: MarketCandle,
    ) -> StockTrekResult<&mut Self> {
        let function = "MarketBuilder::aligned_push_candle";
        let start = candle.start_time_millis_inc();
        let bounds = window.bounds_for(start).map_err(|error| {
            Validation::error(
                function,
                format!("could not determine the {window} window bounds at {start}: {error}"),
            )
        })?;
        if start != bounds.lower() || candle.end_time_millis_exc() != bounds.upper() {
            return Err(Validation::error(
                function,
                format!(
                    "candle interval {start}..{} does not match the {window} window bounds {}..{}",
                    candle.end_time_millis_exc(),
                    bounds.lower(),
                    bounds.upper()
                ),
            ));
        }

        self.aligned.entry(window).or_default().push(candle);
        Ok(self)
    }

    /// Adds a validated bid quote to the order book.
    ///
    /// Returns an error unless the quote respects the market's increments and
    /// minimum notional and its price is below every existing ask.
    pub fn order_book_bid(&mut self, quote: MarketQuote) -> StockTrekResult<&mut Self> {
        let function = "MarketBuilder::order_book_bid";
        Self::validate_quote(
            function,
            "order book bid",
            &quote,
            self.base_increment,
            self.quote_increment,
            Some(self.minimum_notional),
        )?;
        if let Some(lowest_ask) = Self::lowest_price(&self.order_book_asks) {
            Self::validate_bid_and_ask(function, quote.price(), lowest_ask)?;
        }

        self.order_book_bids.push(quote);
        Ok(self)
    }

    /// Adds a validated ask quote to the order book.
    ///
    /// Returns an error unless the quote respects the market's increments and
    /// minimum notional and its price is above every existing bid.
    pub fn order_book_ask(&mut self, quote: MarketQuote) -> StockTrekResult<&mut Self> {
        let function = "MarketBuilder::order_book_ask";
        Self::validate_quote(
            function,
            "order book ask",
            &quote,
            self.base_increment,
            self.quote_increment,
            Some(self.minimum_notional),
        )?;
        if let Some(highest_bid) = Self::highest_price(&self.order_book_bids) {
            Self::validate_bid_and_ask(function, highest_bid, quote.price())?;
        }

        self.order_book_asks.push(quote);
        Ok(self)
    }

    /// Builds the [`Market`] from the collected data.
    pub fn build(self) -> Market {
        Market {
            base_increment: self.base_increment,
            quote_increment: self.quote_increment,
            minimum_notional: self.minimum_notional,
            ticks: MarketTicks::new(self.ticks),
            rolling: MarketRollingWindow {
                candles: self.rolling,
            },
            aligned: MarketAlignedWindow::from_map(self.aligned),
            order_book: MarketOrderBook::new(self.order_book_bids, self.order_book_asks),
        }
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
        steps.is_finite()
            && (steps - steps.round()).abs() <= INCREMENT_TOLERANCE * steps.abs().max(1.0)
    }

    fn validate_bid_and_ask(
        function: &'static str,
        bid_price: f64,
        ask_price: f64,
    ) -> StockTrekResult<()> {
        if bid_price >= ask_price {
            return Err(Validation::error(
                function,
                format!("bid price {bid_price} must be below ask price {ask_price}"),
            ));
        }
        Ok(())
    }

    fn lowest_price(quotes: &[MarketQuote]) -> Option<f64> {
        quotes.iter().map(MarketQuote::price).min_by(f64::total_cmp)
    }

    fn highest_price(quotes: &[MarketQuote]) -> Option<f64> {
        quotes.iter().map(MarketQuote::price).max_by(f64::total_cmp)
    }
}
