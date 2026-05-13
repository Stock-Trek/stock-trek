use crate::{portfolios::portfolio::Portfolio, scratch::scratch_pad::ScratchPad};
use rust_decimal::RoundingStrategy;

pub struct ResolvedContext {
    pub price_rounding: RoundingStrategy,
    pub quantity_rounding: RoundingStrategy,
    pub rate_rounding: RoundingStrategy,
    pub portfolio: Portfolio,
    pub scratch_pad: ScratchPad,
}
