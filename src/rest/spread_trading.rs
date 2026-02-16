use crate::error::OkxResult;
use crate::rest::RestClient;

impl RestClient {

    /// Place a spread order.
    /// POST /api/v5/sprd/order
    pub async fn place_spread_order(
        &self,
        params: &serde_json::Value,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.post_signed("/api/v5/sprd/order", params).await
    }

    /// Cancel a spread order.
    /// POST /api/v5/sprd/cancel-order
    pub async fn cancel_spread_order(
        &self,
        params: &serde_json::Value,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.post_signed("/api/v5/sprd/cancel-order", params).await
    }

    /// Cancel all spread orders.
    /// POST /api/v5/sprd/mass-cancel
    pub async fn cancel_all_spread_orders(
        &self,
        params: &serde_json::Value,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.post_signed("/api/v5/sprd/mass-cancel", params).await
    }

    /// Get spread order details.
    /// GET /api/v5/sprd/order
    pub async fn get_spread_order(
        &self,
        params: &serde_json::Value,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.get_signed("/api/v5/sprd/order", Some(params)).await
    }

    /// Get spread order list (active).
    /// GET /api/v5/sprd/orders-pending
    pub async fn get_spread_order_list(
        &self,
        params: &serde_json::Value,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.get_signed("/api/v5/sprd/orders-pending", Some(params))
            .await
    }

    /// Get spread order history.
    /// GET /api/v5/sprd/orders-history
    pub async fn get_spread_order_history(
        &self,
        params: &serde_json::Value,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.get_signed("/api/v5/sprd/orders-history", Some(params))
            .await
    }

    /// Get spread trades.
    /// GET /api/v5/sprd/trades
    pub async fn get_spread_trades(
        &self,
        params: &serde_json::Value,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.get_signed("/api/v5/sprd/trades", Some(params)).await
    }

    /// Get spreads.
    /// GET /api/v5/sprd/spreads
    pub async fn get_spreads(
        &self,
        params: &serde_json::Value,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.get("/api/v5/sprd/spreads", Some(params)).await
    }

    /// Get spread order book.
    /// GET /api/v5/sprd/books
    pub async fn get_spread_order_book(
        &self,
        params: &serde_json::Value,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.get("/api/v5/sprd/books", Some(params)).await
    }

    /// Get spread ticker.
    /// GET /api/v5/sprd/ticker
    pub async fn get_spread_ticker(
        &self,
        params: &serde_json::Value,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.get("/api/v5/sprd/ticker", Some(params)).await
    }

    /// Get spread public trades.
    /// GET /api/v5/sprd/public-trades
    pub async fn get_spread_public_trades(
        &self,
        params: &serde_json::Value,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.get("/api/v5/sprd/public-trades", Some(params)).await
    }
}
