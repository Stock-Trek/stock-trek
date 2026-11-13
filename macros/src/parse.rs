use syn::{
    Ident,
    parse::{Parse, ParseStream},
};

/// The trait implementation that a registration attribute is attached to.
pub(crate) struct RegisterItem {
    pub(crate) item: syn::ItemImpl,
}

impl RegisterItem {
    /// Returns the implemented trait's path.
    pub(crate) fn trait_path(&self) -> syn::Result<&syn::Path> {
        let (trait_not, trait_path, _) = self.item.trait_.as_ref().ok_or_else(|| {
            syn::Error::new_spanned(
                &self.item.self_ty,
                "Can only register a trait implementation, 'impl <Trait> for <Type>'.",
            )
        })?;

        if trait_not.is_some() {
            return Err(syn::Error::new_spanned(
                trait_path,
                "Cannot register an inverted trait implementation.",
            ));
        }

        Ok(trait_path)
    }

    /// Returns the implementing type's path.
    pub(crate) fn self_type_path(&self) -> syn::Result<&syn::Path> {
        if let syn::Type::Path(type_path) = &*self.item.self_ty {
            return Ok(&type_path.path);
        }

        Err(syn::Error::new_spanned(
            &self.item.self_ty,
            "Cannot register an implementation on this type, expected a path type.",
        ))
    }
}

impl Parse for RegisterItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            item: syn::ItemImpl::parse(input)?,
        })
    }
}

/// The trait named by a registry attribute.
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

/// The static item that a registry attribute is attached to.
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
