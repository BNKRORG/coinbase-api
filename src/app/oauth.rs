//! Coinbase App OAuth2 client

use std::fmt::Debug;

use oauth2::{AccessToken, RefreshToken};

// /// Coinbase App OAuth2 refreshed token
// #[derive(Debug, Clone)]
// pub struct CoinbaseOAuth2Token {
//     /// The new access token
//     pub access_token: AccessToken,
//     /// The refresh token
//     pub refresh_token: RefreshToken,
//     /// The expiration time
//     pub expires_at: Option<u64>,
// }

/// Coinbase App OAuth2 callback
#[async_trait::async_trait]
pub trait CoinbaseAppOAuth2Callback: Debug + Send + Sync {
    // /// The token has been refreshed
    // async fn token_refreshed(
    //     &self,
    //     token: CoinbaseOAuth2Token,
    // ) -> Result<(), Box<dyn std::error::Error>>;

    async fn get_access_token(&self) -> Result<AccessToken, Box<dyn std::error::Error>>;
}
