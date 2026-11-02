use crate::dex::string_id::string_id_funcs;
use serde::{Deserialize, Serialize};

/// Unique identifier for a blockchain.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct BlockchainId(std::borrow::Cow<'static, str>);

string_id_funcs!(BlockchainId);
