use crate::{
    market_data::{extract::dec_to_f64, market_quote::MarketQuote, market_tick::MarketTick},
    prelude::TimestampMillis,
};
use std::sync::OnceLock;

#[derive(Debug)]
pub struct MarketTicks {
    ticks: Vec<MarketTick>,
    bids: OnceLock<Vec<(TimestampMillis, f64, f64)>>,
    asks: OnceLock<Vec<(TimestampMillis, f64, f64)>>,
    lasts: OnceLock<Vec<(TimestampMillis, f64, f64)>>,
}

impl MarketTicks {
    pub fn new(ticks: Vec<MarketTick>) -> Self {
        Self {
            ticks,
            bids: OnceLock::new(),
            asks: OnceLock::new(),
            lasts: OnceLock::new(),
        }
    }
    pub fn ticks(&self) -> &Vec<MarketTick> {
        &self.ticks
    }
    pub fn bids(&self) -> &Vec<(TimestampMillis, f64, f64)> {
        self.bids.get_or_init(|| self.values(|tick| tick.bid()))
    }
    pub fn asks(&self) -> &Vec<(TimestampMillis, f64, f64)> {
        self.asks.get_or_init(|| self.values(|tick| tick.ask()))
    }
    pub fn lasts(&self) -> &Vec<(TimestampMillis, f64, f64)> {
        self.lasts.get_or_init(|| self.values(|tick| tick.last()))
    }

    fn values(
        &self,
        to_quote: impl Fn(&MarketTick) -> &MarketQuote,
    ) -> Vec<(TimestampMillis, f64, f64)> {
        self.ticks
            .iter()
            .map(|tick| self.to_tick_tuple(tick, &to_quote))
            .collect()
    }
    fn to_tick_tuple(
        &self,
        tick: &MarketTick,
        to_quote: impl Fn(&MarketTick) -> &MarketQuote,
    ) -> (TimestampMillis, f64, f64) {
        let timestamp = tick.timestamp_millis();
        let quote = to_quote(tick);
        let price = dec_to_f64(quote.price());
        let quantity = dec_to_f64(quote.quantity());
        (timestamp, price, quantity)
    }
}
