use crate::{
    dto::raw_market::RawMarket,
    market_data::{
        market_aligned_window::MarketAlignedWindow, market_order_book::MarketOrderBook,
        market_rolling_window::MarketRollingWindow, market_ticks::MarketTicks,
    },
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

impl TryFrom<RawMarket> for Market {
    type Error = String;

    fn try_from(value: RawMarket) -> Result<Self, Self::Error> {
        let RawMarket {
            aligned: raw_aligned,
            base_increment,
            minimum_notional,
            order_book,
            quote_increment,
            rolling: raw_rolling,
            ticks,
        } = value;
        let rolling = MarketRollingWindow::try_from(raw_rolling)?;
        let aligned = MarketAlignedWindow::try_from(raw_aligned)?;
        Ok(Market {
            base_increment: Decimal::from(base_increment),
            quote_increment: Decimal::from(quote_increment),
            minimum_notional: Decimal::from(minimum_notional),
            ticks: MarketTicks::from(ticks),
            rolling,
            aligned,
            order_book: MarketOrderBook::from(order_book),
        })
    }
}
