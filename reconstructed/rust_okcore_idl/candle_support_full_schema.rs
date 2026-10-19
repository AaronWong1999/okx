// =============================================================================
// Reconstructed Rust UniFFI IDL & Schema for crate: `candle_support`
// Extracted from OKX Android v6.194.0 (lib/arm64-v8a/libOKCore.so)
// Total symbols decoded: 878
//   Records: 158 | Enums: 13 | Interfaces: 22
//   Constructors: 7 | Methods: 607 | Functions: 70
// =============================================================================

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

// -----------------------------------------------------------------------------
// 1. Enums (13)
// -----------------------------------------------------------------------------

/// Matches native `OKTKLineSettingCase` (KLine scene).
/// 
/// IMPORTANT: keep numeric values stable once shipped (used across UniFFI).
#[derive(Debug, Clone, PartialEq)]
#[repr(i32)]
pub enum ChartSettingCase {
    /// Default OKX (Pro) KLine.
    Cefi = 0,
    /// DeFi KLine.
    Defi = 1,
    /// Lite / CR KLine.
    Lite = 2,
    /// DEX market KLine.
    DexMarket = 3,
    /// CeDeFi KLine.
    CeDeFi = 4,
    /// Runes.
    Runes = 5,
}

/// Matches native/Kotlin `EntryType` (chart entry subtype).
/// 
/// This is used to further refine chart settings behavior within a `SettingCase`.
/// For example, `SettingCase::Cefi` may have multiple entry subtypes (portrait, landscape, etc).
/// 
/// IMPORTANT: keep numeric values stable once shipped (used across UniFFI).
#[derive(Debug, Clone, PartialEq)]
#[repr(i32)]
pub enum ChartSettingCaseLayoutType {
    /// Pro classic portrait.
    ProPortrait = 0,
    /// Pro classic portrait - TradingView mode.
    ProPortraitTv = 1,
    /// Pro fullscreen.
    ProFullScreen = 2,
    /// Pro landscape.
    ProLandscape = 3,
    /// Pro formula / combined KLine.
    ProFormula = 4,
    /// Pro compare KLine.
    ProCompare = 5,
    /// Lite classic portrait.
    LitePortrait = 6,
    /// Lite landscape.
    LiteLandscape = 7,
    /// CeDeFi DEX KLine entry.
    CedefiDex = 8,
    /// DEX KLine entry.
    Dex = 9,
    /// Others.
    Others = 10,
}

/// Runtime-resolved system color (K-line theme rise/fall).
/// 
/// When an item's static default is unsuitable because the visual should track
/// the K-line theme color scheme — e.g., TSI bull/bear gradient should follow
/// the user's red-up-green-down / green-up-red-down preference — set this on the item and let
/// the native side resolve it at draw / display time.
/// 
/// Mutually exclusive with `default_color` and `default_color_block`.
#[derive(Debug, Clone, PartialEq)]
pub enum DefaultSystemColor {
    None,
    Rise,
    Fall,
}

/// Enum of all indicator names for type-safe access from Swift/Kotlin.
/// 
/// This enum provides compile-time safety for indicator names, avoiding
/// string typos and enabling IDE auto-completion on native platforms.
/// 
/// # Example (Swift)
/// ```swift
/// let name = getIndicatorName(indicator: .ma)
/// // name == "MA"
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum IndicatorName {
    /// MA (Moving Average) indicator
    Ma,
    /// EMA (Exponential Moving Average) indicator
    Ema,
    /// VWAP (Volume Weighted Average Price) indicator
    Vwap,
    /// VOL (Volume) indicator
    Vol,
    /// MACD (Moving Average Convergence Divergence) indicator
    Macd,
    /// KDJ (Stochastic Oscillator) indicator
    Kdj,
    /// BOLL (Bollinger Bands) indicator
    Boll,
    /// RSI (Relative Strength Index) indicator
    Rsi,
    /// StochRSI (Stochastic RSI) indicator
    StochRsi,
    /// OBV (On-Balance Volume) indicator
    Obv,
    /// SAR (Parabolic Stop and Reverse) indicator
    Sar,
    /// TRIX (Triple Exponential Average) indicator
    Trix,
    /// WR (Williams %R) indicator
    Wr,
    /// ROC (Rate of Change) indicator
    Roc,
    /// PSY (Psychological Line) indicator
    Psy,
    /// DMI (Directional Movement Index) indicator
    Dmi,
    /// CCI (Commodity Channel Index) indicator
    Cci,
    /// DMA (Different of Moving Average) indicator
    Dma,
    /// BRAR (Bull and Bear Ratio) indicator
    Brar,
    /// VR (Volatility Ratio) indicator
    Vr,
    /// EMV (Ease of Movement) indicator
    Emv,
    /// MTM (Momentum) indicator
    Mtm,
    /// SR (Support/Resistance) indicator
    Sr,
    /// SuperTrend indicator
    SuperTrend,
    /// ENVELOPE (Nadaraya-Watson Envelope) indicator
    Envelope,
    /// SKDJ (Slow Stochastic) indicator
    Skdj,
    /// WMA (Weighted Moving Average) indicator
    Wma,
    /// VWMA (Volume Weighted Moving Average) indicator
    Vwma,
    /// AVL (Average Value Line) indicator
    Avl,
    /// ATR (Average True Range) indicator
    Atr,
    /// MFI (Money Flow Index) indicator
    Mfi,
    /// SMI (Squeeze Momentum Indicator) indicator
    Smi,
    /// DonchianChannels indicator
    DonchianChannels,
    /// PVT (Price Volume Trend) indicator
    Pvt,
    /// TSI (True Strength Index) indicator
    Tsi,
    /// RVI (Relative Vigor Index) indicator
    Rvi,
    /// QQE (Qualitative Quantitative Estimation) indicator
    Qqe,
    /// MassIndex indicator (key uppercase matches ObjC `OKKLineIndicatorMASSINDEX`)
    MassIndex,
    /// KST (Know Sure Thing) indicator
    Kst,
    /// BBP (Bull Bear Power) indicator
    Bbp,
    /// NVI (Negative Volume Index) indicator
    Nvi,
    /// PMO (Price Momentum Oscillator) indicator
    Pmo,
    /// RCI Ribbon (Rank Correlation Index, 3-line ribbon) indicator
    RciRibbon,
    /// OI (Open Interest) indicator
    Oi,
    /// Elite Long/Short Position Ratio indicator
    EliteLsPositionRatio,
    /// Elite Long/Short Account Ratio indicator
    EliteLsAccountRatio,
    /// Long/Short Position Ratio indicator
    LsPositionRatio,
    /// Taker Buy/Sell Volume indicator
    TakerVol,
    /// CVD (Cumulative Volume Delta) indicator
    Cvd,
    /// SLOPE (moving-average rate of change) indicator
    /// 
    /// Kept at the end of the enum to preserve existing UniFFI variant ordinals.
    Slope,
}

/// Indicator type classification.
#[derive(Debug, Clone, PartialEq)]
pub enum IndicatorType {
    /// Main chart indicators (overlay on candlestick)
    Main,
    /// Sub chart indicators (separate panel below)
    Sub,
}

/// Mirrors native `OKTBizType*` (instrument business line classification),
/// scoped to the chart-settings domain.
/// 
/// Named with a `KLineSettings` prefix to avoid colliding with `retail-kline`'s
/// own `KLineBizType` UniFFI enum (the two share the same flat namespace in the
/// generated Swift bindings). Kept independent from core-trading's
/// `FFITradeBizType` to avoid platform-service reverse-depending on biz-service.
/// Variants must stay in sync with the native enum
/// (Spot / Margin / Swap / Futures / Option / Events).
#[derive(Debug, Clone, PartialEq)]
pub enum KLineSettingsBizType {
    Spot,
    Margin,
    Swap,
    Futures,
    Option,
    Events,
}

/// Drawing state enumeration
/// 
/// Must match iOS `OKKLineDrawingState` enum exactly.
/// Exported to native as `OKKLineDrawingState`.
#[derive(Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum OKKLineDrawingState {
    /// Exit drawing mode (OKKLineDrawingStateExit)
    Exit = 0,
    /// Idle state (OKKLineDrawingStateIdle)
    Idle = 1,
    /// Ready to draw (OKKLineDrawingStateReady)
    Ready = 2,
    /// Currently drawing (OKKLineDrawingStateDrawing)
    Drawing = 3,
    /// Drawing completed (OKKLineDrawingStateDone)
    Done = 4,
    /// Line selected (OKKLineDrawingStateSelected)
    Selected = 5,
    /// Dragging control point (completed line) (OKKLineDrawingStateDragCircle)
    DragCircle = 6,
    /// Dragging control point (while drawing) (OKKLineDrawingStateDrawingWhenDragCircle)
    DrawingWhenDragCircle = 7,
    /// Dragging line (OKKLineDrawingStateDragLine)
    DragLine = 8,
}

/// Drawing line style enumeration
/// 
/// Exported to native as `OKKLineDrawingStyle`.
#[derive(Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum OKKLineDrawingStyle {
    /// Solid line
    Solid = 0,
    /// Dashed line
    Dash = 1,
}

/// Drawing line type enumeration
/// 
/// Represents different types of drawing tools available on K-line charts.
/// Must match iOS `OKKLineDrawingToolsLineType` enum exactly.
/// Exported to native as `OKKLineDrawingType`.
#[derive(Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum OKKLineDrawingType {
    /// Unknown (OKKLineDrawingToolsLineTypeUnknown)
    Unknown = 0,
    /// Horizontal segment (OKKLineHorizontalTypeSegment)
    HorizontalSegment = 1,
    /// Horizontal line (OKKLineHorizontalTypeLinear)
    HorizontalLine = 2,
    /// Horizontal ray (OKKLineHorizontalTypeRay)
    HorizontalRay = 3,
    /// Trend line segment (OKKLineSlashTypeSegment)
    TrendLine = 4,
    /// Straight line (OKKLineSlashTypeLinear)
    StraightLine = 5,
    /// Ray (OKKLineSlashTypeRay)
    Ray = 6,
    /// Arrow (OKKLineSlashTypeArrow)
    Arrow = 7,
    /// Price line (OKKLineDrawingToolsLineTypePrice)
    PriceLine = 8,
    /// Price channel (OKKLineDrawingToolsLineTypePriceChannel)
    PriceChannel = 9,
    /// Fibonacci retracement (OKKLineDrawingToolsLineTypeFibonacci)
    Fibonacci = 10,
    /// Rectangle (OKKLineFigureTypeRectAngle)
    Rectangle = 11,
    /// Circle (OKKLineFigureTypeCircle)
    Circle = 12,
    /// Triangle (OKKLineFigureTypeTriAngle)
    Triangle = 13,
    /// Info line (OKKLineFigureTypeInfoLine)
    InfoLine = 14,
    /// Measurement tool (OKKLineFigureTypeMeasurement). Appended last —
    /// discriminants are persisted in SQLite and transmuted back, so existing
    /// values must never shift. iOS OC enum must add the same raw value 15.
    Measurement = 15,
    /// Vertical extended line (single-point vertical line spanning the main
    /// chart top-to-bottom, bound to one time point). Append-only: 0..15 are
    /// frozen, so this takes the next free discriminant 16 and, once released,
    /// must never be reassigned. iOS OC enum must add the same raw value 16.
    VerticalLine = 16,
}

/// Mirrors native `OKKLinePeriodsStyle`.
#[derive(Debug, Clone, PartialEq)]
#[repr(i32)]
pub enum PeriodsStyle {
    Default = 0,
    DropDown = 1,
}

/// Represents different groups of settings keys for selective migration.
/// 
/// This allows native code to migrate only a subset of keys, e.g., migrate
/// Settings Page keys at app launch, and Settings Panel keys when chart is opened.
#[derive(Debug, Clone, PartialEq)]
pub enum SettingsKeyGroup {
    /// Settings Page keys (used by `OKKLineSettingViewController`)
    Page,
    /// Settings Panel keys (used by quick settings overlay on chart)
    Panel,
    /// All keys (both Page and Panel)
    All,
}

/// Profit display style for TP/SL or floating PnL.
/// 
/// Raw values are aligned with native:
/// - `Rate` = 0
/// - `Amount` = 1
#[derive(Debug, Clone, PartialEq)]
pub enum TpslProfitStyle {
    Rate,
    Amount,
}

/// Tag position for order/position labels on the chart.
/// 
/// Raw values are aligned with native:
/// - `Left` = 0
/// - `Right` = 1
#[derive(Debug, Clone, PartialEq)]
pub enum TradeTagsPosition {
    Left,
    Right,
}

// -----------------------------------------------------------------------------
// 2. Records / Data Models (158)
// -----------------------------------------------------------------------------

/// ATR input parameters.
/// 
/// - **Names**: period
/// - **Default**: period=14 (range: 2~200)
/// - **Output**: ATR line
#[derive(Debug, Clone, PartialEq)]
pub struct AtrInputParams {
    /// ATR calculation period, default: 14
    pub period: i64,
}

/// ATR indicator output.
#[derive(Debug, Clone, PartialEq)]
pub struct AtrOutput {
    /// ATR line data
    pub atr: IndicatorData,
    /// Reserved field for native use (not set by Rust)
    pub visible_range: VisibleRange,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AtrSettings {
    /// Period parameter (key: "period")
    pub period: IndicatorSettingsItem,
    /// ATR line (key: "atr")
    pub atr: IndicatorSettingsItem,
}

/// AVL indicator output.
/// 
/// AVL = amount / volume per candle. Requires extended CandleBarData with amount field.
/// NOTE: Calculation not yet implemented - CandleBarData trait needs `amount()` method.
#[derive(Debug, Clone, PartialEq)]
pub struct AvlOutput {
    /// AVL line data
    pub avl: IndicatorData,
    /// Reserved field for native use (not set by Rust)
    pub visible_range: VisibleRange,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AvlSettings {
    /// AVL line (key: "out")
    pub avl: IndicatorSettingsItem,
}

/// BBP input parameters.
/// 
/// - **Names**: length
/// - **Default**: length=13 (range: 2~200)
/// - **Output**: BullPower, BearPower
#[derive(Debug, Clone, PartialEq)]
pub struct BbpInputParams {
    /// EMA calculation period, default: 13
    pub length: i64,
}

/// BBP indicator output.
#[derive(Debug, Clone, PartialEq)]
pub struct BbpOutput {
    /// Bull Power (High - EMA)
    pub bull_power: IndicatorData,
    /// Bear Power (Low - EMA)
    pub bear_power: IndicatorData,
    /// Reserved field for native use (not set by Rust)
    pub visible_range: VisibleRange,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BbpSettings {
    /// EMA period (key: "length")
    pub length: IndicatorSettingsItem,
    /// Bull Power line (key: "bullPower")
    pub bull_power: IndicatorSettingsItem,
    /// Bear Power line (key: "bearPower")
    pub bear_power: IndicatorSettingsItem,
}

/// BOLL input parameters.
/// 
/// - **Names**: MID (length), STD (mult)
/// - **Default**: length=20 (range: 2~500), mult=2 (range: 1~500)
/// - **Output**: BOLL (middle), UB (upper), LB (lower)
#[derive(Debug, Clone, PartialEq)]
pub struct BollInputParams {
    /// Period for the middle band SMA, default: 20
    pub length: i64,
    /// Standard deviation multiplier for bands, default: 2.0
    pub mult: f64,
}

/// BOLL indicator output.
/// 
/// Bollinger Bands consist of a middle band (SMA) and two outer bands
/// that represent standard deviation levels above and below the middle band.
#[derive(Debug, Clone, PartialEq)]
pub struct BollOutput {
    /// Upper band (middle + multiplier × standard deviation)
    pub ub: IndicatorData,
    /// Middle band (Simple Moving Average)
    pub boll: IndicatorData,
    /// Lower band (middle - multiplier × standard deviation)
    pub lb: IndicatorData,
    /// Reserved field for native use (not set by Rust)
    pub visible_range: VisibleRange,
}

/// Strongly-typed BOLL settings.
/// 
/// Keys: length, mult, median, upper, lower, background
#[derive(Debug, Clone, PartialEq)]
pub struct BollSettings {
    /// Period length (key: "length")
    pub length: IndicatorSettingsItem,
    /// Multiplier (key: "mult")
    pub mult: IndicatorSettingsItem,
    /// Middle band / BOLL line (key: "median")
    pub boll: IndicatorSettingsItem,
    /// Upper band (key: "upper")
    pub ub: IndicatorSettingsItem,
    /// Lower band (key: "lower")
    pub lb: IndicatorSettingsItem,
    /// Background fill (key: "background")
    pub background: IndicatorSettingsItem,
}

/// BRAR input parameters.
/// 
/// - **Names**: Calculation Period
/// - **Default**: length=26 (range: 2~120)
/// - **Output**: BR, AR
#[derive(Debug, Clone, PartialEq)]
pub struct BrarInputParams {
    /// Calculation period, default: 26
    pub length: i64,
}

/// BRAR indicator output.
/// 
/// BRAR measures buying and selling pressure through BR and AR lines.
#[derive(Debug, Clone, PartialEq)]
pub struct BrarOutput {
    /// BR line (buying pressure indicator)
    pub br: IndicatorData,
    /// AR line (selling pressure indicator)
    pub ar: IndicatorData,
    /// Reserved field for native use (not set by Rust)
    pub visible_range: VisibleRange,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BrarSettings {
    pub length: IndicatorSettingsItem,
    /// BR line (key: "out1")
    pub br: IndicatorSettingsItem,
    /// AR line (key: "out2")
    pub ar: IndicatorSettingsItem,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CandleSettingsState {
    pub is_loaded: bool,
    pub selected_main_indicators: Vec<String>,
    pub selected_sub_indicators: Vec<String>,
    pub selected_all_sub_indicators: Vec<String>,
    pub selected_trade_data_indicators: Vec<String>,
    pub selected_period: String,
    pub prefer_periods: Vec<String>,
    pub all_periods: Vec<String>,
    pub enable_expand_order_line: bool,
    pub enable_expand_position_line: bool,
    /// TradeTagsPosition: Left=0, Right=1
    pub order_location_value: i32,
    /// TradeTagsPosition: Left=0, Right=1
    pub position_location_value: i32,
    /// TpslProfitStyle: Rate=0, Amount=1
    pub profit_style_value: i32,
    pub enable_revert_position: bool,
    pub enable_cancel_order_confirmation: bool,
    pub enable_tpsl_adjustment_confirmation: bool,
    pub enable_display_all_positions: bool,
    pub show_liquidation_px: bool,
    pub show_balance_px: bool,
    /// Whether quick order is available — mirror of
    /// `page_config().derived_enable_quick_order()`. Main Cefi chart: derived
    /// from the shared SCENE `chart_context` (pushed via
    /// `page_config()/panel_config().set_chart_context`) + the session-global
    /// account `trade_facts` (pushed by retail-kline) +
    /// `current_is_market_cap`; other cases/layouts: the constructor-preset
    /// stored flag. Recomputed on `load_settings` and on every
    /// `notify_page_config_changed` / `notify_panel_config_changed` — call one
    /// of those after pushing a new chart context.
    pub enable_quick_order: bool,
    /// Whether buy/sell history-point markers are available — mirror of
    /// `panel_config().derived_enable_buy_sell_point()` (stored preset AND NOT
    /// TM-sub-account hidden-transaction, from the pushed chart context).
    /// Recomputed together with `enable_quick_order`.
    pub enable_buy_sell_point: bool,
    /// Mirror of `panel_config().derived_enable_order_tags()` (stored preset
    /// AND NOT TM-sub-account hidden-cur-pos). Recomputed with the group.
    pub enable_order_tags: bool,
    /// Mirror of `panel_config().derived_enable_position_tags()`.
    pub enable_position_tags: bool,
    /// Mirror of `panel_config().derived_enable_asset_cost_tags()`.
    pub enable_asset_cost_tags: bool,
    /// Mirror of `panel_config().derived_enable_balance_px()` — main Cefi
    /// chart: `isSwapOrFutures(biz_type) && !hidden_cur_pos`. Distinct from
    /// `show_balance_px` (the user's toggle); this is the capability gate.
    pub enable_balance_px: bool,
    /// Mirror of `panel_config().derived_enable_liquidation_px()` — same
    /// semantics as `enable_balance_px`. Distinct from `show_liquidation_px`.
    pub enable_liquidation_px: bool,
    /// Mirror of `panel_config().derived_enable_guide()` (stored preset AND
    /// the entity supports all five business lines).
    pub enable_guide: bool,
    /// Mirror of `panel_config().derived_is_average_asset_cost()` — spot
    /// cost-price basis (`true` = average/open cost) from the pushed context;
    /// feeds native `configuration.isAverageAssetCost`.
    pub is_average_asset_cost: bool,
    pub page_config_version: u64,
    pub panel_config_version: u64,
    pub indicator_config_version: u64,
    pub period_version: u64,
    pub trade_settings_version: u64,
    pub migration_complete: bool,
}

/// CCI input parameters - 1 period value.
/// 
/// - **Count**: 1
/// - **Names**: CCI
/// - **Default**: `[14]`
/// - **Range**: 2 ~ 100
#[derive(Debug, Clone, PartialEq)]
pub struct CciInputParams {
    /// Period values for CCI line (1 value)
    pub periods: Vec<i64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CciSettings {
    pub length: IndicatorSettingsItem,
    /// CCI line (key: "out")
    pub cci: IndicatorSettingsItem,
}

/// Legacy indicator-related base keys from the former Rust chart-settings cache.
/// 
/// This record is UniFFI-exported so native bridges (Swift/ObjC/Kotlin) can avoid hardcoding
/// string literals and drifting from Rust.
#[derive(Debug, Clone, PartialEq)]
pub struct ChartIndicatorKeys {
    /// Selected main indicators cache key (native: `OKKLineToolbarDataCenterMainIndicatorKey`).
    pub chart_selected_main_indicator: String,
    /// Selected sub indicators cache key (native: `OKKLineToolbarDataCenterSubIndicatorKey`).
    /// 
    /// This selection excludes trading-data indicators.
    pub chart_selected_sub_indicator: String,
    /// Selected "all sub indicators" cache key (native: `OKKLineToolbarDataCenterAllSubIndicatorKey`).
    /// 
    /// This selection includes trading-data indicators (i.e. sub + trading).
    pub chart_selected_all_sub_indicator: String,
    /// Selected trading-data indicators cache key (native: `OKKLineToolbarDataCenterTradingDataIndicatorKey`).
    pub chart_selected_trading_data_indicator: String,
}

/// Period string constants used by native code.
/// 
/// UniFFI does not export Rust `const` values directly, so we expose these via a UniFFI `Record`.
#[derive(Debug, Clone, PartialEq)]
pub struct ChartKlinePeriodConstants {
    /// Period string: `"1s"`.
    pub period_1s: String,
    /// Period string: `"5s"`.
    pub period_5s: String,
    /// Period string: `"15s"`.
    pub period_15s: String,
    /// Period string: `"30s"`.
    pub period_30s: String,
    /// Period string: `"1min"`.
    pub period_1min: String,
    /// Period string: `"2min"`.
    pub period_2min: String,
    /// Period string: `"3min"`.
    pub period_3min: String,
    /// Period string: `"5min"`.
    pub period_5min: String,
    /// Period string: `"15min"`.
    pub period_15min: String,
    /// Period string: `"30min"`.
    pub period_30min: String,
    /// Period string: `"1hour"`.
    pub period_1h: String,
    /// Period string: `"2hour"`.
    pub period_2h: String,
    /// Period string: `"4hour"`.
    pub period_4h: String,
    /// Period string: `"6hour"`.
    pub period_6h: String,
    /// Period string: `"12hour"`.
    pub period_12h: String,
    /// Period string: `"day"`.
    pub period_1d: String,
    /// Period string: `"2day"`.
    pub period_2d: String,
    /// Period string: `"3day"`.
    pub period_3d: String,
    /// Period string: `"5day"`.
    pub period_5d: String,
    /// Period string: `"week"`.
    pub period_1w: String,
    /// Period string: `"month"`.
    pub period_1m: String,
    /// Period string: `"3month"`.
    pub period_3m: String,
    /// Period string: `"6month"`.
    pub period_6m: String,
    /// Period string: `"year"`.
    pub period_1y: String,
    /// Time chart special period string: `"OKKLineTimeChart"`.
    pub period_time_chart: String,
    /// Market cap special period string: `"OKKLineMarketCap"`.
    pub period_market_cap: String,
}

/// Period-related base keys used by Rust chart settings persistence.
/// 
/// This record is UniFFI-exported so native bridges (Swift/ObjC/Kotlin) can avoid hardcoding
/// string literals and drifting from Rust.
#[derive(Debug, Clone, PartialEq)]
pub struct ChartPeriodKeys {
    /// Current selected period key (native: `OKKLineToolbarDataCenterPeriodKey`).
    pub chart_period_current: String,
    /// User-preferred periods key (native: `OKKLinePeriodsByUserDefaultKey`).
    pub chart_periods_user_preferred: String,
    /// All available periods ordering key (native: `OKKLineAllPeriodsByUserDefaultKey`).
    pub chart_all_available_periods: String,
}

/// Rust version of `OKTKLineSettingObject`.
/// 
/// NOTE: No `UIColor` (native `NSArray<UIColor *> *colors`) is stored here.
/// Instead, store colors as hex strings and/or color block identifiers.
#[derive(Debug, Clone, PartialEq)]
pub struct ChartSettingObject {
    pub name: Option<String>,
    /// JSON string for indicator params (portable, UniFFI-safe).
    /// 
    /// Native `OKTKLineSettingObject.params` can be heterogenous; we keep it as JSON.
    pub params_json: Option<String>,
    /// Colors as hex strings (e.g. "#FF00AA") or other portable representations.
    pub colors: Option<Vec<String>>,
    /// Map of hex -> block (or any string->string mapping).
    pub color_blks: Option<HashMap<String, String>>,
    pub keys: Option<Vec<String>>,
    pub alphas: Option<Vec<f64>>,
    pub flags: Option<Vec<i32>>,
}

/// Base keys used by Rust chart settings persistence.
/// 
/// These are the *base* keys; Rust may derive an effective key based on `SettingCase`.
/// 
/// This record is UniFFI-exported so native bridges (Swift/ObjC/Kotlin) can avoid
/// hardcoding string literals and drifting from Rust.
#[derive(Debug, Clone, PartialEq)]
pub struct ChartSettingsBaseKeys {
    /// Persisted key for TradingView/Native chart switch status.
    pub chart_trading_view_status: String,
    /// Persisted key for dark theme enablement for the chart.
    pub chart_dark_theme: String,
    /// Persisted key for current chart mode (e.g. classic / full-screen).
    pub chart_chart_mode: String,
    /// Persisted key for zoom/scaling behavior type (bar / line).
    pub chart_zoom_type: String,
    /// Persisted key for trade button position (left / right).
    pub chart_trade_button_left: String,
    /// Persisted key for trade button mode (0 left / 1 right / 2 quick).
    pub chart_trade_button_mode: String,
    /// Persisted key for candle style (e.g. candle/bar/hollow).
    pub chart_candle_style: String,
    /// Persisted key for candle rise/fall color preference.
    pub chart_candle_color: String,
    /// Persisted key for "digest" display type (top panel / floating window).
    pub chart_digest_type: String,
    /// Persisted key for drag/scroll behavior type (left / center / right).
    pub chart_drag_type: String,
    /// Persisted key for enabling time anchor mode.
    pub chart_time_anchor: String,
    /// Persisted key for enabling "switch pair keeps time anchor" behavior.
    pub chart_switch_pair_time_anchor: String,
    /// Persisted key for enabling the news entry on the chart page (SettingObject archive).
    pub chart_news_entry: String,
    /// Persisted key for enabling the TradFi earnings banner beside the news entry.
    pub chart_earnings_banner: String,
    /// Persisted key for quick order (fast trading) toggle.
    pub chart_quick_order: String,
    /// Persisted key for tapping an order-book cell to open quick order.
    pub chart_order_book_quick_order: String,
    /// Persisted key for "keep screen on" preference on the chart page.
    pub chart_keep_on: String,
    /// Persisted key for double-tap behavior (e.g. enter/exit fullscreen).
    pub chart_double_tap_fullscreen: String,
    /// Persisted key for cancel order confirmation toggle.
    pub chart_cancel_order_confirmation: String,
    /// Persisted key for TP/SL adjustment confirmation toggle.
    pub chart_tpsl_adjustment_confirmation: String,
    /// Persisted key for syncing draw-line data across devices/contexts (SettingObject archive).
    pub chart_draw_line_sync: String,
    /// Persisted key for auto-clearing draw-line data (SettingObject archive).
    pub chart_draw_line_auto_clear: String,
    /// Persisted key for the draw-line magnet mode (0 off / 1 weak / 2 strong; CeDeFi-shared).
    pub chart_draw_line_magnet_mode: String,
    /// Persisted key for multi-period change toggle.
    pub chart_show_multi_period_change: String,
    /// Persisted key for chart type (K-line vs line chart).
    pub chart_chart_type: String,
    /// Persisted key for K-line render style (0 = Candlestick, 1 = OHLC Bar).
    pub chart_kline_style: String,
    /// Persisted key for stock price adjustment mode (0 = none, 1 = forward).
    pub chart_stock_price_adjustment: String,
    /// Notification name posted when the chart height changes (used for layout sync).
    pub chart_height_changed_notification: String,
    /// Persisted key for the "graduate/scale" (grid/grade) setting of the chart.
    pub chart_graduate: String,
    /// Persisted key for candle width / bar width preference.
    pub chart_candle_width: String,
    /// Persisted key for the "history orders" display toggle/state.
    pub chart_history_order: String,
    /// Persisted key for the "position cost" display toggle/state.
    pub chart_position_cost: String,
    /// Persisted key for the "orders overlay" display toggle/state.
    pub chart_orders: String,
    /// Persisted key for toolbar coordinate display type (linear / percentage).
    pub chart_toolbar_data_center_coordinate_type: String,
    /// Persisted key for native chart height value (used for restoring size).
    pub chart_native_height: String,
    /// Persisted key for the countdown toggle/config (SettingObject archive).
    pub chart_count_down: String,
    /// Persisted key for price alert toggle/config (SettingObject archive).
    pub chart_price_alert: String,
    /// Persisted key for showing price difference / price change indicator.
    pub chart_show_price_diff: String,
    /// Persisted key for showing asset cost on the chart.
    pub chart_show_asset_cost: String,
    /// Persisted key for showing upcoming economic calendar events.
    pub chart_economic_upcoming: String,
    /// Persisted key for showing past economic calendar events.
    pub chart_economic_past: String,
    /// Persisted key for the TradFi earnings-calendar master toggle.
    pub chart_earnings_calendar: String,
    /// Persisted key for showing upcoming (not yet released) earnings events.
    pub chart_earnings_upcoming: String,
    /// Persisted key for showing published earnings events.
    pub chart_earnings_published: String,
    /// Persisted key for the bot grid (algo order) lines overlay toggle.
    /// Native reads/writes this only for the Lite/Cefi setting cases (default ON).
    pub chart_bot_grid_lines: String,
}

/// Rust version of `OKKLineToolbarIndicatorItem`.
#[derive(Debug, Clone, PartialEq)]
pub struct ChartToolbarIndicatorItem {
    pub indicator: String,
    pub display_name: String,
    /// Mirrors native `OKKLineIndicatorType` numeric value.
    pub indicator_type: i32,
}

/// Rust version of `OKKLineToolbarPeriodItem`.
/// 
/// NOTE: `display_name` is UI-facing and localized in native today. Rust does not own i18n,
/// so by default this is often set to `period` unless the host app provides a mapping.
#[derive(Debug, Clone, PartialEq)]
pub struct ChartToolbarPeriodItem {
    pub period: String,
    pub display_name: String,
}

/// Color pair with light and dark mode hex values.
#[derive(Debug, Clone, PartialEq)]
pub struct ColorPair {
    /// Light mode hex color (e.g., 0xF23645)
    pub light: u32,
    /// Dark mode hex color (e.g., 0xF23645)
    pub dark: u32,
}

/// CVD indicator output.
/// 
/// CVD = cumulative(takerBuyVol - takerSellVol), resets daily at UTC 00:00.
/// NOTE: Calculation not yet implemented - CandleBarData trait needs
/// `taker_buy_vol()`, `taker_sell_vol()`, and timestamp fields.
#[derive(Debug, Clone, PartialEq)]
pub struct CvdOutput {
    /// CVD line data
    pub cvd: IndicatorData,
    /// Reserved field for native use (not set by Rust)
    pub visible_range: VisibleRange,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CvdSettings {
    /// Buy delta (key: "buy")
    pub buy: IndicatorSettingsItem,
    /// Sell delta (key: "sell")
    pub sell: IndicatorSettingsItem,
}

/// DMA input parameters.
/// 
/// - **Names**: Short Period, Long Period, MA
/// - **Default**: fast=10 (2~60), slow=50 (2~250), signal=10 (2~100)
/// - **Output**: DMA, MA
#[derive(Debug, Clone, PartialEq)]
pub struct DmaInputParams {
    /// Fast MA period (short period), default: 10
    pub fast_length: i64,
    /// Slow MA period (long period), default: 50
    pub slow_length: i64,
    /// Signal line smoothing period (MA), default: 10
    pub signal_length: i64,
}

/// DMA indicator output.
/// 
/// DMA shows the difference between fast and slow moving averages.
#[derive(Debug, Clone, PartialEq)]
pub struct DmaOutput {
    /// DIF line (fast MA - slow MA)
    pub dif: IndicatorData,
    /// AMA/DIF_MA line (moving average of DIF)
    pub dif_ma: IndicatorData,
    /// Reserved field for native use (not set by Rust)
    pub visible_range: VisibleRange,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DmaSettings {
    pub fast_length: IndicatorSettingsItem,
    pub slow_length: IndicatorSettingsItem,
    /// DIF line (key: "out1")
    pub dif: IndicatorSettingsItem,
    /// DIF MA line (key: "out2")
    pub dif_ma: IndicatorSettingsItem,
}

/// DMI input parameters.
/// 
/// - **Names**: DI, ADX
/// - **Default**: length_di=14 (2~90), length_adx=6 (2~60)
/// - **Output**: PDI (+DI), MDI (-DI), ADX, ADXR
#[derive(Debug, Clone, PartialEq)]
pub struct DmiInputParams {
    /// DI calculation period, default: 14
    pub length_di: i64,
    /// ADX smoothing period, default: 6
    pub length_adx: i64,
    /// Rust-resolved `kline_dmi_indicator_gray_key` branch.
    pub optimized: bool,
}

/// DMI indicator output.
/// 
/// DMI measures trend strength and direction.
#[derive(Debug, Clone, PartialEq)]
pub struct DmiOutput {
    /// +DI (Positive Directional Indicator)
    pub pdi: IndicatorData,
    /// -DI (Negative Directional Indicator)
    pub mdi: IndicatorData,
    /// ADX (Average Directional Index, trend strength)
    pub adx: IndicatorData,
    /// ADXR (ADX Rating, smoothed ADX)
    pub adxr: IndicatorData,
    /// Reserved field for native use (not set by Rust)
    pub visible_range: VisibleRange,
}

/// Strongly-typed DMI settings.
/// 
/// Keys: lengthDI, lengthADX, plusDI, minusDI, adx, adxr
#[derive(Debug, Clone, PartialEq)]
pub struct DmiSettings {
    /// DI calculation period (key: "lengthDI")
    pub length_di: IndicatorSettingsItem,
    /// ADX smoothing period (key: "lengthADX")
    pub length_adx: IndicatorSettingsItem,
    /// +DI / PDI line (key: "plusDI")
    pub pdi: IndicatorSettingsItem,
    /// -DI / MDI line (key: "minusDI")
    pub mdi: IndicatorSettingsItem,
    /// ADX line (key: "adx")
    pub adx: IndicatorSettingsItem,
    /// ADXR line (key: "adxr")
    pub adxr: IndicatorSettingsItem,
}

/// Donchian Channels input parameters.
/// 
/// - **Names**: length
/// - **Default**: length=20 (range: 1~200)
/// - **Output**: upper, basis, lower
#[derive(Debug, Clone, PartialEq)]
pub struct DonchianChannelsInputParams {
    /// Lookback period, default: 20
    pub length: i64,
}

/// Donchian Channels output.
#[derive(Debug, Clone, PartialEq)]
pub struct DonchianChannelsOutput {
    /// Upper band: highest high over period
    pub upper: IndicatorData,
    /// Basis: (upper + lower) / 2
    pub basis: IndicatorData,
    /// Lower band: lowest low over period
    pub lower: IndicatorData,
    /// Reserved field for native use (not set by Rust)
    pub visible_range: VisibleRange,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DonchianChannelsSettings {
    /// Length parameter (key: "length")
    pub length: IndicatorSettingsItem,
    /// Upper band (key: "upper")
    pub upper: IndicatorSettingsItem,
    /// Basis line (key: "basis")
    pub basis: IndicatorSettingsItem,
    /// Lower band (key: "lower")
    pub lower: IndicatorSettingsItem,
    /// Background fill (key: "background")
    pub background: IndicatorSettingsItem,
}

/// Drawing line settings that persist across sessions
/// 
/// This struct holds user preferences for drawing lines, including colors,
/// line width, style, and Fibonacci levels.
#[derive(Debug, Clone, PartialEq)]
pub struct DrawingLineSettings {
    /// Line color in hex format with alpha (e.g., "#0569FFFF")
    pub line_color: String,
    /// Line color block identifier for theme colors
    pub line_color_blk: Option<String>,
    /// Fill color in hex format with alpha
    pub fill_color: String,
    /// Fill color block identifier
    pub fill_color_blk: Option<String>,
    /// Line width (default: 1.0)
    pub line_width: f64,
    /// Line style: 0 = Solid, 1 = Dash
    pub line_style: u8,
    /// Whether all lines are hidden
    pub hide_all_lines: bool,
    /// Fibonacci price levels with selection state
    pub fibonacci_levels: Vec<FibonacciLevelSetting>,
}

/// EMA input parameters - 6 period values.
/// 
/// - **Count**: 6
/// - **Names**: EMA1, EMA2, EMA3, EMA4, EMA5, EMA6
/// - **Default**: `[5, 10, 20, 30, 60, 120]`
/// - **Range**: 2 ~ 1000
/// 
/// Example: `[5, 10, 20, 30, 60, 120]` creates EMA5, EMA10, EMA20, EMA30, EMA60, EMA120.
#[derive(Debug, Clone, PartialEq)]
pub struct EmaInputParams {
    /// Period values for each EMA line (6 values: EMA1-EMA6)
    pub periods: Vec<i64>,
}

/// Strongly-typed EMA settings (6 lines).
/// Same structure as MaSettings but with "EMA" prefix titles.
#[derive(Debug, Clone, PartialEq)]
pub struct EmaSettings {
    pub ema1: IndicatorSettingsItem,
    pub ema2: IndicatorSettingsItem,
    pub ema3: IndicatorSettingsItem,
    pub ema4: IndicatorSettingsItem,
    pub ema5: IndicatorSettingsItem,
    pub ema6: IndicatorSettingsItem,
}

/// EMV input parameters.
/// 
/// - **Names**: EMV, MAEMV
/// - **Default**: length=14 (2~90), ma_period=9 (2~60)
/// - **Output**: EMV, MAEMV
#[derive(Debug, Clone, PartialEq)]
pub struct EmvInputParams {
    /// EMV calculation period, default: 14
    pub length: i64,
    /// MA smoothing period, default: 9
    pub ma_period: i64,
}

/// EMV indicator output.
/// 
/// EMV relates price change to volume to measure ease of price movement.
#[derive(Debug, Clone, PartialEq)]
pub struct EmvOutput {
    /// EMV line (ease of movement value)
    pub emv: IndicatorData,
    /// Moving average of EMV
    pub ma_emv: IndicatorData,
    /// Reserved field for native use (not set by Rust)
    pub visible_range: VisibleRange,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EmvSettings {
    pub length: IndicatorSettingsItem,
    /// EMV line (key: "out")
    pub emv: IndicatorSettingsItem,
    /// MA of EMV line (key: "ma1") - includes both param and output
    pub ma_emv: IndicatorSettingsItem,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnvelopeSettings {
    pub length: IndicatorSettingsItem,
    pub mult: IndicatorSettingsItem,
    pub lookback: IndicatorSettingsItem,
    pub upper: IndicatorSettingsItem,
    pub lower: IndicatorSettingsItem,
    pub background: IndicatorSettingsItem,
}

/// Fibonacci level setting
/// 
/// Represents a single Fibonacci retracement level with its visibility state.
#[derive(Debug, Clone, PartialEq)]
pub struct FibonacciLevelSetting {
    /// Price level as string (e.g., "0.618")
    pub price: String,
    /// Whether this level is selected/visible
    pub selected: bool,
    /// Color in hex format (optional, for custom colors)
    pub color: Option<String>,
}

/// Payload for indicator cache migration from native to Rust.
/// 
/// This is designed to be forward-compatible: when we add new indicator-related preference keys,
/// native can include them here without changing the method signature.
#[derive(Debug, Clone, PartialEq)]
pub struct IndicatorCacheMigrationPayload {
    /// String values keyed by **base key** (reserved for future indicator settings).
    pub strings: HashMap<String, String>,
    /// String-list values keyed by **base key** (e.g. `KEY_CHART_SELECTED_MAIN_INDICATOR`).
    pub string_lists: HashMap<String, Vec<String>>,
}

/// A single indicator line data.
/// 
/// This is the core structure for indicator line outputs, similar to
/// ObjC's `OKKLineIndicatorLineData`.
#[derive(Debug, Clone, PartialEq)]
pub struct IndicatorData {
    /// Name/title of this line (e.g., "MA5", "K", "MACD")
    pub name: String,
    /// Calculated values, one per candle bar
    pub data: Vec<f64>,
    /// Number of leading values that are invalid/blank
    pub prefix_blank_length: u64,
    /// Reserved field for native use (not set by Rust)
    pub visible_range: VisibleRange,
}

/// Indicator metadata without UI-specific or localized content.
/// 
/// Native platforms should complement this with:
/// - `title`: Localized display title
/// - `settingsTitle`: Localized settings panel title
/// - `settingsDescription`: Localized description
/// - `layerClass`: Platform-specific rendering class
/// - `trackEvent`: Analytics event name
#[derive(Debug, Clone, PartialEq)]
pub struct IndicatorMeta {
    /// Unique identifier for the indicator (e.g., "MA", "EMA", "MACD")
    pub name: String,
    /// Server synchronization key (e.g., "MA:MAIN", "MACD")
    pub server_key: String,
    /// Indicator type (main or sub)
    pub indicator_type: IndicatorType,
    /// Enable cases, if set, only the indicators in the enable_cases will be displayed
    pub enable_cases: Option<Vec<ChartSettingCase>>,
    /// Disable cases, only effective when enable_cases is not set
    pub disable_cases: Option<Vec<ChartSettingCase>>,
    /// AB test feature flag key (mirrors ObjC `OKKLineIndicatorRegistration.abTestKey`).
    /// 
    /// Used by `IndicatorManager` to consult the `experiment` crate (which bridges to
    /// native `OKFSkylab`) and hide the indicator when the user is not in the treatment
    /// bucket. See `meta_ab_hit` in `indicator_manager.rs`.
    pub ab_test_key: Option<String>,
}

/// All registered indicators with their metadata.
#[derive(Debug, Clone, PartialEq)]
pub struct IndicatorRegistry {
    /// All available indicators
    pub indicators: Vec<IndicatorMeta>,
}

/// Complete settings configuration for an indicator.
#[derive(Debug, Clone, PartialEq)]
pub struct IndicatorSettings {
    /// The indicator name this settings belongs to
    pub indicator_name: IndicatorName,
    /// The indicator type (main or sub)
    pub indicator_type: IndicatorType,
    /// List of parameter configurations
    pub items: Vec<IndicatorSettingsItem>,
}

/// A single settings parameter configuration.
/// 
/// This defines the constraints and default value for one parameter
/// of an indicator's settings.
#[derive(Debug, Clone, PartialEq)]
pub struct IndicatorSettingsItem {
    /// Unique key for this parameter (e.g., "out1", "length")
    pub key: String,
    /// Display title for this parameter
    pub title: String,
    /// Whether this parameter has a numeric value
    pub has_param: bool,
    /// Minimum allowed value (if has_param is true)
    pub minimum: Option<f64>,
    /// Maximum allowed value (if has_param is true)
    pub maximum: Option<f64>,
    /// Default value (if has_param is true)
    pub default_value: Option<f64>,
    /// Current value (if has_param is true)
    pub current_value: Option<f64>,
    /// Whether this parameter has a color
    pub has_color: bool,
    /// Current colors
    pub current_color: Option<String>,
    /// Static hex default. Mutually exclusive with `default_color_block` and `default_system_color`.
    pub default_color: Option<String>,
    /// Current color blocks
    pub current_color_block: Option<String>,
    /// Static color-block ID default (e.g., "A1", "A2"). Mutually exclusive with `default_color` and `default_system_color`.
    pub default_color_block: Option<String>,
    /// Runtime-resolved system color default (K-line theme rise/fall).
    /// Native side reads this and injects the live theme color when both
    /// `current_color` / `current_color_block` are unset, and uses this to
    /// avoid persisting the resolved hex back to Rust state on save.
    /// Mutually exclusive with `default_color` and `default_color_block`.
    pub default_system_color: DefaultSystemColor,
    /// Default/current line width in logical pixels. V1 persists this beside each visual item,
    /// and the cloud protocol carries the same value as `lineWidth`.
    pub default_line_width: Option<f64>,
    pub current_line_width: Option<f64>,
    /// Whether this parameter has an alpha/opacity value
    pub has_alpha: bool,
    /// Default alpha value (0.0-1.0)
    pub default_alpha: Option<f64>,
    /// Current alphas value (0.0-1.0)
    pub current_alpha: Option<f64>,
    /// Whether this parameter can be selected/enabled
    pub can_selected: bool,
    /// Default selected state
    pub is_default_selected: Option<bool>,
    /// Current selected state
    pub is_selected: Option<bool>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IndicatorSettingsLocalState {
    /// Version counter for indicator lists — bumped when `initialize` completes.
    /// Native calls `get_main_indicators()` / `get_sub_indicators()` on change.
    pub indicators_version: u64,
    /// Monotonic revision of the case-scoped shared selection/parameter repository.
    /// Every live settings interactor mirrors this value, regardless of which page wrote it.
    pub configuration_version: u64,
    /// Currently selected main indicator names.
    pub selected_main: Vec<String>,
    /// Currently selected sub indicator names (including trading-data indicators).
    pub selected_all_sub: Vec<String>,
}

/// Indicator visible state for cloud sync: server_key → whether shown.
/// Case-agnostic domain type, moved down from retail-kline 2026-05-31 (step 1A)
/// so both candle-support LocalInteractor and retail-kline SyncInteractor can share it.
#[derive(Debug, Clone, PartialEq)]
pub struct IndicatorVisibleState {
    pub server_key: String,
    pub visible: bool,
}

/// Per-VC SCENE inputs feeding the derived properties: `enable_compare_btn` /
/// `enable_formula_btn` / `enable_date_btn` on `ChartSettingsPanelConfig`,
/// and the `derived_*` flags on both configs.
/// 
/// Native pushes this Record once per chart open (and again on input changes —
/// instrument switch, klineViewType toggle, isArbi flip) via
/// `set_chart_context(...)` on either `panel_config` or `page_config`
/// (both write the same shared storage). Only Cefi / Lite charts need to
/// push it; for other setting cases the case-level gate already produces
/// `false` for the derived flags regardless of context.
/// 
/// ACCOUNT facts (MCC/PM level, TM hosting visibility, isSupportAllBizTypes,
/// spot cost basis) deliberately do NOT live here: they are session-global,
/// pushed by retail-kline into `settings::trade_facts` — native no longer
/// forwards them.
#[derive(Debug, Clone, PartialEq)]
pub struct KLineSettingsFeatureContext {
    /// Arbitrage chart (multi-instrument spread).
    pub is_arbi: bool,
    /// Trading-bot grid chart (`OKKLineBotChartVC`). The bot chart runs as
    /// `SettingCase::Cefi` with a main-chart layout on the shared singleton,
    /// so this explicit flag is the only way the derived gates can tell it
    /// apart (compare/formula/date buttons and quick order are always off there).
    /// NOT WIRED YET: no caller passes `true` today — the bot chart keeps its
    /// native overrides and does not call `applyToolbarBtnFlags`; wire this
    /// when the bot chart migrates to the Rust settings path.
    pub is_bot: bool, // default = false
    /// Current instrument's business line.
    pub biz_type: KLineSettingsBizType,
    /// Pre-market trading instrument.
    pub is_pre_market: bool,
    /// `true` = OKX native renderer; `false` = TradingView mode.
    /// KLine engine has only these two implementations.
    pub is_native_chart: bool,
}

/// KDJ input parameters.
/// 
/// - **Names**: Calculation Period, MA Period 1, MA Period 2
/// - **Default**: length=9, smooth_k=3, smooth_d=3 (range: 2~90)
/// - **Output**: K, D, J
#[derive(Debug, Clone, PartialEq)]
pub struct KdjInputParams {
    /// Calculation period, default: 9
    pub length: i64,
    /// K line smoothing period (MA period 1), default: 3
    pub smooth_k: i64,
    /// D line smoothing period (MA period 2), default: 3
    pub smooth_d: i64,
}

/// KDJ indicator output.
/// 
/// KDJ is a momentum indicator that shows the location of the close
/// relative to the high-low range over a set number of periods.
#[derive(Debug, Clone, PartialEq)]
pub struct KdjOutput {
    /// K line (fast stochastic)
    pub k: IndicatorData,
    /// D line (slow stochastic, SMA of K)
    pub d: IndicatorData,
    /// J line (3K - 2D, more sensitive)
    pub j: IndicatorData,
    /// Reserved field for native use (not set by Rust)
    pub visible_range: VisibleRange,
}

/// Strongly-typed KDJ settings.
/// 
/// Keys: length, smoothK, smoothD, k, d, j
#[derive(Debug, Clone, PartialEq)]
pub struct KdjSettings {
    /// Calculation period (key: "length")
    pub length: IndicatorSettingsItem,
    /// K line smoothing (key: "smoothK")
    pub smooth_k: IndicatorSettingsItem,
    /// D line smoothing (key: "smoothD")
    pub smooth_d: IndicatorSettingsItem,
    /// K line (key: "k")
    pub k: IndicatorSettingsItem,
    /// D line (key: "d")
    pub d: IndicatorSettingsItem,
    /// J line (key: "j")
    pub j: IndicatorSettingsItem,
}

/// KST input parameters.
/// 
/// - **Names**: roc1..roc4, sma1..sma4, signal
/// - **Default**: roc1=10, roc2=15, roc3=20, roc4=30, sma1=10, sma2=10, sma3=10, sma4=15, signal=9
/// - **Output**: KST, Signal
#[derive(Debug, Clone, PartialEq)]
pub struct KstInputParams {
    /// ROC period 1, default: 10
    pub roc1: i64,
    /// ROC period 2, default: 15
    pub roc2: i64,
    /// ROC period 3, default: 20
    pub roc3: i64,
    /// ROC period 4, default: 30
    pub roc4: i64,
    /// SMA period 1, default: 10
    pub sma1: i64,
    /// SMA period 2, default: 10
    pub sma2: i64,
    /// SMA period 3, default: 10
    pub sma3: i64,
    /// SMA period 4, default: 15
    pub sma4: i64,
    /// Signal line period, default: 9
    pub signal_length: i64,
}

/// KST indicator output.
#[derive(Debug, Clone, PartialEq)]
pub struct KstOutput {
    /// KST line data
    pub kst: IndicatorData,
    /// Signal line (SMA of KST)
    pub signal: IndicatorData,
    /// Reserved field for native use (not set by Rust)
    pub visible_range: VisibleRange,
}

#[derive(Debug, Clone, PartialEq)]
pub struct KstSettings {
    /// ROC period 1 (key: "roc1")
    pub roc1: IndicatorSettingsItem,
    /// ROC period 2 (key: "roc2")
    pub roc2: IndicatorSettingsItem,
    /// ROC period 3 (key: "roc3")
    pub roc3: IndicatorSettingsItem,
    /// ROC period 4 (key: "roc4")
    pub roc4: IndicatorSettingsItem,
    /// SMA period 1 (key: "sma1")
    pub sma1: IndicatorSettingsItem,
    /// SMA period 2 (key: "sma2")
    pub sma2: IndicatorSettingsItem,
    /// SMA period 3 (key: "sma3")
    pub sma3: IndicatorSettingsItem,
    /// SMA period 4 (key: "sma4")
    pub sma4: IndicatorSettingsItem,
    /// Signal line period (key: "signalLength")
    pub signal_length: IndicatorSettingsItem,
    /// KST line (key: "kst")
    pub kst: IndicatorSettingsItem,
    /// Signal line (key: "signal")
    pub signal: IndicatorSettingsItem,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LsRatioSettings {
    pub ratio: IndicatorSettingsItem,
    pub long: IndicatorSettingsItem,
    pub short: IndicatorSettingsItem,
}

/// MA input parameters - 6 period values.
/// 
/// - **Count**: 6
/// - **Names**: MA1, MA2, MA3, MA4, MA5, MA6
/// - **Default**: `[5, 10, 20, 30, 60, 120]`
/// - **Range**: 2 ~ 1000
/// 
/// Example: `[5, 10, 20, 30, 60, 120]` creates MA5, MA10, MA20, MA30, MA60, MA120.
#[derive(Debug, Clone, PartialEq)]
pub struct MaInputParams {
    /// Period values for each MA line (6 values: MA1-MA6)
    pub periods: Vec<i64>,
}

/// Strongly-typed MA settings (11 lines, matching Android V1/SP/cloud schema).
/// 
/// Keys: out1 through out11.
#[derive(Debug, Clone, PartialEq)]
pub struct MaSettings {
    /// MA line 1 (key: "out1")
    pub ma1: IndicatorSettingsItem,
    /// MA line 2 (key: "out2")
    pub ma2: IndicatorSettingsItem,
    /// MA line 3 (key: "out3")
    pub ma3: IndicatorSettingsItem,
    /// MA line 4 (key: "out4")
    pub ma4: IndicatorSettingsItem,
    /// MA line 5 (key: "out5")
    pub ma5: IndicatorSettingsItem,
    /// MA line 6 (key: "out6")
    pub ma6: IndicatorSettingsItem,
    pub ma7: IndicatorSettingsItem,
    pub ma8: IndicatorSettingsItem,
    pub ma9: IndicatorSettingsItem,
    pub ma10: IndicatorSettingsItem,
    pub ma11: IndicatorSettingsItem,
}

/// MACD input parameters.
/// 
/// - **Names**: Short Period, Long Period, MA Period
/// - **Default**: fast=12, slow=26, signal=9 (range: 2~200)
/// - **Output**: DIFF, DEA, STICK (histogram)
#[derive(Debug, Clone, PartialEq)]
pub struct MacdInputParams {
    /// Fast EMA period (short period), default: 12
    pub fast_length: i64,
    /// Slow EMA period (long period), default: 26
    pub slow_length: i64,
    /// Signal line smoothing period (MA period), default: 9
    pub signal_length: i64,
}

/// MACD indicator output.
/// 
/// MACD shows the relationship between two moving averages of a price.
#[derive(Debug, Clone, PartialEq)]
pub struct MacdOutput {
    /// DIF/DIFF line (fast EMA - slow EMA)
    pub diff: IndicatorData,
    /// DEA line (EMA of DIFF)
    pub dea: IndicatorData,
    /// STICK/Histogram ((DIFF - DEA) × 2)
    pub stick: IndicatorData,
    /// Reserved field for native use (not set by Rust)
    pub visible_range: VisibleRange,
}

/// Strongly-typed MACD settings.
/// 
/// Field naming matches `MacdOutput` for consistency: diff, dea, stick
#[derive(Debug, Clone, PartialEq)]
pub struct MacdSettings {
    /// Fast EMA period (key: "fastLength")
    pub fast_length: IndicatorSettingsItem,
    /// Slow EMA period (key: "slowLength")
    pub slow_length: IndicatorSettingsItem,
    /// Signal smoothing period (key: "signalLength")
    pub signal_length: IndicatorSettingsItem,
    /// DIFF line (key: "macd")
    pub diff: IndicatorSettingsItem,
    /// DEA line (key: "signal")
    pub dea: IndicatorSettingsItem,
    /// STICK/Histogram (key: "histogram")
    pub stick: IndicatorSettingsItem,
}

/// MassIndex input parameters.
/// 
/// - **Names**: sumLength
/// - **Default**: sum_length=10
/// - **Output**: MassIndex
/// 
/// Note: EMA smoothing period is a fixed algorithm constant (9), inlined in
/// `indicator_calculator::mass_index`. Not exposed here as a parameter,
/// matching ObjC `OKTKLineMassIndexIndicatorLayer.paramNames`.
#[derive(Debug, Clone, PartialEq)]
pub struct MassIndexInputParams {
    /// Summation period, default: 10
    pub sum_length: i64,
}

/// MassIndex indicator output.
#[derive(Debug, Clone, PartialEq)]
pub struct MassIndexOutput {
    /// Mass Index line data
    pub mass_index: IndicatorData,
    /// Reserved field for native use (not set by Rust)
    pub visible_range: VisibleRange,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MassIndexSettings {
    /// Summation period (key: "sumLength")
    pub sum_length: IndicatorSettingsItem,
    /// Mass Index line (key: "massIndex")
    pub mass_index: IndicatorSettingsItem,
}

/// MFI input parameters.
/// 
/// - **Names**: period
/// - **Default**: period=14 (range: 2~100)
/// - **Output**: MFI line (0~100)
#[derive(Debug, Clone, PartialEq)]
pub struct MfiInputParams {
    /// MFI calculation period, default: 14
    pub period: i64,
}

/// MFI indicator output.
#[derive(Debug, Clone, PartialEq)]
pub struct MfiOutput {
    /// MFI line data (0~100 range)
    pub mfi: IndicatorData,
    /// Reserved field for native use (not set by Rust)
    pub visible_range: VisibleRange,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MfiSettings {
    /// Period parameter (key: "period")
    pub period: IndicatorSettingsItem,
    /// Overbought level (key: "overboughtLevel")
    pub overbought_level: IndicatorSettingsItem,
    /// Oversold level (key: "oversoldLevel")
    pub oversold_level: IndicatorSettingsItem,
    /// MFI line (key: "mfi")
    pub mfi: IndicatorSettingsItem,
    /// OB line (key: "obLine")
    pub ob_line: IndicatorSettingsItem,
    /// OS line (key: "osLine")
    pub os_line: IndicatorSettingsItem,
    /// Background (key: "background")
    pub background: IndicatorSettingsItem,
}

/// MTM input parameters.
/// 
/// - **Names**: MTM, MAMTM
/// - **Default**: length=12 (2~120), ma_period=6 (2~60)
/// - **Output**: MTM, MAMTM
#[derive(Debug, Clone, PartialEq)]
pub struct MtmInputParams {
    /// MTM calculation period, default: 12
    pub length: i64,
    /// Signal line MA period, default: 6
    pub ma_period: i64,
}

/// MTM indicator output.
/// 
/// MTM measures the rate of change in price.
#[derive(Debug, Clone, PartialEq)]
pub struct MtmOutput {
    /// MTM line (price difference over period)
    pub mtm: IndicatorData,
    /// Moving average of MTM
    pub ma_mtm: IndicatorData,
    /// Reserved field for native use (not set by Rust)
    pub visible_range: VisibleRange,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MtmSettings {
    pub length: IndicatorSettingsItem,
    /// MTM line (key: "out")
    pub mtm: IndicatorSettingsItem,
    /// MA of MTM (key: "ma1") - includes both param and output
    pub ma_mtm: IndicatorSettingsItem,
}

/// Generic multi-line indicator output.
/// 
/// Used when the number of output lines depends on input parameters.
/// For example, `ma(&[5, 10, 20])` returns 3 lines: MA5, MA10, MA20.
#[derive(Debug, Clone, PartialEq)]
pub struct MultiLineOutput {
    /// Collection of indicator lines (e.g., MA5, MA10, MA20)
    pub lines: Vec<IndicatorData>,
    /// Reserved field for native use (not set by Rust)
    pub visible_range: VisibleRange,
}

/// NVI input parameters.
/// 
/// - **Names**: emaLength
/// - **Default**: emaLength=255 (range: 1~500)
/// - **Output**: NVI, MANVI (EMA of NVI)
#[derive(Debug, Clone, PartialEq)]
pub struct NviInputParams {
    /// EMA smoothing period of NVI, default: 255
    pub ema_length: i64,
}

/// NVI indicator output.
/// 
/// NVI starts at 1000 and additively tracks percent close-change on bars
/// where volume decreases. MANVI is the EMA of NVI.
#[derive(Debug, Clone, PartialEq)]
pub struct NviOutput {
    /// NVI line (Negative Volume Index)
    pub nvi: IndicatorData,
    /// EMA of NVI
    pub ma_nvi: IndicatorData,
    /// Reserved field for native use (not set by Rust)
    pub visible_range: VisibleRange,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NviSettings {
    /// EMA period of NVI (key: "ema")
    pub ema_length: IndicatorSettingsItem,
    /// NVI line (key: "nvi")
    pub nvi: IndicatorSettingsItem,
    /// MANVI (EMA of NVI) line (key: "manvi")
    pub ma_nvi: IndicatorSettingsItem,
}

/// NWE/Envelope input parameters.
/// 
/// - **Names**: BandWidth, Mult, Lookback
/// - **Default**: bandwidth=8 (2~15), mult=3 (1~10), lookback=100 (50~1000)
/// - **Output**: Upper, Lower bands
#[derive(Debug, Clone, PartialEq)]
pub struct NweInputParams {
    /// Kernel bandwidth parameter, default: 8.0
    pub bandwidth: f64,
    /// Envelope multiplier, default: 3.0
    pub mult: f64,
    /// Lookback period, default: 100
    pub lookback: i64,
}

/// NWE indicator output.
/// 
/// NWE uses kernel regression to create smooth price envelopes.
#[derive(Debug, Clone, PartialEq)]
pub struct NweOutput {
    /// Upper envelope band
    pub ub: IndicatorData,
    /// Lower envelope band
    pub lb: IndicatorData,
    /// Reserved field for native use (not set by Rust)
    pub visible_range: VisibleRange,
}

/// Drawing line entity - Core data model
/// 
/// Mirrors iOS `OKKLineDrawingData` structure.
/// Only stores essential fields; complex backend format handling is done in convertor.
/// 
/// Exported to native as `OKKLineDrawingData`.
#[derive(Debug, Clone, PartialEq)]
pub struct OKKLineDrawingData {
    /// Unique identifier (lineId)
    pub line_id: String,
    /// Instrument ID (e.g., "BTC-USDT")
    /// Note: Not in iOS OKKLineDrawingData, but needed for database storage
    pub inst_id: String,
    /// Line type (type)
    pub line_type: OKKLineDrawingType,
    /// Line color in hex format (color) - e.g., "#0569FFFF"
    pub color: String,
    /// Color block identifier for theme (colorBlk)
    pub color_blk: Option<String>,
    /// Fill color for shapes (fillColor)
    pub fill_color: Option<String>,
    /// Fill color block identifier (fillColorBlk)
    pub fill_color_blk: Option<String>,
    /// Line width (lineWidth)
    pub line_width: f64,
    /// Line style (lineStyle)
    pub line_style: OKKLineDrawingStyle,
    /// Whether the line is locked (lock)
    pub lock: bool,
    /// Whether the line is deleted - soft delete (deleted)
    pub deleted: bool,
    /// Whether the line is synced to server (sync)
    pub sync: bool,
    /// Coordinate points
    pub points: Vec<OKKLineDrawingPoint>,
    /// Fibonacci levels (prices) - only for Fibonacci type
    pub prices: Option<Vec<OKKLineFibonacciLevel>>,
    /// Bars covered by a measurement line (barsCount) - only for Measurement
    /// type; web renders the stored value verbatim.
    pub bars_count: Option<i32>,
    /// Volume sum of a measurement line (volumeSum) - only for Measurement type
    pub volume_sum: Option<f64>,
    /// Creation timestamp in milliseconds (createTimeInterval)
    pub create_time_interval: f64,
    /// Opaque server lineData JSON. Native edits only patch fields they own;
    /// every unrecognized style field is preserved verbatim for re-upload.
    pub raw_line_data: Option<String>,
}

/// Drawing point (used for backend communication)
/// 
/// Exported to native as `OKKLineDrawingPoint`.
#[derive(Debug, Clone, PartialEq)]
pub struct OKKLineDrawingPoint {
    /// Timestamp in milliseconds (for backend)
    pub timestamp: i64,
    /// Price value
    pub price: f64,
    /// Offset (usually 0)
    pub offset: i32,
}

/// Fibonacci level data (only for Fibonacci line type)
/// 
/// Exported to native as `OKKLineFibonacciLevel`.
#[derive(Debug, Clone, PartialEq)]
pub struct OKKLineFibonacciLevel {
    /// Price level as string (e.g., "0.618")
    pub price: String,
    /// Whether this level is selected/visible
    pub selected: bool,
    /// Color in hex format
    pub color: String,
}

/// OBV input parameters.
/// 
/// - **Names**: MAOBV
/// - **Default**: ma_period=30 (range: 2~100)
/// - **Output**: OBV, MAOBV
#[derive(Debug, Clone, PartialEq)]
pub struct ObvInputParams {
    /// MA smoothing period, default: 30
    pub ma_period: i64,
}

/// OBV indicator output.
/// 
/// OBV uses volume flow to predict changes in price.
#[derive(Debug, Clone, PartialEq)]
pub struct ObvOutput {
    /// OBV line (cumulative volume)
    pub obv: IndicatorData,
    /// Moving average of OBV
    pub ma_obv: IndicatorData,
    /// Reserved field for native use (not set by Rust)
    pub visible_range: VisibleRange,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ObvSettings {
    /// OBV line (key: "out")
    pub obv: IndicatorSettingsItem,
    /// MA of OBV line (key: "ma1") - includes both param and output
    pub ma_obv: IndicatorSettingsItem,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OiSettings {
    /// OI line (key: "out")
    pub oi: IndicatorSettingsItem,
}

/// Payload for period cache migration from native to Rust.
/// 
/// This is designed to be forward-compatible: when we add new period-related preference keys,
/// native can include them here without changing the method signature.
#[derive(Debug, Clone, PartialEq)]
pub struct PeriodCacheMigrationPayload {
    /// String values keyed by **base key** (e.g. `KEY_CHART_PERIOD_CURRENT`).
    pub strings: HashMap<String, String>,
    /// String-list values keyed by **base key** (e.g. `KEY_CHART_PERIODS_USER_PREFERRED`).
    pub string_lists: HashMap<String, Vec<String>>,
}

/// PMO input parameters.
/// 
/// - **Names**: length1, length2, signalLength
/// - **Default**: length1=35, length2=20, signalLength=10 (range: each 1~200)
/// - **Output**: PMO, Signal
#[derive(Debug, Clone, PartialEq)]
pub struct PmoInputParams {
    /// Smoothing period for ROC EMA, default: 35
    pub length1: i64,
    /// EMA period applied to (smoothedROC × 10) to get PMO, default: 20
    pub length2: i64,
    /// EMA period of PMO to get Signal, default: 10
    pub signal_length: i64,
}

/// PMO indicator output.
#[derive(Debug, Clone, PartialEq)]
pub struct PmoOutput {
    /// PMO line: EMA(EMA(ROC×100, length1) × 10, length2)
    pub pmo: IndicatorData,
    /// Signal line: EMA(PMO, signalLength)
    pub signal: IndicatorData,
    /// Reserved field for native use (not set by Rust)
    pub visible_range: VisibleRange,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PmoSettings {
    /// ROC EMA smoothing length (key: "length1"), default 35.
    pub length1: IndicatorSettingsItem,
    /// PMO EMA length (key: "length2"), default 20.
    pub length2: IndicatorSettingsItem,
    /// Signal EMA length (key: "signal" — matches ObjC PMO param key), default 10.
    pub signal_length: IndicatorSettingsItem,
    /// PMO line (key: "pmo").
    pub pmo: IndicatorSettingsItem,
    /// Signal line (key: "signal_line" — matches ObjC PMO line key).
    pub signal: IndicatorSettingsItem,
}

/// PSY input parameters.
/// 
/// - **Names**: PSY, MAPSY
/// - **Default**: length=12, ma_period=6 (range: 2~100)
/// - **Output**: PSY, MAPSY
#[derive(Debug, Clone, PartialEq)]
pub struct PsyInputParams {
    /// PSY calculation period, default: 12
    pub length: i64,
    /// Signal line MA period, default: 6
    pub ma_period: i64,
}

/// PSY indicator output.
/// 
/// PSY measures the ratio of rising periods to total periods,
/// reflecting market psychology.
#[derive(Debug, Clone, PartialEq)]
pub struct PsyOutput {
    /// PSY line (percentage of up days)
    pub psy: IndicatorData,
    /// Moving average of PSY
    pub psy_ma: IndicatorData,
    /// Reserved field for native use (not set by Rust)
    pub visible_range: VisibleRange,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PsySettings {
    pub length: IndicatorSettingsItem,
    /// PSY line (key: "out")
    pub psy: IndicatorSettingsItem,
    /// MA of PSY (key: "ma1") - includes both param and output
    pub psy_ma: IndicatorSettingsItem,
}

/// PVT indicator output.
#[derive(Debug, Clone, PartialEq)]
pub struct PvtOutput {
    /// PVT line data
    pub pvt: IndicatorData,
    /// Reserved field for native use (not set by Rust)
    pub visible_range: VisibleRange,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PvtSettings {
    /// PVT line (key: "pvt")
    pub pvt: IndicatorSettingsItem,
}

/// QQE input parameters.
/// 
/// - **Names**: rsiLength, smoothingFactor, fastFactor, slowFactor
/// - **Default**: rsi_length=14, smoothing_factor=2, fast_factor=2.618, slow_factor=4.236
/// - **Output**: QQE, UpperBand, LowerBand
#[derive(Debug, Clone, PartialEq)]
pub struct QqeInputParams {
    /// RSI calculation period, default: 14
    pub rsi_length: i64,
    /// RSI smoothing factor, default: 2
    pub smoothing_factor: i64,
    /// Fast trailing level multiplier, default: 2.618
    pub fast_factor: f64,
    /// Slow trailing level multiplier, default: 4.236
    pub slow_factor: f64,
}

/// QQE indicator output.
#[derive(Debug, Clone, PartialEq)]
pub struct QqeOutput {
    /// QQE line (smoothed RSI)
    pub qqe: IndicatorData,
    /// Upper band
    pub upper_band: IndicatorData,
    /// Lower band
    pub lower_band: IndicatorData,
    /// Reserved field for native use (not set by Rust)
    pub visible_range: VisibleRange,
}

#[derive(Debug, Clone, PartialEq)]
pub struct QqeSettings {
    /// RSI calculation period (key: "rsiLength")
    pub rsi_length: IndicatorSettingsItem,
    /// Smoothing factor (key: "smoothingFactor")
    pub smoothing_factor: IndicatorSettingsItem,
    /// Fast trailing level multiplier (key: "fastFactor")
    pub fast_factor: IndicatorSettingsItem,
    /// Slow trailing level multiplier (key: "slowFactor")
    pub slow_factor: IndicatorSettingsItem,
    /// QQE line (key: "qqe")
    pub qqe: IndicatorSettingsItem,
    /// Upper band (key: "upperBand")
    pub upper_band: IndicatorSettingsItem,
    /// Lower band (key: "lowerBand")
    pub lower_band: IndicatorSettingsItem,
}

/// RCI Ribbon input parameters.
/// 
/// - **Names**: shortLength, middleLength, longLength
/// - **Default**: 10 / 30 / 50 (range: each 1~200)
/// - **Output**: 3 lines (short / middle / long), each is rolling Spearman
/// rank correlation × 100 between time (position) and price (close) over
/// the window length. Ties in price use average rank.
#[derive(Debug, Clone, PartialEq)]
pub struct RciRibbonInputParams {
    /// Short window length, default: 10
    pub short_length: i64,
    /// Middle window length, default: 30
    pub middle_length: i64,
    /// Long window length, default: 50
    pub long_length: i64,
}

/// RCI Ribbon indicator output.
#[derive(Debug, Clone, PartialEq)]
pub struct RciRibbonOutput {
    /// Short-period RCI line
    pub short_rci: IndicatorData,
    /// Middle-period RCI line
    pub middle_rci: IndicatorData,
    /// Long-period RCI line
    pub long_rci: IndicatorData,
    /// Reserved field for native use (not set by Rust)
    pub visible_range: VisibleRange,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RciRibbonSettings {
    /// Short RCI period (key: "short"), default 10 — matches ObjC RCI Ribbon.
    pub short_length: IndicatorSettingsItem,
    /// Middle RCI period (key: "middle"), default 30.
    pub middle_length: IndicatorSettingsItem,
    /// Long RCI period (key: "long"), default 50.
    pub long_length: IndicatorSettingsItem,
    /// Short RCI line (key: "short_rci").
    pub short_rci: IndicatorSettingsItem,
    /// Middle RCI line (key: "middle_rci").
    pub middle_rci: IndicatorSettingsItem,
    /// Long RCI line (key: "long_rci").
    pub long_rci: IndicatorSettingsItem,
}

/// ROC input parameters.
/// 
/// - **Names**: ROC, MAROC
/// - **Default**: length=12 (2~120), ma_period=6 (2~60)
/// - **Output**: ROC, MAROC
#[derive(Debug, Clone, PartialEq)]
pub struct RocInputParams {
    /// ROC calculation period, default: 12
    pub length: i64,
    /// Signal line MA period, default: 6
    pub ma_period: i64,
}

/// ROC indicator output.
/// 
/// ROC measures the percentage change in price over a specified period.
#[derive(Debug, Clone, PartialEq)]
pub struct RocOutput {
    /// ROC line (percentage price change)
    pub roc: IndicatorData,
    /// Moving average of ROC
    pub ma_roc: IndicatorData,
    /// Reserved field for native use (not set by Rust)
    pub visible_range: VisibleRange,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RocSettings {
    pub length: IndicatorSettingsItem,
    /// ROC line (key: "out")
    pub roc: IndicatorSettingsItem,
    /// MA of ROC (key: "ma1") - includes both param and output
    pub ma_roc: IndicatorSettingsItem,
}

/// RSI input parameters - 3 period values with bounds.
/// 
/// - **Count**: 3 periods + 2 bounds
/// - **Names**: RSI1, RSI2, RSI3, UB (upper bound), LB (lower bound)
/// - **Default periods**: `[6, 12, 24]` (range: 2 ~ 1000)
/// - **Default bounds**: upper=70, lower=30 (range: 0 ~ 100)
#[derive(Debug, Clone, PartialEq)]
pub struct RsiInputParams {
    /// Period values for each RSI line (3 values: RSI1-RSI3)
    pub periods: Vec<i64>,
    /// Upper bound (overbought level), default: 70
    pub upper_bound: f64,
    /// Lower bound (oversold level), default: 30
    pub lower_bound: f64,
}

/// Strongly-typed RSI settings.
/// 
/// Keys: out1, out2, out3, upperBound, lowerBound, background
#[derive(Debug, Clone, PartialEq)]
pub struct RsiSettings {
    /// RSI line 1 (key: "out1")
    pub rsi1: IndicatorSettingsItem,
    /// RSI line 2 (key: "out2")
    pub rsi2: IndicatorSettingsItem,
    /// RSI line 3 (key: "out3")
    pub rsi3: IndicatorSettingsItem,
    /// Upper bound (key: "upperBound")
    pub upper_bound: IndicatorSettingsItem,
    /// Lower bound (key: "lowerBound")
    pub lower_bound: IndicatorSettingsItem,
    /// Background fill (key: "background")
    pub background: IndicatorSettingsItem,
}

/// RVI input parameters.
/// 
/// - **Names**: length
/// - **Default**: length=10 (range: 2~200)
/// - **Output**: RVI, Signal
#[derive(Debug, Clone, PartialEq)]
pub struct RviInputParams {
    /// Calculation period, default: 10
    pub length: i64,
}

/// RVI indicator output.
#[derive(Debug, Clone, PartialEq)]
pub struct RviOutput {
    /// RVI line data
    pub rvi: IndicatorData,
    /// Signal line (smoothed RVI)
    pub signal: IndicatorData,
    /// Reserved field for native use (not set by Rust)
    pub visible_range: VisibleRange,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RviSettings {
    /// Calculation period (key: "length")
    pub length: IndicatorSettingsItem,
    /// RVI line (key: "rvi")
    pub rvi: IndicatorSettingsItem,
    /// Signal line (key: "signal")
    pub signal: IndicatorSettingsItem,
}

/// SAR input parameters.
/// 
/// - **Names**: SAR (no user-adjustable parameters in UI)
/// - **Default**: af_start=0.02, af_step=0.02, af_max=0.2
/// - **Output**: SAR points
/// 
/// Note: These are internal calculation parameters, not exposed in settings UI.
#[derive(Debug, Clone, PartialEq)]
pub struct SarInputParams {
    /// Initial acceleration factor, default: 0.02
    pub af_start: f64,
    /// Acceleration factor step, default: 0.02
    pub af_step: f64,
    /// Maximum acceleration factor, default: 0.2
    pub af_max: f64,
}

/// SAR indicator output.
/// 
/// SAR provides potential reversal points and trailing stop levels.
#[derive(Debug, Clone, PartialEq)]
pub struct SarOutput {
    /// SAR values (stop and reverse price levels)
    pub sar: IndicatorData,
    /// Direction flag (0.0 = bearish, 1.0 = bullish, 2.0 = reversal)
    pub flag: IndicatorData,
    /// Reserved field for native use (not set by Rust)
    pub visible_range: VisibleRange,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SarSettings {
    /// SAR line (key: "out")
    pub sar: IndicatorSettingsItem,
}

/// Payload for migrating native settings cache to Rust storage.
/// 
/// Native code should populate this record with values read from UserDefaults/SharedPreferences
/// before calling `migrate_native_cache()`.
#[derive(Debug, Clone, PartialEq)]
pub struct SettingsCacheMigrationPayload {
    /// Integer values to migrate (key → value).
    pub i64s: HashMap<String, i64>,
    /// Float values to migrate (key → value).
    pub f64s: HashMap<String, f64>,
    /// Boolean values to migrate (key → value).
    pub bools: HashMap<String, bool>,
    /// String values to migrate (key → value).
    pub strings: HashMap<String, String>,
}

/// Read-side snapshot of cached settings scalars — the counterpart of the write-side
/// `SettingsCacheMigrationPayload`. Lets native replace ~25 per-key `store_get_*` FFI
/// reads with a single call. Keyed by base key; values already carry the exact same
/// per-key defaults as `store_get_bool/int/double` (this snapshot reuses them).
#[derive(Debug, Clone, PartialEq)]
pub struct SettingsCacheSnapshot {
    pub bools: HashMap<String, bool>,
    pub i64s: HashMap<String, i64>,
    pub f64s: HashMap<String, f64>,
}

/// SKDJ input parameters.
/// 
/// - **Names**: Calculation Period, MA Period
/// - **Default**: length=9, smooth=3 (range: 2~90)
/// - **Output**: K, D
#[derive(Debug, Clone, PartialEq)]
pub struct SkdjInputParams {
    /// Calculation period, default: 9
    pub length: i64,
    /// Smoothing period (MA period), default: 3
    pub smooth: i64,
}

/// SKDJ indicator output.
/// 
/// SKDJ is a slower version of KDJ that uses additional smoothing.
#[derive(Debug, Clone, PartialEq)]
pub struct SkdjOutput {
    /// K line (smoothed stochastic)
    pub k: IndicatorData,
    /// D line (SMA of K)
    pub d: IndicatorData,
    /// Reserved field for native use (not set by Rust)
    pub visible_range: VisibleRange,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SkdjSettings {
    pub length: IndicatorSettingsItem,
    pub smooth: IndicatorSettingsItem,
    pub k: IndicatorSettingsItem,
    pub d: IndicatorSettingsItem,
}

/// SLOPE input parameters. `ma_type` uses the V1 wire codes: SMA=0, EMA=1,
/// HMA=2, VWMA=3 and LSMA=4. The Rust boundary accepts the same codes as V1;
/// string-to-code compatibility is handled by the sync/request decoder.
#[derive(Debug, Clone, PartialEq)]
pub struct SlopeInputParams {
    pub lookback: i64,
    pub length: i64,
    pub ma_type: i64,
}

/// SLOPE output (single line).
#[derive(Debug, Clone, PartialEq)]
pub struct SlopeOutput {
    pub slope: IndicatorData,
    pub visible_range: VisibleRange,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SlopeSettings {
    pub lookback: IndicatorSettingsItem,
    pub length: IndicatorSettingsItem,
    /// V1-compatible MA type code: SMA=0, EMA=1, HMA=2, VWMA=3, LSMA=4.
    /// The Android Rust settings surface can render this as a selector using
    /// the shared MA type catalog; the wire converter accepts both codes and labels.
    pub ma_type: IndicatorSettingsItem,
}

/// SMI input parameters.
/// 
/// - **Names**: bbLength, bbMult, kcLength, kcMult, momLength
/// - **Default**: 20, 2.0, 20, 1.5, 12
#[derive(Debug, Clone, PartialEq)]
pub struct SmiInputParams {
    /// Bollinger Band length, default: 20
    pub bb_length: i64,
    /// Bollinger Band multiplier, default: 2.0
    pub bb_mult: f64,
    /// Keltner Channel length, default: 20
    pub kc_length: i64,
    /// Keltner Channel ATR multiplier, default: 1.5
    pub kc_mult: f64,
    /// Momentum (linear regression) length, default: 12
    pub mom_length: i64,
}

/// SMI indicator output.
#[derive(Debug, Clone, PartialEq)]
pub struct SmiOutput {
    /// Momentum value (positive = bullish, negative = bearish)
    pub val: IndicatorData,
    /// Squeeze state: 1.0 = sqzOn, -1.0 = sqzOff, 0.0 = noSqz
    pub sqz: IndicatorData,
    /// Reserved field for native use (not set by Rust)
    pub visible_range: VisibleRange,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SmiSettings {
    /// BB Length (key: "bbLength")
    pub bb_length: IndicatorSettingsItem,
    /// BB Mult (key: "bbMult")
    pub bb_mult: IndicatorSettingsItem,
    /// KC Length (key: "kcLength")
    pub kc_length: IndicatorSettingsItem,
    /// KC Mult (key: "kcMult")
    pub kc_mult: IndicatorSettingsItem,
    /// Momentum Length (key: "momLength")
    pub mom_length: IndicatorSettingsItem,
    /// Momentum histogram (key: "VAL")
    pub val: IndicatorSettingsItem,
    /// Squeeze state points (key: "SQZ")
    pub sqz: IndicatorSettingsItem,
}

/// SR input parameters.
/// 
/// - **Names**: Ticks Per Row
/// - **Default**: length=10 (range: 2~500)
/// - **Output**: Resistance line, Support line
#[derive(Debug, Clone, PartialEq)]
pub struct SrInputParams {
    /// Ticks per row (price grouping granularity), default: 10
    pub length: i64,
}

/// SR indicator output.
/// 
/// SR identifies key support and resistance levels based on volume profile.
#[derive(Debug, Clone, PartialEq)]
pub struct SrOutput {
    /// Support level (highest volume below current price)
    pub support: IndicatorData,
    /// Resistance level (highest volume above current price)
    pub resistance: IndicatorData,
    /// Reserved field for native use (not set by Rust)
    pub visible_range: VisibleRange,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SrSettings {
    pub length: IndicatorSettingsItem,
    pub resistance: IndicatorSettingsItem,
    pub support: IndicatorSettingsItem,
}

/// StochRSI input parameters.
/// 
/// - **Names**: RSI Length, Stochastic Length, K, D
/// - **Default**: rsi=14 (3~100), stoch=14 (3~100), smooth_k=3 (2~50), smooth_d=3 (2~50)
/// - **Output**: STOCHRSI, MASTOCHRSI
#[derive(Debug, Clone, PartialEq)]
pub struct StochRsiInputParams {
    /// RSI calculation period, default: 14
    pub length_rsi: i64,
    /// Stochastic calculation period, default: 14
    pub length_stoch: i64,
    /// K line smoothing period, default: 3
    pub smooth_k: i64,
    /// D line smoothing period, default: 3
    pub smooth_d: i64,
}

/// StochRSI indicator output.
/// 
/// StochRSI applies the Stochastic formula to RSI values.
#[derive(Debug, Clone, PartialEq)]
pub struct StochRsiOutput {
    /// StochRSI value (normalized RSI)
    pub stoch_rsi: IndicatorData,
    /// Moving average of StochRSI (signal line)
    pub ma: IndicatorData,
    /// Reserved field for native use (not set by Rust)
    pub visible_range: VisibleRange,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StochRsiSettings {
    pub length_rsi: IndicatorSettingsItem,
    pub length_stoch: IndicatorSettingsItem,
    pub smooth_k: IndicatorSettingsItem,
    pub smooth_d: IndicatorSettingsItem,
    /// StochRSI line (key: "k")
    pub stoch_rsi: IndicatorSettingsItem,
    /// MA of StochRSI (key: "d")
    pub ma: IndicatorSettingsItem,
}

/// SuperTrend input parameters.
/// 
/// - **Names**: ATR Length, Multiplier
/// - **Default**: atr_length=14 (1~100), mult=3.0 (1~10)
/// - **Output**: SuperTrend line with up/down colors
#[derive(Debug, Clone, PartialEq)]
pub struct SuperTrendInputParams {
    /// ATR calculation period (ATR Length), default: 14
    pub atr_length: i64,
    /// ATR multiplier for bands (Multiplier), default: 3.0
    pub mult: f64,
}

/// SuperTrend indicator output.
/// 
/// SuperTrend is a trend-following indicator based on ATR.
#[derive(Debug, Clone, PartialEq)]
pub struct SuperTrendOutput {
    /// SuperTrend line (dynamic support/resistance)
    pub supertrend: IndicatorData,
    /// Trend direction (1.0 = bullish/up, -1.0 = bearish/down)
    pub direction: IndicatorData,
    /// HL2 (High + Low) / 2, the basis for calculation
    pub hl2: IndicatorData,
    /// Reserved field for native use (not set by Rust)
    pub visible_range: VisibleRange,
}

/// Strongly-typed SuperTrend settings.
/// 
/// Keys: atrLength, mult, upColor, downColor
#[derive(Debug, Clone, PartialEq)]
pub struct SuperTrendSettings {
    /// ATR calculation period (key: "atrLength")
    pub atr_length: IndicatorSettingsItem,
    /// ATR multiplier (key: "mult")
    pub mult: IndicatorSettingsItem,
    /// Uptrend color (key: "upColor")
    pub up_color: IndicatorSettingsItem,
    /// Downtrend color (key: "downColor")
    pub down_color: IndicatorSettingsItem,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TakerVolSettings {
    pub total: IndicatorSettingsItem,
    pub buy: IndicatorSettingsItem,
    pub sell: IndicatorSettingsItem,
}

/// Payload for trade settings cache migration from native to Rust.
/// 
/// This is designed to be forward-compatible: when we add new trade-related preference keys,
/// native can include them here without changing the method signature.
#[derive(Debug, Clone, PartialEq)]
pub struct TradeCacheMigrationPayload {
    /// Boolean values keyed by **base key** (e.g. `KEY_TRADE_ENABLE_REVERT_POSITION`).
    pub bools: HashMap<String, bool>,
    /// Integer values keyed by **base key** (e.g. `KEY_TRADE_EXPAND_ORDER_LINE_POSITION`).
    pub ints: HashMap<String, i64>,
}

/// Trade-related base keys used by Rust chart settings persistence.
/// 
/// This record is UniFFI-exported so native bridges (Swift/ObjC/Kotlin) can avoid hardcoding
/// string literals and drifting from Rust.
#[derive(Debug, Clone, PartialEq)]
pub struct TradeSettingKeys {
    /// Trade setting key (native uses OKBGlobalLocalSetting with `kSettingsKey(...)`).
    pub trade_enable_revert_position: String,
    /// Trade setting key (native uses OKBGlobalLocalSetting with `kSettingsKey(...)`).
    pub trade_enable_display_all_positions: String,
    /// Trade setting key (native uses OKBGlobalLocalSetting with `kSettingsKey(...)`).
    pub trade_display_liquidation_px: String,
    /// Trade setting key (native uses OKBGlobalLocalSetting with `kSettingsKey(...)`).
    pub trade_display_balance_px: String,
    /// Expand order line display (native: `KLineExpandOrderLineKey`).
    pub trade_expand_order_line: String,
    /// Expand position line display (native: `KLineExpandPositionLineKey`).
    pub trade_expand_position_line: String,
    /// Order line position (native: `KLineExpandOrderLinePositionKey`).
    pub trade_expand_order_line_position: String,
    /// Position line position (native: `KLineExpandPositionLinePositionKey`).
    pub trade_expand_position_line_position: String,
    /// Position TP/SL profit style (native: `KLinePositionTPSLProfitStyleKey`).
    pub trade_position_tpsl_profit_style: String,
}

/// Batch update payload for `ChartTradeSettings`.
/// 
/// Use this with `apply_batch()` to update multiple settings in a single operation,
/// reducing lock contention and I/O operations.
/// 
/// Only fields set to `Some(value)` will be updated; `None` fields are skipped.
#[derive(Debug, Clone, PartialEq)]
pub struct TradeSettingsBatch {
    /// Enable reverse position opening.
    pub enable_revert_position: Option<bool>,
    /// Enable order extended line.
    pub enable_expand_order_line: Option<bool>,
    /// Enable position extended line.
    pub enable_expand_position_line: Option<bool>,
    /// Order extended line position (0 = Left, 1 = Right).
    pub order_line_position: Option<i64>,
    /// Position extended line position (0 = Left, 1 = Right).
    pub position_line_position: Option<i64>,
    /// Position TP/SL profit display style (0 = Rate, 1 = Amount).
    pub position_tpsl_profit_style: Option<i64>,
    /// Show estimated liquidation price.
    pub show_liquidation_px: Option<bool>,
    /// Show break-even price.
    pub show_balance_px: Option<bool>,
    /// Enable display all positions.
    pub enable_display_all_positions: Option<bool>,
}

/// Read-side snapshot of the 9 trade settings — the counterpart of the write-side
/// `TradeSettingsBatch`. Lets native replace 9 per-getter FFI reads with one call.
/// All fields are present (no `Option`) since a read always yields a concrete value.
#[derive(Debug, Clone, PartialEq)]
pub struct TradeSettingsSnapshot {
    pub enable_revert_position: bool,
    pub enable_expand_order_line: bool,
    pub enable_expand_position_line: bool,
    pub order_line_position: i64,
    pub position_line_position: i64,
    pub position_tpsl_profit_style: i64,
    pub show_liquidation_px: bool,
    pub show_balance_px: bool,
    pub enable_display_all_positions: bool,
}

/// TRIX input parameters.
/// 
/// - **Names**: TRIX, MATRIX
/// - **Default**: length=12, ma_period=9 (range: 2~100)
/// - **Output**: TRIX, MATRIX
#[derive(Debug, Clone, PartialEq)]
pub struct TrixInputParams {
    /// TRIX calculation period, default: 12
    pub length: i64,
    /// Signal line MA period, default: 9
    pub ma_period: i64,
}

/// TRIX indicator output.
/// 
/// TRIX is a momentum oscillator that shows the percent rate of change
/// of a triple exponentially smoothed moving average.
#[derive(Debug, Clone, PartialEq)]
pub struct TrixOutput {
    /// TRIX line (rate of change of triple EMA)
    pub trix: IndicatorData,
    /// Moving average of TRIX (signal line)
    pub matrix: IndicatorData,
    /// Reserved field for native use (not set by Rust)
    pub visible_range: VisibleRange,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TrixSettings {
    pub length: IndicatorSettingsItem,
    /// TRIX line (key: "out")
    pub trix: IndicatorSettingsItem,
    /// MA of TRIX (key: "ma1") - includes both param and output
    pub matrix: IndicatorSettingsItem,
}

/// TSI input parameters.
/// 
/// - **Names**: length
/// - **Default**: length=14
/// - **Output**: TSI (single-line, signal unused on chart)
#[derive(Debug, Clone, PartialEq)]
pub struct TsiInputParams {
    /// Rolling window length, default: 14, range: [2, 200]
    pub length: i64,
}

/// TSI indicator output.
#[derive(Debug, Clone, PartialEq)]
pub struct TsiOutput {
    /// TSI line data
    pub tsi: IndicatorData,
    /// Reserved field for native use (not set by Rust)
    pub visible_range: VisibleRange,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TsiSettings {
    /// Rolling window length (server key: "length"), default 14, range 2..=200
    /// (matches Android V1 `TSIIndicator`).
    pub length: IndicatorSettingsItem,
    /// TSI line color (server key: "lineColor")
    pub line_color: IndicatorSettingsItem,
    /// Bull gradient color (server key: "bullColor"), V1 default A4.
    pub bull_color: IndicatorSettingsItem,
    /// Bear gradient color (server key: "bearColor"), V1 default A1.
    pub bear_color: IndicatorSettingsItem,
}

/// Visible range for indicator data display.
/// 
/// **Reserved field for native (iOS/Android) use.**
/// This field is not set by the Rust calculation engine.
/// Native platforms can use this to track the visible/displayed range.
#[derive(Debug, Clone, PartialEq)]
pub struct VisibleRange {
    /// Start index (inclusive)
    pub start: u64,
    /// End index (exclusive)
    pub end: u64,
}

/// VOL input parameters.
/// 
/// - **Names**: MA1, MA2
/// - **Default**: ma1=5, ma2=10 (range: 2~500)
/// - **Output**: Volume bars, MA1, MA2
#[derive(Debug, Clone, PartialEq)]
pub struct VolInputParams {
    /// First MA period, default: 5
    pub ma1_period: i64,
    /// Second MA period, default: 10
    pub ma2_period: i64,
}

/// VOL indicator output.
/// 
/// Volume with optional moving averages.
#[derive(Debug, Clone, PartialEq)]
pub struct VolOutput {
    /// Raw volume values
    pub volume: IndicatorData,
    /// Moving average lines of volume (e.g., MA5, MA10)
    pub ma_lines: Vec<IndicatorData>,
    /// Reserved field for native use (not set by Rust)
    pub visible_range: VisibleRange,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VolSettings {
    pub ma1: IndicatorSettingsItem,
    pub ma2: IndicatorSettingsItem,
    /// Volume bar transparency (key: "out")
    pub transparency: IndicatorSettingsItem,
}

/// VR input parameters.
/// 
/// - **Names**: VR, MAVR
/// - **Default**: length=26, ma_period=6 (range: 2~100)
/// - **Output**: VR, MAVR
#[derive(Debug, Clone, PartialEq)]
pub struct VrInputParams {
    /// VR calculation period, default: 26
    pub length: i64,
    /// Signal line MA period, default: 6
    pub ma_period: i64,
}

/// VR indicator output.
/// 
/// VR measures the ratio of up-volume to down-volume.
#[derive(Debug, Clone, PartialEq)]
pub struct VrOutput {
    /// VR line (volume ratio)
    pub vr: IndicatorData,
    /// Moving average of VR
    pub ma_vr: IndicatorData,
    /// Reserved field for native use (not set by Rust)
    pub visible_range: VisibleRange,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VrSettings {
    pub length: IndicatorSettingsItem,
    /// VR line (key: "out")
    pub vr: IndicatorSettingsItem,
    /// MA of VR (key: "ma1") - includes both param and output
    pub ma_vr: IndicatorSettingsItem,
}

/// VWAP input parameters.
/// 
/// - **Names**: Length
/// - **Default**: length=14 (range: 1~1000)
/// - **Output**: VWAP line
#[derive(Debug, Clone, PartialEq)]
pub struct VwapInputParams {
    /// VWAP calculation period (Length), default: 14
    pub length: i64,
}

/// VWAP indicator output.
/// 
/// VWAP represents the average price weighted by volume.
#[derive(Debug, Clone, PartialEq)]
pub struct VwapOutput {
    /// VWAP values
    pub vwap: IndicatorData,
    /// Reserved field for native use (not set by Rust)
    pub visible_range: VisibleRange,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VwapSettings {
    pub vwap: IndicatorSettingsItem,
}

/// VWMA input parameters - same structure as MA (6 period values).
/// 
/// - **Count**: 6
/// - **Names**: VWMA1, VWMA2, VWMA3, VWMA4, VWMA5, VWMA6
/// - **Default**: `[5, 10, 20, 30, 60, 120]`
/// - **Range**: 2 ~ 1000
/// 
/// Note: Cannot use a type alias because UniFFI does not export type aliases.
#[derive(Debug, Clone, PartialEq)]
pub struct VwmaInputParams {
    /// Period values for each VWMA line (6 values: VWMA1-VWMA6)
    pub periods: Vec<i64>,
}

/// WMA input parameters - same structure as MA (6 period values).
/// 
/// - **Count**: 6
/// - **Names**: WMA1, WMA2, WMA3, WMA4, WMA5, WMA6
/// - **Default**: `[5, 10, 20, 30, 60, 120]`
/// - **Range**: 2 ~ 1000
/// 
/// Note: Cannot use `type WmaInputParams = MaInputParams` because UniFFI
/// does not export type aliases. Must be a standalone struct.
#[derive(Debug, Clone, PartialEq)]
pub struct WmaInputParams {
    /// Period values for each WMA line (6 values: WMA1-WMA6)
    pub periods: Vec<i64>,
}

/// WR input parameters - 2 period values.
/// 
/// - **Count**: 2
/// - **Names**: WR1, WR2
/// - **Default**: `[10, 6]`
/// - **Range**: 2 ~ 100
#[derive(Debug, Clone, PartialEq)]
pub struct WrInputParams {
    /// Period values for each WR line (2 values: WR1, WR2)
    pub periods: Vec<i64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WrSettings {
    /// WR1 line — value is the period, color is the line color (key: "out1")
    pub wr1: IndicatorSettingsItem,
    /// WR2 line — value is the period, color is the line color (key: "out2")
    pub wr2: IndicatorSettingsItem,
}

// -----------------------------------------------------------------------------
// 3. Interfaces / Interactors / Services (22)
// -----------------------------------------------------------------------------

/// Trait for accessing candle bar OHLCV data.
/// 
/// Any type that provides OHLCV (Open, High, Low, Close, Volume) data
/// can implement this trait to be used with indicator calculations.
pub struct CandleBarData;
impl CandleBarData {
    /// Returns the turnover/amount (price × volume). Used by AVL indicator.
    /// Returns 0.0 if not available.
    pub fn amount(&self) -> f64 { todo!() }
    /// Returns the closing price (C in OHLCV)
    pub fn c(&self) -> f64 { todo!() }
    /// Returns the coin-denominated volume (base asset quantity).
    /// Used by AVL indicator which always needs coin volume regardless of display mode.
    pub fn coin_vol(&self) -> f64 { todo!() }
    /// Returns the highest price (H in OHLCV)
    pub fn h(&self) -> f64 { todo!() }
    /// Returns the lowest price (L in OHLCV)
    pub fn l(&self) -> f64 { todo!() }
    /// Returns the opening price (O in OHLCV)
    pub fn o(&self) -> f64 { todo!() }
    /// Returns the taker buy volume. Used by CVD indicator.
    /// Returns 0.0 if not available.
    pub fn taker_buy_vol(&self) -> f64 { todo!() }
    /// Returns the taker sell volume. Used by CVD indicator.
    /// Returns 0.0 if not available.
    pub fn taker_sell_vol(&self) -> f64 { todo!() }
    /// Returns the timestamp in seconds (Unix epoch). Used by CVD daily reset.
    /// Returns 0.0 if not available.
    pub fn ts(&self) -> f64 { todo!() }
    /// Returns the trading volume (V in OHLCV)
    pub fn v(&self) -> f64 { todo!() }
}

/// Cached OHLCV arrays for indicator calculations.
/// 
/// This is a UniFFI Object type that native platforms can hold and pass around
/// without copying the underlying data. The object is created in Rust and
/// native only uses it as an opaque handle.
/// 
/// # Lazy Loading
/// 
/// OHLCV arrays are computed lazily on first access and cached for reuse.
/// Uses `OnceLock` for thread-safe lazy initialization.
pub struct CandleDataWrapper;
impl CandleDataWrapper {
    pub fn blank() -> Arc<CandleDataWrapper> { todo!() }
    /// Creates a new candle data wrapper from pre-extracted arrays.
    /// This avoids FFI callbacks by accepting data directly as primitive arrays.
    pub fn from_arrays(opens: Vec<f64>, highs: Vec<f64>, lows: Vec<f64>, closes: Vec<f64>, volumes: Vec<f64>, coin_volumes: Vec<f64>, amounts: Vec<f64>, taker_buy_vols: Vec<f64>, taker_sell_vols: Vec<f64>, timestamps: Vec<f64>) -> Arc<CandleDataWrapper> { todo!() }
    /// Creates a new candle data wrapper from bar data.
    pub fn from_bars(bars: Vec<Arc<dyn CandleBarData>>) -> Arc<CandleDataWrapper> { todo!() }
}

pub struct CandleSettingsInteractor;
impl CandleSettingsInteractor {
    pub fn get_all_period_items(&self) -> Vec<ChartToolbarPeriodItem> { todo!() }
    pub fn get_all_sub_indicator_items(&self) -> Vec<ChartToolbarIndicatorItem> { todo!() }
    pub fn get_landscape_period_items(&self) -> Vec<ChartToolbarPeriodItem> { todo!() }
    pub fn get_main_indicator_items(&self) -> Vec<ChartToolbarIndicatorItem> { todo!() }
    pub fn get_sub_indicator_items(&self) -> Vec<ChartToolbarIndicatorItem> { todo!() }
    pub fn get_trade_indicator_items(&self) -> Vec<ChartToolbarIndicatorItem> { todo!() }
    /// Pull the full indicators configuration (available lists, items with display names).
    pub fn indicators_configuration(&self) -> Arc<ChartIndicatorsConfiguration> { todo!() }
    /// One-shot snapshot of all cached settings scalars, so native can replace ~25
    /// per-key `store_get_*` FFI reads (`p_loadInteractorCachedSettings`) with a single
    /// call. Reuses `store_get_bool/int/double` directly, so every value carries the
    /// exact same per-key default as the individual reads — no default duplication.
    /// The key lists mirror the `store_get_bool` / `store_get_int` match arms.
    pub fn load_cached_settings_snapshot(&self) -> SettingsCacheSnapshot { todo!() }
    /// Load all current settings from Rust stores into State.
    /// Called automatically in constructor; also callable for manual reload.
    pub fn load_settings(&self) { todo!() }
    pub fn notify_migration_complete(&self) { todo!() }
    pub fn notify_page_config_changed(&self) { todo!() }
    pub fn notify_panel_config_changed(&self) { todo!() }
    pub fn on_clear(&self) { todo!() }
    /// Pull page config for reading/modifying page-level feature flags.
    /// After modifying, call `notify_page_config_changed()` to update observers.
    pub fn page_config(&self) -> Arc<ChartSettingsPageConfig> { todo!() }
    /// Pull panel config for reading/modifying panel-level feature flags.
    /// After modifying, call `notify_panel_config_changed()` to update observers.
    pub fn panel_config(&self) -> Arc<ChartSettingsPanelConfig> { todo!() }
    /// Pull the full periods configuration (available lists, items with display names).
    pub fn periods_configuration(&self) -> Arc<ChartPeriodsConfiguration> { todo!() }
    /// Stop-gap teardown (TEMPORARY — target state is the stream APIs,
    /// which need no teardown call).
    /// 
    /// One idempotent call at host lifecycle end that breaks the
    /// UniFFI handleMap cycle for any registered callback observers,
    /// ends active streams, and stops effects. On Kotlin call BEFORE
    /// `destroy()`. Includes `stop()`.
    pub fn release(&self) { todo!() }
    /// Remove the state observer (UniFFI export).
    /// 
    /// DEPRECATED: callback-based observation. Use `state_stream()` —
    /// it needs no removal call. Until migrated, every
    /// `set_state_observer` MUST be paired with this (or `release()`)
    /// at host lifecycle end, or the UniFFI handleMap pins the
    /// observer and its host forever.
    pub fn remove_state_observer(&self) { todo!() }
    pub fn save_all_periods(&self, periods: Vec<String>) { todo!() }
    pub fn save_prefer_periods(&self, periods: Vec<String>) { todo!() }
    pub fn save_selected_all_sub_indicators(&self, indicators: Vec<String>) { todo!() }
    pub fn save_selected_main_indicators(&self, indicators: Vec<String>) { todo!() }
    pub fn save_selected_period(&self, period: String) { todo!() }
    pub fn save_selected_sub_indicators(&self, indicators: Vec<String>) { todo!() }
    pub fn save_selected_trade_indicators(&self, indicators: Vec<String>) { todo!() }
    pub fn set_cancel_order_confirmation(&self, enabled: bool) { todo!() }
    /// Enable or disable debug logging.
    /// 
    /// In release builds this is a no-op, but the symbol must exist
    /// so that UniFFI bindings generated in debug mode continue to
    /// link successfully against the release library.
    pub fn set_debug(&self, _enabled: bool) { todo!() }
    pub fn set_display_all_positions(&self, enabled: bool) { todo!() }
    pub fn set_expand_order_line(&self, enabled: bool) { todo!() }
    pub fn set_expand_position_line(&self, enabled: bool) { todo!() }
    /// `value`: 0 = Left, 1 = Right
    pub fn set_order_location(&self, value: i32) { todo!() }
    /// `value`: 0 = Left, 1 = Right
    pub fn set_position_location(&self, value: i32) { todo!() }
    /// `value`: 0 = Rate, 1 = Amount
    pub fn set_profit_style(&self, value: i32) { todo!() }
    pub fn set_revert_position(&self, enabled: bool) { todo!() }
    pub fn set_show_balance_px(&self, enabled: bool) { todo!() }
    pub fn set_show_liquidation_px(&self, enabled: bool) { todo!() }
    /// Set a state observer (UniFFI export).
    /// 
    /// DEPRECATED: callback-based observer. Use `state_stream()` —
    /// the callback observer pins the foreign object in UniFFI's
    /// static handleMap and produces a cross-language retain cycle
    /// (Android GC + iOS ARC).
    pub fn set_state_observer(&self, observer: Arc<dyn InteractorStateChangeObserver>) { todo!() }
    pub fn set_tpsl_adjustment_confirmation(&self, enabled: bool) { todo!() }
    pub fn setting_case(&self) -> ChartSettingCase { todo!() }
    /// Pull the underlying settings configuration (page + panel + trade).
    pub fn settings_configuration(&self) -> Arc<ChartSettingsConfiguration> { todo!() }
    /// Start the interactor lifecycle.
    /// 
    /// Calls the user's `InteractorLifecycle::start()` implementation.
    /// Override that method to add custom startup behavior (e.g., initial data loads).
    pub fn start(&self) { todo!() }
    /// Get the current state (UniFFI export).
    /// 
    /// Returns the External type (UniFFI-compatible).
    pub fn state(&self) -> CandleSettingsState { todo!() }
    /// Opt-in async state-change stream (cycle-free, signal-only).
    /// Single-subscriber, latest-wins. `changed()` resolves `true` on
    /// change (pull `state()` for the snapshot), `false` when the
    /// interactor is released or a newer subscription replaces this one.
    pub fn state_stream(&self) -> Arc<InteractorStateStream> { todo!() }
    /// Stop the interactor and cancel all effects.
    /// 
    /// Calls the user's `InteractorLifecycle::stop()` implementation first,
    /// then cancels all effects as a framework guarantee.
    pub fn stop(&self) { todo!() }
    pub fn store_get_bool(&self, key: String) -> bool { todo!() }
    pub fn store_get_double(&self, key: String, default_value: f64) -> f64 { todo!() }
    pub fn store_get_int(&self, key: String) -> i64 { todo!() }
    pub fn store_set_bool(&self, key: String, value: bool) { todo!() }
    pub fn store_set_double(&self, key: String, value: f64) { todo!() }
    pub fn store_set_int(&self, key: String, value: i64) { todo!() }
    /// Pull the trade features configuration.
    pub fn trade_features_configuration(&self) -> Arc<ChartTradeFeaturesConfiguration> { todo!() }
}

/// Case-scoped indicator settings persistence service.
/// 
/// Native platforms should prefer this typed API over raw key-value access.
pub struct ChartIndicatorStoreService;
impl ChartIndicatorStoreService {
    /// Returns all trading data indicator bar display names (indicator -> display name).
    pub fn all_trading_data_indicator_bar_names(&self) -> HashMap<String, String> { todo!() }
    /// Returns whether a value exists in the chart-settings preference store for `base_key`
    /// under this service's `SettingCase` namespacing.
    /// 
    /// `base_key` is intended to be **one of the four indicator base keys** exposed by
    /// `chart_indicator_keys()`:
    /// - `KEY_CHART_SELECTED_MAIN_INDICATOR`
    /// - `KEY_CHART_SELECTED_SUB_INDICATOR`
    /// - `KEY_CHART_SELECTED_ALL_SUB_INDICATOR`
    /// - `KEY_CHART_SELECTED_TRADING_DATA_INDICATOR`
    pub fn contains_preference_key(&self, base_key: String) -> bool { todo!() }
    /// Returns the persisted "all sub indicators" array for this service's `SettingCase`.
    /// 
    /// This reads from the case-scoped settings cache using
    /// `KEY_CHART_SELECTED_ALL_SUB_INDICATOR`.
    /// 
    /// If no cached value exists (i.e. the key has never been persisted), this returns an empty
    /// array (`Vec::new()`).
    pub fn current_all_sub_indicator_array(&self) -> Vec<String> { todo!() }
    /// Returns the persisted main indicator array for this service's `SettingCase`.
    pub fn current_main_indicator_array(&self) -> Vec<String> { todo!() }
    /// Returns the persisted sub indicator array for this service's `SettingCase`.
    pub fn current_sub_indicator_array(&self) -> Vec<String> { todo!() }
    /// Returns the persisted trading data indicator array for this service's `SettingCase`.
    pub fn current_trading_indicator_array(&self) -> Vec<String> { todo!() }
    /// Returns the default main indicator names for this service's `SettingCase`.
    /// 
    /// This list is derived from the indicator registry and respects `enable_cases` / `disable_cases`
    /// gating rules.
    pub fn default_main_indicator_array(&self) -> Vec<String> { todo!() }
    /// Returns the default sub indicator names for this service's `SettingCase`.
    /// 
    /// This list is derived from the indicator registry and respects `enable_cases` / `disable_cases`
    /// gating rules.
    pub fn default_sub_indicator_array(&self) -> Vec<String> { todo!() }
    /// Returns the default sub indicator names excluding all trading-data indicators.
    /// 
    /// This is equivalent to:
    /// `SUB_INDICATORS - TRADING_INDICATORS` (with the same case gating rules applied).
    pub fn default_sub_indicator_without_trading_array(&self) -> Vec<String> { todo!() }
    /// Returns the default trading-data indicator names for this service's `SettingCase`.
    /// 
    /// This list is derived from the indicator registry and respects `enable_cases` / `disable_cases`
    /// gating rules.
    pub fn default_trading_data_indicator_array(&self) -> Vec<String> { todo!() }
    /// Returns whether native indicator cache migration has been completed for this `SettingCase`.
    pub fn has_migrated_native_cache(&self) -> bool { todo!() }
    /// Migrates native-cached indicator settings into Rust-managed storage (batch payload).
    /// 
    /// - Supports migrating multiple keys in one call.
    /// - Skips writes for keys that already exist in Rust storage.
    /// 
    /// Keys are **base keys** and will be namespaced by Rust according to this service's `SettingCase`.
    /// 
    /// # Returns
    /// - `true` if migration flag was newly set by this call
    /// - `false` if migration had already been performed before
    pub fn migrate_native_cache(&self, payload: IndicatorCacheMigrationPayload) -> bool { todo!() }
    /// Marks native indicator cache migration as completed for this `SettingCase`.
    /// 
    /// Call this after native cache migration is done to prevent re-migration.
    pub fn migrated_native_cache_done(&self) { todo!() }
    /// Persists the "all sub indicators" array for this service's `SettingCase`.
    pub fn save_custom_all_sub_indicator_array(&self, indicator_array: Vec<String>) { todo!() }
    /// Persists the main indicator array for this service's `SettingCase`.
    pub fn save_custom_main_indicator_array(&self, indicator_array: Vec<String>) { todo!() }
    /// Persists the sub indicator array for this service's `SettingCase`.
    pub fn save_custom_sub_indicator_array(&self, indicator_array: Vec<String>) { todo!() }
    /// Persists the trading data indicator array for this service's `SettingCase`.
    pub fn save_custom_trading_indicator_array(&self, indicator_array: Vec<String>) { todo!() }
}

/// Rust equivalent of `OKKLineIndicatorsConfiguration`.
pub struct ChartIndicatorsConfiguration;
impl ChartIndicatorsConfiguration {
    /// Native: `allSubIndicatorItems`
    pub fn all_sub_indicator_items(&self) -> Vec<ChartToolbarIndicatorItem> { todo!() }
    /// Native: `allSubIndicators`
    pub fn all_sub_indicators(&self) -> Vec<String> { todo!() }
    pub fn all_trading_data_indicator_bar_names(&self) -> HashMap<String, String> { todo!() }
    pub fn editable(&self) -> bool { todo!() }
    /// Returns a case-scoped indicator settings persistence service.
    /// 
    /// This is backed by the same `ChartSettingManager` as this configuration instance, so reads
    /// and writes share the same settings namespace for `setting_case`.
    pub fn indicator_store(&self) -> Arc<ChartIndicatorStoreService> { todo!() }
    pub fn items_with_indicators(&self, indicators: Vec<String>, is_main: bool) -> Vec<ChartToolbarIndicatorItem> { todo!() }
    pub fn main_indicator_items(&self) -> Vec<ChartToolbarIndicatorItem> { todo!() }
    pub fn main_indicators(&self) -> Vec<String> { todo!() }
    pub fn save_custom_all_sub_indicator_array(&self, indicator_array: Vec<String>) { todo!() }
    pub fn save_custom_main_indicator_array(&self, indicator_array: Vec<String>) { todo!() }
    pub fn save_custom_sub_indicator_array(&self, indicator_array: Vec<String>) { todo!() }
    pub fn save_custom_trading_indicator_array(&self, indicator_array: Vec<String>) { todo!() }
    /// Native: `selectedAllSubIndicatorItems`
    pub fn selected_all_sub_indicator_items(&self) -> Vec<ChartToolbarIndicatorItem> { todo!() }
    /// Native: `selectedAllSubIndicators` (lazy load from cache)
    pub fn selected_all_sub_indicators(&self) -> Vec<String> { todo!() }
    /// Native: `selectedMainIndicatorItems`
    pub fn selected_main_indicator_items(&self) -> Vec<ChartToolbarIndicatorItem> { todo!() }
    /// Native: `selectedMainIndicators` (lazy load from cache)
    pub fn selected_main_indicators(&self) -> Vec<String> { todo!() }
    /// Native: `selectedSubIndicatorItems`
    pub fn selected_sub_indicator_items(&self) -> Vec<ChartToolbarIndicatorItem> { todo!() }
    /// Native: `selectedSubIndicators` (derived)
    pub fn selected_sub_indicators(&self) -> Vec<String> { todo!() }
    /// Native: `selectedTradeDataIndicatorItems`
    pub fn selected_trade_data_indicator_items(&self) -> Vec<ChartToolbarIndicatorItem> { todo!() }
    /// Native: `selectedTradeDataIndicators` (derived)
    pub fn selected_trade_data_indicators(&self) -> Vec<String> { todo!() }
    pub fn set_editable(&self, value: bool) { todo!() }
    pub fn set_main_indicators(&self, value: Vec<String>) { todo!() }
    pub fn set_sub_indicators(&self, value: Vec<String>) { todo!() }
    pub fn set_trade_data_indicators(&self, value: Vec<String>) { todo!() }
    pub fn sub_indicator_items(&self) -> Vec<ChartToolbarIndicatorItem> { todo!() }
    pub fn sub_indicators(&self) -> Vec<String> { todo!() }
    pub fn trade_data_indicators(&self) -> Vec<String> { todo!() }
    pub fn trade_indicator_items(&self) -> Vec<ChartToolbarIndicatorItem> { todo!() }
}

/// Case-scoped period settings persistence service.
/// 
/// Native platforms should prefer this typed API over raw key-value access.
pub struct ChartPeriodStoreService;
impl ChartPeriodStoreService {
    /// Returns whether a value exists in the chart-settings preference store for `base_key`
    /// under this service's `SettingCase` namespacing.
    /// 
    /// `base_key` is intended to be **one of the period base keys** exposed by this crate:
    /// - `KEY_CHART_PERIOD_CURRENT`
    /// - `KEY_CHART_PERIODS_USER_PREFERRED`
    /// - `KEY_CHART_ALL_AVAILABLE_PERIODS`
    pub fn contains_preference_key(&self, base_key: String) -> bool { todo!() }
    /// Returns the current order array for this service's `SettingCase`.
    pub fn current_order_array(&self) -> Vec<String> { todo!() }
    /// Returns whether native period cache migration has been completed for this `SettingCase`.
    pub fn has_migrated_native_cache(&self) -> bool { todo!() }
    /// Returns periods for landscape toolbar (max 10 items) without converting to `ToolbarPeriodItem`.
    pub fn landscape_periods(&self, current_order: Vec<String>) -> Vec<String> { todo!() }
    /// Returns the current selected period for this service's `SettingCase`.
    /// 
    /// - Parameter current_order: The available periods list; if the persisted value is not in
    /// this list, the fallback behavior matches the legacy native semantics.
    pub fn load_current_period(&self, current_order: Vec<String>) -> String { todo!() }
    /// Migrates native-cached period settings into Rust-managed storage (batch payload).
    /// 
    /// - Supports migrating multiple keys in one call.
    /// - Skips writes for keys that already exist in Rust storage.
    /// 
    /// Keys are **base keys** and will be namespaced by Rust according to this service's `SettingCase`.
    /// 
    /// # Returns
    /// - `true` if migration flag was newly set by this call
    /// - `false` if migration had already been performed before
    pub fn migrate_native_cache(&self, payload: PeriodCacheMigrationPayload) -> bool { todo!() }
    /// Marks native period cache migration as completed for this `SettingCase`.
    /// 
    /// Call this after native cache migration is done to prevent re-migration.
    pub fn migrated_native_cache_done(&self) { todo!() }
    /// Returns the user-default period list for this service's `SettingCase`.
    /// 
    /// If missing/empty, this method will write the defaults back to storage (matching
    /// existing legacy behavior).
    pub fn periods_by_user_default(&self) -> Vec<String> { todo!() }
    /// Persists the user-default "all periods" list for this service's `SettingCase`.
    pub fn save_all_available_periods(&self, periods: Vec<String>) { todo!() }
    /// Persists the current selected period for this service's `SettingCase`.
    pub fn save_current_period(&self, period: String) { todo!() }
    /// Persists the user-default period list for this service's `SettingCase`.
    pub fn save_user_preferred_periods(&self, periods: Vec<String>) { todo!() }
    /// Returns a display list for a landscape toolbar (legacy behavior: max 10 items).
    pub fn toolbar_landscape_period_item_array(&self, current_order: Vec<String>) -> Vec<ChartToolbarPeriodItem> { todo!() }
}

/// Rust equivalent of `OKKLinePeriodsConfiguration`.
pub struct ChartPeriodsConfiguration;
impl ChartPeriodsConfiguration {
    /// Native: `-allPeriodItemArray`
    pub fn all_period_item_array(&self) -> Vec<ChartToolbarPeriodItem> { todo!() }
    pub fn all_periods(&self) -> Vec<String> { todo!() }
    /// Native: `-allPeriodsByDefault`
    pub fn all_periods_by_default_items(&self) -> Vec<ChartToolbarPeriodItem> { todo!() }
    /// Native: `-changeWithAllPeriods:preferItems:`
    pub fn change_with_all_periods(&self, all_period_items: Vec<ChartToolbarPeriodItem>, prefer_items: Vec<ChartToolbarPeriodItem>) { todo!() }
    /// Native: `-configureCurrentPeriod:`
    pub fn configure_current_period(&self, period: String) { todo!() }
    pub fn default_preference_periods(&self) -> Vec<String> { todo!() }
    /// Native: `-displayNameWithPeriod:`
    pub fn display_name_with_period(&self, period: String) -> String { todo!() }
    pub fn display_style(&self) -> PeriodsStyle { todo!() }
    pub fn editable(&self) -> bool { todo!() }
    /// Native: `-forkPeriodConfig`
    pub fn fork_period_config(&self) -> Arc<ChartPeriodsConfiguration> { todo!() }
    /// Native: `-getItemsWithPeriods:`
    pub fn get_items_with_periods(&self, periods: Vec<String>) -> Vec<ChartToolbarPeriodItem> { todo!() }
    /// Native: `-getLandscapePeriodItemArray`
    pub fn get_landscape_period_item_array(&self) -> Vec<ChartToolbarPeriodItem> { todo!() }
    /// Returns periods for landscape toolbar (max 10 items) without converting to `ToolbarPeriodItem`.
    pub fn get_landscape_periods(&self) -> Vec<String> { todo!() }
    /// Native: `-getPeriodsWithItems:`
    pub fn get_periods_with_items(&self, items: Vec<ChartToolbarPeriodItem>) -> Vec<String> { todo!() }
    /// Native: `-itemWithPeriod:`
    pub fn item_with_period(&self, period: String) -> ChartToolbarPeriodItem { todo!() }
    /// Returns a case-scoped period settings persistence service.
    /// 
    /// This is backed by the same `ChartSettingManager` as this configuration instance, so reads
    /// and writes share the same settings namespace for `setting_case`.
    pub fn period_store(&self) -> Arc<ChartPeriodStoreService> { todo!() }
    /// Native: `-periodsByUserDefault`
    pub fn periods_by_user_default_items(&self) -> Vec<ChartToolbarPeriodItem> { todo!() }
    /// Returns the user preferred periods list.
    pub fn preference_periods(&self) -> Vec<String> { todo!() }
    /// Native: `-reloadCachedPreferPeriods`
    pub fn reload_cached_prefer_periods(&self) { todo!() }
    /// Persists the user-default "all periods" list for this configuration's `SettingCase`.
    /// 
    /// This updates both in-memory state and disk cache.
    pub fn save_all_available_periods(&self, periods: Vec<String>) { todo!() }
    /// Native: `-saveCurrentPeriod:`
    /// 
    /// NOTE: Native triggers `selectPeriodBlock`; Rust does not include UI callbacks.
    pub fn save_current_period(&self, period: String) { todo!() }
    /// Persists the user-default period list for this configuration's `SettingCase`.
    /// 
    /// This updates both in-memory state and disk cache.
    pub fn save_user_preferred_periods(&self, periods: Vec<String>) { todo!() }
    pub fn selected_period(&self) -> String { todo!() }
    pub fn set_display_style(&self, style: PeriodsStyle) { todo!() }
    pub fn set_editable(&self, value: bool) { todo!() }
    pub fn set_upper_limit(&self, value: i32) { todo!() }
    pub fn set_use_kline_cache(&self, value: bool) { todo!() }
    pub fn upper_limit(&self) -> i32 { todo!() }
    pub fn use_kline_cache(&self) -> bool { todo!() }
}

/// Rust equivalent of `OKKLineSettingsConfiguration`.
pub struct ChartSettingsConfiguration;
impl ChartSettingsConfiguration {
    /// Returns `true` if any chart-display feature is enabled
    /// (Y-axis mark, countdown, price alert, economic events).
    pub fn enable_chart_display(&self) -> bool { todo!() }
    /// Returns `true` if any of the main setting-panel buttons is enabled
    /// (landscape, indicators, drawings, settings, formula, compare).
    pub fn enable_main_setting_btns(&self) -> bool { todo!() }
    /// Returns `true` if any order-display feature is enabled
    /// (buy/sell point, order tags, position tags, asset cost tags).
    pub fn enable_order_display(&self) -> bool { todo!() }
    /// Returns `true` if the chart is in CeDeFi market-cap mode.
    pub fn is_ce_defi_market_cap(&self) -> bool { todo!() }
    /// Returns `true` if the chart is in CeFi/Lite market-cap mode.
    pub fn is_cefi_cr_in_market_cap(&self) -> bool { todo!() }
    /// Returns a lazily-initialized Settings Page configuration sub-object.
    /// 
    /// The returned instance shares the same `Inner` state as this configuration,
    /// exposing only Settings Page property getters/setters.
    pub fn page_config(&self) -> Arc<ChartSettingsPageConfig> { todo!() }
    /// Returns a lazily-initialized Settings Panel configuration sub-object.
    /// 
    /// The returned instance shares the same `Inner` state as this configuration,
    /// exposing only Settings Panel property getters/setters.
    pub fn panel_config(&self) -> Arc<ChartSettingsPanelConfig> { todo!() }
    /// Returns the `SettingCase` this configuration instance is scoped to.
    pub fn setting_case(&self) -> ChartSettingCase { todo!() }
    /// Returns the `SettingCaseLayoutType` this configuration instance is scoped to.
    pub fn setting_case_layout_type(&self) -> ChartSettingCaseLayoutType { todo!() }
    /// Returns a case-scoped `SettingsStoreService` for persisted settings reads/writes.
    /// 
    /// This avoids exposing raw keys to native and ensures all persistence is scoped
    /// to the same `SettingCase` as this configuration.
    pub fn settings_store(&self) -> Arc<ChartSettingsStoreService> { todo!() }
    /// Returns `true` if the chart is currently in market-cap mode and should handle big numbers.
    pub fn should_deal_big_number(&self) -> bool { todo!() }
    /// Returns a lazily-initialized, cached `ChartTradeFeaturesConfiguration`.
    /// 
    /// The returned instance shares the same `ChartSettingManager` as this configuration,
    /// ensuring all trade-feature reads/writes are scoped to the same `SettingCase`.
    pub fn trade_features_configuration(&self) -> Arc<ChartTradeFeaturesConfiguration> { todo!() }
}

/// UniFFI Object exposing Settings Page property getters/setters.
/// 
/// Shares the same `Inner` state with its parent `ChartSettingsConfiguration`
/// via `Arc<RwLock<Inner>>`.
pub struct ChartSettingsPageConfig;
impl ChartSettingsPageConfig {
    /// Returns the current chart context (same shared storage as the panel
    /// config's `chart_context`).
    pub fn chart_context(&self) -> KLineSettingsFeatureContext { todo!() }
    /// Whether the chart is currently in compare (multi-asset) mode.
    pub fn current_is_compare_mode(&self) -> bool { todo!() }
    /// Whether the current display mode is market-cap (vs price).
    pub fn current_is_market_cap(&self) -> bool { todo!() }
    /// Quick-order availability DERIVED from the pushed chart context —
    /// migrated native `OKKLineChartNewVC.enableQuickOrder`.
    /// 
    /// For `SettingCase::Cefi` with a main-chart layout (`ProPortrait` /
    /// `ProFullScreen` / `ProLandscape`) this evaluates
    /// `feature_gates::enable_quick_order` from:
    /// - the shared SCENE `chart_context` (`is_arbi` / `is_bot` / `biz_type` /
    /// `is_pre_market`) — push via [`Self::set_chart_context`] (or the panel
    /// config's, same storage) BEFORE reading,
    /// - the session-global account `trade_facts` (MCC/PM level, PMC/Trade
    /// acctLv) — pushed by retail-kline, never by native,
    /// - `current_is_market_cap` (native `isCefiOrCRInMarketCap`, maintained
    /// by its own setter),
    /// - the compile-time TR gate.
    /// 
    /// Every other case/layout falls back to the stored flag (Lite → `true`,
    /// TradingView / formula / compare / non-Cefi → their presets), so callers
    /// can use this getter unconditionally after a push.
    /// 
    /// NOTE: the context is per-VC data in a shared singleton (last writer
    /// wins) — read this immediately after pushing your own context and copy
    /// it into your per-VC snapshot; do not consume it lazily.
    pub fn derived_enable_quick_order(&self) -> bool { todo!() }
    /// Whether the cancel-order confirmation dialog is enabled.
    pub fn enable_cancel_order_confirm(&self) -> bool { todo!() }
    /// Whether the chart-settings entry is available.
    pub fn enable_chart_settings(&self) -> bool { todo!() }
    /// Whether the DeFi UTC reminder is enabled.
    pub fn enable_defi_utc_reminder(&self) -> bool { todo!() }
    /// Whether switching between price and market-cap display is enabled.
    pub fn enable_market_cap(&self) -> bool { todo!() }
    /// Whether multi-period change (price change ratio) display is enabled.
    pub fn enable_multi_period_change(&self) -> bool { todo!() }
    /// Whether the quick-order (fast trading) toggle is enabled — the STORED
    /// flag (constructor preset / last `set_enable_quick_order`).
    /// 
    /// Deliberately NOT derived: this getter is snapshotted by every chart
    /// surface through the shared singleton configuration (half-screen, PIP,
    /// event-contract, …), so its value must not depend on whichever chart
    /// pushed `chart_context` last. The main Cefi chart should instead push
    /// its context and read [`Self::derived_enable_quick_order`] into its own
    /// per-VC snapshot (same pattern as the panel's derived compare/date
    /// buttons).
    pub fn enable_quick_order(&self) -> bool { todo!() }
    /// Whether auto-cleaning draw-line data is enabled.
    pub fn enable_set_auto_clean_draw_lines_data(&self) -> bool { todo!() }
    /// Whether auto-coordinate icon "appeared phase" can be configured.
    pub fn enable_set_auto_coordinate_icon(&self) -> bool { todo!() }
    /// Whether candle-bar style (candle/bar/hollow) can be configured.
    pub fn enable_set_candle_bar_style(&self) -> bool { todo!() }
    /// Whether color preference (custom palette) can be configured.
    pub fn enable_set_color_preference(&self) -> bool { todo!() }
    /// Whether the candle-bar "digest type" can be configured.
    pub fn enable_set_digest_type(&self) -> bool { todo!() }
    /// Whether double-tap to switch chart mode can be configured.
    pub fn enable_set_double_tap_to_switch_chart_mode(&self) -> bool { todo!() }
    /// Whether KLine drag position can be configured.
    pub fn enable_set_drag_position(&self) -> bool { todo!() }
    /// Whether the "keep screen on" preference can be configured.
    pub fn enable_set_keep_screen_on(&self) -> bool { todo!() }
    /// Whether the News entry is shown in settings.
    pub fn enable_set_news_entry(&self) -> bool { todo!() }
    /// Whether rise/fall candle color can be configured.
    pub fn enable_set_rise_fall_color(&self) -> bool { todo!() }
    /// Whether the rise/fall period can be edited.
    pub fn enable_set_rise_fall_period(&self) -> bool { todo!() }
    /// Whether X-axis scaling style can be configured.
    pub fn enable_set_scaling_style(&self) -> bool { todo!() }
    /// Whether switching chart mode (fullscreen / normal) is supported.
    pub fn enable_set_switch_chart_mode(&self) -> bool { todo!() }
    /// Whether switching chart view (native / TradingView) is supported.
    pub fn enable_set_switch_chart_view(&self) -> bool { todo!() }
    /// Whether syncing draw-line data from server is enabled.
    pub fn enable_set_sync_draw_lines_data(&self) -> bool { todo!() }
    /// Whether chart theme type (light/dark/system) can be configured.
    pub fn enable_set_theme_type(&self) -> bool { todo!() }
    /// Whether trade-button position (left / right) can be configured.
    pub fn enable_set_trade_btn_position(&self) -> bool { todo!() }
    /// Whether Y-axis mark style (number vs percentage) can be configured.
    pub fn enable_set_y_axis_mark_style(&self) -> bool { todo!() }
    /// Whether "switch pairs keeps time anchor" behavior is supported.
    pub fn enable_switch_pair_time_anchor(&self) -> bool { todo!() }
    /// Whether time-anchor mode is supported on the chart.
    pub fn enable_time_anchor(&self) -> bool { todo!() }
    /// Whether the TP/SL adjustment confirmation toggle is enabled.
    pub fn enable_tpsl_adjust_confirm(&self) -> bool { todo!() }
    /// Whether watching KLine in widgets is supported.
    pub fn enable_watch_kline_in_widgets(&self) -> bool { todo!() }
    /// Whether market-cap mode uses the time-chart style.
    pub fn market_cap_use_time_chart(&self) -> bool { todo!() }
    /// Last price snapshot used by market-cap mode (may be `None`).
    pub fn mcap_last_price(&self) -> Option<String> { todo!() }
    /// Pushes a fresh chart context — SAME shared storage as
    /// `panel_config.set_chart_context` (page and panel share one `Inner`),
    /// exposed here too so quick-order callers don't need the panel object.
    /// Feeds the derived `enable_quick_order` (this config) and the derived
    /// compare/date buttons (panel config).
    /// 
    /// Native should call this once per chart open — before the first
    /// derived read — and again whenever any scene input changes
    /// (instrument switch, isArbi flip). Account facts are NOT part of this
    /// context; retail-kline maintains them in `settings::trade_facts`.
    pub fn set_chart_context(&self, value: KLineSettingsFeatureContext) { todo!() }
    /// Sets whether the chart is currently in compare mode.
    pub fn set_current_is_compare_mode(&self, value: bool) { todo!() }
    /// Sets whether the current display mode is market-cap.
    pub fn set_current_is_market_cap(&self, value: bool) { todo!() }
    /// Sets whether the cancel-order confirmation dialog is enabled.
    pub fn set_enable_cancel_order_confirm(&self, value: bool) { todo!() }
    /// Sets whether the chart-settings entry is available.
    pub fn set_enable_chart_settings(&self, value: bool) { todo!() }
    /// Sets whether the DeFi UTC reminder is enabled.
    pub fn set_enable_defi_utc_reminder(&self, value: bool) { todo!() }
    /// Sets whether switching between price and market-cap display is enabled.
    pub fn set_enable_market_cap(&self, value: bool) { todo!() }
    /// Sets whether multi-period change display is enabled.
    pub fn set_enable_multi_period_change(&self, value: bool) { todo!() }
    /// Sets the stored quick-order flag.
    pub fn set_enable_quick_order(&self, value: bool) { todo!() }
    /// Sets whether auto-cleaning draw-line data is enabled.
    pub fn set_enable_set_auto_clean_draw_lines_data(&self, value: bool) { todo!() }
    /// Sets whether auto-coordinate icon can be configured.
    pub fn set_enable_set_auto_coordinate_icon(&self, value: bool) { todo!() }
    /// Sets whether candle-bar style can be configured.
    pub fn set_enable_set_candle_bar_style(&self, value: bool) { todo!() }
    /// Sets whether color preference can be configured.
    pub fn set_enable_set_color_preference(&self, value: bool) { todo!() }
    /// Sets whether the candle-bar "digest type" can be configured.
    pub fn set_enable_set_digest_type(&self, value: bool) { todo!() }
    /// Sets whether double-tap to switch chart mode can be configured.
    pub fn set_enable_set_double_tap_to_switch_chart_mode(&self, value: bool) { todo!() }
    /// Sets whether KLine drag position can be configured.
    pub fn set_enable_set_drag_position(&self, value: bool) { todo!() }
    /// Sets whether the "keep screen on" preference can be configured.
    pub fn set_enable_set_keep_screen_on(&self, value: bool) { todo!() }
    /// Sets whether the News entry is shown in settings.
    pub fn set_enable_set_news_entry(&self, value: bool) { todo!() }
    /// Sets whether rise/fall candle color can be configured.
    pub fn set_enable_set_rise_fall_color(&self, value: bool) { todo!() }
    /// Sets whether the rise/fall period can be edited.
    pub fn set_enable_set_rise_fall_period(&self, value: bool) { todo!() }
    /// Sets whether X-axis scaling style can be configured.
    pub fn set_enable_set_scaling_style(&self, value: bool) { todo!() }
    /// Sets whether switching chart mode is supported.
    pub fn set_enable_set_switch_chart_mode(&self, value: bool) { todo!() }
    /// Sets whether switching chart view is supported.
    pub fn set_enable_set_switch_chart_view(&self, value: bool) { todo!() }
    /// Sets whether syncing draw-line data from server is enabled.
    pub fn set_enable_set_sync_draw_lines_data(&self, value: bool) { todo!() }
    /// Sets whether chart theme type can be configured.
    pub fn set_enable_set_theme_type(&self, value: bool) { todo!() }
    /// Sets whether trade-button position can be configured.
    pub fn set_enable_set_trade_btn_position(&self, value: bool) { todo!() }
    /// Sets whether Y-axis mark style can be configured.
    pub fn set_enable_set_y_axis_mark_style(&self, value: bool) { todo!() }
    /// Sets whether "switch pairs keeps time anchor" behavior is supported.
    pub fn set_enable_switch_pair_time_anchor(&self, value: bool) { todo!() }
    /// Sets whether time-anchor mode is supported.
    pub fn set_enable_time_anchor(&self, value: bool) { todo!() }
    /// Sets whether the TP/SL adjustment confirmation toggle is enabled.
    pub fn set_enable_tpsl_adjust_confirm(&self, value: bool) { todo!() }
    /// Sets whether watching KLine in widgets is supported.
    pub fn set_enable_watch_kline_in_widgets(&self, value: bool) { todo!() }
    /// Sets whether market-cap mode uses the time-chart style.
    pub fn set_market_cap_use_time_chart(&self, value: bool) { todo!() }
    /// Sets the last price snapshot for market-cap mode.
    pub fn set_mcap_last_price(&self, value: Option<String>) { todo!() }
}

/// UniFFI Object exposing Settings Panel property getters/setters.
/// 
/// Shares the same `Inner` state with its parent `ChartSettingsConfiguration`
/// via `Arc<RwLock<Inner>>`.
pub struct ChartSettingsPanelConfig;
impl ChartSettingsPanelConfig {
    /// Returns the current chart context.
    pub fn chart_context(&self) -> KLineSettingsFeatureContext { todo!() }
    /// Whether the chart is currently in formula mode.
    pub fn current_is_formula(&self) -> bool { todo!() }
    /// `enableAssetCostTags`, gated by the TM-sub-account hidden-cur-pos flag
    /// AND the asset-cost line being a spot-only, non-TR feature (native
    /// `showAsset = bizType == SPOT && !isTR`; folded in because the flag's
    /// only consumer ANDs it with `showAsset` — point-wise equivalent). TR is
    /// the compile-time build gate, as in `feature_gates`.
    pub fn derived_enable_asset_cost_tags(&self) -> bool { todo!() }
    /// `enableBalancePx` — main Cefi chart: `isSwapOrFutures(bizType)` gated
    /// by hidden-cur-pos; other cases/layouts: the stored preset.
    pub fn derived_enable_balance_px(&self) -> bool { todo!() }
    /// Buy/sell point availability DERIVED from the pushed chart context —
    /// migrated native:
    /// `if (tradeSetting.isTmSubAccountNeedHiddenTransaction) { enableBuySellPoint = NO; }`.
    /// 
    /// Simply `stored && !is_tm_sub_account_need_hidden_transaction` — no
    /// case/layout gate needed: the constructor presets already encode the
    /// per-scene base value (formula / compare / landscape → `false`, main
    /// chart portrait/fullscreen → `true`, …), and the TM-sub-account hide is
    /// an account-level compliance gate that applies to every chart. With no
    /// context pushed (default `false`) this equals the stored flag.
    /// 
    /// Push the context via `set_chart_context` (page or panel entry) BEFORE
    /// reading, and copy the result into the per-VC snapshot (shared-singleton
    /// caveat as `derived_enable_quick_order`).
    pub fn derived_enable_buy_sell_point(&self) -> bool { todo!() }
    /// Guide availability DERIVED from the session-global trade facts —
    /// migrated native `enableGuide = [trade isSupportAllBizTypes]`
    /// (native comment: show the newcomer guide only when all five business
    /// lines are supported).
    /// 
    /// `stored && trade_facts.is_support_all_biz_types`: the stored flag
    /// carries the scene presets (fullscreen / landscape / formula / compare /
    /// Lite → `false`) and the non-Cefi clamp; the facts field carries the
    /// core-trading entity capability, pushed by retail-kline. With
    /// no facts pushed the default is `false` — guide hidden, mirroring
    /// native's `trade?.isSupportAllBizTypes() ?? false`.
    pub fn derived_enable_guide(&self) -> bool { todo!() }
    /// `enableLiquidationPx` — main Cefi chart: margin, swap, or futures,
    /// gated by hidden-cur-pos; other cases/layouts: the stored preset.
    pub fn derived_enable_liquidation_px(&self) -> bool { todo!() }
    /// `enableOrderTags`, gated by the TM-sub-account hidden-cur-pos flag.
    pub fn derived_enable_order_tags(&self) -> bool { todo!() }
    /// `enablePositionTags`, gated by the TM-sub-account hidden-cur-pos flag
    /// AND the instrument having a position concept at all (native
    /// `showPosition = bizType != SPOT`; folded in here because the flag's
    /// only consumer ANDs it with `showPosition` anyway — spot has no
    /// positions, so the capability is off, point-wise equivalent).
    pub fn derived_enable_position_tags(&self) -> bool { todo!() }
    /// Spot cost-price basis from the session-global trade facts — migrated
    /// native `isAverageCost = (tradeSetting.spotAverageConfig ==
    /// OKTTradeSettingAvgCostPriceType)` → `configuration.isAverageAssetCost`.
    /// 
    /// Pure fact pass-through (no scene preset, no gate): `true` = average
    /// (open) cost, `false` = diluted cost. Defaults to `true` before any
    /// facts push, matching native's not-yet-loaded trade config.
    /// NOTE: the formula chart hardcodes `isAverageAssetCost = NO` natively —
    /// that override must stay; do not consume this getter there.
    pub fn derived_is_average_asset_cost(&self) -> bool { todo!() }
    /// Whether the KLine view height can be adjusted by the user.
    pub fn enable_adjust_height(&self) -> bool { todo!() }
    /// Whether asset-cost tags overlay is enabled.
    pub fn enable_asset_cost_tags(&self) -> bool { todo!() }
    /// Whether break-even (balance) price line is shown.
    pub fn enable_balance_px(&self) -> bool { todo!() }
    /// Whether buy/sell history-point markers are shown — the STORED flag
    /// (constructor preset / last `set_enable_buy_sell_point`). Snapshot
    /// surfaces keep reading this; see [`Self::derived_enable_buy_sell_point`]
    /// for the TM-sub-account-aware value.
    pub fn enable_buy_sell_point(&self) -> bool { todo!() }
    /// Whether the Compare button is shown in the panel.
    /// 
    /// Derived from: chart context (`is_arbi` / `is_bot` / `biz_type` /
    /// `is_pre_market`), the configuration's `setting_case` +
    /// `setting_case_layout_type`, and `FeatureRestrictionService`
    /// (TR build / Lite cefi mode).
    /// 
    /// Scene exclusions on top of the native `enableCompareKLine` checks:
    /// compare (`ProCompare`) / formula (`ProFormula`) layouts and the bot
    /// grid chart (`chart_context.is_bot` — it runs as Cefi + main layout on
    /// the shared singleton) never show the Compare entry.
    pub fn enable_compare_btn(&self) -> bool { todo!() }
    /// Whether countdown display is enabled on the chart.
    pub fn enable_count_down(&self) -> bool { todo!() }
    /// Whether the Date button is shown in the panel.
    /// 
    /// Derived from chart context (`is_arbi` / `is_bot` / `biz_type` /
    /// `is_native_chart`) and `setting_case` (Cefi/Lite only). Independent of
    /// the TR/Lite app-mode gate — native logic doesn't gate the date entry on
    /// `isFeatureHidden*`. Compare (`ProCompare`) / formula (`ProFormula`)
    /// layouts and the bot grid chart never show the Date entry.
    pub fn enable_date_btn(&self) -> bool { todo!() }
    /// Whether the drawings button is shown in the panel.
    pub fn enable_drawings_btn(&self) -> bool { todo!() }
    /// Whether the Formula button is shown in the panel.
    /// 
    /// Derived from `setting_case` (Cefi/Lite only) + `FeatureRestrictionService`
    /// (not TR build, not Lite cefi mode) — mirrors native: default `true`,
    /// forced off for TR builds and Lite app mode. The compare chart
    /// (`ProCompare`) and the bot grid chart (`chart_context.is_bot`) never
    /// show the Formula entry either.
    pub fn enable_formula_btn(&self) -> bool { todo!() }
    /// Whether the guide (tutorial) overlay is enabled.
    /// 
    /// Setting this to `true` is only effective for `SettingCase::Cefi`;
    /// all other cases will be clamped to `false`.
    pub fn enable_guide(&self) -> bool { todo!() }
    /// Whether the indicators button is shown in the panel.
    pub fn enable_indicators_btn(&self) -> bool { todo!() }
    /// Whether the landscape button is shown in the panel.
    pub fn enable_landscape_btn(&self) -> bool { todo!() }
    /// Whether liquidation-price line is shown.
    pub fn enable_liquidation_px(&self) -> bool { todo!() }
    /// Whether the normal (KLine) guide overlay is enabled.
    pub fn enable_normal_guide(&self) -> bool { todo!() }
    /// Whether order tags overlay is enabled.
    pub fn enable_order_tags(&self) -> bool { todo!() }
    /// Whether past economic calendar events can be shown on the chart.
    pub fn enable_past_events(&self) -> bool { todo!() }
    /// Whether position tags overlay is enabled.
    pub fn enable_position_tags(&self) -> bool { todo!() }
    /// Whether price-alert entry is enabled on the chart.
    pub fn enable_price_alert(&self) -> bool { todo!() }
    /// Whether the settings button is shown in the panel.
    pub fn enable_settings_btn(&self) -> bool { todo!() }
    /// Whether the trade (order/position) guide overlay is enabled.
    pub fn enable_trade_guide(&self) -> bool { todo!() }
    /// Whether the trade-settings entry is shown.
    pub fn enable_trade_settings(&self) -> bool { todo!() }
    /// Whether upcoming economic calendar events can be shown on the chart.
    pub fn enable_upcoming_events(&self) -> bool { todo!() }
    /// Whether Y-axis mark display is enabled.
    pub fn enable_y_axis_mark(&self) -> bool { todo!() }
    /// Returns the formula title text (empty string if not in formula mode).
    pub fn formula_title(&self) -> String { todo!() }
    /// Pushes a fresh chart context. Native should call this once per chart
    /// open and again whenever any input changes (instrument switch,
    /// `klineViewType` toggle, `isArbi` flip). Only Cefi / Lite charts need
    /// to push — for other setting cases the per-button case gate already
    /// returns `false` regardless of context.
    pub fn set_chart_context(&self, value: KLineSettingsFeatureContext) { todo!() }
    /// Sets whether the chart is in formula mode.
    pub fn set_current_is_formula(&self, value: bool) { todo!() }
    /// Sets whether the KLine view height can be adjusted.
    pub fn set_enable_adjust_height(&self, value: bool) { todo!() }
    /// Sets whether asset-cost tags overlay is enabled.
    pub fn set_enable_asset_cost_tags(&self, value: bool) { todo!() }
    /// Sets whether break-even price line is shown.
    pub fn set_enable_balance_px(&self, value: bool) { todo!() }
    /// Sets whether buy/sell point markers are shown.
    pub fn set_enable_buy_sell_point(&self, value: bool) { todo!() }
    /// Sets whether countdown display is enabled.
    pub fn set_enable_count_down(&self, value: bool) { todo!() }
    /// Sets whether the drawings button is shown.
    pub fn set_enable_drawings_btn(&self, value: bool) { todo!() }
    /// Sets whether the guide overlay is enabled. Clamped by `SettingCase`.
    pub fn set_enable_guide(&self, value: bool) { todo!() }
    /// Sets whether the indicators button is shown.
    pub fn set_enable_indicators_btn(&self, value: bool) { todo!() }
    /// Sets whether the landscape button is shown.
    pub fn set_enable_landscape_btn(&self, value: bool) { todo!() }
    /// Sets whether liquidation-price line is shown.
    pub fn set_enable_liquidation_px(&self, value: bool) { todo!() }
    /// Sets whether the normal guide overlay is enabled.
    pub fn set_enable_normal_guide(&self, value: bool) { todo!() }
    /// Sets whether order tags overlay is enabled.
    pub fn set_enable_order_tags(&self, value: bool) { todo!() }
    /// Sets whether past economic calendar events can be shown.
    pub fn set_enable_past_events(&self, value: bool) { todo!() }
    /// Sets whether position tags overlay is enabled.
    pub fn set_enable_position_tags(&self, value: bool) { todo!() }
    /// Sets whether price-alert entry is enabled.
    pub fn set_enable_price_alert(&self, value: bool) { todo!() }
    /// Sets whether the settings button is shown.
    pub fn set_enable_settings_btn(&self, value: bool) { todo!() }
    /// Sets whether the trade guide overlay is enabled.
    pub fn set_enable_trade_guide(&self, value: bool) { todo!() }
    /// Sets whether the trade-settings entry is shown.
    pub fn set_enable_trade_settings(&self, value: bool) { todo!() }
    /// Sets whether upcoming economic calendar events can be shown.
    pub fn set_enable_upcoming_events(&self, value: bool) { todo!() }
    /// Sets whether Y-axis mark display is enabled.
    pub fn set_enable_y_axis_mark(&self, value: bool) { todo!() }
    /// Sets the formula title text.
    pub fn set_formula_title(&self, value: String) { todo!() }
}

/// Unified native-facing settings service.
/// 
/// Design goals:
/// - Native calls always pass `SettingCase` explicitly.
/// - Rust does **not** perform bulk migration from native caches.
/// - If Rust has no cache, native can fallback and then write back via the returned managers.
pub struct ChartSettingsService;
impl ChartSettingsService {
    /// Returns whether a `SettingObject` exists in the chart-settings object store for `base_key`
    /// under this service's `SettingCase` namespacing.
    pub fn contains_object_key(&self, base_key: String) -> Result<bool, ChartSettingsError> { todo!() }
    /// Returns whether a value exists in the chart-settings preference store for `base_key`
    /// under this service's `SettingCase` namespacing.
    /// 
    /// This is a lightweight presence check (equivalent to `Preference.contains(effective_key)`).
    pub fn contains_preference_key(&self, base_key: String) -> bool { todo!() }
    /// Returns a case-scoped `ChartIndicatorsConfiguration`.
    /// 
    /// The returned configuration is cached as a single instance; native can modify it via
    /// the instance's setter methods.
    pub fn indicators_configuration(&self) -> Arc<ChartIndicatorsConfiguration> { todo!() }
    /// Returns a case-scoped `ChartPeriodsConfiguration`.
    /// 
    /// The returned configuration is cached as a single instance; native can modify it via
    /// the instance's setter methods.
    pub fn periods_configuration(&self) -> Arc<ChartPeriodsConfiguration> { todo!() }
    /// Returns the `SettingCase` this service instance is scoped to.
    pub fn setting_case(&self) -> ChartSettingCase { todo!() }
    /// Returns a case-scoped `ChartSettingsConfiguration`.
    /// 
    /// The returned configuration is cached as a single instance. Native can modify
    /// properties (e.g., `enable_time_anchor`) via setter methods on the returned object.
    pub fn settings_configuration(&self) -> Arc<ChartSettingsConfiguration> { todo!() }
    /// Returns a case-scoped `ChartTradeFeaturesConfiguration`.
    /// 
    /// The returned configuration is cached as a single instance shared with
    /// `settings_configuration().trade_features_configuration()`. Both access paths
    /// return the same `Arc`, so in-memory state is always consistent.
    /// Native can modify capability toggles (e.g. enableOrderTags/enablePositionsTags/enableExpandLine)
    /// via setter methods on the returned object.
    pub fn trade_features_configuration(&self) -> Arc<ChartTradeFeaturesConfiguration> { todo!() }
}

/// Case-scoped settings persistence service.
/// 
/// This is a UniFFI-friendly wrapper around `SettingsStore` that holds a
/// `KlineSettingManager` instance, so native does not need to pass managers
/// or keys around to perform persisted KV reads/writes.
pub struct ChartSettingsStoreService;
impl ChartSettingsStoreService {
    /// Returns the X-perp "Backfill history" toggle (native: `KLineBackfillHistoryKey`).
    pub fn backfill_history(&self, default_value: bool) -> bool { todo!() }
    /// Returns the persisted bot grid lines toggle (native: `KLineBotGidLinesKey`).
    pub fn bot_grid_lines(&self, default_value: bool) -> bool { todo!() }
    /// Returns the persisted cancel order confirmation toggle.
    /// Defaults to `true` (show confirmation dialog) when no value has been persisted.
    pub fn cancel_order_confirmation(&self) -> bool { todo!() }
    /// Returns the persisted candle color info for bullish or bearish.
    /// 
    /// Native stores as `"hexColor,blockName"` under `KLine_CandleColor_bullish` / `_bearish`.
    pub fn candle_color(&self, is_bullish: bool, default_value: String) -> String { todo!() }
    /// Returns the persisted candle width (native: `KLine_CandleWidth`).
    pub fn candle_width(&self, default_value: f64) -> f64 { todo!() }
    /// Returns the persisted chart mode (native: `KLine_ChartMode`).
    pub fn chart_mode(&self, default_value: i64) -> i64 { todo!() }
    /// Reads the chart type setting (K-line vs line chart).
    pub fn chart_type(&self, default_value: i64) -> i64 { todo!() }
    /// Returns whether a value exists in the chart-settings preference store for `base_key`
    /// under this service's `SettingCase` namespacing.
    /// 
    /// Only chart-settings base keys (as defined in `chart_settings_base_keys()`) are accepted;
    /// other keys return `false`.
    pub fn contains_preference_key(&self, base_key: String) -> bool { todo!() }
    /// Returns the persisted count down toggle (native: `KLineCountDownKey`).
    pub fn count_down(&self, default_value: bool) -> bool { todo!() }
    /// Returns the persisted candle style (native: `KLine_CandleStyle`).
    pub fn current_candle_type(&self, default_value: i64) -> i64 { todo!() }
    /// Returns the persisted digest type (native: `OKKLineToolbarDataCenterDigestTypeKey`).
    pub fn current_digest_type(&self, default_value: i64) -> i64 { todo!() }
    /// Returns the persisted drag type (native: `OKKLineToolbarDataCenterDragTypeKey`).
    pub fn current_drag_type(&self, default_value: i64) -> i64 { todo!() }
    /// Returns the persisted zoom/scaling type (native: `OKKLineToolbarDataCenterKLineZoomTypeKey`).
    pub fn current_kline_zoom_type(&self, default_value: i64) -> i64 { todo!() }
    /// Returns the persisted double-tap fullscreen toggle (native: `KLineChartDoubleTapScreenKey`).
    pub fn double_tap_fullscreen(&self, default_value: bool) -> bool { todo!() }
    /// Returns the persisted draw line auto clear toggle (native: `KLineDrawLineAutoClearKey`).
    pub fn draw_line_auto_clear(&self, default_value: bool) -> bool { todo!() }
    /// Returns the draw-line magnet mode (native: `KLineDrawLineMagnetModeKey`).
    /// 
    /// 0 = off, 1 = weak, 2 = strong; the key is CeDeFi-shared (no namespacing).
    pub fn draw_line_magnet_mode(&self, default_value: i64) -> i64 { todo!() }
    /// Returns the persisted draw line sync toggle (native: `KLineDrawLineSyncKey`).
    pub fn draw_line_sync(&self, default_value: bool) -> bool { todo!() }
    /// Returns the persisted earnings banner toggle (native: `KLineEarningsBannerKey`).
    pub fn earnings_banner(&self, default_value: bool) -> bool { todo!() }
    /// Returns the persisted earnings-calendar toggle (native:
    /// `KLineChartEarningsCalendarKey`). Independent of the economic keys.
    pub fn earnings_calendar(&self, default_value: bool) -> bool { todo!() }
    /// Returns the "show published earnings" sub-toggle (native:
    /// `KLineChartEarningsPublishedKey`).
    pub fn earnings_published(&self, default_value: bool) -> bool { todo!() }
    /// Returns the "show upcoming earnings" sub-toggle (native:
    /// `KLineChartEarningsUpcomingKey`). Only meaningful while the master
    /// earnings-calendar toggle is on.
    pub fn earnings_upcoming(&self, default_value: bool) -> bool { todo!() }
    /// Returns the persisted economic past toggle (native: `KLineChartEconomicPastKey`).
    pub fn economic_past(&self, default_value: bool) -> bool { todo!() }
    /// Returns the persisted economic upcoming toggle (native: `KLineChartEconomicUpcomingKey`).
    pub fn economic_upcoming(&self, default_value: bool) -> bool { todo!() }
    /// Returns the persisted graduate/grid setting (native: `KLine_Graduate`).
    pub fn graduate(&self, default_value: i64) -> i64 { todo!() }
    /// Returns whether native settings cache migration has been completed for this `SettingCase`.
    /// This checks if ALL keys (both Page and Panel) have been migrated.
    pub fn has_migrated_native_cache(&self) -> bool { todo!() }
    /// Returns whether native settings cache migration has been completed for the specified group.
    pub fn has_migrated_native_cache_for_group(&self, group: SettingsKeyGroup) -> bool { todo!() }
    /// Returns the persisted history order toggle (native: `KLineHistoryOrderKey`).
    pub fn history_order(&self, default_value: bool) -> bool { todo!() }
    /// Returns the persisted keep-on toggle (native: `KLineChartKeepOnKey`).
    pub fn keep_on(&self, default_value: bool) -> bool { todo!() }
    /// Reads the K-line render style (0 = Candlestick, 1 = OHLC Bar). Dedicated key, separate from
    /// the Lite K-line-vs-line `chart_type`.
    pub fn kline_style(&self, default_value: i64) -> i64 { todo!() }
    /// Migrates native settings cache to Rust storage.
    /// 
    /// - Supports migrating multiple keys in one call.
    /// - Skips writes for keys that already exist in Rust storage.
    /// 
    /// Keys are **base keys** and will be namespaced by Rust according to this service's `SettingCase`.
    /// 
    /// # Returns
    /// - `true` if migration flag was newly set by this call
    /// - `false` if migration had already been performed before
    pub fn migrate_native_cache(&self, payload: SettingsCacheMigrationPayload) -> bool { todo!() }
    /// Migrates native platform cached settings to Rust storage for a specific key group.
    /// 
    /// This allows selective migration of Settings Page keys vs Settings Panel keys,
    /// enabling native code to migrate keys at different times (e.g., Page keys at app
    /// launch, Panel keys when chart is opened).
    /// 
    /// # Parameters
    /// - `payload`: Native settings values to migrate
    /// - `group`: Which group of keys to migrate (`Page`, `Panel`, or `All`)
    /// 
    /// # Returns
    /// - `true` if migration flag was newly set by this call
    /// - `false` if migration had already been performed before
    pub fn migrate_native_cache_for_group(&self, payload: SettingsCacheMigrationPayload, group: SettingsKeyGroup) -> bool { todo!() }
    /// Marks native settings cache migration as completed for this `SettingCase`.
    /// 
    /// Call this after native cache migration is done to prevent re-migration.
    /// This marks ALL keys (both Page and Panel) as migrated.
    pub fn migrated_native_cache_done(&self) { todo!() }
    /// Returns the persisted native height (native: `OKKLineNativeHeightKey`).
    pub fn native_height(&self, default_value: f64) -> f64 { todo!() }
    /// Returns the persisted news entry toggle (native: `KLineNewsEntryKey`).
    pub fn news_entry(&self, default_value: bool) -> bool { todo!() }
    /// Returns the persisted order-book quick-order toggle.
    /// This is independent from the chart "+" quick-order toggle.
    pub fn order_book_quick_order(&self, default_value: bool) -> bool { todo!() }
    /// Returns the persisted orders overlay toggle (native: `KLineOrdersKey`).
    pub fn orders(&self, default_value: bool) -> bool { todo!() }
    /// Returns the persisted position cost toggle (native: `KLinePositionCostKey`).
    pub fn position_cost(&self, default_value: bool) -> bool { todo!() }
    /// Returns the persisted price alert toggle (native: `KLinePriceAlertKey`).
    pub fn price_alert(&self, default_value: bool) -> bool { todo!() }
    /// Returns the persisted quick order toggle (native: `KLineChartQuickOrderKey`).
    pub fn quick_order(&self, default_value: bool) -> bool { todo!() }
    /// Persists the candle style (native: `KLine_CandleStyle`).
    pub fn save_current_candle_type(&self, value: i64) { todo!() }
    /// Persists the digest type (native: `OKKLineToolbarDataCenterDigestTypeKey`).
    pub fn save_current_digest_type(&self, value: i64) { todo!() }
    /// Persists the drag type (native: `OKKLineToolbarDataCenterDragTypeKey`).
    pub fn save_current_drag_type(&self, value: i64) { todo!() }
    /// Persists the zoom/scaling type (native: `OKKLineToolbarDataCenterKLineZoomTypeKey`).
    pub fn save_current_kline_zoom_type(&self, value: i64) { todo!() }
    /// Persists the X-perp "Backfill history" toggle (native: `KLineBackfillHistoryKey`).
    pub fn set_backfill_history(&self, value: bool) { todo!() }
    /// Persists the bot grid lines toggle (native: `KLineBotGidLinesKey`).
    pub fn set_bot_grid_lines(&self, value: bool) { todo!() }
    /// Persists the cancel order confirmation toggle.
    pub fn set_cancel_order_confirmation(&self, value: bool) { todo!() }
    /// Persists the candle color info for bullish or bearish.
    pub fn set_candle_color(&self, is_bullish: bool, value: String) { todo!() }
    /// Persists the candle width (native: `KLine_CandleWidth`).
    pub fn set_candle_width(&self, value: f64) { todo!() }
    /// Persists the chart mode (native: `KLine_ChartMode`).
    pub fn set_chart_mode(&self, value: i64) { todo!() }
    /// Persists the chart type setting.
    pub fn set_chart_type(&self, value: i64) { todo!() }
    /// Persists the count down toggle (native: `KLineCountDownKey`).
    pub fn set_count_down(&self, value: bool) { todo!() }
    /// Persists the double-tap fullscreen toggle (native: `KLineChartDoubleTapScreenKey`).
    pub fn set_double_tap_fullscreen(&self, value: bool) { todo!() }
    /// Persists the draw line auto clear toggle (native: `KLineDrawLineAutoClearKey`).
    pub fn set_draw_line_auto_clear(&self, value: bool) { todo!() }
    /// Persists the draw-line magnet mode (native: `KLineDrawLineMagnetModeKey`).
    pub fn set_draw_line_magnet_mode(&self, value: i64) { todo!() }
    /// Persists the draw line sync toggle (native: `KLineDrawLineSyncKey`).
    pub fn set_draw_line_sync(&self, value: bool) { todo!() }
    /// Persists the earnings banner toggle (native: `KLineEarningsBannerKey`).
    pub fn set_earnings_banner(&self, value: bool) { todo!() }
    /// Persists the earnings-calendar toggle (native: `KLineChartEarningsCalendarKey`).
    pub fn set_earnings_calendar(&self, value: bool) { todo!() }
    /// Persists the "show published earnings" sub-toggle (native:
    /// `KLineChartEarningsPublishedKey`).
    pub fn set_earnings_published(&self, value: bool) { todo!() }
    /// Persists the "show upcoming earnings" sub-toggle (native:
    /// `KLineChartEarningsUpcomingKey`).
    pub fn set_earnings_upcoming(&self, value: bool) { todo!() }
    /// Persists the economic past toggle (native: `KLineChartEconomicPastKey`).
    pub fn set_economic_past(&self, value: bool) { todo!() }
    /// Persists the economic upcoming toggle (native: `KLineChartEconomicUpcomingKey`).
    pub fn set_economic_upcoming(&self, value: bool) { todo!() }
    /// Persists the graduate/grid setting (native: `KLine_Graduate`).
    pub fn set_graduate(&self, value: i64) { todo!() }
    /// Persists the history order toggle (native: `KLineHistoryOrderKey`).
    pub fn set_history_order(&self, value: bool) { todo!() }
    /// Persists the keep-on toggle (native: `KLineChartKeepOnKey`).
    pub fn set_keep_on(&self, value: bool) { todo!() }
    /// Persists the K-line render style (0 = Candlestick, 1 = OHLC Bar).
    pub fn set_kline_style(&self, value: i64) { todo!() }
    /// Persists the native height (native: `OKKLineNativeHeightKey`).
    pub fn set_native_height(&self, value: f64) { todo!() }
    /// Persists the news entry toggle (native: `KLineNewsEntryKey`).
    pub fn set_news_entry(&self, value: bool) { todo!() }
    /// Persists the order-book quick-order toggle.
    pub fn set_order_book_quick_order(&self, value: bool) { todo!() }
    /// Persists the orders overlay toggle (native: `KLineOrdersKey`).
    pub fn set_orders(&self, value: bool) { todo!() }
    /// Persists the position cost toggle (native: `KLinePositionCostKey`).
    pub fn set_position_cost(&self, value: bool) { todo!() }
    /// Persists the price alert toggle (native: `KLinePriceAlertKey`).
    pub fn set_price_alert(&self, value: bool) { todo!() }
    /// Persists the quick order toggle (native: `KLineChartQuickOrderKey`).
    pub fn set_quick_order(&self, value: bool) { todo!() }
    /// Persists the show asset cost toggle (native: `KLineChartShowAssetCostKey`).
    pub fn set_show_asset_cost(&self, value: bool) { todo!() }
    /// Persists the multi-period change toggle.
    pub fn set_show_multi_period_change(&self, value: bool) { todo!() }
    /// Persists the show price diff toggle (native: `KLineChartShowPriceDiffKey`).
    pub fn set_show_price_diff(&self, value: bool) { todo!() }
    /// Persists the stock price adjustment mode (native: `KLineStockPriceAdjustmentKey`).
    pub fn set_stock_price_adjustment(&self, value: i64) { todo!() }
    /// Persists the switch pair time anchor toggle (native: `KLineSwitchPairsTimeAnchorKey`).
    pub fn set_switch_pair_time_anchor(&self, value: bool) { todo!() }
    /// Persists the theme type (native: `KLine_DarkTheme`).
    /// Value: 0 = light, 1 = dark, 2 = follow system.
    pub fn set_theme_type(&self, value: i64) { todo!() }
    /// Persists the time anchor toggle (native: `KLineTimeAnchorKey`).
    pub fn set_time_anchor(&self, value: bool) { todo!() }
    /// Persists the toolbar coordinate type (native: `OKKLineToolbarDataCenterCoordinateTypeKey`).
    pub fn set_toolbar_coordinate_type(&self, value: i64) { todo!() }
    /// Persists the TP/SL adjustment confirmation toggle.
    pub fn set_tpsl_adjustment_confirmation(&self, value: bool) { todo!() }
    /// Persists the trade button position (native: `OKKLineTradeButtonLeft`).
    pub fn set_trade_button_left(&self, value: bool) { todo!() }
    /// Persists the trade button mode (native: `OKKLineTradeButtonMode`).
    pub fn set_trade_button_mode(&self, value: i64) { todo!() }
    /// Persists the TradingView status (native: `OKKLineTradingViewStatusCacheKey`).
    pub fn set_trading_view_status(&self, value: i64) { todo!() }
    /// Returns the persisted show asset cost toggle (native: `KLineChartShowAssetCostKey`).
    pub fn show_asset_cost(&self, default_value: bool) -> bool { todo!() }
    /// Reads the multi-period change toggle.
    pub fn show_multi_period_change(&self, default_value: bool) -> bool { todo!() }
    /// Returns the persisted show price diff toggle (native: `KLineChartShowPriceDiffKey`).
    pub fn show_price_diff(&self, default_value: bool) -> bool { todo!() }
    /// Returns the persisted stock price adjustment mode (native: `KLineStockPriceAdjustmentKey`).
    /// Value: 0 = None (default), 1 = Forward adjusted.
    pub fn stock_price_adjustment(&self, default_value: i64) -> i64 { todo!() }
    /// Returns the persisted switch pair time anchor toggle (native: `KLineSwitchPairsTimeAnchorKey`).
    pub fn switch_pair_time_anchor(&self, default_value: bool) -> bool { todo!() }
    /// Returns the persisted theme type (native: `KLine_DarkTheme`).
    /// Value: 0 = light, 1 = dark, 2 = follow system.
    pub fn theme_type(&self, default_value: i64) -> i64 { todo!() }
    /// Returns the persisted time anchor toggle (native: `KLineTimeAnchorKey`).
    pub fn time_anchor(&self, default_value: bool) -> bool { todo!() }
    /// Returns the persisted toolbar coordinate type (native: `OKKLineToolbarDataCenterCoordinateTypeKey`).
    pub fn toolbar_coordinate_type(&self, default_value: i64) -> i64 { todo!() }
    /// Returns the persisted TP/SL adjustment confirmation toggle.
    /// Defaults to `true` (show confirmation page) when no value has been persisted.
    pub fn tpsl_adjustment_confirmation(&self) -> bool { todo!() }
    /// Returns the trade button position (native: `OKKLineTradeButtonLeft`).
    /// 
    /// `true` = left side, `false` = right side.
    pub fn trade_button_left(&self, default_value: bool) -> bool { todo!() }
    /// Returns the trade button mode (native: `OKKLineTradeButtonMode`).
    /// 
    /// 0 = left, 1 = right, 2 = quick trade.
    pub fn trade_button_mode(&self, default_value: i64) -> i64 { todo!() }
    /// Returns the TradingView status (native: `OKKLineTradingViewStatusCacheKey`).
    /// 
    /// Value interpretation: -1 = not set, 0 = native chart, 1 = TradingView.
    pub fn trading_view_status(&self, default_value: i64) -> i64 { todo!() }
}

/// Rust equivalent of `OKKLineTradeFeaturesConfiguration`.
pub struct ChartTradeFeaturesConfiguration;
impl ChartTradeFeaturesConfiguration {
    /// Returns the current order tag location (left or right).
    pub fn current_order_location(&self) -> TradeTagsPosition { todo!() }
    /// Returns the current position tag location (left or right).
    pub fn current_position_location(&self) -> TradeTagsPosition { todo!() }
    /// Returns the current TP/SL profit display style (rate or amount).
    pub fn current_profit_style(&self) -> TpslProfitStyle { todo!() }
    /// Returns whether the order extended line is enabled.
    pub fn enable_expand_order_line(&self) -> bool { todo!() }
    /// Returns whether the position extended line is enabled.
    pub fn enable_expand_position_line(&self) -> bool { todo!() }
    /// Sets the order tag location and persists the value.
    pub fn set_current_order_location(&self, value: TradeTagsPosition) { todo!() }
    /// Sets the position tag location and persists the value.
    pub fn set_current_position_location(&self, value: TradeTagsPosition) { todo!() }
    /// Sets the TP/SL profit display style and persists the value.
    pub fn set_current_profit_style(&self, value: TpslProfitStyle) { todo!() }
    /// Sets whether the order extended line is enabled and persists the value.
    pub fn set_enable_expand_order_line(&self, value: bool) { todo!() }
    /// Sets whether the position extended line is enabled and persists the value.
    pub fn set_enable_expand_position_line(&self, value: bool) { todo!() }
    /// Returns the underlying trade settings service.
    pub fn trade_settings(&self) -> Arc<ChartTradeSettingsService> { todo!() }
    /// Returns a case-scoped trade settings persistence service.
    /// 
    /// This is backed by the same underlying settings namespace as this configuration instance.
    pub fn trade_settings_store(&self) -> Arc<ChartTradeSettingsStoreService> { todo!() }
}

/// Rust equivalent of `OKKLineTradeSettings`.
/// 
/// Holds 9 properties matching the native implementation:
/// 1. `enable_revert_position` - Enable reverse position opening
/// 2. `enable_expand_order_line` - Order extended line
/// 3. `enable_expand_position_line` - Position extended line
/// 4. `order_line_position` - Order extended line position
/// 5. `position_line_position` - Position extended line position
/// 6. `position_tpsl_profit_style` - Position TP/SL profit display style
/// 7. `show_liquidation_px` - Show estimated liquidation price
/// 8. `show_balance_px` - Show break-even price
/// 9. `enable_display_all_positions` - Enable display all positions
pub struct ChartTradeSettings;
impl ChartTradeSettings {
}

/// Case-scoped trade settings service.
/// 
/// This is the UniFFI-exported wrapper around `ChartTradeSettings`, providing
/// native platforms with typed access to all 9 trade settings properties.
pub struct ChartTradeSettingsService;
impl ChartTradeSettingsService {
    /// Applies multiple settings changes in a single batch operation.
    /// 
    /// This method updates all specified fields and persists all changes to storage.
    /// Use this when updating multiple settings at once to reduce lock contention
    /// and I/O operations.
    pub fn apply_batch(&self, batch: TradeSettingsBatch) { todo!() }
    /// Whether displaying all positions is enabled.
    pub fn enable_display_all_positions(&self) -> bool { todo!() }
    /// Whether the order extended line is enabled.
    pub fn enable_expand_order_line(&self) -> bool { todo!() }
    /// Whether the position extended line is enabled.
    pub fn enable_expand_position_line(&self) -> bool { todo!() }
    /// Whether "revert position" is enabled.
    pub fn enable_revert_position(&self) -> bool { todo!() }
    /// Returns the order extended line position (0 = Left, 1 = Right).
    pub fn order_line_position(&self) -> i64 { todo!() }
    /// Returns the position extended line position (0 = Left, 1 = Right).
    pub fn position_line_position(&self) -> i64 { todo!() }
    /// Returns the position TP/SL profit display style (0 = Rate, 1 = Amount).
    pub fn position_tpsl_profit_style(&self) -> i64 { todo!() }
    /// Reads all 9 trade settings in a single call — the read counterpart of
    /// `apply_batch`. Lets native replace 9 per-getter FFI reads with one.
    pub fn read_batch(&self) -> TradeSettingsSnapshot { todo!() }
    /// Sets whether displaying all positions is enabled.
    pub fn set_enable_display_all_positions(&self, value: bool) { todo!() }
    /// Sets whether the order extended line is enabled.
    pub fn set_enable_expand_order_line(&self, value: bool) { todo!() }
    /// Sets whether the position extended line is enabled.
    pub fn set_enable_expand_position_line(&self, value: bool) { todo!() }
    /// Persists the "revert position" toggle.
    pub fn set_enable_revert_position(&self, value: bool) { todo!() }
    /// Sets the order extended line position.
    pub fn set_order_line_position(&self, value: i64) { todo!() }
    /// Sets the position extended line position.
    pub fn set_position_line_position(&self, value: i64) { todo!() }
    /// Sets the position TP/SL profit display style.
    pub fn set_position_tpsl_profit_style(&self, value: i64) { todo!() }
    /// Sets whether the break-even price is shown.
    pub fn set_show_balance_px(&self, value: bool) { todo!() }
    /// Sets whether the estimated liquidation price is shown.
    pub fn set_show_liquidation_px(&self, value: bool) { todo!() }
    /// Whether the break-even price is shown.
    pub fn show_balance_px(&self) -> bool { todo!() }
    /// Whether the estimated liquidation price is shown.
    pub fn show_liquidation_px(&self) -> bool { todo!() }
}

/// Case-scoped trade settings persistence service.
/// 
/// Native platforms should prefer this typed API over raw key-value access.
pub struct ChartTradeSettingsStoreService;
impl ChartTradeSettingsStoreService {
    /// Returns whether a `SettingObject` exists in the chart-settings object store for `base_key`
    /// under this service's `SettingCase` namespacing.
    /// 
    /// `base_key` is intended to be **one of the trade setting keys** defined in
    /// `trade_store_keys`:
    /// - `KEY_TRADE_ENABLE_REVERT_POSITION`
    /// - `KEY_TRADE_ENABLE_DISPLAY_ALL_POSITIONS`
    /// - `KEY_TRADE_DISPLAY_LIQUIDATION_PX`
    /// - `KEY_TRADE_DISPLAY_BALANCE_PX`
    /// - `KEY_TRADE_EXPAND_ORDER_LINE`
    /// - `KEY_TRADE_EXPAND_POSITION_LINE`
    /// - `KEY_TRADE_EXPAND_ORDER_LINE_POSITION`
    /// - `KEY_TRADE_EXPAND_POSITION_LINE_POSITION`
    /// - `KEY_TRADE_POSITION_TPSL_PROFIT_STYLE`
    pub fn contains_object_key(&self, base_key: String) -> Result<bool, ChartSettingsError> { todo!() }
    /// Returns whether a value exists in the chart-settings preference store for `base_key`
    /// under this service's `SettingCase` namespacing.
    /// 
    /// Only trade-related base keys are accepted; other keys return `false`.
    pub fn contains_preference_key(&self, base_key: String) -> bool { todo!() }
    pub fn enable_display_all_positions(&self) -> bool { todo!() }
    pub fn enable_expand_order_line(&self) -> bool { todo!() }
    pub fn enable_expand_position_line(&self) -> bool { todo!() }
    pub fn enable_revert_position(&self) -> bool { todo!() }
    /// Returns whether native trade settings cache migration has been completed for this `SettingCase`.
    pub fn has_migrated_native_cache(&self) -> bool { todo!() }
    /// Migrates native-cached trade settings into Rust-managed storage (batch payload).
    /// 
    /// - Supports migrating multiple keys in one call.
    /// - Skips writes for keys that already exist in Rust KV storage.
    /// 
    /// Keys are **base keys** and will be namespaced by Rust according to this service's `SettingCase`.
    /// 
    /// # Returns
    /// - `true` if migration flag was newly set by this call
    /// - `false` if migration had already been performed before
    pub fn migrate_native_cache(&self, payload: TradeCacheMigrationPayload) -> bool { todo!() }
    /// Marks native trade settings cache migration as completed for this `SettingCase`.
    /// 
    /// Call this after native cache migration is done to prevent re-migration.
    pub fn migrated_native_cache_done(&self) { todo!() }
    pub fn order_line_position(&self) -> i64 { todo!() }
    pub fn position_line_position(&self) -> i64 { todo!() }
    pub fn position_tpsl_profit_style(&self) -> i64 { todo!() }
    pub fn set_enable_display_all_positions(&self, value: bool) { todo!() }
    pub fn set_enable_expand_order_line(&self, value: bool) { todo!() }
    pub fn set_enable_expand_position_line(&self, value: bool) { todo!() }
    pub fn set_enable_revert_position(&self, value: bool) { todo!() }
    pub fn set_order_line_position(&self, value: i64) { todo!() }
    pub fn set_position_line_position(&self, value: i64) { todo!() }
    pub fn set_position_tpsl_profit_style(&self, value: i64) { todo!() }
    pub fn set_show_balance_px(&self, value: bool) { todo!() }
    pub fn set_show_liquidation_px(&self, value: bool) { todo!() }
    pub fn show_balance_px(&self) -> bool { todo!() }
    pub fn show_liquidation_px(&self) -> bool { todo!() }
}

/// Drawing Line Service
/// 
/// A unified service that provides all drawing line functionality for native platforms.
/// This includes data storage, settings management, type information, and utility functions.
/// 
/// # Unique ID
/// - **CeFi**: Use `inst_id` directly (e.g., "BTC-USDT")
/// - **CeDeFi**: Use `"{chain_id}_{token_address}"` format (e.g., "1_0xabc123...")
/// 
/// # Example (Swift)
/// ```swift
/// let service = DrawingLineService(settingCase: .cefi, uid: "user_123", maxLines: 50)
/// 
/// // CeFi: Use inst_id as unique_id
/// let lines = service.loadDrawingLines(uniqueId: "BTC-USDT")
/// 
/// // CeDeFi: Use "{chain_id}_{token_address}" as unique_id
/// let defiLines = service.loadDrawingLines(uniqueId: "1_0xabc...")
/// 
/// // Data operations
/// service.saveDrawingLines(lines: lines)
/// service.deleteDrawingLines(uniqueId: "BTC-USDT", lineIds: ["line_1", "line_2"])
/// 
/// // Settings operations
/// let settings = service.loadSettings()
/// service.setLineColor(color: "#FF0000FF", colorBlk: nil)
/// 
/// // Type utilities
/// let isFigure = service.isFigureType(lineType: .rectangle)
/// let pointsNeeded = service.getRequiredPoints(lineType: .triangle)
/// ```
pub struct DrawingLineService;
impl DrawingLineService {
    /// Create a new DrawingLineService for the given setting case and user
    /// 
    /// # Parameters
    /// - `setting_case`: The business setting case (Cefi, CeDeFi, etc.)
    /// - `uid`: User ID for data isolation
    /// - `max_lines`: Maximum number of drawing lines per user
    pub fn new(setting_case: ChartSettingCase, uid: String, max_lines: u32) -> Arc<DrawingLineService> { todo!() }
    /// Run cleanup to remove redundant lines and enforce max lines limit
    pub fn cleanup(&self) -> Result<(), OKKLineDrawingError> { todo!() }
    /// Clear all drawing lines for the current user
    pub fn clear_all_drawing_lines(&self) -> Result<(), OKKLineDrawingError> { todo!() }
    /// Clear all drawing lines for a specific unique ID
    /// 
    /// # Parameters
    /// - `unique_id`: Unique identifier (CeFi: inst_id, CeDeFi: chainId_tokenAddress)
    pub fn clear_drawing_lines(&self, unique_id: String) -> Result<(), OKKLineDrawingError> { todo!() }
    /// Create a new drawing line with minimal required fields
    /// 
    /// # Parameters
    /// - `line_type`: Type of drawing line
    /// - `points`: Coordinate points
    /// - `color`: Line color in hex format (e.g., "#0569FFFF")
    /// - `unique_id`: Unique identifier for the drawing line
    /// - CeFi: Use `inst_id` (e.g., "BTC-USDT")
    /// - CeDeFi: Use `format_cedefi_unique_id(chain_id, token_address)`
    /// 
    /// # Returns
    /// A new DrawingLine instance with generated ID and current timestamp
    pub fn create_drawing_line(&self, line_type: OKKLineDrawingType, points: Vec<OKKLineDrawingPoint>, color: String, unique_id: String) -> OKKLineDrawingData { todo!() }
    /// Create a new drawing point
    /// 
    /// # Parameters
    /// - `timestamp`: Timestamp in milliseconds
    /// - `price`: Price value
    /// 
    /// # Returns
    /// A new DrawingPoint instance
    pub fn create_drawing_point(&self, timestamp: i64, price: f64) -> OKKLineDrawingPoint { todo!() }
    /// Create a new Fibonacci level
    /// 
    /// # Parameters
    /// - `price`: Price level as string (e.g., "0.618")
    /// - `selected`: Whether this level is visible
    /// - `color`: Color in hex format
    /// 
    /// # Returns
    /// A new FibonacciLevel instance
    pub fn create_fibonacci_level(&self, price: String, selected: bool, color: String) -> OKKLineFibonacciLevel { todo!() }
    /// Delete drawing lines by IDs
    /// 
    /// # Parameters
    /// - `unique_id`: Unique identifier (CeFi: inst_id, CeDeFi: chainId_tokenAddress)
    /// - `line_ids`: IDs of drawing lines to delete
    pub fn delete_drawing_lines(&self, unique_id: String, line_ids: Vec<String>) -> Result<(), OKKLineDrawingError> { todo!() }
    /// Generate a unique line ID
    /// 
    /// # Returns
    /// A unique line ID in base36 format (e.g., "mk2dt913")
    pub fn generate_line_id(&self) -> String { todo!() }
    /// Get default Fibonacci levels
    pub fn get_default_fibonacci_levels(&self) -> Vec<FibonacciLevelSetting> { todo!() }
    /// Get default fill color alpha
    pub fn get_default_fill_alpha(&self) -> f64 { todo!() }
    /// Get default line color
    pub fn get_default_line_color(&self) -> String { todo!() }
    /// Get default line width
    pub fn get_default_line_width(&self) -> f64 { todo!() }
    /// Get default drawing line settings
    pub fn get_default_settings(&self) -> DrawingLineSettings { todo!() }
    /// Get Fibonacci levels
    pub fn get_fibonacci_levels(&self) -> Vec<FibonacciLevelSetting> { todo!() }
    /// Get fill color
    pub fn get_fill_color(&self) -> String { todo!() }
    /// Get fill color block
    pub fn get_fill_color_blk(&self) -> Option<String> { todo!() }
    /// Get hide all lines flag
    pub fn get_hide_all_lines(&self) -> bool { todo!() }
    /// Get line color
    pub fn get_line_color(&self) -> String { todo!() }
    /// Get line color block
    pub fn get_line_color_blk(&self) -> Option<String> { todo!() }
    /// Get the total count of drawing lines for the current user
    pub fn get_line_count(&self) -> Result<u32, OKKLineDrawingError> { todo!() }
    /// Get line style (0 = Solid, 1 = Dash)
    pub fn get_line_style(&self) -> u8 { todo!() }
    /// Get line width
    pub fn get_line_width(&self) -> f64 { todo!() }
    /// Get the maximum number of drawing lines per user
    pub fn get_max_lines(&self) -> u32 { todo!() }
    /// Get required number of points for a line type
    pub fn get_required_points(&self, line_type: OKKLineDrawingType) -> u32 { todo!() }
    /// Get the setting case this service is bound to
    pub fn get_setting_case(&self) -> ChartSettingCase { todo!() }
    /// Get the user ID this service is bound to
    pub fn get_uid(&self) -> String { todo!() }
    /// Get drawing lines that need to be synced
    /// 
    /// # Parameters
    /// - `unique_id`: Unique identifier (CeFi: inst_id, CeDeFi: chainId_tokenAddress)
    /// 
    /// # Returns
    /// List of drawing lines with sync = false
    pub fn get_unsynced_lines(&self, unique_id: String) -> Result<Vec<OKKLineDrawingData>, OKKLineDrawingError> { todo!() }
    /// Imports a migration batch in one transaction without replacing rows
    /// that already exist in the Rust store. This makes a retry after process
    /// death safe: a newer Rust edit always wins over stale legacy data.
    pub fn import_drawing_lines_atomically(&self, lines: Vec<OKKLineDrawingData>) -> Result<(), OKKLineDrawingError> { todo!() }
    /// Initialize the drawing line store (creates database if needed)
    /// 
    /// Call this before performing any data operations.
    pub fn initialize(&self) -> Result<(), OKKLineDrawingError> { todo!() }
    /// Check if a line type is a figure (requires fill color)
    pub fn is_figure_type(&self, line_type: OKKLineDrawingType) -> bool { todo!() }
    /// Parse line type from server type string
    pub fn line_type_from_server_type(&self, server_type: String) -> OKKLineDrawingType { todo!() }
    /// Convert line type to server type string
    pub fn line_type_to_server_type(&self, line_type: OKKLineDrawingType) -> String { todo!() }
    /// Load all drawing lines for a specific unique ID.
    /// 
    /// # Parameters
    /// - `unique_id`: Unique identifier for the drawing lines
    /// - CeFi: Use `inst_id` (e.g., "BTC-USDT")
    /// - CeDeFi: Use `format_cedefi_unique_id(chain_id, token_address)`
    /// 
    /// # Returns
    /// List of drawing lines for the unique ID
    pub fn load_drawing_lines(&self, unique_id: String) -> Result<Vec<OKKLineDrawingData>, OKKLineDrawingError> { todo!() }
    /// Load all settings from disk
    pub fn load_settings(&self) -> DrawingLineSettings { todo!() }
    /// Mark drawing lines as deleted (soft delete for sync)
    /// 
    /// # Parameters
    /// - `unique_id`: Unique identifier (CeFi: inst_id, CeDeFi: chainId_tokenAddress)
    /// - `line_ids`: IDs of drawing lines to mark as deleted
    pub fn mark_as_deleted(&self, unique_id: String, line_ids: Vec<String>) -> Result<(), OKKLineDrawingError> { todo!() }
    /// Mark drawing lines as synced
    /// 
    /// # Parameters
    /// - `unique_id`: Unique identifier (CeFi: inst_id, CeDeFi: chainId_tokenAddress)
    /// - `line_ids`: IDs of drawing lines to mark as synced
    pub fn mark_as_synced(&self, unique_id: String, line_ids: Vec<String>) -> Result<(), OKKLineDrawingError> { todo!() }
    /// Reset all settings to default
    pub fn reset_all_settings(&self) -> Result<(), DrawingLineSettingsError> { todo!() }
    /// Reset Fibonacci levels to default
    pub fn reset_fibonacci_levels(&self) -> Result<(), DrawingLineSettingsError> { todo!() }
    /// Save drawing lines (insert or update)
    /// 
    /// # Parameters
    /// - `lines`: Drawing lines to save
    /// 
    /// Note: Each `DrawingLine.inst_id` should be set to the appropriate unique ID:
    /// - CeFi: `inst_id` (e.g., "BTC-USDT")
    /// - CeDeFi: `format_cedefi_unique_id(chain_id, token_address)`
    pub fn save_drawing_lines(&self, lines: Vec<OKKLineDrawingData>) -> Result<(), OKKLineDrawingError> { todo!() }
    /// Save all settings to disk
    pub fn save_settings(&self, settings: DrawingLineSettings) -> Result<(), DrawingLineSettingsError> { todo!() }
    /// Set Fibonacci levels
    pub fn set_fibonacci_levels(&self, levels: Vec<FibonacciLevelSetting>) -> Result<(), DrawingLineSettingsError> { todo!() }
    /// Set fill color
    pub fn set_fill_color(&self, color: String, color_blk: Option<String>) -> Result<(), DrawingLineSettingsError> { todo!() }
    /// Set hide all lines flag
    pub fn set_hide_all_lines(&self, hide: bool) -> Result<(), DrawingLineSettingsError> { todo!() }
    /// Set line color
    pub fn set_line_color(&self, color: String, color_blk: Option<String>) -> Result<(), DrawingLineSettingsError> { todo!() }
    /// Set line style
    pub fn set_line_style(&self, style: u8) -> Result<(), DrawingLineSettingsError> { todo!() }
    /// Set line width
    pub fn set_line_width(&self, width: f64) -> Result<(), DrawingLineSettingsError> { todo!() }
}

/// Drawing line settings manager
/// 
/// Provides thread-safe access to drawing line user preferences.
/// Settings are persisted to disk and loaded on demand.
pub struct DrawingLineSettingsManager;
impl DrawingLineSettingsManager {
    /// Create a new settings manager for the given setting case
    pub fn new(setting_case: ChartSettingCase) -> Arc<DrawingLineSettingsManager> { todo!() }
    /// Get default Fibonacci levels
    pub fn get_default_fibonacci_levels(&self) -> Vec<FibonacciLevelSetting> { todo!() }
    /// Get default fill color alpha
    pub fn get_default_fill_alpha(&self) -> f64 { todo!() }
    /// Get default line color
    pub fn get_default_line_color(&self) -> String { todo!() }
    /// Get default line width
    pub fn get_default_line_width(&self) -> f64 { todo!() }
    /// Get default drawing line settings
    pub fn get_default_settings(&self) -> DrawingLineSettings { todo!() }
    /// Get Fibonacci levels
    pub fn get_fibonacci_levels(&self) -> Vec<FibonacciLevelSetting> { todo!() }
    /// Get fill color
    pub fn get_fill_color(&self) -> String { todo!() }
    /// Get fill color block
    pub fn get_fill_color_blk(&self) -> Option<String> { todo!() }
    /// Get hide all lines flag
    pub fn get_hide_all_lines(&self) -> bool { todo!() }
    /// Get line color and color block
    pub fn get_line_color(&self) -> String { todo!() }
    /// Get line color block
    pub fn get_line_color_blk(&self) -> Option<String> { todo!() }
    /// Get line style (0 = Solid, 1 = Dash)
    pub fn get_line_style(&self) -> u8 { todo!() }
    /// Get line width
    pub fn get_line_width(&self) -> f64 { todo!() }
    /// Get the setting case
    pub fn get_setting_case(&self) -> ChartSettingCase { todo!() }
    /// Load all settings from disk
    pub fn load_settings(&self) -> DrawingLineSettings { todo!() }
    /// Reset all settings to default
    pub fn reset_all(&self) -> Result<(), DrawingLineSettingsError> { todo!() }
    /// Reset Fibonacci levels to default
    pub fn reset_fibonacci_levels(&self) -> Result<(), DrawingLineSettingsError> { todo!() }
    /// Save all settings to disk
    pub fn save_settings(&self, settings: DrawingLineSettings) -> Result<(), DrawingLineSettingsError> { todo!() }
    /// Set Fibonacci levels
    pub fn set_fibonacci_levels(&self, levels: Vec<FibonacciLevelSetting>) -> Result<(), DrawingLineSettingsError> { todo!() }
    /// Set fill color
    pub fn set_fill_color(&self, color: String, color_blk: Option<String>) -> Result<(), DrawingLineSettingsError> { todo!() }
    /// Set hide all lines flag
    pub fn set_hide_all_lines(&self, hide: bool) -> Result<(), DrawingLineSettingsError> { todo!() }
    /// Set line color
    pub fn set_line_color(&self, color: String, color_blk: Option<String>) -> Result<(), DrawingLineSettingsError> { todo!() }
    /// Set line style
    pub fn set_line_style(&self, style: u8) -> Result<(), DrawingLineSettingsError> { todo!() }
    /// Set line width
    pub fn set_line_width(&self, width: f64) -> Result<(), DrawingLineSettingsError> { todo!() }
}

/// Indicator calculator for computing technical indicators.
/// 
/// # Example
/// ```ignore
/// let mut wrapper = CandleDataWrapper::from_bars(bars);
/// let calc = IndicatorCalculator::new(&mut wrapper);
/// let ma = calc.ma(&params);
/// ```
pub struct IndicatorCalculator;
impl IndicatorCalculator {
    /// Creates a new indicator calculator with the given candle data wrapper.
    pub fn new(wrapper: Arc<CandleDataWrapper>) -> Arc<IndicatorCalculator> { todo!() }
    /// Calculates Average True Range as a standalone indicator.
    /// 
    /// TR = max(H-L, |H-prevC|, |L-prevC|)
    /// ATR = RMA(TR, period)
    pub fn atr(&self, params: AtrInputParams) -> AtrOutput { todo!() }
    /// Calculates Average Value Line.
    /// 
    /// AVL = amount / coin_volume per candle (always uses coin-denominated volume,
    /// not the display-mode-dependent volume).
    /// If coin_volume is zero, AVL is NaN for that bar.
    pub fn avl(&self) -> AvlOutput { todo!() }
    /// Calculates Bull Bear Power (aligned with iOS: combined single line).
    /// 
    /// BBP = High + Low - 2 * EMA(Close, length)
    /// bull_power holds the combined BBP value, bear_power is zeroed.
    pub fn bbp(&self, params: BbpInputParams) -> BbpOutput { todo!() }
    /// Calculates Bollinger Bands.
    /// 
    /// # Formula
    /// - Middle Band (BOLL): SMA(close, period)
    /// - Upper Band (UB): BOLL + mult × StdDev(close, period)
    /// - Lower Band (LB): BOLL - mult × StdDev(close, period)
    /// 
    /// # Algorithm Note
    /// Standard deviation uses the **centered formula** `√(Σ(x-mean)²/N)` with
    /// the precomputed SMA as mean, matching TA-Lib's `TA_BBANDS` behaviour.
    /// The previous `E[X²]-E[X]²` shortcut suffers from catastrophic cancellation
    /// on high-priced instruments (e.g. BTC ≈ 80 000) and produced values that
    /// diverged from the ObjC/TA-Lib baseline.
    /// 
    /// Android V1 exposes the first complete period at absolute index
    /// `period - 1` through `RangeOutput(period)`.
    /// 
    /// # Complexity
    /// - Time: O(n × period). period is small (default 20) so this is fine.
    pub fn boll(&self, params: BollInputParams) -> BollOutput { todo!() }
    /// Calculates BRAR indicator.
    /// 
    /// - AR = SUM(H - O) / SUM(O - L) * 100
    /// - BR = SUM(max(H - prevC, 0)) / SUM(max(prevC - L, 0)) * 100
    pub fn brar(&self, params: BrarInputParams) -> BrarOutput { todo!() }
    /// Calculates CCI for multiple periods.
    /// 
    /// CCI = (TP - SMA(TP)) / (0.015 * Mean Deviation)
    pub fn cci(&self, params: CciInputParams) -> MultiLineOutput { todo!() }
    /// Calculates Cumulative Volume Delta.
    /// 
    /// CVD = cumulative(takerBuyVol - takerSellVol), resets daily at UTC 00:00.
    pub fn cvd(&self) -> CvdOutput { todo!() }
    /// Calculates DMA indicator.
    /// 
    /// - DIF = MA(fast) - MA(slow)
    /// - DIF_MA = MA(DIF)
    pub fn dma(&self, params: DmaInputParams) -> DmaOutput { todo!() }
    /// Calculates DMI indicator.
    /// 
    /// - +DM = max(High - prevHigh, 0)
    /// - -DM = max(prevLow - Low, 0)
    /// - TR = max(H-L, |H-prevC|, |L-prevC|)
    /// - +DI = Expmema(+DM) / Expmema(TR) * 100
    /// - -DI = Expmema(-DM) / Expmema(TR) * 100
    /// - DX = |+DI - -DI| / (+DI + -DI) * 100
    /// - ADX = Expmema(DX)
    /// - ADXR = (ADX + prevADX) / 2
    pub fn dmi(&self, params: DmiInputParams) -> DmiOutput { todo!() }
    /// Calculates Donchian Channels.
    /// 
    /// Upper = highest high over period
    /// Lower = lowest low over period
    /// Basis = (Upper + Lower) / 2
    pub fn donchian_channels(&self, params: DonchianChannelsInputParams) -> DonchianChannelsOutput { todo!() }
    /// Calculates Exponential Moving Average (EMA) for multiple periods.
    pub fn ema(&self, params: EmaInputParams) -> MultiLineOutput { todo!() }
    /// Calculates EMV indicator (Tongdaxin-style formula, aligned with iOS/Android).
    /// 
    /// - VOLUME_RATIO = MA(VOL, N) / VOL
    /// - MID = 100 * (H + L - prevH - prevL) / (H + L)
    /// - HL_RANGE_RATIO = (H - L) / MA(H - L, N)
    /// - preEMV = MID * VOLUME_RATIO * HL_RANGE_RATIO
    /// - EMV = MA(preEMV, N)
    /// - MAEMV = MA(EMV, M)
    pub fn emv(&self, params: EmvInputParams) -> EmvOutput { todo!() }
    /// Calculates KDJ indicator.
    /// 
    /// - RSV = (Close - LLV) / (HHV - LLV) * 100
    /// - K = SMA(RSV, smooth_k)
    /// - D = SMA(K, smooth_d)
    /// - J = 3K - 2D
    pub fn kdj(&self, params: KdjInputParams) -> KdjOutput { todo!() }
    /// Calculates Know Sure Thing.
    /// 
    /// KST = SMA(ROC1, sma1) * 1 + SMA(ROC2, sma2) * 2 + SMA(ROC3, sma3) * 3 + SMA(ROC4, sma4) * 4
    /// Signal = SMA(KST, signal)
    pub fn kst(&self, params: KstInputParams) -> KstOutput { todo!() }
    /// Calculates Simple Moving Average (SMA) for multiple periods.
    pub fn ma(&self, params: MaInputParams) -> MultiLineOutput { todo!() }
    /// Calculates MACD indicator.
    /// 
    /// - DIFF = EMA(fast) - EMA(slow)
    /// - DEA = EMA(DIFF)
    /// - STICK = (DIFF - DEA) * 2
    pub fn macd(&self, params: MacdInputParams) -> MacdOutput { todo!() }
    /// Calculates Mass Index.
    /// 
    /// SingleEMA = EMA(High - Low, 9)
    /// DoubleEMA = EMA(SingleEMA, 9)
    /// Ratio = SingleEMA / DoubleEMA
    /// MassIndex = Sum(Ratio, sum_length)
    pub fn mass_index(&self, params: MassIndexInputParams) -> MassIndexOutput { todo!() }
    /// Calculates Money Flow Index.
    /// 
    /// TP = (H + L + C) / 3
    /// RawMF = TP × Volume
    /// MFI = 100 × PositiveMF / (PositiveMF + NegativeMF)
    pub fn mfi(&self, params: MfiInputParams) -> MfiOutput { todo!() }
    /// Calculates MTM indicator.
    /// 
    /// - MTM = Close - Close[N]
    /// - MAMTM = MA(MTM)
    pub fn mtm(&self, params: MtmInputParams) -> MtmOutput { todo!() }
    /// Calculates Negative Volume Index aligned with TradingView PineScript v6:
    /// nvi[0] = 1000.0
    /// nvi[i] = nvi[i-1] * close[i] / close[i-1]
    /// if volume[i] < volume[i-1] && |close[i-1]| > 1e-12
    /// nvi[i] = nvi[i-1]  otherwise
    /// manvi = EMA(nvi, ema_length)
    pub fn nvi(&self, params: NviInputParams) -> NviOutput { todo!() }
    /// Calculates Nadaraya-Watson Envelope indicator.
    /// 
    /// Uses Gaussian kernel regression for smoothing.
    pub fn nwe(&self, params: NweInputParams) -> NweOutput { todo!() }
    /// Calculates OBV indicator.
    /// 
    /// - OBV = cumsum(sign(close_change) * volume)
    /// - MA_OBV = MA(OBV)
    pub fn obv(&self, params: ObvInputParams) -> ObvOutput { todo!() }
    /// Calculates Price Momentum Oscillator (matches ObjC pmoIndexWithParam:):
    /// roc[i] = (close[i] / close[i-1] - 1) * 100         (roc[0] = 0)
    /// smoothedROC = EMA(roc, length1)
    /// pmoInput = smoothedROC * 10
    /// pmo = EMA(pmoInput, length2)
    /// signal = EMA(pmo, signalLength)
    pub fn pmo(&self, params: PmoInputParams) -> PmoOutput { todo!() }
    /// Calculates PSY indicator.
    /// 
    /// - PSY = Count(Close > Close[1]) / N * 100
    /// - MAPSY = MA(PSY)
    pub fn psy(&self, params: PsyInputParams) -> PsyOutput { todo!() }
    /// Calculates Price Volume Trend.
    /// 
    /// PVT = prev_PVT + volume * (close - prev_close) / prev_close
    pub fn pvt(&self) -> PvtOutput { todo!() }
    /// Calculates QQE indicator — iOS/TradingView Pine Script aligned.
    /// 
    /// Params mapping:
    /// rsi_length    -> rsiPeriod (default 14)
    /// smoothing_factor -> sf (RSI EMA smoothing, default 2)
    /// fast_factor   -> fast trailing level multiplier (default 2.618)
    /// slow_factor   -> slow trailing level multiplier (default 4.236)
    /// 
    /// Output:
    /// qqe        -> RSIndex (EMA-smoothed RSI)
    /// upper_band -> FastAtrRSI (trailing level with fast factor)
    /// lower_band -> SlowAtrRSI (trailing level with slow factor)
    pub fn qqe(&self, params: QqeInputParams) -> QqeOutput { todo!() }
    /// Calculates RCI Ribbon (matches ObjC rciRibbonIndexWithParam:):
    /// For each bar i and window length L:
    /// - Take last L closes (oldest..newest).
    /// - Price rank: ascending (smallest=1), ties use average rank.
    /// - Time rank: oldest=1 ... newest=L.
    /// - d[k] = priceRank[k] - timeRank[k]
    /// - rci = (1 - 6 * Σd² / (L * (L² - 1))) * 100
    /// First L-1 bars in each line are 0.0 (alignment with ObjC).
    pub fn rci_ribbon(&self, params: RciRibbonInputParams) -> RciRibbonOutput { todo!() }
    /// Calculates ROC indicator.
    /// 
    /// - ROC = (Close - Close[N]) / Close[N] * 100
    /// - MAROC = MA(ROC)
    pub fn roc(&self, params: RocInputParams) -> RocOutput { todo!() }
    /// Calculates RSI for multiple periods.
    pub fn rsi(&self, params: RsiInputParams) -> MultiLineOutput { todo!() }
    /// Calculates Relative Volatility Index with V1 expression semantics.
    /// 
    /// Algorithm:
    /// stddev = population_stddev(close, length)
    /// upper_raw[i] = close[i] > close[i-1] ? stddev[i] : 0
    /// lower_raw[i] = close[i] <= close[i-1] ? stddev[i] : 0
    /// upper_ema = EMA(upper_raw, 14), seeded by the first value
    /// lower_ema = EMA(lower_raw, 14), seeded by the first value
    /// RVI = upper_ema / (upper_ema + lower_ema) * 100
    /// Signal = SMA(RVI, 14)
    pub fn rvi(&self, params: RviInputParams) -> RviOutput { todo!() }
    /// Calculates SAR indicator (Parabolic SAR, aligned with iOS OKKLineIndicatorDataManager).
    /// 
    /// - Always starts bullish (uptrend)
    /// - No prior-2-bar constraint on SAR
    /// - On reversal, finds N=4 bar lookback for extreme to set new SAR
    pub fn sar(&self, params: SarInputParams) -> SarOutput { todo!() }
    /// Calculates SKDJ indicator.
    /// 
    /// - RSV = (Close - LLV) / (HHV - LLV) * 100
    /// - K = EMA(EMA(RSV))
    /// - D = EMA(K)
    pub fn skdj(&self, params: SkdjInputParams) -> SkdjOutput { todo!() }
    /// Calculates the V1-compatible SLOPE indicator.
    /// 
    /// SLOPE = (MA - REF(MA, lookback)) / REF(MA, lookback) * 63.92 * 100.
    /// MA type codes are SMA=0, EMA=1, HMA=2, VWMA=3 and LSMA=4. Invalid
    /// parameters produce an all-NaN line rather than panicking or leaking a
    /// non-finite value to the renderer.
    pub fn slope(&self, params: SlopeInputParams) -> SlopeOutput { todo!() }
    /// Calculates Squeeze Momentum Indicator (TTM Squeeze).
    /// 
    /// Combines Bollinger Band / Keltner Channel squeeze detection
    /// with linear regression momentum.
    pub fn smi(&self, params: SmiInputParams) -> SmiOutput { todo!() }
    /// Calculates Support and Resistance levels based on volume profile.
    /// 
    /// Note: SR requires additional runtime parameters beyond `SrInputParams`
    /// because the calculation depends on the visible range and price scale.
    pub fn sr(&self, params: SrInputParams, range: VisibleRange, max_price: f64, min_price: f64, decimal_places: i32) -> SrOutput { todo!() }
    /// Calculates StochRSI indicator.
    /// 
    /// - RSI = RSI(close, length_rsi)
    /// - StochRSI = (RSI - LLV(RSI)) / (HHV(RSI) - LLV(RSI)) * 100
    /// - K = SMA(StochRSI, smooth_k)
    /// - D = SMA(K, smooth_d)
    pub fn stoch_rsi(&self, params: StochRsiInputParams) -> StochRsiOutput { todo!() }
    /// Calculates SuperTrend indicator.
    /// 
    /// - BasicUpperBand = HL2 + Multiplier * ATR
    /// - BasicLowerBand = HL2 - Multiplier * ATR
    /// - SuperTrend follows price and flips on trend change
    pub fn super_trend(&self, params: SuperTrendInputParams) -> SuperTrendOutput { todo!() }
    /// Calculates TRIX with V1's first-value-seeded recursive EMA.
    /// 
    /// - EMA1 = EMA_SMA_SEED(Close, N)
    /// - EMA2 = EMA_SMA_SEED(EMA1, N)
    /// - EMA3 = EMA_SMA_SEED(EMA2, N)
    /// - TRIX = (EMA3 - prev_EMA3) / prev_EMA3 * 100
    /// - MATRIX = MA(TRIX)
    pub fn trix(&self, params: TrixInputParams) -> TrixOutput { todo!() }
    /// Calculates Trend Strength Index (OKX self-defined, ObjC-aligned).
    /// 
    /// TSI[i] = Pearson_r(close[i-n+1..i], [0, 1, ..., n-1])
    /// 
    /// Single-line indicator (gradient bull/bear shading is the Native layer's job).
    /// Uses incremental rolling formula for O(1) per step.
    /// `length` is the period (default 14, V1-supported range [2, 200]).
    pub fn tsi(&self, params: TsiInputParams) -> TsiOutput { todo!() }
    /// Calculates Volume with MA lines.
    pub fn vol(&self, params: VolInputParams) -> VolOutput { todo!() }
    /// Calculates VR indicator.
    /// 
    /// - VR = (SUM(UpVol) + 0.5 * SUM(FlatVol)) / (SUM(DownVol) + 0.5 * SUM(FlatVol)) * 100
    /// - MAVR = MA(VR)
    pub fn vr(&self, params: VrInputParams) -> VrOutput { todo!() }
    /// Calculates VWAP indicator.
    /// 
    /// VWAP = Cumulative(TP * Volume) / Cumulative(Volume)
    /// where TP = (H + L + C) / 3
    pub fn vwap(&self, params: VwapInputParams) -> VwapOutput { todo!() }
    /// Calculates Volume Weighted Moving Average for multiple periods.
    /// 
    /// VWMA(n) = sum(Close * Volume, n) / sum(Volume, n)
    /// 
    /// Zero-volume fallback: when sum(Volume, n) == 0 the window degenerates to
    /// SMA(Close, n), matching the PRD spec on low-liquidity safety.
    pub fn vwma(&self, params: VwmaInputParams) -> MultiLineOutput { todo!() }
    /// Calculates Weighted Moving Average for multiple periods.
    /// 
    /// WMA gives more weight to recent prices:
    /// WMA(n) = (P[0]*n + P[1]*(n-1) + ... + P[n-1]*1) / (n*(n+1)/2)
    pub fn wma(&self, params: WmaInputParams) -> MultiLineOutput { todo!() }
    /// Calculates Williams %R for multiple periods.
    /// 
    /// WR = (HHV - Close) / (HHV - LLV) * 100
    pub fn wr(&self, params: WrInputParams) -> MultiLineOutput { todo!() }
}

/// Unified manager for indicator operations.
/// 
/// Provides all indicator-related functionality including:
/// - Querying indicator metadata and registry
/// - Getting default settings for indicators
/// - Persisting and loading user-customized settings
/// 
/// Android shares V1's SharedPreferences contract; iOS uses the versioned candle-support
/// UserDefaults contract.
/// 
/// # Example (Swift)
/// ```swift
/// let manager = IndicatorManager(settingCase: .cefi)
/// 
/// // Query indicators
/// let allIndicators = manager.getAllIndicators()
/// let meta = manager.getIndicatorMeta(indicatorName: .ma, indicatorType: .main)
/// 
/// // Get default settings and save
/// let settings = manager.getDefaultSettings(indicatorName: .ma, indicatorType: .main)
/// manager.saveIndicatorSettings(settings: settings)
/// 
/// // Load persisted settings
/// let loaded = manager.loadIndicatorSettings(indicatorName: .ma, indicatorType: .main)
/// ```
pub struct IndicatorManager;
impl IndicatorManager {
    /// Creates a new indicator manager for a specific business setting case.
    /// 
    /// Platform storage is selected automatically for Android/iOS.
    /// 
    /// # Parameters
    /// - `setting_case`: The business setting case (Cefi, Defi, Lite, etc.)
    pub fn new(setting_case: ChartSettingCase) -> Arc<IndicatorManager> { todo!() }
    /// Clears all indicator settings for this module.
    /// 
    /// Removes all persisted settings files.
    pub fn clear_all_indicator_settings(&self) -> Result<(), IndicatorSettingsError> { todo!() }
    /// Get all registered indicators.
    /// 
    /// # Returns
    /// A list of all available indicators with their metadata.
    pub fn get_all_indicators(&self) -> Vec<IndicatorMeta> { todo!() }
    /// Get default settings for an indicator.
    /// 
    /// Returns the default configuration for the specified indicator, including
    /// all parameters, colors, and selection states.
    /// 
    /// # Parameters
    /// - `indicator_name`: The indicator to get settings for
    /// - `indicator_type`: The indicator type (main or sub)
    /// 
    /// # Returns
    /// The default settings for the indicator.
    pub fn get_default_settings(&self, indicator_name: IndicatorName, indicator_type: IndicatorType) -> IndicatorSettings { todo!() }
    /// Get indicator metadata by name and type.
    /// 
    /// # Parameters
    /// - `indicator_name`: The indicator name enum
    /// - `indicator_type`: Whether it's a main or sub indicator
    /// 
    /// # Returns
    /// The indicator metadata if found, None otherwise.
    pub fn get_indicator_meta(&self, indicator_name: IndicatorName, indicator_type: IndicatorType) -> Option<IndicatorMeta> { todo!() }
    /// Get indicator metadata by server key.
    /// 
    /// # Parameters
    /// - `server_key`: The server synchronization key (e.g., "MA:MAIN", "MACD")
    /// 
    /// # Returns
    /// The indicator metadata if found, None otherwise.
    pub fn get_indicator_meta_by_server_key(&self, server_key: String) -> Option<IndicatorMeta> { todo!() }
    /// Get the main chart indicators available for the current setting case and AB state.
    /// 
    /// Returns only indicators whose `enable_cases` / `disable_cases` allow the current
    /// setting case AND whose `ab_test_key` (if any) hits "treatment". Mirrors ObjC
    /// `OKKLineToolbarDataCenter.mainIndicatorsArrayWithSettingCase:` + `p_isIndicatorABTestHit:`.
    pub fn get_main_indicators(&self) -> Vec<IndicatorMeta> { todo!() }
    /// Get the sub chart indicators available for the current setting case and AB state.
    /// See `get_main_indicators` for filter rules.
    /// 
    /// NOTE: this includes trading-data indicators because they are registered as
    /// `IndicatorType::Sub` in the registry. Use `get_sub_indicators_excluding_trading`
    /// + `get_trading_indicators` if you need them separated for UI rendering.
    pub fn get_sub_indicators(&self) -> Vec<IndicatorMeta> { todo!() }
    /// Get the sub chart indicators with the trading-data subset removed.
    /// 
    /// Mirrors `ChartIndicatorStoreService.defaultSubIndicatorWithoutTradingArray()` on the
    /// store-service pipeline. UI screens that render sub and trading as separate sections
    /// (e.g. the indicator list VC, the Panel sheet) should use this method together with
    /// `get_trading_indicators` instead of calling `get_sub_indicators` and bucketing the
    /// result themselves.
    /// 
    /// Filter chain is identical to `get_sub_indicators` (case + AB).
    pub fn get_sub_indicators_excluding_trading(&self) -> Vec<IndicatorMeta> { todo!() }
    /// Get the trading-data indicators (OI / EliteLS / TakerVOL / CVD …) for the current
    /// setting case and AB state.
    /// 
    /// Mirrors `ChartIndicatorStoreService.defaultTradingDataIndicatorArray()` on the
    /// store-service pipeline. The set is derived from the static `TRADING_INDICATORS`
    /// registry subset; case + AB filter chain matches `get_main_indicators`.
    pub fn get_trading_indicators(&self) -> Vec<IndicatorMeta> { todo!() }
    /// Checks if there are persisted settings for an indicator.
    /// 
    /// # Parameters
    /// - `indicator_name`: The indicator to check
    /// - `indicator_type`: The indicator type (main or sub)
    /// 
    /// # Returns
    /// true if there are persisted settings, false otherwise.
    pub fn has_indicator_settings(&self, indicator_name: IndicatorName, indicator_type: IndicatorType) -> bool { todo!() }
    /// Loads indicator settings from storage.
    /// 
    /// Returns the persisted settings if found, otherwise returns the default settings.
    /// 
    /// # Parameters
    /// - `indicator_name`: The indicator to load settings for
    /// - `indicator_type`: The indicator type (main or sub)
    /// 
    /// # Returns
    /// The indicator settings (persisted or default).
    pub fn load_indicator_settings(&self, indicator_name: IndicatorName, indicator_type: IndicatorType) -> IndicatorSettings { todo!() }
    /// Resets indicator settings to defaults.
    /// 
    /// Removes any persisted user modifications for the indicator.
    /// 
    /// # Parameters
    /// - `indicator_name`: The indicator to reset
    /// - `indicator_type`: The indicator type (main or sub)
    pub fn reset_indicator_settings(&self, indicator_name: IndicatorName, indicator_type: IndicatorType) -> Result<(), IndicatorSettingsError> { todo!() }
    /// Saves indicator settings to storage.
    /// 
    /// # Parameters
    /// - `settings`: The indicator settings to save
    /// 
    /// # Returns
    /// Ok(()) on success, or an error if serialization/storage fails.
    pub fn save_indicator_settings(&self, settings: IndicatorSettings) -> Result<(), IndicatorSettingsError> { todo!() }
}

pub struct IndicatorSettingsLocalInteractor;
impl IndicatorSettingsLocalInteractor {
    /// Compatibility-only entry point for clients generated against the earlier Native relay
    /// API. New Android/iOS integrations must let retail-kline merge downloads directly into the
    /// shared repository.
    pub fn apply_synced_visible_states(&self, visible_states: Vec<IndicatorVisibleState>, time_version: String) { todo!() }
    /// Available MAIN indicator names — the registry pool for this case, case + AB
    /// filtered. This is the name list the chart / Panel / `OKKLineToolbarDataCenter`
    /// consume; cf. `get_main_indicators()` which returns full `IndicatorMeta` for the
    /// settings list.
    pub fn available_main_indicator_names(&self) -> Vec<String> { todo!() }
    /// Available SUB indicator names (excludes trading-data) — registry pool, case + AB filtered.
    pub fn available_sub_indicator_names(&self) -> Vec<String> { todo!() }
    /// Available trading-data indicator names — registry pool, case + AB filtered.
    pub fn available_trading_indicator_names(&self) -> Vec<String> { todo!() }
    /// Monotonic version of the case-scoped shared indicator configuration.
    /// 
    /// Native surfaces use this cheap synchronous read to distinguish a real
    /// cross-page settings change from an ordinary lifecycle resume. The
    /// repository remains the source of truth; the asynchronously mirrored
    /// interactor state is deliberately not consulted here.
    pub fn configuration_version(&self) -> u64 { todo!() }
    /// Apply the final Native-owned numeric policy before selection initialization. Rust does not
    /// read or interpret the platform AB key.
    pub fn configure_sub_indicator_limit(&self, limit: i32) { todo!() }
    /// Synchronous snapshot of currently selected all-sub indicators (sub + trading-data).
    /// See `current_selected_main` for rationale.
    pub fn current_selected_all_sub(&self) -> Vec<String> { todo!() }
    /// Synchronous snapshot of currently selected main indicators.
    /// 
    /// Reads the case-scoped repository directly so callers see writes from every page
    /// immediately, before the asynchronous state mirror reaches Native.
    pub fn current_selected_main(&self) -> Vec<String> { todo!() }
    /// Currently selected SUB indicators (excludes trading-data), derived from the
    /// all-sub selection. Consistent with the independent SUB persistence key
    /// (all-sub = concat(SUB, TRADING) at the storage layer, so this filtered view
    /// equals the SUB key contents).
    pub fn current_selected_sub(&self) -> Vec<String> { todo!() }
    /// Currently selected trading-data indicators, derived from the all-sub selection.
    /// Consistent with the independent TRADING persistence key (see `current_selected_sub`).
    pub fn current_selected_trading(&self) -> Vec<String> { todo!() }
    /// Get default parameter settings for an indicator.
    pub fn get_default_indicator_params(&self, indicator_name: IndicatorName, indicator_type: IndicatorType) -> IndicatorSettings { todo!() }
    /// Indicators that the shared Rust execution engine can currently render
    /// on the main chart. V2 selection/settings pages must use this list rather
    /// than the full product registry; unsupported items have no Native
    /// fallback on the Rust path.
    pub fn get_executable_main_indicators(&self) -> Vec<IndicatorMeta> { todo!() }
    /// Executable sub-chart indicators for the V2 Rust path.
    pub fn get_executable_sub_indicators(&self) -> Vec<IndicatorMeta> { todo!() }
    /// Load the current parameter settings for an indicator (synchronous getter).
    pub fn get_indicator_params(&self, indicator_name: IndicatorName, indicator_type: IndicatorType) -> IndicatorSettings { todo!() }
    /// Get all available main indicators. Call after `indicators_version` changes.
    pub fn get_main_indicators(&self) -> Vec<IndicatorMeta> { todo!() }
    /// Get all available sub indicators (includes trading-data indicators, which Rust
    /// registers as `IndicatorType::Sub`). For screens that render sub and trading as
    /// separate sections, prefer `get_sub_indicators_excluding_trading()` +
    /// `get_trading_indicators()`.
    pub fn get_sub_indicators(&self) -> Vec<IndicatorMeta> { todo!() }
    /// Get sub indicators with the trading-data subset removed (Rust-side bucketing).
    pub fn get_sub_indicators_excluding_trading(&self) -> Vec<IndicatorMeta> { todo!() }
    /// Get trading-data indicators (OI / EliteLS / TakerVOL / CVD …).
    pub fn get_trading_indicators(&self) -> Vec<IndicatorMeta> { todo!() }
    /// Bootstrap the cedefi local store from Native NSUserDefaults (one-shot, called
    /// by the Swift owner whenever a cedefi events-manager is freshly created/rebuilt).
    pub fn hydrate_cedefi_local_store(&self, main: Vec<String>, all_sub: Vec<String>) { todo!() }
    /// Initialize: load indicator lists and selections for the given setting case.
    pub fn initialize(&self, setting_case: ChartSettingCase) { todo!() }
    /// Cleanup when the page is destroyed.
    pub fn on_clear(&self) { todo!() }
    /// Reconcile writes performed outside this Rust repository (notably Android V1, which shares
    /// the same SharedPreferences files). This is intentionally synchronous: a V2 lifecycle
    /// checkpoint must publish the platform-backed revision before Native asks the chart to sync
    /// its selection. Unchanged storage is a no-op.
    pub fn refresh_from_platform_storage(&self) -> u64 { todo!() }
    /// Stop-gap teardown (TEMPORARY — target state is the stream APIs,
    /// which need no teardown call).
    /// 
    /// One idempotent call at host lifecycle end that breaks the
    /// UniFFI handleMap cycle for any registered callback observers,
    /// ends active streams, and stops effects. On Kotlin call BEFORE
    /// `destroy()`. Includes `stop()`.
    pub fn release(&self) { todo!() }
    /// Remove the state observer (UniFFI export).
    /// 
    /// DEPRECATED: callback-based observation. Use `state_stream()` —
    /// it needs no removal call. Until migrated, every
    /// `set_state_observer` MUST be paired with this (or `release()`)
    /// at host lifecycle end, or the UniFFI handleMap pins the
    /// observer and its host forever.
    pub fn remove_state_observer(&self) { todo!() }
    /// Reset an indicator's parameters to defaults (local persistence only).
    pub fn reset_indicator_params(&self, indicator_name: IndicatorName, indicator_type: IndicatorType) { todo!() }
    /// Save updated parameter settings for an indicator (local persistence only).
    pub fn save_indicator_params(&self, settings: IndicatorSettings) { todo!() }
    /// Batch-replace the selected-all-sub list (sub + trading-data) in one shot
    /// (local write only).
    pub fn save_selected_all_sub(&self, indicators: Vec<String>) { todo!() }
    /// Batch-replace the selected-main list in one shot (local write only).
    pub fn save_selected_main(&self, indicators: Vec<String>) { todo!() }
    /// Batch-replace the selected SUB indicators (excludes trading-data), persisting to
    /// the independent SUB key (local write only). Refreshes the all-sub view in State.
    pub fn save_selected_sub(&self, indicators: Vec<String>) { todo!() }
    /// Batch-replace the selected trading-data indicators, persisting to the independent
    /// TRADING key (local write only). Refreshes the all-sub view in State.
    pub fn save_selected_trading(&self, indicators: Vec<String>) { todo!() }
    /// Atomically replace the complete visible selection. Selection panels use this instead of
    /// publishing separate main/sub revisions, so every calculation runtime observes one
    /// coherent configuration and cloud sync captures one durable change set.
    pub fn save_selection(&self, main: Vec<String>, all_sub: Vec<String>) { todo!() }
    /// Enable or disable debug logging.
    /// 
    /// In release builds this is a no-op, but the symbol must exist
    /// so that UniFFI bindings generated in debug mode continue to
    /// link successfully against the release library.
    pub fn set_debug(&self, _enabled: bool) { todo!() }
    /// Set a state observer (UniFFI export).
    /// 
    /// DEPRECATED: callback-based observer. Use `state_stream()` —
    /// the callback observer pins the foreign object in UniFFI's
    /// static handleMap and produces a cross-language retain cycle
    /// (Android GC + iOS ARC).
    pub fn set_state_observer(&self, observer: Arc<dyn InteractorStateChangeObserver>) { todo!() }
    /// Start the interactor lifecycle.
    /// 
    /// Calls the user's `InteractorLifecycle::start()` implementation.
    /// Override that method to add custom startup behavior (e.g., initial data loads).
    pub fn start(&self) { todo!() }
    /// Get the current state (UniFFI export).
    /// 
    /// Returns the External type (UniFFI-compatible).
    pub fn state(&self) -> IndicatorSettingsLocalState { todo!() }
    /// Opt-in async state-change stream (cycle-free, signal-only).
    /// Single-subscriber, latest-wins. `changed()` resolves `true` on
    /// change (pull `state()` for the snapshot), `false` when the
    /// interactor is released or a newer subscription replaces this one.
    pub fn state_stream(&self) -> Arc<InteractorStateStream> { todo!() }
    /// Stop the interactor and cancel all effects.
    /// 
    /// Calls the user's `InteractorLifecycle::stop()` implementation first,
    /// then cancels all effects as a framework guarantee.
    pub fn stop(&self) { todo!() }
    /// Toggle an indicator on or off. The process-shared cloud coordinator observes the
    /// resulting Local-origin repository revision; Native does not relay an upload payload.
    pub fn toggle_indicator(&self, indicator_name: String, is_main: bool, enabled: bool) { todo!() }
}

/// Platform-backed storage facade for one setting case.
/// 
/// Android uses the existing V1 SharedPreferences files and keys; iOS uses candle-support's
/// versioned UserDefaults namespace. The obsolete App Support JSON layout is intentionally not
/// read or migrated.
pub struct IndicatorStore;
impl IndicatorStore {
}

// -----------------------------------------------------------------------------
// 4. Top-Level Exported Functions (70)
// -----------------------------------------------------------------------------

/// Returns the indicator-related base keys defined by this crate.
/// 
/// Compatibility export only. New Native integrations must not use these keys for indicator
/// persistence; use `IndicatorSettingsLocalInteractor` instead.
pub fn chart_indicator_keys() -> ChartIndicatorKeys { todo!() }

/// Returns the standard set of KLine period string constants.
pub fn chart_kline_period_constants() -> ChartKlinePeriodConstants { todo!() }

/// Returns the period duration gap in seconds for the given period string.
/// 
/// This mirrors native `OKKLinePeriodDurationGap(period)` behavior:
/// lazily initializes a static map once, then does a lookup per call.
/// 
/// - Returns `None` if `period` is not a supported fixed-interval period (e.g. time chart / market cap).
pub fn chart_period_duration_gap_seconds(period: String) -> Option<i64> { todo!() }

/// Returns the period-related base keys defined by this crate.
/// 
/// Native bridges should use this instead of hardcoding string literals.
pub fn chart_period_keys() -> ChartPeriodKeys { todo!() }

/// Returns the chart settings base keys defined by this crate.
/// 
/// Native bridges should use this instead of hardcoding string literals.
pub fn chart_settings_base_keys() -> ChartSettingsBaseKeys { todo!() }

/// Create a new CandleSettingsInteractor with settings auto-loaded from Rust stores.
/// Native can call `load_settings()` later to reload if needed.
/// 
/// # Contract: inputs must be in place BEFORE the first derived read
/// 
/// `load_settings` runs here with whatever inputs exist at creation time:
/// - SCENE context defaults (Spot / non-arbi) until native pushes
/// `set_chart_context(...)` — covers pre-market / arbi gating;
/// - session-global `trade_facts` defaults (no account restriction, guide
/// capability OFF) until retail-kline's kline-init `run_ensure` pushes the
/// real account facts.
/// 
/// So `state.enable_quick_order` can start fail-open (`true` on the main Cefi
/// chart with default inputs). Native must push the scene context + call a
/// `notify_*_config_changed` BEFORE copying derived values into a per-VC
/// snapshot, and the main chart must only be created after kline-init is
/// ready (facts pushed). The standard integration (VC created on
/// `onInitReady`, `applyToolbarBtnFlags` during configuration build)
/// satisfies both naturally.
pub fn create_candle_settings_interactor(setting_case: ChartSettingCase, setting_case_layout_type: ChartSettingCaseLayoutType) -> Arc<CandleSettingsInteractor> { todo!() }

/// Creates a new `ChartIndicatorStoreService` instance for the given `setting_case`.
pub fn create_chart_indicator_store_service(setting_case: ChartSettingCase) -> Arc<ChartIndicatorStoreService> { todo!() }

/// Creates a new `ChartPeriodStoreService` instance for the given `setting_case`.
pub fn create_chart_period_store_service(setting_case: ChartSettingCase) -> Arc<ChartPeriodStoreService> { todo!() }

/// Creates a new `ChartSettingsService` instance scoped to the given setting case and layout type.
/// 
/// Each returned service holds lazily-initialized, cached configuration objects
/// (`ChartIndicatorsConfiguration`, `ChartPeriodsConfiguration`,
/// `ChartSettingsConfiguration`, `ChartTradeFeaturesConfiguration`)
/// that native code can read and modify via setter methods.
/// 
/// # Parameters
/// - `setting_case`: The business scenario this service is scoped to (e.g. `Cefi`, `Lite`, `DexMarket`).
/// - `setting_case_layout_type`: The chart entry / layout variant within the same `SettingCase`
/// (e.g. portrait-pro, landscape-fullscreen).
/// 
/// # Returns
/// A thread-safe, reference-counted `ChartSettingsService` instance ready for use
/// from any thread on the native side.
pub fn create_chart_settings_service(setting_case: ChartSettingCase, setting_case_layout_type: ChartSettingCaseLayoutType) -> Arc<ChartSettingsService> { todo!() }

/// Creates a new `ChartSettingsStoreService` instance for the given `setting_case`.
pub fn create_chart_settings_store_service(setting_case: ChartSettingCase) -> Arc<ChartSettingsStoreService> { todo!() }

/// Creates a new `ChartTradeSettingsService` instance for the given `setting_case`.
pub fn create_chart_trade_settings_service(setting_case: ChartSettingCase) -> Arc<ChartTradeSettingsService> { todo!() }

/// Creates a new `ChartTradeSettingsStoreService` instance for the given `setting_case`.
pub fn create_chart_trade_settings_store_service(setting_case: ChartSettingCase) -> Arc<ChartTradeSettingsStoreService> { todo!() }

/// Android factory that installs the host-owned limit before the first persisted selection read.
pub fn create_configured_indicator_settings_local_interactor(setting_case: ChartSettingCase, sub_indicator_limit: i32) -> Arc<IndicatorSettingsLocalInteractor> { todo!() }

pub fn create_indicator_settings_local_interactor(setting_case: ChartSettingCase) -> Arc<IndicatorSettingsLocalInteractor> { todo!() }

/// Get all panel colors (5 rows × 9 columns).
/// 
/// Returns Vec of rows, each row is Vec of ColorPair with light/dark mode values.
pub fn get_all_panel_colors() -> Vec<Vec<ColorPair>> { todo!() }

pub fn get_atr_settings() -> AtrSettings { todo!() }

pub fn get_avl_settings() -> AvlSettings { todo!() }

pub fn get_bbp_settings() -> BbpSettings { todo!() }

pub fn get_boll_settings() -> BollSettings { todo!() }

pub fn get_brar_settings() -> BrarSettings { todo!() }

pub fn get_cci_settings() -> CciSettings { todo!() }

pub fn get_cvd_settings() -> CvdSettings { todo!() }

pub fn get_dma_settings() -> DmaSettings { todo!() }

pub fn get_dmi_settings() -> DmiSettings { todo!() }

pub fn get_donchian_channels_settings() -> DonchianChannelsSettings { todo!() }

pub fn get_ema_settings() -> EmaSettings { todo!() }

pub fn get_emv_settings() -> EmvSettings { todo!() }

pub fn get_envelope_settings() -> EnvelopeSettings { todo!() }

pub fn get_kdj_settings() -> KdjSettings { todo!() }

pub fn get_kst_settings() -> KstSettings { todo!() }

pub fn get_ls_ratio_settings() -> LsRatioSettings { todo!() }

pub fn get_ma_settings() -> MaSettings { todo!() }

pub fn get_macd_settings() -> MacdSettings { todo!() }

pub fn get_mass_index_settings() -> MassIndexSettings { todo!() }

pub fn get_mfi_settings() -> MfiSettings { todo!() }

pub fn get_mtm_settings() -> MtmSettings { todo!() }

pub fn get_nvi_settings() -> NviSettings { todo!() }

pub fn get_obv_settings() -> ObvSettings { todo!() }

pub fn get_oi_settings() -> OiSettings { todo!() }

pub fn get_pmo_settings() -> PmoSettings { todo!() }

pub fn get_psy_settings() -> PsySettings { todo!() }

pub fn get_pvt_settings() -> PvtSettings { todo!() }

pub fn get_qqe_settings() -> QqeSettings { todo!() }

pub fn get_rci_ribbon_settings() -> RciRibbonSettings { todo!() }

pub fn get_roc_settings() -> RocSettings { todo!() }

pub fn get_rsi_settings() -> RsiSettings { todo!() }

pub fn get_rvi_settings() -> RviSettings { todo!() }

pub fn get_sar_settings() -> SarSettings { todo!() }

pub fn get_skdj_settings() -> SkdjSettings { todo!() }

pub fn get_slope_settings() -> SlopeSettings { todo!() }

pub fn get_smi_settings() -> SmiSettings { todo!() }

pub fn get_sr_settings() -> SrSettings { todo!() }

pub fn get_stoch_rsi_settings() -> StochRsiSettings { todo!() }

pub fn get_super_trend_settings() -> SuperTrendSettings { todo!() }

pub fn get_taker_vol_settings() -> TakerVolSettings { todo!() }

pub fn get_trix_settings() -> TrixSettings { todo!() }

pub fn get_tsi_settings() -> TsiSettings { todo!() }

pub fn get_vol_settings() -> VolSettings { todo!() }

pub fn get_vr_settings() -> VrSettings { todo!() }

pub fn get_vwap_settings() -> VwapSettings { todo!() }

pub fn get_wr_settings() -> WrSettings { todo!() }

/// Get the default indicator line color block id by index.
/// 
/// Returns `None` when `index` is out of range of the default pool.
pub fn indicator_line_color_block_at_index(index: u32) -> Option<String> { todo!() }

/// Returns the default indicators configuration scoped to `setting_case`.
/// 
/// Prefer this over `indicators_default_configuration()` to avoid reading/writing
/// the wrong settings namespace.
pub fn indicators_default_configuration_with_case(setting_case: ChartSettingCase) -> Arc<ChartIndicatorsConfiguration> { todo!() }

/// Parse a string into an indicator name enum.
/// 
/// # Parameters
/// - `name`: The indicator name string (e.g., "MA", "MACD")
/// 
/// # Returns
/// The corresponding `IndicatorName` enum variant if valid, None otherwise.
pub fn kline_indicator_name_from_string(name: String) -> Option<IndicatorName> { todo!() }

/// Convert indicator enum to its canonical string name.
/// 
/// # Parameters
/// - `indicator`: The indicator enum variant
/// 
/// # Returns
/// The canonical indicator name string (e.g., "MA", "MACD").
pub fn kline_indicator_name_to_string(indicator: IndicatorName) -> String { todo!() }

/// Returns the default periods configuration scoped to `setting_case`.
/// 
/// Prefer this over `periods_default_configuration()` to avoid reading/writing
/// the wrong settings namespace.
pub fn periods_default_configuration_with_case(setting_case: ChartSettingCase) -> Arc<ChartPeriodsConfiguration> { todo!() }

/// Returns the default settings configuration scoped to `setting_case`.
/// 
/// Prefer this over `settings_default_configuration()` to avoid reading/writing the
/// wrong settings namespace.
pub fn settings_default_configuration_with_case(setting_case: ChartSettingCase) -> Arc<ChartSettingsConfiguration> { todo!() }

/// Returns a trade-features configuration scoped to `setting_case` (compare mode).
/// 
/// Currently identical to `trade_features_default_configuration_with_case`.
/// Kept as a separate entry point for native API compatibility.
pub fn trade_features_compare_configuration_with_case(setting_case: ChartSettingCase) -> Arc<ChartTradeFeaturesConfiguration> { todo!() }

/// Returns a trade-features configuration scoped to `setting_case`.
/// 
/// Both default and compare modes use the same underlying configuration;
/// the distinction is handled at the caller level.
pub fn trade_features_default_configuration_with_case(setting_case: ChartSettingCase) -> Arc<ChartTradeFeaturesConfiguration> { todo!() }

/// Returns the trade-related base keys defined by this crate.
/// 
/// Native bridges should use this instead of hardcoding string literals.
pub fn trade_setting_keys() -> TradeSettingKeys { todo!() }
