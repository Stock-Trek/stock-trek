use crate::market_data::{
    market_quote::{MarketQuote, TimedPriceQuantity},
    market_tick::MarketTick,
};
use serde::{Deserialize, Deserializer, Serialize};
use std::sync::OnceLock;

#[derive(Debug, Clone, Serialize)]
pub struct MarketTicks {
    pub ticks: Vec<MarketTick>,
    #[serde(skip)]
    bids: OnceLock<Vec<TimedPriceQuantity>>,
    #[serde(skip)]
    asks: OnceLock<Vec<TimedPriceQuantity>>,
    #[serde(skip)]
    lasts: OnceLock<Vec<TimedPriceQuantity>>,
}

impl<'de> Deserialize<'de> for MarketTicks {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            ticks: Vec<MarketTick>,
        }
        let helper = Helper::deserialize(deserializer)?;
        Ok(MarketTicks::new(helper.ticks))
    }
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
    pub fn bid_timed_price_quantities(&self) -> &Vec<TimedPriceQuantity> {
        self.bids.get_or_init(|| self.values(|tick| &tick.bid))
    }
    pub fn ask_timed_price_quantities(&self) -> &Vec<TimedPriceQuantity> {
        self.asks.get_or_init(|| self.values(|tick| &tick.ask))
    }
    pub fn last_timed_price_quantities(&self) -> &Vec<TimedPriceQuantity> {
        self.lasts.get_or_init(|| self.values(|tick| &tick.last))
    }

    fn values(&self, to_quote: impl Fn(&MarketTick) -> &MarketQuote) -> Vec<TimedPriceQuantity> {
        self.ticks
            .iter()
            .map(|tick| self.to_tick_tuple(tick, &to_quote))
            .collect()
    }
    fn to_tick_tuple(
        &self,
        tick: &MarketTick,
        to_quote: impl Fn(&MarketTick) -> &MarketQuote,
    ) -> TimedPriceQuantity {
        let timestamp = tick.timestamp_millis;
        let quote = to_quote(tick);
        quote.to_timed_price_quantity(timestamp)
    }
}
