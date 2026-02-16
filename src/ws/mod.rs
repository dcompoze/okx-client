pub mod api;
pub mod auth;
pub mod connection;
pub mod heartbeat;
pub mod router;
pub mod store;
pub mod types;

use std::sync::Arc;

use tokio::sync::{broadcast, mpsc, Mutex, RwLock};
use tracing::{debug, error, info, warn};

use crate::error::{OkxError, OkxResult};
use crate::types::ws::channels::WsSubscriptionArg;
use crate::types::ws::events::{WsConnectionType, WsMessage};
use crate::types::ws::requests::WsSubRequest;

use self::api::PendingRequests;
use self::store::{ConnectionState, WsStore};
use self::types::WsConfig;

/// WebSocket client for OKX real-time data and order management.
///
/// Manages multiple connections (public, private, business) and
/// automatically routes subscriptions to the correct connection.
///
/// # Example
///
/// ```no_run
/// use okx_client::ws::WebsocketClient;
/// use okx_client::ws::types::WsConfig;
/// use okx_client::types::ws::channels::WsSubscriptionArg;
///
/// # async fn example() {
/// let config = WsConfig::default();
/// let client = WebsocketClient::new(config);
///
/// // Subscribe to BTC-USDT ticker
/// let mut rx = client.subscribe(vec![
///     WsSubscriptionArg::with_inst_id("tickers", "BTC-USDT"),
/// ]).await.unwrap();
///
/// // Receive messages
/// while let Ok(msg) = rx.recv().await {
///     println!("Received: {:?}", msg);
/// }
/// # }
/// ```
pub struct WebsocketClient {
    config: WsConfig,
    store: Arc<RwLock<WsStore>>,
    event_tx: broadcast::Sender<WsMessage>,
    pending_requests: Arc<Mutex<PendingRequests>>,
    /// Channel for sending raw text to the write loops
    write_txs: Arc<RwLock<WriteChannels>>,
}

#[derive(Default)]
struct WriteChannels {
    public: Option<mpsc::UnboundedSender<String>>,
    private: Option<mpsc::UnboundedSender<String>>,
    business: Option<mpsc::UnboundedSender<String>>,
}

impl WriteChannels {
    fn get(&self, conn_type: WsConnectionType) -> Option<&mpsc::UnboundedSender<String>> {
        match conn_type {
            WsConnectionType::Public => self.public.as_ref(),
            WsConnectionType::Private => self.private.as_ref(),
            WsConnectionType::Business => self.business.as_ref(),
        }
    }

    fn set(&mut self, conn_type: WsConnectionType, tx: mpsc::UnboundedSender<String>) {
        match conn_type {
            WsConnectionType::Public => self.public = Some(tx),
            WsConnectionType::Private => self.private = Some(tx),
            WsConnectionType::Business => self.business = Some(tx),
        }
    }

    fn remove(&mut self, conn_type: WsConnectionType) {
        match conn_type {
            WsConnectionType::Public => self.public = None,
            WsConnectionType::Private => self.private = None,
            WsConnectionType::Business => self.business = None,
        }
    }
}

impl WebsocketClient {
    /// Create a new WebSocket client with the given configuration.
    pub fn new(config: WsConfig) -> Self {
        let (event_tx, _) = broadcast::channel(1024);
        Self {
            config,
            store: Arc::new(RwLock::new(WsStore::new())),
            event_tx,
            pending_requests: Arc::new(Mutex::new(PendingRequests::new())),
            write_txs: Arc::new(RwLock::new(WriteChannels::default())),
        }
    }

    /// Get a broadcast receiver for all WebSocket events.
    pub fn event_receiver(&self) -> broadcast::Receiver<WsMessage> {
        self.event_tx.subscribe()
    }

    /// Subscribe to one or more channels.
    /// Automatically connects if needed and routes to the correct connection.
    pub async fn subscribe(
        &self,
        args: Vec<WsSubscriptionArg>,
    ) -> OkxResult<broadcast::Receiver<WsMessage>> {
        // Group subscriptions by connection type
        let mut public_args = Vec::new();
        let mut private_args = Vec::new();
        let mut business_args = Vec::new();

        for arg in &args {
            match router::route_subscription(arg) {
                WsConnectionType::Public => public_args.push(arg.clone()),
                WsConnectionType::Private => private_args.push(arg.clone()),
                WsConnectionType::Business => business_args.push(arg.clone()),
            }
        }

        // Connect and subscribe to each connection type as needed
        if !public_args.is_empty() {
            self.ensure_connected(WsConnectionType::Public).await?;
            self.send_subscribe(WsConnectionType::Public, public_args)
                .await?;
        }
        if !private_args.is_empty() {
            self.ensure_connected(WsConnectionType::Private).await?;
            self.send_subscribe(WsConnectionType::Private, private_args)
                .await?;
        }
        if !business_args.is_empty() {
            self.ensure_connected(WsConnectionType::Business).await?;
            self.send_subscribe(WsConnectionType::Business, business_args)
                .await?;
        }

        Ok(self.event_tx.subscribe())
    }

    /// Unsubscribe from one or more channels.
    pub async fn unsubscribe(&self, args: Vec<WsSubscriptionArg>) -> OkxResult<()> {
        let mut public_args = Vec::new();
        let mut private_args = Vec::new();
        let mut business_args = Vec::new();

        for arg in &args {
            match router::route_subscription(arg) {
                WsConnectionType::Public => public_args.push(arg.clone()),
                WsConnectionType::Private => private_args.push(arg.clone()),
                WsConnectionType::Business => business_args.push(arg.clone()),
            }
        }

        if !public_args.is_empty() {
            self.send_unsubscribe(WsConnectionType::Public, public_args)
                .await?;
        }
        if !private_args.is_empty() {
            self.send_unsubscribe(WsConnectionType::Private, private_args)
                .await?;
        }
        if !business_args.is_empty() {
            self.send_unsubscribe(WsConnectionType::Business, business_args)
                .await?;
        }

        Ok(())
    }

    /// Send a WS API request and wait for the response.
    pub async fn send_api_request(
        &self,
        op: &str,
        args: Vec<serde_json::Value>,
    ) -> OkxResult<crate::types::ws::events::WsApiResponse> {
        let conn_type = if op.starts_with("sprd-") {
            WsConnectionType::Business
        } else {
            WsConnectionType::Private
        };

        self.ensure_connected(conn_type).await?;

        let request = api::build_api_request(op, args);
        let id = request.id.clone();

        // Register pending request
        let rx = {
            let mut pending = self.pending_requests.lock().await;
            pending.register(id)
        };

        // Send the request
        let json = serde_json::to_string(&request)?;
        let write_txs = self.write_txs.read().await;
        if let Some(tx) = write_txs.get(conn_type) {
            tx.send(json)
                .map_err(|_| OkxError::Ws("Write channel closed".into()))?;
        } else {
            return Err(OkxError::Ws(format!("No {conn_type} connection")));
        }

        // Wait for response with timeout
        let response = tokio::time::timeout(std::time::Duration::from_secs(10), rx)
            .await
            .map_err(|_| OkxError::Ws("WS API request timed out".into()))?
            .map_err(|_| OkxError::Ws("WS API request cancelled".into()))?;

        if response.code == "0" {
            Ok(response)
        } else {
            Err(OkxError::Api {
                code: response.code,
                msg: response.msg,
            })
        }
    }

    /// Ensure a connection of the given type is established.
    async fn ensure_connected(&self, conn_type: WsConnectionType) -> OkxResult<()> {
        // Check if already connected
        {
            let store = self.store.read().await;
            if let Some(conn) = store.get(conn_type) {
                if conn.state == ConnectionState::Connected
                    || conn.state == ConnectionState::Authenticated
                {
                    return Ok(());
                }
            }
        }

        // Connect
        self.connect(conn_type).await
    }

    /// Establish a WebSocket connection.
    async fn connect(&self, conn_type: WsConnectionType) -> OkxResult<()> {
        let url = self.config.ws_url(conn_type);
        info!("Connecting WS {conn_type} to {url}");

        // Update store state
        {
            let mut store = self.store.write().await;
            let conn = store.get_or_create(conn_type);
            conn.state = ConnectionState::Connecting;
        }

        let ws = connection::connect(url).await?;

        // Split into read/write
        let (mut write, read) = futures_util::StreamExt::split(ws);

        // Create write channel
        let (write_tx, mut write_rx) = mpsc::unbounded_channel::<String>();

        // Store the write channel
        {
            let mut write_txs = self.write_txs.write().await;
            write_txs.set(conn_type, write_tx.clone());
        }

        // Spawn the write loop
        tokio::spawn(async move {
            use futures_util::SinkExt;
            while let Some(msg) = write_rx.recv().await {
                if let Err(e) = write.send(tokio_tungstenite::tungstenite::Message::Text(msg.into())).await {
                    error!("WS {conn_type} write error: {e}");
                    break;
                }
            }
            debug!("WS {conn_type} write loop ended");
        });

        // Spawn the heartbeat
        let (hb_stop_tx, hb_stop_rx) = tokio::sync::oneshot::channel();
        let hb_tx = write_tx.clone();
        let ping_interval = self.config.ping_interval;
        tokio::spawn(async move {
            heartbeat::heartbeat_loop(hb_tx, ping_interval, hb_stop_rx).await;
        });

        // Spawn the message processing loop
        let event_tx = self.event_tx.clone();
        let (msg_tx, mut msg_rx) = mpsc::unbounded_channel();
        let read_stream = futures_util::stream::StreamExt::map(read, |r| r);

        // Read loop task
        tokio::spawn(async move {
            use futures_util::StreamExt;
            let mut read_stream = read_stream;
            while let Some(msg_result) = read_stream.next().await {
                match msg_result {
                    Ok(tokio_tungstenite::tungstenite::Message::Text(text)) => {
                        if let Some(parsed) = connection::parse_ws_message(&text) {
                            if msg_tx.send(parsed).is_err() {
                                break;
                            }
                        }
                    }
                    Ok(tokio_tungstenite::tungstenite::Message::Close(_)) => {
                        let _ = msg_tx.send(WsMessage::Disconnected(conn_type));
                        break;
                    }
                    Err(e) => {
                        error!("WS {conn_type} read error: {e}");
                        let _ = msg_tx.send(WsMessage::Disconnected(conn_type));
                        break;
                    }
                    _ => {}
                }
            }
        });

        // Message processing task
        let store = self.store.clone();
        let pending_requests = self.pending_requests.clone();
        let write_txs = self.write_txs.clone();
        let _config = self.config.clone();
        tokio::spawn(async move {
            while let Some(msg) = msg_rx.recv().await {
                match &msg {
                    WsMessage::Event(evt) if evt.event == "login" => {
                        if evt.code.as_deref() == Some("0") {
                            info!("WS {conn_type} authenticated");
                            let mut s = store.write().await;
                            let conn = s.get_or_create(conn_type);
                            conn.is_authenticated = true;
                            conn.state = ConnectionState::Authenticated;

                            // Subscribe to any pending private topics
                            let pending: Vec<_> = conn.pending_topics.drain().collect();
                            if !pending.is_empty() {
                                let req = WsSubRequest::subscribe(pending.clone());
                                if let Ok(json) = serde_json::to_string(&req) {
                                    let wt = write_txs.read().await;
                                    if let Some(tx) = wt.get(conn_type) {
                                        let _ = tx.send(json);
                                    }
                                }
                                let conn = s.get_or_create(conn_type);
                                for topic in pending {
                                    conn.subscribed_topics.insert(topic);
                                }
                            }
                        } else {
                            error!("WS {conn_type} login failed: {:?}", evt.msg);
                        }
                    }
                    WsMessage::ApiResponse(resp) => {
                        let mut pending = pending_requests.lock().await;
                        pending.resolve(&resp.id, resp.clone());
                    }
                    WsMessage::Disconnected(_) => {
                        warn!("WS {conn_type} disconnected");
                        let mut s = store.write().await;
                        let conn = s.get_or_create(conn_type);
                        conn.state = ConnectionState::Disconnected;
                        conn.is_authenticated = false;

                        // Reject all pending API requests
                        let mut pending = pending_requests.lock().await;
                        pending.reject_all();

                        // Clean up write channel
                        let mut wt = write_txs.write().await;
                        wt.remove(conn_type);

                        break;
                    }
                    _ => {}
                }

                // Broadcast to all subscribers
                let _ = event_tx.send(msg);
            }

            // Stop heartbeat on exit
            let _ = hb_stop_tx.send(());
        });

        // Update state to connected
        {
            let mut s = self.store.write().await;
            let conn = s.get_or_create(conn_type);
            conn.state = ConnectionState::Connected;
        }

        // Auto-login for private/business connections
        if conn_type != WsConnectionType::Public {
            if let Some(creds) = &self.config.client_config.credentials {
                let login_req = auth::build_login_request(creds)?;
                let json = serde_json::to_string(&login_req)?;
                let write_txs = self.write_txs.read().await;
                if let Some(tx) = write_txs.get(conn_type) {
                    tx.send(json)
                        .map_err(|_| OkxError::Ws("Write channel closed".into()))?;
                }
            }
        }

        let _ = self
            .event_tx
            .send(WsMessage::Connected(conn_type));

        info!("WS {conn_type} connected");
        Ok(())
    }

    /// Send a subscribe message on a specific connection.
    async fn send_subscribe(
        &self,
        conn_type: WsConnectionType,
        args: Vec<WsSubscriptionArg>,
    ) -> OkxResult<()> {
        // For private connections, queue topics if not yet authenticated
        if conn_type != WsConnectionType::Public {
            let store = self.store.read().await;
            if let Some(conn) = store.get(conn_type) {
                if !conn.is_authenticated {
                    drop(store);
                    let mut store = self.store.write().await;
                    let conn = store.get_or_create(conn_type);
                    for arg in args {
                        conn.pending_topics.insert(arg);
                    }
                    return Ok(());
                }
            }
        }

        let req = WsSubRequest::subscribe(args.clone());
        let json = serde_json::to_string(&req)?;

        let write_txs = self.write_txs.read().await;
        if let Some(tx) = write_txs.get(conn_type) {
            tx.send(json)
                .map_err(|_| OkxError::Ws("Write channel closed".into()))?;
        }

        // Track subscribed topics
        let mut store = self.store.write().await;
        let conn = store.get_or_create(conn_type);
        for arg in args {
            conn.subscribed_topics.insert(arg);
        }

        Ok(())
    }

    /// Send an unsubscribe message on a specific connection.
    async fn send_unsubscribe(
        &self,
        conn_type: WsConnectionType,
        args: Vec<WsSubscriptionArg>,
    ) -> OkxResult<()> {
        let req = WsSubRequest::unsubscribe(args.clone());
        let json = serde_json::to_string(&req)?;

        let write_txs = self.write_txs.read().await;
        if let Some(tx) = write_txs.get(conn_type) {
            tx.send(json)
                .map_err(|_| OkxError::Ws("Write channel closed".into()))?;
        }

        // Remove from tracked topics
        let mut store = self.store.write().await;
        let conn = store.get_or_create(conn_type);
        for arg in &args {
            conn.subscribed_topics.remove(arg);
        }

        Ok(())
    }

    /// Close all connections.
    pub async fn close_all(&self) {
        let mut write_txs = self.write_txs.write().await;
        write_txs.public = None;
        write_txs.private = None;
        write_txs.business = None;

        let mut store = self.store.write().await;
        if let Some(conn) = &mut store.public {
            conn.state = ConnectionState::Disconnected;
        }
        if let Some(conn) = &mut store.private {
            conn.state = ConnectionState::Disconnected;
        }
        if let Some(conn) = &mut store.business {
            conn.state = ConnectionState::Disconnected;
        }
    }
}
