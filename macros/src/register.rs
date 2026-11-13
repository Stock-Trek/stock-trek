use crate::parse::RegisterItem;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

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

        if let Err(error) = parsed_item.ensure_not_generic() {
            return error.to_compile_error();
        }

        let trait_path = match parsed_item.trait_path() {
            Ok(trait_path) => trait_path,
            Err(error) => return error.to_compile_error(),
        };
        let trait_ident = match RegisterItem::final_ident(trait_path) {
            Ok(trait_ident) => trait_ident,
            Err(error) => return error.to_compile_error(),
        };
        let type_path = match parsed_item.self_type_path() {
            Ok(type_path) => type_path,
            Err(error) => return error.to_compile_error(),
        };
        let type_ident = match RegisterItem::final_ident(type_path) {
            Ok(type_ident) => type_ident,
            Err(error) => return error.to_compile_error(),
        };

        let trait_name = trait_ident.to_string();
        let type_name = type_ident.to_string();
        let suffix = format!(
            "{}_{}",
            Self::path_suffix(type_path),
            Self::path_suffix(trait_path)
        );
        let register_static_ident = syn::parse_str::<syn::Ident>(&format!(
            "__STOCK_TREK_REGISTER_{}",
            suffix.to_uppercase()
        ))
        .expect("Unable to create registration static identifier");
        let register_static_fn_ident = syn::parse_str::<syn::Ident>(&format!(
            "__stock_trek_register_{}",
            suffix.to_lowercase()
        ))
        .expect("Unable to create registration function identifier");
        let trait_object = quote!(::std::boxed::Box<dyn #trait_path>);

        let mut result = quote! {
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
                    ::stock_trek::__private::Registration::register::<#trait_object>(
                        ::stock_trek::__private::RegisteredImpl::<#trait_object>::new(
                            || ::std::boxed::Box::new(
                                <#type_path as ::core::default::Default>::default(),
                            ),
                            #type_name,
                            stringify!(#type_path),
                            ::core::file!(),
                            ::core::module_path!(),
                            #trait_name,
                        ),
                    );
                }
                #register_static_fn_ident
            };
        };

        result.extend(self.item);
        result
    }

    /// Builds an identifier-safe suffix from a path.
    fn path_suffix(path: &syn::Path) -> String {
        let mut suffix = path.to_token_stream().to_string();
        suffix.retain(|character| character.is_alphanumeric() || character == '_');

        if suffix.is_empty() {
            String::from("path")
        } else {
            suffix
        }
    }
}
