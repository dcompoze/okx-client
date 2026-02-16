use base64::Engine;
use ring::hmac;
use secrecy::{ExposeSecret, SecretString};

use crate::error::OkxError;

/// Sign a message with HMAC-SHA256 and return the base64-encoded signature.
pub fn sign_hmac_sha256(message: &str, secret: &SecretString) -> Result<String, OkxError> {
    let key = hmac::Key::new(hmac::HMAC_SHA256, secret.expose_secret().as_bytes());
    let signature = hmac::sign(&key, message.as_bytes());
    Ok(base64::engine::general_purpose::STANDARD.encode(signature.as_ref()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hmac_sha256_signing() {
        let secret = SecretString::from("test-secret".to_string());
        let message = "2024-01-15T12:30:45.123ZGET/api/v5/account/balance";
        let result = sign_hmac_sha256(message, &secret).unwrap();
        // Verify that this is valid base64.
        assert!(!result.is_empty());
        base64::engine::general_purpose::STANDARD
            .decode(&result)
            .unwrap();
    }
}
