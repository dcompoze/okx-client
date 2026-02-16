use crate::error::OkxResult;
use crate::rest::RestClient;

impl RestClient {

    /// Get system status.
    /// GET /api/v5/system/status
    pub async fn get_system_status(&self) -> OkxResult<Vec<serde_json::Value>> {
        self.get::<serde_json::Value, ()>("/api/v5/system/status", None)
            .await
    }
}
