use syn::{
    Ident,
    parse::{Parse, ParseStream},
};

/// The optional constructor method named by a `register` attribute.
pub(crate) struct RegisterAttribute {
    pub(crate) constructor_fn_ident: Ident,
}

impl Parse for RegisterAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            constructor_fn_ident: Ident::parse(input)?,
        })
    }
}

/// The trait implementation that a `register` attribute is attached to.
pub(crate) struct RegisterItem {
    pub(crate) item: syn::ItemImpl,
}

impl RegisterItem {
    /// Returns the path of the implementing type.
    pub(crate) fn self_type_path(&self) -> &syn::Path {
        if let syn::Type::Path(type_path) = &*self.item.self_ty {
            return &type_path.path;
        }

        let error_type = match &*self.item.self_ty {
            syn::Type::Array(_) => "n array",
            syn::Type::BareFn(_) => " function",
            syn::Type::Group(_) => " group",
            syn::Type::ImplTrait(_) => " trait impl",
            syn::Type::Infer(_) => "n inferred type (_)",
            syn::Type::Macro(_) => " macro",
            syn::Type::Never(_) => " never type",
            syn::Type::Paren(_) => " parenthesis",
            syn::Type::Ptr(_) => " pointer",
            syn::Type::Reference(_) => " reference",
            syn::Type::Slice(_) => " slice",
            syn::Type::TraitObject(_) => " trait object",
            syn::Type::Tuple(_) => " tuple",
            syn::Type::Verbatim(_) => "n unknown syntax",
            _ => unreachable!(),
        };

        panic!(
            "Cannot register implementation on a{}, expected a struct, enum, union or type alias.",
            error_type
        )
    }
}

impl Parse for RegisterItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            item: syn::ItemImpl::parse(input)?,
        })
    }
}

/// The trait named by a `registry` attribute.
pub(crate) struct RegistryAttribute {
    pub(crate) trait_ident: Ident,
}

impl Parse for RegistryAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            trait_ident: Ident::parse(input)?,
        })
    }
}

/// The static item that a `registry` attribute is attached to.
pub(crate) struct RegistryItem {
    pub(crate) item: syn::ItemStatic,
}

impl Parse for RegistryItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            item: syn::ItemStatic::parse(input)?,
        })
    }
}
