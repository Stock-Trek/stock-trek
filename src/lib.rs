pub mod algorithm;
pub mod asset_id;
pub mod capability;
pub mod commands;
pub mod conditions;
pub mod error;
pub mod examples;
pub mod exchange_id;
pub mod market_data;
pub mod order;
pub mod portfolios;
pub mod preferences;
pub mod resolved_context;
pub mod signal;
pub mod signal_context;
pub mod statistics;
pub mod strategy_context;
pub mod util;
pub mod values;
pub mod verification;

pub mod prelude {
    pub use crate::{
        algorithm::Algorithm,
        asset_id::AssetId,
        commands::command::Command,
        exchange_id::ExchangeId,
        order::{
            order_activation::OrderActivation,
            order_constraint::OrderConstraint,
            order_intent::OrderIntent,
            order_price_basis::OrderPriceBasis,
            order_pricing::OrderPricing,
            order_quantity::OrderQuantity,
            order_request::OrderRequest,
            order_response::OrderResponse,
            order_side::OrderSide,
            order_status::OrderStatus,
            order_time_in_force::OrderTimeInForce,
            order_trigger_direction::OrderTriggerDirection,
            order_trigger_mode::OrderTriggerMode,
            orders::{
                one_cancels_other::{OneCancelsOtherOrder, OneCancelsOtherOrderRaw},
                one_triggers_oco::{OneTriggersOcoOrder, OneTriggersOcoOrderRaw},
                one_triggers_other::{OneTriggersOtherOrder, OneTriggersOtherOrderRaw},
                single::{SingleOrder, SingleOrderRaw},
            },
        },
        portfolios::portfolio_factory::PortfolioFactory,
        preferences::{MultiLeg, OnDifferent, Preferences, Rounding},
        resolved_context::ResolvedContext,
        signal::{key::SignalKey, signals::Signals, value::SignalValue},
        signal_context::SignalContext,
        strategy_context::StrategyContext,
    };

    pub use rust_decimal::RoundingStrategy;

    pub use traitreg;
    pub use traitreg::register as register_algorithm;
}
