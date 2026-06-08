use crate::{
    error::result::StockTrekResult,
    resolved_context::ResolvedContext,
    values::value::{AssetIdValue, CexIdValue, NumberValue, NumberValueTrait},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct AssetInCexValue {
    cex_id_value: CexIdValue,
    asset_id_value: AssetIdValue,
}

impl AssetInCexValue {
    pub fn new(cex_id_value: CexIdValue, asset_id_value: AssetIdValue) -> NumberValue {
        Box::new(Self {
            cex_id_value,
            asset_id_value,
        })
    }
}

#[typetag::serde]
impl NumberValueTrait for AssetInCexValue {
    fn clone_box(&self) -> NumberValue {
        Box::new(self.clone())
    }
    fn number(&self, c: &ResolvedContext) -> StockTrekResult<f64> {
        let cex_id = self.cex_id_value.cex_id(c)?;
        let asset_id = self.asset_id_value.asset_id(c)?;
        Ok(c.portfolio.asset_in_cex(&asset_id, &cex_id))
    }
}
