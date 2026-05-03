use crate::portfolios::{
    in_memory_portfolio::Builder as InMemoryPortfolioBuilder, portfolio::Portfolio,
    stub_portfolio::StubPortfolio,
};

pub struct PortfolioFactory;

impl PortfolioFactory {
    pub fn stub() -> Portfolio {
        StubPortfolio::new().into()
    }
    pub fn in_memory_builder() -> InMemoryPortfolioBuilder {
        InMemoryPortfolioBuilder::new()
    }
}
