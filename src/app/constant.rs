/// Root resource for the API
pub(super) const API_ROOT_URL: &str = "https://api.coinbase.com";
pub(super) const API_SANDBOX_URL: &str = "https://api-sandbox.coinbase.com";

pub(super) const OAUTH_AUTHORIZE_URL: &str = "https://login.coinbase.com/oauth2/auth";
pub(super) const OAUTH_ACCESS_TOKEN_URL: &str = "https://login.coinbase.com/oauth2/token";

/// User Agent for the client
pub(super) const USER_AGENT_NAME: &str = concat!("coinbase-api/", env!("CARGO_PKG_VERSION"));

/// Coinbase App Versioning
///
/// <https://docs.cdp.coinbase.com/coinbase-app/api-architecture/versioning>
pub(super) const CB_VERSION: &str = "2022-01-06";
