use crate::{
    assembler_context::AssemblerContext,
    values::value::{AssetValue, NumberValue, NumberValueTrait},
};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AssetTotalValue {
    asset: AssetValue,
}

impl AssetTotalValue {
    pub fn new(asset: AssetValue) -> NumberValue {
        Box::new(Self { asset })
    }
}

#[typetag::serde]
impl NumberValueTrait for AssetTotalValue {
    fn number(&self, context: &AssemblerContext) -> Result<f64> {
        let asset = self.asset.asset(context)?;
        Ok(context.portfolio.asset_total(asset))
    }
}
