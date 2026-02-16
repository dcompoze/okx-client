use base64::Engine;
use ring::signature::Ed25519KeyPair;
use secrecy::{ExposeSecret, SecretString};

use crate::error::OkxError;

/// Sign a message with Ed25519 and return the base64-encoded signature.
pub fn sign_ed25519(message: &str, secret: &SecretString) -> Result<String, OkxError> {
    // The secret should be a PEM-encoded PKCS8 Ed25519 private key.
    // We need to decode the PEM to get the DER bytes.
    let pem_str = secret.expose_secret();
    let der_bytes = pem_to_der(pem_str)?;

    let key_pair = Ed25519KeyPair::from_pkcs8(&der_bytes)
        .map_err(|e| OkxError::Auth(format!("Invalid Ed25519 key: {e}")))?;
    let signature = key_pair.sign(message.as_bytes());
    Ok(base64::engine::general_purpose::STANDARD.encode(signature.as_ref()))
}

fn pem_to_der(pem: &str) -> Result<Vec<u8>, OkxError> {
    let lines: Vec<&str> = pem
        .lines()
        .filter(|l| !l.starts_with("-----"))
        .collect();
    let b64 = lines.join("");
    base64::engine::general_purpose::STANDARD
        .decode(b64)
        .map_err(|e| OkxError::Auth(format!("Failed to decode PEM: {e}")))
}
