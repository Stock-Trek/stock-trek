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
    /// An identifier was empty or whitespace-only.
    InvalidIdentifier {
        /// The kind of identifier that was invalid.
        identifier_type: &'static str,
        /// The invalid identifier value.
        value: String,
    },
    /// A contract address did not match the blockchain's address format.
    InvalidContractAddress {
        /// The identifier of the blockchain.
        blockchain_id: String,
        /// The invalid contract address.
        address: String,
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
            DexError::InvalidIdentifier {
                identifier_type,
                value,
            } => write!(
                f,
                "Invalid {} '{}': identifiers must not be empty or whitespace-only",
                identifier_type, value
            ),
            DexError::InvalidContractAddress {
                blockchain_id,
                address,
            } => write!(
                f,
                "Invalid contract address '{}' for blockchain '{}'",
                address, blockchain_id
            ),
        }
    }
}

impl std::error::Error for DexError {}
