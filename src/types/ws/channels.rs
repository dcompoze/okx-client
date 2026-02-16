use serde::{Deserialize, Serialize};

/// WebSocket channel names.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum WsChannel {
    // Public channels
    Instruments,
    Tickers,
    OpenInterest,
    Trades,
    EstimatedPrice,
    MarkPrice,
    PriceLimit,
    OptSummary,
    FundingRate,
    IndexTickers,
    Status,
    LiquidationOrders,

    // Order book channels
    Books,
    Books5,
    #[serde(rename = "bbo-tbt")]
    BboTbt,
    #[serde(rename = "books-l2-tbt")]
    BooksL2Tbt,
    #[serde(rename = "books50-l2-tpt")]
    Books50L2Tpt,

    // Private channels
    Account,
    Positions,
    BalanceAndPosition,
    Orders,
    OrdersAlgo,
    AlgoAdvance,
    LiquidationWarning,
    AccountGreeks,

    // Grid channels (private)
    GridOrdersSpot,
    GridOrdersContract,
    GridOrdersMoon,
    GridPositions,
    GridSubOrders,

    // Candle channels (use string for dynamic bar sizes)
    #[serde(untagged)]
    Candle(String),
}

/// Subscription argument sent to OKX WebSocket.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsSubscriptionArg {
    pub channel: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algo_id: Option<String>,
}

impl WsSubscriptionArg {
    /// Create a simple subscription arg with just a channel and instrument ID.
    pub fn with_inst_id(channel: &str, inst_id: &str) -> Self {
        Self {
            channel: channel.to_string(),
            inst_id: Some(inst_id.to_string()),
            inst_type: None,
            inst_family: None,
            ccy: None,
            uid: None,
            algo_id: None,
        }
    }

    /// Create a subscription arg with channel and instrument type.
    pub fn with_inst_type(channel: &str, inst_type: &str) -> Self {
        Self {
            channel: channel.to_string(),
            inst_type: Some(inst_type.to_string()),
            inst_id: None,
            inst_family: None,
            ccy: None,
            uid: None,
            algo_id: None,
        }
    }

    /// Create a subscription arg with just a channel name.
    pub fn channel_only(channel: &str) -> Self {
        Self {
            channel: channel.to_string(),
            inst_type: None,
            inst_id: None,
            inst_family: None,
            ccy: None,
            uid: None,
            algo_id: None,
        }
    }

    /// Determine if this is a private channel subscription.
    pub fn is_private(&self) -> bool {
        matches!(
            self.channel.as_str(),
            "account"
                | "positions"
                | "balance_and_position"
                | "orders"
                | "orders-algo"
                | "algo-advance"
                | "liquidation-warning"
                | "account-greeks"
                | "grid-orders-spot"
                | "grid-orders-contract"
                | "grid-orders-moon"
                | "grid-positions"
                | "grid-sub-orders"
        )
    }

    /// Determine if this is a business channel subscription.
    pub fn is_business(&self) -> bool {
        let ch = self.channel.as_str();
        ch.starts_with("candle")
            || ch.starts_with("mark-price-candle")
            || ch.starts_with("index-candle")
            || matches!(
                ch,
                "deposit-info" | "withdrawal-info" | "grid-orders-spot" | "grid-orders-contract"
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_inst_id() {
        let arg = WsSubscriptionArg::with_inst_id("tickers", "BTC-USDT");
        assert_eq!(arg.channel, "tickers");
        assert_eq!(arg.inst_id.as_deref(), Some("BTC-USDT"));
        assert!(arg.inst_type.is_none());
    }

    #[test]
    fn test_with_inst_type() {
        let arg = WsSubscriptionArg::with_inst_type("tickers", "SPOT");
        assert_eq!(arg.channel, "tickers");
        assert_eq!(arg.inst_type.as_deref(), Some("SPOT"));
        assert!(arg.inst_id.is_none());
    }

    #[test]
    fn test_channel_only() {
        let arg = WsSubscriptionArg::channel_only("account");
        assert_eq!(arg.channel, "account");
        assert!(arg.inst_id.is_none());
        assert!(arg.inst_type.is_none());
    }

    #[test]
    fn test_is_private() {
        assert!(WsSubscriptionArg::channel_only("account").is_private());
        assert!(WsSubscriptionArg::channel_only("positions").is_private());
        assert!(WsSubscriptionArg::channel_only("orders").is_private());
        assert!(!WsSubscriptionArg::channel_only("tickers").is_private());
        assert!(!WsSubscriptionArg::channel_only("trades").is_private());
    }

    #[test]
    fn test_is_business() {
        assert!(WsSubscriptionArg::channel_only("candle1m").is_business());
        assert!(WsSubscriptionArg::channel_only("candle5m").is_business());
        assert!(WsSubscriptionArg::channel_only("mark-price-candle1H").is_business());
        assert!(WsSubscriptionArg::channel_only("index-candle1D").is_business());
        assert!(WsSubscriptionArg::channel_only("deposit-info").is_business());
        assert!(!WsSubscriptionArg::channel_only("tickers").is_business());
        assert!(!WsSubscriptionArg::channel_only("account").is_business());
    }

    #[test]
    fn test_serialize_subscription_arg() {
        let arg = WsSubscriptionArg::with_inst_id("tickers", "BTC-USDT");
        let json = serde_json::to_string(&arg).unwrap();
        assert!(json.contains("\"channel\":\"tickers\""));
        assert!(json.contains("\"instId\":\"BTC-USDT\""));
        // `None` fields should not appear.
        assert!(!json.contains("instType"));
    }

    #[test]
    fn test_deserialize_subscription_arg() {
        let json = r#"{"channel":"tickers","instId":"BTC-USDT"}"#;
        let arg: WsSubscriptionArg = serde_json::from_str(json).unwrap();
        assert_eq!(arg.channel, "tickers");
        assert_eq!(arg.inst_id.as_deref(), Some("BTC-USDT"));
    }
}
