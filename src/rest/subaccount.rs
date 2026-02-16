use crate::error::OkxResult;
use crate::rest::RestClient;
use crate::types::request::subaccount::*;
use crate::types::response::subaccount::*;

impl RestClient {

    /// Get sub-account list.
    /// GET /api/v5/users/subaccount/list
    pub async fn get_sub_account_list(
        &self,
        params: &GetSubAccountListRequest,
    ) -> OkxResult<Vec<SubAccount>> {
        self.get_signed("/api/v5/users/subaccount/list", Some(params))
            .await
    }

    /// Get sub-account trading balance.
    /// GET /api/v5/account/subaccount/balances
    pub async fn get_sub_account_balance(
        &self,
        params: &GetSubAccountBalanceRequest,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.get_signed("/api/v5/account/subaccount/balances", Some(params))
            .await
    }

    /// Get sub-account funding balance.
    /// GET /api/v5/asset/subaccount/balances
    pub async fn get_sub_account_funding_balance(
        &self,
        params: &GetSubAccountFundingBalanceRequest,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.get_signed("/api/v5/asset/subaccount/balances", Some(params))
            .await
    }

    /// Transfer between sub-accounts.
    /// POST /api/v5/asset/subaccount/transfer
    pub async fn sub_account_transfer(
        &self,
        params: &SubAccountTransferRequest,
    ) -> OkxResult<Vec<SubAccountTransferResult>> {
        self.post_signed("/api/v5/asset/subaccount/transfer", params)
            .await
    }
}
