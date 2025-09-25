//! Coinbase App APIs responses
//!
//! <https://docs.cdp.coinbase.com/coinbase-app/introduction/welcome>

use std::fmt;

use serde::{Deserialize, Serialize};

/// Coinbase App error message
///
/// <https://docs.cdp.coinbase.com/coinbase-app/api-architecture/error-messages>
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct CoinbaseErrorMessage {
    /// Error message ID
    pub id: String,
    /// Message
    pub message: String,
}

impl fmt::Display for CoinbaseErrorMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.message)
    }
}
