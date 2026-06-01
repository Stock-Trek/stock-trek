use crate::{
    error::result::StockTrekResult,
    verification::{
        file_existence_verifier::FileExistenceVerifier,
        node_location::NodeLocation,
        path_util::{collate, path_string},
        policy::{BlockedLocations, SyntaxPolicy},
        syntax_verifier::SyntaxVerifier,
        verifier_map::{blocked_node_policy, node_to_location, rust_node, VerifierMap},
    },
};

pub fn verify(path: String) -> StockTrekResult<()> {
    let file_existence_verifier = FileExistenceVerifier::new();
    match file_existence_verifier.verify(path) {
        Err(e) => Err(e),
        Ok(contents) => {
            let mut syntax_verifier = syntax_verifier();
            syntax_verifier.verify(&contents)
        }
    }
}

const ALLOWED_ATTRIBUTE_PATH: &str = "register_algorithm";
const ALLOWED_PATH_PREFIXES: &[&str] = &[
    ALLOWED_ATTRIBUTE_PATH,
    "crate::",
    "super::",
    "self::",
    "std::collections::",
    "stock_trek::",
];

fn allowed_by_prefix(path_str: &str) -> bool {
    ALLOWED_PATH_PREFIXES
        .iter()
        .any(|&p| path_str.starts_with(p))
}

fn syntax_verifier() -> SyntaxVerifier {
    let mut verifiers = VerifierMap::new();
    verifiers
        .allow::<rust_node::AngleBracketedGenericArguments>()
        .allow::<rust_node::Arm>()
        .allow::<rust_node::AssocConst>()
        .allow::<rust_node::AssocType>()
        .allow::<rust_node::BinOp>()
        .allow::<rust_node::Block>()
        .allow::<rust_node::Constraint>()
        .allow::<rust_node::Expr>()
        .allow::<rust_node::ExprArray>()
        .allow::<rust_node::ExprAssign>()
        .allow::<rust_node::ExprBinary>()
        .allow::<rust_node::ExprBlock>()
        .allow::<rust_node::ExprBreak>()
        .allow::<rust_node::ExprCall>()
        .allow::<rust_node::ExprCast>()
        .allow::<rust_node::ExprClosure>()
        .allow::<rust_node::ExprConst>()
        .allow::<rust_node::ExprContinue>()
        .allow::<rust_node::ExprField>()
        .allow::<rust_node::ExprForLoop>()
        .allow::<rust_node::ExprGroup>()
        .allow::<rust_node::ExprIf>()
        .allow::<rust_node::ExprIndex>()
        .allow::<rust_node::ExprInfer>()
        .allow::<rust_node::ExprLet>()
        .allow::<rust_node::ExprLit>()
        .allow::<rust_node::ExprLoop>()
        .allow::<rust_node::ExprMatch>()
        .allow::<rust_node::ExprMethodCall>()
        .allow::<rust_node::ExprParen>()
        .allow::<rust_node::ExprRange>()
        .allow::<rust_node::ExprReference>()
        .allow::<rust_node::ExprRepeat>()
        .allow::<rust_node::ExprReturn>()
        .allow::<rust_node::ExprStruct>()
        .allow::<rust_node::ExprTuple>()
        .allow::<rust_node::ExprUnary>()
        .allow::<rust_node::ExprWhile>()
        .allow::<rust_node::Field>()
        .allow::<rust_node::FieldPat>()
        .allow::<rust_node::FieldValue>()
        .allow::<rust_node::Fields>()
        .allow::<rust_node::FieldsNamed>()
        .allow::<rust_node::FieldsUnnamed>()
        .allow::<rust_node::File>()
        .allow::<rust_node::FnArg>()
        .allow::<rust_node::GenericArgument>()
        .allow::<rust_node::GenericParam>()
        .allow::<rust_node::Generics>()
        .allow::<rust_node::Ident>()
        .allow::<rust_node::ImplItem>()
        .allow::<rust_node::ImplItemConst>()
        .allow::<rust_node::ImplItemFn>()
        .allow::<rust_node::ImplItemType>()
        .allow::<rust_node::Index>()
        .allow::<rust_node::Item>()
        .allow::<rust_node::ItemConst>()
        .allow::<rust_node::ItemEnum>()
        .allow::<rust_node::ItemFn>()
        .allow::<rust_node::ItemImpl>()
        .allow::<rust_node::ItemStruct>()
        .allow::<rust_node::ItemTrait>()
        .allow::<rust_node::ItemTraitAlias>()
        .allow::<rust_node::ItemType>()
        .allow::<rust_node::ItemUnion>()
        .allow::<rust_node::Label>()
        .allow::<rust_node::Lifetime>()
        .allow::<rust_node::LifetimeParam>()
        .allow::<rust_node::Lit>()
        .allow::<rust_node::LitBool>()
        .allow::<rust_node::LitByte>()
        .allow::<rust_node::LitByteStr>()
        .allow::<rust_node::LitCStr>()
        .allow::<rust_node::LitChar>()
        .allow::<rust_node::LitFloat>()
        .allow::<rust_node::LitInt>()
        .allow::<rust_node::LitStr>()
        .allow::<rust_node::Local>()
        .allow::<rust_node::Member>()
        .allow::<rust_node::Meta>()
        .allow::<rust_node::MetaList>()
        .allow::<rust_node::MetaNameValue>()
        .allow::<rust_node::ParenthesizedGenericArguments>()
        .allow::<rust_node::Pat>()
        .allow::<rust_node::PatIdent>()
        .allow::<rust_node::PatOr>()
        .allow::<rust_node::PatParen>()
        .allow::<rust_node::PatReference>()
        .allow::<rust_node::PatRest>()
        .allow::<rust_node::PatSlice>()
        .allow::<rust_node::PatStruct>()
        .allow::<rust_node::PatTuple>()
        .allow::<rust_node::PatTupleStruct>()
        .allow::<rust_node::PatType>()
        .allow::<rust_node::PatWild>()
        .allow::<rust_node::PathArguments>()
        .allow::<rust_node::PathSegment>()
        .allow::<rust_node::PredicateLifetime>()
        .allow::<rust_node::PredicateType>()
        .allow::<rust_node::QSelf>()
        .allow::<rust_node::RangeLimits>()
        .allow::<rust_node::Receiver>()
        .allow::<rust_node::ReturnType>()
        .allow::<rust_node::Signature>()
        .allow::<rust_node::Stmt>()
        .allow::<rust_node::TraitBound>()
        .allow::<rust_node::TraitBoundModifier>()
        .allow::<rust_node::TraitItem>()
        .allow::<rust_node::TraitItemConst>()
        .allow::<rust_node::TraitItemFn>()
        .allow::<rust_node::TraitItemType>()
        .allow::<rust_node::Type>()
        .allow::<rust_node::TypeArray>()
        .allow::<rust_node::TypeGroup>()
        .allow::<rust_node::TypeImplTrait>()
        .allow::<rust_node::TypeInfer>()
        .allow::<rust_node::TypeNever>()
        .allow::<rust_node::TypeParam>()
        .allow::<rust_node::TypeParamBound>()
        .allow::<rust_node::TypeParen>()
        .allow::<rust_node::TypePath>()
        .allow::<rust_node::TypeReference>()
        .allow::<rust_node::TypeSlice>()
        .allow::<rust_node::TypeTraitObject>()
        .allow::<rust_node::TypeTuple>()
        .allow::<rust_node::UnOp>()
        .allow::<rust_node::UseGroup>()
        .allow::<rust_node::UseName>()
        .allow::<rust_node::UsePath>()
        .allow::<rust_node::UseTree>()
        .allow::<rust_node::Variant>()
        .allow::<rust_node::VisRestricted>()
        .allow::<rust_node::Visibility>()
        .allow::<rust_node::WhereClause>()
        .allow::<rust_node::WherePredicate>();
    //

    fn verify_node_path<N: syn::spanned::Spanned>(node: &N, path: &str) -> SyntaxPolicy {
        if allowed_by_prefix(path) || !path.contains("::") {
            SyntaxPolicy::Allowed
        } else {
            blocked_node_policy(node)
        }
    }

    verifiers
        .scope::<rust_node::Attribute>(|node| {
            let is_allowed = match &node.meta {
                syn::Meta::Path(path) => ALLOWED_ATTRIBUTE_PATH == path_string(path),
                _ => false,
            };
            if is_allowed {
                SyntaxPolicy::Allowed
            } else {
                blocked_node_policy(node)
            }
        })
        .scope::<rust_node::ExprPath>(|node| {
            let expr_path_str = &path_string(&node.path);
            verify_node_path(node, expr_path_str)
        })
        .scope::<rust_node::Path>(|node| {
            let path_str = &path_string(node);
            verify_node_path(node, path_str)
        })
        .scope::<rust_node::ItemUse>(|node| {
            let locations: Vec<NodeLocation> = collate(&node.tree)
                .iter()
                .filter(|path| !allowed_by_prefix(path))
                .map(|_| node_to_location(node))
                .collect();
            if locations.is_empty() {
                SyntaxPolicy::Allowed
            } else {
                SyntaxPolicy::Blocked(BlockedLocations { locations })
            }
        });
    //
    verifiers
        .block::<rust_node::Abi>()
        .block::<rust_node::BareFnArg>()
        .block::<rust_node::BareVariadic>()
        .block::<rust_node::BoundLifetimes>()
        .block::<rust_node::CapturedParam>()
        .block::<rust_node::ConstParam>()
        .block::<rust_node::DeriveInput>()
        .block::<rust_node::ExprAsync>()
        .block::<rust_node::ExprAwait>()
        .block::<rust_node::ExprMacro>()
        .block::<rust_node::ExprRawAddr>()
        .block::<rust_node::ExprTry>()
        .block::<rust_node::ExprTryBlock>()
        .block::<rust_node::ExprUnsafe>()
        .block::<rust_node::ExprYield>()
        .block::<rust_node::ForeignItem>()
        .block::<rust_node::ForeignItemFn>()
        .block::<rust_node::ForeignItemMacro>()
        .block::<rust_node::ForeignItemStatic>()
        .block::<rust_node::ForeignItemType>()
        .block::<rust_node::ImplItemMacro>()
        .block::<rust_node::ItemExternCrate>()
        .block::<rust_node::ItemForeignMod>()
        .block::<rust_node::ItemMacro>()
        .block::<rust_node::ItemMod>()
        .block::<rust_node::ItemStatic>()
        .block::<rust_node::Macro>()
        .block::<rust_node::PointerMutability>()
        .block::<rust_node::PreciseCapture>()
        .block::<rust_node::StaticMutability>()
        .block::<rust_node::StmtMacro>()
        .block::<rust_node::TraitItemMacro>()
        .block::<rust_node::TypeBareFn>()
        .block::<rust_node::TypeMacro>()
        .block::<rust_node::TypePtr>()
        .block::<rust_node::UseGlob>()
        .block::<rust_node::UseRename>()
        .block::<rust_node::Variadic>();

    SyntaxVerifier::new(verifiers)
}
