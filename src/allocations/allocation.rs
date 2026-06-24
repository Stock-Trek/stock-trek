use stock_trek_types::cex::{asset_id::AssetId, cex_id::CexId};

pub type Allocation = Box<dyn AllocationTrait>;

pub trait AllocationTrait {
    fn allocation_for_asset_total(&self, asset_id: &AssetId) -> f64;
    fn allocation_for_asset_in_cex(&self, asset_id: &AssetId, cex_id: &CexId) -> f64;
}
