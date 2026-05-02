use crate::{portfolios::portfolio::Portfolio, scratch::scratch_pad::ScratchPad};

pub struct ResolvedContext {
    pub portfolio: Portfolio,
    pub scratch_pad: ScratchPad,
}
