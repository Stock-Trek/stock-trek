use crate::parse::{RegistryAttribute, RegistryItem};
use proc_macro2::TokenStream;
use quote::quote;

/// Expands a registry attribute.
pub(crate) struct RegistryExpansion {
    attr: TokenStream,
    item: TokenStream,
}

impl RegistryExpansion {
    /// Captures the attribute and item token streams.
    pub(crate) fn new(attr: TokenStream, item: TokenStream) -> Self {
        Self { attr, item }
    }

    /// Builds the lazily initialised registry wrapper.
    pub(crate) fn expand(self) -> TokenStream {
        let registry_attr = match syn::parse2::<RegistryAttribute>(self.attr) {
            Ok(registry_attr) => registry_attr,
            Err(error) => return error.to_compile_error(),
        };
        let registry_item = match syn::parse2::<RegistryItem>(self.item) {
            Ok(registry_item) => registry_item,
            Err(error) => return error.to_compile_error(),
        };

        let trait_ident = registry_attr.trait_ident;
        let trait_name = trait_ident.to_string();
        let item_ident = registry_item.item.ident;
        let storage_ident =
            syn::parse_str::<syn::Ident>(&format!("{item_ident}__STORAGE")).expect("identifier");
        let wrapper_struct_ident =
            syn::parse_str::<syn::Ident>(&format!("{item_ident}__TraitReg")).expect("identifier");

        quote! {
            static #storage_ident: ::std::sync::OnceLock<
                ::stock_trek::traitreg::TraitRegStorage<Box<dyn #trait_ident>>,
            > = ::std::sync::OnceLock::new();

            static #item_ident: #wrapper_struct_ident = #wrapper_struct_ident;

            struct #wrapper_struct_ident;

            impl ::core::ops::Deref for #wrapper_struct_ident {
                type Target =
                    ::stock_trek::traitreg::TraitRegStorage<Box<dyn #trait_ident>>;

                fn deref(&self) -> &'static Self::Target {
                    #storage_ident.get_or_init(|| {
                        ::stock_trek::traitreg::TraitRegStorage::new(#trait_name)
                    })
                }
            }
        }
    }
}
