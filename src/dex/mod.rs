//! Decentralised exchange (DEX) support.
//!
//! DEX identifiers differ from CEX identifiers: an asset is identified by the
//! blockchain and network it is deployed on, plus either its native status or a
//! contract address. Each [`Blockchain`](crate::dex::blockchain::Blockchain)
//! defines the address format for its contracts, and
//! [`AssetOnChain`](crate::dex::asset_on_chain::AssetOnChain) validates
//! addresses against it. Use
//! [`networks`](crate::dex::networks) for well-known blockchain and network
//! definitions.

/// Kinds of assets that can exist on a blockchain.
pub mod asset_kind;
/// An asset together with the blockchain and network it is deployed on.
pub mod asset_on_chain;
/// Blockchain definitions and address validation.
pub mod blockchain;
/// Networks within a blockchain.
pub mod network;
/// Well-known blockchain and network definitions.
pub mod networks;
/// Shared implementation for string-backed identifiers.
mod string_id;
