use okx_client::{ClientConfigBuilder, RestClient};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClientConfigBuilder::new().build();
    let client = RestClient::new(config)?;

    let times = client.get_server_time().await?;
    for time in times {
        println!("OKX server time: {}", time.ts);
    }

    Ok(())
}
