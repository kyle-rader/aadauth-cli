use crate::auth_flow::AuthorizationCodeFlow;
use crate::error::{MsalError, MsalResult};
use crate::token::{AccessToken, IdToken, TokenResponse};

/// Configuration for a public client application
#[derive(Debug, Clone)]
pub struct PublicClientApplicationOptions {
    /// The client ID (application ID) assigned by Azure AD
    pub client_id: String,
    /// The tenant ID or tenant name (e.g., "common", "organizations", or a specific tenant ID)
    pub tenant_id: String,
    /// Optional redirect URI (defaults to localhost)
    pub redirect_uri: Option<String>,
}

/// Public client application for OAuth 2.0 authentication flows
/// 
/// This is the main entry point for authenticating users in public client applications
/// such as desktop applications, mobile applications, or single-page applications.
#[derive(Debug)]
pub struct PublicClientApplication {
    options: PublicClientApplicationOptions,
    auth_flow: AuthorizationCodeFlow,
}

impl PublicClientApplication {
    /// Create a new public client application
    pub fn new(options: PublicClientApplicationOptions) -> MsalResult<Self> {
        let auth_flow = AuthorizationCodeFlow::new(
            options.client_id.clone(),
            options.tenant_id.clone(),
        )?;

        Ok(Self { options, auth_flow })
    }

    /// Create a public client application with minimal configuration
    pub fn create(client_id: String, tenant_id: String) -> MsalResult<Self> {
        let options = PublicClientApplicationOptions {
            client_id,
            tenant_id,
            redirect_uri: None,
        };
        Self::new(options)
    }

    /// Acquire a token interactively using the system browser
    /// 
    /// This method will:
    /// 1. Generate a PKCE challenge
    /// 2. Open the system browser to the Microsoft login page
    /// 3. Start a local server to receive the callback
    /// 4. Exchange the authorization code for tokens
    /// 
    /// # Arguments
    /// * `scopes` - The scopes to request access for
    /// 
    /// # Returns
    /// A `TokenResponse` containing the access token and optionally an ID token and refresh token
    pub async fn acquire_token_interactive(&self, scopes: Vec<String>) -> MsalResult<TokenResponse> {
        if scopes.is_empty() {
            return Err(MsalError::ConfigurationError(
                "At least one scope must be specified".to_string(),
            ));
        }

        self.auth_flow.authenticate_interactive(scopes).await
    }

    /// Acquire an access token for the specified scopes
    /// 
    /// This is a convenience method that calls `acquire_token_interactive` and returns just the access token
    pub async fn acquire_access_token(&self, scopes: Vec<String>) -> MsalResult<AccessToken> {
        let token_response = self.acquire_token_interactive(scopes.clone()).await?;
        Ok(token_response.to_access_token(scopes))
    }

    /// Acquire tokens including an ID token
    /// 
    /// This method requests both access tokens and an ID token by including "openid" in the scopes
    pub async fn acquire_tokens_with_id_token(&self, mut scopes: Vec<String>) -> MsalResult<(AccessToken, Option<IdToken>)> {
        // Ensure openid scope is included for ID token
        if !scopes.iter().any(|s| s == "openid") {
            scopes.push("openid".to_string());
        }

        let token_response = self.acquire_token_interactive(scopes.clone()).await?;
        let access_token = token_response.to_access_token(scopes);
        let id_token = token_response.to_id_token();

        Ok((access_token, id_token))
    }

    /// Get the client ID
    pub fn client_id(&self) -> &str {
        &self.options.client_id
    }

    /// Get the tenant ID
    pub fn tenant_id(&self) -> &str {
        &self.options.tenant_id
    }
}

/// Builder for creating a PublicClientApplication
#[derive(Debug, Default)]
pub struct PublicClientApplicationBuilder {
    client_id: Option<String>,
    tenant_id: Option<String>,
    redirect_uri: Option<String>,
}

impl PublicClientApplicationBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the client ID
    pub fn with_client_id<S: Into<String>>(mut self, client_id: S) -> Self {
        self.client_id = Some(client_id.into());
        self
    }

    /// Set the tenant ID
    pub fn with_tenant_id<S: Into<String>>(mut self, tenant_id: S) -> Self {
        self.tenant_id = Some(tenant_id.into());
        self
    }

    /// Set a custom redirect URI
    pub fn with_redirect_uri<S: Into<String>>(mut self, redirect_uri: S) -> Self {
        self.redirect_uri = Some(redirect_uri.into());
        self
    }

    /// Build the PublicClientApplication
    pub fn build(self) -> MsalResult<PublicClientApplication> {
        let client_id = self
            .client_id
            .ok_or_else(|| MsalError::ConfigurationError("client_id is required".to_string()))?;
        let tenant_id = self
            .tenant_id
            .ok_or_else(|| MsalError::ConfigurationError("tenant_id is required".to_string()))?;

        let options = PublicClientApplicationOptions {
            client_id,
            tenant_id,
            redirect_uri: self.redirect_uri,
        };

        PublicClientApplication::new(options)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_pattern() {
        let result = PublicClientApplicationBuilder::new()
            .with_client_id("test-client-id")
            .with_tenant_id("common")
            .build();

        assert!(result.is_ok());
        let app = result.unwrap();
        assert_eq!(app.client_id(), "test-client-id");
        assert_eq!(app.tenant_id(), "common");
    }

    #[test]
    fn test_builder_missing_client_id() {
        let result = PublicClientApplicationBuilder::new()
            .with_tenant_id("common")
            .build();

        assert!(result.is_err());
        match result.unwrap_err() {
            MsalError::ConfigurationError(msg) => assert!(msg.contains("client_id")),
            _ => panic!("Expected ConfigurationError"),
        }
    }

    #[test]
    fn test_create_method() {
        let result = PublicClientApplication::create(
            "test-client-id".to_string(),
            "common".to_string(),
        );

        assert!(result.is_ok());
        let app = result.unwrap();
        assert_eq!(app.client_id(), "test-client-id");
        assert_eq!(app.tenant_id(), "common");
    }
}