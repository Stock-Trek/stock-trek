use crate::{
    error::result::{StockTrekError, StockTrekResult},
    resolved_context::ResolvedContext,
    scratch::{
        key::{ScratchKey, ScratchPadKeyType},
        value::ScratchValue,
    },
    values::value::{AssetValueTrait, ExchangeValueTrait, FlagValueTrait, NumberValueTrait},
};
use digdigdig3::{Asset, ExchangeId};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ScratchPadValue {
    key: String,
}

impl ScratchPadValue {
    pub fn new<T>(key: &ScratchKey<T>) -> Box<Self>
    where
        T: ScratchPadKeyType + Into<ScratchValue> + TryFrom<ScratchValue, Error = StockTrekError>,
    {
        Box::new(Self { key: key.key() })
    }
    pub fn read<T>(&self, context: &ResolvedContext) -> StockTrekResult<T>
    where
        T: ScratchPadKeyType + Into<ScratchValue> + TryFrom<ScratchValue, Error = StockTrekError>,
    {
        context.scratch_pad.read(&ScratchKey::new(&self.key))
    }
}

#[typetag::serde]
impl AssetValueTrait for ScratchPadValue {
    fn asset(&self, context: &ResolvedContext) -> StockTrekResult<Asset> {
        self.read(context)
    }
}
#[typetag::serde]
impl ExchangeValueTrait for ScratchPadValue {
    fn exchange(&self, context: &ResolvedContext) -> StockTrekResult<ExchangeId> {
        self.read(context)
    }
}
#[typetag::serde]
impl FlagValueTrait for ScratchPadValue {
    fn flag(&self, context: &ResolvedContext) -> StockTrekResult<bool> {
        self.read(context)
    }
}
#[typetag::serde]
impl NumberValueTrait for ScratchPadValue {
    fn number(&self, context: &ResolvedContext) -> StockTrekResult<f64> {
        self.read(context)
    }
}
