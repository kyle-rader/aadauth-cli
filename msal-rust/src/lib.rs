//! Microsoft Authentication Library (MSAL) for Rust
//! 
//! This crate provides OAuth 2.0 authentication flows for Microsoft Identity Platform,
//! specifically focusing on the Authorization Code Flow with PKCE for public clients.

pub mod client;
pub mod error;
pub mod token;
pub mod pkce;
pub mod auth_flow;

pub use client::{PublicClientApplication, PublicClientApplicationOptions};
pub use error::{MsalError, MsalResult};
pub use token::{AccessToken, IdToken, TokenResponse};
pub use auth_flow::AuthorizationCodeFlow;