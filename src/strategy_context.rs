use crate::{
    market_data::market::Market,
    scratch::key::{ExchangeName, TokenName},
    statistics::{
        advanced::Advanced, decompose::Decompose, evaluation::Evaluation,
        exponential_smoothing::ExponentialSmoothing, filter::Filter, frequency::Frequency,
        hypothesis::Hypothesis, moving_average::MovingAverage, stats::Stats,
        time_series::TimeSeries, transformation::Transformation, wavelet::Wavelet,
    },
};
use std::collections::HashMap;

pub struct StrategyContext {
    market_data: HashMap<ExchangeName, MarketDataByBaseContext>,
    pub stats: Stats,
}

impl StrategyContext {
    pub fn new(market_data: HashMap<ExchangeName, MarketDataByBaseContext>) -> Self {
        Self {
            market_data,
            stats: Stats {
                advanced: Advanced,
                decompose: Decompose,
                evaluation: Evaluation,
                exponential_smoothing: ExponentialSmoothing,
                filter: Filter,
                frequency: Frequency,
                hypothesis: Hypothesis,
                moving_average: MovingAverage,
                time_series: TimeSeries,
                transformation: Transformation,
                wavelet: Wavelet,
            },
        }
    }
    pub fn market_for(
        &self,
        exchange: &ExchangeName,
        base: &TokenName,
        quote: &TokenName,
    ) -> Option<&Market> {
        self.market_data
            .get(exchange)
            .and_then(|m| m.markets_by_base.get(base))
            .and_then(|m| m.markets_by_quote.get(quote))
    }
}

pub struct MarketDataByBaseContext {
    markets_by_base: HashMap<TokenName, MarketDataByQuoteContext>,
}

impl MarketDataByBaseContext {
    pub fn new(markets_by_base: HashMap<TokenName, MarketDataByQuoteContext>) -> Self {
        Self { markets_by_base }
    }
}

pub struct MarketDataByQuoteContext {
    markets_by_quote: HashMap<TokenName, Market>,
}

impl MarketDataByQuoteContext {
    pub fn new(markets_by_quote: HashMap<TokenName, Market>) -> Self {
        Self { markets_by_quote }
    }
}
