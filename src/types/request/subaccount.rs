use serde::Serialize;

/// Get sub-account list.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetSubAccountListRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_acct: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Get sub-account trading balance.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetSubAccountBalanceRequest {
    pub sub_acct: String,
}

/// Get sub-account funding balance.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetSubAccountFundingBalanceRequest {
    pub sub_acct: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
}

/// Sub-account transfer.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SubAccountTransferRequest {
    pub ccy: String,
    pub amt: String,
    pub from: String,
    pub to: String,
    pub from_sub_account: String,
    pub to_sub_account: String,
}
