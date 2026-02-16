use crate::error::OkxResult;
use crate::rest::RestClient;
use crate::types::request::market::*;
use crate::types::response::market::*;

impl RestClient {
    // ──────────────────── Market Data (Public) ────────────────────

    /// Get tickers for all instruments of a given type.
    /// GET /api/v5/market/tickers
    pub async fn get_tickers(&self, params: &GetTickersRequest) -> OkxResult<Vec<Ticker>> {
        self.get("/api/v5/market/tickers", Some(params)).await
    }

    /// Get ticker for a single instrument.
    /// GET /api/v5/market/ticker
    pub async fn get_ticker(&self, params: &GetTickerRequest) -> OkxResult<Vec<Ticker>> {
        self.get("/api/v5/market/ticker", Some(params)).await
    }

    /// Get order book for an instrument.
    /// GET /api/v5/market/books
    pub async fn get_order_book(&self, params: &GetOrderBookRequest) -> OkxResult<Vec<OrderBook>> {
        self.get("/api/v5/market/books", Some(params)).await
    }

    /// Get candlestick charts (most recent 1,440 data entries).
    /// GET /api/v5/market/candles
    pub async fn get_candles(&self, params: &GetCandlesRequest) -> OkxResult<Vec<Candle>> {
        self.get("/api/v5/market/candles", Some(params)).await
    }

    /// Get historic candlestick charts (older data).
    /// GET /api/v5/market/history-candles
    pub async fn get_history_candles(&self, params: &GetCandlesRequest) -> OkxResult<Vec<Candle>> {
        self.get("/api/v5/market/history-candles", Some(params))
            .await
    }

    /// Get recent trades.
    /// GET /api/v5/market/trades
    pub async fn get_trades(&self, params: &GetTradesRequest) -> OkxResult<Vec<Trade>> {
        self.get("/api/v5/market/trades", Some(params)).await
    }

    /// Get historic trades (last 3 months).
    /// GET /api/v5/market/history-trades
    pub async fn get_history_trades(
        &self,
        params: &GetHistoricTradesRequest,
    ) -> OkxResult<Vec<Trade>> {
        self.get("/api/v5/market/history-trades", Some(params)).await
    }

    /// Get 24-hour total trading volume on the platform.
    /// GET /api/v5/market/platform-24-volume
    pub async fn get_24h_total_volume(&self) -> OkxResult<Vec<PlatformVolume>> {
        self.get::<PlatformVolume, ()>("/api/v5/market/platform-24-volume", None)
            .await
    }

    /// Get index tickers.
    /// GET /api/v5/market/index-tickers
    pub async fn get_index_tickers(
        &self,
        params: &GetIndexTickersRequest,
    ) -> OkxResult<Vec<IndexTicker>> {
        self.get("/api/v5/market/index-tickers", Some(params)).await
    }

    /// Get index candlestick charts.
    /// GET /api/v5/market/index-candles
    pub async fn get_index_candles(
        &self,
        params: &GetIndexCandlesRequest,
    ) -> OkxResult<Vec<Candle>> {
        self.get("/api/v5/market/index-candles", Some(params)).await
    }

    /// Get mark price candlestick charts.
    /// GET /api/v5/market/mark-price-candles
    pub async fn get_mark_price_candles(
        &self,
        params: &GetMarkPriceCandlesRequest,
    ) -> OkxResult<Vec<Candle>> {
        self.get("/api/v5/market/mark-price-candles", Some(params))
            .await
    }
}
