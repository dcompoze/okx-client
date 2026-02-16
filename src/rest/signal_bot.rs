use crate::error::OkxResult;
use crate::rest::RestClient;

impl RestClient {

    /// Create a signal.
    /// POST /api/v5/tradingBot/signal/create-signal
    pub async fn create_signal(
        &self,
        params: &serde_json::Value,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.post_signed("/api/v5/tradingBot/signal/create-signal", params)
            .await
    }

    /// Get signals.
    /// GET /api/v5/tradingBot/signal/signals
    pub async fn get_signals(
        &self,
        params: &serde_json::Value,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.get_signed("/api/v5/tradingBot/signal/signals", Some(params))
            .await
    }

    /// Create a signal bot order.
    /// POST /api/v5/tradingBot/signal/order-algo
    pub async fn create_signal_bot(
        &self,
        params: &serde_json::Value,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.post_signed("/api/v5/tradingBot/signal/order-algo", params)
            .await
    }

    /// Stop a signal bot order.
    /// POST /api/v5/tradingBot/signal/stop-order-algo
    pub async fn stop_signal_bot(
        &self,
        params: &serde_json::Value,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.post_signed("/api/v5/tradingBot/signal/stop-order-algo", params)
            .await
    }

    /// Get signal bot order list.
    /// GET /api/v5/tradingBot/signal/orders-algo-pending
    pub async fn get_signal_bot_order_list(
        &self,
        params: &serde_json::Value,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.get_signed(
            "/api/v5/tradingBot/signal/orders-algo-pending",
            Some(params),
        )
        .await
    }

    /// Get signal bot order history.
    /// GET /api/v5/tradingBot/signal/orders-algo-history
    pub async fn get_signal_bot_order_history(
        &self,
        params: &serde_json::Value,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.get_signed(
            "/api/v5/tradingBot/signal/orders-algo-history",
            Some(params),
        )
        .await
    }

    /// Get signal bot sub-orders.
    /// GET /api/v5/tradingBot/signal/sub-orders
    pub async fn get_signal_bot_sub_orders(
        &self,
        params: &serde_json::Value,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.get_signed("/api/v5/tradingBot/signal/sub-orders", Some(params))
            .await
    }
}
