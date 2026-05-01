use crate::{
    exchanges::exchange::Exchange,
    statistics::{
        advanced::Advanced, decompose::Decompose, evaluation::Evaluation,
        exponential_smoothing::ExponentialSmoothing, filter::Filter, frequency::Frequency,
        hypothesis::Hypothesis, moving_average::MovingAverage, stats::Stats,
        time_series::TimeSeries, transformation::Transformation, wavelet::Wavelet,
    },
};
use digdigdig3::{ExchangeId, Symbol};
use std::collections::HashMap;

pub struct StrategyContext {
    pub exchanges: HashMap<ExchangeId, Exchange>,
    pub stats: Stats,
}

impl StrategyContext {
    pub fn new(exchanges: HashMap<ExchangeId, Exchange>) -> Self {
        Self {
            exchanges,
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
    pub fn symbol(&self, base: impl AsRef<str>, quote: impl AsRef<str>) -> Symbol {
        Symbol::new(base.as_ref(), quote.as_ref())
    }
}
