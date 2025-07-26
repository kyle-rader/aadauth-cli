use crate::error::{MsalError, MsalResult};
use crate::pkce::PkceChallenge;
use crate::token::{ErrorResponse, TokenResponse};
use reqwest::Client;
use std::collections::HashMap;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::time::timeout;
use url::Url;
use uuid::Uuid;

/// Microsoft Identity Platform endpoints
pub const AUTHORITY_URL: &str = "https://login.microsoftonline.com";
const AUTHORIZE_ENDPOINT: &str = "/oauth2/v2.0/authorize";
const TOKEN_ENDPOINT: &str = "/oauth2/v2.0/token";

/// Authorization Code Flow with PKCE implementation
#[derive(Debug)]
pub struct AuthorizationCodeFlow {
    client_id: String,
    redirect_uri: String,
    authority: String,
    http_client: Client,
}

impl AuthorizationCodeFlow {
    /// Create a new authorization code flow
    pub fn new(client_id: String, tenant_id: String) -> MsalResult<Self> {
        // Use localhost with a random port for redirect URI
        let redirect_uri = "http://localhost:8080".to_string();
        let authority = format!("{}/{}", AUTHORITY_URL, tenant_id);
        
        let http_client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()?;

        Ok(Self {
            client_id,
            redirect_uri,
            authority,
            http_client,
        })
    }

    /// Start the interactive authentication flow
    pub async fn authenticate_interactive(&self, scopes: Vec<String>) -> MsalResult<TokenResponse> {
        // Generate PKCE challenge
        let pkce = PkceChallenge::new();
        
        // Generate state parameter for CSRF protection
        let state = Uuid::new_v4().to_string();
        
        // Build authorization URL
        let auth_url = self.build_authorization_url(&scopes, &pkce, &state)?;
        
        // Start local server to receive callback
        let listener = TcpListener::bind("127.0.0.1:8080").await?;
        
        // Open browser
        println!("Opening browser for authentication...");
        println!("If the browser doesn't open automatically, please visit: {}", auth_url);
        
        if let Err(e) = webbrowser::open(&auth_url) {
            return Err(MsalError::BrowserError(format!("Failed to open browser: {}", e)));
        }
        
        // Wait for callback with timeout
        let callback_result = timeout(Duration::from_secs(300), self.handle_callback(listener, &state)).await;
        
        match callback_result {
            Ok(Ok(auth_code)) => {
                // Exchange authorization code for tokens
                self.exchange_code_for_tokens(auth_code, &pkce, scopes).await
            }
            Ok(Err(e)) => Err(e),
            Err(_) => Err(MsalError::TimeoutError),
        }
    }

    /// Build the authorization URL
    fn build_authorization_url(&self, scopes: &[String], pkce: &PkceChallenge, state: &str) -> MsalResult<String> {
        let mut url = Url::parse(&format!("{}{}", self.authority, AUTHORIZE_ENDPOINT))?;
        
        {
            let mut query_pairs = url.query_pairs_mut();
            query_pairs.append_pair("client_id", &self.client_id);
            query_pairs.append_pair("response_type", "code");
            query_pairs.append_pair("redirect_uri", &self.redirect_uri);
            query_pairs.append_pair("scope", &scopes.join(" "));
            query_pairs.append_pair("state", state);
            query_pairs.append_pair("code_challenge", &pkce.code_challenge);
            query_pairs.append_pair("code_challenge_method", &pkce.code_challenge_method);
            query_pairs.append_pair("response_mode", "query");
        }
        
        Ok(url.to_string())
    }

    /// Handle the callback from the browser
    async fn handle_callback(&self, listener: TcpListener, expected_state: &str) -> MsalResult<String> {
        loop {
            let (mut stream, _) = listener.accept().await?;
            
            // Read the HTTP request
            let mut buffer = [0; 1024];
            use tokio::io::AsyncReadExt;
            let bytes_read = stream.read(&mut buffer).await?;
            let request = String::from_utf8_lossy(&buffer[..bytes_read]);
            
            // Parse the request line to get the callback URL
            if let Some(first_line) = request.lines().next() {
                if let Some(path_and_query) = first_line.split_whitespace().nth(1) {
                    let callback_url = format!("http://localhost:8080{}", path_and_query);
                    let parsed_url = Url::parse(&callback_url)?;
                    
                    // Send response to browser
                    let response = if parsed_url.query_pairs().any(|(key, _)| key == "code") {
                        "HTTP/1.1 200 OK\r\n\r\n<html><body><h1>Authentication successful!</h1><p>You can close this window.</p></body></html>"
                    } else {
                        "HTTP/1.1 400 Bad Request\r\n\r\n<html><body><h1>Authentication failed!</h1><p>Please try again.</p></body></html>"
                    };
                    
                    use tokio::io::AsyncWriteExt;
                    stream.write_all(response.as_bytes()).await?;
                    stream.flush().await?;
                    
                    // Parse query parameters
                    let query_params: HashMap<String, String> = parsed_url
                        .query_pairs()
                        .map(|(key, value)| (key.to_string(), value.to_string()))
                        .collect();
                    
                    // Check for errors
                    if let Some(error) = query_params.get("error") {
                        let description = query_params
                            .get("error_description")
                            .cloned()
                            .unwrap_or_else(|| "Unknown error".to_string());
                        return Err(MsalError::OAuthError {
                            error: error.clone(),
                            description,
                        });
                    }
                    
                    // Verify state parameter
                    if let Some(state) = query_params.get("state") {
                        if state != expected_state {
                            return Err(MsalError::AuthenticationError(
                                "State parameter mismatch".to_string(),
                            ));
                        }
                    } else {
                        return Err(MsalError::AuthenticationError(
                            "Missing state parameter".to_string(),
                        ));
                    }
                    
                    // Extract authorization code
                    if let Some(code) = query_params.get("code") {
                        return Ok(code.clone());
                    } else {
                        return Err(MsalError::AuthenticationError(
                            "Missing authorization code".to_string(),
                        ));
                    }
                }
            }
        }
    }

    /// Exchange authorization code for tokens
    async fn exchange_code_for_tokens(
        &self,
        auth_code: String,
        pkce: &PkceChallenge,
        scopes: Vec<String>,
    ) -> MsalResult<TokenResponse> {
        let token_url = format!("{}{}", self.authority, TOKEN_ENDPOINT);
        
        let scope_string = scopes.join(" ");
        let mut params = HashMap::new();
        params.insert("client_id", self.client_id.as_str());
        params.insert("code", auth_code.as_str());
        params.insert("redirect_uri", self.redirect_uri.as_str());
        params.insert("grant_type", "authorization_code");
        params.insert("code_verifier", pkce.code_verifier.as_str());
        params.insert("scope", scope_string.as_str());
        
        let response = self
            .http_client
            .post(&token_url)
            .form(&params)
            .send()
            .await?;
        
        let status = response.status();
        let response_text = response.text().await?;
        
        if status.is_success() {
            let token_response: TokenResponse = serde_json::from_str(&response_text)?;
            Ok(token_response)
        } else {
            // Try to parse as error response
            if let Ok(error_response) = serde_json::from_str::<ErrorResponse>(&response_text) {
                Err(MsalError::OAuthError {
                    error: error_response.error,
                    description: error_response.error_description.unwrap_or_else(|| "Unknown error".to_string()),
                })
            } else {
                Err(MsalError::AuthenticationError(format!(
                    "Token exchange failed with status {}: {}",
                    status, response_text
                )))
            }
        }
    }
}