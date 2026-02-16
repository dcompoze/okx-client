use serde::Serialize;

/// Estimate quote for conversion.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EstimateQuoteRequest {
    pub base_ccy: String,
    pub quote_ccy: String,
    pub side: String,
    pub rfq_sz: String,
    pub rfq_sz_ccy: String,
}

/// Execute a conversion trade.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ConvertTradeRequest {
    pub quote_id: String,
    pub base_ccy: String,
    pub quote_ccy: String,
    pub side: String,
    pub sz: String,
    pub sz_ccy: String,
}

/// Get convert history.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetConvertHistoryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}
