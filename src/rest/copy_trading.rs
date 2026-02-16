use crate::error::OkxResult;
use crate::rest::RestClient;

impl RestClient {
    // ──────────────────── Copy Trading ────────────────────

    /// Get existing leading positions.
    /// GET /api/v5/copytrading/current-subpositions
    pub async fn get_copy_trading_positions(
        &self,
        params: &serde_json::Value,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.get_signed("/api/v5/copytrading/current-subpositions", Some(params))
            .await
    }

    /// Get leading position history.
    /// GET /api/v5/copytrading/subpositions-history
    pub async fn get_copy_trading_positions_history(
        &self,
        params: &serde_json::Value,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.get_signed("/api/v5/copytrading/subpositions-history", Some(params))
            .await
    }

    /// Close a leading position.
    /// POST /api/v5/copytrading/close-subposition
    pub async fn close_copy_trading_position(
        &self,
        params: &serde_json::Value,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.post_signed("/api/v5/copytrading/close-subposition", params)
            .await
    }

    /// Get lead instruments.
    /// GET /api/v5/copytrading/instruments
    pub async fn get_copy_trading_instruments(
        &self,
        params: &serde_json::Value,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.get_signed("/api/v5/copytrading/instruments", Some(params))
            .await
    }

    /// Set lead instruments.
    /// POST /api/v5/copytrading/set-instruments
    pub async fn set_copy_trading_instruments(
        &self,
        params: &serde_json::Value,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.post_signed("/api/v5/copytrading/set-instruments", params)
            .await
    }

    /// Get profit sharing details.
    /// GET /api/v5/copytrading/profit-sharing-details
    pub async fn get_copy_trading_profit_sharing(
        &self,
        params: &serde_json::Value,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.get_signed("/api/v5/copytrading/profit-sharing-details", Some(params))
            .await
    }

    /// Get total profit sharing.
    /// GET /api/v5/copytrading/total-profit-sharing
    pub async fn get_copy_trading_total_profit(
        &self,
        params: &serde_json::Value,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.get_signed("/api/v5/copytrading/total-profit-sharing", Some(params))
            .await
    }
}
