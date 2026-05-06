pub mod error;
pub mod examples;
pub mod exchanges;
pub mod market_data;
pub mod order;
pub mod portfolios;
pub mod predicates;
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
        error::result::{StockTrekError, StockTrekResult},
        exchanges::exchange_factory::ExchangeFactory,
        order::{
            order_intent::OrderIntent, order_pricing::OrderPricing, order_quantity::OrderQuantity,
            order_request::OrderRequest, order_response::OrderResponse, order_side::OrderSide,
            order_status::OrderStatus, order_time_in_force::OrderTimeInForce,
            order_timing::OrderTiming, order_trigger_direction::OrderTriggerDirection,
            single_order::SingleOrder,
        },
        portfolios::portfolio_factory::PortfolioFactory,
        resolved_context::ResolvedContext,
        resolver_context::ResolverContext,
        resolvers::resolver::Resolver,
        scratch::{
            key::{ExchangeName, ScratchKey, TokenName},
            scratch_pad::ScratchPad,
            value::ScratchValue,
        },
        strategy::Strategy,
        strategy_context::StrategyContext,
    };

    pub use traitreg;
    pub use traitreg::register as register_strategy;
}
