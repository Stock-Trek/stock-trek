use crate::{
    dto::raw_context::RawContext,
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
    pub fn exchanges(&self) -> &HashMap<String, Exchange> {
        &self.exchanges
    }
}

impl TryFrom<RawContext> for StockTrekContext {
    type Error = String;

    fn try_from(value: RawContext) -> Result<Self, Self::Error> {
        let RawContext {
            exchanges: raw_exchanges,
        } = value;
        let mut exchanges = HashMap::new();
        for (name, raw_exchange) in raw_exchanges {
            match Exchange::try_from(raw_exchange) {
                Ok(exchange) => {
                    exchanges.insert(name, exchange);
                }
                Err(error) => return Err(error),
            }
        }
        Ok(StockTrekContext {
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
        })
    }
}
