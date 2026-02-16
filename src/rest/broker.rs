use crate::error::OkxResult;
use crate::rest::RestClient;

impl RestClient {

    /// Get broker account info.
    /// GET /api/v5/broker/nd/info
    pub async fn get_broker_info(&self) -> OkxResult<Vec<serde_json::Value>> {
        self.get_signed::<serde_json::Value, ()>("/api/v5/broker/nd/info", None)
            .await
    }

    /// Create a sub-account (broker).
    /// POST /api/v5/broker/nd/create-subaccount
    pub async fn broker_create_sub_account(
        &self,
        params: &serde_json::Value,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.post_signed("/api/v5/broker/nd/create-subaccount", params)
            .await
    }

    /// Create API key for a sub-account (broker).
    /// POST /api/v5/broker/nd/subaccount/apikey
    pub async fn broker_create_sub_account_api_key(
        &self,
        params: &serde_json::Value,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.post_signed("/api/v5/broker/nd/subaccount/apikey", params)
            .await
    }

    /// Get sub-account deposit history (broker).
    /// GET /api/v5/broker/nd/subaccount-deposit-history
    pub async fn broker_get_sub_account_deposit_history(
        &self,
        params: &serde_json::Value,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.get_signed(
            "/api/v5/broker/nd/subaccount-deposit-history",
            Some(params),
        )
        .await
    }

    /// Get rebate daily (broker).
    /// GET /api/v5/broker/nd/rebate-daily
    pub async fn broker_get_rebate_daily(
        &self,
        params: &serde_json::Value,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.get_signed("/api/v5/broker/nd/rebate-daily", Some(params))
            .await
    }
}
