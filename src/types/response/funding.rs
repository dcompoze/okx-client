use serde::Deserialize;

/// Currency information.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Currency {
    #[serde(default)]
    pub ccy: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub logo_link: String,
    #[serde(default)]
    pub chain: String,
    #[serde(default)]
    pub can_dep: bool,
    #[serde(default)]
    pub can_wd: bool,
    #[serde(default)]
    pub can_internal: bool,
    #[serde(default)]
    pub min_dep: String,
    #[serde(default)]
    pub min_wd: String,
    #[serde(default)]
    pub max_wd: String,
    #[serde(default)]
    pub wd_tick_sz: String,
    #[serde(default)]
    pub min_fee: String,
    #[serde(default)]
    pub max_fee: String,
}

/// Asset balance.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AssetBalance {
    #[serde(default)]
    pub ccy: String,
    #[serde(default)]
    pub bal: String,
    #[serde(default)]
    pub frozen_bal: String,
    #[serde(default)]
    pub avail_bal: String,
}

/// Withdrawal result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct WithdrawalResult {
    #[serde(default)]
    pub ccy: String,
    #[serde(default)]
    pub chain: String,
    #[serde(default)]
    pub amt: String,
    #[serde(default)]
    pub wd_id: String,
    #[serde(default)]
    pub client_id: String,
}

/// Transfer result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct TransferResult {
    #[serde(default)]
    pub trans_id: String,
    #[serde(default)]
    pub ccy: String,
    #[serde(default)]
    pub from: String,
    #[serde(default)]
    pub to: String,
    #[serde(default)]
    pub amt: String,
    #[serde(default)]
    pub client_id: String,
}

/// Deposit record.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct DepositRecord {
    #[serde(default)]
    pub ccy: String,
    #[serde(default)]
    pub chain: String,
    #[serde(default)]
    pub amt: String,
    #[serde(default)]
    pub to: String,
    #[serde(default)]
    pub tx_id: String,
    #[serde(default)]
    pub state: String,
    #[serde(default)]
    pub dep_id: String,
    #[serde(default)]
    pub ts: String,
}

/// Withdrawal record.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct WithdrawalRecord {
    #[serde(default)]
    pub ccy: String,
    #[serde(default)]
    pub chain: String,
    #[serde(default)]
    pub amt: String,
    #[serde(default)]
    pub to: String,
    #[serde(default)]
    pub tx_id: String,
    #[serde(default)]
    pub fee: String,
    #[serde(default)]
    pub state: String,
    #[serde(default)]
    pub wd_id: String,
    #[serde(default)]
    pub client_id: String,
    #[serde(default)]
    pub ts: String,
}

/// Deposit address.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct DepositAddress {
    #[serde(default)]
    pub addr: String,
    #[serde(default)]
    pub tag: String,
    #[serde(default)]
    pub memo: String,
    #[serde(default)]
    pub pmt_id: String,
    #[serde(default)]
    pub ccy: String,
    #[serde(default)]
    pub chain: String,
    #[serde(default)]
    pub ct_addr: String,
    #[serde(default)]
    pub selected: bool,
}
