use crate::{
    assembler_context::AssemblerContext,
    values::value::{
        AssetValue, AssetValueTrait, ExchangeValue, ExchangeValueTrait, FlagValue, FlagValueTrait,
        NumberValue, NumberValueTrait,
    },
};
use anyhow::Result;
use digdigdig3::{Asset, ExchangeId};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LiteralAssetValue {
    literal: Asset,
}

impl LiteralAssetValue {
    pub fn new(literal: Asset) -> AssetValue {
        Box::new(Self { literal })
    }
}

#[typetag::serde]
impl AssetValueTrait for LiteralAssetValue {
    fn asset(&self, _: &AssemblerContext) -> Result<Asset> {
        Ok(self.literal.clone())
    }
}

#[derive(Serialize, Deserialize)]
pub struct LiteralExchangeValue {
    literal: ExchangeId,
}

impl LiteralExchangeValue {
    pub fn new(literal: ExchangeId) -> ExchangeValue {
        Box::new(Self { literal })
    }
}

#[typetag::serde]
impl ExchangeValueTrait for LiteralExchangeValue {
    fn exchange(&self, _: &AssemblerContext) -> Result<ExchangeId> {
        Ok(self.literal.clone())
    }
}

#[derive(Serialize, Deserialize)]
pub struct LiteralFlagValue {
    literal: bool,
}

impl LiteralFlagValue {
    pub fn new(literal: bool) -> FlagValue {
        Box::new(Self { literal })
    }
}

#[typetag::serde]
impl FlagValueTrait for LiteralFlagValue {
    fn flag(&self, _: &AssemblerContext) -> Result<bool> {
        Ok(self.literal.clone())
    }
}

#[derive(Serialize, Deserialize)]
pub struct LiteralNumberValue {
    literal: f64,
}

impl LiteralNumberValue {
    pub fn new(literal: f64) -> NumberValue {
        Box::new(Self { literal })
    }
}

#[typetag::serde]
impl NumberValueTrait for LiteralNumberValue {
    fn number(&self, _: &AssemblerContext) -> Result<f64> {
        Ok(self.literal.clone())
    }
}
