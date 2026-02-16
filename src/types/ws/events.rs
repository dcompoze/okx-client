use serde::{Deserialize, Serialize};

use super::channels::WsSubscriptionArg;

/// A WebSocket data event (pushed data from subscriptions).
#[derive(Debug, Clone, Deserialize)]
pub struct WsDataEvent {
    /// The subscription arg that identifies the channel and parameters.
    pub arg: WsSubscriptionArg,
    /// The actual data payload. Structure depends on the channel.
    pub data: Vec<serde_json::Value>,
    /// Action type for order book updates.
    #[serde(default)]
    pub action: Option<String>,
}

/// A WebSocket event (login, subscribe, unsubscribe, error, etc.).
#[derive(Debug, Clone, Deserialize)]
pub struct WsEvent {
    pub event: String,
    #[serde(default)]
    pub code: Option<String>,
    #[serde(default)]
    pub msg: Option<String>,
    #[serde(default)]
    pub arg: Option<serde_json::Value>,
    #[serde(default)]
    pub data: Option<serde_json::Value>,
    /// Connection count info.
    #[serde(default, rename = "connCount")]
    pub conn_count: Option<String>,
}

/// Events emitted by the WebSocket client.
#[derive(Debug, Clone)]
pub enum WsMessage {
    /// Data update from a subscription.
    Data(WsDataEvent),
    /// Control event (login, subscribe confirmation, error, etc.).
    Event(WsEvent),
    /// Raw pong response.
    Pong,
    /// WS API response.
    ApiResponse(WsApiResponse),
    /// Connection opened.
    Connected(WsConnectionType),
    /// Connection closed.
    Disconnected(WsConnectionType),
}

/// WS API response (for order management via WebSocket).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsApiResponse {
    pub id: String,
    pub op: String,
    pub code: String,
    #[serde(default)]
    pub msg: String,
    #[serde(default)]
    pub data: Vec<serde_json::Value>,
    #[serde(default, rename = "inTime")]
    pub in_time: Option<String>,
    #[serde(default, rename = "outTime")]
    pub out_time: Option<String>,
}

/// Type of WebSocket connection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WsConnectionType {
    Public,
    Private,
    Business,
}

impl std::fmt::Display for WsConnectionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Public => write!(f, "public"),
            Self::Private => write!(f, "private"),
            Self::Business => write!(f, "business"),
        }
    }
}
