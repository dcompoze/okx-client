mod response;

pub mod account;
pub mod affiliate;
pub mod algo;
pub mod block_trading;
pub mod broker;
pub mod convert;
pub mod copy_trading;
pub mod finance;
pub mod funding;
pub mod grid_trading;
pub mod market;
pub mod public;
pub mod signal_bot;
pub mod spread_trading;
pub mod subaccount;
pub mod system;
pub mod trade;
pub mod trading_data;

use reqwest::header::{HeaderMap, HeaderValue};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::policies::ExponentialBackoff;
use reqwest_retry::RetryTransientMiddleware;
use reqwest_tracing::TracingMiddleware;
use secrecy::ExposeSecret;
use serde::de::DeserializeOwned;
use serde::Serialize;
use tracing::instrument;

use crate::auth;
use crate::config::{ClientConfig, TradingMode};
use crate::constants;
use crate::error::{OkxError, OkxResult};

use self::response::OkxResponse;

/// HTTP REST client for the OKX API v5.
///
/// Provides methods covering all OKX REST endpoints, organized by domain.
/// Methods are defined in domain-specific files (e.g., `trade.rs`, `account.rs`).
pub struct RestClient {
    http: ClientWithMiddleware,
    config: ClientConfig,
}

impl RestClient {
    /// Create a new `RestClient` with the given configuration.
    pub fn new(config: ClientConfig) -> OkxResult<Self> {
        let mut default_headers = HeaderMap::new();
        default_headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        default_headers.insert("Accept", HeaderValue::from_static("application/json"));

        if config.trading_mode == TradingMode::Demo {
            default_headers.insert(
                constants::HEADER_SIMULATED_TRADING,
                HeaderValue::from_static("1"),
            );
        }

        let client = reqwest::Client::builder()
            .default_headers(default_headers)
            .timeout(config.request_timeout)
            .pool_max_idle_per_host(10)
            .build()
            .map_err(OkxError::Http)?;

        let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);

        let http = ClientBuilder::new(client)
            .with(TracingMiddleware::default())
            .with(RetryTransientMiddleware::new_with_policy(retry_policy))
            .build();

        Ok(Self { http, config })
    }

    /// Create a `RestClient` with default configuration (unauthenticated, global, live).
    pub fn default_client() -> OkxResult<Self> {
        Self::new(ClientConfig::default())
    }

    /// Returns the base URL for REST requests.
    fn base_url(&self) -> &str {
        if let Some(ref url) = self.config.base_url_override {
            return url;
        }
        self.config.region.rest_base_url()
    }

    /// Returns a reference to the client configuration.
    pub fn config(&self) -> &ClientConfig {
        &self.config
    }

    /// Generate an ISO 8601 timestamp for REST signing.
    fn timestamp() -> String {
        // Use system time to build an ISO 8601 timestamp.
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time is before unix epoch");
        let secs = now.as_secs();
        let millis = now.subsec_millis();

        // Convert to datetime components without `chrono`.
        let days = secs / 86400;
        let time_secs = secs % 86400;
        let hours = time_secs / 3600;
        let minutes = (time_secs % 3600) / 60;
        let seconds = time_secs % 60;

        // Calculate `year`, `month`, and `day` from days since epoch.
        let (year, month, day) = days_to_date(days);

        format!(
            "{year:04}-{month:02}-{day:02}T{hours:02}:{minutes:02}:{seconds:02}.{millis:03}Z"
        )
    }

    /// Build auth headers for signed requests.
    fn auth_headers(
        &self,
        timestamp: &str,
        method: &str,
        endpoint: &str,
        body: &str,
    ) -> OkxResult<HeaderMap> {
        let creds = self
            .config
            .credentials
            .as_ref()
            .ok_or_else(|| OkxError::Auth("Credentials required for private endpoint".into()))?;

        let signature = auth::sign_rest(timestamp, method, endpoint, body, &creds.api_secret)?;

        let mut headers = HeaderMap::new();
        headers.insert(
            constants::HEADER_ACCESS_KEY,
            HeaderValue::from_str(&creds.api_key)
                .map_err(|e| OkxError::Auth(format!("Invalid API key header: {e}")))?,
        );
        headers.insert(
            constants::HEADER_ACCESS_SIGN,
            HeaderValue::from_str(&signature)
                .map_err(|e| OkxError::Auth(format!("Invalid signature header: {e}")))?,
        );
        headers.insert(
            constants::HEADER_ACCESS_TIMESTAMP,
            HeaderValue::from_str(timestamp)
                .map_err(|e| OkxError::Auth(format!("Invalid timestamp header: {e}")))?,
        );
        headers.insert(
            constants::HEADER_ACCESS_PASSPHRASE,
            HeaderValue::from_str(creds.passphrase.expose_secret())
                .map_err(|e| OkxError::Auth(format!("Invalid passphrase header: {e}")))?,
        );

        Ok(headers)
    }

    /// Serialize query parameters to a query string (e.g., `?key=val&key2=val2`).
    fn serialize_query_string<P: Serialize>(params: &P) -> OkxResult<String> {
        let value = serde_json::to_value(params)?;
        if let serde_json::Value::Object(map) = value {
            let pairs: Vec<String> = map
                .into_iter()
                .filter(|(_, v)| !v.is_null())
                .map(|(k, v)| {
                    let val = match v {
                        serde_json::Value::String(s) => s,
                        other => other.to_string(),
                    };
                    format!("{}={}", k, urlencoding::encode(&val))
                })
                .collect();
            if pairs.is_empty() {
                Ok(String::new())
            } else {
                Ok(format!("?{}", pairs.join("&")))
            }
        } else {
            Ok(String::new())
        }
    }


    /// Public GET request.
    #[instrument(skip(self, params), fields(endpoint))]
    pub(crate) async fn get<T, P>(&self, endpoint: &str, params: Option<&P>) -> OkxResult<Vec<T>>
    where
        T: DeserializeOwned,
        P: Serialize,
    {
        let url = format!("{}{}", self.base_url(), endpoint);
        let mut request = self.http.get(&url);

        if let Some(p) = params {
            let qs = Self::serialize_query_string(p)?;
            if !qs.is_empty() {
                request = self.http.get(format!("{url}{qs}"));
            }
        }

        let response = request.send().await?;
        let body = response.text().await.map_err(OkxError::Http)?;
        let parsed: OkxResponse<Vec<T>> = serde_json::from_str(&body)?;
        parsed.into_result()
    }

    /// Public POST request.
    #[instrument(skip(self, params), fields(endpoint))]
    #[allow(dead_code)]
    pub(crate) async fn post<T, P>(&self, endpoint: &str, params: &P) -> OkxResult<Vec<T>>
    where
        T: DeserializeOwned,
        P: Serialize,
    {
        let url = format!("{}{}", self.base_url(), endpoint);
        let body = serde_json::to_string(params)?;

        let response = self
            .http
            .post(&url)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await?;

        let resp_body = response.text().await.map_err(OkxError::Http)?;
        let parsed: OkxResponse<Vec<T>> = serde_json::from_str(&resp_body)?;
        parsed.into_result()
    }


    /// Signed GET request (for private endpoints).
    #[instrument(skip(self, params), fields(endpoint))]
    pub(crate) async fn get_signed<T, P>(
        &self,
        endpoint: &str,
        params: Option<&P>,
    ) -> OkxResult<Vec<T>>
    where
        T: DeserializeOwned,
        P: Serialize,
    {
        let timestamp = Self::timestamp();
        let qs = if let Some(p) = params {
            Self::serialize_query_string(p)?
        } else {
            String::new()
        };

        let auth_headers = self.auth_headers(&timestamp, "GET", endpoint, &qs)?;
        let url = format!("{}{}{}", self.base_url(), endpoint, qs);

        let response = self.http.get(&url).headers(auth_headers).send().await?;

        let body = response.text().await.map_err(OkxError::Http)?;
        let parsed: OkxResponse<Vec<T>> = serde_json::from_str(&body)?;
        parsed.into_result()
    }

    /// Signed POST request (for private endpoints).
    /// Auto-injects the program ID tag into the request body.
    #[instrument(skip(self, params), fields(endpoint))]
    pub(crate) async fn post_signed<T, P>(
        &self,
        endpoint: &str,
        params: &P,
    ) -> OkxResult<Vec<T>>
    where
        T: DeserializeOwned,
        P: Serialize,
    {
        let timestamp = Self::timestamp();
        let body = inject_program_tag(&serde_json::to_value(params)?)?;

        let auth_headers = self.auth_headers(&timestamp, "POST", endpoint, &body)?;
        let url = format!("{}{}", self.base_url(), endpoint);

        let response = self
            .http
            .post(&url)
            .headers(auth_headers)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await?;

        let resp_body = response.text().await.map_err(OkxError::Http)?;
        let parsed: OkxResponse<Vec<T>> = serde_json::from_str(&resp_body)?;
        parsed.into_result()
    }
}

/// Inject the OKX program ID tag into a JSON value.
/// If the value is an object, adds `"tag": PROGRAM_ID"`.
/// If the value is an array, injects into each element.
fn inject_program_tag(value: &serde_json::Value) -> OkxResult<String> {
    let mut val = value.clone();
    match &mut val {
        serde_json::Value::Object(map) => {
            if !map.contains_key("tag") {
                map.insert(
                    "tag".to_string(),
                    serde_json::Value::String(constants::PROGRAM_ID.to_string()),
                );
            }
        }
        serde_json::Value::Array(arr) => {
            for item in arr.iter_mut() {
                if let serde_json::Value::Object(map) = item {
                    if !map.contains_key("tag") {
                        map.insert(
                            "tag".to_string(),
                            serde_json::Value::String(constants::PROGRAM_ID.to_string()),
                        );
                    }
                }
            }
        }
        _ => {}
    }
    Ok(serde_json::to_string(&val)?)
}

/// Convert days since Unix epoch to (year, month, day).
fn days_to_date(total_days: u64) -> (u64, u64, u64) {
    // Based on http://howardhinnant.github.io/date_algorithms.html.
    let z = total_days + 719468;
    let era = z / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timestamp_format() {
        let ts = RestClient::timestamp();
        // Expected format: `2024-01-15T12:30:45.123Z`.
        assert!(ts.ends_with('Z'));
        assert_eq!(ts.len(), 24);
        assert_eq!(&ts[4..5], "-");
        assert_eq!(&ts[7..8], "-");
        assert_eq!(&ts[10..11], "T");
        assert_eq!(&ts[13..14], ":");
        assert_eq!(&ts[16..17], ":");
        assert_eq!(&ts[19..20], ".");
    }

    #[test]
    fn test_days_to_date_epoch() {
        let (y, m, d) = days_to_date(0);
        assert_eq!((y, m, d), (1970, 1, 1));
    }

    #[test]
    fn test_days_to_date_known() {
        // `2024-01-15` is day `19737`.
        let (y, m, d) = days_to_date(19737);
        assert_eq!((y, m, d), (2024, 1, 15));
    }

    #[test]
    fn test_serialize_query_string() {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            inst_id: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            ccy: Option<String>,
        }

        let params = Params {
            inst_id: "BTC-USDT".to_string(),
            ccy: None,
        };
        let qs = RestClient::serialize_query_string(&params).unwrap();
        assert_eq!(qs, "?instId=BTC-USDT");
    }

    #[test]
    fn test_inject_program_tag_object() {
        let val = serde_json::json!({"instId": "BTC-USDT", "sz": "1"});
        let result = inject_program_tag(&val).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert_eq!(
            parsed["tag"],
            serde_json::Value::String(constants::PROGRAM_ID.to_string())
        );
    }

    #[test]
    fn test_inject_program_tag_array() {
        let val = serde_json::json!([{"instId": "BTC-USDT"}, {"instId": "ETH-USDT"}]);
        let result = inject_program_tag(&val).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert_eq!(
            parsed[0]["tag"],
            serde_json::Value::String(constants::PROGRAM_ID.to_string())
        );
        assert_eq!(
            parsed[1]["tag"],
            serde_json::Value::String(constants::PROGRAM_ID.to_string())
        );
    }

    #[test]
    fn test_inject_program_tag_preserves_existing() {
        let val = serde_json::json!({"instId": "BTC-USDT", "tag": "custom"});
        let result = inject_program_tag(&val).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert_eq!(
            parsed["tag"],
            serde_json::Value::String("custom".to_string())
        );
    }
}
