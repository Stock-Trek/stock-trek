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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Market {
    pub base_increment: f64,
    pub quote_increment: f64,
    pub minimum_notional: f64,
    pub ticks: MarketTicks,
    pub rolling: MarketRollingWindow,
    pub aligned: MarketAlignedWindow,
    pub order_book: MarketOrderBook,
}

pub struct Builder {
    pub base_increment: f64,
    pub quote_increment: f64,
    pub minimum_notional: f64,
    pub ticks: Vec<MarketTick>,
    pub rolling: HashMap<RollingWindow, MarketCandle>,
    pub aligned: HashMap<AlignedWindow, Vec<MarketCandle>>,
    pub order_book_asks: Vec<MarketQuote>,
    pub order_book_bids: Vec<MarketQuote>,
}

impl Builder {
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
    pub fn rolling_set_candle(&mut self, window: RollingWindow, candle: MarketCandle) -> &mut Self {
        self.rolling.insert(window, candle);
        self
    }
    pub fn aligned_push_candle(
        &mut self,
        window: AlignedWindow,
        candle: MarketCandle,
    ) -> &mut Self {
        self.aligned.entry(window).or_default().push(candle);
        self
    }
    pub fn order_book_bid(&mut self, price: f64, quantity: f64) -> &mut Self {
        self.order_book_bids.push(MarketQuote { price, quantity });
        self
    }
    pub fn order_book_ask(&mut self, price: f64, quantity: f64) -> &mut Self {
        self.order_book_asks.push(MarketQuote { price, quantity });
        self
    }
    pub fn build(&self) -> Market {
        Market {
            base_increment: self.base_increment,
            quote_increment: self.quote_increment,
            minimum_notional: self.minimum_notional,
            ticks: MarketTicks::new(self.ticks.clone()),
            rolling: MarketRollingWindow::new(self.rolling.clone()),
            aligned: MarketAlignedWindow::new(self.aligned.clone()),
            order_book: MarketOrderBook::new(
                self.order_book_bids.clone(),
                self.order_book_asks.clone(),
            ),
        }
    }
}
