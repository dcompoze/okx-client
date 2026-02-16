use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};

use tokio::sync::oneshot;

use crate::types::ws::events::WsApiResponse;
use crate::types::ws::requests::WsApiRequest;

/// Counter for generating unique request IDs.
static REQUEST_ID_COUNTER: AtomicU64 = AtomicU64::new(1);

/// Generate a unique request ID.
pub fn next_request_id() -> String {
    REQUEST_ID_COUNTER.fetch_add(1, Ordering::SeqCst).to_string()
}

/// Build a WS API request.
pub fn build_api_request(op: &str, args: Vec<serde_json::Value>) -> WsApiRequest {
    WsApiRequest {
        id: next_request_id(),
        op: op.to_string(),
        args,
    }
}

/// Pending WS API request tracker. Maps request ID to oneshot sender.
#[derive(Debug, Default)]
pub struct PendingRequests {
    inner: HashMap<String, oneshot::Sender<WsApiResponse>>,
}

impl PendingRequests {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a pending request and return a receiver for the response.
    pub fn register(&mut self, id: String) -> oneshot::Receiver<WsApiResponse> {
        let (tx, rx) = oneshot::channel();
        self.inner.insert(id, tx);
        rx
    }

    /// Resolve a pending request with a response.
    pub fn resolve(&mut self, id: &str, response: WsApiResponse) -> bool {
        if let Some(tx) = self.inner.remove(id) {
            let _ = tx.send(response);
            true
        } else {
            false
        }
    }

    /// Reject all pending requests (e.g., on disconnect).
    pub fn reject_all(&mut self) {
        self.inner.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_request_id_unique() {
        let id1 = next_request_id();
        let id2 = next_request_id();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_build_api_request() {
        let req = build_api_request("order", vec![serde_json::json!({"instId": "BTC-USDT"})]);
        assert_eq!(req.op, "order");
        assert_eq!(req.args.len(), 1);
        assert!(!req.id.is_empty());
    }

    #[test]
    fn test_pending_requests_resolve() {
        let mut pending = PendingRequests::new();
        let mut rx = pending.register("test-1".to_string());

        let response = WsApiResponse {
            id: "test-1".to_string(),
            op: "order".to_string(),
            code: "0".to_string(),
            msg: String::new(),
            data: vec![],
            in_time: None,
            out_time: None,
        };

        assert!(pending.resolve("test-1", response));

        let result = rx.try_recv().unwrap();
        assert_eq!(result.id, "test-1");
        assert_eq!(result.code, "0");
    }

    #[test]
    fn test_pending_requests_resolve_unknown() {
        let mut pending = PendingRequests::new();

        let response = WsApiResponse {
            id: "unknown".to_string(),
            op: "order".to_string(),
            code: "0".to_string(),
            msg: String::new(),
            data: vec![],
            in_time: None,
            out_time: None,
        };

        assert!(!pending.resolve("unknown", response));
    }

    #[test]
    fn test_pending_requests_reject_all() {
        let mut pending = PendingRequests::new();
        let mut rx1 = pending.register("1".to_string());
        let mut rx2 = pending.register("2".to_string());

        pending.reject_all();

        // Receivers should get an error because senders were dropped.
        assert!(rx1.try_recv().is_err());
        assert!(rx2.try_recv().is_err());
    }
}
