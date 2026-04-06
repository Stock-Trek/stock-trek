pub mod algorithm;
pub mod aligned_window;
pub mod context;
pub mod errors;
pub mod exchange;
pub mod market_data;
pub mod rolling_window;
pub mod signal;
pub mod statistics;
pub mod trading_pair;
pub mod verification;

pub mod window {
    pub use crate::aligned_window::AlignedWindow;
    pub use crate::rolling_window::RollingWindow;
}

pub mod prelude {
    pub use crate::algorithm::StockTrekAlgorithm;
    pub use crate::context::StockTrekContext;
    pub use crate::exchange::Exchange;
    pub use crate::signal::StockTrekEvent;
    pub use crate::signal::StockTrekSignal;
    pub use crate::trading_pair::TradingPair;
    pub use crate::window;

    pub use traitreg;
    pub use traitreg::register as register_algorithm;

    pub type TimestampMillis = u64;
}
