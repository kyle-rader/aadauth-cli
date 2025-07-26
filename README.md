# MSAL Rust - Microsoft Authentication Library for Rust

This project provides a Rust implementation of the Microsoft Authentication Library (MSAL) based on the [Microsoft Authentication Library for .NET](https://github.com/AzureAD/microsoft-authentication-library-for-dotnet). It focuses on the OAuth 2.0 Authorization Code Flow with PKCE (Proof Key for Code Exchange) for public client applications.

## Features

- **OAuth 2.0 Authorization Code Flow with PKCE**: Secure authentication flow for public clients
- **Web Browser Authentication**: Interactive authentication using the system's default web browser
- **Microsoft Identity Platform Integration**: Works with Azure AD and Microsoft accounts
- **Token Management**: Automatic token parsing and validation
- **CLI Tool**: Ready-to-use command-line interface for authentication

## Project Structure

- `msal-rust/`: The core MSAL Rust library
- `src/main.rs`: The aadauth CLI application

## Installation

### Prerequisites

- Rust 1.70+ 
- OpenSSL development libraries (on Linux: `libssl-dev pkg-config`)

### Building

```bash
cargo build --release
```

## Usage

### CLI Tool (aadauth)

The `aadauth` CLI provides an easy way to authenticate with Microsoft Identity Platform:

```bash
# Authenticate and acquire tokens
cargo run -- auth --client <CLIENT_ID> --tenant <TENANT_ID> --scopes <SCOPE1> <SCOPE2>

# Example with Microsoft Graph
cargo run -- auth \
  --client "your-client-id" \
  --tenant "your-tenant-id" \
  --scopes "https://graph.microsoft.com/User.Read" "https://graph.microsoft.com/Mail.Read"

# Example with environment variables
export AZURE_CLIENT_ID="your-client-id"
export AZURE_TENANT_ID="your-tenant-id"
cargo run --example simple_auth
```

#### Parameters

- `--client`: Your Azure AD application's client ID
- `--tenant`: The tenant ID (or "common" for multi-tenant apps)
- `--scopes`: One or more OAuth 2.0 scopes to request

### Library Usage

```rust
use msal_rust::{PublicClientApplication, PublicClientApplicationOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = PublicClientApplicationOptions {
        client_id: "your-client-id".to_string(),
        tenant_id: "your-tenant-id".to_string(),
        redirect_uri: None, // Uses http://localhost:8080 by default
    };

    let app = PublicClientApplication::new(options)?;
    
    let scopes = vec![
        "https://graph.microsoft.com/User.Read".to_string(),
        "https://graph.microsoft.com/Mail.Read".to_string(),
    ];

    match app.acquire_token_interactive(scopes).await {
        Ok(token_response) => {
            println!("Access Token: {}", token_response.access_token.token);
            println!("Expires At: {}", token_response.access_token.expires_at);
            
            if let Some(id_token) = token_response.id_token {
                println!("User ID: {}", id_token.subject);
            }
        }
        Err(e) => eprintln!("Authentication failed: {}", e),
    }

    Ok(())
}
```

## Authentication Flow

The library implements the OAuth 2.0 Authorization Code Flow with PKCE:

1. **Generate PKCE Challenge**: Creates a code verifier and challenge for security
2. **Build Authorization URL**: Constructs the Microsoft Identity Platform authorization URL
3. **Launch Browser**: Opens the system's default browser to the authorization URL
4. **Start Local Server**: Listens on localhost:8080 for the authorization callback
5. **Handle Callback**: Receives the authorization code from the redirect
6. **Exchange Code for Tokens**: Uses the authorization code to request access and ID tokens
7. **Return Tokens**: Provides the tokens to the application

## Supported Scopes

The library works with any Microsoft Identity Platform scopes, including:

- Microsoft Graph API scopes (e.g., `https://graph.microsoft.com/User.Read`)
- Azure Resource Manager scopes (e.g., `https://management.azure.com/user_impersonation`)
- Custom API scopes for your own applications

## Security Features

- **PKCE (RFC 7636)**: Proof Key for Code Exchange prevents authorization code interception attacks
- **State Parameter**: CSRF protection using a random state value
- **Secure Random Generation**: Cryptographically secure random number generation for PKCE
- **Token Validation**: Basic token format and expiration validation

## Error Handling

The library provides comprehensive error handling through the `MsalError` enum:

- `HttpError`: Network-related errors
- `JsonError`: JSON parsing errors
- `OAuthError`: OAuth 2.0 protocol errors
- `AuthenticationError`: Authentication-specific errors
- `TokenValidationError`: Token validation errors

## Comparison with MSAL.NET

This Rust implementation follows the same OAuth 2.0 patterns as MSAL.NET but is tailored for Rust applications:

| Feature | MSAL.NET | MSAL Rust |
|---------|----------|-----------|
| Authorization Code Flow | ✅ | ✅ |
| PKCE Support | ✅ | ✅ |
| Interactive Auth | ✅ | ✅ |
| Token Cache | ✅ | ❌ (Future) |
| Device Code Flow | ✅ | ❌ (Future) |
| Client Credentials | ✅ | ❌ (Future) |

## Contributing

Contributions are welcome! Please ensure:

1. Code follows Rust best practices
2. All tests pass: `cargo test`
3. Code is properly formatted: `cargo fmt`
4. No clippy warnings: `cargo clippy`

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Related Projects

- [Microsoft Authentication Library for .NET](https://github.com/AzureAD/microsoft-authentication-library-for-dotnet)
- [Microsoft Authentication Library for JavaScript](https://github.com/AzureAD/microsoft-authentication-library-for-js)
- [Microsoft Authentication Library for Python](https://github.com/AzureAD/microsoft-authentication-library-for-python)