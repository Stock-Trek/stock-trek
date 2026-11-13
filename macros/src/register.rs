use crate::parse::{RegisterAttribute, RegisterItem};
use quote::quote;

/// Expands a `register` attribute.
pub(crate) struct RegisterExpansion {
    attr: proc_macro2::TokenStream,
    item: proc_macro2::TokenStream,
}

impl RegisterExpansion {
    /// Captures the attribute and item token streams.
    pub(crate) fn new(attr: proc_macro2::TokenStream, item: proc_macro2::TokenStream) -> Self {
        Self { attr, item }
    }

    /// Builds the registration implementation and constructor.
    pub(crate) fn expand(self) -> proc_macro2::TokenStream {
        let constructor_fn = if self.attr.is_empty() {
            None
        } else {
            Some(syn::parse2::<RegisterAttribute>(self.attr).expect("Invalid register attribute"))
        };

        let has_constructor = constructor_fn.is_some();
        let constructor_fn_call = match constructor_fn {
            Some(attribute) => {
                let ident = attribute.constructor_fn_ident;
                quote! { Some(Box::new(Self::#ident())) }
            }
            None => quote! { None },
        };

        let item_clone = self.item.clone();
        let parsed_item =
            syn::parse2::<RegisterItem>(self.item).unwrap_or_else(|error| panic!("{error}"));
        let item_impl = &parsed_item.item;

        let (trait_not, trait_path, _) = item_impl
            .trait_
            .as_ref()
            .expect("Can only register an implementation of a trait, 'impl <Trait> for <Type>'.");
        assert!(
            trait_not.is_none(),
            "Cannot register inverted impl trait: 'impl !Trait for Type'."
        );

        let trait_ident = trait_path
            .require_ident()
            .expect("Expected trait in impl block to have an identifier.");
        let trait_name = trait_ident.to_string();

        let type_path = parsed_item.self_type_path();
        let type_ident = type_path
            .require_ident()
            .expect("Expected type in impl block to have an identifier.");
        let type_name = type_ident.to_string();

        let register_static_ident =
            syn::parse_str::<syn::Ident>(format!("{type_ident}_{trait_ident}__Register").as_ref())
                .expect("Unable to create identifier");
        let register_static_fn_ident = syn::parse_str::<syn::Ident>(
            format!("{type_ident}_{trait_ident}__RegisterFn").as_ref(),
        )
        .expect("Unable to create identifier");

        let mut result = quote! {
            impl ::stock_trek::traitreg::RegisteredImpl<Box<dyn #trait_path>> for #type_path {
                const INSTANCIATE: fn() -> Option<Box<dyn #trait_path>> = || {
                    #constructor_fn_call
                };
                const HAS_CONSTRUCTOR: bool = #has_constructor;
                const NAME: &'static str = #type_name;
                const PATH: &'static str = stringify!(#type_path);
                const FILE: &'static str = ::core::file!();
                const MODULE_PATH: &'static str = ::core::module_path!();
                const TRAIT_NAME: &'static str = #trait_name;
            }

            #[used]
            #[cfg_attr(any(target_os = "linux", target_os = "android"), unsafe(link_section = ".init_array.10000"))]
            #[cfg_attr(target_os = "freebsd", unsafe(link_section = ".init_array.10000"))]
            #[cfg_attr(target_os = "netbsd", unsafe(link_section = ".init_array.10000"))]
            #[cfg_attr(target_os = "openbsd", unsafe(link_section = ".init_array.10000"))]
            #[cfg_attr(target_os = "dragonfly", unsafe(link_section = ".init_array.10000"))]
            #[cfg_attr(target_os = "illumos", unsafe(link_section = ".init_array.10000"))]
            #[cfg_attr(target_os = "haiku", unsafe(link_section = ".init_array.10000"))]
            #[cfg_attr(target_vendor = "apple", unsafe(link_section = "__DATA,__mod_init_func"))]
            #[cfg_attr(target_family = "wasm", unsafe(link_section = ".init_array"))]
            #[cfg_attr(windows, unsafe(link_section = ".CRT$XCT"))]
            static #register_static_ident: extern "C" fn() = {
                extern "C" fn #register_static_fn_ident() {
                    ::stock_trek::traitreg::Registration::register::<
                        Box<dyn #trait_path>,
                        #type_path,
                    >();
                }
                #register_static_fn_ident
            };
        };

        result.extend(item_clone);
        result
    }
}
