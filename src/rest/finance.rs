use crate::error::OkxResult;
use crate::rest::RestClient;

impl RestClient {

    /// Get staking offers.
    /// GET /api/v5/finance/staking-defi/offers
    pub async fn get_staking_offers(
        &self,
        params: &serde_json::Value,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.get_signed("/api/v5/finance/staking-defi/offers", Some(params))
            .await
    }

    /// Stake / purchase.
    /// POST /api/v5/finance/staking-defi/purchase
    pub async fn stake_purchase(
        &self,
        params: &serde_json::Value,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.post_signed("/api/v5/finance/staking-defi/purchase", params)
            .await
    }

    /// Redeem staking.
    /// POST /api/v5/finance/staking-defi/redeem
    pub async fn stake_redeem(
        &self,
        params: &serde_json::Value,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.post_signed("/api/v5/finance/staking-defi/redeem", params)
            .await
    }

    /// Get staking orders.
    /// GET /api/v5/finance/staking-defi/orders-active
    pub async fn get_staking_orders_active(
        &self,
        params: &serde_json::Value,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.get_signed("/api/v5/finance/staking-defi/orders-active", Some(params))
            .await
    }

    /// Get staking order history.
    /// GET /api/v5/finance/staking-defi/orders-history
    pub async fn get_staking_order_history(
        &self,
        params: &serde_json::Value,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.get_signed(
            "/api/v5/finance/staking-defi/orders-history",
            Some(params),
        )
        .await
    }

    /// Get savings balance.
    /// GET /api/v5/finance/savings/balance
    pub async fn get_savings_balance(
        &self,
        params: &serde_json::Value,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.get_signed("/api/v5/finance/savings/balance", Some(params))
            .await
    }

    /// Savings purchase/redemption.
    /// POST /api/v5/finance/savings/purchase-redempt
    pub async fn savings_purchase_redempt(
        &self,
        params: &serde_json::Value,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.post_signed("/api/v5/finance/savings/purchase-redempt", params)
            .await
    }

    /// Get lending rate summary.
    /// GET /api/v5/finance/savings/lending-rate-summary
    pub async fn get_lending_rate_summary(&self) -> OkxResult<Vec<serde_json::Value>> {
        self.get::<serde_json::Value, ()>(
            "/api/v5/finance/savings/lending-rate-summary",
            None,
        )
        .await
    }

    /// Get lending rate history.
    /// GET /api/v5/finance/savings/lending-rate-history
    pub async fn get_lending_rate_history(
        &self,
        params: &serde_json::Value,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.get(
            "/api/v5/finance/savings/lending-rate-history",
            Some(params),
        )
        .await
    }
}
