use crate::types::ws::channels::WsSubscriptionArg;
use crate::types::ws::events::WsConnectionType;

/// Route a subscription to the correct connection type (public, private, or business).
pub fn route_subscription(arg: &WsSubscriptionArg) -> WsConnectionType {
    if arg.is_private() {
        return WsConnectionType::Private;
    }
    if arg.is_business() {
        return WsConnectionType::Business;
    }
    WsConnectionType::Public
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_route_public() {
        let arg = WsSubscriptionArg::with_inst_id("tickers", "BTC-USDT");
        assert_eq!(route_subscription(&arg), WsConnectionType::Public);
    }

    #[test]
    fn test_route_private() {
        let arg = WsSubscriptionArg::channel_only("orders");
        assert_eq!(route_subscription(&arg), WsConnectionType::Private);

        let arg = WsSubscriptionArg::channel_only("account");
        assert_eq!(route_subscription(&arg), WsConnectionType::Private);
    }

    #[test]
    fn test_route_business() {
        let arg = WsSubscriptionArg::channel_only("candle1m");
        assert_eq!(route_subscription(&arg), WsConnectionType::Business);

        let arg = WsSubscriptionArg::channel_only("deposit-info");
        assert_eq!(route_subscription(&arg), WsConnectionType::Business);
    }
}
