use crate::{
    asset_id::AssetId,
    error::result::StockTrekResult,
    exchange_id::ExchangeId,
    execute::{capability::Capability, executor::Executor, increment_sizes::IncrementSizes},
    order::{order_request::OrderRequest, trading_pair::TradingPair},
};
use async_trait::async_trait;
use rust_decimal::Decimal;
use std::collections::HashMap;

pub type ExchangeAdapter = Box<dyn ExchangeAdapterTrait>;

#[async_trait]
pub trait ExchangeAdapterTrait {
    fn id(&self) -> &ExchangeId;
    fn capabilities(&self) -> &Vec<Capability>;
    fn increments(&self) -> &HashMap<TradingPair, IncrementSizes>;
    fn asset_ticker(&self, asset_id: &AssetId) -> &str {
        asset_id.default_ticker()
    }
    fn symbol_ticker_divider(&self) -> Option<&str> {
        None
    }
    fn to_symbol(&self, base: &AssetId, quote: &AssetId) -> String {
        let base_ticker = self.asset_ticker(base);
        let quote_ticker = self.asset_ticker(quote);
        match self.symbol_ticker_divider() {
            None => format!("{}{}", base_ticker, quote_ticker),
            Some(divider) => format!("{}{}{}", base_ticker, divider, quote_ticker),
        }
    }
    fn is_valid(&self, order: &OrderRequest<AssetId, Decimal>) -> StockTrekResult<bool>;
    fn enqueue_order(
        &self,
        order: &OrderRequest<AssetId, Decimal>,
        executor: &Executor,
    ) -> StockTrekResult<()>;
}
