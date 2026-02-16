use serde::{Deserialize, Serialize};

/// Instrument type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "UPPERCASE")]
pub enum InstrumentType {
    #[default]
    Spot,
    Margin,
    Swap,
    Futures,
    Option,
}

/// "ANY" or a specific instrument type, used in some subscription args.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum InstrumentTypeFilter {
    Spot,
    Margin,
    Swap,
    Futures,
    Option,
    Any,
}

/// Order side.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum OrderSide {
    #[default]
    Buy,
    Sell,
}

/// Order type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum OrderType {
    Market,
    #[default]
    Limit,
    PostOnly,
    Fok,
    Ioc,
    OptimalLimitIoc,
    Mmp,
    MmpAndPostOnly,
    #[serde(rename = "elp")]
    Elp,
}

/// Order state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderState {
    Canceled,
    Live,
    PartiallyFilled,
    Filled,
    MmpCanceled,
}

/// Trade mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum TradeMode {
    Cross,
    Isolated,
    #[default]
    Cash,
    SpotIsolated,
}

/// Position side.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PositionSide {
    Net,
    Long,
    Short,
}

/// Margin mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum MarginMode {
    #[default]
    Cross,
    Isolated,
}

/// Position mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum PosMode {
    LongShortMode,
    #[default]
    NetMode,
}

/// Algo order type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AlgoOrderType {
    Conditional,
    Oco,
    Trigger,
    MoveOrderStop,
    Iceberg,
    Twap,
    Chase,
}

/// Algo order state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AlgoOrderState {
    Live,
    Pause,
    PartiallyEffective,
    Effective,
    Canceled,
    OrderFailed,
    PartiallyFailed,
}

/// Price trigger type for algo orders.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PriceTriggerType {
    Last,
    Index,
    Mark,
}

/// Account level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AccountLevel {
    #[serde(rename = "1")]
    Simple,
    #[serde(rename = "2")]
    SingleCurrencyMargin,
    #[serde(rename = "3")]
    MultiCurrencyMargin,
    #[serde(rename = "4")]
    PortfolioMargin,
}

/// Greeks display type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum GreeksType {
    #[default]
    #[serde(rename = "PA")]
    GreeksInCoins,
    #[serde(rename = "BS")]
    BlackScholesGreeks,
}

/// Transfer type for funds transfers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TransferType {
    #[serde(rename = "0")]
    WithinAccount,
    #[serde(rename = "1")]
    MasterToSubAccount,
    #[serde(rename = "2")]
    SubAccountToMaster,
    #[serde(rename = "3")]
    SubAccountToMasterManaged,
    #[serde(rename = "4")]
    SubAccountToSubAccount,
}

/// Candle bar size / period.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Bar {
    #[serde(rename = "1s")]
    S1,
    #[serde(rename = "1m")]
    M1,
    #[serde(rename = "3m")]
    M3,
    #[serde(rename = "5m")]
    M5,
    #[serde(rename = "15m")]
    M15,
    #[serde(rename = "30m")]
    M30,
    #[serde(rename = "1H")]
    H1,
    #[serde(rename = "2H")]
    H2,
    #[serde(rename = "4H")]
    H4,
    #[serde(rename = "6H")]
    H6,
    #[serde(rename = "12H")]
    H12,
    #[serde(rename = "1D")]
    D1,
    #[serde(rename = "2D")]
    D2,
    #[serde(rename = "3D")]
    D3,
    #[serde(rename = "1W")]
    W1,
    #[serde(rename = "1M")]
    Mo1,
    #[serde(rename = "3M")]
    Mo3,
}

/// Withdrawal destination.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum WithdrawDest {
    #[serde(rename = "3")]
    Internal,
    #[default]
    #[serde(rename = "4")]
    OnChain,
}

/// Self-trade prevention mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StpMode {
    CancelMaker,
    CancelTaker,
    CancelBoth,
}

/// Grid algo order type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GridAlgoOrderType {
    SpotGrid,
    ContractGrid,
    MoonGrid,
}
