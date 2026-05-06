use crate::{
    portfolios::portfolio::{Portfolio, PortfolioTrait},
    scratch::key::{ExchangeName, TokenName},
};
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct InMemoryPortfolio {
    exchange_tokens: HashMap<ExchangeName, Tokens>,
}
impl InMemoryPortfolio {
    pub fn new(exchange_tokens: HashMap<ExchangeName, Tokens>) -> Self {
        Self { exchange_tokens }
    }
    pub fn builder() -> Builder {
        Builder::new()
    }
}

impl From<InMemoryPortfolio> for Portfolio {
    fn from(value: InMemoryPortfolio) -> Self {
        Box::new(value)
    }
}

#[derive(Debug, Clone, Default)]
pub struct Tokens {
    tokens: HashMap<TokenName, f64>,
}
impl Tokens {
    pub fn new(tokens: HashMap<TokenName, f64>) -> Self {
        Self { tokens }
    }
}

impl PortfolioTrait for InMemoryPortfolio {
    fn has_account_in_exchange(&self, exchange: &ExchangeName) -> bool {
        self.exchange_tokens.contains_key(exchange)
    }
    fn owns_token(&self, token: &TokenName) -> bool {
        self.exchange_tokens
            .values()
            .any(|tokens| tokens.tokens.contains_key(token))
    }
    fn owns_token_in_exchange(&self, token: &TokenName, exchange: &ExchangeName) -> bool {
        self.exchange_tokens
            .get(exchange)
            .map(|tokens| tokens.tokens.contains_key(token))
            .unwrap_or(false)
    }
    fn token_total(&self, token: &TokenName) -> f64 {
        self.exchange_tokens
            .values()
            .map(|tokens| tokens.tokens.get(token).unwrap_or(&0.0))
            .sum()
    }
    fn token_in_exchange(&self, token: &TokenName, exchange: &ExchangeName) -> f64 {
        self.exchange_tokens
            .get(exchange)
            .and_then(|tokens| tokens.tokens.get(token))
            .copied()
            .unwrap_or(0.0)
    }
}

#[derive(Clone, Default)]
pub struct Builder {
    exchange_tokens: HashMap<ExchangeName, Tokens>,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            exchange_tokens: HashMap::new(),
        }
    }
    pub fn tokens(&mut self, exchange: ExchangeName, token: TokenName, quantity: f64) -> &mut Self {
        self.exchange_tokens
            .entry(exchange)
            .or_insert_with(|| Tokens::new(HashMap::new()))
            .tokens
            .entry(token)
            .and_modify(|prev| *prev += quantity)
            .or_insert(quantity);
        self
    }
    pub fn build(&self) -> InMemoryPortfolio {
        InMemoryPortfolio::new(self.exchange_tokens.clone())
    }
}
