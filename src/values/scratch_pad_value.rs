use crate::{
    error::result::StockTrekResult,
    prelude::ScratchKey,
    resolved_context::ResolvedContext,
    values::value::{AssetValueTrait, ExchangeValueTrait, FlagValueTrait, NumberValueTrait},
};
use digdigdig3::{Asset, ExchangeId};

#[typetag::serde]
impl AssetValueTrait for ScratchKey<Asset> {
    fn asset(&self, context: &ResolvedContext) -> StockTrekResult<Asset> {
        self.read(context)
    }
}
#[typetag::serde]
impl ExchangeValueTrait for ScratchKey<ExchangeId> {
    fn exchange(&self, context: &ResolvedContext) -> StockTrekResult<ExchangeId> {
        self.read(context)
    }
}
#[typetag::serde]
impl FlagValueTrait for ScratchKey<bool> {
    fn flag(&self, context: &ResolvedContext) -> StockTrekResult<bool> {
        self.read(context)
    }
}
#[typetag::serde]
impl NumberValueTrait for ScratchKey<f64> {
    fn number(&self, context: &ResolvedContext) -> StockTrekResult<f64> {
        self.read(context)
    }
}
