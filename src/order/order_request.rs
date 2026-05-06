use crate::order::single_order::SingleOrder;
use serde::{Deserialize, Serialize};
use std::hash::Hash;

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum OrderRequest<T, N> {
    Single(SingleOrder<T, N>),
    OneCancelsTheOther {
        a: Box<OrderRequest<T, N>>,
        b: Box<OrderRequest<T, N>>,
    },
    OneTriggersTheOther {
        primary: Box<OrderRequest<T, N>>,
        secondary: Box<OrderRequest<T, N>>,
    },
}
