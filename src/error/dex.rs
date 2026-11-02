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
    /// An asset address did not match its blockchain's address format.
    InvalidAddress {
        /// The identifier of the blockchain.
        blockchain_id: String,
        /// The address that failed validation.
        address: String,
    },
    /// A network does not belong to the blockchain it was paired with.
    InvalidNetwork {
        /// The identifier of the blockchain.
        blockchain_id: String,
        /// The identifier of the network.
        network_id: String,
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
            DexError::InvalidAddress {
                blockchain_id,
                address,
            } => write!(
                f,
                "Invalid address '{}' for blockchain '{}'",
                address, blockchain_id
            ),
            DexError::InvalidNetwork {
                blockchain_id,
                network_id,
            } => write!(
                f,
                "Network '{}' does not belong to blockchain '{}'",
                network_id, blockchain_id
            ),
        }
    }
}

impl std::error::Error for DexError {}
