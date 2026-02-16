use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Ticker {
    #[serde(default)]
    pub inst_type: String,
    #[serde(default)]
    pub inst_id: String,
    #[serde(default)]
    pub last: String,
    #[serde(default)]
    pub last_sz: String,
    #[serde(default)]
    pub ask_px: String,
    #[serde(default)]
    pub ask_sz: String,
    #[serde(default)]
    pub bid_px: String,
    #[serde(default)]
    pub bid_sz: String,
    #[serde(default)]
    pub open24h: String,
    #[serde(default)]
    pub high24h: String,
    #[serde(default)]
    pub low24h: String,
    #[serde(default)]
    pub vol_ccy24h: String,
    #[serde(default)]
    pub vol24h: String,
    #[serde(default)]
    pub ts: String,
    #[serde(default)]
    pub sod_utc0: String,
    #[serde(default)]
    pub sod_utc8: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct OrderBook {
    #[serde(default)]
    pub asks: Vec<Vec<String>>,
    #[serde(default)]
    pub bids: Vec<Vec<String>>,
    #[serde(default)]
    pub ts: String,
}

pub type Candle = Vec<String>;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Trade {
    #[serde(default)]
    pub inst_id: String,
    #[serde(default)]
    pub trade_id: String,
    #[serde(default)]
    pub px: String,
    #[serde(default)]
    pub sz: String,
    #[serde(default)]
    pub side: String,
    #[serde(default)]
    pub ts: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct PlatformVolume {
    #[serde(default)]
    pub vol_usd: String,
    #[serde(default)]
    pub vol_cny: String,
    #[serde(default)]
    pub ts: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct IndexTicker {
    #[serde(default)]
    pub inst_id: String,
    #[serde(default)]
    pub idx_px: String,
    #[serde(default)]
    pub high24h: String,
    #[serde(default)]
    pub sod_utc0: String,
    #[serde(default)]
    pub open24h: String,
    #[serde(default)]
    pub low24h: String,
    #[serde(default)]
    pub ts: String,
}
