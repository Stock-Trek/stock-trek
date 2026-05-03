use crate::{
    error::result::StockTrekResult, exchanges::order_capability::OrderCapability,
    market_data::market::Market,
};
use digdigdig3::{core::OrderRequest, Order, Symbol};

pub type Exchange = Box<dyn ExchangeTrait>;

pub trait ExchangeTrait: Send + Sync {
    fn has_capability(&self, capability: &OrderCapability) -> StockTrekResult<bool>;
    fn market_for(&self, symbol: &Symbol) -> StockTrekResult<Option<&Market>>;
    fn place_order(&self, request: &OrderRequest) -> StockTrekResult<Order>;
}
