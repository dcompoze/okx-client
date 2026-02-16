use okx_client::constants;
use okx_client::error::OkxError;
use okx_client::types::enums::PosMode;
use okx_client::types::request::account::{GetBalanceRequest, SetPositionModeRequest};
use okx_client::{ClientConfigBuilder, RestClient, TradingMode};
use serde_json::Value;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn header_value(request: &wiremock::Request, name: &str) -> String {
    request
        .headers
        .get(name)
        .and_then(|v| v.to_str().ok())
        .unwrap_or_default()
        .to_string()
}

#[tokio::test]
async fn public_get_server_time_parses_response() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v5/public/time"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "code": "0",
            "msg": "",
            "data": [
                { "ts": "1700000000000" }
            ]
        })))
        .mount(&server)
        .await;

    let config = ClientConfigBuilder::new().base_url(&server.uri()).build();
    let client = RestClient::new(config).expect("client should build");

    let result = client
        .get_server_time()
        .await
        .expect("request should succeed");

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].ts, "1700000000000");
}

#[tokio::test]
async fn signed_get_includes_auth_and_demo_headers() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v5/account/balance"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "code": "0",
            "msg": "",
            "data": [{}]
        })))
        .mount(&server)
        .await;

    let config = ClientConfigBuilder::new()
        .base_url(&server.uri())
        .trading_mode(TradingMode::Demo)
        .credentials("test-api-key", "test-api-secret", "test-passphrase")
        .build();
    let client = RestClient::new(config).expect("client should build");

    client
        .get_balance(&GetBalanceRequest {
            ccy: Some("BTC,ETH".to_string()),
        })
        .await
        .expect("signed request should succeed");

    let requests = server
        .received_requests()
        .await
        .expect("should capture requests");
    assert_eq!(requests.len(), 1);

    let request = &requests[0];
    assert_eq!(request.url.path(), "/api/v5/account/balance");
    assert_eq!(request.url.query(), Some("ccy=BTC%2CETH"));

    assert_eq!(header_value(request, "ok-access-key"), "test-api-key");
    assert_eq!(
        header_value(request, "ok-access-passphrase"),
        "test-passphrase"
    );
    assert!(!header_value(request, "ok-access-sign").is_empty());
    assert!(!header_value(request, "ok-access-timestamp").is_empty());
    assert_eq!(header_value(request, "x-simulated-trading"), "1");
}

#[tokio::test]
async fn signed_post_injects_program_tag() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/api/v5/account/set-position-mode"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "code": "0",
            "msg": "",
            "data": [
                { "posMode": "net_mode" }
            ]
        })))
        .mount(&server)
        .await;

    let config = ClientConfigBuilder::new()
        .base_url(&server.uri())
        .credentials("test-api-key", "test-api-secret", "test-passphrase")
        .build();
    let client = RestClient::new(config).expect("client should build");

    let result = client
        .set_position_mode(&SetPositionModeRequest {
            pos_mode: PosMode::NetMode,
        })
        .await
        .expect("signed post should succeed");

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].data["posMode"], "net_mode");

    let requests = server
        .received_requests()
        .await
        .expect("should capture requests");
    assert_eq!(requests.len(), 1);

    let request = &requests[0];
    let body: Value = serde_json::from_slice(&request.body).expect("body should be json");

    assert_eq!(body["posMode"], "net_mode");
    assert_eq!(body["tag"], constants::PROGRAM_ID);
    assert_eq!(header_value(request, "ok-access-key"), "test-api-key");
    assert!(!header_value(request, "ok-access-sign").is_empty());
}

#[tokio::test]
async fn private_endpoint_without_credentials_fails_before_http_request() {
    let server = MockServer::start().await;

    let config = ClientConfigBuilder::new().base_url(&server.uri()).build();
    let client = RestClient::new(config).expect("client should build");

    let err = client.get_account_config().await.expect_err("must fail");
    match err {
        OkxError::Auth(msg) => assert!(msg.contains("Credentials required")),
        other => panic!("expected auth error, got: {other:?}"),
    }

    let requests = server
        .received_requests()
        .await
        .expect("should capture requests");
    assert!(requests.is_empty());
}
