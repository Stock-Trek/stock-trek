//! Attribute macros for registering stock-trek trait implementations.
//!
//! Registration is performed exclusively with `#[register_algorithm]` on a
//! trait implementation. Annotated implementations are constructed through
//! [`Default`] when registrations are read, which keeps the API independent of
//! platform-specific constructor ordering.

use proc_macro::TokenStream;

mod parse;
mod register;

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
