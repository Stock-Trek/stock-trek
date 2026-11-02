use crate::{
    Portfolio,
    action::{recoverable_action::RecoveryPolicy, resolved_action::ResolvedAction},
    cex::cex_metadata::{CexMetadata, MarketMetadata},
    error::{result::StockTrekResult, value::ValueError},
    preferences::Preferences,
    signal::signals::Signals,
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
    /// The static CEX metadata used to resolve order increments and minimum
    /// notionals.
    pub cex_metadata: CexMetadata,
}

/// Callback type for enqueuing a resolved action with a recovery policy.
pub type EnqueueActionFn = Box<dyn FnMut(&ResolvedAction, &RecoveryPolicy) -> StockTrekResult<()>>;

impl ResolvedContext {
    /// Creates a resolved context from an action callback, portfolio, signals,
    /// preferences, and CEX metadata.
    pub fn new(
        enqueue_action: EnqueueActionFn,
        portfolio: Portfolio,
        signals: Signals,
        preferences: Preferences,
        cex_metadata: CexMetadata,
    ) -> Self {
        Self {
            enqueue_action,
            portfolio,
            signals,
            preferences,
            cex_metadata,
        }
    }

    /// Returns the market metadata for a trading pair on a specific CEX.
    pub fn market_metadata(
        &self,
        cex_id: &CexId,
        base: &AssetId,
        quote: &AssetId,
    ) -> StockTrekResult<&MarketMetadata> {
        self.cex_metadata
            .cex_market_for(cex_id, base, quote)
            .ok_or_else(|| {
                ValueError::NotFound {
                    name: "Market metadata".into(),
                    key: format!("{cex_id} {base}/{quote}"),
                }
                .into()
            })
    }

    /// Returns the market metadata for a trading pair on a specific CEX.
    pub fn market(
        &self,
        cex_id: &CexId,
        base: &AssetId,
        quote: &AssetId,
    ) -> StockTrekResult<&MarketMetadata> {
        self.market_metadata(cex_id, base, quote)
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
