use std::time::Duration;

use crate::config::{ClientConfig, Region, TradingMode};
use crate::constants::ws_urls;
use crate::types::ws::events::WsConnectionType;

/// Configuration for the WebSocket client.
#[derive(Debug, Clone)]
pub struct WsConfig {
    /// Client configuration (credentials, region, trading mode).
    pub client_config: ClientConfig,
    /// Ping interval (default: 10 seconds).
    pub ping_interval: Duration,
    /// Pong timeout (default: 5 seconds).
    pub pong_timeout: Duration,
    /// Reconnect delay (default: 500ms).
    pub reconnect_delay: Duration,
    /// Whether auto-reconnect is enabled (default: true).
    pub auto_reconnect: bool,
}

impl WsConfig {
    pub fn new(client_config: ClientConfig) -> Self {
        Self {
            client_config,
            ping_interval: Duration::from_secs(10),
            pong_timeout: Duration::from_secs(5),
            reconnect_delay: Duration::from_millis(500),
            auto_reconnect: true,
        }
    }

    /// Get the WebSocket URL for a given connection type.
    pub fn ws_url(&self, conn_type: WsConnectionType) -> &str {
        if self.client_config.trading_mode == TradingMode::Demo {
            return match conn_type {
                WsConnectionType::Public => ws_urls::DEMO_PUBLIC,
                WsConnectionType::Private => ws_urls::DEMO_PRIVATE,
                WsConnectionType::Business => ws_urls::DEMO_BUSINESS,
            };
        }

        match (&self.client_config.region, conn_type) {
            (Region::Global, WsConnectionType::Public) => ws_urls::GLOBAL_PUBLIC,
            (Region::Global, WsConnectionType::Private) => ws_urls::GLOBAL_PRIVATE,
            (Region::Global, WsConnectionType::Business) => ws_urls::GLOBAL_BUSINESS,
            (Region::Eea, WsConnectionType::Public) => ws_urls::EEA_PUBLIC,
            (Region::Eea, WsConnectionType::Private) => ws_urls::EEA_PRIVATE,
            (Region::Eea, WsConnectionType::Business) => ws_urls::EEA_BUSINESS,
            (Region::Us, WsConnectionType::Public) => ws_urls::US_PUBLIC,
            (Region::Us, WsConnectionType::Private) => ws_urls::US_PRIVATE,
            (Region::Us, WsConnectionType::Business) => ws_urls::US_BUSINESS,
        }
    }
}

impl Default for WsConfig {
    fn default() -> Self {
        Self::new(ClientConfig::default())
    }
}
