use serde::Serialize;

use crate::types::enums::*;

/// Get balance request.
///
/// Retrieve a list of assets (with non-zero balance), remaining balance,
/// and available amount in the trading account.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetBalanceRequest {
    /// Single currency or comma-separated list of currencies, e.g. "BTC" or "BTC,ETH".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
}

/// Get positions request.
///
/// Retrieve information on your positions. When the account is in `net` mode,
/// `net` positions will be displayed, and when the account is in `long/short`
/// mode, `long` or `short` positions will be displayed.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetPositionsRequest {
    /// Instrument type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<InstrumentType>,
    /// Instrument ID, e.g. "BTC-USDT-SWAP".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    /// Position ID. Supports multiple IDs separated by commas (max 20).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos_id: Option<String>,
}

/// Get positions history request.
///
/// Retrieve the updated position data for the last 3 months.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetPositionsHistoryRequest {
    /// Instrument type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<InstrumentType>,
    /// Instrument ID, e.g. "BTC-USD-SWAP".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    /// Margin mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mgn_mode: Option<MarginMode>,
    /// The type of closing position: 1 - Close position, 2 - Partially closed,
    /// 3 - Liquidation, 4 - Partial liquidation, 5 - ADL.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// Position ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos_id: Option<String>,
    /// Pagination of data to return records earlier than the requested `posId`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Pagination of data to return records newer than the requested `posId`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Number of results per request. Maximum 100; default 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Get account position risk request.
///
/// Get account position risk data.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetAccountPositionRiskRequest {
    /// Instrument type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<InstrumentType>,
}

/// Get bills detail request.
///
/// Retrieve the bills of the account. The bill refers to all transaction records
/// that result in changing of the balance of an account.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetBillsRequest {
    /// Instrument type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<InstrumentType>,
    /// Currency, e.g. "BTC".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    /// Margin mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mgn_mode: Option<MarginMode>,
    /// Contract type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ct_type: Option<String>,
    /// Bill type.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// Bill sub-type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_type: Option<String>,
    /// Pagination of data to return records earlier than the requested `billId`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Pagination of data to return records newer than the requested `billId`.
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

/// Set position mode request.
///
/// Set the position mode: `long_short_mode` or `net_mode`.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetPositionModeRequest {
    /// Position mode.
    pub pos_mode: PosMode,
}

/// Set leverage request.
///
/// Set the leverage for an instrument or a currency in a given margin mode.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetLeverageRequest {
    /// Instrument ID, e.g. "BTC-USDT-SWAP".
    /// Required when setting leverage for a specific instrument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    /// Currency, e.g. "BTC". Required when setting leverage for a currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    /// Leverage value.
    pub lever: String,
    /// Margin mode.
    pub mgn_mode: MarginMode,
    /// Position side. Required in `long_short_mode` under `cross` margin mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos_side: Option<PositionSide>,
}

/// Get leverage request.
///
/// Get the leverage of an instrument.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetLeverageRequest {
    /// Instrument ID, e.g. "BTC-USDT-SWAP".
    pub inst_id: String,
    /// Margin mode.
    pub mgn_mode: MarginMode,
}

/// Get maximum buy/sell amount request.
///
/// Get the maximum tradeable amount for an instrument.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetMaxBuySellAmountRequest {
    /// Instrument ID, e.g. "BTC-USDT".
    pub inst_id: String,
    /// Trade mode: "cross", "isolated", "cash".
    pub td_mode: String,
    /// Currency, e.g. "BTC". Required in `cross` mode for `SPOT/MARGIN`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    /// Price. Influences max buy amount in certain modes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub px: Option<String>,
    /// Leverage for the instrument.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leverage: Option<String>,
    /// Whether to offset with spot positions. `true` or `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub un_spot_offset: Option<String>,
}

/// Get maximum available tradeable size request.
///
/// Get the maximum available size for an instrument.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetMaxAvailSizeRequest {
    /// Instrument ID, e.g. "BTC-USDT".
    pub inst_id: String,
    /// Trade mode: "cross", "isolated", "cash".
    pub td_mode: String,
    /// Currency, e.g. "BTC".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    /// Whether to reduce position only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    /// Whether to offset with spot positions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub un_spot_offset: Option<String>,
    /// Quick margin type. Only applicable to Quick Margin Mode of isolated margin.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quick_mgn_type: Option<String>,
}

/// Get maximum loan request.
///
/// Get the maximum loan amount for an instrument.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetMaxLoanRequest {
    /// Instrument ID, e.g. "BTC-USDT".
    pub inst_id: String,
    /// Margin mode.
    pub mgn_mode: MarginMode,
    /// Margin currency. Required for cross margin mode in
    /// multi-currency margin and portfolio margin.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mgn_ccy: Option<String>,
}

/// Get fee rates request.
///
/// Get the trading fee rate for an instrument type.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetFeeRatesRequest {
    /// Instrument type.
    pub inst_type: InstrumentType,
    /// Instrument ID, e.g. "BTC-USDT". Only applicable to `SPOT`/`MARGIN`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    /// Underlying, e.g. "BTC-USD". Only applicable to `FUTURES`/`SWAP`/`OPTION`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uly: Option<String>,
    /// Instrument family, e.g. "BTC-USD". Only applicable to `FUTURES`/`SWAP`/`OPTION`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,
}

/// Get interest accrued request.
///
/// Get the interest accrued data.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetInterestAccruedRequest {
    /// Instrument ID, e.g. "BTC-USDT".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    /// Currency, e.g. "BTC".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    /// Margin mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mgn_mode: Option<MarginMode>,
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

/// Set greeks display type request.
///
/// Set the display type of Greeks.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetGreeksRequest {
    /// Display type of Greeks.
    pub greeks_type: GreeksType,
}

/// Set isolated margin trading mode request.
///
/// Set the isolated margin trading settings for a given instrument type.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetIsolatedModeRequest {
    /// Isolated margin trading settings: "automatic" or "autonomy".
    pub iso_mode: String,
    /// Instrument type: "MARGIN" or "CONTRACTS".
    #[serde(rename = "type")]
    pub type_: String,
}

/// Get maximum withdrawals request.
///
/// Retrieve the maximum transferable amount.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetMaxWithdrawalsRequest {
    /// Currency, e.g. "BTC".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
}

/// Set MMP (Market Maker Protection) config request.
///
/// Set the MMP configuration for a given instrument family.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetMmpConfigRequest {
    /// Instrument family, e.g. "BTC-USD".
    pub inst_family: String,
    /// Time window (ms). MMP will be triggered if the order count exceeds
    /// `qtyLimit` within the time window.
    pub time_interval: String,
    /// Frozen period (ms). MMP frozen time after being triggered.
    pub frozen_interval: String,
    /// Quantity limit. The number of contracts that can be traded within the time window.
    pub qty_limit: String,
}

/// Get MMP config request.
///
/// Get the MMP configuration.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetMmpConfigRequest {
    /// Instrument family, e.g. "BTC-USD".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,
}

/// Set account level request.
///
/// Set the account level.
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetAccountLevelRequest {
    /// Account level: "1" - Simple, "2" - Single-currency margin,
    /// "3" - Multi-currency margin, "4" - Portfolio margin.
    pub acct_lv: String,
}
