use crate::allocations::allocation::{Allocation, AllocationTrait};
use hashbrown::HashMap;
use stock_trek_types::cex::{asset_id::AssetId, cex_id::CexId};

#[derive(Debug, Clone, Default)]
pub struct InMemoryAllocation {
    cex_assets: HashMap<CexId, Allocations>,
}
impl InMemoryAllocation {
    pub fn new(cex_assets: HashMap<CexId, Allocations>) -> Self {
        Self { cex_assets }
    }
    pub fn builder() -> Builder {
        Builder::new()
    }
}

impl From<InMemoryAllocation> for Allocation {
    fn from(value: InMemoryAllocation) -> Self {
        Box::new(value)
    }
}

#[derive(Debug, Clone, Default)]
pub struct Allocations {
    asset_allocations: HashMap<AssetId, f64>,
}
impl Allocations {
    pub fn new(asset_allocations: HashMap<AssetId, f64>) -> Self {
        Self { asset_allocations }
    }
}

impl AllocationTrait for InMemoryAllocation {
    fn allocation_for_asset_total(&self, asset_id: &AssetId) -> f64 {
        self.cex_assets
            .values()
            .map(|assets| assets.asset_allocations.get(asset_id).unwrap_or(&0.0))
            .sum()
    }
    fn allocation_for_asset_in_cex(&self, asset_id: &AssetId, cex_id: &CexId) -> f64 {
        self.cex_assets
            .get(cex_id)
            .and_then(|assets| assets.asset_allocations.get(asset_id))
            .copied()
            .unwrap_or(0.0)
    }
}

#[derive(Clone, Default)]
pub struct Builder {
    cex_allocations: HashMap<CexId, Allocations>,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            cex_allocations: HashMap::new(),
        }
    }
    pub fn allocation(&mut self, cex_id: CexId, asset_id: AssetId, quantity: f64) -> &mut Self {
        assert!(quantity > 0.0, "allocation must be greater than 0.0");
        assert!(
            quantity <= 100.0,
            "allocation must be less or equal to 100.0"
        );
        self.cex_allocations
            .entry(cex_id)
            .or_insert_with(|| Allocations::new(HashMap::new()))
            .asset_allocations
            .entry(asset_id)
            .and_modify(|prev| *prev += quantity)
            .or_insert(quantity);
        self
    }
    pub fn build(&self) -> InMemoryAllocation {
        InMemoryAllocation::new(self.cex_allocations.clone())
    }
}
