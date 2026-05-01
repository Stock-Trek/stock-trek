use crate::{
    exchanges::{
        exchange::{Exchange, ExchangeTrait},
        order_capability::OrderCapability,
    },
    market_data::market::Market,
};
use anyhow::{bail, Result};
use chrono::Utc;
use digdigdig3::{core::OrderRequest, Order, OrderStatus, Symbol};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct InMemoryExchange {
    capabilities: HashSet<OrderCapability>,
    markets: HashMap<Symbol, Market>,
}

impl InMemoryExchange {
    pub fn new(
        capabilities: HashSet<OrderCapability>,
        markets: HashMap<Symbol, Market>,
    ) -> Exchange {
        Box::new(Self {
            capabilities,
            markets,
        })
    }
}

impl ExchangeTrait for InMemoryExchange {
    fn has_capability(&self, capability: OrderCapability) -> Result<bool> {
        Ok(self.capabilities.contains(&capability))
    }
    fn market_for(&self, symbol: &Symbol) -> Result<Option<&Market>> {
        Ok(self.markets.get(symbol))
    }
    fn place_order(&self, request: &OrderRequest) -> Result<Order> {
        let symbol = &request.symbol;
        let pair = Symbol::new(symbol.base.clone(), symbol.quote.clone());
        match self.markets.get(&pair) {
            None => bail!("No market found for {}", symbol),
            Some(_market) => Ok(Order {
                id: Uuid::new_v4().to_string(),
                client_order_id: request.client_order_id.clone(),
                symbol: request.symbol.to_underscore(),
                side: request.side,
                order_type: request.order_type.clone(),
                status: OrderStatus::New,
                price: None,
                stop_price: None,
                quantity: request.quantity,
                filled_quantity: 0.0,
                average_price: None,
                commission: None,
                commission_asset: None,
                created_at: Utc::now().timestamp_millis(),
                updated_at: None,
                time_in_force: request.time_in_force,
            }),
        }
    }
}
