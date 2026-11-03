use crate::market_data::{
    market_quote::{MarketQuote, TimedPriceQuantity},
    market_tick::MarketTick,
};
use serde::{Deserialize, Deserializer, Serialize};

/// A collection of recent market ticks.
///
/// Ticks are kept in chronological order, regardless of the order in which they
/// are supplied.
#[derive(Debug, Clone, Serialize)]
pub struct MarketTicks {
    ticks: Vec<MarketTick>,
}

impl MarketTicks {
    /// Creates a tick collection from a list of ticks.
    ///
    /// Ticks are sorted into chronological order.
    pub fn new(mut ticks: Vec<MarketTick>) -> Self {
        ticks.sort_by_key(|tick| tick.timestamp_millis());
        Self { ticks }
    }
    /// Returns all ticks in chronological order.
    pub fn ticks(&self) -> &[MarketTick] {
        &self.ticks
    }
    /// Returns the most recent tick, if there is one.
    pub fn latest(&self) -> Option<&MarketTick> {
        self.ticks.last()
    }
    /// Returns the timestamped price and quantity for each tick's best bid.
    pub fn bid_timed_price_quantities(&self) -> Vec<TimedPriceQuantity> {
        self.values(MarketTick::bid)
    }
    /// Returns the timestamped price and quantity for each tick's best ask.
    pub fn ask_timed_price_quantities(&self) -> Vec<TimedPriceQuantity> {
        self.values(MarketTick::ask)
    }
    /// Returns the timestamped price and quantity for each tick's last trade.
    pub fn last_timed_price_quantities(&self) -> Vec<TimedPriceQuantity> {
        self.values(MarketTick::last)
    }

    fn values(&self, to_quote: impl Fn(&MarketTick) -> &MarketQuote) -> Vec<TimedPriceQuantity> {
        self.ticks
            .iter()
            .map(|tick| {
                let quote = to_quote(tick);
                quote.to_timed_price_quantity(tick.timestamp_millis())
            })
            .collect()
    }
}

impl<'de> Deserialize<'de> for MarketTicks {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct MarketTicksData {
            ticks: Vec<MarketTick>,
        }

        let data = MarketTicksData::deserialize(deserializer)?;
        Ok(Self::new(data.ticks))
    }
}

#[cfg(test)]
#[path = "market_ticks_tests.rs"]
mod tests;
