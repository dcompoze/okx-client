use okx_client::{ClientConfigBuilder, RestClient};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("OKX_API_KEY")?;
    let api_secret = std::env::var("OKX_API_SECRET")?;
    let passphrase = std::env::var("OKX_API_PASSPHRASE")?;

    let config = ClientConfigBuilder::new()
        .credentials(&api_key, &api_secret, &passphrase)
        .build();
    let client = RestClient::new(config)?;

    let account_config = client.get_account_config().await?;
    println!("Fetched {} account config record(s)", account_config.len());

    Ok(())
}
