use std::fmt;

/// Errors produced while working with decentralised exchange types.
#[derive(Debug, Clone)]
pub enum DexError {
    /// A blockchain address format was not a valid regular expression.
    InvalidAddressFormat {
        /// The identifier of the blockchain.
        blockchain_id: String,
        /// Details about the invalid address format.
        message: String,
    },
}

impl fmt::Display for DexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DexError::InvalidAddressFormat {
                blockchain_id,
                message,
            } => write!(
                f,
                "Invalid address format for blockchain '{}': {}",
                blockchain_id, message
            ),
        }
    }
}

impl std::error::Error for DexError {}
