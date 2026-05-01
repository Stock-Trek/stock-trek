pub mod actions;
pub mod assembler_context;
pub mod assemblers;
pub mod example;
pub mod exchanges;
pub mod market_data;
pub mod portfolio;
pub mod predicates;
pub mod scratch_pad;
pub mod statistics;
pub mod strategy;
pub mod strategy_context;
pub mod util;
pub mod values;
pub mod verification;

pub mod prelude {
    pub use crate::{
        actions::actions::Actions,
        assemblers::{assembler::Assembler, assemblers::Assemblers},
        portfolio::portfolios::Portfolios,
        predicates::predicates::Predicates,
        scratch_pad::{ScratchPad, ScratchValue},
        strategy::Strategy,
        strategy_context::StrategyContext,
        values::values::Values,
    };

    pub use digdigdig3::{Asset, ExchangeId, Symbol};

    pub use traitreg;
    pub use traitreg::register as register_strategy;

    pub type TimestampMillis = u64;
}
