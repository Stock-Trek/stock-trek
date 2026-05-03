use serde::{Deserialize, Serialize};
use strum::EnumIter;

#[derive(Debug, Clone, Copy, EnumIter, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OrderCapability {
    Market,
    Limit,
    StopMarket,
    StopLimit,
    TrailingStop,
    OneCancelsOther,
    Bracket,
    Iceberg,
    TimeWeightedAveragePrice,
    PostOnly,
    ImmediateOrCancel,
    FillOrKill,
    GoodTillDate,
    ReduceOnly,
    OneTriggersOther,
    ConditionalPlan,
    DcaRecurring,
}
