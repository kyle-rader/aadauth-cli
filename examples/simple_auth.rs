use msal_rust::{PublicClientApplication, PublicClientApplicationOptions};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get configuration from environment variables
    let client_id = env::var("AZURE_CLIENT_ID")
        .unwrap_or_else(|_| "your-client-id-here".to_string());
    let tenant_id = env::var("AZURE_TENANT_ID")
        .unwrap_or_else(|_| "common".to_string());
    
    println!("MSAL Rust Example");
    println!("=================");
    println!("Client ID: {}", client_id);
    println!("Tenant ID: {}", tenant_id);
    println!();

    // Configure the public client application
    let options = PublicClientApplicationOptions {
        client_id: client_id.clone(),
        tenant_id: tenant_id.clone(),
        redirect_uri: None, // Uses http://localhost:8080 by default
    };

    let app = PublicClientApplication::new(options)?;
    
    // Define the scopes we want to request
    let scopes = vec![
        "https://graph.microsoft.com/User.Read".to_string(),
    ];

    println!("Starting interactive authentication...");
    println!("Your browser will open shortly. Please complete the sign-in process.");
    println!();

    match app.acquire_token_interactive(scopes.clone()).await {
        Ok(token_response) => {
            println!("✅ Authentication successful!");
            println!();
            
            // Convert to access token
            let access_token = token_response.to_access_token(scopes);
            
            println!("Access Token (first 50 chars): {}...", 
                     &access_token.token[..50.min(access_token.token.len())]);
            println!("Token Type: {}", token_response.token_type);
            println!("Expires At: {}", access_token.expires_at);
            println!("Scopes: {:?}", access_token.scopes);
            
            if let Some(id_token) = token_response.to_id_token() {
                println!();
                println!("ID Token Information:");
                println!("  Subject: {}", id_token.subject);
                if let Some(name) = id_token.name {
                    println!("  Name: {}", name);
                }
                if let Some(email) = id_token.email {
                    println!("  Email: {}", email);
                }
            }
            
            // You can now use the access token to make API calls
            println!();
            println!("You can now use this token to make authenticated API calls!");
            println!("Example: Authorization header = 'Bearer {}'...", 
                     &access_token.token[..20.min(access_token.token.len())]);
        }
        Err(e) => {
            eprintln!("❌ Authentication failed: {}", e);
            eprintln!();
            eprintln!("Common issues:");
            eprintln!("  - Make sure your client ID is correct");
            eprintln!("  - Ensure your app registration allows public client flows");
            eprintln!("  - Check that the redirect URI is configured (http://localhost:8080)");
            std::process::exit(1);
        }
    }

    Ok(())
}