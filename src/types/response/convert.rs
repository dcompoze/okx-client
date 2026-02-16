use serde::Deserialize;

/// Convert currency info.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ConvertCurrency {
    #[serde(default)]
    pub ccy: String,
    #[serde(default)]
    pub min: String,
    #[serde(default)]
    pub max: String,
}

/// Estimated conversion quote.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ConvertQuote {
    #[serde(default)]
    pub quote_id: String,
    #[serde(default)]
    pub quote_time: String,
    #[serde(default)]
    pub ttl: String,
    #[serde(default)]
    pub cnv_px: String,
    #[serde(default)]
    pub base_ccy: String,
    #[serde(default)]
    pub base_sz: String,
    #[serde(default)]
    pub quote_ccy: String,
    #[serde(default)]
    pub quote_sz: String,
    #[serde(default)]
    pub side: String,
}

/// Conversion trade result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ConvertTradeResult {
    #[serde(default)]
    pub trade_id: String,
    #[serde(default)]
    pub quote_id: String,
    #[serde(default)]
    pub cnv_px: String,
    #[serde(default)]
    pub base_ccy: String,
    #[serde(default)]
    pub base_sz: String,
    #[serde(default)]
    pub quote_ccy: String,
    #[serde(default)]
    pub quote_sz: String,
    #[serde(default)]
    pub side: String,
    #[serde(default)]
    pub state: String,
    #[serde(default)]
    pub ts: String,
}
