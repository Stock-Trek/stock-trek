use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
