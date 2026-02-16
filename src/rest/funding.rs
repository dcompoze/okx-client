use crate::error::OkxResult;
use crate::rest::RestClient;
use crate::types::request::funding::*;
use crate::types::response::funding::*;

impl RestClient {

    /// Get list of currencies.
    /// GET /api/v5/asset/currencies
    pub async fn get_currencies(
        &self,
        params: &GetCurrenciesRequest,
    ) -> OkxResult<Vec<Currency>> {
        self.get_signed("/api/v5/asset/currencies", Some(params))
            .await
    }

    /// Get asset balances (funding account).
    /// GET /api/v5/asset/balances
    pub async fn get_asset_balances(
        &self,
        params: &GetAssetBalancesRequest,
    ) -> OkxResult<Vec<AssetBalance>> {
        self.get_signed("/api/v5/asset/balances", Some(params))
            .await
    }

    /// Submit a withdrawal request.
    /// POST /api/v5/asset/withdrawal
    pub async fn withdraw(&self, params: &WithdrawRequest) -> OkxResult<Vec<WithdrawalResult>> {
        self.post_signed("/api/v5/asset/withdrawal", params).await
    }

    /// Transfer funds between accounts.
    /// POST /api/v5/asset/transfer
    pub async fn funds_transfer(
        &self,
        params: &FundsTransferRequest,
    ) -> OkxResult<Vec<TransferResult>> {
        self.post_signed("/api/v5/asset/transfer", params).await
    }

    /// Get deposit history.
    /// GET /api/v5/asset/deposit-history
    pub async fn get_deposit_history(
        &self,
        params: &GetDepositHistoryRequest,
    ) -> OkxResult<Vec<DepositRecord>> {
        self.get_signed("/api/v5/asset/deposit-history", Some(params))
            .await
    }

    /// Get withdrawal history.
    /// GET /api/v5/asset/withdrawal-history
    pub async fn get_withdrawal_history(
        &self,
        params: &GetWithdrawalHistoryRequest,
    ) -> OkxResult<Vec<WithdrawalRecord>> {
        self.get_signed("/api/v5/asset/withdrawal-history", Some(params))
            .await
    }

    /// Get deposit addresses.
    /// GET /api/v5/asset/deposit-address
    pub async fn get_deposit_address(
        &self,
        params: &GetDepositAddressRequest,
    ) -> OkxResult<Vec<DepositAddress>> {
        self.get_signed("/api/v5/asset/deposit-address", Some(params))
            .await
    }
}
