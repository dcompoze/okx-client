use serde::Deserialize;

/// Full account balance information.
///
/// Contains overall account equity, margin, and per-currency balance details.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AccountBalance {
    /// Update time, Unix timestamp in milliseconds.
    #[serde(default)]
    pub u_time: String,
    /// Total equity in USD.
    #[serde(default)]
    pub total_eq: String,
    /// Isolated margin equity in USD.
    #[serde(default)]
    pub iso_eq: String,
    /// Adjusted / effective equity in USD.
    #[serde(default)]
    pub adj_eq: String,
    /// Cross margin frozen for pending orders.
    #[serde(default)]
    pub ord_froz: String,
    /// Initial margin requirement.
    #[serde(default)]
    pub imr: String,
    /// Maintenance margin requirement.
    #[serde(default)]
    pub mmr: String,
    /// Notional value of positions in USD.
    #[serde(default)]
    pub notional_usd: String,
    /// Per-currency balance details.
    #[serde(default)]
    pub details: Vec<BalanceDetail>,
}

/// Per-currency balance detail.
///
/// Provides detailed balance information for a single currency within the
/// trading account.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct BalanceDetail {
    /// Currency, e.g. "BTC".
    #[serde(default)]
    pub ccy: String,
    /// Equity of the currency.
    #[serde(default)]
    pub eq: String,
    /// Cash balance.
    #[serde(default)]
    pub cash_bal: String,
    /// Update time, Unix timestamp in milliseconds.
    #[serde(default)]
    pub u_time: String,
    /// Isolated margin equity of the currency.
    #[serde(default)]
    pub iso_eq: String,
    /// Available equity of the currency.
    #[serde(default)]
    pub avail_eq: String,
    /// Discount equity of the currency in USD.
    #[serde(default)]
    pub dis_eq: String,
    /// Available balance of the currency.
    #[serde(default)]
    pub avail_bal: String,
    /// Frozen balance of the currency.
    #[serde(default)]
    pub frozen_bal: String,
    /// Margin frozen for open orders.
    #[serde(default)]
    pub ord_frozen: String,
    /// Liabilities of the currency.
    #[serde(default)]
    pub liab: String,
    /// Unrealized profit and loss.
    #[serde(default)]
    pub upl: String,
    /// Unrealized profit and loss for liabilities.
    #[serde(default)]
    pub upl_liab: String,
    /// Cross liabilities of the currency.
    #[serde(default)]
    pub cross_liab: String,
    /// Isolated liabilities of the currency.
    #[serde(default)]
    pub iso_liab: String,
    /// Margin ratio of the currency.
    #[serde(default)]
    pub mgn_ratio: String,
    /// Accrued interest of the currency.
    #[serde(default)]
    pub interest: String,
    /// TWAP value.
    #[serde(default)]
    pub twap: String,
    /// Maximum loan of the currency.
    #[serde(default)]
    pub max_loan: String,
    /// Equity in USD.
    #[serde(default)]
    pub eq_usd: String,
    /// Leverage used in the notional value of the currency.
    #[serde(default)]
    pub notional_lever: String,
}

/// Position information.
///
/// Represents a single open position in the trading account.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Position {
    /// Instrument type.
    #[serde(default)]
    pub inst_type: String,
    /// Margin mode: "cross" or "isolated".
    #[serde(default)]
    pub mgn_mode: String,
    /// Position ID.
    #[serde(default)]
    pub pos_id: String,
    /// Position side: "net", "long", or "short".
    #[serde(default)]
    pub pos_side: String,
    /// Quantity of positions.
    #[serde(default)]
    pub pos: String,
    /// Base currency balance (applicable to SPOT/MARGIN positions in isolated margin).
    #[serde(default)]
    pub base_bal: String,
    /// Quote currency balance (applicable to SPOT/MARGIN positions in isolated margin).
    #[serde(default)]
    pub quote_bal: String,
    /// Base currency borrowed (applicable to SPOT/MARGIN).
    #[serde(default)]
    pub base_borrowed: String,
    /// Quote currency borrowed (applicable to SPOT/MARGIN).
    #[serde(default)]
    pub quote_borrowed: String,
    /// Position currency, only applicable to MARGIN positions.
    #[serde(default)]
    pub pos_ccy: String,
    /// Position that can be closed.
    #[serde(default)]
    pub avail_pos: String,
    /// Average open price.
    #[serde(default)]
    pub avg_px: String,
    /// Unrealized profit and loss.
    #[serde(default)]
    pub upl: String,
    /// Unrealized profit and loss ratio.
    #[serde(default)]
    pub upl_ratio: String,
    /// Instrument ID, e.g. "BTC-USDT-SWAP".
    #[serde(default)]
    pub inst_id: String,
    /// Leverage.
    #[serde(default)]
    pub lever: String,
    /// Estimated liquidation price.
    #[serde(default)]
    pub liq_px: String,
    /// Mark price.
    #[serde(default)]
    pub mark_px: String,
    /// Initial margin requirement.
    #[serde(default)]
    pub imr: String,
    /// Margin, can be added or reduced.
    #[serde(default)]
    pub margin: String,
    /// Margin ratio.
    #[serde(default)]
    pub mgn_ratio: String,
    /// Maintenance margin requirement.
    #[serde(default)]
    pub mmr: String,
    /// Liabilities. Only applicable to MARGIN.
    #[serde(default)]
    pub liab: String,
    /// Liabilities currency. Only applicable to MARGIN.
    #[serde(default)]
    pub liab_ccy: String,
    /// Interest. Only applicable to MARGIN.
    #[serde(default)]
    pub interest: String,
    /// Last trade ID.
    #[serde(default)]
    pub trade_id: String,
    /// Notional value of positions in USD.
    #[serde(default)]
    pub notional_usd: String,
    /// Auto-deleveraging indicator (1-5, higher means more risk).
    #[serde(default)]
    pub adl: String,
    /// Currency used for margin.
    #[serde(default)]
    pub ccy: String,
    /// Latest traded price.
    #[serde(default)]
    pub last: String,
    /// Update time, Unix timestamp in milliseconds.
    #[serde(default)]
    pub u_time: String,
    /// Creation time, Unix timestamp in milliseconds.
    #[serde(default)]
    pub c_time: String,
    /// Accumulated PnL of closing orders for the position.
    #[serde(default)]
    pub pnl: String,
    /// Accumulated fee. Negative means user transaction fee charged by the platform.
    #[serde(default)]
    pub fee: String,
    /// Accumulated funding fee.
    #[serde(default)]
    pub funding_fee: String,
    /// Realized profit and loss.
    #[serde(default)]
    pub real_pnl: String,
}

/// Account configuration.
///
/// Contains account-level settings and metadata.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AccountConfig {
    /// User ID.
    #[serde(default)]
    pub uid: String,
    /// Account level: "1" Simple, "2" Single-currency margin,
    /// "3" Multi-currency margin, "4" Portfolio margin.
    #[serde(default)]
    pub acct_lv: String,
    /// Position mode: "long_short_mode" or "net_mode".
    #[serde(default)]
    pub pos_mode: String,
    /// Whether to borrow coins automatically: "true" or "false".
    #[serde(default)]
    pub auto_loan: String,
    /// Current display type of Greeks: "PA" (coins) or "BS" (Black-Scholes).
    #[serde(default)]
    pub greeks_type: String,
    /// User level.
    #[serde(default)]
    pub level: String,
    /// Temporary user level experience.
    #[serde(default)]
    pub level_tmp: String,
    /// Contract isolated margin trading settings.
    #[serde(default)]
    pub ct_iso_mode: String,
    /// Margin isolated margin trading settings.
    #[serde(default)]
    pub mgn_iso_mode: String,
    /// Spot offset type.
    #[serde(default)]
    pub spot_offset_type: String,
    /// Current request IP.
    #[serde(default)]
    pub ip: String,
    /// Permission of the current API Key.
    #[serde(default)]
    pub perm: String,
    /// Label of the current API Key.
    #[serde(default)]
    pub label: String,
}

/// Leverage information.
///
/// Contains the leverage setting for a given instrument and margin mode.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct LeverageInfo {
    /// Instrument ID, e.g. "BTC-USDT-SWAP".
    #[serde(default)]
    pub inst_id: String,
    /// Margin mode: "cross" or "isolated".
    #[serde(default)]
    pub mgn_mode: String,
    /// Position side: "net", "long", or "short".
    #[serde(default)]
    pub pos_side: String,
    /// Leverage value.
    #[serde(default)]
    pub lever: String,
}

/// Maximum buy/sell amount.
///
/// Maximum tradeable buy and sell amounts for an instrument.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct MaxBuySellAmount {
    /// Instrument ID, e.g. "BTC-USDT".
    #[serde(default)]
    pub inst_id: String,
    /// Currency, e.g. "BTC".
    #[serde(default)]
    pub ccy: String,
    /// Maximum quantity to buy.
    #[serde(default)]
    pub max_buy: String,
    /// Maximum quantity to sell.
    #[serde(default)]
    pub max_sell: String,
}

/// Fee rate information.
///
/// Contains maker/taker fee rates and other fee-related details for an
/// instrument type.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct FeeRate {
    /// Fee rate level.
    #[serde(default)]
    pub level: String,
    /// Taker fee rate. Negative means the platform charges a fee;
    /// positive means the platform pays a rebate.
    #[serde(default)]
    pub taker: String,
    /// Maker fee rate.
    #[serde(default)]
    pub maker: String,
    /// Delivery fee rate.
    #[serde(default)]
    pub delivery: String,
    /// Fee rate for exercising options.
    #[serde(default)]
    pub exercise: String,
    /// Instrument type.
    #[serde(default)]
    pub inst_type: String,
    /// Timestamp, Unix timestamp in milliseconds.
    #[serde(default)]
    pub ts: String,
}

/// Maximum withdrawal amount.
///
/// Contains the maximum amount that can be withdrawn for a given currency.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct MaxWithdrawal {
    /// Currency, e.g. "BTC".
    #[serde(default)]
    pub ccy: String,
    /// Maximum withdrawal amount.
    #[serde(default)]
    pub max_wd: String,
    /// Maximum withdrawal amount (excluding borrowed assets).
    #[serde(default)]
    pub max_wd_ex: String,
    /// Max withdrawal with spot offset.
    #[serde(default)]
    pub spot_offset_max_wd: String,
    /// Max withdrawal with spot offset (excluding borrowed assets).
    #[serde(default)]
    pub spot_offset_max_wd_ex: String,
}

/// MMP (Market Maker Protection) configuration.
///
/// Contains the MMP configuration settings for an instrument family.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct MmpConfig {
    /// Instrument family, e.g. "BTC-USD".
    #[serde(default)]
    pub inst_family: String,
    /// Time window (ms).
    #[serde(default)]
    pub time_interval: String,
    /// Frozen period (ms).
    #[serde(default)]
    pub frozen_interval: String,
    /// Quantity limit.
    #[serde(default)]
    pub qty_limit: String,
}

/// Account risk state.
///
/// Represents the current risk state of the account.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct AccountRiskState {
    /// Whether the account is at risk.
    #[serde(default, rename = "atRisk")]
    pub at_risk: String,
    /// Index-based risk indicator.
    #[serde(default, rename = "atRiskIdx")]
    pub at_risk_idx: String,
    /// Margin-based risk indicator.
    #[serde(default, rename = "atRiskMgn")]
    pub at_risk_mgn: String,
    /// Timestamp, Unix timestamp in milliseconds.
    #[serde(default)]
    pub ts: String,
}

/// Generic result for set operations.
///
/// Used for responses from endpoints like `setPositionMode`, `setLeverage`,
/// `setGreeks`, etc. Since the response fields vary by endpoint, this uses
/// `serde_json::Value` to flexibly capture any returned fields.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct SetResult {
    /// The raw result data. Different set operations return different fields
    /// (e.g. `posMode` for set position mode, `lever` for set leverage).
    #[serde(flatten)]
    pub data: serde_json::Value,
}
