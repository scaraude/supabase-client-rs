//! Error types for the Supabase client.

use thiserror::Error;

/// The main error type for Supabase operations.
#[derive(Debug, Error)]
pub enum Error {
    /// Configuration error (missing URL, invalid key, etc.)
    #[error("configuration error: {0}")]
    Config(String),

    /// URL parsing error
    #[error("invalid URL: {0}")]
    Url(#[from] url::ParseError),

    /// HTTP request error
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// JSON serialization/deserialization error
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// PostgREST-specific error
    #[error("PostgREST error: {message}")]
    PostgREST {
        message: String,
        code: Option<String>,
        details: Option<String>,
        hint: Option<String>,
    },

    /// Authentication error
    #[error("authentication error: {0}")]
    Auth(String),

    /// Storage error
    #[error("storage error: {0}")]
    Storage(String),

    /// Realtime connection error
    #[error("realtime error: {0}")]
    Realtime(String),

    /// Edge function invocation error
    #[error("function error: {0}")]
    Function(String),

    /// Feature not available (crate not enabled)
    #[error("{0} is not available - enable the '{1}' feature")]
    FeatureNotEnabled(&'static str, &'static str),
}

/// A specialized Result type for Supabase operations.
pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    /// Create a configuration error.
    pub fn config(msg: impl Into<String>) -> Self {
        Self::Config(msg.into())
    }

    /// Create a PostgREST error from response details.
    pub fn postgrest(
        message: impl Into<String>,
        code: Option<String>,
        details: Option<String>,
        hint: Option<String>,
    ) -> Self {
        Self::PostgREST {
            message: message.into(),
            code,
            details,
            hint,
        }
    }
}

/// Convert RealtimeError to our Error type when the realtime feature is enabled
#[cfg(feature = "realtime")]
impl From<supabase_realtime_rs::RealtimeError> for Error {
    fn from(err: supabase_realtime_rs::RealtimeError) -> Self {
        Self::Realtime(err.to_string())
    }
}
