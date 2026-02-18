use serde::Serialize;

use super::channels::WsSubscriptionArg;

/// WebSocket subscribe/unsubscribe request.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WsSubRequest {
    pub op: String,
    pub args: Vec<WsSubscriptionArg>,
}

impl WsSubRequest {
    /// Create a subscribe request.
    pub fn subscribe(args: Vec<WsSubscriptionArg>) -> Self {
        Self {
            op: "subscribe".to_string(),
            args,
        }
    }

    /// Create an unsubscribe request.
    pub fn unsubscribe(args: Vec<WsSubscriptionArg>) -> Self {
        Self {
            op: "unsubscribe".to_string(),
            args,
        }
    }
}

/// WebSocket login request.
#[derive(Debug, Clone, Serialize)]
pub struct WsLoginRequest {
    pub op: String,
    pub args: Vec<WsLoginArg>,
}

/// Login argument.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WsLoginArg {
    pub api_key: String,
    pub passphrase: String,
    pub timestamp: String,
    pub sign: String,
}

/// WS API request (order operations via WebSocket).
#[derive(Debug, Clone, Serialize)]
pub struct WsApiRequest {
    pub id: String,
    pub op: String,
    pub args: Vec<serde_json::Value>,
}
