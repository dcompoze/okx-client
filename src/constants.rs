/// OKX program ID tag, auto-injected into private POST requests.
pub const PROGRAM_ID: &str = "159881cb7207BCDE";

/// HTTP header names used by OKX API.
pub const HEADER_ACCESS_KEY: &str = "OK-ACCESS-KEY";
pub const HEADER_ACCESS_SIGN: &str = "OK-ACCESS-SIGN";
pub const HEADER_ACCESS_TIMESTAMP: &str = "OK-ACCESS-TIMESTAMP";
pub const HEADER_ACCESS_PASSPHRASE: &str = "OK-ACCESS-PASSPHRASE";
pub const HEADER_SIMULATED_TRADING: &str = "x-simulated-trading";

/// REST API base URLs by region.
pub mod rest_urls {
    pub const GLOBAL: &str = "https://www.okx.com";
    pub const EEA: &str = "https://eea.okx.com";
    pub const US: &str = "https://us.okx.com";
}

/// WebSocket URLs by region and connection type.
pub mod ws_urls {
    // Production
    pub const GLOBAL_PUBLIC: &str = "wss://ws.okx.com:8443/ws/v5/public";
    pub const GLOBAL_PRIVATE: &str = "wss://ws.okx.com:8443/ws/v5/private";
    pub const GLOBAL_BUSINESS: &str = "wss://ws.okx.com:8443/ws/v5/business";

    pub const EEA_PUBLIC: &str = "wss://wseea.okx.com:8443/ws/v5/public";
    pub const EEA_PRIVATE: &str = "wss://wseea.okx.com:8443/ws/v5/private";
    pub const EEA_BUSINESS: &str = "wss://wseea.okx.com:8443/ws/v5/business";

    pub const US_PUBLIC: &str = "wss://wsus.okx.com:8443/ws/v5/public";
    pub const US_PRIVATE: &str = "wss://wsus.okx.com:8443/ws/v5/private";
    pub const US_BUSINESS: &str = "wss://wsus.okx.com:8443/ws/v5/business";

    // Demo trading
    pub const DEMO_PUBLIC: &str = "wss://wspap.okx.com:8443/ws/v5/public?brokerId=9999";
    pub const DEMO_PRIVATE: &str = "wss://wspap.okx.com:8443/ws/v5/private?brokerId=9999";
    pub const DEMO_BUSINESS: &str = "wss://wspap.okx.com:8443/ws/v5/business?brokerId=9999";
}
