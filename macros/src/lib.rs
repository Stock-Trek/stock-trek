//! Attribute macros for registering stock-trek trait implementations.
//!
//! Registration is performed exclusively with
//! `#[register_algorithm]` on a trait implementation. Every registered
//! implementation is constructed through [`Default`] when a registry is read,
//! which keeps the API independent of platform-specific constructor ordering.

use proc_macro::TokenStream;

mod parse;
mod register;
mod registry;

/// Registers an implementation of a trait on a concrete type.
///
/// The attribute takes no arguments and is only valid on a trait
/// implementation. The implementing type must implement [`Default`], which is
/// used to construct it when the registry is read.
#[proc_macro_attribute]
pub fn register_algorithm(attr: TokenStream, item: TokenStream) -> TokenStream {
    register::RegisterExpansion::new(attr.into(), item.into())
        .expand()
        .into()
}

/// Builds a registry of all implementations registered for a trait.
///
/// The annotated static item is replaced by a value that dereferences to a
/// lazily initialised registry.
#[proc_macro_attribute]
pub fn registry(attr: TokenStream, item: TokenStream) -> TokenStream {
    registry::RegistryExpansion::new(attr.into(), item.into())
        .expand()
        .into()
}
