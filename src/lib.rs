pub mod asset_id;
pub mod capability;
pub mod error;
pub mod examples;
pub mod exchange_id;
pub mod market_data;
pub mod order;
pub mod portfolios;
pub mod predicates;
pub mod preferences;
pub mod resolved_context;
pub mod resolver_context;
pub mod resolvers;
pub mod scratch;
pub mod statistics;
pub mod strategy;
pub mod strategy_context;
pub mod util;
pub mod values;
pub mod verification;

pub mod prelude {
    pub use crate::{
        asset_id::AssetId,
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
        preferences::{MultiLeg, OnDifferent, Preferences},
        resolved_context::ResolvedContext,
        resolver_context::ResolverContext,
        resolvers::resolver::Resolver,
        scratch::{key::ScratchKey, scratch_pad::ScratchPad, value::ScratchValue},
        strategy::Strategy,
        strategy_context::StrategyContext,
    };

    pub use traitreg;
    pub use traitreg::register as register_strategy;
}
