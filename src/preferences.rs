use serde::{Deserialize, Serialize};
use stock_trek_types::cex::preferences::CexPreferences;

/// Preferences that control how an algorithm's orders are handled.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Preferences {
    /// Per-CEX rounding and network preferences, returned by
    /// [`Algorithm::preferences`](crate::Algorithm::preferences).
    pub cex: CexPreferences,
}
