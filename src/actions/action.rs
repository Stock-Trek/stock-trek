use crate::{
    actions::resolved_action::ResolvedAction, cex::capability::HasRequiredCapabilities,
    resolveable::Resolvable,
};

pub type Action = Box<dyn ActionTrait>;

#[typetag::serde]
pub trait ActionTrait: Resolvable<ResolvedAction> + HasRequiredCapabilities + Send + Sync {}
