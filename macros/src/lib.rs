//! Attribute macros for registering stock-trek trait implementations.
//!
//! The macros mirror the `traitreg` API but add WebAssembly constructor
//! support and build the registry lazily on first access.

use proc_macro::TokenStream;

mod parse;
mod register;
mod registry;

/// Registers an implementation of a trait on a concrete type.
///
/// The attribute accepts an optional constructor method with the signature
/// `fn() -> Self`, for example `#[register(default)]` or `#[register(new)]`.
#[proc_macro_attribute]
pub fn register(attr: TokenStream, item: TokenStream) -> TokenStream {
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
