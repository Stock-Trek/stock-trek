use serde::{Deserialize, Serialize};
use stock_trek_types::cex::preferences::CexPreferences;

/// Runtime preferences that control how an algorithm's orders are handled.
///
/// These are returned from [`crate::Algorithm::preferences`]. Preferences apply
/// to every order produced by the algorithm and are not part of the strategy
/// command.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Preferences {
    /// CEX-specific preferences such as rounding strategies and the maximum
    /// tolerated delay between the local clock and the exchange.
    pub cex: CexPreferences,
}
