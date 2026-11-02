use serde::{Deserialize, Serialize};
use stock_trek_types::cex::preferences::CexPreferences;

/// Preferences that control how an algorithm's orders are handled.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Preferences {
    pub cex: CexPreferences,
}
