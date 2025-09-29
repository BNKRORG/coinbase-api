//! Coinbase App APIs responses
//!
//! <https://docs.cdp.coinbase.com/coinbase-app/introduction/welcome>

use std::fmt;

use chrono::{DateTime, Utc};
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
    pub created_at: Option<DateTime<Utc>>,
    /// Updated at
    pub updated_at: Option<DateTime<Utc>>,
}

/// Account balance
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct Balance {
    /// Amount
    #[serde(deserialize_with = "deserialize_string_to_f64")]
    pub amount: f64,
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

/// Transaction type
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
pub enum TransactionType {
    /// Fills for an advanced trade order
    #[serde(rename = "advanced_trade_fill")]
    AdvancedTradeFill,
    /// Buy a digital asset
    #[serde(rename = "buy")]
    Buy,
    /// Recover money already disbursed
    #[serde(rename = "clawback")]
    Clawback,
    /// Daily cash transfers between futures and spot accounts for the US-regulated futures product
    #[serde(rename = "derivatives_settlement")]
    DerivativesSettlement,
    /// Payout for user earn on Coinbase
    #[serde(rename = "earn_payout")]
    EarnPayout,
    /// Deposit funds into a fiat account from a financial institution
    #[serde(rename = "fiat_deposit")]
    FiatDeposit,
    /// Withdraw funds from a fiat account
    #[serde(rename = "fiat_withdrawal")]
    FiatWithdrawal,
    /// Redemptions for Incentive & Referral campaigns
    #[serde(rename = "incentives_rewards_payout")]
    IncentivesRewardsPayout,
    /// Clawback incentive payout from customer account
    #[serde(rename = "incentives_shared_clawback")]
    IncentivesSharedClawback,
    /// Deposit crypto to customer international account
    #[serde(rename = "intx_deposit")]
    IntxDeposit,
    /// Withdraw crypto from customer international account
    #[serde(rename = "intx_withdrawal")]
    IntxWithdrawal,
    /// Receive a digital asset
    #[serde(rename = "receive")]
    Receive,
    /// Request a digital asset from a user or email
    #[serde(rename = "request")]
    Request,
    /// Sweep of dust balance from the account
    #[serde(rename = "retail_simple_dust")]
    RetailSimpleDust,
    /// Sell a digital asset
    #[serde(rename = "sell")]
    Sell,
    /// Send a supported digital asset to a corresponding address or email.
    #[serde(rename = "send")]
    Send,
    /// Funds from primary account moved to staked account
    #[serde(rename = "staking_transfer")]
    StakingTransfer,
    /// Transaction for Coinbase subscription rebate
    #[serde(rename = "subscription_rebate")]
    SubscriptionRebate,
    /// Transaction for Coinbase subscription
    #[serde(rename = "subscription")]
    Subscription,
    /// Exchange one cryptocurrency for another cryptocurrency or fiat currency
    #[serde(rename = "trade")]
    Trade,
    /// Transfer funds between two of your own accounts
    #[serde(rename = "transfer")]
    Transfer,
    /// Default transaction type, uncategorized.
    #[default]
    #[serde(rename = "tx")]
    Tx,
    /// Funds from staked funds moved to primary account
    #[serde(rename = "unstaking_transfer")]
    UnstakingTransfer,
    /// Recover unsupported ERC-20s deposited to Coinbase on ethereum mainnet
    #[serde(rename = "unsupported_asset_recovery")]
    UnsupportedAssetRecovery,
    /// Unwrap wrapped assets, e.g. cbETH, to wrappable assets, e.g. staked ETH
    #[serde(rename = "unwrap_asset")]
    UnwrapAsset,
    /// Withdraw funds from a vault account
    #[serde(rename = "vault_withdrawal")]
    VaultWithdrawal,
    /// Wrap wrappable assets, e.g. staked ETH, to wrapped assets, e.g. cbETH
    #[serde(rename = "wrap_asset")]
    WrapAsset,
    /// Conversion of USDC to USD to support the anticipated margin requirement for a futures trade
    #[serde(rename = "fcm_futures_usdc_sell")]
    FcmFuturesUsdcSell,
    /// Conversion of USDC to USD to support additional margin requirements or cover losses for open futures positions
    #[serde(rename = "fcm_futures_usdc_sell_additional_encumberment_rollup")]
    FcmFuturesUsdcSellAdditionalEncumbermentRollup,
}

/// Transaction status
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
pub enum TransactionStatus {
    /// Transaction was canceled
    #[serde(rename = "canceled")]
    Canceled,
    /// Completed transactions (e.g., a send or a buy)
    #[serde(rename = "completed")]
    Completed,
    /// Conditional transaction expired due to external factors
    #[serde(rename = "expired")]
    Expired,
    /// Failed transactions (e.g., failed buy)
    #[serde(rename = "failed")]
    Failed,
    /// Pending transactions (e.g., a send or a buy)
    #[serde(rename = "pending")]
    Pending,
    /// Vault withdrawal is waiting to be cleared
    #[serde(rename = "waiting_for_clearing")]
    WaitingForClearing,
    /// Vault withdrawal is waiting for approval
    #[serde(rename = "waiting_for_signature")]
    WaitingForSignature,
}

/// Transaction
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct Transaction {
    /// Transaction ID
    pub id: String,
    /// Transaction type
    pub r#type: TransactionType,
    /// Transaction status
    pub status: TransactionStatus,
    ///Amount of any supported digital asset.
    ///
    /// Value is negative to indicate the debiting of funds.
    pub amount: Balance,
    /// Amount in user’s native currency.
    ///
    /// Value is negative to indicate the debiting of funds.
    pub native_amount: Balance,
    /// User defined description
    pub description: Option<String>,
    /// Created at
    pub created_at: Option<DateTime<Utc>>,
}

fn deserialize_string_to_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = String::deserialize(deserializer)?;
    s.parse().map_err(serde::de::Error::custom)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_account() {
        let json = r##"
        {
          "data": {
            "id": "2bbf394c-193b-5b2a-9155-3b4732659ede",
            "name": "My Wallet",
            "primary": true,
            "type": "wallet",
            "currency": {
              "address_regex": "^([13][a-km-zA-HJ-NP-Z1-9]{25,34})|^(bc1[qzry9x8gf2tvdw0s3jn54khce6mua7l]([qpzry9x8gf2tvdw0s3jn54khce6mua7l]{38}|[qpzry9x8gf2tvdw0s3jn54khce6mua7l]{58}))$",
              "asset_id": "5b71fc48-3dd3-540c-809b-f8c94d0e68b5",
              "code": "BTC",
              "color": "#F7931A",
              "exponent": 8,
              "name": "Bitcoin",
              "slug": "bitcoin",
              "sort_index": 100,
              "type": "crypto"
            },
            "balance": {
              "amount": "39.59000000",
              "currency": "BTC"
            },
            "created_at": "2024-01-31T20:49:02Z",
            "updated_at": "2024-01-31T20:49:02Z",
            "resource": "account",
            "resource_path": "/v2/accounts/2bbf394c-193b-5b2a-9155-3b4732659ede"
          }
        }"##;

        let response: CoinbaseResponse<Account> = serde_json::from_str(json).unwrap();
        let account = response.data;

        // Verify account fields
        assert_eq!(account.id, "2bbf394c-193b-5b2a-9155-3b4732659ede");
        assert_eq!(account.name, "My Wallet");
        assert_eq!(account.primary, true);
        assert_eq!(account.r#type, "wallet");

        // Verify currency fields
        assert_eq!(
            account.currency.asset_id,
            "5b71fc48-3dd3-540c-809b-f8c94d0e68b5"
        );
        assert_eq!(account.currency.code, "BTC");
        assert_eq!(account.currency.name, "Bitcoin");

        // Verify balance fields - this is the key test for string-to-f64 deserialization
        assert_eq!(account.balance.amount, 39.59);
        assert_eq!(account.balance.currency, "BTC");

        // Verify optional fields
        assert_eq!(account.created_at.map(|t| t.timestamp()), Some(1706734142));
        assert_eq!(account.updated_at.map(|t| t.timestamp()), Some(1706734142));
    }
}
