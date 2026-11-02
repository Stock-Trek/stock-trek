use crate::{
    error::result::{StockTrekError, StockTrekResult},
    market_data::{
        market_aligned_window::MarketAlignedWindow, market_order_book::MarketOrderBook,
        market_rolling_window::MarketRollingWindow, market_ticks::MarketTicks,
    },
    validation::Validation,
};
use serde::{Deserialize, Serialize};

mod market_builder;

pub use market_builder::MarketBuilder;

/// Market data for a single trading pair on a CEX.
///
/// The increment and notional fields describe exchange constraints. Ticks,
/// rolling candles, aligned candles, and the order book hold the recent market
/// history.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "MarketData")]
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
    fn validate_parameters(
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
        Self::validate_parameters(
            "Market",
            data.base_increment,
            data.quote_increment,
            data.minimum_notional,
        )?;
        Ok(Self {
            base_increment: data.base_increment,
            quote_increment: data.quote_increment,
            minimum_notional: data.minimum_notional,
            ticks: data.ticks,
            rolling: data.rolling,
            aligned: data.aligned,
            order_book: data.order_book,
        })
    }
}

#[cfg(test)]
mod tests;
