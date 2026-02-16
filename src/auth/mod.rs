pub mod ed25519;
pub mod hmac;
pub mod rsa;

use secrecy::{ExposeSecret, SecretString};

use crate::error::OkxError;

/// Supported signing algorithms.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SigningAlgorithm {
    HmacSha256,
    RsaPkcs1v15,
    Ed25519,
}

/// Auto-detect the signing algorithm from the secret key format.
///
/// - If the secret contains "PRIVATE KEY", it's a PEM key:
///   - If it contains "RSA PRIVATE KEY", use RSA.
///   - If the key is short (<= 150 chars), use Ed25519.
///   - Otherwise, use RSA.
/// - Otherwise, use HMAC-SHA256 (the default for OKX).
pub fn detect_signing_algorithm(secret: &str) -> SigningAlgorithm {
    if secret.contains("PRIVATE KEY") {
        if secret.contains("RSA PRIVATE KEY") {
            return SigningAlgorithm::RsaPkcs1v15;
        }
        if secret.len() <= 150 {
            return SigningAlgorithm::Ed25519;
        }
        return SigningAlgorithm::RsaPkcs1v15;
    }
    SigningAlgorithm::HmacSha256
}

/// Sign a message using the auto-detected algorithm.
pub fn sign_message(message: &str, secret: &SecretString) -> Result<String, OkxError> {
    let algo = detect_signing_algorithm(secret.expose_secret());
    match algo {
        SigningAlgorithm::HmacSha256 => hmac::sign_hmac_sha256(message, secret),
        SigningAlgorithm::RsaPkcs1v15 => rsa::sign_rsa(message, secret),
        SigningAlgorithm::Ed25519 => ed25519::sign_ed25519(message, secret),
    }
}

/// Build and sign the REST API prehash string.
///
/// Format: `{timestamp}{METHOD}{endpoint}{body}`
/// - GET body: query string like `?key=val&key2=val2` (or empty)
/// - POST body: JSON string
pub fn sign_rest(
    timestamp: &str,
    method: &str,
    endpoint: &str,
    body: &str,
    secret: &SecretString,
) -> Result<String, OkxError> {
    let message = format!("{timestamp}{method}{endpoint}{body}");
    sign_message(&message, secret)
}

/// Build and sign the WebSocket authentication prehash string.
///
/// Format: `{unix_seconds}GET/users/self/verify`
pub fn sign_ws(timestamp: &str, secret: &SecretString) -> Result<String, OkxError> {
    let message = format!("{timestamp}GET/users/self/verify");
    sign_message(&message, secret)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_hmac() {
        assert_eq!(
            detect_signing_algorithm("some-api-secret-key"),
            SigningAlgorithm::HmacSha256
        );
    }

    #[test]
    fn test_detect_rsa() {
        let key = "-----BEGIN RSA PRIVATE KEY-----\nMIIEpAIBAAKCAQEA...\n-----END RSA PRIVATE KEY-----";
        assert_eq!(
            detect_signing_algorithm(key),
            SigningAlgorithm::RsaPkcs1v15
        );
    }

    #[test]
    fn test_detect_ed25519() {
        // Short PEM key (<= 150 chars)
        let key = "-----BEGIN PRIVATE KEY-----\nMC4CAQAwBQYDK2VwBCIEIDk=\n-----END PRIVATE KEY-----";
        assert_eq!(detect_signing_algorithm(key), SigningAlgorithm::Ed25519);
    }

    #[test]
    fn test_sign_rest() {
        let secret = SecretString::from("test-secret".to_string());
        let result =
            sign_rest("2024-01-15T12:30:45.123Z", "GET", "/api/v5/account/balance", "", &secret);
        assert!(result.is_ok());
    }

    #[test]
    fn test_sign_ws() {
        let secret = SecretString::from("test-secret".to_string());
        let result = sign_ws("1705312245", &secret);
        assert!(result.is_ok());
    }
}
