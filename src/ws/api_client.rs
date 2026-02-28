use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::constants::PROGRAM_ID;
use crate::error::{OkxError, OkxResult};
use crate::types::request::trade::{
    AmendOrderRequest, CancelOrderRequest, MassCancelRequest, OrderRequest,
};
use crate::types::response::trade::{AmendedOrder, CancelledOrder, MassCancelResult, OrderResult};
use crate::types::ws::responses::{
    WsSpreadAmendResult, WsSpreadCancelResult, WsSpreadOrderResult,
};
use crate::ws::types::WsConfig;
use crate::ws::WebsocketClient;

/// Typed WebSocket API client for order management.
///
/// Wraps [`WebsocketClient`] and exposes typed methods for order placement,
/// cancellation, and amendment over the private WebSocket connection.
///
/// Operations that succeed at the transport level but fail per-item (e.g. batch
/// orders where one leg is rejected) return the full result vec; callers should
/// inspect `s_code` on each item.
///
/// # Example
///
/// ```no_run
/// use okx_client::ws::api_client::WsApiClient;
/// use okx_client::ws::types::WsConfig;
/// use okx_client::types::request::trade::OrderRequest;
/// use okx_client::types::enums::{OrderSide, OrderType, TradeMode};
///
/// # async fn example() -> okx_client::error::OkxResult<()> {
/// let client = WsApiClient::new(WsConfig::default());
/// let req = OrderRequest {
///     inst_id: "BTC-USDT".into(),
///     td_mode: TradeMode::Cash,
///     side: OrderSide::Buy,
///     ord_type: OrderType::Market,
///     sz: "0.001".into(),
///     ..Default::default()
/// };
/// let result = client.place_order(req).await?;
/// println!("{result:?}");
/// # Ok(())
/// # }
/// ```
pub struct WsApiClient {
    inner: WebsocketClient,
}

impl WsApiClient {
    /// Create a new `WsApiClient` with the given configuration.
    pub fn new(config: WsConfig) -> Self {
        Self {
            inner: WebsocketClient::new(config),
        }
    }

    /// Create a `WsApiClient` from an existing [`WebsocketClient`].
    ///
    /// The two clients share the same underlying connections.
    pub fn from_client(client: WebsocketClient) -> Self {
        Self { inner: client }
    }

    /// Access the underlying [`WebsocketClient`].
    pub fn ws_client(&self) -> &WebsocketClient {
        &self.inner
    }

    /// Place a single order.
    /// WS operation: `order`
    pub async fn place_order(&self, req: OrderRequest) -> OkxResult<OrderResult> {
        let arg = to_tagged_value(&req)?;
        let resp = self.inner.send_api_request("order", vec![arg]).await?;
        deserialize_first(&resp.data)
    }

    /// Place multiple orders (up to 20).
    /// WS operation: `batch-orders`
    pub async fn place_orders(&self, reqs: Vec<OrderRequest>) -> OkxResult<Vec<OrderResult>> {
        let args = reqs
            .iter()
            .map(to_tagged_value)
            .collect::<OkxResult<Vec<_>>>()?;
        let resp = self.inner.send_api_request("batch-orders", args).await?;
        deserialize_all(&resp.data)
    }

    /// Cancel a single order.
    /// WS operation: `cancel-order`
    pub async fn cancel_order(&self, req: CancelOrderRequest) -> OkxResult<CancelledOrder> {
        let arg = serde_json::to_value(&req)?;
        let resp = self
            .inner
            .send_api_request("cancel-order", vec![arg])
            .await?;
        deserialize_first(&resp.data)
    }

    /// Cancel multiple orders (up to 20).
    /// WS operation: `batch-cancel-orders`
    pub async fn cancel_orders(
        &self,
        reqs: Vec<CancelOrderRequest>,
    ) -> OkxResult<Vec<CancelledOrder>> {
        let args = reqs
            .iter()
            .map(|r| serde_json::to_value(r).map_err(OkxError::Serialization))
            .collect::<OkxResult<Vec<_>>>()?;
        let resp = self
            .inner
            .send_api_request("batch-cancel-orders", args)
            .await?;
        deserialize_all(&resp.data)
    }

    /// Amend a single order.
    /// WS operation: `amend-order`
    pub async fn amend_order(&self, req: AmendOrderRequest) -> OkxResult<AmendedOrder> {
        let arg = serde_json::to_value(&req)?;
        let resp = self
            .inner
            .send_api_request("amend-order", vec![arg])
            .await?;
        deserialize_first(&resp.data)
    }

    /// Amend multiple orders (up to 20).
    /// WS operation: `batch-amend-orders`
    pub async fn amend_orders(
        &self,
        reqs: Vec<AmendOrderRequest>,
    ) -> OkxResult<Vec<AmendedOrder>> {
        let args = reqs
            .iter()
            .map(|r| serde_json::to_value(r).map_err(OkxError::Serialization))
            .collect::<OkxResult<Vec<_>>>()?;
        let resp = self
            .inner
            .send_api_request("batch-amend-orders", args)
            .await?;
        deserialize_all(&resp.data)
    }

    /// Mass cancel orders by instrument type and family.
    /// WS operation: `mass-cancel`
    pub async fn mass_cancel(&self, req: MassCancelRequest) -> OkxResult<MassCancelResult> {
        let arg = serde_json::to_value(&req)?;
        let resp = self
            .inner
            .send_api_request("mass-cancel", vec![arg])
            .await?;
        deserialize_first(&resp.data)
    }

    /// Place a spread order.
    /// WS operation: `sprd-order`
    pub async fn place_spread_order(
        &self,
        req: serde_json::Value,
    ) -> OkxResult<WsSpreadOrderResult> {
        let arg = to_tagged_value_raw(req)?;
        let resp = self.inner.send_api_request("sprd-order", vec![arg]).await?;
        deserialize_first(&resp.data)
    }

    /// Cancel a spread order.
    /// WS operation: `sprd-cancel-order`
    pub async fn cancel_spread_order(
        &self,
        req: serde_json::Value,
    ) -> OkxResult<WsSpreadCancelResult> {
        let resp = self
            .inner
            .send_api_request("sprd-cancel-order", vec![req])
            .await?;
        deserialize_first(&resp.data)
    }

    /// Amend a spread order.
    /// WS operation: `sprd-amend-order`
    pub async fn amend_spread_order(
        &self,
        req: serde_json::Value,
    ) -> OkxResult<WsSpreadAmendResult> {
        let resp = self
            .inner
            .send_api_request("sprd-amend-order", vec![req])
            .await?;
        deserialize_first(&resp.data)
    }

    /// Mass cancel all spread orders.
    /// WS operation: `sprd-mass-cancel`
    pub async fn mass_cancel_spread_orders(
        &self,
        req: serde_json::Value,
    ) -> OkxResult<MassCancelResult> {
        let resp = self
            .inner
            .send_api_request("sprd-mass-cancel", vec![req])
            .await?;
        deserialize_first(&resp.data)
    }
}

/// Serialize a value and inject the OKX program tag if not already present.
fn to_tagged_value(v: &impl Serialize) -> OkxResult<serde_json::Value> {
    let mut value = serde_json::to_value(v)?;
    inject_tag(&mut value);
    Ok(value)
}

/// Inject the OKX program tag into a raw JSON value if not already present.
fn to_tagged_value_raw(mut value: serde_json::Value) -> OkxResult<serde_json::Value> {
    inject_tag(&mut value);
    Ok(value)
}

/// Add `tag: PROGRAM_ID` to a JSON object if the key is absent.
fn inject_tag(value: &mut serde_json::Value) {
    if let serde_json::Value::Object(map) = value {
        map.entry("tag")
            .or_insert_with(|| serde_json::json!(PROGRAM_ID));
    }
}

/// Deserialize the first element of a WS API response data array.
fn deserialize_first<T: DeserializeOwned>(data: &[serde_json::Value]) -> OkxResult<T> {
    let v = data
        .first()
        .ok_or_else(|| OkxError::Ws("empty response data".into()))?;
    serde_json::from_value(v.clone()).map_err(OkxError::Serialization)
}

/// Deserialize all elements of a WS API response data array.
fn deserialize_all<T: DeserializeOwned>(data: &[serde_json::Value]) -> OkxResult<Vec<T>> {
    data.iter()
        .map(|v| serde_json::from_value(v.clone()).map_err(OkxError::Serialization))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inject_tag_adds_when_absent() {
        let mut v = serde_json::json!({"instId": "BTC-USDT"});
        inject_tag(&mut v);
        assert_eq!(v["tag"], serde_json::json!(PROGRAM_ID));
    }

    #[test]
    fn inject_tag_does_not_overwrite() {
        let mut v = serde_json::json!({"tag": "custom"});
        inject_tag(&mut v);
        assert_eq!(v["tag"], serde_json::json!("custom"));
    }

    #[test]
    fn to_tagged_value_injects_tag() {
        let req = OrderRequest {
            inst_id: "BTC-USDT".into(),
            ..Default::default()
        };
        let v = to_tagged_value(&req).unwrap();
        assert_eq!(v["tag"], serde_json::json!(PROGRAM_ID));
    }
}
