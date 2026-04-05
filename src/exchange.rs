use crate::{dto::raw_exchange::RawExchange, market_data::market::Market, prelude::TradingPair};
use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Exchange {
    markets: HashMap<TradingPair, Market>,
}

impl Exchange {
    pub fn new(markets: HashMap<TradingPair, Market>) -> Self {
        Self { markets }
    }
    pub fn markets(&self) -> &HashMap<TradingPair, Market> {
        &self.markets
    }
}

impl TryFrom<RawExchange> for Exchange {
    type Error = String;

    fn try_from(value: RawExchange) -> Result<Self, Self::Error> {
        let RawExchange {
            markets: raw_markets,
        } = value;
        let mut markets = HashMap::new();
        for (instrument, raw_market) in raw_markets {
            let currencies: Vec<&str> = instrument.split("/").collect();
            if currencies.len() != 2 {
                return Err(format!(
                    "Cannot parse instrument {} into base and quote currencies",
                    instrument
                ));
            }
            let base = currencies[0];
            let quote = currencies[1];
            let trading_pair = TradingPair::new(base, quote);
            let market = Market::try_from(raw_market)?;
            markets.insert(trading_pair, market);
        }
        Ok(Exchange { markets })
    }
}
