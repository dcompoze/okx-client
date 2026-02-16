use serde::Deserialize;

/// Sub-account information.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SubAccount {
    #[serde(default, rename = "type")]
    pub type_: String,
    #[serde(default)]
    pub enable: bool,
    #[serde(default)]
    pub sub_acct: String,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub mobile: String,
    #[serde(default)]
    pub can_trans_out: bool,
    #[serde(default)]
    pub ts: String,
}

/// Sub-account transfer result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SubAccountTransferResult {
    #[serde(default)]
    pub trans_id: String,
}
