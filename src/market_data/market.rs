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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Market {
    /// The minimum increment for the base asset quantity.
    pub base_increment: f64,
    /// The minimum increment for the quote asset price.
    pub quote_increment: f64,
    /// The minimum notional value for an order.
    pub minimum_notional: f64,
    /// Recent market ticks.
    pub ticks: MarketTicks,
    /// Candles for fixed-length rolling windows.
    pub rolling: MarketRollingWindow,
    /// Candles for calendar-aligned windows.
    pub aligned: MarketAlignedWindow,
    /// The current order book.
    pub order_book: MarketOrderBook,
}

/// Builder for constructing a [`Market`].
#[derive(Debug, Clone)]
pub struct MarketBuilder {
    /// The minimum increment for the base asset quantity.
    pub base_increment: f64,
    /// The minimum increment for the quote asset price.
    pub quote_increment: f64,
    /// The minimum notional value for an order.
    pub minimum_notional: f64,
    /// Ticks to include in the market.
    pub ticks: Vec<MarketTick>,
    /// Rolling-window candles to include in the market.
    pub rolling: HashMap<RollingWindow, MarketCandle>,
    /// Aligned-window candles to include in the market.
    pub aligned: HashMap<AlignedWindow, Vec<MarketCandle>>,
    /// Order book ask quotes.
    pub order_book_asks: Vec<MarketQuote>,
    /// Order book bid quotes.
    pub order_book_bids: Vec<MarketQuote>,
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
    /// Adds a market tick with the given bid, ask, and last price/quantity pairs.
    pub fn tick(
        &mut self,
        timestamp_millis: TimestampMillis,
        bid: PriceQuantity,
        ask: PriceQuantity,
        last: PriceQuantity,
    ) -> &mut Self {
        let tick = MarketTick {
            timestamp_millis,
            bid: MarketQuote {
                price: bid.0,
                quantity: bid.1,
            },
            ask: MarketQuote {
                price: ask.0,
                quantity: ask.1,
            },
            last: MarketQuote {
                price: last.0,
                quantity: last.1,
            },
        };
        self.ticks.push(tick);
        self
    }
    /// Sets the candle for a fixed-length rolling window.
    pub fn rolling_set_candle(&mut self, window: RollingWindow, candle: MarketCandle) -> &mut Self {
        self.rolling.insert(window, candle);
        self
    }
    /// Appends a candle to a calendar-aligned window.
    pub fn aligned_push_candle(
        &mut self,
        window: AlignedWindow,
        candle: MarketCandle,
    ) -> &mut Self {
        self.aligned.entry(window).or_default().push(candle);
        self
    }
    /// Adds a bid quote to the order book.
    pub fn order_book_bid(&mut self, price: f64, quantity: f64) -> &mut Self {
        self.order_book_bids.push(MarketQuote { price, quantity });
        self
    }
    /// Adds an ask quote to the order book.
    pub fn order_book_ask(&mut self, price: f64, quantity: f64) -> &mut Self {
        self.order_book_asks.push(MarketQuote { price, quantity });
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
