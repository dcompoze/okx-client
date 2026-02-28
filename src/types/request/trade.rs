use serde::Serialize;

use crate::types::enums::*;

fn serialize_csv<S>(values: &[String], serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&values.join(","))
}

/// Place a single order.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct OrderRequest {
    /// Instrument ID, e.g. "BTC-USDT".
    pub inst_id: String,
    /// Trade mode: cross, isolated, cash, spot_isolated.
    pub td_mode: TradeMode,
    /// Margin currency. Only applicable to cross MARGIN orders in single-currency margin.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    /// Client Order ID as assigned by the client.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cl_ord_id: Option<String>,
    /// Order tag. A combination of case-sensitive alphanumerics, all numbers, or all letters of up to 16 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// Order side: buy or sell.
    pub side: OrderSide,
    /// Position side: net, long, or short.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos_side: Option<PositionSide>,
    /// Order type: market, limit, post_only, fok, ioc, etc.
    pub ord_type: OrderType,
    /// Quantity to buy or sell.
    pub sz: String,
    /// Order price. Only applicable to limit, post_only, fok, ioc order types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub px: Option<String>,
    /// Whether orders can only reduce position size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    /// Target currency for the quantity: base_ccy or quote_ccy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tgt_ccy: Option<String>,
    /// Whether to disallow the system from amending the size of the SPOT Market Order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ban_amend: Option<bool>,
    /// Self trade prevention ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stp_id: Option<String>,
    /// Self trade prevention mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stp_mode: Option<StpMode>,
    /// Take-profit trigger price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_trigger_px: Option<String>,
    /// Take-profit order price. If the price is -1, take-profit will be executed at market price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_ord_px: Option<String>,
    /// Stop-loss trigger price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_trigger_px: Option<String>,
    /// Stop-loss order price. If the price is -1, stop-loss will be executed at market price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_ord_px: Option<String>,
    /// Take-profit trigger price type: last, index, mark.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_trigger_px_type: Option<String>,
    /// Stop-loss trigger price type: last, index, mark.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_trigger_px_type: Option<String>,
    /// Expiration time, a Unix timestamp in milliseconds. Order will be cancelled after expiry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_time: Option<String>,
}

/// Cancel a single order.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderRequest {
    /// Instrument ID, e.g. "BTC-USDT".
    pub inst_id: String,
    /// Order ID. Either ordId or clOrdId is required; if both are passed, ordId will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ord_id: Option<String>,
    /// Client Order ID as assigned by the client.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cl_ord_id: Option<String>,
}

/// Amend an existing order.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AmendOrderRequest {
    /// Instrument ID, e.g. "BTC-USDT".
    pub inst_id: String,
    /// Whether the order needs to be automatically cancelled when amendment fails.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cxl_on_fail: Option<bool>,
    /// Order ID. Either ordId or clOrdId is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ord_id: Option<String>,
    /// Client Order ID as assigned by the client.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cl_ord_id: Option<String>,
    /// Client Request ID as assigned by the client for order amendment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub req_id: Option<String>,
    /// New quantity after amendment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_sz: Option<String>,
    /// New price after amendment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_px: Option<String>,
    /// New take-profit trigger price after amendment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_tp_trigger_px: Option<String>,
    /// New take-profit order price after amendment. -1 for market price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_tp_ord_px: Option<String>,
    /// New stop-loss trigger price after amendment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_sl_trigger_px: Option<String>,
    /// New stop-loss order price after amendment. -1 for market price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_sl_ord_px: Option<String>,
    /// New take-profit trigger price type after amendment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_tp_trigger_px_type: Option<String>,
    /// New stop-loss trigger price type after amendment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_sl_trigger_px_type: Option<String>,
}

/// Close a position.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClosePositionRequest {
    /// Instrument ID, e.g. "BTC-USDT-SWAP".
    pub inst_id: String,
    /// Margin mode: cross or isolated.
    pub mgn_mode: MarginMode,
    /// Position side: net, long, or short. Default is net.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos_side: Option<PositionSide>,
    /// Margin currency. Required for cross MARGIN orders in single-currency margin.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    /// Whether to automatically cancel all pending orders of the instrument after closing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_cxl: Option<bool>,
    /// Client Order ID as assigned by the client.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cl_ord_id: Option<String>,
    /// Order tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

/// Get details of a single order.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderRequest {
    /// Instrument ID, e.g. "BTC-USDT".
    pub inst_id: String,
    /// Order ID. Either ordId or clOrdId is required; if both are passed, ordId will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ord_id: Option<String>,
    /// Client Order ID as assigned by the client.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cl_ord_id: Option<String>,
}

/// Get a list of pending (unfilled/partially filled) orders.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderListRequest {
    /// Instrument type: SPOT, MARGIN, SWAP, FUTURES, OPTION.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<InstrumentType>,
    /// Underlying, e.g. "BTC-USD". Applicable to FUTURES/SWAP/OPTION.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uly: Option<String>,
    /// Instrument family, e.g. "BTC-USD". Applicable to FUTURES/SWAP/OPTION.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,
    /// Instrument ID, e.g. "BTC-USDT".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    /// Order type: market, limit, post_only, fok, ioc, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ord_type: Option<OrderType>,
    /// Order state: live, partially_filled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Pagination of data to return records earlier than the requested ordId.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Pagination of data to return records newer than the requested ordId.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Number of results per request. Maximum 100; default 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Get order history (last 7 days or last 3 months depending on endpoint).
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderHistoryRequest {
    /// Instrument type: SPOT, MARGIN, SWAP, FUTURES, OPTION.
    pub inst_type: InstrumentType,
    /// Underlying, e.g. "BTC-USD". Applicable to FUTURES/SWAP/OPTION.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uly: Option<String>,
    /// Instrument family, e.g. "BTC-USD". Applicable to FUTURES/SWAP/OPTION.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,
    /// Instrument ID, e.g. "BTC-USDT".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    /// Order type: market, limit, post_only, fok, ioc, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ord_type: Option<OrderType>,
    /// Order state: canceled, filled, mmp_canceled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Category: twap, adl, full_liquidation, partial_liquidation, delivery, ddh.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// Pagination of data to return records earlier than the requested ordId.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Pagination of data to return records newer than the requested ordId.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Number of results per request. Maximum 100; default 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
    /// Filter with a begin timestamp (Unix timestamp in milliseconds).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin: Option<String>,
    /// Filter with an end timestamp (Unix timestamp in milliseconds).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
}

/// Get transaction (fill) details.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetFillsRequest {
    /// Instrument type: SPOT, MARGIN, SWAP, FUTURES, OPTION.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<InstrumentType>,
    /// Underlying, e.g. "BTC-USD". Applicable to FUTURES/SWAP/OPTION.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uly: Option<String>,
    /// Instrument family, e.g. "BTC-USD". Applicable to FUTURES/SWAP/OPTION.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,
    /// Instrument ID, e.g. "BTC-USDT".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    /// Order ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ord_id: Option<String>,
    /// Pagination of data to return records earlier than the requested billId.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Pagination of data to return records newer than the requested billId.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Number of results per request. Maximum 100; default 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
    /// Filter with a begin timestamp (Unix timestamp in milliseconds).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin: Option<String>,
    /// Filter with an end timestamp (Unix timestamp in milliseconds).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
}

/// Place an algo order (e.g. trigger, OCO, conditional, iceberg, TWAP).
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AlgoOrderRequest {
    /// Instrument ID, e.g. "BTC-USDT".
    pub inst_id: String,
    /// Trade mode: cross, isolated, cash, spot_isolated.
    pub td_mode: TradeMode,
    /// Margin currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    /// Order side: buy or sell.
    pub side: OrderSide,
    /// Position side: net, long, or short.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos_side: Option<PositionSide>,
    /// Algo order type: conditional, oco, trigger, move_order_stop, iceberg, twap, chase.
    pub ord_type: String,
    /// Quantity to buy or sell.
    pub sz: String,
    /// Order tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// Target currency for the quantity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tgt_ccy: Option<String>,
    /// Whether orders can only reduce position size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    /// Take-profit trigger price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_trigger_px: Option<String>,
    /// Take-profit order price. -1 for market price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_ord_px: Option<String>,
    /// Stop-loss trigger price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_trigger_px: Option<String>,
    /// Stop-loss order price. -1 for market price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_ord_px: Option<String>,
    /// Trigger price for trigger orders.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_px: Option<String>,
    /// Order price for trigger orders. -1 for market price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_px: Option<String>,
    /// Take-profit trigger price type: last, index, mark.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_trigger_px_type: Option<String>,
    /// Stop-loss trigger price type: last, index, mark.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_trigger_px_type: Option<String>,
    /// Trigger price type: last, index, mark.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_px_type: Option<String>,
    /// Price ratio for iceberg/TWAP. Either pxVar or pxSpread is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub px_var: Option<String>,
    /// Price variance for iceberg/TWAP. Either pxVar or pxSpread is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub px_spread: Option<String>,
    /// Average amount for each order in iceberg/TWAP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sz_limit: Option<String>,
    /// Price limit for iceberg/TWAP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub px_limit: Option<String>,
    /// Time interval in seconds for TWAP orders.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_interval: Option<String>,
}

/// Cancel an algo order.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CancelAlgoOrderRequest {
    /// Instrument ID, e.g. "BTC-USDT".
    pub inst_id: String,
    /// Algo order ID.
    pub algo_id: String,
}

/// Amend an algo order.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AmendAlgoOrderRequest {
    /// Instrument ID, e.g. "BTC-USDT".
    pub inst_id: String,
    /// Algo order ID.
    pub algo_id: String,
    /// Whether the order needs to be automatically cancelled when amendment fails.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cxl_on_fail: Option<bool>,
    /// Client Request ID as assigned by the client for order amendment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub req_id: Option<String>,
    /// New quantity after amendment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_sz: Option<String>,
    /// New take-profit trigger price after amendment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_tp_trigger_px: Option<String>,
    /// New take-profit order price after amendment. -1 for market price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_tp_ord_px: Option<String>,
    /// New stop-loss trigger price after amendment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_sl_trigger_px: Option<String>,
    /// New stop-loss order price after amendment. -1 for market price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_sl_ord_px: Option<String>,
    /// New take-profit trigger price type after amendment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_tp_trigger_px_type: Option<String>,
    /// New stop-loss trigger price type after amendment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_sl_trigger_px_type: Option<String>,
}

/// Get details of a single algo order.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetAlgoOrderRequest {
    /// Algo order ID. Either algoId or algoClOrdId is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algo_id: Option<String>,
    /// Client-supplied Algo Order ID. Either algoId or algoClOrdId is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algo_cl_ord_id: Option<String>,
}

/// Get a list of pending algo orders.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetAlgoOrderListRequest {
    /// Algo order type: conditional, oco, trigger, move_order_stop, iceberg, twap, chase.
    pub ord_type: String,
    /// Algo order ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algo_id: Option<String>,
    /// Instrument type: SPOT, MARGIN, SWAP, FUTURES, OPTION.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<InstrumentType>,
    /// Instrument ID, e.g. "BTC-USDT".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    /// Pagination of data to return records earlier than the requested algoId.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Pagination of data to return records newer than the requested algoId.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Number of results per request. Maximum 100; default 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Mass cancel orders for an instrument type.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MassCancelRequest {
    /// Instrument type: SWAP, FUTURES, OPTION.
    pub inst_type: InstrumentType,
    /// Instrument family, e.g. "BTC-USD".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,
}

/// Cancel all orders after a specified countdown timer.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllAfterRequest {
    /// The countdown timer value in seconds. Setting "0" will effectively cancel the timer.
    pub time_out: String,
    /// Order tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

/// Easy convert request.
///
/// Convert small assets into OKB.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EasyConvertRequest {
    /// Source currencies to convert from. Comma-separated list.
    #[serde(serialize_with = "serialize_csv")]
    pub from_ccy: Vec<String>,
    /// Target currency to convert to.
    pub to_ccy: String,
}

/// Get easy convert history request.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetEasyConvertHistoryRequest {
    /// Pagination of data to return records earlier than the requested timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Pagination of data to return records newer than the requested timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Number of results per request. Maximum 100; default 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// One-click repay request.
///
/// Repay cross margin debt with a single click.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct OneClickRepayRequest {
    /// Currencies with debt to repay. Comma-separated list.
    #[serde(serialize_with = "serialize_csv")]
    pub debt_ccy: Vec<String>,
    /// Currency to use for repayment.
    pub repay_ccy: String,
}

/// Get one-click repay history request.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetOneClickRepayHistoryRequest {
    /// Pagination of data to return records earlier than the requested timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Pagination of data to return records newer than the requested timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Number of results per request. Maximum 100; default 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn easy_convert_serializes_currency_list_as_csv() {
        let req = EasyConvertRequest {
            from_ccy: vec!["BTC".into(), "ETH".into()],
            to_ccy: "USDT".into(),
        };

        let value = serde_json::to_value(req).unwrap();
        assert_eq!(value["fromCcy"], "BTC,ETH");
    }

    #[test]
    fn one_click_repay_serializes_currency_list_as_csv() {
        let req = OneClickRepayRequest {
            debt_ccy: vec!["BTC".into(), "ETH".into()],
            repay_ccy: "USDT".into(),
        };

        let value = serde_json::to_value(req).unwrap();
        assert_eq!(value["debtCcy"], "BTC,ETH");
    }
}
