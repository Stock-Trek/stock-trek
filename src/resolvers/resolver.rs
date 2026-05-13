use crate::{
    asset_id::AssetId, error::result::StockTrekResult,
    capability::HasRequiredCapabilities, order::order_request::OrderRequest,
    resolved_context::ResolvedContext,
};

pub type Resolver = Box<dyn ResolverTrait>;

#[typetag::serde]
pub trait ResolverTrait: HasRequiredCapabilities + Send + Sync {
    fn resolve(
        &self,
        c: &ResolvedContext,
        order_requests: &mut Vec<OrderRequest<AssetId, f64>>,
    ) -> StockTrekResult<()>;
}
