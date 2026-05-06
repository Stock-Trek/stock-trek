use crate::verification::{
    node_location::NodeLocation,
    path_util::last_segment_name,
    policy::{BlockedLocations, SyntaxPolicy},
    verifier_map::rust_node::*,
};
use std::{any::Any, collections::HashMap};
use syn::spanned::Spanned;

pub trait SyntaxVariant {
    type SynType: Spanned + 'static;
    const ELEMENT: SyntaxElement;
}

type VerifierFn = Box<dyn Fn(&dyn Any) -> SyntaxPolicy>;

pub struct VerifierMap {
    verifiers: HashMap<&'static str, VerifierFn>,
}

impl Default for VerifierMap {
    fn default() -> Self {
        Self::new()
    }
}

impl VerifierMap {
    pub fn new() -> Self {
        VerifierMap {
            verifiers: HashMap::new(),
        }
    }
    pub fn variant_type_name<V: SyntaxVariant>(&self) -> &'static str {
        last_segment_name::<V::SynType>()
    }
    pub fn node_type_name<N: Spanned + 'static>(&self) -> &'static str {
        last_segment_name::<N>()
    }
    pub fn allow<V: SyntaxVariant>(&mut self) -> &mut Self {
        self.verifiers.insert(
            self.variant_type_name::<V>(),
            Box::new(|_| SyntaxPolicy::Allowed),
        );
        self
    }
    pub fn block<V: SyntaxVariant>(&mut self) -> &mut Self {
        self.verifiers.insert(
            self.variant_type_name::<V>(),
            Box::new(|node| {
                let typed_node = node.downcast_ref::<V::SynType>().unwrap();
                SyntaxPolicy::Blocked(BlockedLocations {
                    locations: vec![NodeLocation::from(typed_node)],
                })
            }),
        );
        self
    }
    pub fn scope<V: SyntaxVariant>(
        &mut self,
        verifier: impl Fn(&V::SynType) -> SyntaxPolicy + 'static,
    ) -> &mut Self {
        self.verifiers.insert(
            self.variant_type_name::<V>(),
            Box::new(move |any: &dyn Any| {
                let node = any.downcast_ref::<V::SynType>().unwrap();
                verifier(node)
            }),
        );
        self
    }
    pub fn verify_node<N: Spanned + 'static>(&mut self, node: &N) -> SyntaxPolicy {
        let node_name = self.node_type_name::<N>();
        let policy = self
            .verifiers
            .get(node_name)
            .map(|f| f(node))
            .unwrap_or(blocked_node_policy(node));
        policy
    }
}

pub fn blocked_node_policy<T: Spanned>(node: &T) -> SyntaxPolicy {
    SyntaxPolicy::Blocked(BlockedLocations {
        locations: vec![node_to_location(node)],
    })
}

pub fn node_to_location<T: Spanned>(node: &T) -> NodeLocation {
    let start = node.span().start();
    NodeLocation {
        line: start.line,
        column: start.column,
    }
}

macro_rules! syntax_elements {
    ($($variant:ident => $syn:ty),* $(,)?) => {

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum SyntaxElement {
            $($variant),*
        }

        pub mod rust_node {
            $(
                #[derive(Debug, Clone, Copy)]
                pub struct $variant;
            )*
        }

        $(
            impl SyntaxVariant for $variant {
                type SynType = $syn;
                const ELEMENT: SyntaxElement = SyntaxElement::$variant;
            }
        )*
    };
}

syntax_elements! {
    Abi => syn::Abi,
    AngleBracketedGenericArguments => syn::AngleBracketedGenericArguments,
    Arm => syn::Arm,
    AssocConst => syn::AssocConst,
    AssocType => syn::AssocType,
    Attribute => syn::Attribute,
    BareFnArg => syn::BareFnArg,
    BareVariadic => syn::BareVariadic,
    BinOp => syn::BinOp,
    Block => syn::Block,
    BoundLifetimes => syn::BoundLifetimes,
    CapturedParam => syn::CapturedParam,
    ConstParam => syn::ConstParam,
    Constraint => syn::Constraint,
    DeriveInput => syn::DeriveInput,
    Expr => syn::Expr,
    ExprArray => syn::ExprArray,
    ExprAssign => syn::ExprAssign,
    ExprAsync => syn::ExprAsync,
    ExprAwait => syn::ExprAwait,
    ExprBinary => syn::ExprBinary,
    ExprBlock => syn::ExprBlock,
    ExprBreak => syn::ExprBreak,
    ExprCall => syn::ExprCall,
    ExprCast => syn::ExprCast,
    ExprClosure => syn::ExprClosure,
    ExprConst => syn::ExprConst,
    ExprContinue => syn::ExprContinue,
    ExprField => syn::ExprField,
    ExprForLoop => syn::ExprForLoop,
    ExprGroup => syn::ExprGroup,
    ExprIf => syn::ExprIf,
    ExprIndex => syn::ExprIndex,
    ExprInfer => syn::ExprInfer,
    ExprLet => syn::ExprLet,
    ExprLit => syn::ExprLit,
    ExprLoop => syn::ExprLoop,
    ExprMacro => syn::ExprMacro,
    ExprMatch => syn::ExprMatch,
    ExprMethodCall => syn::ExprMethodCall,
    ExprParen => syn::ExprParen,
    ExprPath => syn::ExprPath,
    ExprRange => syn::ExprRange,
    ExprRawAddr => syn::ExprRawAddr,
    ExprReference => syn::ExprReference,
    ExprRepeat => syn::ExprRepeat,
    ExprReturn => syn::ExprReturn,
    ExprStruct => syn::ExprStruct,
    ExprTry => syn::ExprTry,
    ExprTryBlock => syn::ExprTryBlock,
    ExprTuple => syn::ExprTuple,
    ExprUnary => syn::ExprUnary,
    ExprUnsafe => syn::ExprUnsafe,
    ExprWhile => syn::ExprWhile,
    ExprYield => syn::ExprYield,
    Field => syn::Field,
    FieldPat => syn::FieldPat,
    FieldValue => syn::FieldValue,
    Fields => syn::Fields,
    FieldsNamed => syn::FieldsNamed,
    FieldsUnnamed => syn::FieldsUnnamed,
    File => syn::File,
    FnArg => syn::FnArg,
    ForeignItem => syn::ForeignItem,
    ForeignItemFn => syn::ForeignItemFn,
    ForeignItemMacro => syn::ForeignItemMacro,
    ForeignItemStatic => syn::ForeignItemStatic,
    ForeignItemType => syn::ForeignItemType,
    GenericArgument => syn::GenericArgument,
    GenericParam => syn::GenericParam,
    Generics => syn::Generics,
    Ident => syn::Ident,
    ImplItem => syn::ImplItem,
    ImplItemConst => syn::ImplItemConst,
    ImplItemFn => syn::ImplItemFn,
    ImplItemMacro => syn::ImplItemMacro,
    ImplItemType => syn::ImplItemType,
    Index => syn::Index,
    Item => syn::Item,
    ItemConst => syn::ItemConst,
    ItemEnum => syn::ItemEnum,
    ItemExternCrate => syn::ItemExternCrate,
    ItemFn => syn::ItemFn,
    ItemForeignMod => syn::ItemForeignMod,
    ItemImpl => syn::ItemImpl,
    ItemMacro => syn::ItemMacro,
    ItemMod => syn::ItemMod,
    ItemStatic => syn::ItemStatic,
    ItemStruct => syn::ItemStruct,
    ItemTrait => syn::ItemTrait,
    ItemTraitAlias => syn::ItemTraitAlias,
    ItemType => syn::ItemType,
    ItemUnion => syn::ItemUnion,
    ItemUse => syn::ItemUse,
    Label => syn::Label,
    Lifetime => syn::Lifetime,
    LifetimeParam => syn::LifetimeParam,
    Lit => syn::Lit,
    LitBool => syn::LitBool,
    LitByte => syn::LitByte,
    LitByteStr => syn::LitByteStr,
    LitCStr => syn::LitCStr,
    LitChar => syn::LitChar,
    LitFloat => syn::LitFloat,
    LitInt => syn::LitInt,
    LitStr => syn::LitStr,
    Local => syn::Local,
    Macro => syn::Macro,
    Member => syn::Member,
    Meta => syn::Meta,
    MetaList => syn::MetaList,
    MetaNameValue => syn::MetaNameValue,
    ParenthesizedGenericArguments => syn::ParenthesizedGenericArguments,
    Pat => syn::Pat,
    PatIdent => syn::PatIdent,
    PatOr => syn::PatOr,
    PatParen => syn::PatParen,
    PatReference => syn::PatReference,
    PatRest => syn::PatRest,
    PatSlice => syn::PatSlice,
    PatStruct => syn::PatStruct,
    PatTuple => syn::PatTuple,
    PatTupleStruct => syn::PatTupleStruct,
    PatType => syn::PatType,
    PatWild => syn::PatWild,
    Path => syn::Path,
    PathArguments => syn::PathArguments,
    PathSegment => syn::PathSegment,
    PointerMutability => syn::PointerMutability,
    PreciseCapture => syn::PreciseCapture,
    PredicateLifetime => syn::PredicateLifetime,
    PredicateType => syn::PredicateType,
    QSelf => syn::QSelf,
    RangeLimits => syn::RangeLimits,
    Receiver => syn::Receiver,
    ReturnType => syn::ReturnType,
    Signature => syn::Signature,
    StaticMutability => syn::StaticMutability,
    Stmt => syn::Stmt,
    StmtMacro => syn::StmtMacro,
    TraitBound => syn::TraitBound,
    TraitBoundModifier => syn::TraitBoundModifier,
    TraitItem => syn::TraitItem,
    TraitItemConst => syn::TraitItemConst,
    TraitItemFn => syn::TraitItemFn,
    TraitItemMacro => syn::TraitItemMacro,
    TraitItemType => syn::TraitItemType,
    Type => syn::Type,
    TypeArray => syn::TypeArray,
    TypeBareFn => syn::TypeBareFn,
    TypeGroup => syn::TypeGroup,
    TypeImplTrait => syn::TypeImplTrait,
    TypeInfer => syn::TypeInfer,
    TypeMacro => syn::TypeMacro,
    TypeNever => syn::TypeNever,
    TypeParam => syn::TypeParam,
    TypeParamBound => syn::TypeParamBound,
    TypeParen => syn::TypeParen,
    TypePath => syn::TypePath,
    TypePtr => syn::TypePtr,
    TypeReference => syn::TypeReference,
    TypeSlice => syn::TypeSlice,
    TypeTraitObject => syn::TypeTraitObject,
    TypeTuple => syn::TypeTuple,
    UnOp => syn::UnOp,
    UseGlob => syn::UseGlob,
    UseGroup => syn::UseGroup,
    UseName => syn::UseName,
    UsePath => syn::UsePath,
    UseRename => syn::UseRename,
    UseTree => syn::UseTree,
    Variadic => syn::Variadic,
    Variant => syn::Variant,
    VisRestricted => syn::VisRestricted,
    Visibility => syn::Visibility,
    WhereClause => syn::WhereClause,
    WherePredicate => syn::WherePredicate,
}
