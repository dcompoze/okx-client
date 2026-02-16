use crate::error::OkxResult;
use crate::rest::RestClient;
use crate::types::request::trade::*;
use crate::types::response::trade::*;

impl RestClient {
    // ──────────────────── Orders ────────────────────

    /// Place a single order.
    /// POST /api/v5/trade/order
    pub async fn place_order(&self, params: &OrderRequest) -> OkxResult<Vec<OrderResult>> {
        self.post_signed("/api/v5/trade/order", params).await
    }

    /// Place multiple orders (up to 20) in a single request.
    /// POST /api/v5/trade/batch-orders
    pub async fn place_multiple_orders(
        &self,
        params: &Vec<OrderRequest>,
    ) -> OkxResult<Vec<OrderResult>> {
        self.post_signed("/api/v5/trade/batch-orders", params).await
    }

    /// Cancel a single order.
    /// POST /api/v5/trade/cancel-order
    pub async fn cancel_order(
        &self,
        params: &CancelOrderRequest,
    ) -> OkxResult<Vec<CancelledOrder>> {
        self.post_signed("/api/v5/trade/cancel-order", params).await
    }

    /// Cancel multiple orders (up to 20) in a single request.
    /// POST /api/v5/trade/cancel-batch-orders
    pub async fn cancel_multiple_orders(
        &self,
        params: &Vec<CancelOrderRequest>,
    ) -> OkxResult<Vec<CancelledOrder>> {
        self.post_signed("/api/v5/trade/cancel-batch-orders", params)
            .await
    }

    /// Amend an existing order.
    /// POST /api/v5/trade/amend-order
    pub async fn amend_order(&self, params: &AmendOrderRequest) -> OkxResult<Vec<AmendedOrder>> {
        self.post_signed("/api/v5/trade/amend-order", params).await
    }

    /// Amend multiple orders (up to 20) in a single request.
    /// POST /api/v5/trade/amend-batch-orders
    pub async fn amend_multiple_orders(
        &self,
        params: &Vec<AmendOrderRequest>,
    ) -> OkxResult<Vec<AmendedOrder>> {
        self.post_signed("/api/v5/trade/amend-batch-orders", params)
            .await
    }

    /// Close a position.
    /// POST /api/v5/trade/close-position
    pub async fn close_position(
        &self,
        params: &ClosePositionRequest,
    ) -> OkxResult<Vec<OrderResult>> {
        self.post_signed("/api/v5/trade/close-position", params)
            .await
    }

    /// Get details of a single order.
    /// GET /api/v5/trade/order
    pub async fn get_order(&self, params: &GetOrderRequest) -> OkxResult<Vec<OrderDetails>> {
        self.get_signed("/api/v5/trade/order", Some(params)).await
    }

    /// Get a list of pending (unfilled/partially filled) orders.
    /// GET /api/v5/trade/orders-pending
    pub async fn get_order_list(
        &self,
        params: &GetOrderListRequest,
    ) -> OkxResult<Vec<OrderDetails>> {
        self.get_signed("/api/v5/trade/orders-pending", Some(params))
            .await
    }

    /// Get order history for the last 7 days.
    /// GET /api/v5/trade/orders-history
    pub async fn get_order_history(
        &self,
        params: &GetOrderHistoryRequest,
    ) -> OkxResult<Vec<OrderDetails>> {
        self.get_signed("/api/v5/trade/orders-history", Some(params))
            .await
    }

    /// Get order history archive (last 3 months).
    /// GET /api/v5/trade/orders-history-archive
    pub async fn get_order_history_archive(
        &self,
        params: &GetOrderHistoryRequest,
    ) -> OkxResult<Vec<OrderDetails>> {
        self.get_signed("/api/v5/trade/orders-history-archive", Some(params))
            .await
    }

    /// Get recent transaction (fill) details for the last 3 days.
    /// GET /api/v5/trade/fills
    pub async fn get_fills(&self, params: &GetFillsRequest) -> OkxResult<Vec<Fill>> {
        self.get_signed("/api/v5/trade/fills", Some(params)).await
    }

    /// Get transaction (fill) details history for the last 3 months.
    /// GET /api/v5/trade/fills-history
    pub async fn get_fills_history(&self, params: &GetFillsRequest) -> OkxResult<Vec<Fill>> {
        self.get_signed("/api/v5/trade/fills-history", Some(params))
            .await
    }

    /// Mass cancel all pending orders for an instrument type.
    /// POST /api/v5/trade/mass-cancel
    pub async fn mass_cancel(&self, params: &MassCancelRequest) -> OkxResult<Vec<MassCancelResult>> {
        self.post_signed("/api/v5/trade/mass-cancel", params).await
    }

    /// Cancel all orders after a countdown timer (dead man's switch).
    /// POST /api/v5/trade/cancel-all-after
    pub async fn cancel_all_after(
        &self,
        params: &CancelAllAfterRequest,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.post_signed("/api/v5/trade/cancel-all-after", params)
            .await
    }

    // ──────────────────── Algo Orders ────────────────────

    /// Place an algo order (trigger, OCO, conditional, iceberg, TWAP, etc.).
    /// POST /api/v5/trade/order-algo
    pub async fn place_algo_order(
        &self,
        params: &AlgoOrderRequest,
    ) -> OkxResult<Vec<AlgoOrderResult>> {
        self.post_signed("/api/v5/trade/order-algo", params).await
    }

    /// Cancel algo orders.
    /// POST /api/v5/trade/cancel-algos
    pub async fn cancel_algo_orders(
        &self,
        params: &Vec<CancelAlgoOrderRequest>,
    ) -> OkxResult<Vec<AlgoOrderResult>> {
        self.post_signed("/api/v5/trade/cancel-algos", params).await
    }

    /// Amend an algo order.
    /// POST /api/v5/trade/amend-algos
    pub async fn amend_algo_order(
        &self,
        params: &AmendAlgoOrderRequest,
    ) -> OkxResult<Vec<AlgoOrderResult>> {
        self.post_signed("/api/v5/trade/amend-algos", params).await
    }

    /// Get details of a single algo order.
    /// GET /api/v5/trade/order-algo
    pub async fn get_algo_order(
        &self,
        params: &GetAlgoOrderRequest,
    ) -> OkxResult<Vec<AlgoOrderDetails>> {
        self.get_signed("/api/v5/trade/order-algo", Some(params))
            .await
    }

    /// Get a list of pending algo orders.
    /// GET /api/v5/trade/orders-algo-pending
    pub async fn get_algo_order_list(
        &self,
        params: &GetAlgoOrderListRequest,
    ) -> OkxResult<Vec<AlgoOrderDetails>> {
        self.get_signed("/api/v5/trade/orders-algo-pending", Some(params))
            .await
    }

    /// Get algo order history.
    /// GET /api/v5/trade/orders-algo-history
    pub async fn get_algo_order_history(
        &self,
        params: &GetAlgoOrderListRequest,
    ) -> OkxResult<Vec<AlgoOrderDetails>> {
        self.get_signed("/api/v5/trade/orders-algo-history", Some(params))
            .await
    }
}
