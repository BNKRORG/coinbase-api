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
    // NOTE: the ID appears to be either a UUID or a token name e.g: "BTC"
    /// Account ID
    pub id: String,
    /// User or system defined name
    pub name: String,
    /// Primary account (or not)
    pub primary: bool,
    /// Account’s type.
    ///
    /// Valid values: `wallet`, `fiat`, `vault`
    pub r#type: String,
    /// Account’s currency
    pub currency: Currency,
    /// Account balance
    pub balance: Balance,
    /// Created at
    pub created_at: Option<String>,
    /// Updated at
    pub updated_at: Option<String>,
}

/// Account balance
#[derive(Debug, Deserialize)]
pub struct Balance {
    /// Amount
    pub amount: BigDecimal,
    /// Currency
    pub currency: String,
}

/// Currency
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
pub struct Currency {
    /// Asset ID
    pub asset_id: String,
    /// Currency code (i.e., BTC)
    pub code: String,
    /// Currency name (i.e., Bitcoin)
    pub name: String,
}
