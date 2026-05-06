use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum OrderQuantity<Q> {
    OfBase(Q),
    OfQuote(Q),
}
