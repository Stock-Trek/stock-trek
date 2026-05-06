use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum OrderIntent {
    Open,
    Close { reduce_only: bool },
}
