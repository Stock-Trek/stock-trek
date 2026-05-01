use digdigdig3::{Asset, ExchangeId};

pub type Portfolio = Box<dyn PortfolioTrait>;

pub trait PortfolioTrait {
    fn has_account_in_exchange(&self, exchange: ExchangeId) -> bool;
    fn owns_asset(&self, asset: &Asset) -> bool;
    fn owns_asset_in_exchange(&self, asset: &Asset, exchange: &ExchangeId) -> bool;
    fn cash_total(&self) -> f64;
    fn cash_in_exchange(&self, exchange: ExchangeId) -> f64;
    fn asset_total(&self, asset: Asset) -> f64;
    fn asset_in_exchange(&self, asset: Asset, exchange: ExchangeId) -> f64;
}
