use crate::{
    error::result::{StockTrekError, StockTrekResult},
    market_data::{
        market_aligned_window::MarketAlignedWindow, market_order_book::MarketOrderBook,
        market_rolling_window::MarketRollingWindow, market_ticks::MarketTicks,
    },
    validation::Validation,
};
use serde::{Deserialize, Serialize};

pub use crate::market_data::market_builder::MarketBuilder;

/// Market data for a single trading pair on a CEX.
///
/// The increment and notional fields describe exchange constraints. Ticks,
/// rolling candles, aligned candles, and the order book hold the recent market
/// history.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "MarketData")]
pub struct Market {
    pub(super) base_increment: f64,
    pub(super) quote_increment: f64,
    pub(super) minimum_notional: f64,
    pub(super) ticks: MarketTicks,
    pub(super) rolling: MarketRollingWindow,
    pub(super) aligned: MarketAlignedWindow,
    pub(super) order_book: MarketOrderBook,
}

impl Market {
    pub(super) fn validate_parameters(
        function: &'static str,
        base_increment: f64,
        quote_increment: f64,
        minimum_notional: f64,
    ) -> StockTrekResult<()> {
        Validation::positive(function, "base increment", base_increment)?;
        Validation::positive(function, "quote increment", quote_increment)?;
        Validation::non_negative(function, "minimum notional", minimum_notional)?;
        Ok(())
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

#[derive(Deserialize)]
struct MarketData {
    base_increment: f64,
    quote_increment: f64,
    minimum_notional: f64,
    ticks: MarketTicks,
    rolling: MarketRollingWindow,
    aligned: MarketAlignedWindow,
    order_book: MarketOrderBook,
}

impl TryFrom<MarketData> for Market {
    type Error = StockTrekError;

    fn try_from(data: MarketData) -> Result<Self, Self::Error> {
        let mut builder = MarketBuilder::new(
            data.base_increment,
            data.quote_increment,
            data.minimum_notional,
        );

        for tick in data.ticks.ticks() {
            builder.tick(
                tick.timestamp_millis(),
                tick.bid().clone(),
                tick.ask().clone(),
                tick.last().clone(),
            );
        }
        for (window, candle) in data.rolling.candles() {
            builder.rolling_set_candle(*window, candle.clone());
        }
        for (window, candles) in data.aligned.candles() {
            for candle in candles {
                builder.aligned_push_candle(*window, candle.clone());
            }
        }
        for quote in data.order_book.bids() {
            builder.order_book_bid(quote.clone());
        }
        for quote in data.order_book.asks() {
            builder.order_book_ask(quote.clone());
        }

        builder.build()
    }
}
