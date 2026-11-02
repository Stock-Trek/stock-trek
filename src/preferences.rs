use serde::{Deserialize, Serialize};
use stock_trek_types::cex::preferences::CexPreferences;

/// Preferences that control how an algorithm's orders are handled.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Preferences {
    cex: CexPreferences,
}

impl Preferences {
    /// Creates preferences from the CEX preference settings.
    pub fn new(cex: CexPreferences) -> Self {
        Self { cex }
    }

    /// Returns the CEX preference settings.
    pub fn cex(&self) -> CexPreferences {
        self.cex
    }
}
