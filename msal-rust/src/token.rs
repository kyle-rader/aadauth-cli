use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Access token used to authenticate API requests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessToken {
    /// The access token string
    pub token: String,
    /// When the token expires
    pub expires_at: DateTime<Utc>,
    /// The scopes this token is valid for
    pub scopes: Vec<String>,
}

impl AccessToken {
    /// Check if the token is expired
    pub fn is_expired(&self) -> bool {
        Utc::now() >= self.expires_at
    }

    /// Get the Authorization header value
    pub fn authorization_header(&self) -> String {
        format!("Bearer {}", self.token)
    }
}

/// ID token containing user identity information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdToken {
    /// The raw ID token string
    pub token: String,
    /// When the token expires
    pub expires_at: DateTime<Utc>,
    /// The subject (user ID)
    pub subject: String,
    /// The user's email (if available)
    pub email: Option<String>,
    /// The user's name (if available)
    pub name: Option<String>,
}

/// Response from the token endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenResponse {
    /// Access token
    pub access_token: String,
    /// Token type (usually "Bearer")
    pub token_type: String,
    /// Token lifetime in seconds
    pub expires_in: u64,
    /// Scopes granted
    pub scope: Option<String>,
    /// Refresh token (if requested)
    pub refresh_token: Option<String>,
    /// ID token (if requested)
    pub id_token: Option<String>,
}

impl TokenResponse {
    /// Convert to AccessToken
    pub fn to_access_token(&self, scopes: Vec<String>) -> AccessToken {
        let expires_at = Utc::now() + chrono::Duration::seconds(self.expires_in as i64);
        AccessToken {
            token: self.access_token.clone(),
            expires_at,
            scopes,
        }
    }

    /// Convert to IdToken (if present)
    pub fn to_id_token(&self) -> Option<IdToken> {
        self.id_token.as_ref().map(|token| {
            // For simplicity, we'll set a default expiration
            // In a real implementation, you'd parse the JWT to get the actual expiration
            let expires_at = Utc::now() + chrono::Duration::hours(1);
            IdToken {
                token: token.clone(),
                expires_at,
                subject: "user".to_string(), // Would be parsed from JWT
                email: None,
                name: None,
            }
        })
    }
}

/// Error response from the OAuth server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    /// Error code
    pub error: String,
    /// Error description
    pub error_description: Option<String>,
    /// Error URI for more information
    pub error_uri: Option<String>,
}