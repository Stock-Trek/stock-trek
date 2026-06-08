use crate::cex::cex_preferences::CexPreferences;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preferences {
    pub cex: CexPreferences,
}
