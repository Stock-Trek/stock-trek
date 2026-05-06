use crate::scratch::key::{ExchangeName, TokenName};

pub type Portfolio = Box<dyn PortfolioTrait>;

pub trait PortfolioTrait {
    fn has_account_in_exchange(&self, exchange: &ExchangeName) -> bool;
    fn owns_token(&self, token: &TokenName) -> bool;
    fn owns_token_in_exchange(&self, token: &TokenName, exchange: &ExchangeName) -> bool;
    fn token_total(&self, token: &TokenName) -> f64;
    fn token_in_exchange(&self, token: &TokenName, exchange: &ExchangeName) -> f64;
    // TODO
    // fn orders_in_exchange(&self, exchange: &ExchangeName) -> f64;
    // fn order_by_order_id(
    //     &self,
    //     exchange: &ExchangeName,
    //     order_id: &OrderId,
    // ) -> Option<OrderResponse>;
    // fn order_by_client_order_id(
    //     &self,
    //     exchange: &ExchangeName,
    //     client_order_id: &ClientOrderId,
    // ) -> Option<OrderResponse>;
}
