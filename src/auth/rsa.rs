use base64::Engine;
use rsa::pkcs8::DecodePrivateKey;
use rsa::RsaPrivateKey;
use secrecy::{ExposeSecret, SecretString};
use sha2::Sha256;

use crate::error::OkxError;

/// Sign a message with RSA-PKCS1v1.5-SHA256 and return the base64-encoded signature.
pub fn sign_rsa(message: &str, secret: &SecretString) -> Result<String, OkxError> {
    use rsa::pkcs1v15::SigningKey;
    use rsa::signature::{SignatureEncoding, Signer};

    let private_key = RsaPrivateKey::from_pkcs8_pem(secret.expose_secret())
        .map_err(|e| OkxError::Auth(format!("Invalid RSA key: {e}")))?;
    let signing_key = SigningKey::<Sha256>::new(private_key);
    let signature = signing_key
        .sign(message.as_bytes());
    Ok(base64::engine::general_purpose::STANDARD.encode(signature.to_bytes()))
}
