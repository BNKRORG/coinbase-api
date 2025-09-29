//! Coinbase App client builder

use std::sync::Arc;
use std::time::Duration;

use super::auth::CoinbaseAuth;
use super::client::CoinbaseAppClient;
use super::error::Error;
use super::oauth::CoinbaseAppOAuth2Callback;

/// Coinbase App client builder
#[derive(Debug, Clone)]
pub struct CoinbaseAppClientBuilder {
    /// Authentication
    pub auth: CoinbaseAuth,
    /// OAuth2 callback
    ///
    /// This is needed for receiving the refreshed token
    pub oauth_callback: Option<Arc<dyn CoinbaseAppOAuth2Callback>>,
    /// Use sandbox APIs
    pub sandbox: bool,
    /// Requests timeout
    pub timeout: Duration,
}

impl Default for CoinbaseAppClientBuilder {
    fn default() -> Self {
        Self {
            auth: CoinbaseAuth::default(),
            oauth_callback: None,
            sandbox: false,
            timeout: Duration::from_secs(20),
        }
    }
}

impl CoinbaseAppClientBuilder {
    /// Set authentication
    #[inline]
    pub fn auth(mut self, auth: CoinbaseAuth) -> Self {
        self.auth = auth;
        self
    }

    /// Set OAuth2 callback
    #[inline]
    pub fn oauth2_callback<T>(mut self, callback: T) -> Self
    where
        T: CoinbaseAppOAuth2Callback + 'static,
    {
        self.oauth_callback = Some(Arc::new(callback));
        self
    }

    /// Set sandbox APIs
    #[inline]
    pub fn sandbox(mut self, sandbox: bool) -> Self {
        self.sandbox = sandbox;
        self
    }

    /// Set timeout (default: 20 secs)
    #[inline]
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Build client
    #[inline]
    pub fn build(self) -> Result<CoinbaseAppClient, Error> {
        CoinbaseAppClient::from_builder(self)
    }
}
