use crate::statistics::stats_error::StatsError;
use anyhow::Result;

pub type StatsResult<T> = Result<T, StatsError>;
