use crate::{
    exchange::Exchange,
    statistics::{
        advanced::Advanced, decompose::Decompose, evaluation::Evaluation,
        exponential_smoothing::ExponentialSmoothing, filter::Filter, frequency::Frequency,
        hypothesis::Hypothesis, moving_average::MovingAverage, stats::Stats,
        time_series::TimeSeries, transformation::Transformation, wavelet::Wavelet,
    },
};
use std::collections::HashMap;

pub struct StockTrekContext {
    exchanges: HashMap<String, Exchange>,
    pub stats: Stats,
}

impl StockTrekContext {
    pub fn new(exchanges: HashMap<String, Exchange>) -> Self {
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
    pub fn exchanges(&self) -> &HashMap<String, Exchange> {
        &self.exchanges
    }
}
