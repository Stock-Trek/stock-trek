use serde::{Deserialize, Serialize};
use stock_trek_types::cex::preferences::CexPreferences;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preferences {
    pub cex: CexPreferences,
}
