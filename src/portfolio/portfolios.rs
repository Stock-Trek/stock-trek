use crate::portfolio::{in_memory_portfolio::InMemoryPortfolio, portfolio::Portfolio};

pub struct Portfolios {}

impl Portfolios {
    pub fn in_memory() -> Portfolio {
        InMemoryPortfolio::new()
    }
}
