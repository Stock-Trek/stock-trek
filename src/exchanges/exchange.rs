use crate::{exchanges::order_capability::OrderCapability, market_data::market::Market};
use anyhow::Result;
use digdigdig3::{core::OrderRequest, Order, Symbol};

pub type Exchange = Box<dyn ExchangeTrait>;

pub trait ExchangeTrait: Send + Sync {
    fn has_capability(&self, capability: OrderCapability) -> Result<bool>;
    fn market_for(&self, symbol: &Symbol) -> Result<Option<&Market>>;
    fn place_order(&self, request: &OrderRequest) -> Result<Order>;
}
