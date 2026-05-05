pub mod actions;
pub mod error;
pub mod examples;
pub mod exchanges;
pub mod market_data;
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
        exchanges::{
            client_order_id::ClientOrderId, exchange_factory::ExchangeFactory,
            order_capability::OrderCapability,
        },
        portfolios::portfolio_factory::PortfolioFactory,
        resolved_context::ResolvedContext,
        resolver_context::ResolverContext,
        resolvers::{on_action_error::OnActionError, resolver::Resolver},
        scratch::{key::ScratchKey, scratch_pad::ScratchPad, value::ScratchValue},
        strategy::Strategy,
        strategy_context::StrategyContext,
    };

    pub use digdigdig3::{
        core::{OrderRequest, TimeInForce},
        AccountType, ExchangeId, OrderSide, OrderType, Symbol,
    };

    pub use traitreg;
    pub use traitreg::register as register_strategy;
}
