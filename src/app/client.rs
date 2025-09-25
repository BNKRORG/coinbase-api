//! Coinbase App client

use super::agent::SecureHttpClientAgent;
use super::auth::CoinbaseAuth;
use super::error::Error;

/// Coinbase App client
#[derive(Debug, Clone)]
pub struct CoinbaseAppClient {
    client: SecureHttpClientAgent,
}

impl CoinbaseAppClient {
    /// Construct a new Coinbase App client.
    pub fn new(auth: CoinbaseAuth, use_sandbox: bool) -> Result<Self, Error> {
        Ok(Self {
            client: SecureHttpClientAgent::new(auth, use_sandbox)?,
        })
    }
}
