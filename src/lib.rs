//! Stock Trek time-series analysis.
//!
//! This crate provides the core types and building blocks for writing trading
//! algorithms that run on [stock-trek.com](https://stock-trek.com).
//!
//! # Execution model
//!
//! An [`Algorithm`] separates *observation* from *decision making*:
//!
//! - [`Algorithm::signals`] runs before every evaluation and reduces the current
//!   market data into the signals the algorithm cares about. Signals are typed
//!   and keyed, so the strategy can read them without re-examining raw candles
//!   or order books.
//! - [`Algorithm::strategy`] runs exactly once. It returns a [`Command`] that
//!   describes how to react to signals and the current [`Portfolio`]. The same
//!   command is interpreted on every evaluation as signals and portfolio
//!   balances change.
//!
//! Commands are declarative: a strategy does not place orders directly. It
//! builds [`Condition`]s and [`actions`] that are resolved against a
//! [`ResolvedContext`] when the command executes. This keeps strategies
//! deterministic and lets the runtime validate values and CEX capabilities
//! before anything is sent to an exchange.
//!
//! # Building a strategy
//!
//! [`StrategyContext`] groups the factories used to build a command. Literal and
//! signal-backed values live in [`values`], conditions and signal checks in
//! [`ConditionFactory`], order requests in [`OrderFactory`], and executable
//! actions in [`actions`]. Reusable conditions can be combined with
//! [`ConditionFactory::quantity_of`], and actions carry a
//! [`actions::RecoveryPolicy`] that decides what happens when an exchange
//! rejects them.
//!
//! # Market data
//!
//! [`markets::Market`] holds tick data, rolling candles, calendar-aligned
//! candles, and the current order book for one trading pair.
//! [`markets::RollingWindow`] has a fixed duration, while
//! [`markets::AlignedWindow`] also supports calendar boundaries such as months
//! and years. [`markets::TimestampMillis`] is the millisecond timestamp used
//! throughout the API.
//!
//! # CEX constraints
//!
//! Exchange constraints are part of the market data rather than hard-coded in
//! strategies. [`markets::Market::base_increment`],
//! [`markets::Market::quote_increment`], and
//! [`markets::Market::minimum_notional`] describe the smallest valid order, and
//! [`Preferences`] controls how the runtime rounds prices and quantities.
//!
//! # DEX support
//!
//! The [`dex`] module models assets deployed on blockchains, validates contract
//! addresses against each chain's address format, and provides definitions for
//! well-known networks under [`dex::networks`].

#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![deny(unsafe_code)]
#![deny(rustdoc::broken_intra_doc_links)]

mod action;
mod algorithm;
mod cex;
mod commands;
mod conditions;
/// Decentralised exchange (DEX) types.
#[cfg(feature = "dex")]
pub mod dex;
mod error;
mod market_data;
mod pending_order;
mod portfolio;
mod preferences;
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
pub use resolved_context::{EnqueueActionFn, ResolvedContext};
pub use rust_decimal::RoundingStrategy;
pub use strategy_context::StrategyContext;
pub use traitreg;
pub use traitreg::register as register_algorithm;

/// Types for actions that can be enqueued by an algorithm.
///
/// An action describes a side effect the runtime should perform, such as
/// placing an order or cancelling orders. Actions are resolved against the
/// current signals and portfolio immediately before execution. Every action is
/// paired with a [`crate::actions::RecoveryPolicy`] so a strategy can specify retries
/// or fallback plans for exchange errors.
pub mod actions {
    pub use crate::action::{
        action::Action,
        action_factory::ActionFactory,
        recoverable_action::{
            ActionErrorCause, ActionErrorResponse, RecoverableAction, RecoveryPolicy,
        },
        resolved_action::ResolvedAction,
    };
}

/// Traits and helpers for CEX capabilities.
///
/// Capabilities describe optional exchange features, such as support for
/// quote-denominated quantities on limit orders. Strategies can use this module
/// to inspect the capabilities required by a command before submitting it.
pub mod capabilities {
    pub use crate::cex::capability::{HasRequiredCapabilities, combine_capabilities};
}

/// Error types returned by stock-trek operations.
pub mod errors {
    pub use crate::error::{
        dex::DexError,
        domain::DomainError,
        result::{StockTrekError, StockTrekResult},
        stats::StatsError,
        value::ValueError,
    };
}

/// Market data types and time window helpers.
///
/// A [`crate::markets::Market`] contains everything a strategy needs to
/// evaluate one trading pair: recent ticks, rolling and calendar-aligned
/// candles, the order book, and the exchange's order-size constraints. Use
/// [`crate::markets::RollingWindow`] for candles with a fixed duration and
/// [`crate::markets::AlignedWindow`] for windows that follow calendar
/// boundaries.
pub mod markets {
    pub use crate::market_data::{
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
        rolling_window::RollingWindow,
        timestamp::TimestampMillis,
    };
}

/// Signal keys, values, and contexts.
///
/// Signals are the interface between [`crate::Algorithm::signals`] and the strategy
/// command. An algorithm writes typed values under
/// [`crate::signals::SignalKey`]s, and the strategy reads them through the
/// factories in [`crate::values`] or [`crate::ConditionFactory`].
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
///
/// Re-exports of identifiers, order requests, order responses, and other types
/// shared between algorithms and the stock-trek runtime.
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
///
/// Values are lazy: they are resolved against the current signals and portfolio
/// when a condition or action executes. This module provides literal values,
/// signal-backed values, portfolio queries, and arithmetic operators.
pub mod values {
    pub use crate::value::{
        binary_operator::BinaryOperator,
        unary_operator::UnaryOperator,
        value::{AccountIdValue, AssetIdValue, CexIdValue, FlagValue, NumberValue},
        values_factory::{
            CalculationValuesFactory, LiteralValuesFactory, PortfolioValuesFactory,
            SignalValuesFactory,
        },
    };
}

/// Commonly used imports for writing algorithms.
pub mod prelude {
    pub use super::{
        Algorithm, Command, Preferences, RoundingStrategy, StrategyContext,
        actions::{ActionErrorCause, ActionErrorResponse, RecoveryPolicy},
        errors::{DexError, DomainError, StatsError, StockTrekError, StockTrekResult, ValueError},
        pending_order::PendingOrder,
        portfolio::{CexAccountPortfolio, Portfolio},
        register_algorithm,
        signals::{SignalContext, SignalContextBuilder, SignalKey, Signals},
        traitreg,
        types::{
            AccountId, Activation, AssetId, CexId, CexPreferences, CexRoundingPreferences,
            Quantity, Side, Tag, TimeInForce,
        },
    };
}
