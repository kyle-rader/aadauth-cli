use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use rand::Rng;
use sha2::{Digest, Sha256};

/// PKCE (Proof Key for Code Exchange) implementation
#[derive(Debug, Clone)]
pub struct PkceChallenge {
    /// The code verifier
    pub code_verifier: String,
    /// The code challenge
    pub code_challenge: String,
    /// The code challenge method (always S256)
    pub code_challenge_method: String,
}

impl PkceChallenge {
    /// Generate a new PKCE challenge
    pub fn new() -> Self {
        let code_verifier = Self::generate_code_verifier();
        let code_challenge = Self::generate_code_challenge(&code_verifier);
        
        Self {
            code_verifier,
            code_challenge,
            code_challenge_method: "S256".to_string(),
        }
    }

    /// Generate a cryptographically random code verifier
    /// 
    /// According to RFC 7636, the code verifier should be a cryptographically
    /// random string using the characters [A-Z] / [a-z] / [0-9] / "-" / "." / "_" / "~",
    /// with a minimum length of 43 characters and a maximum length of 128 characters.
    fn generate_code_verifier() -> String {
        // Generate 32 random bytes and base64url encode them to get a 43-character string
        let random_bytes: Vec<u8> = (0..32).map(|_| rand::thread_rng().gen()).collect();
        URL_SAFE_NO_PAD.encode(&random_bytes)
    }

    /// Generate code challenge from code verifier using SHA256
    /// 
    /// code_challenge = BASE64URL-ENCODE(SHA256(ASCII(code_verifier)))
    fn generate_code_challenge(code_verifier: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(code_verifier.as_bytes());
        let hash = hasher.finalize();
        URL_SAFE_NO_PAD.encode(&hash)
    }
}

impl Default for PkceChallenge {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pkce_challenge_generation() {
        let pkce = PkceChallenge::new();
        
        // Code verifier should be at least 43 characters
        assert!(pkce.code_verifier.len() >= 43);
        
        // Code challenge should be 43 characters (base64url encoded SHA256 hash)
        assert_eq!(pkce.code_challenge.len(), 43);
        
        // Method should be S256
        assert_eq!(pkce.code_challenge_method, "S256");
    }

    #[test]
    fn test_code_challenge_deterministic() {
        let code_verifier = "dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk";
        let expected_challenge = "E9Melhoa2OwvFrEMTJguCHaoeK1t8URWbuGJSstw-cM";
        
        let actual_challenge = PkceChallenge::generate_code_challenge(code_verifier);
        assert_eq!(actual_challenge, expected_challenge);
    }

    #[test]
    fn test_multiple_challenges_are_different() {
        let pkce1 = PkceChallenge::new();
        let pkce2 = PkceChallenge::new();
        
        // Each PKCE challenge should be unique
        assert_ne!(pkce1.code_verifier, pkce2.code_verifier);
        assert_ne!(pkce1.code_challenge, pkce2.code_challenge);
    }
}