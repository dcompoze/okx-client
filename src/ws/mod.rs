pub mod api;
pub mod api_client;
pub mod auth;
pub mod connection;
pub mod heartbeat;
pub mod router;
pub mod store;
pub mod types;

use std::sync::Arc;

use futures_util::future::BoxFuture;
use tokio::sync::{broadcast, mpsc, Mutex, RwLock};
use tracing::{error, info, warn};

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
/// The client is cheap to clone -- all clones share the same underlying
/// connections and state.
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
/// let mut rx = client.subscribe(vec![
///     WsSubscriptionArg::with_inst_id("tickers", "BTC-USDT"),
/// ]).await.unwrap();
/// let msg = rx.recv().await.unwrap();
/// println!("{msg:?}");
/// # }
/// ```
#[derive(Clone)]
pub struct WebsocketClient {
    config: WsConfig,
    store: Arc<RwLock<WsStore>>,
    event_tx: broadcast::Sender<WsMessage>,
    pending_requests: Arc<Mutex<PendingRequests>>,
    /// Channels for sending raw text to the per-connection write loops.
    write_txs: Arc<RwLock<WriteChannels>>,
}

#[derive(Default, Clone)]
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

/// Partition subscription args by their target connection type.
fn partition_args(
    args: Vec<WsSubscriptionArg>,
) -> (
    Vec<WsSubscriptionArg>,
    Vec<WsSubscriptionArg>,
    Vec<WsSubscriptionArg>,
) {
    let mut public = Vec::new();
    let mut private = Vec::new();
    let mut business = Vec::new();
    for arg in args {
        match router::route_subscription(&arg) {
            WsConnectionType::Public => public.push(arg),
            WsConnectionType::Private => private.push(arg),
            WsConnectionType::Business => business.push(arg),
        }
    }
    (public, private, business)
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
    ///
    /// Automatically connects if needed and routes to the correct connection.
    pub async fn subscribe(
        &self,
        args: Vec<WsSubscriptionArg>,
    ) -> OkxResult<broadcast::Receiver<WsMessage>> {
        let (public_args, private_args, business_args) = partition_args(args);

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
        let (public_args, private_args, business_args) = partition_args(args);

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
        let json = serde_json::to_string(&request)?;

        let rx = {
            let mut pending = self.pending_requests.lock().await;
            pending.register(request.id)
        };
        let write_txs = self.write_txs.read().await;
        if let Some(tx) = write_txs.get(conn_type) {
            tx.send(json)
                .map_err(|_| OkxError::Ws("write channel closed".into()))?;
        } else {
            return Err(OkxError::Ws(format!("no {conn_type} connection")));
        }

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

        self.connect(conn_type).await
    }

    /// Establish a WebSocket connection.
    async fn connect(&self, conn_type: WsConnectionType) -> OkxResult<()> {
        self.clone().connect_inner(conn_type).await
    }

    /// Send a subscribe message on a specific connection.
    async fn send_subscribe(
        &self,
        conn_type: WsConnectionType,
        args: Vec<WsSubscriptionArg>,
    ) -> OkxResult<()> {
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

        let req = WsSubRequest::subscribe(args);
        let json = serde_json::to_string(&req)?;

        let write_txs = self.write_txs.read().await;
        if let Some(tx) = write_txs.get(conn_type) {
            tx.send(json)
                .map_err(|_| OkxError::Ws("write channel closed".into()))?;
        }

        let mut store = self.store.write().await;
        let conn = store.get_or_create(conn_type);
        for arg in req.args {
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
        let req = WsSubRequest::unsubscribe(args);
        let json = serde_json::to_string(&req)?;

        let write_txs = self.write_txs.read().await;
        if let Some(tx) = write_txs.get(conn_type) {
            tx.send(json)
                .map_err(|_| OkxError::Ws("write channel closed".into()))?;
        }

        let mut store = self.store.write().await;
        let conn = store.get_or_create(conn_type);
        for arg in &req.args {
            conn.subscribed_topics.remove(arg);
        }

        Ok(())
    }

    /// Establish a WebSocket connection, taking `self` by value.
    ///
    /// Owning `self` (rather than borrowing) makes the returned future
    /// provably `Send`, which is required when this is awaited inside a
    /// `tokio::spawn` task (e.g. the auto-reconnect path).
    fn connect_inner(self, conn_type: WsConnectionType) -> BoxFuture<'static, OkxResult<()>> {
        Box::pin(async move {
        let url = self.config.ws_url(conn_type).to_owned();
        info!("Connecting WS {conn_type} to {url}");

        {
            let mut store = self.store.write().await;
            store.get_or_create(conn_type).state = ConnectionState::Connecting;
        }

        let ws = connection::connect(&url).await?;
        let (write_tx, mut msg_rx) = connection::spawn_io_tasks(ws, conn_type);

        let (hb_stop_tx, hb_stop_rx) = tokio::sync::oneshot::channel::<()>();
        let hb_tx = write_tx.clone();
        let ping_interval = self.config.ping_interval;
        tokio::spawn(async move {
            heartbeat::heartbeat_loop(hb_tx, ping_interval, hb_stop_rx).await;
        });

        {
            let mut write_txs = self.write_txs.write().await;
            write_txs.set(conn_type, write_tx.clone());
        }

        let event_tx = self.event_tx.clone();
        let client_for_reconnect = self.clone();
        let store = self.store.clone();
        let pending_requests = self.pending_requests.clone();
        let write_txs = self.write_txs.clone();

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

                            let pending: Vec<_> = conn.pending_topics.drain().collect();
                            if !pending.is_empty() {
                                let req = WsSubRequest::subscribe(pending);
                                if let Ok(json) = serde_json::to_string(&req) {
                                    let wt = write_txs.read().await;
                                    if let Some(tx) = wt.get(conn_type) {
                                        let _ = tx.send(json);
                                    }
                                }
                                let conn = s.get_or_create(conn_type);
                                for topic in req.args {
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
                        {
                            let mut s = store.write().await;
                            let conn = s.get_or_create(conn_type);
                            conn.state = ConnectionState::Disconnected;
                            conn.is_authenticated = false;
                        }

                        {
                            let mut pending = pending_requests.lock().await;
                            pending.reject_all();
                        }

                        {
                            let mut wt = write_txs.write().await;
                            wt.remove(conn_type);
                        }

                        if client_for_reconnect.config.auto_reconnect {
                            let delay = client_for_reconnect.config.reconnect_delay;
                            let client = client_for_reconnect.clone();
                            tokio::spawn(async move {
                                info!("WS {conn_type} reconnecting in {delay:?}");
                                tokio::time::sleep(delay).await;

                                // For authenticated connections, move subscribed topics into
                                // pending so the login handler resubscribes them after auth.
                                // For public connections, capture them for direct resubscription.
                                let public_topics =
                                    if conn_type == WsConnectionType::Public {
                                        let s = client.store.read().await;
                                        s.get(conn_type)
                                            .map(|c| {
                                                c.subscribed_topics
                                                    .iter()
                                                    .cloned()
                                                    .collect::<Vec<_>>()
                                            })
                                            .unwrap_or_default()
                                    } else {
                                        let mut s = client.store.write().await;
                                        let conn = s.get_or_create(conn_type);
                                        let topics: Vec<_> =
                                            conn.subscribed_topics.drain().collect();
                                        for topic in &topics {
                                            conn.pending_topics.insert(topic.clone());
                                        }
                                        Vec::new()
                                    };

                                // Keep a clone for resubscription since connect_inner
                                // consumes `client`.
                                let client_ref = client.clone();
                                match client_ref.connect(conn_type).await {
                                    Ok(()) => {
                                        if !public_topics.is_empty() {
                                            if let Err(e) = client_ref
                                                .send_subscribe(conn_type, public_topics)
                                                .await
                                            {
                                                error!(
                                                    "WS {conn_type} resubscribe failed: {e}"
                                                );
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        error!("WS {conn_type} reconnect failed: {e}");
                                    }
                                }
                            });
                        }

                        break;
                    }
                    _ => {}
                }

                let _ = event_tx.send(msg);
            }

            let _ = hb_stop_tx.send(());
        });

        {
            let mut s = self.store.write().await;
            s.get_or_create(conn_type).state = ConnectionState::Connected;
        }

        if conn_type != WsConnectionType::Public {
            if let Some(creds) = self.config.client_config.credentials.clone() {
                let login_req = auth::build_login_request(&creds)?;
                let json = serde_json::to_string(&login_req)?;
                let write_txs = self.write_txs.read().await;
                if let Some(tx) = write_txs.get(conn_type) {
                    tx.send(json)
                        .map_err(|_| OkxError::Ws("write channel closed".into()))?;
                }
            }
        }

        let _ = self.event_tx.send(WsMessage::Connected(conn_type));

        info!("WS {conn_type} connected");
        Ok(())
        })
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
