use clap::Parser;
use msal_rust::{PublicClientApplication, MsalError};
use std::process;

#[derive(Debug, Parser)]
struct Target {
    /// Client ID
    #[clap(long)]
    client: String,
    /// Tenant ID
    #[clap(long)]
    tenant: String,
    /// Scopes to request
    #[clap(long, required = true)]
    scopes: Vec<String>,
}

/// Do Auth Well - Microsoft Authentication Library for Rust
#[derive(Debug, Parser)]
#[clap(version, about = "Microsoft Authentication Library (MSAL) for Rust CLI")]
enum Args {
    /// Authenticate and acquire tokens
    Auth(Target),
    /// Clear token cache (not implemented yet)
    Clear(Target),
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    
    match args {
        Args::Auth(target) => {
            if let Err(e) = authenticate(target).await {
                eprintln!("Authentication failed: {}", e);
                process::exit(1);
            }
        }
        Args::Clear(target) => {
            println!("Clear token cache functionality not implemented yet for client: {}", target.client);
            println!("In a full implementation, this would clear cached tokens for the specified client and tenant.");
        }
    }
}

async fn authenticate(target: Target) -> Result<(), MsalError> {
    println!("Starting authentication for client: {}", target.client);
    println!("Tenant: {}", target.tenant);
    println!("Scopes: {:?}", target.scopes);
    
    // Create the public client application
    let app = PublicClientApplication::create(target.client.clone(), target.tenant.clone())?;
    
    println!("Acquiring access token interactively...");
    
    // Acquire tokens interactively
    let token_response = app.acquire_token_interactive(target.scopes.clone()).await?;
    
    // Convert to access token for easier handling
    let access_token = token_response.to_access_token(target.scopes);
    
    println!("✅ Authentication successful!");
    println!("Access Token: {}", access_token.token);
    println!("Expires at: {}", access_token.expires_at);
    println!("Scopes: {:?}", access_token.scopes);
    
    // If we have an ID token, display some info
    if let Some(id_token) = token_response.to_id_token() {
        println!("ID Token received");
        println!("Subject: {}", id_token.subject);
    }
    
    // In a real application, you would typically:
    // 1. Store the tokens securely (encrypted)
    // 2. Use the access token to make API calls
    // 3. Refresh tokens when they expire
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_target_parsing() {
        // This test verifies that the CLI argument parsing works correctly
        let target = Target {
            client: "test-client-id".to_string(),
            tenant: "common".to_string(),
            scopes: vec!["https://graph.microsoft.com/User.Read".to_string()],
        };
        
        assert_eq!(target.client, "test-client-id");
        assert_eq!(target.tenant, "common");
        assert_eq!(target.scopes.len(), 1);
    }
}
