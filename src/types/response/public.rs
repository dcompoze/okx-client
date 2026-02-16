use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Instrument {
    #[serde(default)]
    pub inst_type: String,
    #[serde(default)]
    pub inst_id: String,
    #[serde(default)]
    pub uly: String,
    #[serde(default)]
    pub inst_family: String,
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub base_ccy: String,
    #[serde(default)]
    pub quote_ccy: String,
    #[serde(default)]
    pub settle_ccy: String,
    #[serde(default)]
    pub ct_val: String,
    #[serde(default)]
    pub ct_mult: String,
    #[serde(default)]
    pub ct_val_ccy: String,
    #[serde(default)]
    pub opt_type: String,
    #[serde(default)]
    pub stk: String,
    #[serde(default)]
    pub list_time: String,
    #[serde(default)]
    pub exp_time: String,
    #[serde(default)]
    pub lever: String,
    #[serde(default)]
    pub tick_sz: String,
    #[serde(default)]
    pub lot_sz: String,
    #[serde(default)]
    pub min_sz: String,
    #[serde(default)]
    pub ct_type: String,
    #[serde(default)]
    pub alias: String,
    #[serde(default)]
    pub state: String,
    #[serde(default)]
    pub max_lmt_sz: String,
    #[serde(default)]
    pub max_mkt_sz: String,
    #[serde(default)]
    pub max_twap_sz: String,
    #[serde(default)]
    pub max_iceberg_sz: String,
    #[serde(default)]
    pub max_trigger_sz: String,
    #[serde(default)]
    pub max_stop_sz: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct FundingRate {
    #[serde(default)]
    pub inst_type: String,
    #[serde(default)]
    pub inst_id: String,
    #[serde(default)]
    pub funding_rate: String,
    #[serde(default)]
    pub realized_rate: String,
    #[serde(default)]
    pub funding_time: String,
    #[serde(default)]
    pub next_funding_rate: String,
    #[serde(default)]
    pub next_funding_time: String,
    #[serde(default)]
    pub min_funding_rate: String,
    #[serde(default)]
    pub max_funding_rate: String,
    #[serde(default)]
    pub method: String,
    #[serde(default)]
    pub premium: String,
    #[serde(default)]
    pub settle_state: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct MarkPrice {
    #[serde(default)]
    pub inst_type: String,
    #[serde(default)]
    pub inst_id: String,
    #[serde(default)]
    pub mark_px: String,
    #[serde(default)]
    pub ts: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct OpenInterest {
    #[serde(default)]
    pub inst_type: String,
    #[serde(default)]
    pub inst_id: String,
    #[serde(default)]
    pub oi: String,
    #[serde(default)]
    pub oi_ccy: String,
    #[serde(default)]
    pub ts: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ServerTime {
    #[serde(default)]
    pub ts: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct PositionTier {
    #[serde(default)]
    pub uly: String,
    #[serde(default)]
    pub inst_id: String,
    #[serde(default)]
    pub tier: String,
    #[serde(default)]
    pub min_sz: String,
    #[serde(default)]
    pub max_sz: String,
    #[serde(default)]
    pub mmr: String,
    #[serde(default)]
    pub imr: String,
    #[serde(default)]
    pub max_lever: String,
    #[serde(default)]
    pub opt_mrgn_factor: String,
    #[serde(default)]
    pub quote_max_loan: String,
    #[serde(default)]
    pub base_max_loan: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct InsuranceFund {
    #[serde(default)]
    pub total: String,
    #[serde(default)]
    pub details: Vec<InsuranceFundDetail>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct InsuranceFundDetail {
    #[serde(default)]
    pub amt: String,
    #[serde(default)]
    pub ccy: String,
    #[serde(default, rename = "type")]
    pub type_: String,
    #[serde(default)]
    pub ts: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct UnitConvertResult {
    #[serde(default, rename = "type")]
    pub type_: String,
    #[serde(default)]
    pub inst_id: String,
    #[serde(default)]
    pub px: String,
    #[serde(default)]
    pub sz: String,
    #[serde(default)]
    pub unit: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct DeliveryExerciseHistory {
    #[serde(default)]
    pub ts: String,
    #[serde(default)]
    pub details: Vec<DeliveryDetail>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct DeliveryDetail {
    #[serde(default, rename = "type")]
    pub type_: String,
    #[serde(default)]
    pub inst_id: String,
    #[serde(default)]
    pub px: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct DiscountRate {
    #[serde(default)]
    pub ccy: String,
    #[serde(default)]
    pub amt: String,
    #[serde(default)]
    pub discount_lv: String,
    #[serde(default)]
    pub discount_info: Vec<DiscountInfo>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct DiscountInfo {
    #[serde(default)]
    pub discount_rate: String,
    #[serde(default)]
    pub max_amt: String,
    #[serde(default)]
    pub min_amt: String,
}
