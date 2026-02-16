use crate::error::OkxResult;
use crate::rest::RestClient;
use crate::types::request::convert::*;
use crate::types::response::convert::*;

impl RestClient {

    /// Get convert currencies.
    /// GET /api/v5/asset/convert/currencies
    pub async fn get_convert_currencies(&self) -> OkxResult<Vec<ConvertCurrency>> {
        self.get_signed::<ConvertCurrency, ()>("/api/v5/asset/convert/currencies", None)
            .await
    }

    /// Estimate conversion quote.
    /// POST /api/v5/asset/convert/estimate-quote
    pub async fn estimate_quote(
        &self,
        params: &EstimateQuoteRequest,
    ) -> OkxResult<Vec<ConvertQuote>> {
        self.post_signed("/api/v5/asset/convert/estimate-quote", params)
            .await
    }

    /// Execute a conversion trade.
    /// POST /api/v5/asset/convert/trade
    pub async fn convert_trade(
        &self,
        params: &ConvertTradeRequest,
    ) -> OkxResult<Vec<ConvertTradeResult>> {
        self.post_signed("/api/v5/asset/convert/trade", params)
            .await
    }

    /// Get convert trade history.
    /// GET /api/v5/asset/convert/history
    pub async fn get_convert_history(
        &self,
        params: &GetConvertHistoryRequest,
    ) -> OkxResult<Vec<ConvertTradeResult>> {
        self.get_signed("/api/v5/asset/convert/history", Some(params))
            .await
    }
}
