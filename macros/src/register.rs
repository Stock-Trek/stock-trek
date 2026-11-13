use crate::parse::RegisterItem;
use proc_macro2::TokenStream;
use quote::quote;

/// Expands a registration attribute.
pub(crate) struct RegisterExpansion {
    attr: TokenStream,
    item: TokenStream,
}

impl RegisterExpansion {
    /// Captures the attribute and item token streams.
    pub(crate) fn new(attr: TokenStream, item: TokenStream) -> Self {
        Self { attr, item }
    }

    /// Builds the registration implementation and constructor.
    pub(crate) fn expand(self) -> TokenStream {
        if !self.attr.is_empty() {
            return syn::Error::new_spanned(
                self.attr,
                "Registration does not accept arguments; use `#[register_algorithm]`.",
            )
            .to_compile_error();
        }

        let parsed_item = match syn::parse2::<RegisterItem>(self.item.clone()) {
            Ok(parsed_item) => parsed_item,
            Err(error) => return error.to_compile_error(),
        };

        let trait_path = match parsed_item.trait_path() {
            Ok(trait_path) => trait_path,
            Err(error) => return error.to_compile_error(),
        };
        let trait_ident = match trait_path.get_ident() {
            Some(trait_ident) => trait_ident,
            None => {
                return syn::Error::new_spanned(
                    trait_path,
                    "Expected the trait in the implementation to have an identifier.",
                )
                .to_compile_error();
            }
        };

        let type_path = match parsed_item.self_type_path() {
            Ok(type_path) => type_path,
            Err(error) => return error.to_compile_error(),
        };
        let type_ident = match type_path.get_ident() {
            Some(type_ident) => type_ident,
            None => {
                return syn::Error::new_spanned(
                    type_path,
                    "Expected the type in the implementation to have an identifier.",
                )
                .to_compile_error();
            }
        };

        let trait_name = trait_ident.to_string();
        let type_name = type_ident.to_string();
        let register_static_ident =
            syn::parse_str::<syn::Ident>(&format!("{type_ident}_{trait_ident}__Register"))
                .expect("Unable to create identifier");
        let register_static_fn_ident =
            syn::parse_str::<syn::Ident>(&format!("{type_ident}_{trait_ident}__RegisterFn"))
                .expect("Unable to create identifier");

        let mut result = quote! {
            impl ::stock_trek::traitreg::RegisteredImpl<Box<dyn #trait_path>> for #type_path {
                const INSTANCIATE: fn() -> Option<Box<dyn #trait_path>> = || {
                    Some(Box::new(<#type_path as ::core::default::Default>::default()))
                };
                const HAS_CONSTRUCTOR: bool = true;
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

        result.extend(self.item);
        result
    }
}
