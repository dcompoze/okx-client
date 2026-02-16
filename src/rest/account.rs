use crate::error::OkxResult;
use crate::rest::RestClient;
use crate::types::request::account::*;
use crate::types::response::account::*;

impl RestClient {
    // ──────────────────── Account ────────────────────

    /// Get account balance.
    /// GET /api/v5/account/balance
    pub async fn get_balance(
        &self,
        params: &GetBalanceRequest,
    ) -> OkxResult<Vec<AccountBalance>> {
        self.get_signed("/api/v5/account/balance", Some(params))
            .await
    }

    /// Get positions. When the account is in `net` mode, net positions will be
    /// displayed; when in `long/short` mode, long or short positions will be displayed.
    /// GET /api/v5/account/positions
    pub async fn get_positions(&self, params: &GetPositionsRequest) -> OkxResult<Vec<Position>> {
        self.get_signed("/api/v5/account/positions", Some(params))
            .await
    }

    /// Get position history for the last 3 months.
    /// GET /api/v5/account/positions-history
    pub async fn get_positions_history(
        &self,
        params: &GetPositionsHistoryRequest,
    ) -> OkxResult<Vec<Position>> {
        self.get_signed("/api/v5/account/positions-history", Some(params))
            .await
    }

    /// Get account position risk data.
    /// GET /api/v5/account/account-position-risk
    pub async fn get_account_position_risk(
        &self,
        params: &GetAccountPositionRiskRequest,
    ) -> OkxResult<Vec<AccountRiskState>> {
        self.get_signed("/api/v5/account/account-position-risk", Some(params))
            .await
    }

    /// Get bills detail (last 7 days).
    /// GET /api/v5/account/bills
    pub async fn get_bills(
        &self,
        params: &GetBillsRequest,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.get_signed("/api/v5/account/bills", Some(params)).await
    }

    /// Get bills archive (last 3 months).
    /// GET /api/v5/account/bills-archive
    pub async fn get_bills_archive(
        &self,
        params: &GetBillsRequest,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.get_signed("/api/v5/account/bills-archive", Some(params))
            .await
    }

    /// Get account configuration.
    /// GET /api/v5/account/config
    pub async fn get_account_config(&self) -> OkxResult<Vec<AccountConfig>> {
        self.get_signed::<AccountConfig, ()>("/api/v5/account/config", None)
            .await
    }

    /// Set position mode: `long_short_mode` or `net_mode`.
    /// POST /api/v5/account/set-position-mode
    pub async fn set_position_mode(
        &self,
        params: &SetPositionModeRequest,
    ) -> OkxResult<Vec<SetResult>> {
        self.post_signed("/api/v5/account/set-position-mode", params)
            .await
    }

    /// Set leverage for an instrument or currency in a given margin mode.
    /// POST /api/v5/account/set-leverage
    pub async fn set_leverage(&self, params: &SetLeverageRequest) -> OkxResult<Vec<LeverageInfo>> {
        self.post_signed("/api/v5/account/set-leverage", params)
            .await
    }

    /// Get leverage of an instrument.
    /// GET /api/v5/account/leverage-info
    pub async fn get_leverage(&self, params: &GetLeverageRequest) -> OkxResult<Vec<LeverageInfo>> {
        self.get_signed("/api/v5/account/leverage-info", Some(params))
            .await
    }

    /// Get maximum buy/sell amount.
    /// GET /api/v5/account/max-size
    pub async fn get_max_buy_sell_amount(
        &self,
        params: &GetMaxBuySellAmountRequest,
    ) -> OkxResult<Vec<MaxBuySellAmount>> {
        self.get_signed("/api/v5/account/max-size", Some(params))
            .await
    }

    /// Get maximum available tradeable size.
    /// GET /api/v5/account/max-avail-size
    pub async fn get_max_avail_size(
        &self,
        params: &GetMaxAvailSizeRequest,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.get_signed("/api/v5/account/max-avail-size", Some(params))
            .await
    }

    /// Get maximum loan amount.
    /// GET /api/v5/account/max-loan
    pub async fn get_max_loan(
        &self,
        params: &GetMaxLoanRequest,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.get_signed("/api/v5/account/max-loan", Some(params))
            .await
    }

    /// Get trading fee rates.
    /// GET /api/v5/account/trade-fee
    pub async fn get_fee_rates(&self, params: &GetFeeRatesRequest) -> OkxResult<Vec<FeeRate>> {
        self.get_signed("/api/v5/account/trade-fee", Some(params))
            .await
    }

    /// Get interest accrued data.
    /// GET /api/v5/account/interest-accrued
    pub async fn get_interest_accrued(
        &self,
        params: &GetInterestAccruedRequest,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.get_signed("/api/v5/account/interest-accrued", Some(params))
            .await
    }

    /// Get interest rate.
    /// GET /api/v5/account/interest-rate
    pub async fn get_interest_rate(&self) -> OkxResult<Vec<serde_json::Value>> {
        self.get_signed::<serde_json::Value, ()>("/api/v5/account/interest-rate", None)
            .await
    }

    /// Get maximum withdrawals.
    /// GET /api/v5/account/max-withdrawal
    pub async fn get_max_withdrawals(
        &self,
        params: &GetMaxWithdrawalsRequest,
    ) -> OkxResult<Vec<MaxWithdrawal>> {
        self.get_signed("/api/v5/account/max-withdrawal", Some(params))
            .await
    }

    /// Set Greeks display type.
    /// POST /api/v5/account/set-greeks
    pub async fn set_greeks(&self, params: &SetGreeksRequest) -> OkxResult<Vec<SetResult>> {
        self.post_signed("/api/v5/account/set-greeks", params).await
    }

    /// Set isolated margin trading mode.
    /// POST /api/v5/account/set-isolated-mode
    pub async fn set_isolated_mode(
        &self,
        params: &SetIsolatedModeRequest,
    ) -> OkxResult<Vec<SetResult>> {
        self.post_signed("/api/v5/account/set-isolated-mode", params)
            .await
    }

    /// Get account risk state.
    /// GET /api/v5/account/risk-state
    pub async fn get_account_risk_state(&self) -> OkxResult<Vec<AccountRiskState>> {
        self.get_signed::<AccountRiskState, ()>("/api/v5/account/risk-state", None)
            .await
    }

    /// Set MMP (Market Maker Protection) configuration.
    /// POST /api/v5/account/mmp-config
    pub async fn set_mmp_config(&self, params: &SetMmpConfigRequest) -> OkxResult<Vec<SetResult>> {
        self.post_signed("/api/v5/account/mmp-config", params).await
    }

    /// Get MMP configuration.
    /// GET /api/v5/account/mmp-config
    pub async fn get_mmp_config(&self, params: &GetMmpConfigRequest) -> OkxResult<Vec<MmpConfig>> {
        self.get_signed("/api/v5/account/mmp-config", Some(params))
            .await
    }

    /// Set account level.
    /// POST /api/v5/account/set-account-level
    pub async fn set_account_level(
        &self,
        params: &SetAccountLevelRequest,
    ) -> OkxResult<Vec<SetResult>> {
        self.post_signed("/api/v5/account/set-account-level", params)
            .await
    }
}
