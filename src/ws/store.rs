use std::collections::HashSet;

use crate::types::ws::channels::WsSubscriptionArg;
use crate::types::ws::events::WsConnectionType;

/// Connection state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Authenticated,
    Reconnecting,
}

/// Per-connection state.
#[derive(Debug)]
pub struct ConnectionStore {
    pub conn_type: WsConnectionType,
    pub state: ConnectionState,
    pub subscribed_topics: HashSet<WsSubscriptionArg>,
    pub pending_topics: HashSet<WsSubscriptionArg>,
    pub is_authenticated: bool,
}

impl ConnectionStore {
    pub fn new(conn_type: WsConnectionType) -> Self {
        Self {
            conn_type,
            state: ConnectionState::Disconnected,
            subscribed_topics: HashSet::new(),
            pending_topics: HashSet::new(),
            is_authenticated: false,
        }
    }
}

/// WebSocket state store managing all connection states.
#[derive(Debug, Default)]
pub struct WsStore {
    pub public: Option<ConnectionStore>,
    pub private: Option<ConnectionStore>,
    pub business: Option<ConnectionStore>,
}

impl WsStore {
    pub fn new() -> Self {
        Self::default()
    }

    /// Get or create the connection store for a given type.
    pub fn get_or_create(&mut self, conn_type: WsConnectionType) -> &mut ConnectionStore {
        match conn_type {
            WsConnectionType::Public => self
                .public
                .get_or_insert_with(|| ConnectionStore::new(WsConnectionType::Public)),
            WsConnectionType::Private => self
                .private
                .get_or_insert_with(|| ConnectionStore::new(WsConnectionType::Private)),
            WsConnectionType::Business => self
                .business
                .get_or_insert_with(|| ConnectionStore::new(WsConnectionType::Business)),
        }
    }

    /// Get the connection store for a given type (if it exists).
    pub fn get(&self, conn_type: WsConnectionType) -> Option<&ConnectionStore> {
        match conn_type {
            WsConnectionType::Public => self.public.as_ref(),
            WsConnectionType::Private => self.private.as_ref(),
            WsConnectionType::Business => self.business.as_ref(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_store_new() {
        let store = ConnectionStore::new(WsConnectionType::Public);
        assert_eq!(store.conn_type, WsConnectionType::Public);
        assert_eq!(store.state, ConnectionState::Disconnected);
        assert!(!store.is_authenticated);
        assert!(store.subscribed_topics.is_empty());
        assert!(store.pending_topics.is_empty());
    }

    #[test]
    fn test_ws_store_get_or_create() {
        let mut store = WsStore::new();
        assert!(store.get(WsConnectionType::Public).is_none());

        let conn = store.get_or_create(WsConnectionType::Public);
        conn.state = ConnectionState::Connected;

        assert_eq!(
            store.get(WsConnectionType::Public).unwrap().state,
            ConnectionState::Connected
        );
        assert!(store.get(WsConnectionType::Private).is_none());
    }

    #[test]
    fn test_ws_store_all_types() {
        let mut store = WsStore::new();
        store.get_or_create(WsConnectionType::Public);
        store.get_or_create(WsConnectionType::Private);
        store.get_or_create(WsConnectionType::Business);

        assert!(store.get(WsConnectionType::Public).is_some());
        assert!(store.get(WsConnectionType::Private).is_some());
        assert!(store.get(WsConnectionType::Business).is_some());
    }
}
