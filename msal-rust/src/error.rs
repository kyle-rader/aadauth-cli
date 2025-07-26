use thiserror::Error;

/// Result type alias for MSAL operations
pub type MsalResult<T> = Result<T, MsalError>;

/// Errors that can occur during MSAL operations
#[derive(Error, Debug)]
pub enum MsalError {
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("URL parsing error: {0}")]
    UrlError(#[from] url::ParseError),

    #[error("OAuth error: {error} - {description}")]
    OAuthError {
        error: String,
        description: String,
    },

    #[error("Invalid configuration: {0}")]
    ConfigurationError(String),

    #[error("Authentication failed: {0}")]
    AuthenticationError(String),

    #[error("Token validation failed: {0}")]
    TokenValidationError(String),

    #[error("Browser launch failed: {0}")]
    BrowserError(String),

    #[error("Timeout waiting for authentication")]
    TimeoutError,

    #[error("User cancelled authentication")]
    UserCancelled,

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}