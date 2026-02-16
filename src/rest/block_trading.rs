use crate::error::OkxResult;
use crate::rest::RestClient;

impl RestClient {

    /// Create an RFQ.
    /// POST /api/v5/rfq/create-rfq
    pub async fn create_rfq(
        &self,
        params: &serde_json::Value,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.post_signed("/api/v5/rfq/create-rfq", params).await
    }

    /// Cancel an RFQ.
    /// POST /api/v5/rfq/cancel-rfq
    pub async fn cancel_rfq(
        &self,
        params: &serde_json::Value,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.post_signed("/api/v5/rfq/cancel-rfq", params).await
    }

    /// Cancel multiple RFQs.
    /// POST /api/v5/rfq/cancel-batch-rfqs
    pub async fn cancel_batch_rfqs(
        &self,
        params: &serde_json::Value,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.post_signed("/api/v5/rfq/cancel-batch-rfqs", params)
            .await
    }

    /// Cancel all RFQs.
    /// POST /api/v5/rfq/cancel-all-rfqs
    pub async fn cancel_all_rfqs(&self) -> OkxResult<Vec<serde_json::Value>> {
        self.post_signed("/api/v5/rfq/cancel-all-rfqs", &serde_json::json!({}))
            .await
    }

    /// Execute a quote.
    /// POST /api/v5/rfq/execute-quote
    pub async fn execute_quote(
        &self,
        params: &serde_json::Value,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.post_signed("/api/v5/rfq/execute-quote", params).await
    }

    /// Create a quote.
    /// POST /api/v5/rfq/create-quote
    pub async fn create_quote(
        &self,
        params: &serde_json::Value,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.post_signed("/api/v5/rfq/create-quote", params).await
    }

    /// Cancel a quote.
    /// POST /api/v5/rfq/cancel-quote
    pub async fn cancel_quote(
        &self,
        params: &serde_json::Value,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.post_signed("/api/v5/rfq/cancel-quote", params).await
    }

    /// Cancel all quotes.
    /// POST /api/v5/rfq/cancel-all-quotes
    pub async fn cancel_all_quotes(&self) -> OkxResult<Vec<serde_json::Value>> {
        self.post_signed("/api/v5/rfq/cancel-all-quotes", &serde_json::json!({}))
            .await
    }

    /// Get RFQs.
    /// GET /api/v5/rfq/rfqs
    pub async fn get_rfqs(
        &self,
        params: &serde_json::Value,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.get_signed("/api/v5/rfq/rfqs", Some(params)).await
    }

    /// Get quotes.
    /// GET /api/v5/rfq/quotes
    pub async fn get_quotes(
        &self,
        params: &serde_json::Value,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.get_signed("/api/v5/rfq/quotes", Some(params)).await
    }

    /// Get block trades.
    /// GET /api/v5/rfq/trades
    pub async fn get_block_trades(
        &self,
        params: &serde_json::Value,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.get_signed("/api/v5/rfq/trades", Some(params)).await
    }

    /// Get public block trades.
    /// GET /api/v5/rfq/public-trades
    pub async fn get_public_block_trades(
        &self,
        params: &serde_json::Value,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.get("/api/v5/rfq/public-trades", Some(params)).await
    }
}
