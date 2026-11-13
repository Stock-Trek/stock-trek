//! Write trading algorithms that run on [stock-trek.com](https://stock-trek.com).
//!
//! An algorithm is a struct implementing [`Algorithm`] and registered with the
//! `#[register_algorithm]` annotation. The platform runs it in two
//! phases:
//!
//! 1. [`Algorithm::signals`] runs on every evaluation. It turns the market
//!    data in a [`SignalContext`](crate::signals::SignalContext) into a
//!    [`Signals`](crate::signals::Signals) map that the strategy phase can
//!    read by key.
//! 2. [`Algorithm::strategy`] runs exactly once. It builds a [`Command`]
//!    tree from the factories on a [`StrategyContext`]. The platform then
//!    interprets that tree on every evaluation, resolving deferred values
//!    against the latest [`Signals`](crate::signals::Signals) and
//!    [`Portfolio`] and enqueueing the resulting [`actions::Action`]s for
//!    execution.
//!
//! Because the strategy phase runs once, every value it uses must be
//! produced by a factory (eg. `c.conditions.signal(MY_KEY)`) rather than
//! read directly, so it can be re-evaluated each time the command tree runs.
//!
//! The [`prelude`] module gathers the types needed by most algorithms, and a
//! complete cost-averaging algorithm is maintained in
//! [`examples/cost_averaging.rs`](https://github.com/Stock-Trek/stock-trek/blob/master/examples/cost_averaging.rs).

#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![deny(unsafe_code)]
#![deny(rustdoc::broken_intra_doc_links)]

extern crate self as stock_trek;

mod action;
mod algorithm;
mod cex;
mod commands;
mod conditions;
#[allow(unused)]
#[cfg(feature = "dex")]
mod dex;
mod error;
mod market_data;
mod pending_order;
mod portfolio;
mod preferences;
mod registration;
mod resolvable;
mod resolved_context;
mod signal;
mod signal_context;
mod strategy_context;
mod util;
mod validation;
mod value;

pub use algorithm::Algorithm;
pub use cex::order_factory::OrderFactory;
pub use commands::{Command, CommandFactory};
pub use conditions::{Condition, ConditionFactory, QuantityOf};
pub use pending_order::PendingOrder;
pub use portfolio::{CexAccountPortfolio, Portfolio, PortfolioBuilder};
pub use preferences::Preferences;
pub use registration::RegisteredImpl;
pub use resolved_context::{EnqueueActionFn, ResolvedContext};
pub use rust_decimal::RoundingStrategy;
pub use stock_trek_macros::register_algorithm;
pub use strategy_context::StrategyContext;

/// Support used by the registration macro.
#[doc(hidden)]
pub mod __private {
    pub use crate::registration::{RegisteredImpl, Registration};
}

/// Types for actions that can be enqueued by an algorithm.
pub mod actions {
    pub use crate::action::{
        action_factory::ActionFactory,
        action_type::Action,
        recoverable_action::{
            ActionErrorCause, ActionErrorResponse, RecoverableAction, RecoveryPolicy,
        },
        resolved_action::ResolvedAction,
    };
}

/// Traits and helpers for CEX capabilities.
pub mod capabilities {
    pub use crate::cex::capability::{HasRequiredCapabilities, combine_capabilities};
}

/// Error types returned by stock-trek operations.
pub mod errors {
    #[cfg(feature = "dex")]
    pub use crate::error::dex::DexError;
    pub use crate::error::{
        domain::DomainError,
        result::{StockTrekError, StockTrekResult},
        stats::StatsError,
        value::ValueError,
    };
}

/// Market data types and time window helpers.
pub mod markets {
    pub use crate::market_data::{
        WindowMillis,
        aligned_window::{AlignedTimestampBounds, AlignedWindow},
        market::Market,
        market::MarketBuilder,
        market_aligned_window::MarketAlignedWindow,
        market_candle::MarketCandle,
        market_ohlcv::MarketOhlcv,
        market_order_book::MarketOrderBook,
        market_quote::{MarketQuote, TimedPriceQuantity},
        market_rolling_window::MarketRollingWindow,
        market_tick::MarketTick,
        market_ticks::MarketTicks,
        millis,
        rolling_window::RollingWindow,
        timestamp::TimestampMillis,
    };
}

/// Signal keys, values, and contexts.
pub mod signals {
    pub use crate::{
        signal::{
            key::{SignalKey, SignalKeyType},
            signals::Signals,
            value::SignalValue,
        },
        signal_context::{
            CexMarketDataByBaseContext, CexMarketDataByQuoteContext, SignalContext,
            SignalContextBuilder,
        },
    };
}

/// Shared CEX and asset types.
pub mod types {
    pub use stock_trek_types::cex::{
        account_id::AccountId,
        activation::Activation,
        asset_id::AssetId,
        capability::CexCapability,
        cex_id::CexId,
        order_request::OrderRequest,
        order_response::OrderResponse,
        preferences::{CexPreferences, CexRoundingPreferences},
        price_basis::PriceBasis,
        quantity::Quantity,
        side::Side,
        status::Status,
        tag::Tag,
        time_in_force::TimeInForce,
        trading_pair::TradingPair,
        trigger_direction::TriggerDirection,
        trigger_mode::TriggerMode,
    };
}

/// Value types and factories used to build calculations.
pub mod values {
    pub use crate::value::{
        binary_operator::BinaryOperator,
        unary_operator::UnaryOperator,
        value_types::{AccountIdValue, AssetIdValue, CexIdValue, FlagValue, NumberValue},
        values_factory::{
            CalculationValuesFactory, LiteralValuesFactory, PortfolioValuesFactory,
            SignalValuesFactory,
        },
    };
}

/// Commonly used imports for writing algorithms.
pub mod prelude {
    #[cfg(feature = "dex")]
    pub use super::errors::DexError;
    pub use super::{
        Algorithm, Command, Preferences, RoundingStrategy, StrategyContext,
        actions::{ActionErrorCause, ActionErrorResponse, RecoveryPolicy},
        conditions::{Condition, ConditionFactory, QuantityOf},
        errors::{DomainError, StatsError, StockTrekError, StockTrekResult, ValueError},
        markets::{
            AlignedTimestampBounds, AlignedWindow, Market, MarketAlignedWindow, MarketBuilder,
            MarketCandle, MarketOhlcv, MarketOrderBook, MarketQuote, MarketRollingWindow,
            MarketTick, MarketTicks, RollingWindow, TimedPriceQuantity, TimestampMillis,
            WindowMillis, millis,
        },
        pending_order::PendingOrder,
        portfolio::{CexAccountPortfolio, Portfolio},
        register_algorithm,
        signals::{SignalContext, SignalContextBuilder, SignalKey, Signals},
        types::{
            AccountId, Activation, AssetId, CexId, CexPreferences, CexRoundingPreferences,
            Quantity, Side, Tag, TimeInForce,
        },
        values::{
            AccountIdValue, AssetIdValue, BinaryOperator, CalculationValuesFactory, CexIdValue,
            FlagValue, LiteralValuesFactory, NumberValue, PortfolioValuesFactory,
            SignalValuesFactory, UnaryOperator,
        },
    };
}

#[cfg(test)]
mod conditions_tests;
#[cfg(test)]
mod pending_order_tests;
#[cfg(test)]
mod registration_tests;
#[cfg(test)]
mod signal_context_tests;
#[cfg(test)]
mod validation_tests;
