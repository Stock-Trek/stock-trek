use crate::dex::string_id::string_id_funcs;
use serde::{Deserialize, Serialize};

/// Identifier for a blockchain family, such as EVM.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct FamilyId(std::borrow::Cow<'static, str>);

string_id_funcs!(FamilyId);
