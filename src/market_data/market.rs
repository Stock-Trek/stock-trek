use crate::market_data::{
    aligned_window::AlignedWindow,
    market_aligned_window::MarketAlignedWindow,
    market_candle::MarketCandle,
    market_order_book::MarketOrderBook,
    market_quote::{MarketQuote, PriceQuantity},
    market_rolling_window::MarketRollingWindow,
    market_tick::MarketTick,
    market_ticks::MarketTicks,
    rolling_window::RollingWindow,
    timestamp::TimestampMillis,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Market data for a single trading pair on a CEX.
///
/// The increment and notional fields describe exchange constraints. Ticks,
/// rolling candles, aligned candles, and the order book hold the recent market
/// history.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Market {
    base_increment: f64,
    quote_increment: f64,
    minimum_notional: f64,
    ticks: MarketTicks,
    rolling: MarketRollingWindow,
    aligned: MarketAlignedWindow,
    order_book: MarketOrderBook,
}

impl Market {
    /// Returns the minimum increment for the base asset quantity.
    pub fn base_increment(&self) -> f64 {
        self.base_increment
    }

    /// Returns the minimum increment for the quote asset price.
    pub fn quote_increment(&self) -> f64 {
        self.quote_increment
    }

    /// Returns the minimum notional value for an order.
    pub fn minimum_notional(&self) -> f64 {
        self.minimum_notional
    }

    /// Returns the recent market ticks.
    pub fn ticks(&self) -> &MarketTicks {
        &self.ticks
    }

    /// Returns the candles for fixed-length rolling windows.
    pub fn rolling(&self) -> &MarketRollingWindow {
        &self.rolling
    }

    /// Returns the candles for aligned windows.
    pub fn aligned(&self) -> &MarketAlignedWindow {
        &self.aligned
    }

    /// Returns the current order book.
    pub fn order_book(&self) -> &MarketOrderBook {
        &self.order_book
    }
}

/// Builder for constructing a [`Market`].
///
/// New markets start empty. Add ticks, candles, and order book quotes before
/// calling [`MarketBuilder::build`].
#[derive(Debug, Clone)]
pub struct MarketBuilder {
    base_increment: f64,
    quote_increment: f64,
    minimum_notional: f64,
    ticks: Vec<MarketTick>,
    rolling: HashMap<RollingWindow, MarketCandle>,
    aligned: HashMap<AlignedWindow, Vec<MarketCandle>>,
    order_book_asks: Vec<MarketQuote>,
    order_book_bids: Vec<MarketQuote>,
}

impl MarketBuilder {
    /// Creates a builder with the market's trading increments and minimum notional.
    pub fn new(base_increment: f64, quote_increment: f64, minimum_notional: f64) -> Self {
        Self {
            base_increment,
            quote_increment,
            minimum_notional,
            ticks: Vec::new(),
            rolling: HashMap::new(),
            aligned: HashMap::new(),
            order_book_bids: Vec::new(),
            order_book_asks: Vec::new(),
        }
    }

    /// Returns the minimum increment for the base asset quantity.
    pub fn base_increment(&self) -> f64 {
        self.base_increment
    }

    /// Returns the minimum increment for the quote asset price.
    pub fn quote_increment(&self) -> f64 {
        self.quote_increment
    }

    /// Returns the minimum notional value for an order.
    pub fn minimum_notional(&self) -> f64 {
        self.minimum_notional
    }

    /// Returns the ticks added to the builder.
    pub fn ticks(&self) -> &[MarketTick] {
        &self.ticks
    }

    /// Returns the rolling-window candles added to the builder.
    pub fn rolling(&self) -> &HashMap<RollingWindow, MarketCandle> {
        &self.rolling
    }

    /// Returns the aligned-window candles added to the builder.
    pub fn aligned(&self) -> &HashMap<AlignedWindow, Vec<MarketCandle>> {
        &self.aligned
    }

    /// Returns the bid quotes added to the builder.
    pub fn order_book_bids(&self) -> &[MarketQuote] {
        &self.order_book_bids
    }

    /// Returns the ask quotes added to the builder.
    pub fn order_book_asks(&self) -> &[MarketQuote] {
        &self.order_book_asks
    }

    /// Adds a market tick with the given bid, ask, and last price/quantity pairs.
    pub fn tick(
        mut self,
        timestamp_millis: TimestampMillis,
        bid: PriceQuantity,
        ask: PriceQuantity,
        last: PriceQuantity,
    ) -> Self {
        let tick = MarketTick::new(
            timestamp_millis,
            MarketQuote::new(bid.0, bid.1),
            MarketQuote::new(ask.0, ask.1),
            MarketQuote::new(last.0, last.1),
        );
        self.ticks.push(tick);
        self
    }
    /// Sets the candle for a fixed-length rolling window.
    pub fn rolling_set_candle(mut self, window: RollingWindow, candle: MarketCandle) -> Self {
        self.rolling.insert(window, candle);
        self
    }
    /// Appends a candle to an aligned window.
    pub fn aligned_push_candle(mut self, window: AlignedWindow, candle: MarketCandle) -> Self {
        self.aligned.entry(window).or_default().push(candle);
        self
    }
    /// Adds a bid quote to the order book.
    pub fn order_book_bid(mut self, price: f64, quantity: f64) -> Self {
        self.order_book_bids.push(MarketQuote::new(price, quantity));
        self
    }
    /// Adds an ask quote to the order book.
    pub fn order_book_ask(mut self, price: f64, quantity: f64) -> Self {
        self.order_book_asks.push(MarketQuote::new(price, quantity));
        self
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::market_data::market_ohlcv::MarketOhlcv;

    fn candle(start_millis: u64) -> MarketCandle {
        MarketCandle::new(
            TimestampMillis::new(start_millis),
            TimestampMillis::new(start_millis + 1_000),
            TimestampMillis::new(1_000),
            true,
            MarketOhlcv::new(1.0, 2.0, 0.5, 1.5, 10.0, 15.0, 1.4),
            3,
        )
        .unwrap()
    }

    #[test]
    fn builder_methods_can_be_chained() {
        let market = MarketBuilder::new(0.000_001, 0.01, 1.0)
            .tick(
                TimestampMillis::new(1),
                (100.0, 1.0),
                (101.0, 2.0),
                (100.5, 3.0),
            )
            .rolling_set_candle(RollingWindow::Minutes1, candle(1_000))
            .aligned_push_candle(AlignedWindow::Minutes1, candle(1_000))
            .order_book_bid(100.0, 1.0)
            .order_book_ask(101.0, 2.0)
            .build();

        assert_eq!(market.base_increment(), 0.000_001);
        assert_eq!(market.quote_increment(), 0.01);
        assert_eq!(market.minimum_notional(), 1.0);
        assert_eq!(market.ticks().ticks().len(), 1);
        assert_eq!(market.rolling().candles().len(), 1);
        assert_eq!(market.aligned().candles().len(), 1);
        assert_eq!(market.order_book().bids().len(), 1);
        assert_eq!(market.order_book().asks().len(), 1);
    }
}
