use crate::error::OkxResult;
use crate::rest::RestClient;

impl RestClient {
    // ──────────────────── Trading Data ────────────────────

    /// Get support coin.
    /// GET /api/v5/rubik/stat/trading-data/support-coin
    pub async fn get_support_coin(&self) -> OkxResult<Vec<serde_json::Value>> {
        self.get::<serde_json::Value, ()>(
            "/api/v5/rubik/stat/trading-data/support-coin",
            None,
        )
        .await
    }

    /// Get taker volume.
    /// GET /api/v5/rubik/stat/taker-volume
    pub async fn get_taker_volume(
        &self,
        params: &serde_json::Value,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.get("/api/v5/rubik/stat/taker-volume", Some(params))
            .await
    }

    /// Get margin lending ratio.
    /// GET /api/v5/rubik/stat/margin/loan-ratio
    pub async fn get_margin_lending_ratio(
        &self,
        params: &serde_json::Value,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.get("/api/v5/rubik/stat/margin/loan-ratio", Some(params))
            .await
    }

    /// Get long/short ratio.
    /// GET /api/v5/rubik/stat/contracts/long-short-account-ratio
    pub async fn get_long_short_ratio(
        &self,
        params: &serde_json::Value,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.get(
            "/api/v5/rubik/stat/contracts/long-short-account-ratio",
            Some(params),
        )
        .await
    }

    /// Get open interest and volume.
    /// GET /api/v5/rubik/stat/contracts/open-interest-volume
    pub async fn get_open_interest_volume(
        &self,
        params: &serde_json::Value,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.get(
            "/api/v5/rubik/stat/contracts/open-interest-volume",
            Some(params),
        )
        .await
    }

    /// Get put/call ratio.
    /// GET /api/v5/rubik/stat/option/open-interest-volume-ratio
    pub async fn get_put_call_ratio(
        &self,
        params: &serde_json::Value,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.get(
            "/api/v5/rubik/stat/option/open-interest-volume-ratio",
            Some(params),
        )
        .await
    }

    /// Get open interest and volume (options).
    /// GET /api/v5/rubik/stat/option/open-interest-volume
    pub async fn get_option_open_interest_volume(
        &self,
        params: &serde_json::Value,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.get(
            "/api/v5/rubik/stat/option/open-interest-volume",
            Some(params),
        )
        .await
    }

    /// Get taker volume (contracts).
    /// GET /api/v5/rubik/stat/taker-volume-contract
    pub async fn get_taker_volume_contracts(
        &self,
        params: &serde_json::Value,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.get(
            "/api/v5/rubik/stat/taker-volume-contract",
            Some(params),
        )
        .await
    }
}
