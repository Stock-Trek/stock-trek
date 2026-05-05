use crate::{
    exchanges::exchange_capabilities::ExchangeCapabilities, portfolios::portfolio::Portfolio,
    scratch::scratch_pad::ScratchPad,
};

pub struct ResolvedContext {
    pub exchange_capabilities: ExchangeCapabilities,
    pub portfolio: Portfolio,
    pub scratch_pad: ScratchPad,
}
