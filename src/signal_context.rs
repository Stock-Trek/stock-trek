use crate::{
    cex::{asset_id::AssetId, exchange_id::ExchangeId},
    market_data::market::Market,
};
use std::collections::HashMap;

pub struct SignalContext {
    market_data: HashMap<ExchangeId, MarketDataByBaseContext>,
}

impl SignalContext {
    pub fn new(market_data: HashMap<ExchangeId, MarketDataByBaseContext>) -> Self {
        Self { market_data }
    }
    pub fn exchange_markets_for(
        &self,
        base: AssetId,
        quote: AssetId,
    ) -> impl Iterator<Item = (ExchangeId, &Market)> {
        self.market_data
            .iter()
            .filter_map(move |(exchange, by_base)| {
                by_base
                    .markets_by_base
                    .get(&base)
                    .and_then(|by_quote| by_quote.markets_by_quote.get(&quote))
                    .map(|market| (exchange.clone(), market))
            })
    }
    pub fn market_for(
        &self,
        exchange_id: &ExchangeId,
        base: &AssetId,
        quote: &AssetId,
    ) -> Option<&Market> {
        self.market_data
            .get(exchange_id)
            .and_then(|m| m.markets_by_base.get(base))
            .and_then(|m| m.markets_by_quote.get(quote))
    }
}

pub struct MarketDataByBaseContext {
    markets_by_base: HashMap<AssetId, MarketDataByQuoteContext>,
}

impl MarketDataByBaseContext {
    pub fn new(markets_by_base: HashMap<AssetId, MarketDataByQuoteContext>) -> Self {
        Self { markets_by_base }
    }
}

pub struct MarketDataByQuoteContext {
    markets_by_quote: HashMap<AssetId, Market>,
}

impl MarketDataByQuoteContext {
    pub fn new(markets_by_quote: HashMap<AssetId, Market>) -> Self {
        Self { markets_by_quote }
    }
}
