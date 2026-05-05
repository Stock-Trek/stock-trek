use digdigdig3::OrderType;
use serde::{Deserialize, Serialize};
use strum::EnumIter;

#[derive(Debug, Clone, Copy, EnumIter, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
#[repr(u8)]
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

impl From<OrderType> for OrderCapability {
    fn from(value: OrderType) -> Self {
        match value {
            OrderType::Bracket {
                price: _,
                take_profit: _,
                stop_loss: _,
            } => OrderCapability::Bracket,
            OrderType::ConditionalPlan {
                trigger_price: _,
                trigger_direction: _,
                order_after_trigger: _,
            } => OrderCapability::ConditionalPlan,
            OrderType::DcaRecurring {
                interval_seconds: _,
                total_cycles: _,
                price_limit: _,
            } => OrderCapability::DcaRecurring,
            OrderType::Fok { price: _ } => OrderCapability::FillOrKill,
            OrderType::Gtd {
                price: _,
                expire_time: _,
            } => OrderCapability::GoodTillDate,
            OrderType::Iceberg {
                price: _,
                display_quantity: _,
            } => OrderCapability::Iceberg,
            OrderType::Ioc { price: _ } => OrderCapability::ImmediateOrCancel,
            OrderType::Limit { price: _ } => OrderCapability::Limit,
            OrderType::Market => OrderCapability::Market,
            OrderType::Oco {
                price: _,
                stop_price: _,
                stop_limit_price: _,
            } => OrderCapability::OneCancelsOther,
            OrderType::Oto {
                entry_price: _,
                secondary_order: _,
            } => OrderCapability::OneTriggersOther,
            OrderType::PostOnly { price: _ } => OrderCapability::PostOnly,
            OrderType::ReduceOnly { price: _ } => OrderCapability::ReduceOnly,
            OrderType::StopLimit {
                stop_price: _,
                limit_price: _,
            } => OrderCapability::StopLimit,
            OrderType::StopMarket { stop_price: _ } => OrderCapability::StopMarket,
            OrderType::TrailingStop {
                callback_rate: _,
                activation_price: _,
            } => OrderCapability::TrailingStop,
            OrderType::Twap {
                duration_seconds: _,
                interval_seconds: _,
            } => OrderCapability::TimeWeightedAveragePrice,
        }
    }
}
