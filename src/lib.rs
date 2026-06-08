pub mod actions;
pub mod algorithm;
pub mod cex;
pub mod commands;
pub mod conditions;
// pub mod dex;
pub mod error;
pub mod examples;
pub mod market_data;
pub mod portfolios;
pub mod preferences;
pub mod resolveable;
pub mod resolved_context;
pub mod signal;
pub mod signal_context;
pub mod strategy_context;
pub mod util;
pub mod values;

pub mod prelude {
    pub use crate::{
        actions::recoverable_action::{ErrorCause, ErrorResponse, RecoveryPolicy},
        algorithm::Algorithm,
        cex::{
            asset_id::AssetId,
            cex_id::CexId,
            cex_preferences::{CexPreferences, MultiLeg, OnDifferent, Rounding},
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
        commands::command::Command,
        portfolios::portfolio_factory::PortfolioFactory,
        preferences::Preferences,
        signal::{key::SignalKey, signals::Signals, value::SignalValue},
        signal_context::SignalContext,
        strategy_context::StrategyContext,
    };

    pub use rust_decimal::RoundingStrategy;

    pub use traitreg;
    pub use traitreg::register as register_algorithm;
}
