use serde::Deserialize;

/// Result from placing a single order.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct OrderResult {
    /// Client Order ID as assigned by the client.
    pub cl_ord_id: String,
    /// Order ID assigned by OKX.
    pub ord_id: String,
    /// Order tag.
    pub tag: String,
    /// Timestamp when the order request was received, Unix timestamp in milliseconds.
    pub ts: String,
    /// The code of the event execution result, 0 means success.
    pub s_code: String,
    /// Rejection or success message of event execution.
    pub s_msg: String,
}

/// Result from cancelling a single order.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct CancelledOrder {
    /// Client Order ID as assigned by the client.
    pub cl_ord_id: String,
    /// Order ID.
    pub ord_id: String,
    /// The code of the event execution result, 0 means success.
    pub s_code: String,
    /// Rejection or success message of event execution.
    pub s_msg: String,
}

/// Result from amending an order.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AmendedOrder {
    /// Client Order ID as assigned by the client.
    pub cl_ord_id: String,
    /// Order ID.
    pub ord_id: String,
    /// Client Request ID as assigned by the client for order amendment.
    pub req_id: String,
    /// The code of the event execution result, 0 means success.
    pub s_code: String,
    /// Rejection or success message of event execution.
    pub s_msg: String,
}

/// Full details of an order.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct OrderDetails {
    /// Instrument type.
    pub inst_type: String,
    /// Instrument ID.
    pub inst_id: String,
    /// Margin currency.
    pub ccy: String,
    /// Order ID.
    pub ord_id: String,
    /// Client Order ID as assigned by the client.
    pub cl_ord_id: String,
    /// Order tag.
    pub tag: String,
    /// Price.
    pub px: String,
    /// Quantity to buy or sell.
    pub sz: String,
    /// Profit and loss, applicable to closing orders.
    pub pnl: String,
    /// Order type.
    pub ord_type: String,
    /// Order side: buy, sell.
    pub side: String,
    /// Position side: net, long, short.
    pub pos_side: String,
    /// Trade mode: cross, isolated, cash.
    pub td_mode: String,
    /// Accumulated fill quantity.
    pub acc_fill_sz: String,
    /// Last filled price.
    pub fill_px: String,
    /// Last trade ID.
    pub trade_id: String,
    /// Last filled quantity.
    pub fill_sz: String,
    /// Last filled time.
    pub fill_time: String,
    /// Order state: canceled, live, partially_filled, filled, mmp_canceled.
    pub state: String,
    /// Average filled price. If none is filled, it will return "".
    pub avg_px: String,
    /// Leverage. Not applicable to SPOT, empty if not applicable.
    pub lever: String,
    /// Fee currency.
    pub fee_ccy: String,
    /// Fee and target rebate. Negative value means fee charged; positive means rebate.
    pub fee: String,
    /// Rebate currency.
    pub rebate_ccy: String,
    /// Rebate amount.
    pub rebate: String,
    /// Order source.
    pub source: String,
    /// Category: normal, twap, adl, full_liquidation, partial_liquidation, delivery, ddh.
    pub category: String,
    /// Update time, Unix timestamp in milliseconds.
    pub u_time: String,
    /// Creation time, Unix timestamp in milliseconds.
    pub c_time: String,
    /// Cancel source. Valid when the order is canceled.
    pub cancel_source: String,
    /// Take-profit trigger price.
    pub tp_trigger_px: String,
    /// Take-profit trigger price type: last, index, mark.
    pub tp_trigger_px_type: String,
    /// Take-profit order price.
    pub tp_ord_px: String,
    /// Stop-loss trigger price.
    pub sl_trigger_px: String,
    /// Stop-loss trigger price type: last, index, mark.
    pub sl_trigger_px_type: String,
    /// Stop-loss order price.
    pub sl_ord_px: String,
    /// Self trade prevention ID.
    pub stp_id: String,
    /// Self trade prevention mode.
    pub stp_mode: String,
    /// Whether the order can only reduce position size.
    pub reduce_only: String,
}

/// Fill / trade record.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Fill {
    /// Instrument type.
    pub inst_type: String,
    /// Instrument ID.
    pub inst_id: String,
    /// Trade ID.
    pub trade_id: String,
    /// Order ID.
    pub ord_id: String,
    /// Client Order ID as assigned by the client.
    pub cl_ord_id: String,
    /// Bill ID.
    pub bill_id: String,
    /// Order tag.
    pub tag: String,
    /// Last filled price.
    pub fill_px: String,
    /// Last filled quantity.
    pub fill_sz: String,
    /// Order side: buy, sell.
    pub side: String,
    /// Position side: net, long, short.
    pub pos_side: String,
    /// Execution type: T (taker), M (maker).
    pub exec_type: String,
    /// Fee currency.
    pub fee_ccy: String,
    /// Fee. Negative means fee charged; positive means rebate.
    pub fee: String,
    /// Timestamp of the data generation, Unix timestamp in milliseconds.
    pub ts: String,
    /// Last filled time.
    pub fill_time: String,
    /// Last filled profit and loss.
    pub fill_pnl: String,
    /// Implied volatility when filled, only applicable to options.
    pub fill_px_vol: String,
    /// Options price when filled, in USD, only applicable to options.
    pub fill_px_usd: String,
    /// Mark volatility when filled, only applicable to options.
    pub fill_mark_vol: String,
    /// Forward price when filled, only applicable to options.
    pub fill_fwd_px: String,
    /// Mark price when filled.
    pub fill_mark_px: String,
}

/// Result from placing an algo order.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AlgoOrderResult {
    /// Algo order ID.
    pub algo_id: String,
    /// Client-supplied Algo Order ID.
    pub algo_cl_ord_id: String,
    /// The code of the event execution result, 0 means success.
    pub s_code: String,
    /// Rejection or success message of event execution.
    pub s_msg: String,
}

/// Full details of an algo order.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AlgoOrderDetails {
    /// Instrument type.
    pub inst_type: String,
    /// Instrument ID.
    pub inst_id: String,
    /// Order ID. Only applicable to TP/SL order that has been triggered.
    pub ord_id: String,
    /// Algo order ID.
    pub algo_id: String,
    /// Client Order ID as assigned by the client.
    pub cl_ord_id: String,
    /// Margin currency.
    pub ccy: String,
    /// Quantity to buy or sell.
    pub sz: String,
    /// Algo order type.
    pub ord_type: String,
    /// Order side: buy, sell.
    pub side: String,
    /// Position side: net, long, short.
    pub pos_side: String,
    /// Trade mode: cross, isolated, cash.
    pub td_mode: String,
    /// Algo order state.
    pub state: String,
    /// Leverage.
    pub lever: String,
    /// Take-profit trigger price.
    pub tp_trigger_px: String,
    /// Take-profit order price.
    pub tp_ord_px: String,
    /// Stop-loss trigger price.
    pub sl_trigger_px: String,
    /// Stop-loss order price.
    pub sl_ord_px: String,
    /// Trigger price.
    pub trigger_px: String,
    /// Order price.
    pub ord_px: String,
    /// Actual order quantity.
    pub actual_sz: String,
    /// Actual order price.
    pub actual_px: String,
    /// Actual order side.
    pub actual_side: String,
    /// Trigger time, Unix timestamp in milliseconds.
    pub trigger_time: String,
    /// Creation time, Unix timestamp in milliseconds.
    pub c_time: String,
}

/// Result from mass cancel operation.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct MassCancelResult {
    /// Whether the mass cancel was successful. "true" or "false".
    pub result: String,
}
