use chrono::Utc;
use digdigdig3::core::OrderRequest;
use std::hash::{DefaultHasher, Hash, Hasher};
use uuid::Uuid;

pub struct ClientOrderIdBuilder {}

impl ClientOrderIdBuilder {
    pub fn from(order_request: &OrderRequest) -> Option<String> {
        if let Some(existing) = order_request.client_order_id.clone() {
            return Some(existing);
        }
        let mut hasher = DefaultHasher::new();

        order_request.account_type.hash(&mut hasher);
        match order_request.order_type.clone() {
            digdigdig3::OrderType::Bracket {
                price,
                take_profit,
                stop_loss,
            } => {
                if let Some(v) = price {
                    v.to_bits().hash(&mut hasher);
                }
                take_profit.to_bits().hash(&mut hasher);
                stop_loss.to_bits().hash(&mut hasher);
            }
            digdigdig3::OrderType::ConditionalPlan {
                trigger_price,
                trigger_direction,
                order_after_trigger,
            } => {
                trigger_price.to_bits().hash(&mut hasher);
                format!("{:?}", trigger_direction).hash(&mut hasher);
                format!("{:?}", order_after_trigger).hash(&mut hasher);
            }
            digdigdig3::OrderType::DcaRecurring {
                interval_seconds,
                total_cycles,
                price_limit,
            } => {
                interval_seconds.hash(&mut hasher);
                total_cycles.hash(&mut hasher);
                if let Some(v) = price_limit {
                    v.to_bits().hash(&mut hasher);
                }
            }
            digdigdig3::OrderType::Fok { price } => {
                price.to_bits().hash(&mut hasher);
            }
            digdigdig3::OrderType::Gtd { price, expire_time } => {
                price.to_bits().hash(&mut hasher);
                expire_time.hash(&mut hasher);
            }
            digdigdig3::OrderType::Iceberg {
                price,
                display_quantity,
            } => {
                price.to_bits().hash(&mut hasher);
                display_quantity.to_bits().hash(&mut hasher);
            }
            digdigdig3::OrderType::Ioc { price } => {
                if let Some(v) = price {
                    v.to_bits().hash(&mut hasher);
                }
            }
            digdigdig3::OrderType::Limit { price } => {
                price.to_bits().hash(&mut hasher);
            }
            digdigdig3::OrderType::Market => {}
            digdigdig3::OrderType::Oco {
                price,
                stop_price,
                stop_limit_price,
            } => {
                price.to_bits().hash(&mut hasher);
                stop_price.to_bits().hash(&mut hasher);
                if let Some(v) = stop_limit_price {
                    v.to_bits().hash(&mut hasher);
                }
            }
            digdigdig3::OrderType::Oto {
                entry_price,
                secondary_order,
            } => {
                if let Some(v) = entry_price {
                    v.to_bits().hash(&mut hasher);
                }
                format!("{:?}", secondary_order).hash(&mut hasher);
            }
            digdigdig3::OrderType::PostOnly { price } => {
                price.to_bits().hash(&mut hasher);
            }
            digdigdig3::OrderType::ReduceOnly { price } => {
                if let Some(v) = price {
                    v.to_bits().hash(&mut hasher);
                }
            }
            digdigdig3::OrderType::StopLimit {
                stop_price,
                limit_price,
            } => {
                stop_price.to_bits().hash(&mut hasher);
                limit_price.to_bits().hash(&mut hasher);
            }
            digdigdig3::OrderType::StopMarket { stop_price } => {
                stop_price.to_bits().hash(&mut hasher);
            }
            digdigdig3::OrderType::TrailingStop {
                callback_rate,
                activation_price,
            } => {
                callback_rate.to_bits().hash(&mut hasher);
                if let Some(v) = activation_price {
                    v.to_bits().hash(&mut hasher);
                }
            }
            digdigdig3::OrderType::Twap {
                duration_seconds,
                interval_seconds,
            } => {
                duration_seconds.hash(&mut hasher);
                interval_seconds.hash(&mut hasher);
            }
        }
        order_request.quantity.to_bits().hash(&mut hasher);
        order_request.reduce_only.hash(&mut hasher);
        order_request.side.as_str().hash(&mut hasher);
        order_request.symbol.hash(&mut hasher);
        format!("{:?}", order_request.time_in_force).hash(&mut hasher);

        let high_bits = Utc::now().timestamp_millis() as u64;
        let low_bits = hasher.finish();
        let client_order_id = Uuid::from_u64_pair(high_bits, low_bits).to_string();
        Some(client_order_id)
    }
}
