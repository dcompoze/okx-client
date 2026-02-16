use okx_client::types::enums::InstrumentType;
use okx_client::types::request::public::{
    GetFundingRateRequest, GetInstrumentsRequest, GetMarkPriceRequest, GetOpenInterestRequest,
};
use okx_client::{ClientConfigBuilder, RestClient};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let inst_id = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "BTC-USDT-SWAP".to_string());

    let client = RestClient::new(ClientConfigBuilder::new().build())?;

    let server_time = client.get_server_time().await?;
    if let Some(t) = server_time.first() {
        println!("Server time: {}", t.ts);
    }

    let instruments = client
        .get_instruments(&GetInstrumentsRequest {
            inst_type: InstrumentType::Swap,
            uly: None,
            inst_id: None,
            inst_family: None,
        })
        .await?;
    println!("Swap instruments returned: {}", instruments.len());

    let funding = client
        .get_funding_rate(&GetFundingRateRequest {
            inst_id: inst_id.clone(),
        })
        .await?;
    if let Some(f) = funding.first() {
        println!(
            "Funding {}: rate={}, next_rate={}, next_time={}",
            f.inst_id, f.funding_rate, f.next_funding_rate, f.next_funding_time
        );
    }

    let mark_price = client
        .get_mark_price(&GetMarkPriceRequest {
            inst_type: InstrumentType::Swap,
            uly: None,
            inst_id: Some(inst_id.clone()),
            inst_family: None,
        })
        .await?;
    if let Some(m) = mark_price.first() {
        println!("Mark price {}: {}, ts={}", m.inst_id, m.mark_px, m.ts);
    }

    let open_interest = client
        .get_open_interest(&GetOpenInterestRequest {
            inst_type: InstrumentType::Swap,
            uly: None,
            inst_id: Some(inst_id.clone()),
            inst_family: None,
        })
        .await?;
    if let Some(oi) = open_interest.first() {
        println!(
            "Open interest {}: oi={}, oi_ccy={}, ts={}",
            oi.inst_id, oi.oi, oi.oi_ccy, oi.ts
        );
    }

    Ok(())
}
