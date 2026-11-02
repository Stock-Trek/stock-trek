//! Well-known blockchain and network definitions.
//!
//! EVM and non-EVM definitions are kept in separate submodules and re-exported
//! here so callers can keep using paths such as `networks::ethereum` and
//! `networks::bitcoin`.

#[macro_use]
mod macros;

mod builders;
mod evm;
mod family_ids;
mod non_evm;

pub use evm::*;
pub use family_ids::*;
pub use non_evm::*;

#[cfg(test)]
mod tests;
