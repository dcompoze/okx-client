use okx_client::types::enums::InstrumentType;
use okx_client::types::request::market::{
    GetCandlesRequest, GetOrderBookRequest, GetTickerRequest, GetTickersRequest, GetTradesRequest,
};
use okx_client::{ClientConfigBuilder, RestClient};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let inst_id = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "BTC-USDT".to_string());

    let client = RestClient::new(ClientConfigBuilder::new().build())?;

    let ticker = client
        .get_ticker(&GetTickerRequest {
            inst_id: inst_id.clone(),
        })
        .await?;
    if let Some(t) = ticker.first() {
        println!(
            "Ticker {}: last={}, bid={}, ask={}, ts={}",
            t.inst_id, t.last, t.bid_px, t.ask_px, t.ts
        );
    }

    let tickers = client
        .get_tickers(&GetTickersRequest {
            inst_type: InstrumentType::Spot,
            uly: None,
            inst_family: None,
        })
        .await?;
    println!("Spot tickers returned: {}", tickers.len());

    let books = client
        .get_order_book(&GetOrderBookRequest {
            inst_id: inst_id.clone(),
            sz: Some("5".to_string()),
        })
        .await?;
    if let Some(book) = books.first() {
        println!(
            "Order book {}: bids={}, asks={}, ts={}",
            inst_id,
            book.bids.len(),
            book.asks.len(),
            book.ts
        );
    }

    let trades = client
        .get_trades(&GetTradesRequest {
            inst_id: inst_id.clone(),
            limit: Some("5".to_string()),
        })
        .await?;
    println!("Recent trades for {}: {}", inst_id, trades.len());

    let candles = client
        .get_candles(&GetCandlesRequest {
            inst_id: inst_id.clone(),
            bar: Some("1m".to_string()),
            after: None,
            before: None,
            limit: Some("5".to_string()),
        })
        .await?;
    if let Some(candle) = candles.first() {
        println!("Latest candle {} fields: {:?}", inst_id, candle);
    }

    let volume = client.get_24h_total_volume().await?;
    if let Some(v) = volume.first() {
        println!("Platform 24h volume USD={}, ts={}", v.vol_usd, v.ts);
    }

    Ok(())
}
