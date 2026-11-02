use super::market::Market;
use crate::{
    error::result::StockTrekResult,
    market_data::{
        aligned_window::AlignedWindow, market_aligned_window::MarketAlignedWindow,
        market_candle::MarketCandle, market_order_book::MarketOrderBook, market_quote::MarketQuote,
        market_rolling_window::MarketRollingWindow, market_tick::MarketTick,
        market_ticks::MarketTicks, rolling_window::RollingWindow, timestamp::TimestampMillis,
    },
};
use std::collections::BTreeMap;

/// Builder for constructing a [`Market`].
///
/// New markets start empty. Add ticks, candles, and order book quotes before
/// calling [`MarketBuilder::build`]. Invalid parameters and order book quotes
/// are rejected immediately by the method that receives them.
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
    #[must_use]
    pub fn tick(
        mut self,
        timestamp_millis: TimestampMillis,
        bid: MarketQuote,
        ask: MarketQuote,
        last: MarketQuote,
    ) -> Self {
        self.ticks
            .push(MarketTick::new(timestamp_millis, bid, ask, last));
        self
    }
    /// Sets the candle for a fixed-length rolling window.
    #[must_use]
    pub fn rolling_set_candle(mut self, window: RollingWindow, candle: MarketCandle) -> Self {
        self.rolling.insert(window, candle);
        self
    }
    /// Appends a candle to an aligned window.
    #[must_use]
    pub fn aligned_push_candle(mut self, window: AlignedWindow, candle: MarketCandle) -> Self {
        self.aligned.entry(window).or_default().push(candle);
        self
    }
    /// Adds a bid quote to the order book.
    ///
    /// Returns an error unless both values are finite and positive.
    pub fn order_book_bid(mut self, price: f64, quantity: f64) -> StockTrekResult<Self> {
        self.order_book_bids
            .push(MarketQuote::new(price, quantity)?);
        Ok(self)
    }
    /// Adds an ask quote to the order book.
    ///
    /// Returns an error unless both values are finite and positive.
    pub fn order_book_ask(mut self, price: f64, quantity: f64) -> StockTrekResult<Self> {
        self.order_book_asks
            .push(MarketQuote::new(price, quantity)?);
        Ok(self)
    }
    /// Builds the [`Market`] from the collected data.
    pub fn build(self) -> Market {
        Market {
            base_increment: self.base_increment,
            quote_increment: self.quote_increment,
            minimum_notional: self.minimum_notional,
            ticks: MarketTicks::new(self.ticks),
            rolling: MarketRollingWindow::new(self.rolling),
            aligned: MarketAlignedWindow::new(self.aligned),
            order_book: MarketOrderBook::new(self.order_book_bids, self.order_book_asks),
        }
    }
}
