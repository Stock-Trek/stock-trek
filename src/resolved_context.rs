use crate::{
    Portfolio,
    action::{recoverable_action::RecoveryPolicy, resolved_action::ResolvedAction},
    error::{result::StockTrekResult, value::ValueError},
    market_data::market::Market,
    preferences::Preferences,
    signal::signals::Signals,
    signal_context::SignalContext,
};
use stock_trek_types::cex::{asset_id::AssetId, cex_id::CexId};

/// The context used when resolving actions and values.
pub struct ResolvedContext {
    /// Callback used to enqueue an action with its recovery policy.
    pub enqueue_action: EnqueueActionFn,
    /// The current portfolio state.
    pub portfolio: Portfolio,
    /// The signals produced by the algorithm.
    pub signals: Signals,
    /// The preferences used to quantize resolved order values.
    pub preferences: Preferences,
    /// The market data used to resolve order increments and minimum notionals.
    pub markets: SignalContext,
}

/// Callback type for enqueuing a resolved action with a recovery policy.
pub type EnqueueActionFn = Box<dyn FnMut(&ResolvedAction, &RecoveryPolicy) -> StockTrekResult<()>>;

impl ResolvedContext {
    /// Creates a resolved context from an action callback, portfolio, signals,
    /// preferences, and market data.
    pub fn new(
        enqueue_action: EnqueueActionFn,
        portfolio: Portfolio,
        signals: Signals,
        preferences: Preferences,
        markets: SignalContext,
    ) -> Self {
        Self {
            enqueue_action,
            portfolio,
            signals,
            preferences,
            markets,
        }
    }

    /// Returns the market for a trading pair on a specific CEX.
    pub fn market(
        &self,
        cex_id: &CexId,
        base: &AssetId,
        quote: &AssetId,
    ) -> StockTrekResult<&Market> {
        self.markets
            .cex_market_for(cex_id, base, quote)
            .ok_or_else(|| {
                ValueError::NotFound {
                    name: "Market".into(),
                    key: format!("{cex_id} {base}/{quote}"),
                }
                .into()
            })
    }
}

impl std::fmt::Debug for ResolvedContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ResolvedContext")
            .field("portfolio", &self.portfolio)
            .field("signals", &self.signals)
            .field("preferences", &self.preferences)
            .finish_non_exhaustive()
    }
}
