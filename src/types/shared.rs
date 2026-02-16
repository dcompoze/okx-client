use serde::{Deserialize, Serialize};

/// Pagination parameters for cursor-based pagination.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    /// Pagination of data to return records earlier than the requested ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Pagination of data to return records newer than the requested ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Number of results per request. Maximum 100; default 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Empty request parameters (for endpoints with no params).
#[derive(Debug, Clone, Default, Serialize)]
pub struct Empty {}
