//! Coinbase App APIs responses
//!
//! <https://docs.cdp.coinbase.com/coinbase-app/introduction/welcome>

use std::fmt;

use bigdecimal::BigDecimal;
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

#[derive(Deserialize)]
pub(super) struct CoinbaseResponse<T> {
    pub pagination: Option<Pagination>,
    pub data: T,
}

// #[derive(Deserialize)]
// pub(super) enum Order {
//     #[serde(rename = "asc")]
//     Ascending,
//     #[serde(rename = "desc")]
//     Descending,
// }

#[derive(Deserialize)]
pub(super) struct Pagination {
    // pub ending_before: Option<String>,
    // pub starting_after: Option<String>,
    // pub previous_ending_before: Option<String>,
    // pub next_starting_after: Option<String>,
    // pub limit: usize,
    // pub order: Order,
    // pub previous_uri: Option<String>,
    pub next_uri: Option<String>,
}

/// Account
#[derive(Debug, Deserialize)]
pub struct Account {
    // id appears to be either a UUID or a token name e.g: "LINK"
    /// Account ID
    pub id: String,
    /// Account type
    pub r#type: String,
    /// Created at
    pub created_at: Option<String>,
    /// Updated at
    pub updated_at: Option<String>,
    /// Resource
    pub resource: String,
    /// Resource path
    pub resource_path: String,
    /// Account name
    pub name: String,
    /// Primary account
    pub primary: bool,
    /// Account balance
    pub balance: Balance,
    /// Allows deposits
    pub allow_deposits: bool,
    /// Allow withdrawals
    pub allow_withdrawals: bool,
}

/// Account balance
#[derive(Debug, Deserialize)]
pub struct Balance {
    /// Amount
    pub amount: BigDecimal,
    /// Currency
    pub currency: String,
}
