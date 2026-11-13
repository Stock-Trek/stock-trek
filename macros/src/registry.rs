use crate::parse::{RegistryAttribute, RegistryItem};
use quote::quote;

/// Expands a `registry` attribute.
pub(crate) struct RegistryExpansion {
    attr: proc_macro2::TokenStream,
    item: proc_macro2::TokenStream,
}

impl RegistryExpansion {
    /// Captures the attribute and item token streams.
    pub(crate) fn new(attr: proc_macro2::TokenStream, item: proc_macro2::TokenStream) -> Self {
        Self { attr, item }
    }

    /// Builds the lazily initialised registry wrapper.
    pub(crate) fn expand(self) -> proc_macro2::TokenStream {
        let registry_attr =
            syn::parse2::<RegistryAttribute>(self.attr).unwrap_or_else(|error| panic!("{error}"));
        let registry_item =
            syn::parse2::<RegistryItem>(self.item).unwrap_or_else(|error| panic!("{error}"));

        let trait_ident = registry_attr.trait_ident;
        let trait_name = trait_ident.to_string();
        let item_ident = registry_item.item.ident;
        let storage_ident = syn::parse_str::<syn::Ident>(format!("{item_ident}__STORAGE").as_ref())
            .expect("Unable to create identifier");
        let wrapper_struct_ident =
            syn::parse_str::<syn::Ident>(format!("{item_ident}__TraitReg").as_ref())
                .expect("Unable to create identifier");

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
