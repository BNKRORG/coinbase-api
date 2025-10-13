//! Coinbase App APIs authentication
//!
//! <https://docs.cdp.coinbase.com/coinbase-app/authentication-authorization>

use std::fmt;

use oauth2::{ClientId, ClientSecret};
use url::Url;

pub(super) mod jwt;

use crate::app::oauth::CoinbaseOAuth2Token;

/// Coinbase authentication
#[derive(Clone, Default)]
pub enum CoinbaseAuth {
    /// No authentication
    #[default]
    None,
    /// API Keys
    ApiKeys {
        /// API Key
        api_key: String,
        /// Secret Key
        secret_key: String,
    },
    /// OAuth2
    OAuth {
        /// Client ID
        client_id: ClientId,
        /// Client secret
        client_secret: ClientSecret,
        /// Redirect URL
        redirect_url: Url,
        /// Token
        token: Option<CoinbaseOAuth2Token>,
    },
}

impl fmt::Debug for CoinbaseAuth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CoinbaseAuth").finish()
    }
}
