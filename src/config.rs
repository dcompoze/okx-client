use std::time::Duration;

use secrecy::SecretString;

use crate::constants;

/// OKX regional endpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Region {
    Global,
    Eea,
    Us,
}

impl Region {
    /// Returns the REST API base URL for this region.
    pub fn rest_base_url(&self) -> &'static str {
        match self {
            Region::Global => constants::rest_urls::GLOBAL,
            Region::Eea => constants::rest_urls::EEA,
            Region::Us => constants::rest_urls::US,
        }
    }
}

/// Live vs demo (simulated) trading.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TradingMode {
    Live,
    Demo,
}

/// API credentials for authenticated requests.
#[derive(Clone)]
pub struct Credentials {
    pub api_key: String,
    pub api_secret: SecretString,
    pub passphrase: SecretString,
}

impl std::fmt::Debug for Credentials {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Credentials")
            .field("api_key", &self.api_key)
            .field("api_secret", &"[REDACTED]")
            .field("passphrase", &"[REDACTED]")
            .finish()
    }
}

/// Configuration for `RestClient` and `WebsocketClient`.
#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub region: Region,
    pub trading_mode: TradingMode,
    pub credentials: Option<Credentials>,
    pub base_url_override: Option<String>,
    pub request_timeout: Duration,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            region: Region::Global,
            trading_mode: TradingMode::Live,
            credentials: None,
            base_url_override: None,
            request_timeout: Duration::from_secs(30),
        }
    }
}

/// Builder for `ClientConfig`.
pub struct ClientConfigBuilder {
    config: ClientConfig,
}

impl ClientConfigBuilder {
    pub fn new() -> Self {
        Self {
            config: ClientConfig::default(),
        }
    }

    pub fn region(mut self, region: Region) -> Self {
        self.config.region = region;
        self
    }

    pub fn trading_mode(mut self, mode: TradingMode) -> Self {
        self.config.trading_mode = mode;
        self
    }

    pub fn demo(mut self) -> Self {
        self.config.trading_mode = TradingMode::Demo;
        self
    }

    pub fn credentials(mut self, api_key: &str, api_secret: &str, passphrase: &str) -> Self {
        self.config.credentials = Some(Credentials {
            api_key: api_key.to_string(),
            api_secret: SecretString::from(api_secret.to_string()),
            passphrase: SecretString::from(passphrase.to_string()),
        });
        self
    }

    pub fn base_url(mut self, url: &str) -> Self {
        self.config.base_url_override = Some(url.to_string());
        self
    }

    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.config.request_timeout = timeout;
        self
    }

    pub fn build(self) -> ClientConfig {
        self.config
    }
}

impl Default for ClientConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}
