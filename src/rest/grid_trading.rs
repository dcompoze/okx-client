use crate::error::OkxResult;
use crate::rest::RestClient;

impl RestClient {

    /// Place a grid algo order (spot grid, contract grid, moon grid).
    /// POST /api/v5/tradingBot/grid/order-algo
    pub async fn place_grid_algo_order(
        &self,
        params: &serde_json::Value,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.post_signed("/api/v5/tradingBot/grid/order-algo", params)
            .await
    }

    /// Amend a grid algo order.
    /// POST /api/v5/tradingBot/grid/amend-order-algo
    pub async fn amend_grid_algo_order(
        &self,
        params: &serde_json::Value,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.post_signed("/api/v5/tradingBot/grid/amend-order-algo", params)
            .await
    }

    /// Stop a grid algo order.
    /// POST /api/v5/tradingBot/grid/stop-order-algo
    pub async fn stop_grid_algo_order(
        &self,
        params: &serde_json::Value,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.post_signed("/api/v5/tradingBot/grid/stop-order-algo", params)
            .await
    }

    /// Get grid algo order list.
    /// GET /api/v5/tradingBot/grid/orders-algo-pending
    pub async fn get_grid_algo_order_list(
        &self,
        params: &serde_json::Value,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.get_signed("/api/v5/tradingBot/grid/orders-algo-pending", Some(params))
            .await
    }

    /// Get grid algo order history.
    /// GET /api/v5/tradingBot/grid/orders-algo-history
    pub async fn get_grid_algo_order_history(
        &self,
        params: &serde_json::Value,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.get_signed("/api/v5/tradingBot/grid/orders-algo-history", Some(params))
            .await
    }

    /// Get grid algo order details.
    /// GET /api/v5/tradingBot/grid/orders-algo-details
    pub async fn get_grid_algo_order_details(
        &self,
        params: &serde_json::Value,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.get_signed("/api/v5/tradingBot/grid/orders-algo-details", Some(params))
            .await
    }

    /// Get grid algo sub orders.
    /// GET /api/v5/tradingBot/grid/sub-orders
    pub async fn get_grid_sub_orders(
        &self,
        params: &serde_json::Value,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.get_signed("/api/v5/tradingBot/grid/sub-orders", Some(params))
            .await
    }

    /// Get grid algo order positions.
    /// GET /api/v5/tradingBot/grid/positions
    pub async fn get_grid_positions(
        &self,
        params: &serde_json::Value,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.get_signed("/api/v5/tradingBot/grid/positions", Some(params))
            .await
    }
}
