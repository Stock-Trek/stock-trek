mod action;
mod algorithm;
mod cex;
mod commands;
mod conditions;
// pub mod dex;
mod error;
mod examples;
mod market_data;
mod portfolio;
mod preferences;
mod resolveable;
mod resolved_context;
mod signal;
mod signal_context;
mod strategy_context;
mod util;
mod value;

pub use algorithm::Algorithm;
pub use cex::order_factory::OrderFactory;
pub use commands::{Command, CommandFactory};
pub use conditions::{Condition, ConditionFactory, QuantityOf};
pub use portfolio::{CexAccountPortfolio, PendingOrder, Portfolio, PortfolioBuilder};
pub use preferences::Preferences;
pub use resolved_context::{EnqueueActionFn, ResolvedContext};
pub use rust_decimal::RoundingStrategy;
pub use strategy_context::StrategyContext;
pub use traitreg;
pub use traitreg::register as register_algorithm;

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

pub mod capabilities {
    pub use crate::cex::capability::{HasRequiredCapabilities, combine_capabilities};
}

pub mod errors {
    pub use crate::error::{
        portfolio::PortfolioError,
        result::{StockTrekError, StockTrekResult},
        stats::StatsError,
        value::ValueError,
    };
}

pub mod markets {
    pub use crate::market_data::{
        aligned_window::AlignedWindow,
        market::Market,
        market::MarketBuilder,
        market_aligned_window::MarketAlignedWindow,
        market_candle::MarketCandle,
        market_ohlcv::MarketOhlcv,
        market_order_book::MarketOrderBook,
        market_quote::{MarketQuote, PriceQuantity, TimedPriceQuantity},
        market_rolling_window::{MarketRollingWindow, Ohlcv},
        market_tick::MarketTick,
        market_ticks::MarketTicks,
        rolling_window::RollingWindow,
        timestamp::TimestampMillis,
    };
}

pub mod signals {
    pub use crate::{
        signal::{
            key::{SignalKey, SignalKeyType},
            signals::Signals,
            value::SignalValue,
        },
        signal_context::{CexMarketDataByBaseContext, CexMarketDataByQuoteContext, SignalContext},
    };
}

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

pub mod prelude {
    pub use super::{
        Algorithm, Command, Preferences, RoundingStrategy, StrategyContext,
        actions::{ActionErrorCause, ActionErrorResponse, RecoveryPolicy},
        portfolio::Portfolio,
        register_algorithm,
        signals::{SignalContext, SignalKey, Signals},
        traitreg,
        types::{
            AccountId, Activation, AssetId, CexId, CexPreferences, CexRoundingPreferences,
            Quantity, Side, Tag, TimeInForce,
        },
    };
}
