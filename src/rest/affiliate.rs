use crate::error::OkxResult;
use crate::rest::RestClient;

impl RestClient {

    /// Get affiliate invitee detail.
    /// GET /api/v5/affiliate/invitee/detail
    pub async fn get_affiliate_invitee_detail(
        &self,
        params: &serde_json::Value,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.get_signed("/api/v5/affiliate/invitee/detail", Some(params))
            .await
    }
}
