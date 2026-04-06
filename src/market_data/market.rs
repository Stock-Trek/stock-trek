use crate::market_data::{
    market_aligned_window::MarketAlignedWindow, market_order_book::MarketOrderBook,
    market_rolling_window::MarketRollingWindow, market_ticks::MarketTicks,
};
use rust_decimal::Decimal;

#[derive(Debug)]
pub struct Market {
    base_increment: Decimal,
    quote_increment: Decimal,
    minimum_notional: Decimal,
    ticks: MarketTicks,
    rolling: MarketRollingWindow,
    aligned: MarketAlignedWindow,
    order_book: MarketOrderBook,
}

impl Market {
    pub fn new(
        base_increment: Decimal,
        quote_increment: Decimal,
        minimum_notional: Decimal,
        ticks: MarketTicks,
        rolling: MarketRollingWindow,
        aligned: MarketAlignedWindow,
        order_book: MarketOrderBook,
    ) -> Self {
        Self {
            base_increment,
            quote_increment,
            minimum_notional,
            ticks,
            rolling,
            aligned,
            order_book,
        }
    }
    pub fn base_increment(&self) -> Decimal {
        self.base_increment
    }
    pub fn quote_increment(&self) -> Decimal {
        self.quote_increment
    }
    pub fn minimum_notional(&self) -> Decimal {
        self.minimum_notional
    }
    pub fn ticks(&self) -> &MarketTicks {
        &self.ticks
    }
    pub fn rolling(&self) -> &MarketRollingWindow {
        &self.rolling
    }
    pub fn aligned(&self) -> &MarketAlignedWindow {
        &self.aligned
    }
    pub fn order_book(&self) -> &MarketOrderBook {
        &self.order_book
    }
}
