use thiserror::Error;

/// All errors that can occur when using the OKX client.
#[derive(Error, Debug)]
pub enum OkxError {
    /// OKX API returned an error response (code != "0").
    #[error("OKX API error {code}: {msg}")]
    Api { code: String, msg: String },

    /// HTTP transport error from reqwest.
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// HTTP middleware error (retry exhausted, etc.).
    #[error("HTTP middleware error: {0}")]
    Middleware(#[from] reqwest_middleware::Error),

    /// WebSocket transport error.
    #[error("WebSocket error: {0}")]
    WebSocket(#[from] tokio_tungstenite::tungstenite::Error),

    /// Authentication error (missing credentials, invalid key format).
    #[error("Authentication error: {0}")]
    Auth(String),

    /// JSON serialization/deserialization error.
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// WebSocket API request timed out waiting for response.
    #[error("WS API request timed out (id={id}, op={operation})")]
    WsApiTimeout { id: String, operation: String },

    /// WebSocket connection lost while a request was in flight.
    #[error("WebSocket connection lost")]
    WsConnectionLost,

    /// URL parsing error.
    #[error("URL error: {0}")]
    Url(#[from] url::ParseError),

    /// Configuration error.
    #[error("Configuration error: {0}")]
    Config(String),

    /// General WebSocket error (connection, send, etc.).
    #[error("WebSocket error: {0}")]
    Ws(String),
}

/// Convenience alias for `Result<T, OkxError>`.
pub type OkxResult<T> = Result<T, OkxError>;
