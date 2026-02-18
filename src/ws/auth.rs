use secrecy::ExposeSecret;

use crate::auth;
use crate::config::Credentials;
use crate::error::{OkxError, OkxResult};
use crate::types::ws::requests::{WsLoginArg, WsLoginRequest};

/// Build a WebSocket login request from credentials.
pub fn build_login_request(creds: &Credentials) -> OkxResult<WsLoginRequest> {
    let timestamp = ws_timestamp()?;

    let signature = auth::sign_ws(&timestamp.to_string(), &creds.api_secret)?;

    Ok(WsLoginRequest {
        op: "login".to_string(),
        args: vec![WsLoginArg {
            api_key: creds.api_key.clone(),
            passphrase: creds.passphrase.expose_secret().to_string(),
            timestamp: timestamp.to_string(),
            sign: signature,
        }],
    })
}

/// Generate a Unix timestamp (seconds) for WS auth.
fn ws_timestamp() -> OkxResult<u64> {
    Ok(std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|_| OkxError::Config("system time is before Unix epoch".into()))?
        .as_secs())
}
