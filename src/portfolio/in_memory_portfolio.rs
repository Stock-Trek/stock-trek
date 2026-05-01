use crate::portfolio::portfolio::{Portfolio, PortfolioTrait};
use anyhow::{anyhow, bail, Result};
use digdigdig3::{Asset, ExchangeId};
use std::collections::HashMap;

pub struct InMemoryPortfolio {
    exchange_assets: HashMap<ExchangeId, Assets>,
}

impl InMemoryPortfolio {
    pub fn new() -> Portfolio {
        Box::new(Self {
            exchange_assets: HashMap::new(),
        })
    }
    pub fn add_cash(&mut self, exchange_id: ExchangeId, cash: f64) {
        let exchange = self
            .exchange_assets
            .entry(exchange_id)
            .or_insert(Assets::new());
        exchange.cash += cash;
    }
    pub fn remove_cash(&mut self, exchange_id: ExchangeId, cash: f64) -> Result<()> {
        let exchange = self.exchange_assets.get_mut(&exchange_id);
        match exchange {
            None => bail!("Portfolio has not used exchange {:?}", exchange_id),
            Some(assets) => {
                if assets.cash < cash {
                    bail!(
                        "Exchange {:?} has cash {} and cannot remove {}",
                        exchange_id,
                        assets.cash,
                        cash
                    )
                }
                assets.cash -= cash;
                Ok(())
            }
        }
    }
    pub fn add_tokens(&mut self, exchange_id: ExchangeId, asset: Asset, quantity: f64) {
        let exchange = self
            .exchange_assets
            .entry(exchange_id)
            .or_insert(Assets::new());
        exchange
            .tokens
            .entry(asset)
            .and_modify(|previous| *previous += quantity)
            .or_insert(quantity);
    }
    pub fn remove_tokens(
        &mut self,
        exchange_id: ExchangeId,
        asset: Asset,
        quantity: f64,
    ) -> Result<()> {
        let exchange = self
            .exchange_assets
            .get_mut(&exchange_id)
            .ok_or_else(|| anyhow!("Portfolio has not used exchange {:?}", exchange_id))?;
        let token_quantity = exchange
            .tokens
            .get_mut(&asset)
            .ok_or_else(|| anyhow!("Exchange {:?} does not have asset {:?}", exchange_id, asset))?;
        if *token_quantity < quantity {
            bail!(
                "Exchange {:?} has {} of asset {:?} and cannot remove {}",
                exchange_id,
                token_quantity,
                asset,
                quantity
            );
        }
        *token_quantity -= quantity;
        if *token_quantity == 0.0 {
            exchange.tokens.remove(&asset);
        }
        Ok(())
    }
}

impl PortfolioTrait for InMemoryPortfolio {
    fn cash_total(&self) -> f64 {
        self.exchange_assets.values().map(|e| e.cash).sum()
    }
    fn cash_in_exchange(&self, exchange: ExchangeId) -> f64 {
        self.exchange_assets
            .get(&exchange)
            .map(|e| e.cash)
            .unwrap_or(0.0)
    }
    fn has_account_in_exchange(&self, exchange: ExchangeId) -> bool {
        self.exchange_assets.contains_key(&exchange)
    }
    fn owns_asset(&self, asset: &Asset) -> bool {
        self.exchange_assets
            .values()
            .any(|assets| assets.tokens.contains_key(asset))
    }
    fn owns_asset_in_exchange(&self, asset: &Asset, exchange: &ExchangeId) -> bool {
        self.exchange_assets
            .get(exchange)
            .map(|assets| assets.tokens.contains_key(asset))
            .unwrap_or(false)
    }
    fn asset_total(&self, asset: Asset) -> f64 {
        self.exchange_assets
            .values()
            .map(|assets| assets.tokens.get(&asset).unwrap_or(&0.0))
            .sum()
    }
    fn asset_in_exchange(&self, asset: Asset, exchange: ExchangeId) -> f64 {
        self.exchange_assets
            .get(&exchange)
            .and_then(|assets| assets.tokens.get(&asset))
            .copied()
            .unwrap_or(0.0)
    }
}

struct Assets {
    cash: f64,
    tokens: HashMap<Asset, f64>,
}

impl Assets {
    fn new() -> Self {
        Self {
            cash: 0.0,
            tokens: HashMap::new(),
        }
    }
}
