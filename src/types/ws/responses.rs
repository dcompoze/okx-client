use serde::Deserialize;

/// Result from placing a spread order via WS API.
/// Operation: `sprd-order`
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct WsSpreadOrderResult {
    /// Spread ID.
    pub sprd_id: String,
    /// Order ID assigned by OKX.
    pub ord_id: String,
    /// Client Order ID.
    pub cl_ord_id: String,
    /// Order tag.
    pub tag: String,
    /// Timestamp when the request was received, Unix ms.
    pub ts: String,
    /// Per-item result code; "0" means success.
    pub s_code: String,
    /// Per-item result message.
    pub s_msg: String,
}

/// Result from cancelling a spread order via WS API.
/// Operation: `sprd-cancel-order`
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct WsSpreadCancelResult {
    /// Spread ID.
    pub sprd_id: String,
    /// Order ID.
    pub ord_id: String,
    /// Client Order ID.
    pub cl_ord_id: String,
    /// Per-item result code; "0" means success.
    pub s_code: String,
    /// Per-item result message.
    pub s_msg: String,
}

/// Result from amending a spread order via WS API.
/// Operation: `sprd-amend-order`
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct WsSpreadAmendResult {
    /// Spread ID.
    pub sprd_id: String,
    /// Order ID.
    pub ord_id: String,
    /// Client Order ID.
    pub cl_ord_id: String,
    /// Client Request ID provided by the client.
    pub req_id: String,
    /// Per-item result code; "0" means success.
    pub s_code: String,
    /// Per-item result message.
    pub s_msg: String,
}
