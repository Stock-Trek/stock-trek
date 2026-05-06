use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum OrderTriggerDirection {
    Above,
    Below,
}
