use crate::market_data::{
    market_quote::{MarketQuote, TimedPriceQuantity},
    market_tick::MarketTick,
};
use serde::{Deserialize, Serialize};

/// A collection of recent market ticks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketTicks {
    /// The ticks in chronological order.
    pub ticks: Vec<MarketTick>,
}

impl MarketTicks {
    /// Creates a tick collection from a list of ticks.
    pub fn new(ticks: Vec<MarketTick>) -> Self {
        Self { ticks }
    }
    /// Returns the bid quotes as timestamped price/quantity tuples.
    pub fn bid_timed_price_quantities(&self) -> Vec<TimedPriceQuantity> {
        self.values(|tick| &tick.bid)
    }
    /// Returns the ask quotes as timestamped price/quantity tuples.
    pub fn ask_timed_price_quantities(&self) -> Vec<TimedPriceQuantity> {
        self.values(|tick| &tick.ask)
    }
    /// Returns the last-trade quotes as timestamped price/quantity tuples.
    pub fn last_timed_price_quantities(&self) -> Vec<TimedPriceQuantity> {
        self.values(|tick| &tick.last)
    }

    fn values(&self, to_quote: impl Fn(&MarketTick) -> &MarketQuote) -> Vec<TimedPriceQuantity> {
        self.ticks
            .iter()
            .map(|tick| to_tick_tuple(tick, &to_quote))
            .collect()
    }
}

fn to_tick_tuple(
    tick: &MarketTick,
    to_quote: impl Fn(&MarketTick) -> &MarketQuote,
) -> TimedPriceQuantity {
    let timestamp = tick.timestamp_millis;
    let quote = to_quote(tick);
    quote.to_timed_price_quantity(timestamp)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tick(price: f64) -> MarketTick {
        MarketTick {
            timestamp_millis: 1,
            bid: MarketQuote {
                price,
                quantity: 2.0,
            },
            ask: MarketQuote {
                price: price + 1.0,
                quantity: 3.0,
            },
            last: MarketQuote {
                price: price + 0.5,
                quantity: 4.0,
            },
        }
    }

    #[test]
    fn values_reflect_current_ticks() {
        let mut ticks = MarketTicks::new(vec![tick(100.0)]);

        assert_eq!(ticks.bid_timed_price_quantities(), vec![(1, 100.0, 2.0)]);
        assert_eq!(ticks.ask_timed_price_quantities(), vec![(1, 101.0, 3.0)]);
        assert_eq!(
            ticks.last_timed_price_quantities(),
            vec![(1, 100.5, 4.0)]
        );

        ticks.ticks[0].bid.price = 200.0;
        ticks.ticks[0].ask.price = 201.0;
        ticks.ticks[0].last.price = 200.5;

        assert_eq!(ticks.bid_timed_price_quantities(), vec![(1, 200.0, 2.0)]);
        assert_eq!(ticks.ask_timed_price_quantities(), vec![(1, 201.0, 3.0)]);
        assert_eq!(
            ticks.last_timed_price_quantities(),
            vec![(1, 200.5, 4.0)]
        );
    }

    #[test]
    fn serde_json_round_trip() {
        let ticks = MarketTicks::new(vec![tick(100.0)]);
        let json = serde_json::to_string(&ticks).unwrap();
        let deserialized: MarketTicks = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.bid_timed_price_quantities(), vec![(1, 100.0, 2.0)]);
    }
}
