//! Decentralised exchange (DEX) support.

/// Kinds of assets that can exist on a blockchain.
pub mod asset_kind;
/// An asset together with the blockchain and network it is deployed on.
pub mod asset_on_chain;
/// Blockchain definitions and address validation.
pub mod blockchain;
/// Networks within a blockchain.
pub mod network;
/// Well-known blockchain and network definitions.
#[allow(missing_docs)]
pub mod networks;
