use crate::{
    assemblers::assembler::Assembler, scratch_pad::ScratchPad, strategy_context::StrategyContext,
};
use anyhow::Result;

pub trait Strategy: Send + Sync {
    fn portfolio_response(&self) -> Result<Assembler>;
    fn market_calculations(&self, context: StrategyContext) -> Result<ScratchPad>;
}
