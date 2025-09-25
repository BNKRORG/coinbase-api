//! Coinbase App error

use thiserror::Error;

/// Coinbase App error
#[derive(Debug, Error)]
pub enum Error {
    /// JSON error
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    /// Invalid private key
    #[error("invalid private key: {0}")]
    InvalidPrivateKey(String),
    /// Bad signature
    #[error("bad signature: {0}")]
    BadSignature(String),
}
