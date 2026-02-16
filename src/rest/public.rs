use crate::error::OkxResult;
use crate::rest::RestClient;
use crate::types::request::public::*;
use crate::types::response::public::*;

impl RestClient {

    /// Get instruments (list of trading instruments).
    /// GET /api/v5/public/instruments
    pub async fn get_instruments(
        &self,
        params: &GetInstrumentsRequest,
    ) -> OkxResult<Vec<Instrument>> {
        self.get("/api/v5/public/instruments", Some(params)).await
    }

    /// Get delivery/exercise history.
    /// GET /api/v5/public/delivery-exercise-history
    pub async fn get_delivery_exercise_history(
        &self,
        params: &GetDeliveryExerciseHistoryRequest,
    ) -> OkxResult<Vec<DeliveryExerciseHistory>> {
        self.get("/api/v5/public/delivery-exercise-history", Some(params))
            .await
    }

    /// Get open interest.
    /// GET /api/v5/public/open-interest
    pub async fn get_open_interest(
        &self,
        params: &GetOpenInterestRequest,
    ) -> OkxResult<Vec<OpenInterest>> {
        self.get("/api/v5/public/open-interest", Some(params)).await
    }

    /// Get funding rate for a perpetual swap instrument.
    /// GET /api/v5/public/funding-rate
    pub async fn get_funding_rate(
        &self,
        params: &GetFundingRateRequest,
    ) -> OkxResult<Vec<FundingRate>> {
        self.get("/api/v5/public/funding-rate", Some(params)).await
    }

    /// Get funding rate history.
    /// GET /api/v5/public/funding-rate-history
    pub async fn get_funding_rate_history(
        &self,
        params: &GetFundingRateHistoryRequest,
    ) -> OkxResult<Vec<FundingRate>> {
        self.get("/api/v5/public/funding-rate-history", Some(params))
            .await
    }

    /// Get mark price.
    /// GET /api/v5/public/mark-price
    pub async fn get_mark_price(
        &self,
        params: &GetMarkPriceRequest,
    ) -> OkxResult<Vec<MarkPrice>> {
        self.get("/api/v5/public/mark-price", Some(params)).await
    }

    /// Get position tiers (margin tiers).
    /// GET /api/v5/public/position-tiers
    pub async fn get_position_tiers(
        &self,
        params: &GetPositionTiersRequest,
    ) -> OkxResult<Vec<PositionTier>> {
        self.get("/api/v5/public/position-tiers", Some(params)).await
    }

    /// Get insurance fund balance.
    /// GET /api/v5/public/insurance-fund
    pub async fn get_insurance_fund(
        &self,
        params: &GetInsuranceFundRequest,
    ) -> OkxResult<Vec<InsuranceFund>> {
        self.get("/api/v5/public/insurance-fund", Some(params)).await
    }

    /// Convert between coin and contract units.
    /// GET /api/v5/public/convert-contract-coin
    pub async fn get_unit_convert(
        &self,
        params: &GetUnitConvertRequest,
    ) -> OkxResult<Vec<UnitConvertResult>> {
        self.get("/api/v5/public/convert-contract-coin", Some(params))
            .await
    }

    /// Get option tick bands.
    /// GET /api/v5/public/instrument-tick-bands
    pub async fn get_option_tick_bands(
        &self,
        params: &GetOptionTickBandsRequest,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.get("/api/v5/public/instrument-tick-bands", Some(params))
            .await
    }

    /// Get estimated delivery/exercise price.
    /// GET /api/v5/public/estimated-price
    pub async fn get_estimated_price(
        &self,
        params: &GetEstimatedPriceRequest,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.get("/api/v5/public/estimated-price", Some(params))
            .await
    }

    /// Get discount rate and interest-free quota.
    /// GET /api/v5/public/discount-rate-interest-free-quota
    pub async fn get_discount_rate(
        &self,
        params: &GetDiscountRateRequest,
    ) -> OkxResult<Vec<DiscountRate>> {
        self.get(
            "/api/v5/public/discount-rate-interest-free-quota",
            Some(params),
        )
        .await
    }

    /// Get premium history.
    /// GET /api/v5/public/premium-history
    pub async fn get_premium_history(
        &self,
        params: &GetPremiumHistoryRequest,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.get("/api/v5/public/premium-history", Some(params))
            .await
    }

    /// Get liquidation orders.
    /// GET /api/v5/public/liquidation-orders
    pub async fn get_liquidation_orders(
        &self,
        params: &GetLiquidationOrdersRequest,
    ) -> OkxResult<Vec<serde_json::Value>> {
        self.get("/api/v5/public/liquidation-orders", Some(params))
            .await
    }

    /// Get server time.
    /// GET /api/v5/public/time
    pub async fn get_server_time(&self) -> OkxResult<Vec<ServerTime>> {
        self.get::<ServerTime, ()>("/api/v5/public/time", None).await
    }
}
