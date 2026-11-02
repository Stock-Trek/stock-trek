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
    /// Returns `(timestamp, price, quantity)` for each tick's best bid.
    pub fn bid_timed_price_quantities(&self) -> Vec<TimedPriceQuantity> {
        self.values(MarketTick::bid)
    }
    /// Returns `(timestamp, price, quantity)` for each tick's best ask.
    pub fn ask_timed_price_quantities(&self) -> Vec<TimedPriceQuantity> {
        self.values(MarketTick::ask)
    }
    /// Returns `(timestamp, price, quantity)` for each tick's last trade.
    pub fn last_timed_price_quantities(&self) -> Vec<TimedPriceQuantity> {
        self.values(MarketTick::last)
    }

    fn values(&self, to_quote: impl Fn(&MarketTick) -> &MarketQuote) -> Vec<TimedPriceQuantity> {
        self.ticks
            .iter()
            .map(|tick| Self::to_tick_tuple(tick, &to_quote))
            .collect()
    }

    fn to_tick_tuple(
        tick: &MarketTick,
        to_quote: impl Fn(&MarketTick) -> &MarketQuote,
    ) -> TimedPriceQuantity {
        let timestamp = tick.timestamp_millis();
        let quote = to_quote(tick);
        quote.to_timed_price_quantity(timestamp)
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
mod tests {
    use super::*;
    use crate::market_data::timestamp::TimestampMillis;

    fn tick(price: f64) -> MarketTick {
        tick_with_timestamp(1, price)
    }

    fn tick_with_timestamp(timestamp_millis: u64, price: f64) -> MarketTick {
        MarketTick::new(
            TimestampMillis::new(timestamp_millis),
            MarketQuote::new(price, 2.0).unwrap(),
            MarketQuote::new(price + 1.0, 3.0).unwrap(),
            MarketQuote::new(price + 0.5, 4.0).unwrap(),
        )
    }

    #[test]
    fn ticks_are_sorted_chronologically() {
        let ticks = MarketTicks::new(vec![
            tick_with_timestamp(3, 103.0),
            tick_with_timestamp(1, 100.0),
            tick_with_timestamp(2, 101.0),
        ]);

        assert_eq!(
            ticks.bid_timed_price_quantities(),
            vec![
                (TimestampMillis::new(1), 100.0, 2.0),
                (TimestampMillis::new(2), 101.0, 2.0),
                (TimestampMillis::new(3), 103.0, 2.0)
            ]
        );
    }

    #[test]
    fn serde_json_round_trip() {
        let ticks = MarketTicks::new(vec![tick(100.0)]);
        let json = serde_json::to_string(&ticks).unwrap();
        let deserialized: MarketTicks = serde_json::from_str(&json).unwrap();

        assert_eq!(
            deserialized.bid_timed_price_quantities(),
            vec![(TimestampMillis::new(1), 100.0, 2.0)]
        );
    }

    #[test]
    fn deserialization_sorts_ticks_chronologically() {
        let json = r#"{
            "ticks": [
                {
                    "timestamp_millis": 3,
                    "bid": {"price": 103.0, "quantity": 2.0},
                    "ask": {"price": 104.0, "quantity": 3.0},
                    "last": {"price": 103.5, "quantity": 4.0}
                },
                {
                    "timestamp_millis": 1,
                    "bid": {"price": 100.0, "quantity": 2.0},
                    "ask": {"price": 101.0, "quantity": 3.0},
                    "last": {"price": 100.5, "quantity": 4.0}
                }
            ]
        }"#;
        let deserialized: MarketTicks = serde_json::from_str(json).unwrap();

        assert_eq!(
            deserialized.bid_timed_price_quantities(),
            vec![
                (TimestampMillis::new(1), 100.0, 2.0),
                (TimestampMillis::new(3), 103.0, 2.0)
            ]
        );
    }
}
