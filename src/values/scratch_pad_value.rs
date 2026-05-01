use crate::{
    resolved_context::ResolvedContext,
    values::value::{
        AssetValue, AssetValueTrait, ExchangeValue, ExchangeValueTrait, FlagValue, FlagValueTrait,
        NumberValue, NumberValueTrait,
    },
};
use anyhow::Result;
use digdigdig3::{Asset, ExchangeId};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ScratchPadAssetValue {
    key: String,
}

impl ScratchPadAssetValue {
    pub fn new(key: String) -> AssetValue {
        Box::new(Self { key })
    }
}

#[typetag::serde]
impl AssetValueTrait for ScratchPadAssetValue {
    fn asset(&self, context: &ResolvedContext) -> Result<Asset> {
        context.scratch_pad.read_asset_required(&self.key)
    }
}

#[derive(Serialize, Deserialize)]
pub struct ScratchPadExchangeValue {
    key: String,
}

impl ScratchPadExchangeValue {
    pub fn new(key: String) -> ExchangeValue {
        Box::new(Self { key })
    }
}

#[typetag::serde]
impl ExchangeValueTrait for ScratchPadExchangeValue {
    fn exchange(&self, context: &ResolvedContext) -> Result<ExchangeId> {
        context.scratch_pad.read_exchange_required(&self.key)
    }
}

#[derive(Serialize, Deserialize)]
pub struct ScratchPadFlagValue {
    key: String,
}

impl ScratchPadFlagValue {
    pub fn new(key: String) -> FlagValue {
        Box::new(Self { key })
    }
}

#[typetag::serde]
impl FlagValueTrait for ScratchPadFlagValue {
    fn flag(&self, context: &ResolvedContext) -> Result<bool> {
        context.scratch_pad.read_flag_required(&self.key)
    }
}

#[derive(Serialize, Deserialize)]
pub struct ScratchPadNumberValue {
    key: String,
}

impl ScratchPadNumberValue {
    pub fn new(key: String) -> NumberValue {
        Box::new(Self { key })
    }
}

#[typetag::serde]
impl NumberValueTrait for ScratchPadNumberValue {
    fn number(&self, context: &ResolvedContext) -> Result<f64> {
        context.scratch_pad.read_number_required(&self.key)
    }
}
