pub mod actions;
pub mod algorithm;
pub mod allocations;
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
        allocations::allocation_factory::AllocationFactory,
        commands::command::Command,
        portfolios::portfolio_factory::PortfolioFactory,
        preferences::Preferences,
        signal::{key::SignalKey, signals::Signals, value::SignalValue},
        signal_context::SignalContext,
        strategy_context::StrategyContext,
    };

    pub use rust_decimal::RoundingStrategy;

    pub use stock_trek_types::cex::{
        activation::Activation,
        asset_id::AssetId,
        capability::CexCapability,
        cex_id::CexId,
        order_request::OrderRequest,
        order_response::OrderResponse,
        orders::single_order::SingleOrder,
        preferences::{CexPreferences, CexRoundingPreferences},
        price_basis::PriceBasis,
        pricing::Pricing,
        quantity::Quantity,
        side::Side,
        status::Status,
        tag::Tag,
        time_in_force::TimeInForce,
        trading_pair::TradingPair,
        trigger_direction::TriggerDirection,
        trigger_mode::TriggerMode,
    };

    pub use traitreg;
    pub use traitreg::register as register_algorithm;
}
