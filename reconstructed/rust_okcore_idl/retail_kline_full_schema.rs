// =============================================================================
// Reconstructed Rust UniFFI IDL & Schema for crate: `retail_kline`
// Extracted from OKX Android v6.194.0 (lib/arm64-v8a/libOKCore.so)
// Total symbols decoded: 2018
//   Records: 345 | Enums: 82 | Interfaces: 160
//   Constructors: 31 | Methods: 1203 | Functions: 195
// =============================================================================

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

// -----------------------------------------------------------------------------
// 1. Enums (82)
// -----------------------------------------------------------------------------

/// Attach order type (Take-Profit or Stop-Loss)
#[derive(Debug, Clone, PartialEq)]
pub enum AttachOrderType {
    /// Take-profit order
    TakeProfit,
    /// Stop-loss order
    StopLoss,
}

/// Business line that drives which charts are visible.
#[derive(Debug, Clone, PartialEq)]
pub enum BigDataBizLine {
    Spot,
    Margin,
    Swap,
    Futures,
}

/// Identifies one chart. Note `MarginLongShortRatio` and `MarginBorrowVolume`
/// are distinct charts that share the loan-ratio source.
#[derive(Debug, Clone, PartialEq)]
pub enum BigDataChartId {
    AssetDistribution,
    CapitalFlow,
    MarginLongShortRatio,
    MarginBorrowVolume,
    PositionVolume,
    FundingRate,
    ContractLongShortRatio,
    BuySellVolume,
    FuturesSpotGap,
}

/// Time-period selector for a big-data chart. Aligns with `OKKTDChartsTimeInterval`
/// on the native side. Not all charts support all periods; the value domain per
/// chart is `periods_for_chart` in [`super::big_data_chart_interactor`].
/// 
/// Prefixed `BigData` to avoid a UniFFI name collision with `ChartPeriod` from
/// the `retail-trading` crate (both are exported into the same OKCore Swift module).
#[derive(Debug, Clone, PartialEq)]
pub enum BigDataChartPeriod {
    /// 1-minute granularity ("1m"). Used by FuturesSpotGap.
    OneMin,
    /// 5-minute granularity ("5m").
    FiveMin,
    /// 15-minute granularity ("15m").
    FifteenMin,
    /// 1-hour granularity ("1H").
    OneHour,
    /// 4-hour granularity ("4H").
    FourHour,
    /// 1-day granularity ("1D").
    OneDay,
    /// 1-week granularity ("1W").
    OneWeek,
    /// 1-month granularity ("1M").
    OneMonth,
}

/// Heterogeneous chart payload pulled via `BigDataChartInteractor::chart_data`.
/// `LeverageRatio` backs both `MarginLongShortRatio` and `MarginBorrowVolume`.
/// 
/// Variants differ in size (asset distribution carries many fields); since this
/// crosses the UniFFI boundary the variants can't be boxed without changing the
/// generated Swift type, so the size-difference lint is allowed here.
#[derive(Debug, Clone, PartialEq)]
pub enum BigDataChartResult {
    AssetDistribution {
        data: AssetDistributionData,
    },
    CapitalFlow {
        data: Vec<CapitalFlowData>,
    },
    LeverageRatio {
        data: Vec<LongShortRatioData>,
    },
    PositionVolume {
        data: Vec<InterestAndTradingUnionItem>,
    },
    FundingRate {
        data: Vec<FundingRateData>,
    },
    ContractLongShortRatio {
        data: Vec<ContractLongShortRatioData>,
    },
    BuySellVolume {
        data: BuySellVolumeData,
    },
    FuturesSpotGap {
        data: FuturesSpotGapData,
    },
}

/// Loading state for BizInstrumentProvider (FFI-compatible)
#[derive(Debug, Clone, PartialEq)]
pub enum BizLoadStateBridge {
    /// Initial state, not yet started
    Idle,
    /// Currently loading
    Loading,
    /// Successfully loaded
    Loaded,
    /// Loading failed with error message
    Error {
        message: String,
    },
}

/// Unit selector for the Buy/Sell Volume chart.
/// 
/// Maps to the `unit` field in `BuySellVolumeReq`:
/// - `Coin`  → "crypto" (coin-denominated volume)
/// - `Cont`  → "cont"   (contract-denominated volume)
/// - `Usds`  → "usds"   (USDT-denominated volume)
#[derive(Debug, Clone, PartialEq)]
pub enum BuySellUnit {
    Coin,
    Cont,
    Usds,
}

/// One-shot UI events for native.
#[derive(Debug, Clone, PartialEq)]
pub enum CandleAdjustmentUiEvent {
    /// A candle-adjustment push for some instrument. The channel is a full
    /// broadcast, so native filters by the instIds it cares about.
    AdjustmentReceived {
        item: CandleAdjustmentItem,
    },
    /// Single-instId HTTP query succeeded. `data` is the event package for the
    /// requested instId (events may be empty when there are none).
    RequestSucceeded {
        inst_id: String,
        data: CandleAdjustmentData,
    },
    /// Single-instId HTTP query failed (network / server error). Native should
    /// degrade gracefully (treat as no events) — never crash.
    RequestFailed {
        inst_id: String,
        error: String,
    },
    /// Batch HTTP query succeeded. `results` is one-to-one and same-order with
    /// the queries native passed to `start_multi_request`.
    MultiRequestSucceeded {
        results: Vec<CandleAdjustmentData>,
    },
    /// Batch HTTP query failed at the array level (network / 51000). Native
    /// should degrade gracefully — never crash.
    MultiRequestFailed {
        error: String,
    },
}

/// Tells native what kind of candle state change triggered the latest push.
/// Native picks the right chart-lib API (update / addHistoryCandlesData /
/// addRightHistoryCandlesData) without inferring from list shape.
#[derive(Debug, Clone, PartialEq)]
pub enum CandlesUpdateKind {
    /// No push yet, or state was cleared.
    Idle,
    /// Full snapshot replaced — initial subscribe / period switch /
    /// jump-to-time / reload.
    FullReplace,
    /// Single tick replacing the trailing candle (same ts).
    TickReplaceLast,
    /// Single tick appended as a brand-new candle (next ts).
    AppendTickNew,
    /// Older history prepended at the head.
    PrependHistoryLeft,
    /// Newer history appended at the tail.
    AppendHistoryRight,
}

/// New-coin lifecycle status, matching the `initFragments` branch order.
#[derive(Debug, Clone, PartialEq)]
pub enum CoinStatus {
    Normal,
    NewCoinCountdown,
    Auction,
    PreQuote,
}

/// One-shot UI events for native (success + failure — house convention).
#[derive(Debug, Clone, PartialEq)]
pub enum CompanyDividendUiEvent {
    /// The dividend-splits calendar loaded.
    LoadSucceeded {
        dividends: DividendSplits,
    },
    /// Fetch failed (network / server / empty). Native degrades gracefully.
    LoadFailed {
        error: String,
    },
}

/// Typed deliveries for all consumers sharing the facade.
#[derive(Debug, Clone, PartialEq)]
pub enum CompanyExperienceUiEvent {
    ProfileReady {
        generation: u64,
        request_generation: u64,
        presentation: CompanyProfileExperiencePresentation,
        from_cache: bool,
    },
    ProfileFailed {
        generation: u64,
        request_generation: u64,
        failure: CompanyExperienceFailure,
    },
    ShareholdersReady {
        generation: u64,
        request_generation: u64,
        presentation: CompanyShareholdersPresentation,
        from_cache: bool,
    },
    ShareholdersNoData {
        generation: u64,
        request_generation: u64,
        presentation: CompanyShareholdersPresentation,
        from_cache: bool,
    },
    ShareholdersFailed {
        generation: u64,
        request_generation: u64,
        failure: CompanyExperienceFailure,
    },
    FinancialsReady {
        generation: u64,
        request_generation: u64,
        presentation: CompanyFinancialsPresentation,
        from_cache: bool,
    },
    FinancialsNoData {
        generation: u64,
        request_generation: u64,
        from_cache: bool,
    },
    FinancialsFailed {
        generation: u64,
        request_generation: u64,
        failure: CompanyExperienceFailure,
    },
    StatementReady {
        generation: u64,
        request_generation: u64,
        kind: StatementKind,
        timeframe: String,
        range_kind: String,
        range_value: Option<i32>,
        presentation: CompanyStatementPresentation,
        from_cache: bool,
    },
    StatementFailed {
        generation: u64,
        request_generation: u64,
        kind: StatementKind,
        timeframe: String,
        failure: CompanyExperienceFailure,
    },
    DividendsReady {
        generation: u64,
        request_generation: u64,
        presentation: CompanyDividendsPresentation,
        from_cache: bool,
    },
    DividendsNoData {
        generation: u64,
        request_generation: u64,
        presentation: CompanyDividendsPresentation,
        from_cache: bool,
    },
    DividendsFailed {
        generation: u64,
        request_generation: u64,
        failure: CompanyExperienceFailure,
    },
    EtfTopHoldingsReady {
        generation: u64,
        request_generation: u64,
        presentation: CompanyEtfTopHoldingsPresentation,
    },
    EtfTopHoldingsNoData {
        generation: u64,
        request_generation: u64,
        presentation: CompanyEtfTopHoldingsPresentation,
    },
    EtfTopHoldingsFailed {
        generation: u64,
        request_generation: u64,
        failure: CompanyExperienceFailure,
    },
    /// M2 full Holdings list. `presentation` is always the COMPLETE Rust-merged
    /// list (first page, refresh, sort change, or an appended load-more) — native
    /// replaces its list with it and never concatenates.
    EtfHoldingsReady {
        generation: u64,
        request_generation: u64,
        presentation: CompanyEtfHoldingsPagePresentation,
        has_more: bool,
    },
    EtfHoldingsNoData {
        generation: u64,
        request_generation: u64,
        presentation: CompanyEtfHoldingsPagePresentation,
    },
    EtfHoldingsFailed {
        generation: u64,
        request_generation: u64,
        failure: CompanyExperienceFailure,
    },
    EtfAllocationsReady {
        generation: u64,
        request_generation: u64,
        presentation: CompanyEtfAllocationsPresentation,
    },
    EtfAllocationsNoData {
        generation: u64,
        request_generation: u64,
        presentation: CompanyEtfAllocationsPresentation,
    },
    EtfAllocationsFailed {
        generation: u64,
        request_generation: u64,
        failure: CompanyExperienceFailure,
    },
    EtfPerformanceReady {
        generation: u64,
        request_generation: u64,
        presentation: CompanyEtfPerformancePresentation,
    },
    EtfPerformanceNoData {
        generation: u64,
        request_generation: u64,
        presentation: CompanyEtfPerformancePresentation,
    },
    EtfPerformanceFailed {
        generation: u64,
        request_generation: u64,
        failure: CompanyExperienceFailure,
    },
    /// M5 constituent changes. Like M2, `presentation` is always the COMPLETE
    /// Rust-merged list (first page, refresh, or an appended load-more); native
    /// replaces its list and never concatenates. `has_more` drives the footer.
    EtfConstituentChangesReady {
        generation: u64,
        request_generation: u64,
        presentation: CompanyEtfConstituentChangesPresentation,
        has_more: bool,
    },
    EtfConstituentChangesNoData {
        generation: u64,
        request_generation: u64,
        presentation: CompanyEtfConstituentChangesPresentation,
    },
    EtfConstituentChangesFailed {
        generation: u64,
        request_generation: u64,
        failure: CompanyExperienceFailure,
    },
}

/// One-shot UI events for native (house convention: success + failure).
#[derive(Debug, Clone, PartialEq)]
pub enum CompanyFinancialsUiEvent {
    /// Initial load settled with at least one section present.
    LoadSucceeded {
        data: FinancialsData,
    },
    /// Initial load failed (every section empty / errored). Native degrades.
    LoadFailed {
        error: String,
    },
    /// A statement section reloaded at a new timeframe.
    SectionReloadSucceeded {
        kind: StatementKind,
        statements: FinancialStatements,
    },
    /// A statement section reload failed (network / server / empty).
    SectionReloadFailed {
        kind: StatementKind,
        error: String,
    },
}

/// One-shot UI events for native (success + failure — house convention).
#[derive(Debug, Clone, PartialEq)]
pub enum CompanyProfileUiEvent {
    /// Both endpoints settled and the profile loaded. `quote` is `None` when the
    /// quote endpoint failed or returned empty (grid renders `--`).
    LoadSucceeded {
        profile: CompanyProfile,
        quote: Option<CompanyQuote>,
    },
    /// Profile failed (network / server / empty). Native degrades gracefully.
    LoadFailed {
        error: String,
    },
}

/// One-shot UI events for native (success + failure — house convention).
#[derive(Debug, Clone, PartialEq)]
pub enum CompanyShareholdersUiEvent {
    /// The shareholders stats loaded (donut + history derived).
    LoadSucceeded {
        stats: ShareholdersStats,
    },
    /// Fetch failed (network / server / empty). Native degrades gracefully.
    LoadFailed {
        error: String,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum CompareCandleUpdateType {
    None,
    History,
    Realtime,
    MoreHistory,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CompareIndicatorUiEvent {
    Schema {
        subscription_id: u32,
        schema: KlineIndicatorSchemaVo,
    },
    GenerationReady {
        subscription_id: u32,
        generation: KlineIndicatorGenerationVo,
    },
    DataFrame {
        subscription_id: u32,
        frame: KlineIndicatorDataFrameVo,
    },
    Error {
        subscription_id: u32,
        message: String,
    },
}

/// Coarse state of the current window — Native chooses layout by this enum.
#[derive(Debug, Clone, PartialEq)]
pub enum CurrentState {
    /// Regular intraday window (Pre-market / Open / After-hours / Overnight).
    Regular,
    /// Regular window with an early close announced — banner + marker shown.
    EarlyClose,
    /// Weekend closure (recurring, no reason text).
    Weekend,
    /// Public-holiday closure (custom reason text supplied).
    Holiday,
}

/// Explicit drawing-line storage selection used by platform chart pages.
#[derive(Debug, Clone, PartialEq)]
pub enum DrawingLineStoreCase {
    Cefi,
    Cedefi,
    Defi,
}

/// UI events for one-shot notifications
#[derive(Debug, Clone, PartialEq)]
pub enum DrawingLineUiEvent {
    /// A line was successfully saved
    LineSaved {
        line_id: String,
    },
    /// Lines were successfully deleted
    LinesDeleted {
        count: u32,
    },
    /// All unsynced lines were uploaded
    LinesUploaded,
    /// Line limit exceeded (error code 51283)
    LineLimitExceeded,
    /// A save operation failed
    SaveFailed {
        error_message: String,
    },
}

/// Contract kind for the event chart.
/// 
/// Mirrors the Kotlin-side `OkEventsBiz.METHOD_PRICE_UP_DOWN` (15 minute) /
/// `METHOD_PRICE_ABOVE` (above) split that drives time-range + ws behavior.
#[derive(Debug, Clone, PartialEq)]
#[repr(i32)]
pub enum EventChartContractKind {
    /// 15-minute up/down contract.
    UpDown = 0,
    /// Price-above contract (longer term, supports ALL range).
    Above = 1,
    /// Price-hit contract — same chart behavior as Above (above-like).
    Hit = 2,
    /// Price-between contract — same chart behavior as Above (above-like).
    Between = 3,
}

/// Time range selector on the chart.
#[derive(Debug, Clone, PartialEq)]
#[repr(i32)]
pub enum EventChartTimeRange {
    H1 = 0,
    H4 = 1,
    D1 = 2,
    W1 = 3,
    /// Only valid for the probability tab on above contract; drives `full-candles`.
    All = 4,
    /// 1 month window. Behaves like a normal range (UI supplies bar/limit); not `All`.
    M1 = 5,
    /// 3 month window. Behaves like a normal range (UI supplies bar/limit); not `All`.
    M3 = 6,
}

/// Candle update type for native to know what changed
#[derive(Debug, Clone, PartialEq)]
pub enum FormulaCandleUpdateType {
    None,
    /// First HTTP data loaded (history)
    History,
    /// WS real-time push
    Realtime,
    /// Load-more pagination result
    MoreHistory,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FormulaIndicatorUiEvent {
    Schema {
        subscription_id: u32,
        schema: KlineIndicatorSchemaVo,
    },
    GenerationReady {
        subscription_id: u32,
        generation: KlineIndicatorGenerationVo,
    },
    DataFrame {
        subscription_id: u32,
        frame: KlineIndicatorDataFrameVo,
    },
    Error {
        subscription_id: u32,
        message: String,
    },
}

/// Candle update type for native to know what changed
#[derive(Debug, Clone, PartialEq)]
pub enum HalfScreenCandleUpdateType {
    None,
    History,
    Realtime,
    MoreHistory,
}

/// Indicator-only event surface for the half-screen Interactor. The encoded
/// payload and schema records are identical to the full K-line event contract;
/// a separate observer type is required because Interactor event adapters are
/// generated in the owning Rust module.
#[derive(Debug, Clone, PartialEq)]
pub enum HalfScreenIndicatorUiEvent {
    Schema {
        subscription_id: u32,
        schema: KlineIndicatorSchemaVo,
    },
    GenerationReady {
        subscription_id: u32,
        generation: KlineIndicatorGenerationVo,
    },
    DataFrame {
        subscription_id: u32,
        frame: KlineIndicatorDataFrameVo,
    },
    Error {
        subscription_id: u32,
        message: String,
    },
}

/// Instrument kind shown as a tag on the K-line header. Resolved from
/// `Instrument` metadata (`inst_type` + `is_pre_market_pair` +
/// `is_x_perps_pair`) so native renders tags without duplicating the
/// classification logic.
#[derive(Debug, Clone, PartialEq)]
pub enum HeadInstrumentKind {
    Spot,
    Margin,
    Swap,
    Futures,
    Option,
    /// Spot pair that supports margin trading (V1 "Leverage Nx" tag).
    SpotWithLever,
    PreMarketSpot,
    PreMarketSwap,
    PreMarketFutures,
    /// Cross-asset perpetual (X-Perps), futures-only.
    XPerps,
}

/// Background primitive for one indicator area. Numeric candle values are intentionally not part
/// of the indicator payload; Native reuses the candles already delivered for the active series.
#[derive(Debug, Clone, PartialEq)]
pub enum IndicatorBackdropStyle {
    None,
    Ohlc,
}

#[derive(Debug, Clone, PartialEq)]
pub enum IndicatorColorRole {
    Configured,
    Positive,
    Negative,
    Neutral,
}

/// Dynamic unit appended to an indicator legend by the chart surface.
#[derive(Debug, Clone, PartialEq)]
pub enum IndicatorInfoUnit {
    None,
    Volume,
    /// Turnover unit; Native omits a duplicate when the volume unit is the same.
    Turnover,
}

/// Rendering primitive for one calculated indicator output.
#[derive(Debug, Clone, PartialEq)]
pub enum IndicatorOutputStyle {
    Line,
    Point,
    /// Filled three-state squeeze marker: positive/on, negative/off, zero/neutral.
    SqueezeStatePoint,
    Horizontal,
    FillBoundary,
    Histogram,
    Volume,
    StackedHistogram,
    SecondaryHistogram,
    /// Polyline whose segments use positive/negative/neutral theme colors according to value sign.
    ValueSignLine,
    /// Legend value only; excluded from curves, axes and selected-point ranges.
    InfoOnly,
}

/// Retained for source/ABI compatibility with clients built against the previous relay design.
/// New clients do not need to observe these events: the shared Repository updates render runtimes
/// directly and atomically.
#[derive(Debug, Clone, PartialEq)]
pub enum IndicatorSyncUiEvent {
    SyncDownloadCompleted {
        synced_count: u32,
        time_version: String,
        visible_states: Vec<IndicatorVisibleState>,
    },
    SyncFailed {
        message: String,
    },
}

/// Locale-aware value recipe for indicator legends and axes. Rust owns the
/// semantic choice; Native applies the platform number formatter.
#[derive(Debug, Clone, PartialEq)]
pub enum IndicatorValueFormat {
    Default,
    CompactVolume,
    SignedPercent,
}

/// The five K-line business types. Replaces OC's raw `NSString*` constants
/// (`OKTBizTypeSpot`, etc.) with compile-time safety and exhaustive matching.
/// Exported via UniFFI for native callers.
#[derive(Debug, Clone, PartialEq)]
pub enum KLineBizType {
    Spot,
    Margin,
    Swap,
    Futures,
    Option,
}

/// Error source kind for order position asset data
/// 
/// UniFFI-compatible enum mirroring `KLineDataSourceKind` from the core layer.
#[derive(Debug, Clone, PartialEq)]
pub enum KLineDataSourceErrorKind {
    /// Order data source
    Order,
    /// Position data source
    Position,
    /// Asset (balance) data source (spot only)
    Asset,
    /// Quote coin price fetch (spot only)
    QuoteCoinPrice,
}

/// Candle update type for native to know what changed
#[derive(Debug, Clone, PartialEq)]
pub enum KLineExploreBotsHalfScreenCandleUpdateType {
    None,
    /// First HTTP data loaded (history)
    History,
    /// WS real-time push
    Realtime,
    /// Load-more pagination result
    MoreHistory,
}

/// A chart-level order mutation. Rust resolves every backend field from the
/// fresh [`super::RetailKLineOrder`] snapshot; Native supplies only intent.
#[derive(Debug, Clone, PartialEq)]
pub enum KLineOrderAmendAction {
    MainPrice,
    AddTakeProfit,
    AddStopLoss,
    ModifyTakeProfit {
        attach_algo_id: String,
    },
    ModifyStopLoss {
        attach_algo_id: String,
    },
}

/// Destination family used by the existing OKTrade confirmation-page bridge.
#[derive(Debug, Clone, PartialEq)]
pub enum KLineOrderAmendFamily {
    Normal,
    Strategy,
}

/// Extra business warning required before the cancellation can be submitted.
#[derive(Debug, Clone, PartialEq)]
pub enum KLineOrderCancelConfirmation {
    None,
    OpenPriceStopLoss,
    LinkedStopLossOrder,
}

/// Rust-owned whole-order cancellation request.
/// 
/// Native can carry this value between prepare and submit, but does not need
/// to understand or reconstruct backend identifiers.
#[derive(Debug, Clone, PartialEq)]
pub enum KLineOrderCancelSubmit {
    Normal {
        order_id: String,
        inst_id: String,
    },
    Strategy {
        algo_id: String,
        inst_id: String,
        order_type: String,
    },
}

/// K-Line Order Side
/// 
/// Custom order side for K-line chart display.
/// 
/// # OC Reference
/// - `OKKLineOrderSide` in `OKKLineTypes.h`
#[derive(Debug, Clone, PartialEq)]
pub enum KLineOrderSide {
    /// Neither buy nor sell
    None,
    /// Buy side
    Buy,
    /// Sell side
    Sell,
}

/// K-Line Order Type
/// 
/// Custom order type for K-line chart display.
/// 
/// # OC Reference
/// - `OKKLineOrderType` in `OKKLineTypes.h`
#[derive(Debug, Clone, PartialEq)]
pub enum KLineOrderType {
    /// Limit order
    Limit,
    /// Post-only order (advanced limit)
    PostOnly,
    /// Limit price take profit
    LimitTP,
    /// Take profit (conditional)
    TP,
    /// Stop loss (conditional)
    SL,
    /// Trailing stop
    Trailing,
    /// Trigger order (planned/scheduled)
    Trigger,
}

/// Type-safe alternative to raw bitflag constants for [`KLinePipContextBridge::start_typed`].
#[derive(Debug, Clone, PartialEq)]
pub enum KLinePipDataType {
    Ticker,
    /// Ticker + mark price (needed for contract trading PIP windows).
    Both,
}

/// K-Line Position Side
/// 
/// Custom position side for K-line chart display.
/// 
/// # OC Reference
/// - `OKKLinePositionSide` in `OKKLineTypes.h`
#[derive(Debug, Clone, PartialEq)]
pub enum KLinePositionSide {
    /// Net mode (one-way position)
    Net,
    /// Long position
    Long,
    /// Short position
    Short,
}

/// Mutually exclusive direct-submit payload prepared by retail-kline.
#[derive(Debug, Clone, PartialEq)]
pub enum KLinePreparedOrderSubmit {
    Amend {
        input: KLineAmendInputVo,
    },
    Trigger {
        input: KLineTriggerAmendInputVo,
    },
}

/// K-Line Price Mode
/// 
/// Trigger price type for K-line chart display.
/// 
/// # OC Reference
/// - `OKKLinePriceMode` in `OKKLineTypes.h`
#[derive(Debug, Clone, PartialEq)]
pub enum KLinePriceMode {
    /// Default (last price)
    Default,
    /// Mark price
    MarkPrice,
    /// Index price
    Index,
}

#[derive(Debug, Clone, PartialEq)]
pub enum KlineCandlesUiEvent {
    /// Post-write `candles_list` snapshot + the kind of that write.
    Frame {
        subscription_id: u32,
        candles: Vec<KlineQuoteCandleVo>,
        kind: CandlesUpdateKind,
        /// Same CandleStore revision used by IndicatorDataFrame.
        candle_revision: u64,
    },
    /// Low-frequency metadata for the binary indicator frames. Re-emitted
    /// when either the indicator structure or subscription identity changes.
    IndicatorSchema {
        subscription_id: u32,
        schema: KlineIndicatorSchemaVo,
    },
    /// Rust committed a coherent full indicator ledger for this candle
    /// generation. Native should retain its current pixels until it obtains a
    /// matching viewport window through `fetch_indicator_window`.
    IndicatorGenerationReady {
        subscription_id: u32,
        generation: KlineIndicatorGenerationVo,
    },
    /// High-frequency precomputed values. Numeric arrays are encoded in the
    /// versioned `payload` bytes to avoid boxed `List<Double>` values in Kotlin.
    IndicatorDataFrame {
        subscription_id: u32,
        frame: KlineIndicatorDataFrameVo,
    },
    IndicatorError {
        subscription_id: u32,
        message: String,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum KlineCompactUnit {
    None,
    TenThousand,
    HundredMillion,
    Kilo,
    Million,
    Billion,
}

/// Depth subscription surface. Rust owns surface-specific default ladder
/// selection so native callers do not duplicate it.
#[derive(Debug, Clone, PartialEq)]
pub enum KlineDepthSurface {
    /// Order-book table: default to the first ladder.
    OrderBook,
    /// Depth chart: default to the second ladder, fallback to first, then "0"
    /// when ladder metadata is unavailable.
    DepthMap,
}

/// One-shot UI events. `is_favorite` (the continuous condition driving the star)
/// stays in `State`; the toast is a transient result, so it flows here instead —
/// this keeps the toast tied to the user's toggle action and prevents the
/// subscribe-load `is_favorite` transition from firing a spurious toast.
#[derive(Debug, Clone, PartialEq)]
pub enum KlineFavoriteUxEvent {
    /// A user-initiated toggle completed; carries the new status so native shows
    /// the matching add / remove toast.
    Toggled {
        is_favorite: bool,
    },
    /// A user-initiated toggle failed.
    ToggleFailed {
        error_message: String,
    },
}

/// Whether one selected indicator can produce values for the active chart
/// context. Native owns localized presentation only; capability decisions stay
/// with the Rust runtime that owns the data source.
#[derive(Debug, Clone, PartialEq)]
pub enum KlineIndicatorAvailability {
    Available,
    MinimumFiveMinuteBar,
    UnsupportedInstrument,
}

/// Describes whether Native must replace its indicator cache or can advance an
/// existing cache from one contiguous candle generation.
#[derive(Debug, Clone, PartialEq)]
pub enum KlineIndicatorGenerationMode {
    FullRebase,
    PrependIncremental,
}

/// Which pre-listing flow the instrument is in. Passed in by native on subscribe.
/// 
/// Mirrors V1 `MODE_COIN_TYPE_NEW` / `MODE_COIN_TYPE_AUCTION` / `MODE_COIN_TYPE_PRE_QUOTE`.
#[derive(Debug, Clone, PartialEq)]
pub enum KlineNewCoinMode {
    /// Online countdown (V1 mode "0").
    New,
    /// Call auction (V1 mode "1").
    Auction,
    /// Pre-pending order / early order placement (V1 mode "2").
    PreQuote,
}

/// New-coin countdown stage. Maps `CountDownModel.stage_type` (0/1/2).
/// 
/// Mirrors V1 `showCountdown` branching on `type`.
#[derive(Debug, Clone, PartialEq)]
pub enum KlineNewCoinStage {
    /// `type == 0` — show the live countdown timer.
    CountDown,
    /// `type == 1` — "coming soon, stay tuned".
    WillOnline,
    /// `type == 2` — listing cancelled.
    Cancel,
}

/// One-shot UI events for the new-coin page.
#[derive(Debug, Clone, PartialEq)]
pub enum KlineNewCoinUiEvent {
    /// The pre-listing flow ended (online / cancelled / auction done / pre-quote
    /// done). The host should hide the banner and switch to the normal K-line.
    NewCoinFinished,
}

/// Opening mode type enumeration
/// 
/// Represents the type of opening/listing mode
#[derive(Debug, Clone, PartialEq)]
pub enum KlineOpeningModeType {
    /// Auction mode
    Auction,
    /// Fixed price mode
    FixedPrice,
    /// Pre-quote mode
    PreQuote,
    /// Unknown type
    Unknown,
}

/// Pre-pending-order detail popup mode. Maps core-trading `OktListStageType`.
/// 
/// Mirrors what V1 fed to `IPreQuoteDialogView` to choose the detail content.
#[derive(Debug, Clone, PartialEq)]
pub enum KlinePreQuoteListStage {
    /// Index-price stage.
    PreQuoteIdx,
    /// Estimated-price stage.
    PreQuoteEst,
}

/// K-line price type enumeration
#[derive(Debug, Clone, PartialEq)]
pub enum KlineType {
    /// Latest price candle (real-time market price)
    Latest,
    /// Index price candle (index price for derivatives)
    Index,
    /// Mark price candle (mark price for futures/swaps)
    MarkPrice,
}

/// Unlock progress status
/// 
/// Represents the three possible states of token unlock progress.
/// Corresponds to OKMKLineUnlockProgress in iOS.
#[derive(Debug, Clone, PartialEq)]
pub enum KlineUnlockProgressStatus {
    /// Token is unlocked
    Unlocked = 0,
    /// Token is locked
    Locked = 1,
    /// Token is untracked
    Untracked = 2,
}

/// Time zone modes for up/down indicator display
#[derive(Debug, Clone, PartialEq)]
pub enum KlineUpDownTimeZone {
    /// 24-hour mode (no time zone conversion)
    Mode24h,
    /// UTC+0 time zone
    ModeUtc0,
    /// UTC+8 time zone (e.g., China Standard Time)
    ModeUtc8,
}

/// Liquidity bar selector parsed from the backend `liquidity` string.
/// 
/// Forward-compat contract: unrecognized codes fall back to
/// [`LiquidityLevel::Unknown`] (the localized `liquidityText` still
/// renders; only the bar icon uses a neutral default).
#[derive(Debug, Clone, PartialEq)]
pub enum LiquidityLevel {
    High,
    Moderate,
    ModerateLow,
    Low,
    Unknown,
}

/// Market-cap caliber: which market-cap measure the series represents.
/// 
/// `Unknown` is the forward-compatibility fallback.
#[derive(Debug, Clone, PartialEq)]
pub enum MarketCapV2Caliber {
    /// Total market cap of the underlying asset.
    UnderlyingAsset,
    /// Circulating market cap.
    Circulating,
    /// A caliber value the client does not recognise. Forward-compatible
    /// fallback — the accompanying points are still returned.
    Unknown,
}

/// Origin of the market-cap series.
/// 
/// `Unknown` is the forward-compatibility fallback for any source value the
/// client does not recognise; it is a normal value, never an error.
#[derive(Debug, Clone, PartialEq)]
pub enum MarketCapV2Source {
    /// Financial Modeling Prep (typically TradFi instruments).
    Fmp,
    /// CoinGecko (typically crypto instruments).
    CoinGecko,
    /// A source value the client does not recognise. Forward-compatible
    /// fallback — the accompanying points are still returned.
    Unknown,
}

/// UI events for one-shot notifications
#[derive(Debug, Clone, PartialEq)]
pub enum MarketEventsUiEvent {
    /// Show new event notification
    ShowNewEventToast {
        count: i32,
    },
}

/// Market events view object for UI display
#[derive(Debug, Clone, PartialEq)]
pub enum MarketEventsVo {
    /// Header item (date separator)
    Header {
        /// Timestamp for grouping
        timestamp: i64,
    },
    /// Content item (actual event)
    Content {
        /// Event type
        event_type: String,
        /// Event timestamp in milliseconds
        time_stamp: i64,
        /// Source count
        source_count: Option<String>,
        /// Summary content ID (for navigation)
        summary_content_id: Option<String>,
        /// Event detail content
        content: String,
        /// Type title
        type_title: String,
        /// Whether to show full content (default shows max 5 lines, expand on click)
        is_show_all_content: bool,
        /// Title
        title: String,
        /// Description
        desc: String,
        /// Explanation
        explanation: Option<String>,
        /// Address
        address: Option<String>,
        /// Author IDs
        author_ids: Option<Vec<String>>,
    },
}

/// Business line type enum for display purposes
/// 
/// Represents the different trading business lines.
#[derive(Debug, Clone, PartialEq)]
pub enum MoreTradeBizType {
    /// Spot trading
    Spot,
    /// Perpetual swap trading
    Swap,
    /// Futures (delivery) trading
    Futures,
    /// Options trading
    Option,
}

/// More Trade State
/// 
/// Represents the current state of the MoreTrade list.
/// Uses enum to ensure type-safe, mutually exclusive states.
/// 
/// # Swift Usage
/// ```swift
/// func onStateChanged(state: MoreTradeState) {
/// switch state {
/// case .loading:
/// showLoading()
/// case .success(let items):
/// showData(items)
/// case .empty:
/// showEmptyView()
/// case .error(let message):
/// showError(message)
/// }
/// }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum MoreTradeState {
    /// Initial loading state
    Loading,
    /// Successfully loaded with data
    Success {
        /// List of trade items sorted by display order
        items: Vec<MoreTradeItem>,
    },
    /// Successfully loaded but no matching instruments found
    Empty,
    /// Failed to load data
    Error {
        /// Error message describing what went wrong
        message: String,
    },
}

/// UI events emitted by the More-Trade interactor for one-shot native reactions
/// (typically toast on the K-line "more trade" tab). Persistent state (star icon
/// highlight) stays on [`MoreTradeInteractorState.items[i].is_favorite`]; only
/// transient add/remove/fail signals go here so native mirrors V1's
/// `KlineHeaderFragment` toast behaviour without polling state diffs.
#[derive(Debug, Clone, PartialEq)]
pub enum MoreTradeUiEvent {
    /// Favorite was added successfully (server confirmed).
    FavoriteAdded {
        inst_id: String,
    },
    /// Favorite was removed successfully (server confirmed).
    FavoriteRemoved {
        inst_id: String,
    },
    /// Favorite toggle failed (API error / init failure); state has been
    /// reverted so native should show an error toast + let star reflect state.
    FavoriteFailed {
        inst_id: String,
        error_message: String,
    },
}

/// Mirrors `HalfScreenCandleUpdateType` but PIP has no `MoreHistory`
/// (the OC `OKTKLinePIPProvider` doesn't paginate).
#[derive(Debug, Clone, PartialEq)]
pub enum PipCandleUpdateType {
    None,
    History,
    Realtime,
}

/// UI events for one-shot notifications
#[derive(Debug, Clone, PartialEq)]
pub enum PriceAlertUiEvent {
    /// An alert was successfully added
    AlertAdded,
    /// An alert was successfully updated
    AlertUpdated,
    /// Alert(s) were successfully deleted
    AlertDeleted {
        count: u32,
    },
    /// An alert was successfully activated/deactivated
    AlertActivated {
        rule_id: u64,
        active: bool,
    },
    /// A mutation operation failed
    MutationFailed {
        error_message: String,
    },
}

#[derive(Debug, Clone, PartialEq)]
#[repr(u32)]
pub enum RetailKLinePriceAlertBizType {
    /// CeDeFi
    CeDeFi = 0,
    /// spot
    Spot = 1,
    /// futures
    Futures = 2,
    /// swap
    Swap = 3,
}

/// price change type raise or fall
#[derive(Debug, Clone, PartialEq)]
#[repr(u32)]
pub enum RetailKLinePriceAlertChangeType {
    /// reach
    Reach = 0,
    /// price raise
    PriceRaise = 1,
    /// price fall
    PriceFall = 2,
}

/// ratio changed in a period
#[derive(Debug, Clone, PartialEq)]
#[repr(u32)]
pub enum RetailKLinePriceAlertPeriodType {
    /// none
    None = 0,
    /// 5 min
    Min5 = 1,
    /// 1 hour
    Hour1 = 2,
    /// 4 hour
    Hour4 = 3,
    /// 24 hour
    Hour24 = 4,
}

#[derive(Debug, Clone, PartialEq)]
#[repr(u32)]
pub enum RetailKLinePriceAlertPromptType {
    None = 0,
    /// price raise to
    PriceRaise = 1,
    /// price fall to
    PriceFall = 2,
    /// price raise within 24 hour
    PriceRaiseWithin24Hour = 3,
    /// price fall within 24 hour
    PriceFallWithin24Hour = 4,
    /// price reach
    PriceReach = 5,
    /// us site
    US = 6,
    /// price raise within 5 min
    PriceRaiseWithin5Min = 7,
    /// price fall within 5 min
    PriceFallWithin5Min = 8,
    /// price raise within 1 hour
    PriceRaiseWithin1Hour = 9,
    /// price fall within 1 hour
    PriceFallWithin1Hour = 10,
    /// price raise within 4 hour
    PriceRaiseWithin4Hour = 11,
    /// price fall within 4 hour
    PriceFallWithin4Hour = 12,
    /// Moving Average (MA) indicator alert
    Ma = 13,
}

#[derive(Debug, Clone, PartialEq)]
#[repr(u32)]
pub enum RetailKLinePriceAlertRepeatType {
    /// always repeat
    Always = 0,
    /// only once
    None = 1,
}

/// source type of price alert
#[derive(Debug, Clone, PartialEq)]
#[repr(u32)]
pub enum RetailKLinePriceAlertSourceType {
    None = 0,
    /// CeFi
    CeFi = 1,
    /// CeDeFi
    CeDeFi = 2,
    /// CeDeFi Market Cap
    CeDeFiMarketCap = 3,
    /// Indicators (technical indicator alerts)
    Indicators = 4,
    /// ExchangeOS (DEX perp/spot alerts; identity = assetId)
    ExchangeOs = 5,
}

/// Which drawable the bottom second icon shows: the spot entry or the perp entry.
#[derive(Debug, Clone, PartialEq)]
pub enum SecondIconKind {
    Spot,
    Perp,
}

/// The three timeframe-selectable statement sections.
#[derive(Debug, Clone, PartialEq)]
pub enum StatementKind {
    Income,
    Balance,
    Cashflow,
}

/// Session status, parsed from the server schedule (see
/// [`parse_api_status`](crate::core::stock_hours::parse_api_status)).
/// `Unknown` is the not-a-stock / not-ready sentinel. Exported via UniFFI so
/// native consumers branch on a typed enum instead of magic strings.
#[derive(Debug, Clone, PartialEq)]
pub enum StockSessionStatus {
    Unknown,
    PreMarket,
    Regular,
    PostMarket,
    Overnight,
    Closed,
    Paused,
    Weekend,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TradingIndicatorUiEvent {
    /// Data fetch failed
    DataError {
        message: String,
    },
}

/// Trigger price type for stop/trigger orders
/// 
/// Determines which price to use for triggering the order.
/// 
/// # OC Mapping
/// Maps to `OKKLinePriceMode` enum
#[derive(Debug, Clone, PartialEq)]
pub enum TriggerPriceType {
    /// Last traded price (default)
    Last,
    /// Index price (average price from multiple exchanges)
    Index,
    /// Mark price (fair price for funding rate calculation)
    Mark,
}

/// How the latest state update should be rendered.
/// 
/// Best-effort: the state observer is signal-only, so UI may miss intermediate
/// transitions. Used as a hint for scroll/animation behavior.
#[derive(Debug, Clone, PartialEq)]
#[repr(i32)]
pub enum UpdateKind {
    /// No update yet (fresh state).
    Idle = 0,
    /// Full list replaced (initial HTTP load or reload).
    FullLoad = 1,
    /// New confirmed point appended; UI may animate scroll.
    NewPoint = 2,
    /// Last point value changed (unconfirmed push or same-ts replace).
    ValueUpdate = 3,
}

// -----------------------------------------------------------------------------
// 2. Records / Data Models (345)
// -----------------------------------------------------------------------------

/// Display state for one arbitrage bottom quick-jump icon. The click target is
/// resolved by the native fragment from `mArbCoinInfo`, so no jump fields are
/// carried here.
#[derive(Debug, Clone, PartialEq)]
pub struct ArbBottomIconPlan {
    pub visible: bool,
    pub clickable: bool,
}

/// Asset distribution data for spot trading
#[derive(Debug, Clone, PartialEq)]
pub struct AssetDistributionData {
    /// Timestamp
    pub ts: String,
    /// End timestamp
    pub end_ts: String,
    /// Net volume (net inflow/outflow)
    pub net_vol: String,
    /// Buy total volume
    pub buy_total_vol: String,
    /// Buy extra large volume
    pub buy_x_large_vol: String,
    /// Buy large volume
    pub buy_large_vol: String,
    /// Buy mid volume
    pub buy_mid_vol: String,
    /// Buy small volume
    pub buy_small_vol: String,
    /// Buy extra large percentage
    pub buy_x_large_perc: String,
    /// Buy large percentage
    pub buy_large_perc: String,
    /// Buy mid percentage
    pub buy_mid_perc: String,
    /// Buy small percentage
    pub buy_small_perc: String,
    /// Sell total volume
    pub sell_total_vol: String,
    /// Sell extra large volume
    pub sell_x_large_vol: String,
    /// Sell large volume
    pub sell_large_vol: String,
    /// Sell mid volume
    pub sell_mid_vol: String,
    /// Sell small volume
    pub sell_small_vol: String,
    /// Sell extra large percentage
    pub sell_x_large_perc: String,
    /// Sell large percentage
    pub sell_large_perc: String,
    /// Sell mid percentage
    pub sell_mid_perc: String,
    /// Sell small percentage
    pub sell_small_perc: String,
}

/// Request for asset distribution data (Spot)
/// API: GET /priapi/v5/rubik/stat/spot/{instName}/taker-distribution
#[derive(Debug, Clone, PartialEq)]
pub struct AssetDistributionReq {
    /// Instrument ID (e.g. "BTC-USDT")
    pub inst_id: String,
    /// Period: "5m", "15m", "1H", "4H", "1D"
    pub period: String,
}

/// Author information
#[derive(Debug, Clone, PartialEq)]
pub struct AuthorInfo {
    /// Author ID
    pub author_id: String,
    /// Label
    pub label: String,
    /// Translated bio
    pub translated_bio: String,
    /// Follow status
    pub follow_status: String,
    /// Badge list
    pub badge_list: Vec<String>,
    /// Label icon
    pub label_icon: String,
    /// Source
    pub source: String,
    /// Bio
    pub bio: String,
    /// Follower count
    pub follower_count: String,
    /// Portrait
    pub portrait: String,
    /// Type
    pub type_field: String,
    /// Nick name
    pub nick_name: String,
    /// Related content
    pub related_content: Vec<String>,
    /// Username
    pub username: String,
}

/// Request for candles data (used internally for futures-spot gap calculation)
/// API: GET /priapi/v5/market/candles or /priapi/v5/market/index-candles
#[derive(Debug, Clone, PartialEq)]
pub struct BigDataCandlesReq {
    /// Instrument ID (e.g. "BTC-USDT-SWAP" or "BTC-USD")
    pub inst_id: String,
    /// Bar size: "1m", "3m", "5m", "15m", "30m", "1H", "2H", "4H", "6H", "12H", "1D", "1W", "1M"
    pub bar: String,
    /// Pagination: request data before this timestamp (older data)
    pub after: Option<String>,
    /// Pagination: request data after this timestamp (newer data)
    pub before: Option<String>,
    /// Limit: max 300, default 100
    pub limit: Option<String>,
}

/// Per-instance state for a single chart. Carries everything Native needs to
/// render one chart: which periods are selectable, which is currently selected,
/// the loading / failed flags, the latest payload, and a monotonic `version`
/// that Native watches to know when `data` changed.
#[derive(Debug, Clone, PartialEq)]
pub struct BigDataChartState {
    /// The chart this interactor instance is bound to.
    pub chart_id: BigDataChartId,
    /// Current instrument id (e.g. "BTC-USDT-SWAP"). Set at runtime via
    /// [`BigDataChartActions::configure`]; empty until then. Stored in state (not
    /// a body field, since body fields are construction-time-immutable context).
    pub inst_id: String,
    /// Index instrument id (e.g. "BTC-USD"), needed for FuturesSpotGap requests.
    /// Set at runtime via [`BigDataChartActions::configure`]; empty until then.
    pub index_inst_id: String,
    /// Currently selected period. `None` for charts without a period selector
    /// (e.g. `FundingRate`).
    pub current_period: Option<BigDataChartPeriod>,
    /// Selectable periods for this chart, in display order. Empty for charts
    /// without a period selector.
    pub periods: Vec<BigDataChartPeriod>,
    /// Whether a load is in flight.
    pub loading: bool,
    /// Whether the last load failed.
    pub failed: bool,
    /// Monotonic version bumped whenever the chart payload changes; Native
    /// watches it and pulls the payload via a getter.
    /// 
    /// NOTE: the actual `BigDataChartResult` payload is intentionally kept
    /// **out** of this `#[interactor::state]` struct — see the module-level
    /// "UniFFI / macro-framework compatibility" note. The payload lives behind
    /// a cache body field pulled via `chart_data()`; `version` is the change
    /// signal.
    pub version: u64,
}

/// Instrument display metadata, resolved once per
/// [`BigDataPageActions::configure`] so no sub-chart looks up the instrument.
#[derive(Debug, Clone, PartialEq)]
pub struct BigDataInstMetaVo {
    /// Inverse contracts may return settle_ccy.
    pub trade_symbol: String,
    pub display_trade_symbol: String,
    pub quote_symbol: String,
    pub display_quote_symbol: String,
    /// Empty when the biz cache had no entry.
    pub inst_family: String,
    /// Only meaningful for SWAP / FUTURES / OPTION.
    pub is_positive_contract: bool,
    /// Chart price scale, or [`PRICE_DIGIT_UNKNOWN`].
    pub price_digit: i32,
    /// `"0"` coin / `"1"` contract / else quote-denominated. The label stays
    /// native — the contract case needs a localized string resource.
    pub trade_unit: String,
    /// Futures-basis index instrument: the underlying for SWAP / FUTURES.
    pub index_inst_id: String,
    /// Cross-asset perpetual. Already folded into `visible_charts`; exposed
    /// because the funding-rate chart also gates its fetch on it.
    pub is_x_perps: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BigDataPageState {
    pub inst_id: String,
    pub inst_type: String,
    pub biz_line: BigDataBizLine,
    /// Charts to show, in display order.
    pub visible_charts: Vec<BigDataChartId>,
    pub inst_meta: BigDataInstMetaVo,
    /// Abbreviated volume, rounding DOWN. `value` is left empty — native fills
    /// it per data point via `copy`.
    pub vol_recipe: KlineFormatRecipe,
    /// Abbreviated volume, rounding HALF_UP (capital-flow net volume).
    pub vol_half_up_recipe: KlineFormatRecipe,
    /// Fixed-scale price at instrument precision. Empty when `price_digit` is
    /// [`PRICE_DIGIT_UNKNOWN`], so native applies locale grouping only.
    pub price_recipe: KlineFormatRecipe,
}

/// Big event data object (original data from API)
#[derive(Debug, Clone, PartialEq)]
pub struct BigEventPo {
    /// Instrument ID (e.g., "BTC-USDT")
    pub inst_id: String,
    /// Major event type
    pub event_type: String,
    /// Event timestamp in milliseconds (13-digit)
    pub time_stamp: i64,
    /// Event tag (risk for unfavorable, highlight for favorable)
    pub tag: Option<String>,
    /// Event detail content
    pub event_detail: String,
    /// Event type title
    pub type_title: String,
    /// Source count (only for content type)
    pub source_count: Option<String>,
    /// AI-generated explanation
    pub explanation: Option<String>,
    /// AI interpretation disclaimer
    pub statement: Option<String>,
    /// Summary content ID (for navigation)
    pub summary_content_id: Option<String>,
    /// News title
    pub new_title: String,
    /// News content
    pub new_content: String,
    /// Address for enhanced event description
    pub address: String,
    /// List of author IDs
    pub author_list: Option<Vec<String>>,
}

/// Market depth data (order book)
/// 
/// Aligned with Kotlin BizDepthData
#[derive(Debug, Clone, PartialEq)]
pub struct BizDepth {
    /// Ask orders (sell side, ascending price)
    pub asks: Vec<BizDepthItem>,
    /// Bid orders (buy side, descending price)
    pub bids: Vec<BizDepthItem>,
    /// Timestamp in milliseconds
    pub ts: String,
    /// Action type: "snapshot" or "update"
    pub action: Option<String>,
}

/// Single depth level item (price and amount at that level)
/// 
/// Aligned with Kotlin DepthItem
#[derive(Debug, Clone, PartialEq)]
pub struct BizDepthItem {
    /// Price level
    pub price: String,
    /// Amount at this price level
    pub amount: String,
}

/// Single candle (K-line) data item
/// 
/// Note: Different channels return different fields:
/// - `candle*`: 9 fields (ts, o, h, l, c, vol, volCcy, volCcyQuote, confirm)
/// - `index-candle*` / `mark-price-candle*`: 6 fields (ts, o, h, l, c, confirm)
#[derive(Debug, Clone, PartialEq)]
pub struct BizKLineData {
    /// Timestamp in milliseconds
    pub ts: String,
    /// Open price
    pub o: String,
    /// High price
    pub h: String,
    /// Low price
    pub l: String,
    /// Close price
    pub c: String,
    /// Volume (coin) - only available for `candle*` channel
    pub vol: String,
    /// Volume (currency) - only available for `candle*` channel
    pub vol_ccy: String,
    /// Volume (quote currency) - only available for `candle*` channel
    pub vol_ccy_quote: String,
    /// Confirm status (0: unconfirmed, 1: confirmed)
    pub confirm: Option<String>,
}

/// Business model for a trade fill
#[derive(Debug, Clone, PartialEq)]
pub struct BizLatestDeal {
    /// Instrument ID
    pub inst_id: Option<String>,
    /// First trade ID in aggregation
    pub f_id: Option<String>,
    /// Last trade ID in aggregation
    pub l_id: Option<String>,
    /// Trade ID (for non-aggregated trades, optional)
    pub trade_id: Option<String>,
    /// Price
    pub px: String,
    /// Size
    pub sz: String,
    /// Side (buy/sell)
    pub side: String,
    /// Timestamp in milliseconds
    pub ts: String,
}

/// Buy/Sell bean
#[derive(Debug, Clone, PartialEq)]
pub struct BuySellBean {
    /// Timestamp
    pub ts: String,
    /// Sell volume
    pub sell_vol: String,
    /// Buy volume
    pub buy_vol: String,
}

/// Buy-sell marker data point
/// 
/// Represents a single data point in the buy-sell marker time series.
/// 
/// # Fields
/// 
/// - `time_stamp`: Unix timestamp in milliseconds (as string)
/// - `buy_price`: Total buy price at this timestamp (as string)
/// - `buy_count`: Total buy count at this timestamp (as string)
/// - `sell_price`: Total sell price at this timestamp (as string)
/// - `sell_count`: Total sell count at this timestamp (as string)
/// 
/// # Why String Types?
/// 
/// All fields use `String` instead of numeric types to:
/// - Avoid floating-point precision issues
/// - Support arbitrarily large numbers
/// - Maintain exact values from the API
/// - Ensure consistent serialization across platforms
/// 
/// # Example
/// 
/// ```ignore
/// let point = BuySellMarkerPoint {
/// time_stamp: "1704067200000".to_string(),
/// buy_price: "50000.5".to_string(),
/// buy_count: "100".to_string(),
/// sell_price: "49999.5".to_string(),
/// sell_count: "95".to_string(),
/// };
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct BuySellMarkerPoint {
    /// Unix timestamp in milliseconds
    /// 
    /// Represents the time when this data point was recorded.
    /// Format: String representation of milliseconds since Unix epoch.
    /// 
    /// Example: "1704067200000" = 2024-01-01 00:00:00 UTC
    pub time_stamp: String,
    /// Total buy price
    /// 
    /// The aggregated buy price at this timestamp.
    /// Format: String representation of the price value.
    pub buy_price: String,
    /// Total buy count
    /// 
    /// The number of buy orders at this timestamp.
    /// Format: String representation of the count.
    pub buy_count: String,
    /// Total sell price
    /// 
    /// The aggregated sell price at this timestamp.
    /// Format: String representation of the price value.
    pub sell_price: String,
    /// Total sell count
    /// 
    /// The number of sell orders at this timestamp.
    /// Format: String representation of the count.
    pub sell_count: String,
}

/// Buy-sell marker request parameters
/// 
/// This struct represents the input parameters for fetching buy-sell marker data.
/// 
/// # Fields
/// 
/// - `inst_id`: Instrument ID (e.g., "BTC-USDT")
/// - `bar`: Bar type (e.g., "1m", "5m", "1H")
/// - `type_time`: Time type
/// - `after`: Pagination - data after this timestamp
/// - `inst_type`: Instrument type (e.g., "SPOT", "SWAP")
/// - `ord_type`: Order type (e.g., "limit", "market", "post_only", "fok", "ioc", "optimal_limit_ioc")
/// - `start`: Start timestamp for candle data
/// - `end`: End timestamp for candle data
/// - `state`: State filter (e.g., "filled")
/// - `limit`: Maximum number of results to return
/// - `cache_expire_seconds`: HTTP cache expiration time in seconds (0 = no cache)
/// 
/// # Example
/// 
/// ```ignore
/// let req = BuySellMarkerReq {
/// inst_id: "BTC-USDT".to_string(),
/// bar: Some("1m".to_string()),
/// type_time: None,
/// after: None,
/// inst_type: "SPOT".to_string(),
/// ord_type: "limit,market,post_only,fok,ioc,optimal_limit_ioc".to_string(),
/// start: "1704067200000".to_string(),
/// end: "1704153600000".to_string(),
/// state: "filled".to_string(),
/// limit: Some("100".to_string()),
/// cache_expire_seconds: 60,
/// };
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct BuySellMarkerReq {
    /// Instrument ID
    /// 
    /// Format: `{BASE}-{QUOTE}`
    /// 
    /// Examples:
    /// - "BTC-USDT" - Bitcoin to Tether
    /// - "ETH-USDT" - Ethereum to Tether
    pub inst_id: String,
    /// Bar type (optional)
    /// 
    /// Examples: "1m", "5m", "15m", "1H", "4H", "1D"
    pub bar: Option<String>,
    /// Time type (optional)
    pub type_time: Option<String>,
    /// Pagination - data after this timestamp (optional)
    pub after: Option<String>,
    /// Instrument type
    /// 
    /// Examples: "SPOT", "SWAP", "FUTURES", "OPTION"
    pub inst_type: String,
    /// Order type
    /// 
    /// Comma-separated list of order types
    /// Examples: "limit,market,post_only,fok,ioc,optimal_limit_ioc"
    /// 
    /// Default: "limit,market,post_only,fok,ioc,optimal_limit_ioc"
    pub ord_type: String,
    /// Start timestamp
    /// 
    /// Unix timestamp in milliseconds (as string)
    pub start: String,
    /// End timestamp
    /// 
    /// Unix timestamp in milliseconds (as string)
    pub end: String,
    /// State filter
    /// 
    /// Examples: "filled", "canceled", "live"
    /// 
    /// Default: "filled"
    pub state: String,
    /// Maximum number of results to return
    /// 
    /// - `0`: No limit (return all results)
    /// - `> 0`: Return at most this many results
    /// 
    /// Default: 0 (no limit)
    pub limit: u64,
    /// HTTP cache expiration time in seconds
    /// 
    /// Controls how long the HTTP response should be cached:
    /// - `0`: No caching (always fetch fresh data from server)
    /// - `> 0`: Cache the response for the specified number of seconds
    /// 
    /// Default: 0 (no cache)
    pub cache_expire_seconds: u64,
    /// Bot strategy id, used to request buy/sell markers for a specific bot
    /// (grid/DCA) strategy. Optional.
    /// 
    /// When present and non-empty, it is appended to the order query so the
    /// backend scopes the buy/sell marker results to this bot strategy.
    /// A nil or empty value omits the parameter entirely.
    pub sid: Option<String>,
}

/// Buy-sell marker result
/// 
/// The response structure containing buy-sell marker data.
/// 
/// # Fields
/// 
/// - `data`: Vector of buy-sell marker data points, ordered by timestamp
/// - `order_info_array`: Array of order data arrays, each containing 5 strings
/// 
/// # Usage
/// 
/// The `data` field contains the parsed and validated data points, ready for use.
/// 
/// The `order_info_array` provides the raw order data in array format.
/// Each element is an array of 5 strings: [time_stamp, buy_price, buy_count, sell_price, sell_count]
/// 
/// # Example
/// 
/// ```ignore
/// let result = get_buy_sell_marker(req).await?;
/// 
/// // Use parsed data
/// for point in result.data {
/// println!("Time: {}, Buy: {}/{}, Sell: {}/{}",
/// point.time_stamp,
/// point.buy_price, point.buy_count,
/// point.sell_price, point.sell_count
/// );
/// }
/// 
/// // Or iterate through order info array
/// for order_data in result.order_info_array {
/// println!("Time: {}, Buy: {}/{}, Sell: {}/{}",
/// order_data[0], order_data[1], order_data[2], order_data[3], order_data[4]
/// );
/// }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct BuySellMarkerResult {
    /// List of buy-sell marker data points
    /// 
    /// Ordered by timestamp (ascending). May be empty if no data is available.
    pub data: Vec<BuySellMarkerPoint>,
    /// Array of order data arrays
    /// 
    /// Each element is an array of 5 strings: [time_stamp, buy_price, buy_count, sell_price, sell_count]
    /// 
    /// This provides sequential access to order data in the same order as the `data` field.
    /// 
    /// # Example
    /// ```ignore
    /// // Swift
    /// for orderData in result.orderInfoArray {
    /// print("Time: \(orderData[0]), Buy: \(orderData[1])/\(orderData[2]), Sell: \(orderData[3])/\(orderData[4])")
    /// }
    /// 
    /// // Kotlin
    /// result.orderInfoArray.forEach { orderData ->
    /// println("Time: ${orderData[0]}, Buy: ${orderData[1]}/${orderData[2]}, Sell: ${orderData[3]}/${orderData[4]}")
    /// }
    /// ```
    pub order_info_array: Vec<Vec<String>>,
}

/// Display-ready point. Prices already through `format_quote_price`
/// (precision + inverse/option quote-coin remap). Native adds thousands.
#[derive(Debug, Clone, PartialEq)]
pub struct BuySellPointDisplay {
    /// Unix ms timestamp; `0` when upstream value is unparseable.
    pub time_ms: i64,
    pub buy_price: String,
    pub buy_count: String,
    pub sell_price: String,
    pub sell_count: String,
}

/// Buy & sell volume data
#[derive(Debug, Clone, PartialEq)]
pub struct BuySellVolumeData {
    /// Contract data
    pub cont: Vec<BuySellBean>,
    /// Crypto data
    pub crypto: Vec<BuySellBean>,
    /// USDS data
    pub usds: Vec<BuySellBean>,
}

/// Request for buy & sell volume data (Swap/Futures)
/// API: GET /priapi/v5/rubik/stat/contracts/{instName}/taker-volume
#[derive(Debug, Clone, PartialEq)]
pub struct BuySellVolumeReq {
    /// Instrument ID (e.g. "BTC-USDT-SWAP")
    pub inst_id: String,
    /// Period: "5m", "15m", "1H", "4H", "1D"
    pub period: String,
    /// Unit: "crypto" (coin), "cont" (contract), "usds" (USDT)
    pub unit: String,
}

/// One `data[*]` entry: the corporate-action event package for a single instId.
/// 
/// The single-query backend contract guarantees `data` has length 1; the batch
/// query returns one of these per request element, one-to-one and same-order.
/// `events` is sorted by `effectiveTime` descending (new → old) and is empty
/// when there are no matching events.
#[derive(Debug, Clone, PartialEq)]
pub struct CandleAdjustmentData {
    /// Instrument id, echoed from the request.
    /// 
    /// The batch endpoint may echo this as `null` when the requested instId was
    /// malformed (per the multi-candle-adjustments contract: invalid items are
    /// returned with empty events, not a batch failure). A null is tolerated
    /// here and mapped to an empty string — without this a single null element
    /// would fail the whole `Vec<CandleAdjustmentData>` deserialization and lose
    /// every valid result in the batch. Native correlates results by index, so
    /// an empty instId on a degraded item is acceptable.
    pub inst_id: String,
    /// Corporate-action events for this instId (descending by effectiveTime).
    pub events: Vec<CandleAdjustmentEvent>,
}

/// One corporate-action event row (`data[*].events[*]`).
/// 
/// All numeric fields are kept as the raw backend strings to preserve precision
/// across the FFI boundary (native parses `effectiveTime` to ms via integer,
/// never double). `preEventPrice` may be `null`.
#[derive(Debug, Clone, PartialEq)]
pub struct CandleAdjustmentEvent {
    /// SPLIT / CASH_DIVIDEND / STOCK_DIVIDEND / COMBINATION / ...
    pub event_type: Option<String>,
    /// Effective time in milliseconds (backend string).
    pub effective_time: Option<String>,
    /// Share multiplier (split ratio).
    pub share_multiplier: Option<String>,
    /// Price factor.
    pub price_factor: Option<String>,
    /// Cash dividend per share.
    pub cash_per_share: Option<String>,
    /// Pre-event price (may be null).
    pub pre_event_price: Option<String>,
}

/// One adjustment record carrying the cumulative factors after the event.
/// 
/// `cumulative_volume_factor` is the one that shifts `estimatedTotalShares`
/// and therefore the implied valuation on the K-line header.
#[derive(Debug, Clone, PartialEq)]
pub struct CandleAdjustmentItem {
    /// Instrument id, e.g. "AAPL-USDT-SWAP".
    pub inst_id: String,
    /// Cumulative price adjustment factor (kept as raw string to preserve
    /// precision across FFI).
    pub cumulative_price_factor: Option<String>,
    /// Cumulative volume adjustment factor — drives the new
    /// `estimatedTotalShares`. Kept as a string to preserve precision.
    pub cumulative_volume_factor: Option<String>,
    /// Latest event that produced these factors.
    pub last_updated_event: Option<LastUpdatedEvent>,
}

/// Query parameters native passes to the HTTP candle-adjustments actions.
/// 
/// Shared by both the single (`start_request`) and batch (`start_multi_request`)
/// flows: the batch action takes a `Vec<CandleAdjustmentQuery>`, one per instId,
/// matching the endpoint's per-element params. Only `inst_id` is required.
#[derive(Debug, Clone, PartialEq)]
pub struct CandleAdjustmentQuery {
    /// Instrument id, e.g. "AAPL-USDT-SWAP".
    pub inst_id: String,
    /// effectiveTime lower bound, ms (inclusive). `None` = backend default.
    pub begin: Option<i64>,
    /// effectiveTime upper bound, ms (exclusive). `None` = backend default.
    pub end: Option<i64>,
    /// Optional event-type filter: SPLIT / CASH_DIVIDEND / STOCK_DIVIDEND / COMBINATION.
    pub event_type: Option<String>,
    /// Max rows to return (backend clamps to [1, 100]). `None` = backend default (1).
    pub limit: Option<i32>,
    /// Rows to skip (pagination). `None` = backend default (0).
    pub offset: Option<i32>,
}

/// Tiny state — only tracks whether the global subscription is active. Heavy
/// data flows through the UI event.
#[derive(Debug, Clone, PartialEq)]
pub struct CandleAdjustmentState {
    pub is_subscribed: bool,
}

/// Single candle data point
#[derive(Debug, Clone, PartialEq)]
pub struct CandleData {
    /// Timestamp
    pub ts: String,
    /// Open price
    pub o: String,
    /// High price
    pub h: String,
    /// Low price
    pub l: String,
    /// Close price
    pub c: String,
    /// Confirm status
    pub comfirm: String,
}

/// Request parameters for candles initial time
#[derive(Debug, Clone, PartialEq)]
pub struct CandlesInitialTimeReq {
    /// K-line type (latest/index/mark-price)
    pub kline_type: KlineType,
    /// Instrument ID (e.g., "BTC-USDT", "BTC-USDT-SWAP")
    pub inst_id: String,
}

/// Request parameters for candles data
#[derive(Debug, Clone, PartialEq)]
pub struct CandlesReq {
    /// K-line type (latest/index/mark-price)
    pub kline_type: KlineType,
    /// Instrument ID (e.g., "BTC-USDT", "BTC-USDT-SWAP")
    pub inst_id: String,
    /// Bar/timeframe (e.g., "1m", "5m", "15m", "1H", "1D")
    /// Extracted from candle string like "candle15m" -> "15m"
    pub bar: String,
    /// Timestamp after which to fetch data (for pagination)
    pub after: Option<String>,
    /// Limit number of results (default: 100, max: 300)
    pub limit: Option<String>,
    /// Stock price adjustment mode (e.g., "forward", "none").
    /// Sent as `adjust` query param to the HTTP seed request.
    /// Only honored for `KlineType::Latest`; ignored for Index/MarkPrice
    /// since their endpoints do not accept this parameter.
    pub adjustment: Option<String>,
    /// Index instrumentName for X-perp pre-launch backfill (e.g., "BTC-USD").
    /// When set, Latest/MarkPrice HTTP seed requests route to the mixed
    /// endpoints that splice index candles before the first real candle;
    /// ignored for Index. `#[uniffi(default = None)]` keeps existing native
    /// constructors of this Record source-compatible.
    pub index_name: Option<String>, // default = None
}

/// Capital flow data
#[derive(Debug, Clone, PartialEq)]
pub struct CapitalFlowData {
    /// Timestamp
    pub ts: String,
    /// Net volume (net inflow/outflow)
    pub net_vol: String,
    /// Net volume title (optional, may not be returned by backend)
    pub net_vol_title: Option<String>,
    /// Close price
    pub close_price: String,
}

/// Request for capital flow data (Spot)
/// API: GET /priapi/v5/rubik/stat/spot/{instName}/taker-trend
#[derive(Debug, Clone, PartialEq)]
pub struct CapitalFlowReq {
    /// Instrument ID (e.g. "BTC-USDT")
    pub inst_id: String,
    /// Period: "1D", "1W", "1M"
    pub period: String,
}

/// CeDeFi buy-sell marker data point
/// 
/// Represents a single data point in the CeDeFi buy-sell marker time series.
/// Contains aggregated buy and sell order information for a specific time period.
/// 
/// Compared to CeFi `BuySellMarkerPoint`, this struct has 5 additional fields:
/// `buy_amount`, `buy_value`, `sell_amount`, `sell_value`, `from_address_tag`.
/// 
/// # Why String Types?
/// 
/// All fields use `String` instead of numeric types to:
/// - Avoid floating-point precision issues
/// - Support arbitrarily large numbers
/// - Maintain exact values from the API
/// - Ensure consistent serialization across platforms
#[derive(Debug, Clone, PartialEq)]
pub struct CedefiBsMarkerPoint {
    /// Unix timestamp in milliseconds (as string)
    /// 
    /// Represents the K-line bar time when this data point was recorded.
    pub time: String,
    /// Aggregated buy price (as string)
    pub buy_price: String,
    /// Number of buy orders (as string)
    pub buy_count: String,
    /// Aggregated sell price (as string)
    pub sell_price: String,
    /// Number of sell orders (as string)
    pub sell_count: String,
    /// Buy amount (as string) - CeDeFi specific
    pub buy_amount: String,
    /// Buy value (as string) - CeDeFi specific
    pub buy_value: String,
    /// Sell amount (as string) - CeDeFi specific
    pub sell_amount: String,
    /// Sell value (as string) - CeDeFi specific
    pub sell_value: String,
    /// Wallet address tag/label (as string) - CeDeFi specific
    pub from_address_tag: String,
}

/// CeDeFi buy-sell marker request parameters
/// 
/// This struct represents the input parameters for fetching CeDeFi buy-sell marker data.
/// 
/// # Fields
/// 
/// - `chain_id`: Chain ID (e.g., "1" for Ethereum mainnet)
/// - `token_address`: Token contract address
/// - `from_address`: Wallet address (optional, mutually exclusive with `from_address_tag`)
/// - `from_address_tag`: Wallet address tag/label (optional, mutually exclusive with `from_address`)
/// - `bar`: K-line bar period (e.g., "1m", "5m", "1H")
#[derive(Debug, Clone, PartialEq)]
pub struct CedefiBsMarkerReq {
    /// Chain ID (e.g., "1" for Ethereum, "56" for BSC)
    pub chain_id: String,
    /// Token contract address
    pub token_address: String,
    /// Wallet address (optional, used for address-based query)
    /// 
    /// Mutually exclusive with `from_address_tag`.
    /// When both are empty, uses the current user's wallet address.
    pub from_address: Option<String>,
    /// Wallet address tag/label (optional, used for tag-based query)
    /// 
    /// Mutually exclusive with `from_address`.
    /// When set, queries by tag instead of address, and uses the tag endpoint.
    pub from_address_tag: Option<String>,
    /// K-line bar period
    /// 
    /// Examples: "1m", "5m", "15m", "1H", "4H", "1D"
    pub bar: String,
}

/// CeDeFi buy-sell marker result
/// 
/// The response structure containing CeDeFi buy-sell marker data.
#[derive(Debug, Clone, PartialEq)]
pub struct CedefiBsMarkerResult {
    /// List of buy-sell marker data points, ordered by timestamp
    pub data: Vec<CedefiBsMarkerPoint>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChartSettingsVo {
    pub show_y_axis: bool,
    pub show_countdown: bool,
    pub show_price_change: bool,
    pub show_quick_order: bool,
    pub show_price_warning: bool,
    pub show_history_order: bool,
    pub show_current_order: bool,
    pub show_cost_line: bool,
    pub show_cost_be: bool,
    pub show_cost_liq: bool,
    pub show_spot_avg: bool,
    pub show_economic_events: bool,
    pub is_forward_adjusted: bool,
}

/// Content data item from coin news endpoint
/// 
/// Represents a single content item in the contentDataList array.
/// 
/// # Fields (based on API response)
/// 
/// All fields use `String` to handle null values from backend (converted to empty strings).
#[derive(Debug, Clone, PartialEq)]
pub struct CoinNewsDataItem {
    /// View count
    pub view_count: String,
    /// Source platform
    pub source_platform: String,
    /// English title
    pub en_title: String,
    /// Author information
    pub author: AuthorInfo,
    /// Translated title
    pub translated_title: String,
    /// Important flag
    pub important: String,
    /// Like count
    pub like_count: String,
    /// Mention detail
    pub mention_detail: MentionDetail,
    /// Title
    pub title: String,
    /// Content ID
    pub content_id: String,
    /// Format type
    pub format_type: String,
    /// Is liked flag
    pub is_liked: String,
    /// Flash type
    pub flash_type: String,
    /// Publish time
    pub publish_time: String,
    /// Share URL
    pub share_url: String,
    /// Tokens list
    pub tokens: Vec<String>,
    /// Translated content
    pub translated_content: String,
    /// Type
    pub type_field: String,
    /// Content
    pub content: String,
}

/// Coin news request parameters
/// 
/// This struct represents the input parameters for fetching coin news data.
/// It supports both CeFi and CeDeFi use cases.
/// 
/// # Fields
/// 
/// ## CeFi Fields
/// - `token`: Token for pagination or authentication
/// - `group`: Group filter
/// - `important`: Importance filter ("true", "false", "1", "0")
/// 
/// ## CeDeFi Fields
/// - `contract_address`: Smart contract address for DeFi tokens
/// - `chain_id`: Blockchain chain identifier (e.g., "1" for Ethereum mainnet)
/// - `token_symbol`: Token symbol for DeFi assets
/// 
/// ## Common Fields
/// - `size`: Number of items to return per page (default: 20)
/// - `url`: API endpoint URL for the request
/// - `cache_expire_seconds`: HTTP cache expiration time in seconds (0 = no cache)
/// 
/// # Example
/// 
/// ```ignore
/// // CeFi example
/// let req = CoinNewsReq {
/// size: 20,
/// token: "pagination_token".to_string(),
/// group: "crypto".to_string(),
/// important: "true".to_string(),
/// contract_address: String::new(),
/// chain_id: String::new(),
/// token_symbol: String::new(),
/// url: "/priapi/v5/content/public/flash/1".to_string(),
/// cache_expire_seconds: 60,
/// };
/// 
/// // CeDeFi example
/// let req = CoinNewsReq {
/// size: 20,
/// token: String::new(),
/// group: String::new(),
/// important: String::new(),
/// contract_address: "0x1234...".to_string(),
/// chain_id: "1".to_string(),
/// token_symbol: "USDT".to_string(),
/// url: "/priapi/v5/rubik/content/dex/flash/detail/1".to_string(),
/// cache_expire_seconds: 60,
/// };
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct CoinNewsReq {
    /// Number of items to return per page
    /// 
    /// Controls the maximum number of coin news items returned in the response.
    /// 
    /// Default: 20
    pub size: u64,
    /// API endpoint URL
    /// 
    /// The base URL path for the API request.
    /// 
    /// Examples:
    /// - CeFi: "/priapi/v5/content/public/flash/1"
    /// - CeDeFi: "/priapi/v5/rubik/content/dex/flash/detail/1"
    /// 
    /// Default: empty string
    pub url: String,
    /// HTTP cache expiration time in seconds
    /// 
    /// Controls how long the HTTP response should be cached:
    /// - `0`: No caching (always fetch fresh data from server)
    /// - `> 0`: Cache the response for the specified number of seconds
    /// 
    /// Default: 0 (no cache)
    pub cache_expire_seconds: u64,
    /// Token for pagination or authentication (CeFi)
    /// 
    /// Used for pagination in CeFi coin news requests.
    /// 
    /// Default: empty string
    pub token: String,
    /// Group filter (CeFi)
    /// 
    /// Filter news by specific group or category.
    /// 
    /// Default: empty string (no filter)
    pub group: String,
    /// Importance filter (CeFi)
    /// 
    /// Valid values: "true", "false", "1", "0", or empty string
    /// 
    /// Default: "true" (show important news)
    pub important: String,
    /// Smart contract address (CeDeFi)
    /// 
    /// The blockchain contract address for DeFi tokens.
    /// 
    /// Examples: "0x1234567890abcdef...", "So11111111111111111111111111111111111111112"
    /// 
    /// Default: empty string (no filter)
    pub contract_address: String,
    /// Blockchain chain identifier (CeDeFi)
    /// 
    /// Identifies which blockchain network the token is on.
    /// 
    /// Examples: "1" (Ethereum), "56" (BSC), "137" (Polygon)
    /// 
    /// Default: empty string (no filter)
    pub chain_id: String,
    /// Token symbol (CeDeFi)
    /// 
    /// The token symbol or identifier for DeFi assets.
    /// 
    /// Examples: "USDT", "WETH", "DAI"
    /// 
    /// Default: empty string (no filter)
    pub token_symbol: String,
}

/// Coin news result
/// 
/// The response structure containing coin news data.
/// 
/// # Fields
/// 
/// - `content_data_list`: Vector of content data items
/// 
/// # Example
/// 
/// ```ignore
/// let result = get_coin_news(req).await?;
/// 
/// for item in result.content_data_list {
/// println!("Title: {}, View Count: {}", item.title, item.view_count);
/// }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct CoinNewsResult {
    /// List of content data items
    /// 
    /// May be empty if no data is available.
    pub content_data_list: Vec<CoinNewsDataItem>,
}

/// Compact formatting output: numeric body plus a unit-bucket tag.
#[derive(Debug, Clone, PartialEq)]
pub struct CompactValue {
    pub value: String,
    pub unit: KlineCompactUnit,
}

/// `availability.categories` — the five UI category switches; also the FFI
/// projection consumed by native (flat booleans, `false` = hide).
/// 
/// `serde(default)` + the flexible deserializer map absent / `null` / dirty
/// fields to `false`, so one malformed field degrades only itself.
#[derive(Debug, Clone, PartialEq)]
pub struct CompanyAvailabilityCategories {
    /// Company > Profile sub-tab.
    pub profile: bool,
    /// Company > Financials sub-tab.
    pub financials: bool,
    /// Company > Shareholders sub-tab.
    pub shareholders_stats: bool,
    /// Company > Dividends (corporate actions) sub-tab.
    pub dividend_splits: bool,
    /// Data > All-markets surface (not a Company sub-tab).
    pub stock_trading_data: bool,
    /// Company > ETF Holdings sub-tab (ETF instruments only).
    /// 
    /// The single switch that gates every ETF-Holdings module (M0–M5): when
    /// `true` native registers the ETF sub-tab, when `false` / absent it stays
    /// hidden. Independent of the other five booleans — an ETF ticker returning
    /// `financials=false` and `etfHoldings=true` is valid; there is no
    /// mutual-exclusion coupling on either the Rust or the native side.
    pub etf_holdings: bool,
}

/// A validated calendar date. Native renders these components with its locale-aware date APIs.
#[derive(Debug, Clone, PartialEq)]
pub struct CompanyCalendarDate {
    pub year: i32,
    pub month: u8,
    pub day: u8,
}

/// The fixed-shape Chart-tab Company data panel.
#[derive(Debug, Clone, PartialEq)]
pub struct CompanyDataPanelPresentation {
    pub metrics: Vec<CompanyMetricPresentation>,
    /// Normalized reporting currency used by the panel title. Empty means no currency suffix.
    pub currency_code: String,
}

/// A validated calendar date. Native renders these components with its locale-aware date APIs.
#[derive(Debug, Clone, PartialEq)]
pub struct CompanyDividendCalendarDate {
    pub year: i32,
    pub month: u8,
    pub day: u8,
}

/// One validated, oldest-to-newest dividend history point.
#[derive(Debug, Clone, PartialEq)]
pub struct CompanyDividendHistoryPresentation {
    pub year: i32,
    /// Canonical decimal text for exact transport; absent when the wire value is invalid.
    /// Native parses only at the final chart boundary.
    pub amount: Option<String>,
    /// Raw decimal ratio for chart geometry (`0.0256` means 2.56%).
    pub yield_ratio: Option<f64>,
    pub amount_recipe: KlineFormatRecipe,
    pub yield_recipe: KlineFormatRecipe,
}

/// The latest selected stock split. Both ratio sides are valid whenever this record exists.
#[derive(Debug, Clone, PartialEq)]
pub struct CompanyDividendSplitPresentation {
    pub from: KlineFormatRecipe,
    pub to: KlineFormatRecipe,
    pub ex_date: Option<CompanyDividendCalendarDate>,
    /// Known values are `stock_split`, `reverse_split`, and `unknown`.
    pub kind: String,
}

/// Tiny state — the calendar data flows through the UI event, so state only
/// tracks in-flight status.
#[derive(Debug, Clone, PartialEq)]
pub struct CompanyDividendState {
    pub is_loading: bool,
}

/// Complete dividend/split projection for the Company sub-tab.
/// 
/// Known `availability` values are:
/// - `available`: the backend supplied a dividends block.
/// - `non_paying`: the response is valid but omitted the dividends block; a split may still exist.
/// - `no_data`: neither dividend content nor a valid split is available.
#[derive(Debug, Clone, PartialEq)]
pub struct CompanyDividendsPresentation {
    pub availability: String,
    pub currency_code: String,
    pub launch_date: Option<CompanyDividendCalendarDate>,
    pub latest_ex_date: Option<CompanyDividendCalendarDate>,
    pub latest_amount: KlineFormatRecipe,
    pub div_yield_ttm: KlineFormatRecipe,
    pub payout_ratio_lfy: KlineFormatRecipe,
    /// Known values are `monthly`, `quarterly`, `semi_annual`, `annual`, and `unknown`.
    pub frequency_token: String,
    pub history: Vec<CompanyDividendHistoryPresentation>,
    pub split: Option<CompanyDividendSplitPresentation>,
}

/// Latest-earnings card after domain normalization.
#[derive(Debug, Clone, PartialEq)]
pub struct CompanyEarningsCardPresentation {
    /// `fiscal_period` when quarter/year are complete, otherwise `fallback`.
    pub title_kind: String,
    pub fiscal_quarter: Option<u8>,
    pub fiscal_year: Option<i32>,
    pub fallback_title: String,
    pub report_period_start: Option<CompanyFinancialCalendarDate>,
    pub report_period_end: Option<CompanyFinancialCalendarDate>,
    pub currency_code: String,
    pub fallback_currency_code: String,
    pub summary_text: String,
    pub analyst_count: String,
    pub rows: Vec<CompanyEarningsRowPresentation>,
}

/// One reported / forecast / variance cell in the latest-earnings card.
#[derive(Debug, Clone, PartialEq)]
pub struct CompanyEarningsCellPresentation {
    pub main: CompanyFinancialValuePresentation,
    pub sub_percent: CompanyFinancialValuePresentation,
}

/// One fixed earnings row. Known IDs are `revenue` and `eps`.
#[derive(Debug, Clone, PartialEq)]
pub struct CompanyEarningsRowPresentation {
    pub id: String,
    /// Backend-owned token (`BEAT`, `MISSED`, `MET`) or empty when unknown.
    pub beat_miss_token: String,
    pub reported: CompanyEarningsCellPresentation,
    pub forecast: CompanyEarningsCellPresentation,
    pub vs_estimate: CompanyEarningsCellPresentation,
}

/// Canonical business policy for Company tab and metrics entry.
#[derive(Debug, Clone, PartialEq)]
pub struct CompanyEntryPolicy {
    pub business_eligible: bool,
    /// Empty when eligible; otherwise one of the known extensible reason codes.
    pub unavailable_reason: String,
    pub subject: CompanySubject,
}

/// One point of an actual-vs-estimate trend series.
#[derive(Debug, Clone, PartialEq)]
pub struct CompanyEstimatePointPresentation {
    pub fiscal_year: Option<i32>,
    pub fiscal_quarter: Option<u8>,
    pub actual: CompanyFinancialValuePresentation,
    pub estimated: CompanyFinancialValuePresentation,
    pub is_forecast: bool,
}

/// Both consumers of the estimate endpoint.
#[derive(Debug, Clone, PartialEq)]
pub struct CompanyEstimatePresentation {
    pub earnings: CompanyEarningsCardPresentation,
    pub estimates: Vec<CompanyEstimateSeriesPresentation>,
}

/// One fixed trend series. Known IDs are `revenue` and `eps`.
#[derive(Debug, Clone, PartialEq)]
pub struct CompanyEstimateSeriesPresentation {
    pub id: String,
    pub points: Vec<CompanyEstimatePointPresentation>,
}

/// One allocation group (donut + legend).
#[derive(Debug, Clone, PartialEq)]
pub struct CompanyEtfAllocationGroupPresentation {
    /// `available` / `no_data`.
    pub status: String,
    pub slices: Vec<CompanyEtfAllocationSlicePresentation>,
}

/// One donut/legend slice, in the backend's authoritative order. Native supplies
/// the palette color and the localized "Other" label when `is_other`.
#[derive(Debug, Clone, PartialEq)]
pub struct CompanyEtfAllocationSlicePresentation {
    /// Stable backend code: ISO country (`US`), sector enum (`TECHNOLOGY`), or
    /// `OTHER` for the aggregated remainder (empty when a sector is unmapped).
    pub code: String,
    pub name: String,
    /// Weight as a `[0,1]` ratio for the donut sweep (rendered raw; sum may ≠ 1).
    pub ratio: f64,
    pub weight_recipe: KlineFormatRecipe,
    /// Backend-flagged aggregated slice (`code == OTHER`); never client-derived.
    pub is_other: bool,
}

/// M3 both allocation groups from one call.
#[derive(Debug, Clone, PartialEq)]
pub struct CompanyEtfAllocationsPresentation {
    pub country: CompanyEtfAllocationGroupPresentation,
    pub sector: CompanyEtfAllocationGroupPresentation,
}

/// One annual-return row (descending; current year marked YTD).
#[derive(Debug, Clone, PartialEq)]
pub struct CompanyEtfAnnualReturnPresentation {
    pub year: String,
    pub is_ytd: bool,
    pub has_value: bool,
    pub return_recipe: KlineFormatRecipe,
    /// `positive` / `negative` / `neutral`.
    pub return_sign: String,
}

/// One benchmark line. Client selects by `type_` (semantic enum) and renders
/// `name` + `series`; no hardcoded tickers, no client-composed name copy.
#[derive(Debug, Clone, PartialEq)]
pub struct CompanyEtfBenchmarkPresentation {
    /// Semantic type: `DOW_JONES` / `NASDAQ` / `SP500`.
    pub type_: String,
    pub name: String,
    /// True for the default-selected benchmark (`DOW_JONES`, else first item).
    pub is_default: bool,
    /// `false` when the backend did not provide the benchmark cumulative return.
    /// Clients must render this as unavailable instead of formatting the recipe's
    /// placeholder value as `0.00%`.
    pub return_has_value: bool,
    pub return_recipe: KlineFormatRecipe,
    /// `false` when the backend did not provide the benchmark annualized return.
    pub annualized_return_has_value: bool,
    pub annualized_return_recipe: KlineFormatRecipe,
    pub series: Vec<CompanyEtfPerformancePointPresentation>,
}

/// One constituent-change row. The preview card uses the lean columns
/// (name/symbol + share change); the detail page additionally renders the
/// prev→current shares / weight / market-value recipes. All numbers are
/// Rust-formatted recipes; native never computes.
#[derive(Debug, Clone, PartialEq)]
pub struct CompanyEtfConstituentChangeRowPresentation {
    pub name: String,
    pub symbol: String,
    pub instrument_id: String,
    /// Backend OKX instType of [`Self::instrument_id`] (e.g. `SWAP`); empty when unmatched.
    pub inst_type: String, // default = Default::default()
    /// Row clickable iff `is_tradable && !instrument_id.is_empty()`.
    pub is_clickable: bool,
    /// `ADDED` / `REMOVED` / `INCREASED` / `DECREASED` (drives icon/label).
    pub change_type: String,
    /// Signed share-change recipe (prepend_plus drives the `+` for increases).
    pub change_shares_recipe: KlineFormatRecipe,
    /// Share change percent (empty for `ADDED`).
    pub change_pct_recipe: KlineFormatRecipe,
    /// `positive` (increased) / `negative` (decreased) / `neutral` for the color.
    pub change_sign: String,
    /// Detail-page prev→current shares.
    pub previous_shares_recipe: KlineFormatRecipe,
    pub current_shares_recipe: KlineFormatRecipe,
    /// Detail-page prev→current weight percent + signed change.
    pub previous_weight_recipe: KlineFormatRecipe,
    pub current_weight_recipe: KlineFormatRecipe,
    pub change_weight_recipe: KlineFormatRecipe,
    /// Detail-page prev→current market value (USD) + signed change.
    pub previous_market_value_recipe: KlineFormatRecipe,
    pub current_market_value_recipe: KlineFormatRecipe,
    pub change_market_value_recipe: KlineFormatRecipe,
}

/// M5 constituent-changes surface: the full Rust-merged list across pages plus
/// the paging state. Native slices the top-3 for the preview card and consumes
/// the whole list on the detail page — it never concatenates pages itself.
/// 
/// `total` is the backend total (all displayable changes, not `rows.len()`);
/// `unknown_count` / `unchanged_count` are the backend's excluded-row counters
/// (matched-but-incomparable, and unchanged) shown as summary figures.
#[derive(Debug, Clone, PartialEq)]
pub struct CompanyEtfConstituentChangesPresentation {
    /// `ok` / `no_history` / `partial` / `fetch_error`.
    pub status: String,
    /// Raw current disclosure date for the "Data as of" subtitle (native formats).
    pub current_holding_date: String,
    /// Raw prior disclosure date (the compared-against period).
    pub previous_holding_date: String,
    /// Backend total of displayable changes (may be ≫ rows.len()).
    pub total: i64,
    /// Matched-but-incomparable count (excluded from rows).
    pub unknown_count: i64,
    /// Unchanged count (excluded from rows).
    pub unchanged_count: i64,
    /// Opaque continuation token; empty → no further pages (stop paging).
    pub next_cursor: String,
    /// Backend order preserved (client never re-sorts).
    pub rows: Vec<CompanyEtfConstituentChangeRowPresentation>,
}

/// One detail-table row (fixed name/symbol column + scrolling data columns).
#[derive(Debug, Clone, PartialEq)]
pub struct CompanyEtfHoldingRowPresentation {
    pub name: String,
    /// Real stock code (display); empty → show name only.
    pub symbol: String,
    /// In-app instrument id (navigation only).
    pub instrument_id: String,
    /// Backend OKX instType of [`Self::instrument_id`] (e.g. `SWAP`), passed verbatim to the
    /// market-detail page; empty when unmatched.
    pub inst_type: String, // default = Default::default()
    /// Row clickable iff `is_tradable && !instrument_id.is_empty()`.
    pub is_clickable: bool,
    pub weight_recipe: KlineFormatRecipe,
    pub shares_recipe: KlineFormatRecipe,
    pub market_value_recipe: KlineFormatRecipe,
    pub country: String,
    pub price_recipe: KlineFormatRecipe,
    pub change_pct_recipe: KlineFormatRecipe,
    /// `positive` / `negative` / `neutral` for the 24h change color.
    pub change_sign: String,
}

/// M2 one page of the holdings detail table.
#[derive(Debug, Clone, PartialEq)]
pub struct CompanyEtfHoldingsPagePresentation {
    pub status: String,
    pub holding_date: String,
    /// Actual loadable row count (backend `total`).
    pub total: i64,
    /// Top-8 weight sum for THIS snapshot (same `topWeightPct` as `/holdings/top`,
    /// so L2 shows the "Top holdings" total from the same source as the list;
    /// `no_data` → `"0.00"`). Independent of this page's sort/cursor.
    pub top_weight_recipe: KlineFormatRecipe,
    /// Opaque continuation token; empty → no further pages (stop paging).
    pub next_cursor: String,
    pub rows: Vec<CompanyEtfHoldingRowPresentation>,
}

/// One curve point. `cumulative_return` is a parsed `f64` for the chart (exact
/// text also available via the recipe for labels). `has_value == false` marks a
/// point whose `returnPct` was absent — never silently plotted as `0`.
#[derive(Debug, Clone, PartialEq)]
pub struct CompanyEtfPerformancePointPresentation {
    pub date: String,
    pub has_value: bool,
    pub cumulative_return: f64,
    pub return_recipe: KlineFormatRecipe,
}

/// M4 performance chart + stage/annual returns. The top-level fund returns
/// (`return_*`, `annualized_return_*`) render the header block; `*_has_value`
/// lets native distinguish a real `0.00%` from missing data (never coerced to 0).
#[derive(Debug, Clone, PartialEq)]
pub struct CompanyEtfPerformancePresentation {
    pub status: String,
    /// Echoed requested period (`1M`/`3M`/`6M`/`1Y`/`3Y`).
    pub period: String,
    /// Curve window start (raw date; native formats). Empty when absent.
    pub base_date: String,
    /// Curve window end (raw date; native formats). Empty when absent.
    pub end_date: String,
    /// Server-generation timestamp (raw string; native formats). Empty when absent.
    pub updated_at: String,
    /// Fund cumulative return for the period. `return_has_value == false` → no data
    /// (render placeholder, NOT `0.00%`).
    pub return_has_value: bool,
    pub return_recipe: KlineFormatRecipe,
    /// `positive` / `negative` / `neutral`.
    pub return_sign: String,
    /// Fund annualized return. `annualized_return_has_value == false` → no data.
    pub annualized_return_has_value: bool,
    pub annualized_return_recipe: KlineFormatRecipe,
    /// `positive` / `negative` / `neutral`.
    pub annualized_return_sign: String,
    /// ETF's own curve for the period.
    pub series: Vec<CompanyEtfPerformancePointPresentation>,
    pub benchmarks: Vec<CompanyEtfBenchmarkPresentation>,
    /// Fixed 5 (1M/3M/6M/YTD/1Y), independent of the curve period.
    pub stage_returns: Vec<CompanyEtfStageReturnPresentation>,
    pub annual_returns: Vec<CompanyEtfAnnualReturnPresentation>,
}

/// One fixed stage-return chip. `has_value == false` → render placeholder.
#[derive(Debug, Clone, PartialEq)]
pub struct CompanyEtfStageReturnPresentation {
    pub period: String,
    pub has_value: bool,
    pub return_recipe: KlineFormatRecipe,
    /// `positive` / `negative` / `neutral`.
    pub return_sign: String,
}

/// One Top-8 row: code + name + weight + relative bar length.
#[derive(Debug, Clone, PartialEq)]
pub struct CompanyEtfTopHoldingRowPresentation {
    /// Real stock code (display). Empty → name only.
    pub symbol: String,
    pub name: String,
    pub weight_recipe: KlineFormatRecipe,
    /// Relative bar width in `[0.0, 1.0]`: rank-1 = 1.0, others = wp / wp1.
    pub relative_ratio: f64,
}

/// M1 Top-8 holdings card.
#[derive(Debug, Clone, PartialEq)]
pub struct CompanyEtfTopHoldingsPresentation {
    /// `ok` / `stale` / `no_data` / `fetch_error` (normalized lower-snake).
    pub status: String,
    /// Raw disclosure date (native applies locale-aware `formatStandardDate`).
    pub holding_date: String,
    /// Sum of returned items' weight (no normalization); `no_data` → `"0.00"`.
    pub top_weight_recipe: KlineFormatRecipe,
    /// Current-snapshot valid row count (backend `total`, may be ≫ rows.len()).
    pub total: i64,
    /// Up to 8 rows, display order weightPct desc.
    pub rows: Vec<CompanyEtfTopHoldingRowPresentation>,
}

/// Typed failure policy shared by Company experience events.
/// 
/// `kind` remains string-backed so new failure classes can cross older bindings safely. Known
/// values are `not_prepared`, `no_data`, `retryable`, and `unknown`.
#[derive(Debug, Clone, PartialEq)]
pub struct CompanyExperienceFailure {
    pub kind: String,
    pub retryable: bool,
    pub error_message: String,
}

/// Lightweight observable state. Presentation payloads stay in pull-based caches.
#[derive(Debug, Clone, PartialEq)]
pub struct CompanyExperienceState {
    pub generation: u64,
    pub is_prepared: bool,
    pub profile_loading: bool,
    pub shareholders_loading: bool,
    pub financials_loading: bool,
    pub statement_loading: bool,
    pub dividends_loading: bool,
    pub profile_request_generation: u64,
    pub shareholders_request_generation: u64,
    pub financials_request_generation: u64,
    pub statement_request_generation: u64,
    pub dividends_request_generation: u64,
    pub profile_version: u64,
    pub shareholders_version: u64,
    pub financials_version: u64,
    pub statement_version: u64,
    pub dividends_version: u64,
    pub etf_top_loading: bool,
    pub etf_holdings_loading: bool,
    pub etf_allocations_loading: bool,
    pub etf_performance_loading: bool,
    pub etf_changes_loading: bool,
    pub etf_top_request_generation: u64,
    pub etf_holdings_request_generation: u64,
    pub etf_allocations_request_generation: u64,
    pub etf_performance_request_generation: u64,
    pub etf_changes_request_generation: u64,
    pub etf_top_version: u64,
    pub etf_holdings_version: u64,
    pub etf_allocations_version: u64,
    pub etf_performance_version: u64,
    pub etf_changes_version: u64,
    /// M2 pagination, owned by Rust: another page exists, the live cursor, and
    /// whether a load-more is in flight. Native reads these; it never paginates.
    pub etf_holdings_has_more: bool,
    pub etf_holdings_next_cursor: String,
    pub etf_holdings_is_loading_more: bool,
    /// M5 pagination, owned by Rust (same contract as M2).
    pub etf_changes_has_more: bool,
    pub etf_changes_next_cursor: String,
    pub etf_changes_is_loading_more: bool,
}

/// A validated calendar date. Native renders these components with its locale-aware date APIs.
#[derive(Debug, Clone, PartialEq)]
pub struct CompanyFinancialCalendarDate {
    pub year: i32,
    pub month: u8,
    pub day: u8,
}

/// One value plus the semantic renderer native should apply.
/// 
/// Known `value_kind` values are `money`, `number`, `percent`, and `shares`.
#[derive(Debug, Clone, PartialEq)]
pub struct CompanyFinancialValuePresentation {
    pub value_kind: String,
    /// Canonical, unscaled decimal value for charts and calculations. Empty means no value.
    pub raw_value: String,
    pub recipe: KlineFormatRecipe,
    /// Known values are `positive`, `negative`, and `neutral`.
    pub sign: String,
}

/// One partial-success initial financials load.
#[derive(Debug, Clone, PartialEq)]
pub struct CompanyFinancialsPresentation {
    pub estimate: Option<CompanyEstimatePresentation>,
    pub income: Option<CompanyStatementPresentation>,
    pub balance: Option<CompanyStatementPresentation>,
    pub cashflow: Option<CompanyStatementPresentation>,
}

/// Tiny state — the data flows through the UI events, so state only tracks the
/// initial load's in-flight status.
#[derive(Debug, Clone, PartialEq)]
pub struct CompanyFinancialsState {
    pub is_loading: bool,
}

/// One validated point in the oldest-to-newest dual-line institution chart.
#[derive(Debug, Clone, PartialEq)]
pub struct CompanyInstitutionHistoryPointPresentation {
    pub period_label: String,
    pub owned_ratio: f64,
    /// Canonical decimal text for exact transport. Native parses only at the final chart boundary.
    pub price: String,
    pub owned_recipe: KlineFormatRecipe,
    pub price_recipe: KlineFormatRecipe,
}

/// One row in the institution holdings table.
#[derive(Debug, Clone, PartialEq)]
pub struct CompanyInstitutionHolderPresentation {
    pub name: String,
    pub owned_recipe: KlineFormatRecipe,
    pub change_recipe: KlineFormatRecipe,
    /// String-backed for forward compatibility: `positive`, `negative`, or `neutral`.
    pub change_sign: String,
}

/// One of the three fixed-order institution summary metrics.
/// 
/// Known IDs are `owned_pct`, `shares_held`, and `institutions`. Known `delta_sign` values are
/// `positive`, `negative`, and `neutral`.
#[derive(Debug, Clone, PartialEq)]
pub struct CompanyInstitutionMetricPresentation {
    pub id: String,
    pub value_recipe: KlineFormatRecipe,
    pub delta_recipe: KlineFormatRecipe,
    pub delta_sign: String,
}

/// The institution summary, trend, and per-holder table.
#[derive(Debug, Clone, PartialEq)]
pub struct CompanyInstitutionsPresentation {
    /// The shareholders endpoint does not currently report a price currency.
    pub price_currency_code: String,
    /// Compatibility currency used only to render the endpoint's price values.
    pub price_fallback_currency_code: String,
    pub metrics: Vec<CompanyInstitutionMetricPresentation>,
    pub history: Vec<CompanyInstitutionHistoryPointPresentation>,
    pub holders: Vec<CompanyInstitutionHolderPresentation>,
}

/// One metric value plus enough semantic information for a thin native renderer.
/// 
/// IDs and kinds are string-backed so an older native client can ignore unknown future values.
/// Known `value_kind` values are `recipe`, `recipe_pair`, `fiat_symbol`, `fiat_symbol_pair`,
/// `fiat_code`, and `localized_token`.
#[derive(Debug, Clone, PartialEq)]
pub struct CompanyMetricPresentation {
    pub id: String,
    pub value_kind: String,
    pub primary: KlineFormatRecipe,
    pub secondary: Option<KlineFormatRecipe>,
    pub separator: String,
    pub currency_code: String,
    pub fallback_currency_code: String,
    pub localized_token: String,
}

/// Both metric consumers projected from one quote payload.
#[derive(Debug, Clone, PartialEq)]
pub struct CompanyMetricsPresentation {
    pub profile_metrics: Vec<CompanyMetricPresentation>,
    pub data_panel: CompanyDataPanelPresentation,
}

/// `GET /priapi/v5/rubik/public/company/{ticker}/profile`.
/// 
/// Fields mirror the BE response verbatim. Company name is bilingual
/// (`companyNameEn`/`companyNameZh`), while `description` is already resolved
/// by the backend. Classification is delivered as several string arrays.
#[derive(Debug, Clone, PartialEq)]
pub struct CompanyProfile {
    pub symbol: Option<String>,
    pub company_name_en: Option<String>,
    pub company_name_zh: Option<String>,
    pub description: Option<String>,
    /// Market / country code: `US` / `HK` / `JP`.
    pub market: Option<String>,
    /// Market session status, e.g. `OPEN`.
    pub market_status: Option<String>,
    /// Board type, e.g. `MAIN`.
    pub board_type: Option<String>,
    pub logo_url: Option<String>,
    pub website: Option<String>,
    /// Exchange short name, e.g. `Nasdaq`.
    pub exchange: Option<String>,
    /// IPO / launch date, e.g. `"2010-06-29"`.
    pub ipo_date: Option<String>,
    /// Data snapshot timestamp (ISO-8601), e.g. `"2026-06-26T10:00:00Z"`.
    pub as_of: Option<String>,
    /// Human-readable industry labels, e.g. `["New energy vehicles"]`.
    pub industry_label: Vec<String>,
    /// FMP raw industry, e.g. `["Auto - Manufacturers"]` (the "Industry" tag).
    pub industry_raw: Vec<String>,
    /// FMP sector-level classification, e.g. `["Technology", "Consumer Cyclical"]`.
    pub investment_sectors: Vec<String>,
    /// Supply-chain partners, e.g. `["CATL", "Panasonic", "TSMC"]`.
    pub supply_chain: Vec<String>,
    pub latest_split: Option<CompanySplit>,
}

/// Profile and both quote-backed metric surfaces projected from one endpoint pair.
#[derive(Debug, Clone, PartialEq)]
pub struct CompanyProfileExperiencePresentation {
    pub profile: CompanyProfilePresentation,
    pub metrics: Option<CompanyMetricsPresentation>,
}

/// Profile content after locale-source selection and domain normalization.
#[derive(Debug, Clone, PartialEq)]
pub struct CompanyProfilePresentation {
    pub company_name: String,
    pub stock_ticker: String,
    pub logo_url: String,
    pub description: String,
    pub industry_label: String,
    pub category: Option<String>,
    pub split: Option<CompanySplitPresentation>,
    pub launch_date: Option<CompanyCalendarDate>,
    pub listed_exchange: Option<String>,
    pub website: String,
}

/// Tiny state — the profile/quote data flows through the UI event, so state only
/// tracks in-flight status.
#[derive(Debug, Clone, PartialEq)]
pub struct CompanyProfileState {
    pub is_loading: bool,
}

/// `GET /priapi/v5/rubik/public/tradfi/quote?ticker=`.
/// 
/// All numbers are raw values; percent/ratio fields are raw decimals
/// (`0.0065` = 0.65%).
#[derive(Debug, Clone, PartialEq)]
pub struct CompanyQuote {
    pub symbol: Option<String>,
    pub market_cap: Option<String>,
    pub float_market_cap: Option<String>,
    pub shares_outstanding: Option<String>,
    pub float_shares: Option<String>,
    /// Today's cumulative traded volume in shares.
    pub volume_shares: Option<String>,
    pub turnover24h: Option<String>,
    pub turnover_rate: Option<String>,
    pub amplitude_pct: Option<String>,
    pub volume_ratio: Option<String>,
    pub high52w: Option<String>,
    pub low52w: Option<String>,
    pub pe_ttm: Option<String>,
    pub pb: Option<String>,
    pub ps_ttm: Option<String>,
    pub eps_ttm: Option<String>,
    pub roe_pct: Option<String>,
    pub roa_pct: Option<String>,
    pub div_yield: Option<String>,
    /// e.g. `Year` / `Quarter`.
    pub div_frequency: Option<String>,
    pub last_dividend_amount: Option<String>,
    /// e.g. `USD`.
    pub div_currency: Option<String>,
    /// Trailing-twelve-month dividend per share, declared in [`Self::div_currency`]
    /// like [`Self::last_dividend_amount`]. Appended last with
    /// `#[uniffi(default = None)]` so existing native constructors of this Record
    /// stay source-compatible.
    pub dividend_ttm: Option<String>, // default = None
}

/// One ring/legend slice. Native supplies its palette color and the localized "Others" label.
#[derive(Debug, Clone, PartialEq)]
pub struct CompanyShareholderSlicePresentation {
    pub name: String,
    pub ratio: f64,
    pub percent_recipe: KlineFormatRecipe,
    pub is_others: bool,
}

/// Both independently renderable surfaces derived from one shareholders response.
/// 
/// `availability` is string-backed for forward compatibility. Known values are `available` and
/// `no_data`. Empty `slices` hides the ownership ring; `institutions = None` hides that card.
#[derive(Debug, Clone, PartialEq)]
pub struct CompanyShareholdersPresentation {
    pub availability: String,
    pub slices: Vec<CompanyShareholderSlicePresentation>,
    pub institutions: Option<CompanyInstitutionsPresentation>,
}

/// Tiny state — the stats data flows through the UI event, so state only tracks
/// in-flight status.
#[derive(Debug, Clone, PartialEq)]
pub struct CompanyShareholdersState {
    pub is_loading: bool,
}

/// Latest share split / reverse split of the company.
#[derive(Debug, Clone, PartialEq)]
pub struct CompanySplit {
    /// Split ex date, e.g. `"2022-08-25"` (JSON `exDate`).
    pub ex_date: Option<String>,
    /// Numerator, e.g. `4` in a 4-for-1 split (JSON `splitTo`).
    pub split_to: Option<String>,
    /// Denominator, e.g. `1` in a 4-for-1 split (JSON `splitFrom`).
    pub split_from: Option<String>,
    /// `STOCK_SPLIT` / `REVERSE_SPLIT` (JSON `splitType`).
    pub split_type: Option<String>,
}

/// A validated share-split ratio. Both sides are required for the row to be visible.
#[derive(Debug, Clone, PartialEq)]
pub struct CompanySplitPresentation {
    pub from: KlineFormatRecipe,
    pub to: KlineFormatRecipe,
}

/// One value cell in the full statement table.
#[derive(Debug, Clone, PartialEq)]
pub struct CompanyStatementDetailCellPresentation {
    pub value: CompanyFinancialValuePresentation,
    pub yoy: CompanyFinancialValuePresentation,
}

/// One backend-labelled section in the full statement table.
#[derive(Debug, Clone, PartialEq)]
pub struct CompanyStatementDetailGroupPresentation {
    pub key: String,
    pub header: String,
    pub rows: Vec<CompanyStatementDetailRowPresentation>,
}

/// Full table projection after applying the requested client-side period range.
#[derive(Debug, Clone, PartialEq)]
pub struct CompanyStatementDetailPresentation {
    /// Known values are `all`, `quarter`, and `year`.
    pub range_kind: String,
    pub range_value: Option<i32>,
    /// Wire order is preserved (currently newest first).
    pub periods: Vec<CompanyStatementPeriodIdentity>,
    pub groups: Vec<CompanyStatementDetailGroupPresentation>,
    pub available_years: Vec<i32>,
    pub available_quarters: Vec<u8>,
}

/// One backend-labelled row in the full statement table.
#[derive(Debug, Clone, PartialEq)]
pub struct CompanyStatementDetailRowPresentation {
    pub key: String,
    pub label: String,
    pub cells: Vec<CompanyStatementDetailCellPresentation>,
}

/// Parsed identity for a statement period.
#[derive(Debug, Clone, PartialEq)]
pub struct CompanyStatementPeriodIdentity {
    pub period_key: String,
    pub year: Option<i32>,
    pub quarter: Option<u8>,
    pub period_end: Option<CompanyFinancialCalendarDate>,
}

/// One chart column in a statement preview series.
#[derive(Debug, Clone, PartialEq)]
pub struct CompanyStatementPointPresentation {
    pub period: CompanyStatementPeriodIdentity,
    pub primary_bar: CompanyFinancialValuePresentation,
    pub secondary_bar: Option<CompanyFinancialValuePresentation>,
    pub line: CompanyFinancialValuePresentation,
}

/// Preview chart plus full-table projection for one statement endpoint.
#[derive(Debug, Clone, PartialEq)]
pub struct CompanyStatementPresentation {
    /// Known values are `income`, `balance`, and `cashflow`.
    pub statement_kind: String,
    pub timeframe: String,
    /// Currency reported by this statement endpoint. Empty means absent from the payload.
    pub currency_code: String,
    /// Compatibility currency used only when rendering monetary values.
    pub fallback_currency_code: String,
    pub has_double_bar: bool,
    pub series: Vec<CompanyStatementSeriesPresentation>,
    pub detail: CompanyStatementDetailPresentation,
}

/// One selectable statement chart series. `id` is stable and string-backed.
#[derive(Debug, Clone, PartialEq)]
pub struct CompanyStatementSeriesPresentation {
    pub id: String,
    pub is_percent_bar: bool,
    pub points: Vec<CompanyStatementPointPresentation>,
}

/// The subject facts needed to decide whether Company surfaces may load.
#[derive(Debug, Clone, PartialEq)]
pub struct CompanySubject {
    pub inst_id: String,
    pub inst_type: String,
    pub ticker: String,
    pub icon_url: String,
    pub inst_category: String,
    pub coin_groups: Vec<String>,
    pub is_pre_market: bool,
    /// `false` when the source instrument or its supporting coin cache was not
    /// available. Callers must treat the subject as not ready in that state.
    pub is_resolved: bool,
}

/// Compare candles stream update item
/// 
/// Represents a single instrument's K-line data update in a compare candles stream.
/// Used for WebSocket stream updates where multiple instruments are subscribed simultaneously.
#[derive(Debug, Clone, PartialEq)]
pub struct CompareCandlesStreamItem {
    /// Instrument ID (e.g., "BTC-USDT", "BTC-USDT-SWAP")
    pub inst_id: String,
    /// K-line data for this instrument
    pub candles: Vec<BizKLineData>,
}

/// Compare candles stream update
/// 
/// Contains updates for multiple instruments in a single batch.
/// Used for WebSocket stream updates with 200ms buffering.
#[derive(Debug, Clone, PartialEq)]
pub struct CompareCandlesStreamUpdate {
    /// List of instrument updates in this batch
    pub items: Vec<CompareCandlesStreamItem>,
}

/// Revision-addressed, bounded history bridge. The metadata remains typed so
/// Native can reject a stale result before decoding; the page body is one OKCH
/// byte payload instead of thousands of UniFFI candle records and strings.
#[derive(Debug, Clone, PartialEq)]
pub struct CompareHistoryDeltaFrame {
    pub subscription_id: u32,
    pub candle_version: u64,
    pub candle_revision: u64,
    pub payload_version: u16,
    pub payload: Vec<u8>,
}

/// Response item for compare candles data
/// 
/// Represents K-line data for a single instrument in a compare candles response.
/// Contains the instrument ID, candle type, bar/timeframe, and the raw K-line data array.
/// 
/// # Fields
/// - `inst_id`: Instrument ID (e.g., "BTC-USDT", "BTC-USDT-SWAP")
/// - `candle_type`: K-line type, one of: "TRADE", "INDEX", "MARK"
/// - `bar`: Bar/timeframe (e.g., "1m", "5m", "15m", "1H", "1D")
/// - `data`: Raw K-line data as array of arrays
/// Each inner array format: [ts, o, h, l, c, volCcy, vol, volCcyQuote, confirm?]
#[derive(Debug, Clone, PartialEq)]
pub struct CompareKLineResp {
    /// Instrument ID (e.g., "BTC-USDT", "BTC-USDT-SWAP")
    pub inst_id: String,
    /// K-line type, one of: "TRADE", "INDEX", "MARK"
    pub candle_type: String,
    /// Bar/timeframe (e.g., "1m", "5m", "15m", "1H", "1D")
    pub bar: String,
    /// Raw K-line data as array of arrays
    /// 
    /// Each inner array represents a single candle with the following format:
    /// - Index 0: Timestamp (ts) - milliseconds as string
    /// - Index 1: Open price (o) - as string
    /// - Index 2: High price (h) - as string
    /// - Index 3: Low price (l) - as string
    /// - Index 4: Close price (c) - as string
    /// - Index 5: Volume in currency (volCcy) - as string
    /// - Index 6: Volume in coin (vol) - as string
    /// - Index 7: Volume in quote currency (volCcyQuote) - as string
    /// - Index 8: Confirm status (optional) - "0" or "1" as string
    pub data: Vec<Vec<String>>,
}

/// Request parameters for history compare candles data
/// 
/// Used to fetch historical K-line data for multiple instruments in a single request.
/// Compare candles allow fetching K-line data for multiple instruments simultaneously
/// for comparison purposes.
/// 
/// # Parameters
/// - `inst_id`: Instrument ID (e.g., "BTC-USDT", "BTC-USDT-SWAP")
/// - `candle_type`: K-line type, one of: "TRADE", "INDEX", "MARK" (default: "TRADE")
/// - `bar`: Bar/timeframe (e.g., "1m", "5m", "15m", "1H", "1D")
/// - `before`: Start timestamp for query (for pagination)
/// - `after`: End timestamp for query (for pagination)
/// - `limit`: Limit number of results (default: 100, max: 100)
#[derive(Debug, Clone, PartialEq)]
pub struct CompareKlineReq {
    /// Instrument ID (e.g., "BTC-USDT", "BTC-USDT-SWAP")
    pub inst_id: String,
    /// K-line type, one of: "TRADE", "INDEX", "MARK"
    /// Default: "TRADE"
    pub candle_type: String,
    /// Bar/timeframe (e.g., "1m", "5m", "15m", "1H", "1D")
    pub bar: Option<String>,
    /// Start timestamp for query (for pagination)
    /// Request data after this timestamp (newer data)
    pub before: Option<String>,
    /// End timestamp for query (for pagination)
    /// Request data before this timestamp (older data)
    pub after: Option<String>,
    /// Limit number of results (default: 100, max: 100)
    pub limit: Option<String>,
    /// Per-inst forward/backward price-adjustment param (stock insts only).
    /// Wire key = "adjust" (ObjC parity: OKKLineRequestCenter `item[@"adjust"]`).
    /// Omitted from the serialized JSON entirely when `None`, so non-stock insts
    /// send no `adjust` key — matches ObjC's stock-only adjustment dict.
    pub adjustment: Option<String>,
}

/// Contract long/short ratio data
#[derive(Debug, Clone, PartialEq)]
pub struct ContractLongShortRatioData {
    /// Timestamp
    pub ts: String,
    /// Long percentage
    pub long_percent: String,
    /// Short percentage
    pub short_percent: String,
    /// Ratio
    pub ratio: String,
}

/// Request for contract long/short ratio data (Swap/Futures)
/// API: GET /priapi/v5/rubik/stat/contracts/{instName}/long-short-account-ratio
#[derive(Debug, Clone, PartialEq)]
pub struct ContractLongShortRatioReq {
    /// Instrument ID (e.g. "BTC-USDT-SWAP")
    pub inst_id: String,
    /// Period: "5m", "15m", "1H", "4H", "1D"
    pub period: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CostLineVo {
    pub pos_id: String,
    pub kind: String,
    pub price: String,
}

/// Request parameters for market depth WebSocket subscription
#[derive(Debug, Clone, PartialEq)]
pub struct DepthReq {
    /// Instrument ID (e.g., "BTC-USDT", "BTC-USDT-SWAP")
    pub inst_id: String,
    /// Depth grouping/ladder (e.g., "0.1", "0.01", "1")
    /// Determines price aggregation level
    pub grouping: String,
}

/// Display-ready order record for the K-line trade overlay.
#[derive(Debug, Clone, PartialEq)]
pub struct DisplayOrderVo {
    pub id: String,
    /// Attach algo id for TP / SL legs nested under a main order.
    /// Empty for standalone/main orders.
    pub attach_algo_id: String,
    /// Index into Native's V1-compatible origin order list.
    /// 
    /// Main orders use their `raw_orders` index; TP / SL legs nested under a
    /// main order reuse the main order's index; position-attached orders use
    /// the `raw_orders + positions.attach_orders` index.
    pub origin_pos: u32,
    pub inst_id: String,
    pub inst_type: String,
    pub order_type: KLineOrderType,
    pub side: KLineOrderSide,
    pub pos_side: KLinePositionSide,
    /// Trigger price mode (last / index / mark) for TP / SL / trigger orders
    pub trigger_px_mode: KLinePriceMode,
    /// V1-compatible raw trigger price type string (`last` / `index` / `mark`).
    /// Empty keeps the tooltip trigger price label blank.
    pub trigger_px_type: String,
    /// Whether this row is the parent order (true) or one of its attached
    /// TP / SL / trailing legs (false). Pipeline-derived.
    pub is_main_order: bool,
    /// Whether the parent trigger order uses Chase Limit after activation.
    /// Attached TP/SL rows inherit the value from their parent.
    pub is_plan_chase: bool,
    /// Whether this leg participates in a split-TP grouping
    pub is_multiple_order: bool,
    /// Whether this order closes the entire position. Native owns the
    /// localized "All" text; Rust only preserves the business semantic.
    pub is_all_position_tpsl: bool,
    /// Sequence number within its split-TP grouping (0 if not applicable)
    pub multiple_order_num: u32,
    /// Reduce-only flag forwarded from upstream
    pub reduce_only: bool,
    /// Trade mode forwarded from upstream
    pub td_mode: String,
    /// V1-compatible textual order-type code (populated by core-trading)
    pub origin_ord_type: String,
    /// Untouched price from upstream
    pub origin_price: String,
    /// Untouched main price from upstream
    pub origin_main_price: String,
    pub price: String,
    pub main_price: String,
    pub trigger_px: Option<String>,
    pub tp_ord_px: Option<String>,
    pub sl_ord_px: Option<String>,
    pub sl_trigger_px: Option<String>,
    pub move_trigger_px: Option<String>,
    pub active_px: Option<String>,
    pub size: String,
    /// Order leverage; drives the drag-time earning/margin PnL ratio for a
    /// standalone order (no linked position). Empty for spot.
    pub leverage: String,
    pub amount: String,
    pub amount_no_sign: String,
    /// Trade-unit label paired with [`amount`], kept separate from the number so
    /// native localizes it: contract units arrive as a bare Content-Design key
    /// (e.g. "pr_options_footer_switch_amount") that native resolves via
    /// `resolveCdKeyOrLiteral`; coin / quote symbols ("BTC" / "USDT") pass
    /// through. Empty when no unit applies. Native renders `amount` + this suffix.
    pub amount_unit: String,
    pub notional_usd: String,
    pub callback_ratio: Option<String>,
    pub callback_spread: Option<String>,
    pub callback: Option<String>,
    pub have_tp: bool,
    pub have_sl: bool,
    pub is_tp_limit: bool,
    pub pnl: Option<String>,
    pub pnl_ratio: Option<String>,
    /// Unit / currency label paired with [`pnl`]
    pub pnl_sign: Option<String>,
    /// ID of the position this row is attached to (TP / SL on a position)
    pub linked_position_id: Option<String>,
    /// ID of the parent order this row is attached to (split-TP / SL leg)
    pub linked_main_order_id: Option<String>,
    /// Creation time (Unix ms as string)
    pub c_time: String,
}

/// One yearly dividend point (chart bar = `amount`, line = `yield_pct`).
#[derive(Debug, Clone, PartialEq)]
pub struct DividendHistory {
    pub year: Option<String>,
    pub amount: Option<String>,
    /// Dividend yield (raw decimal, `"0.0256"` = 2.56%).
    pub yield_pct: Option<String>,
}

/// Dividends block of the dividend-splits calendar.
#[derive(Debug, Clone, PartialEq)]
pub struct DividendInfo {
    pub latest_ex_date: Option<String>,
    pub latest_amount: Option<String>,
    /// e.g. `USD`.
    pub currency: Option<String>,
    /// Dividend yield TTM (raw decimal, `"0.0256"` = 2.56%).
    pub div_yield_ttm: Option<String>,
    /// Payout ratio LFY (raw decimal, `"0.995"` = 99.5%).
    pub payout_ratio_lfy: Option<String>,
    /// Dividend payments per year.
    pub frequency: Option<String>,
    pub history: Vec<DividendHistory>,
    /// e.g. `Cash Dividend`.
    pub allocation_type: Option<String>,
    pub record_date: Option<String>,
    pub payment_date: Option<String>,
    pub announced_date: Option<String>,
}

/// `GET /priapi/v5/rubik/public/tradfi/calendar/dividend-splits?ticker=`.
#[derive(Debug, Clone, PartialEq)]
pub struct DividendSplits {
    pub symbol: Option<String>,
    /// Company launch / IPO date, e.g. `"2010-06-29"` — the wire key is `ipoDate`
    /// on the dividend-splits response (the tab no longer borrows the profile
    /// endpoint). Kept as `launch_date` since the UI labels it "Launch date".
    pub launch_date: Option<String>,
    pub dividends: Option<DividendInfo>,
    pub splits: Vec<StockSplit>,
}

/// One donut slice: a named top holder, or the aggregated "Others". Derived —
/// not part of the wire schema. Native supplies the localized "Others" label
/// and every slice color.
#[derive(Debug, Clone, PartialEq)]
pub struct DonutSlice {
    /// Holder name for a named slice; `None` for the "Others" slice.
    pub holder_name: Option<String>,
    /// Slice share (raw decimal). For "Others" this is `1 - sum(named)`.
    pub shares_pct: Option<String>,
    /// `true` for the aggregated "Others" slice.
    pub is_others: bool,
}

/// State for drawing line interactor
#[derive(Debug, Clone, PartialEq)]
pub struct DrawingLineState {
    /// All drawing lines for the current instrument
    pub lines: Vec<OKKLineDrawingData>,
    /// True while fetching lines
    pub is_loading: bool,
    /// True while a mutation (save/delete/upload) is in progress
    pub is_mutating: bool,
    /// Whether sync to server is enabled (logged in)
    pub sync_enabled: bool,
    /// Maximum number of drawing lines allowed
    pub max_lines: u32,
    /// Last error message
    pub error: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DrawingLineVo {
    pub id: String,
    pub line_type: String,
    pub points_json: String,
    pub color: String,
}

/// Yellow banner + secondary cursor + pill triple for an early-close window.
#[derive(Debug, Clone, PartialEq)]
pub struct EarlyCloseVo {
    /// Fully-rendered yellow banner text.
    pub banner_text: String,
    /// Precise early-close moment, epoch ms. Native maps it into whichever
    /// schedule segment contains it to place the secondary cursor (per the
    /// aligned marker semantics: the marker follows closeAtTs; if it falls
    /// outside every drawn segment, the cursor is not drawn).
    pub close_at_ts: i64,
    /// Position of the early-close marker within its owning segment, `[0.0, 1.0]`;
    /// `0.0` when [`Self::close_at_ts`] is outside every schedule row.
    pub progress_ratio: f32,
    /// "Early close" pill label under the marker.
    pub tag_text: String,
}

/// `GET /priapi/v5/rubik/public/tradfi/financials/estimate?ticker=`.
#[derive(Debug, Clone, PartialEq)]
pub struct EarningsEstimate {
    pub symbol: Option<String>,
    pub summary_title: Option<String>,
    /// e.g. `"2025-12-31 ~ 2026-03-31"`.
    pub report_period: Option<String>,
    pub currency: Option<String>,
    pub summary_text: Option<String>,
    /// How many analysts contributed to the forecast, quoted in the estimates
    /// disclaimer. Arrives as a bare JSON number, so it relies on the flexible
    /// deserializer to reach native as a string like every other value here.
    pub num_analysts: Option<String>,
    pub latest_actual: Option<EarningsLatest>,
    pub history: Vec<EarningsHistory>,
    /// The next (not-yet-reported) quarter: estimate-only, no actuals. Same shape as
    /// `latest_actual` (fiscal year/quarter + period end + metrics, where each metric carries just
    /// `estimated`). The trend chart appends this as the final forecast point — its Actual line
    /// ends at the last `history` quarter while the Estimate line continues to here.
    /// Appended last with `#[uniffi(default = None)]` so existing native constructors of this
    /// Record stay source-compatible (matches `CompanyQuote::dividend_ttm`).
    pub upcoming: Option<EarningsLatest>, // default = None
}

/// One historical quarter of actual-vs-estimate values (trend chart).
#[derive(Debug, Clone, PartialEq)]
pub struct EarningsHistory {
    pub fiscal_year: Option<String>,
    pub fiscal_quarter: Option<String>,
    pub revenue_actual: Option<String>,
    pub revenue_estimated: Option<String>,
    pub ebit_actual: Option<String>,
    pub ebit_estimated: Option<String>,
    pub eps_actual: Option<String>,
    pub eps_estimated: Option<String>,
}

/// Latest reported quarter.
#[derive(Debug, Clone, PartialEq)]
pub struct EarningsLatest {
    pub fiscal_year: Option<String>,
    pub fiscal_quarter: Option<String>,
    pub period_end: Option<String>,
    pub metrics: Option<EarningsMetrics>,
}

/// One earnings metric (revenue / eps / ebit): reported vs estimated.
#[derive(Debug, Clone, PartialEq)]
pub struct EarningsMetric {
    pub actual: Option<String>,
    /// Reported YoY change (raw decimal).
    pub actual_yoy_pct: Option<String>,
    pub estimated: Option<String>,
    /// Forecast YoY change (raw decimal).
    pub estimated_yoy_pct: Option<String>,
    /// `actual - estimated` (absolute).
    pub vs_estimate_abs: Option<String>,
    /// `(actual - estimated) / |estimated|` (raw decimal).
    pub vs_estimate_pct: Option<String>,
    /// `BEAT` / `MISSED` / `MET` / `None`.
    pub beat_miss: Option<String>,
}

/// The three earnings metrics of a report.
#[derive(Debug, Clone, PartialEq)]
pub struct EarningsMetrics {
    pub revenue: Option<EarningsMetric>,
    pub eps: Option<EarningsMetric>,
    /// EBIT — shown in the UI as "Pre-tax profit".
    pub ebit: Option<EarningsMetric>,
}

/// One allocation group (either the country group or the sector group). Rows
/// arrive pre-sorted and pre-aggregated; `items` is consumed in order.
#[derive(Debug, Clone, PartialEq)]
pub struct EtfAllocationGroup {
    pub status: Option<String>,
    pub items: Vec<EtfAllocationItem>,
}

/// One allocation slice (country or sector). The backend has **already** sorted
/// by `weightPct DESC`, capped to 6 (top-5 + one aggregated `OTHER`), and set
/// `code == "OTHER"` for the aggregated slice — the client must not re-sort,
/// re-cap, re-aggregate, or re-normalize.
#[derive(Debug, Clone, PartialEq)]
pub struct EtfAllocationItem {
    /// Stable code: ISO country (`US`), internal sector enum (`TECHNOLOGY`), or
    /// the literal `OTHER` for the backend-aggregated remainder. Empty when a
    /// sector is unmappable (then `name` carries the raw vendor value).
    pub code: Option<String>,
    /// Backend-localized (country) or enum-mapped (sector) display name; falls
    /// back to the raw FMP name on sector mapping failure.
    pub name: Option<String>,
    /// Raw weight (decimal); rendered as-is (the group sum may ≠ 100%).
    pub weight_pct: Option<String>,
}

/// `GET /tradfi/etf/allocations` — both country and sector groups in one call.
#[derive(Debug, Clone, PartialEq)]
pub struct EtfAllocations {
    pub status: Option<String>,
    pub country: EtfAllocationGroup,
    pub sector: EtfAllocationGroup,
}

/// One annual-return row (latest ≤5 years, descending; current year is YTD).
#[derive(Debug, Clone, PartialEq)]
pub struct EtfAnnualReturn {
    pub year: Option<String>,
    pub return_pct: Option<String>,
    /// Whether this row is the current (in-progress) year, marked "YTD".
    pub is_ytd: bool,
}

/// One benchmark (semantic type, never a ticker). Client renders `name` +
/// `series` and selects by `type_`; default is `DOW_JONES` else the first item.
#[derive(Debug, Clone, PartialEq)]
pub struct EtfBenchmark {
    /// Semantic enum: `DOW_JONES` / `NASDAQ` / `SP500` (never a ticker code).
    pub type_: Option<String>,
    /// Backend-localized benchmark name (rendered directly).
    pub name: Option<String>,
    /// Total return over the requested period (raw decimal string).
    pub return_pct: Option<String>,
    /// Annualized return (raw decimal string).
    pub annualized_return_pct: Option<String>,
    pub series: Vec<EtfPerformancePoint>,
}

/// One constituent change row (paired-disclosure comparison). `change_type`
/// drives sign + color only. `change_pct` is `None` for `ADDED` (no prior
/// period) — tolerated, not an anomaly. All money/quantity fields are raw
/// decimal strings carrying both disclosure snapshots' USD values.
#[derive(Debug, Clone, PartialEq)]
pub struct EtfConstituentChange {
    /// Stable match key (`isin:...` / `cusip:...` / internal). Never ticker-derived.
    pub holding_key: Option<String>,
    /// `ADDED` / `REMOVED` / `INCREASED` / `DECREASED` (4 displayable values;
    /// `UNKNOWN` / `UNCHANGED` are counted, not returned as rows).
    pub change_type: Option<String>,
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub instrument_id: Option<String>,
    /// OKX instType of [`Self::instrument_id`] (e.g. `SWAP` / `SPOT`); empty/None when unmatched.
    pub inst_type: Option<String>,
    pub is_tradable: bool,
    /// Prior-disclosure shares (raw decimal string).
    pub previous_shares: Option<String>,
    /// Current-disclosure shares (raw decimal string).
    pub current_shares: Option<String>,
    /// Signed share change (raw decimal; `+` profit color / `-` loss color).
    pub change_shares: Option<String>,
    /// Share change percent; `None` for `ADDED` (tolerated, still one change).
    pub change_pct: Option<String>,
    /// Prior-disclosure weight percent (raw decimal string).
    pub previous_weight_pct: Option<String>,
    /// Current-disclosure weight percent (raw decimal string).
    pub current_weight_pct: Option<String>,
    /// Signed weight change (raw decimal string).
    pub change_weight_pct: Option<String>,
    /// Prior-disclosure market value, USD (raw decimal string).
    pub previous_market_value: Option<String>,
    /// Current-disclosure market value, USD (raw decimal string).
    pub current_market_value: Option<String>,
    /// Signed market-value change, USD (raw decimal string).
    pub change_market_value: Option<String>,
}

/// One ETF constituent holding row. Superset shape shared by `/holdings/top`
/// (M1, lean use) and `/holdings` (M2, full column use).
/// 
/// `symbol` is the real stock code (display); `instrument_id` is the in-app
/// instrument (navigation) — the backend has already resolved Perp→Spot, so the
/// client never re-resolves and never mixes the two (`symbol == null` → name
/// only). `is_tradable == false` rows are still returned (shown, not clickable).
/// 
/// Two distinct percent fields, do not conflate them:
/// - `change_pct` — share change vs the prior disclosure (paired with
/// `change_shares` / `change_type`; `None` on M1 / for `ADDED`).
/// - `price_change_pct` — the 24h OKX quote change shown in the M2 detail table.
#[derive(Debug, Clone, PartialEq)]
pub struct EtfHolding {
    /// Stable per-snapshot id; never derived from ticker (swap/future/cash rows
    /// have no ticker but still carry a stable id).
    pub holding_id: Option<String>,
    /// Backend-localized display name (rendered directly, no on-device parsing).
    pub name: Option<String>,
    /// Real stock code (display). `None` for ticker-less rows (swap/cash/treasury).
    pub symbol: Option<String>,
    /// ISIN identifier when known (matching key on the changes endpoint).
    pub isin: Option<String>,
    /// In-app instrument id (navigation only). Empty/None → row not clickable.
    pub instrument_id: Option<String>,
    /// OKX instType of [`Self::instrument_id`] (e.g. `SWAP` / `SPOT`), forwarded verbatim to the
    /// market-detail page. Empty/None when unmatched (then `is_tradable == false`).
    pub inst_type: Option<String>,
    /// Whether the resolved in-app instrument is tradable (row clickable iff true).
    pub is_tradable: bool,
    /// Weight of total holdings (raw decimal, `"7.21"` = 7.21%).
    pub weight_pct: Option<String>,
    /// Shares held (raw decimal string).
    pub shares: Option<String>,
    /// Unit of `shares`: `SHARE` / `PAR_VALUE` / `null`.
    pub shares_unit: Option<String>,
    /// Position market value (raw decimal string).
    pub market_value: Option<String>,
    /// Currency of `market_value` (e.g. `USD`).
    pub market_value_currency: Option<String>,
    /// Signed share change vs the prior disclosure (raw decimal string).
    pub change_shares: Option<String>,
    /// Share change percent vs the prior disclosure (raw decimal; `None` for
    /// `ADDED` / M1). NOT the 24h price change — that is [`Self::price_change_pct`].
    pub change_pct: Option<String>,
    /// `ADDED` / `INCREASED` / `DECREASED` / `UNCHANGED` / `UNKNOWN` / `null`.
    pub change_type: Option<String>,
    /// `STOCK` / `DERIVATIVE` / `CASH` / `OTHER`.
    pub asset_type: Option<String>,
    /// Internal sector enum (e.g. `TECHNOLOGY`); empty when unmapped.
    pub sector: Option<String>,
    /// Backend-localized country / region label.
    pub country: Option<String>,
    /// Latest OKX quote price (raw decimal string; empty for non-securities).
    pub price: Option<String>,
    /// 24h OKX quote change percent (raw decimal string; empty for non-securities).
    /// This is the M2 detail table's "24h" column — distinct from `change_pct`.
    pub price_change_pct: Option<String>,
    /// As-of date of `price` (raw string; native formats).
    pub price_date: Option<String>,
    /// Currency of `price` (e.g. `USD`).
    pub price_currency: Option<String>,
}

/// `GET /tradfi/etf/holdings/changes` — compares the latest two valid disclosure
/// periods. Rows arrive already sorted `ABS(changeShares) DESC, holdingKey ASC`
/// (backend-defined authoritative order); the client never re-sorts. Cursor
/// paginated exactly like `/holdings`; `next_cursor` empty → end of list.
#[derive(Debug, Clone, PartialEq)]
pub struct EtfHoldingsChanges {
    /// `OK` / `NO_HISTORY` / `PARTIAL` / `FETCH_ERROR`.
    pub status: Option<String>,
    /// In-app instrument id for the ETF itself.
    pub etf_instrument_id: Option<String>,
    pub etf_symbol: Option<String>,
    /// Current disclosure date for the localized current-holding-date subtitle.
    pub current_holding_date: Option<String>,
    /// Prior disclosure date (the compared-against period).
    pub previous_holding_date: Option<String>,
    /// Total displayable change rows across all pages (backend count).
    pub total: Option<String>,
    /// Count of matched-but-incomparable rows (kept out of `items`).
    pub unknown_count: Option<String>,
    /// Count of unchanged rows (kept out of `items`).
    pub unchanged_count: Option<String>,
    /// Opaque continuation token; empty/None → end of list.
    pub next_cursor: Option<String>,
    pub items: Vec<EtfConstituentChange>,
}

/// `GET /tradfi/etf/holdings` — one page of the detail table (server sorted,
/// cursor paginated). The opaque `next_cursor` is bound backend-side to
/// dataVersion / snapshotId / sort, so it invalidates on sort change; empty
/// means no further pages.
#[derive(Debug, Clone, PartialEq)]
pub struct EtfHoldingsPage {
    pub status: Option<String>,
    /// In-app instrument id for the ETF itself.
    pub etf_instrument_id: Option<String>,
    pub etf_symbol: Option<String>,
    pub etf_name: Option<String>,
    /// Snapshot id shared with `/holdings/top` (M1).
    pub snapshot_id: Option<String>,
    pub holding_date: Option<String>,
    pub previous_holding_date: Option<String>,
    /// Same Top-8 sum as `/holdings/top`; independent of this page's sort/cursor.
    pub top_weight_pct: Option<String>,
    /// Actual loadable row count.
    pub total: Option<String>,
    /// Opaque continuation token; empty/None → end of list.
    pub next_cursor: Option<String>,
    pub source: Option<String>,
    pub items: Vec<EtfHolding>,
}

/// `GET /tradfi/etf/performance` — the ETF's own fund-level returns, the
/// per-period curve, benchmark comparison lines, plus fixed stage and annual
/// returns. `series` is the ETF's own curve for the requested period.
/// 
/// The top-level `return_pct` / `annualized_return_pct` are the fund's headline
/// numbers for the requested period; both are `None` (not `0`) when the backend
/// omits them, so the client can distinguish a real `0.00%` from "no data".
#[derive(Debug, Clone, PartialEq)]
pub struct EtfPerformance {
    pub status: Option<String>,
    pub etf_symbol: Option<String>,
    /// The requested/echoed period for the returned `series`.
    pub period: Option<String>,
    /// Curve window start (first valid trading day at/after the target start).
    pub base_date: Option<String>,
    /// Curve window end (most recent completed trading day).
    pub end_date: Option<String>,
    /// Fund cumulative return over the period (raw decimal string; `None` = no data).
    pub return_pct: Option<String>,
    /// Fund annualized return (raw decimal string; `None` = no data).
    pub annualized_return_pct: Option<String>,
    /// Server-generation timestamp (raw string; native formats if shown).
    pub updated_at: Option<String>,
    pub series: Vec<EtfPerformancePoint>,
    pub benchmarks: Vec<EtfBenchmark>,
    /// Fixed 5 (1M/3M/6M/YTD/1Y), independent of the curve period.
    pub stage_returns: Vec<EtfStageReturn>,
    pub annual_returns: Vec<EtfAnnualReturn>,
}

/// One curve point. The wire field is `returnPct` (cumulative return for the
/// point, curve start normalized to `0.00`). `cumulativeReturnPct` is accepted
/// as a legacy alias only — new payloads use `returnPct`.
#[derive(Debug, Clone, PartialEq)]
pub struct EtfPerformancePoint {
    pub date: Option<String>,
    /// Cumulative return percent at this point (raw decimal string). Canonical
    /// wire name `returnPct`; `cumulativeReturnPct` tolerated as a legacy alias.
    pub return_pct: Option<String>,
}

/// One fixed stage-return item (1M / 3M / 6M / YTD / 1Y). `return_pct == None`
/// → insufficient coverage, render placeholder (never substitute a shorter
/// period).
#[derive(Debug, Clone, PartialEq)]
pub struct EtfStageReturn {
    pub period: Option<String>,
    pub return_pct: Option<String>,
}

/// `GET /tradfi/etf/holdings/top` — current-snapshot top-8 holdings card.
#[derive(Debug, Clone, PartialEq)]
pub struct EtfTopHoldings {
    /// Business status: `OK` / `STALE` / `NO_DATA` / `FETCH_ERROR`.
    pub status: Option<String>,
    /// In-app instrument id for the ETF itself.
    pub etf_instrument_id: Option<String>,
    /// The ETF's own display symbol (e.g. `QQQ`).
    pub etf_symbol: Option<String>,
    /// Backend-localized ETF name.
    pub etf_name: Option<String>,
    /// Snapshot id shared with `/holdings` (M2).
    pub snapshot_id: Option<String>,
    /// Disclosure / as-of date for the localized "as of / updated" label (locale-aware
    /// formatting is done natively; raw string crosses the FFI).
    pub holding_date: Option<String>,
    /// Alternative field name for the disclosure date.
    pub current_holding_date: Option<String>,
    /// The prior disclosure date (paired-snapshot origin of `change_*`).
    pub previous_holding_date: Option<String>,
    /// Sum of the returned items' weight (no normalization). `NO_DATA` → `"0.00"`.
    pub top_weight_pct: Option<String>,
    /// Current-snapshot valid row count (may be ≫ 8); NOT `items.len()`.
    pub total: Option<String>,
    /// Provenance of the snapshot (e.g. `FMP`); diagnostic, not user-facing.
    pub source: Option<String>,
    /// Up to 8 rows (backend already selected + ordered them).
    pub items: Vec<EtfHolding>,
}

/// Request config sent from UI to the interactor.
/// 
/// Matches the Kotlin-side `EventChartConfig` shape but with fields the Rust
/// usecases actually need.
#[derive(Debug, Clone, PartialEq)]
pub struct EventChartConfigVo {
    /// Event-contract instrument id (for the probability WS channel).
    pub inst_id: String,
    /// Underlying index instrument id (for index-candles). Falls back to
    /// `inst_id` when empty.
    pub event_inst_id: String,
    /// BE-canonical series id (IdxConfig.referId, e.g. "BTC-UPDOWN-5MIN") used
    /// to resolve the settlement-window config. Empty → derived from `inst_id`
    /// (first three segments) as a fallback.
    pub series_id: String,
    /// Contract kind.
    pub contract_kind: EventChartContractKind,
    /// Delivery time in milliseconds (for settled + ALL-range history anchor).
    pub delivery_time_ms: i64,
    /// Whether the contract has already settled.
    pub is_settled: bool,
    /// Selected time range.
    pub time_range: EventChartTimeRange,
    /// Candle bar (e.g. "1m", "5m"). Resolved by the UI layer.
    pub bar: String,
    /// Max points to keep in the sliding window.
    pub limit: i32,
    /// Optional `after` cursor in milliseconds (live index price paging).
    pub x_axis_start_ms: Option<i64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EventIndexPriceState {
    pub points: Vec<IndexPricePoint>,
    pub update_kind: UpdateKind,
    pub is_loading: bool,
    pub resolved_bar: String,
    pub error: Option<String>,
    /// Settlement-window duration in seconds, resolved from the event-contract
    /// index config (IdxConfig.settlePeriodSec). 0 when absent/unparsable — the
    /// chart skips the settlement window then. Native renders the localized label.
    pub settle_window_secs: i32,
    /// Suppress the settlement price line: a settled (historical) contract whose
    /// settlePeriodSec is explicitly "0" has no settlement-window concept.
    pub hide_settle_price: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EventProbabilityState {
    pub points: Vec<ProbabilityPoint>,
    pub update_kind: UpdateKind,
    pub is_loading: bool,
    pub resolved_bar: String,
    pub error: Option<String>,
}

/// One section of a statement (a `lineItems[]` entry) grouping its
/// [`FinancialSectionItem`]s under a backend-provided `label`
/// (e.g. "Operating Activities").
#[derive(Debug, Clone, PartialEq)]
pub struct FinancialSection {
    pub key: String,
    pub label: String,
    pub section_items: Vec<FinancialSectionItem>,
    /// Section discriminator; currently always "SECTION" (kept for forward-compat).
    pub kind: String,
}

/// One line item inside a statement [`FinancialSection`]. The BE now returns a
/// display `label` directly (native no longer maps keys -> local copy). `value`
/// is a raw-decimal string and is **absent** for header-only rows (native
/// renders those as a placeholder). `parent_key`, when present, marks this item
/// as an indented sub-item of the sibling whose `key` equals it.
#[derive(Debug, Clone, PartialEq)]
pub struct FinancialSectionItem {
    pub key: String,
    pub label: String,
    pub value: Option<String>,
    pub parent_key: Option<String>,
}

/// One reporting period. `line_items` carries the section-grouped, labeled line
/// items (the BE returns display labels directly). `yoy_pct` is a raw key->value
/// bag of per-period YoY changes keyed by **camelCase summary metric** (e.g.
/// `netIncome`, `freeCashFlow`) — NOT the line-item keys; native attaches YoY to
/// the matching summary rows and derives the rest from the prior-year column.
/// `metrics` is a per-period bag of derived ratios/aggregates (income endpoint:
/// `netAssets`, `currentRatio`, `quickRatio`, `roePct`, `roaPct`,
/// `grossMarginPct`, `netMarginPct`); empty for endpoints that omit it (cash flow).
#[derive(Debug, Clone, PartialEq)]
pub struct FinancialStatementPeriod {
    pub period: Option<String>,
    pub period_end: Option<String>,
    pub filing_date: Option<String>,
    pub line_items: Vec<FinancialSection>,
    pub yoy_pct: HashMap<String, String>,
    pub metrics: HashMap<String, String>,
}

/// Shared shape of the income / balance / cash-flow endpoints (periods
/// newest-first, as the BE returns them).
#[derive(Debug, Clone, PartialEq)]
pub struct FinancialStatements {
    pub symbol: Option<String>,
    pub timeframe: Option<String>,
    pub reported_currency: Option<String>,
    pub as_of: Option<String>,
    pub available_periods: Vec<String>,
    pub statements: Vec<FinancialStatementPeriod>,
}

/// One initial load of the whole tab. Each section is optional — the page
/// renders whatever loaded (partial success).
#[derive(Debug, Clone, PartialEq)]
pub struct FinancialsData {
    pub estimate: Option<EarningsEstimate>,
    pub income: Option<FinancialStatements>,
    pub balance: Option<FinancialStatements>,
    pub cashflow: Option<FinancialStatements>,
}

/// Resolved display + routing for the bottom first icon (strategy / bot).
/// `jump_*` are reserved for future first-icon routing; they are empty today
/// (native `applyFirstIconStyle` still derives the drawable / label from
/// instType, and the first icon has no biz-list-derived jump target yet).
#[derive(Debug, Clone, PartialEq)]
pub struct FirstIconPlan {
    pub visible: bool,
    /// Jump target biz type; empty today (reserved for future extension).
    pub jump_inst_type: String,
    /// Jump target inst id; empty today (reserved for future extension).
    pub jump_inst_id: String,
}

/// Single flash-news item, merged from `NewsCarouselResult` (campaign first,
/// then content_flash — V1 `KlineFlashDataMapper` order).
/// 
/// `flash_type` is `"campaign"` for promotions, `"content"` for news.
/// `description` is the V1 secondary line; native falls back to `title`
/// when empty. `timestamp` reserved for future sort/timeline features.
#[derive(Debug, Clone, PartialEq)]
pub struct FlashNewsItemVo {
    pub id: String,
    pub title: String,
    pub description: String,
    pub url_slug: String,
    pub flash_type: String,
    pub timestamp: i64,
}

/// Funding rate data
#[derive(Debug, Clone, PartialEq)]
pub struct FundingRateData {
    /// Funding timestamp
    pub funding_time: String,
    /// Funding rate
    pub funding_rate: String,
    /// Instrument type (e.g. "SWAP")
    pub inst_type: Option<String>,
    /// Instrument ID (e.g. "BTC-USDT-SWAP")
    pub inst_id: Option<String>,
    /// Realized rate
    pub realized_rate: Option<String>,
    /// Funding rate calculation method
    pub method: Option<String>,
    /// Formula type
    pub formula_type: Option<String>,
}

/// Request for funding rate data (Swap)
/// API: GET /priapi/v5/public/funding-rate-history
#[derive(Debug, Clone, PartialEq)]
pub struct FundingRateReq {
    /// Instrument ID (e.g. "BTC-USDT-SWAP")
    pub inst_id: String,
    /// Limit: max number of records to return (default "100")
    pub limit: String,
}

/// Futures-Spot Gap data (combines futures and index candles)
#[derive(Debug, Clone, PartialEq)]
pub struct FuturesSpotGapData {
    /// Timestamps
    pub dates: Vec<String>,
    /// Futures close prices
    pub futures_data: Vec<String>,
    /// Index close prices
    pub index_data: Vec<String>,
}

/// Request for futures-spot gap data (combines futures candles + index candles)
#[derive(Debug, Clone, PartialEq)]
pub struct FuturesSpotGapReq {
    /// Futures instrument ID (e.g. "BTC-USDT-SWAP")
    pub futures_inst_id: String,
    /// Index instrument ID (e.g. "BTC-USD")
    pub index_inst_id: String,
    /// Bar size: "1m", "5m", "1H", "1D"
    pub bar: String,
    /// Limit: max 300, default 100
    pub limit: String,
}

/// K-line page top header info VO.
/// 
/// Each `*_recipe` carries precision + sign + rounding + flags; native
/// renders via `KlineFormatRecipe.render()`. `*_raw` keeps the unmodified
/// numeric for color logic. `*_unit` is the UI label slot next to the
/// abbreviated volume / turnover value.
#[derive(Debug, Clone, PartialEq)]
pub struct HeadInfoVo {
    pub inst_id: String,
    /// Active price source: "kline" / "index" / "mark".
    pub price_type: String,
    pub price_recipe: KlineFormatRecipe,
    pub price_diff_recipe: KlineFormatRecipe,
    pub change_24h_recipe: KlineFormatRecipe,
    pub local_price_recipe: KlineFormatRecipe,
    pub high_24h_recipe: KlineFormatRecipe,
    pub low_24h_recipe: KlineFormatRecipe,
    pub mark_price_recipe: KlineFormatRecipe,
    pub vol_24h_recipe: KlineFormatRecipe,
    pub vol_ccy_24h_recipe: KlineFormatRecipe,
    pub price_raw: String,
    pub change_24h_raw: String,
    pub vol_24h_raw: String,
    pub vol_ccy_24h_raw: String,
    pub vol_24h_unit: String,
    pub vol_ccy_24h_unit: String,
    /// Whether to show the price-type (latest/mark/index) switch entry. Always true
    /// for derivatives; for spot only when margin-supported or a lever-tagged USDs pair.
    pub price_type_switch_visible: bool,
    /// Whether the price-type menu includes "mark price". False for lever-tagged USDs
    /// spot pairs (they have no mark price).
    pub mark_price_supported: bool,
    /// Open interest (OI), already scaled by the contract multiplier to coin units;
    /// thousands separator rendered by native -> use recipe.
    pub open_interest_recipe: KlineFormatRecipe,
    /// Spot index price; thousands separator rendered by native -> use recipe.
    pub spot_index_recipe: KlineFormatRecipe,
    /// Option greeks (PA / BS chosen by greek_unit), already formatted (8 dp, trailing zeros trimmed).
    pub delta: String,
    pub gamma: String,
    pub vega: String,
    pub theta: String,
    /// Mark volatility, already formatted as a percentage (e.g. "30.00%").
    pub mark_vol: String,
    /// Leverage multiplier, already formatted (2 dp).
    pub leverage: String,
    /// Implied valuation for Pre-IPO equity (TradFi) perpetuals.
    /// `None` when the instrument is not Pre-IPO TradFi, or when
    /// `estimated_total_shares` / latest price are invalid. See
    /// [`ImpliedValuationVo`] and `build_head_info_use_case` for the
    /// price-input / gating rules (mirrors V1 `ImpliedValuationCalculator`).
    pub implied_valuation: Option<ImpliedValuationVo>,
    /// Whether the instrument is a pre-market (Pre-IPO) pair. Native renders the
    /// mark-price label as a dashed, tappable helper label (EMA tooltip) when true;
    /// a plain label otherwise. OC/V1: `instrument.isPreMarketPair()`.
    pub is_pre_market: bool,
    /// USDⓈ(UM)-margined OPTION; `false` for every other inst type. Native hides the
    /// fiat-valuation row for these. V1: `(inst as? OptionInstrument)?.isUSDUMContract()`.
    pub is_usds_option: bool,
}

/// Instrument metadata for the header (tag / leverage / company-name / stock
/// session). Split out from [`HeadInfoVo`] because it only changes on
/// instrument switch / session-band rollover — unlike the per-tick price
/// fields — so native can subscribe it as a separate, naturally-deduped
/// sub-state instead of re-rendering on every ticker push.
#[derive(Debug, Clone, PartialEq)]
pub struct HeadMetaVo {
    /// Resolved instrument kind for tag rendering. See [`HeadInstrumentKind`].
    pub instrument_kind: HeadInstrumentKind,
    /// Max leverage label for SPOT-with-margin / MARGIN ("Nx"). `None` when
    /// the instrument is not margin-enabled.
    pub max_leverage: Option<String>,
    /// Redundant flag derived from `instrument_kind` so native can short-
    /// circuit pre-market UI without re-matching the enum.
    pub is_pre_market: bool,
    /// TradFi / RWA company name, read from in-memory coin info's
    /// `commodity_tag` (same source as V1 `getTradeCoinInfo(symbol).commodityTag`).
    /// `None` when biz is not loaded yet or the coin has no commodity tag.
    pub company_name: Option<String>,
    /// `true` for TradFi / RWA instruments. Kept for non-tag consumers (e.g.
    /// the trade-margin unit display); the trading-hours TAG gate moved to
    /// [`Self::trad_fi_region`] per the PM rule (region non-empty + not
    /// pre-market).
    pub is_tradfi: bool,
    /// TradFi region code from in-memory coin info ("US"/"JP"/"KR"); empty
    /// when absent. The trading-hours tag shows only when non-empty. The
    /// session itself comes from the server-driven CEX/Ondo stock-hours
    /// interactors, not from this VO.
    pub trad_fi_region: String,
}

/// Request parameters for history candles data (historical K-line data from recent years)
/// 
/// Different K-line types use different API endpoints:
/// - `Latest`: `/priapi/v5/market/history-candles` (inst_id = trading pair ID)
/// - `Index`: `/priapi/v5/market/history-index-candles` (inst_id = underlying asset)
/// - `MarkPrice`: `/priapi/v5/market/history-mark-price-candles` (inst_id = trading pair ID)
#[derive(Debug, Clone, PartialEq)]
pub struct HistoryCandlesReq {
    /// K-line type (latest/index/mark-price)
    /// Determines which API endpoint to use
    pub kline_type: KlineType,
    /// Instrument ID (e.g., "BTC-USDT", "BTC-USDT-SWAP")
    /// For Index type, this should be the underlying asset (e.g., "BTC-USD")
    pub inst_id: String,
    /// Bar/timeframe (e.g., "1m", "5m", "15m", "1H", "1D")
    pub bar: Option<String>,
    /// Timestamp after which to fetch data (for pagination)
    /// Request data before this timestamp
    pub after: Option<String>,
    /// Timestamp before which to fetch data (for pagination)
    /// Request data after this timestamp
    pub before: Option<String>,
    /// Limit number of results (default: 100, max: 100)
    pub limit: Option<String>,
    /// Stock price adjustment mode (e.g., "forward", "none").
    /// Sent as `adjust` query param. Only honored for `KlineType::Latest`;
    /// ignored for Index/MarkPrice since those endpoints do not accept it.
    pub adjustment: Option<String>,
    /// Index instrumentName for X-perp pre-launch backfill (e.g., "BTC-USD").
    /// When set, Latest/MarkPrice requests route to the history-mixed
    /// endpoints; ignored for Index. `#[uniffi(default = None)]` keeps
    /// existing native constructors of this Record source-compatible.
    pub index_name: Option<String>, // default = None
}

/// Request parameters for history formula candles data
/// 
/// Used to fetch historical K-line data calculated from formula expressions.
/// Formula candles allow combining multiple instruments using mathematical expressions.
#[derive(Debug, Clone, PartialEq)]
pub struct HistoryFormulaCandlesReq {
    /// Calculation expression string (e.g., "V0 + V1 - V2")
    /// Variables like V0, V1, V2 represent instruments from inst_id_array
    pub calc_expression: String,
    /// Instrument ID array as JSON string
    /// Contains pairs of [instId, instType] for each variable in the formula
    /// Example: ["BTC-USDT","SPOT"],["ETH-USDT","SPOT"]
    pub inst_id_array: String,
    /// Bar/timeframe (e.g., "1m", "5m", "15m", "1H", "1D")
    /// Default: "1m"
    pub bar: Option<String>,
    /// Timestamp after which to fetch data (for pagination)
    /// Request data before this timestamp (older data)
    pub after: Option<String>,
    /// Timestamp before which to fetch data (for pagination)
    /// Request data after this timestamp (newer data)
    pub before: Option<String>,
    /// Limit number of results (default: 100, max: 100)
    pub limit: Option<String>,
    /// Stock candle-adjustment param ("forward" = adjusted; None/empty = raw).
    /// Mirrors native combine-candles `adjustment:` — server returns adjusted
    /// history when set; the realtime WS frame stays RAW (client merges).
    pub adjustment: Option<String>,
}

/// Holiday-closure overlay for the sheet (reason + full closed window).
/// 
/// Backend v0.4 collapses holiday `schedule` to a single row spanning the
/// full closure window, so we don't need a separate `progress_start_ratio` —
/// the whole progress bar IS the holiday window.
#[derive(Debug, Clone, PartialEq)]
pub struct HolidayVo {
    /// Sub-title above the banner, e.g. "Market closed" — backend-localized.
    pub title_text: String,
    /// Fully-rendered banner body, e.g. "Thu 11/27 09:00 - Fri 11/28 09:00,
    /// US Stock Market market is closed for a public holiday." — Rust fills
    /// `{startDate}` / `{endDate}` / `{marketName}` into the backend template.
    pub banner_text: String,
    /// Fully-rendered reopen countdown, e.g. "US Stock Market market opens in 14h 0m.".
    pub reopen_text: String,
    /// "11/26 09:00 - 11/27 09:00" in device-local time (redundant with banner,
    /// exposed separately so callers that want a bare range don't have to parse it).
    pub time_range: String,
    /// Cursor position within the holiday window, `[0.0, 1.0]`.
    pub progress_current_ratio: f32,
}

/// Pre-IPO "Implied valuation" panel for TradFi perpetuals.
/// 
/// Formula (PRD AC-1): `implied_valuation = lastPx × estimated_total_shares`,
/// where `lastPx` is always the ticker last price (not affected by the
/// user's K-line price-type pick — kline / mark / index).
/// 
/// Hidden (the parent field is `None`) when:
/// - instrument is not Pre-IPO TradFi (rule_type ≠ 5 or `tradFi == false`)
/// - `estimated_total_shares` empty / non-numeric / ≤ 0
/// - last price empty / non-numeric / ≤ 0
/// 
/// Both numeric fields are emitted as **raw decimal strings** — Rust does not
/// format the compact suffix because device-locale-aware ICU compact formatting
/// (CJK wan / yi / wan-yi / K / M / B / T buckets) already lives in native via
/// `BigDecimal.formatICUCompact()` (OKLocalization). Pushing the same buckets
/// from Rust would (a) re-invent ICU, (b) miss trillion support, (c) burn FFI
/// surface for no gain.
#[derive(Debug, Clone, PartialEq)]
pub struct ImpliedValuationVo {
    /// Raw valuation = `lastPx × estimated_total_shares`. Native pipes this
    /// through `formatICUCompact()` to render a compact display string
    /// (locale-aware: CJK trillion bucket on zh, `"1.69T"` on en).
    pub value_raw: String,
    /// Quote currency code shown in the header label, e.g. "USDT".
    /// Native renders the label as "Implied valuation ({currency})".
    pub currency: String,
    /// Raw estimated total shares — used in the info-dialog `{amount}`
    /// placeholder. Native composes the same way as `value_raw`.
    pub shares_raw: String,
}

/// Single index price point exposed to the UI.
#[derive(Debug, Clone, PartialEq)]
pub struct IndexPricePoint {
    /// Close price of the candle.
    pub value: f32,
    /// Candle open timestamp (milliseconds).
    pub timestamp: i64,
    /// Whether the candle is confirmed. Only meaningful for above contract.
    pub confirmed: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IndicatorSyncDownloadReq {
    pub time_version: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IndicatorSyncDownloadResult {
    pub time_version: String,
    pub synced_count: u32,
    pub visible_states: Vec<IndicatorVisibleState>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IndicatorSyncState {
    pub last_error: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IndicatorSyncUploadReq {
    pub server_keys: Vec<String>,
    /// Show states for each server key. "0" = visible, "1" = hidden.
    /// If provided, these values are used directly instead of deriving from is_selected.
    pub show_states: Vec<IndicatorUploadShowState>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IndicatorSyncUploadResult {
    pub time_version: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IndicatorUploadShowState {
    pub server_key: String,
    /// "0" = visible, "1" = hidden
    pub show: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IndicatorVo {
    pub name: String,
    pub is_main: bool,
    pub params_json: String,
}

/// Per-inst stock-adjustment parameter fed from the Swift side-car (only the
/// stock insts that need adjustment appear here). UniFFI cannot export HashMap
/// directly (see `get_all_candle_inst_ids`), so the setter takes a Vec of records.
#[derive(Debug, Clone, PartialEq)]
pub struct InstAdjustment {
    pub inst_id: String,
    pub adjustment: String,
}

/// Tail update for one instrument pushed in `state.realtime_batch`: the bars at
/// the END of the cached series that the WS merge replaced or appended since
/// the last pushed batch. Native applies these onto its per-inst local series
/// and falls back to a full reload (`get_candles_for_inst` per instrument)
/// whenever the seq/ts/count checks fail. This is the multi-instrument
/// counterpart of the single-inst pages' `realtime_batch: Vec<KlineQuoteCandleVo>`.
#[derive(Debug, Clone, PartialEq)]
pub struct InstCandlesTail {
    pub inst_id: String,
    /// Same FFI-friendly VO the single-inst pages push (ts: i64, no confirm).
    pub tail: Vec<KlineQuoteCandleVo>,
    /// Total bars cached for this instrument after the update. Native verifies
    /// its local count matches after applying the tail and falls back to a full
    /// reload for that instrument on mismatch.
    pub total_count: u64,
}

/// Per-inst corporate-action effectiveTime(ms) list fed from the Swift side-car
/// after the batch adjustment HTTP returns.
#[derive(Debug, Clone, PartialEq)]
pub struct InstEffectiveTimes {
    pub inst_id: String,
    pub times: Vec<i64>,
}

/// One point of the institutional-ownership history line chart.
#[derive(Debug, Clone, PartialEq)]
pub struct InstHistoryPoint {
    pub period: Option<String>,
    pub period_date: Option<String>,
    /// Institutional ownership at this point (raw decimal).
    pub inst_pct: Option<String>,
    pub price: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InstrumentRef {
    /// Instrument ID, e.g. "BTC-USDT".
    pub inst_id: String,
    /// Instrument type: "SPOT" | "SWAP" | "FUTURES" | "OPTION".
    pub inst_type: String,
}

/// Position volume & trading volume union item
#[derive(Debug, Clone, PartialEq)]
pub struct InterestAndTradingUnionItem {
    /// Open interest in crypto (base currency like BTC)
    pub oi_crypto: String,
    /// Open interest in USDs (notional value)
    pub oi_usds: String,
    /// Timestamp
    pub ts: String,
    /// Trading volume in crypto (base currency)
    pub vol_crypto: String,
    /// Trading volume in USDs (notional value)
    pub vol_usds: String,
}

/// Account-level configuration snapshot for K-line display.
/// 
/// OC: Replaces scattered property reads on `tradeSetting` / `accountConfig`.
#[derive(Debug, Clone, PartialEq)]
pub struct KLineAccountConfig {
    /// Account level: `"1"` = Simple, `"2"` = SCC, `"3"` = MCC, `"4"` = PM.
    /// OC: `OKTAccountLevelType`.
    pub account_level: String,
    /// Position mode: `"long_short_mode"` (hedge) or `"net_mode"` (one-way).
    /// OC: `OKTCangModeConfigCenter` / `tradeSetting.posMode`.
    pub position_mode: String,
    /// Whether the account supports reverse (close) positions.
    /// `false` during contract cooling period.
    /// OC: `OKTContractUserState.isDuringCoolingPeriod`.
    pub is_account_level_support_reverse: bool,
    /// Position P&L display mode: `true` = last price, `false` = mark price.
    /// OC: `tradeSetting.pnlPriceType`.
    pub is_last_price_type_position_pnl: bool,
    /// Cost display mode: `true` = average cost, `false` = total cost.
    /// OC: `tradeSetting.spotAverageConfig`.
    pub is_average_cost: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct KLineCompareState {
    pub is_initialized: bool,
    pub is_loading: bool,
    pub init_error: Option<String>,
    pub inst_id: String,
    pub biz_type: String,
    pub base_ccy: String,
    pub quote_ccy: String,
    pub tick_sz: String,
    pub ct_val: String,
    pub inst_title: String,
    pub amount_unit: String,
    pub quote_symbol: String,
    pub ticker_version: u64,
    pub candle_version: u64,
    pub indicator_subscription_id: u32,
    pub candle_revision: u64,
    pub candle_update_type: CompareCandleUpdateType,
    pub candle_loading: bool,
    pub candle_error: Option<String>,
    pub finished_first_candle_load: bool,
    pub has_more_history: bool,
    pub current_period: String,
    /// Per-inst tails accepted this WS merge window (cleared on History /
    /// MoreHistory / start_candle).
    pub realtime_batch: Vec<InstCandlesTail>,
    /// Bumps once per pushed realtime batch AND once per load_more
    /// (MoreHistory), so a coalesced MoreHistory forces the next realtime tick
    /// into a full reload (seq gap). Reset to 0 on History / start_candle.
    pub realtime_seq: u64,
    /// Legacy transport field retained for UniFFI compatibility. CandleStore is
    /// now always authoritative, so this stays empty.
    pub lean_history_push: Vec<InstCandlesTail>,
    /// Earliest cached bar ts of the PRIMARY inst, retained as a diagnostic and
    /// compatibility cursor. Pagination reads the primary CandleStore directly.
    pub oldest_ts: String,
    pub digit: u32,
    /// User trade-unit setting for the primary inst ("0" coin / "1" contract /
    /// "2" quote; empty for spot/margin). Same source as the main-K init
    /// (`resolve_trade_unit`); native derives the VOL-indicator unit + chart
    /// coinVol from it so the selection popup needs no Kotlin trade biz.
    pub trade_unit: String,
    pub data_source_type: u64,
    pub is_simulator: bool,
    pub is_option: bool,
}

/// Countdown model DTO for new-coin countdown display.
/// OC: `[trade countDownModelWithInstID:]`.
#[derive(Debug, Clone, PartialEq)]
pub struct KLineCountdownModel {
    pub inst_id: String,
    pub icon: String,
    /// Stage type: 0=show countdown, 1=will online soon, 2=cancelled
    pub stage_type: i32,
    pub group: String,
    pub online_ts: String,
    /// Countdown seconds until online (from server), value >= 0
    pub wait_online_time: i64,
    pub notice_url_cn: String,
    pub notice_url_en: String,
    /// Opening method: "0"=auction, "1"=opening price, "2"=pre-quote
    pub open_method: String,
    pub display_id: String,
    pub device_online_timestamp: Option<f64>,
}

/// Data source type bit flags (corresponds to `OKTDataSourceType`).
/// The `value` field is a bitmask indicating which data sources to subscribe to.
/// 
/// Raw `u64` bitmask is used (instead of `Vec<Enum>`) for zero-cost FFI transfer
/// and direct compatibility with OC's `NS_OPTIONS(OKTDataSourceType)` pattern.
/// See `biz_utils_bridge::KLINE_DS_TYPE_*` constants for available flags.
#[derive(Debug, Clone, PartialEq)]
pub struct KLineDataSourceType {
    pub value: u64,
}

/// Result of a decimal arithmetic operation via the decimal bridge.
/// Plain-string representation (like `BigDecimal.toPlainString()`) for lossless FFI transfer.
#[derive(Debug, Clone, PartialEq)]
pub struct KLineDecimalResult {
    pub value: String,
}

/// Delivery / funding countdown snapshot (SWAP / FUTURES / OPTION). OC: `context.deliveryTimer`.
#[derive(Debug, Clone, PartialEq)]
pub struct KLineDeliveryTimer {
    pub remaining_seconds: i64,
    /// e.g. "02:15:30"
    pub time_string: String,
}

/// A single depth level (price + size).
#[derive(Debug, Clone, PartialEq)]
pub struct KLineDepthLevel {
    pub price: String,
    pub size: String,
}

/// Depth snapshot (asks + bids).
#[derive(Debug, Clone, PartialEq)]
pub struct KLineDepthSnapshot {
    /// Ask levels sorted by price ascending (best ask first).
    pub asks: Vec<KLineDepthLevel>,
    /// Bid levels sorted by price descending (best bid first).
    pub bids: Vec<KLineDepthLevel>,
    pub ts: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct KLineExploreBotsHalfScreenState {
    pub is_initialized: bool,
    pub is_loading: bool,
    pub init_error: Option<String>,
    pub inst_id: String,
    pub biz_type: String,
    pub inst_family: String,
    pub base_ccy: String,
    pub quote_ccy: String,
    pub coin_name: String,
    pub tick_sz: String,
    pub ct_val: String,
    pub is_option: bool,
    /// Display title (e.g. "BTC/USDT")
    pub inst_title: String,
    /// Amount unit for chart Y-axis (e.g. "BTC", "contracts")
    pub amount_unit: String,
    /// Quote symbol for price display (e.g. "USDT")
    pub quote_symbol: String,
    /// Price title unit for chart (e.g. "USDT", "USD")
    pub price_title_unit: String,
    pub ticker_version: u64,
    pub candle_version: u64,
    pub candle_update_type: KLineExploreBotsHalfScreenCandleUpdateType,
    pub candle_loading: bool,
    pub candle_error: Option<String>,
    pub finished_first_candle_load: bool,
    pub has_more_history: bool,
    pub current_period: String,
    /// True when stock-split adjustment is on (non-empty and != "none"). Mirrors
    /// kline_half_screen: native gates the effectiveTimes side-car and the adjusted
    /// realtime-merge reaction on this flag instead of re-deriving the adjustment.
    pub is_stock_adjusted: bool,
    /// Bars accepted this WS push (cleared on History / start_candle).
    pub realtime_batch: Vec<KlineQuoteCandleVo>,
    /// Bumps once per realtime batch AND once per load_more (MoreHistory) so a
    /// coalesced MoreHistory forces the next realtime tick into a full reload
    /// (seq gap). Reset to 0 on History / start_candle.
    pub realtime_seq: u64,
    /// Total bars cached after the merge. Native reconciles its local count.
    pub candle_count: u64,
    /// OFF (lean-cache) only: the History/MoreHistory page pushed to native when
    /// Rust does not keep the cache. ON: always empty (native pulls getCandles).
    /// MUST be reset to empty on every realtime bump (lean-cache design §4.4).
    pub lean_history_push: Vec<KlineQuoteCandleVo>,
    /// Earliest cached bar ts (pagination cursor). OFF reads this for load_more's
    /// `after` since the Rust cache is empty. Maintained on History / load_more.
    pub oldest_ts: String,
    pub digit: u32,
    pub data_source_type: u64,
    pub is_simulator: bool,
}

/// Price formatting result. OC: `[cc kl_formatGroupZero:]` and similar helpers.
#[derive(Debug, Clone, PartialEq)]
pub struct KLineFormattedPrice {
    /// Raw numeric string (no grouping separators)
    pub raw_value: String,
    /// Formatted string (with symbol/precision, no grouping — grouping done by native layer)
    pub formatted_value: String,
    pub precision: u32,
}

/// Formatted risk reserve amount. OC: `OKTDigitConvertor.convert(amount)`.
#[derive(Debug, Clone, PartialEq)]
pub struct KLineFormattedRiskReserve {
    /// Rounded value string (no suffix).
    pub rounded_value: String,
    /// Rounded value with suffix (e.g. "1,234.56 USDT").
    pub suffix_value: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct KLineFormulaState {
    pub is_initialized: bool,
    pub is_loading: bool,
    pub init_error: Option<String>,
    pub formula: String,
    pub formula_coins: String,
    pub formula_title: String,
    pub candle_version: u64,
    pub indicator_subscription_id: u32,
    pub candle_revision: u64,
    pub candle_update_type: FormulaCandleUpdateType,
    pub candle_loading: bool,
    pub candle_error: Option<String>,
    pub finished_first_candle_load: bool,
    pub has_more_history: bool,
    pub current_period: String,
    /// Bars accepted as tail-only this WS push (empty on forced-reload / History).
    pub realtime_batch: Vec<KlineQuoteCandleVo>,
    /// Bumps +1 on a tail-only realtime push, +1 on load_more (MoreHistory),
    /// and +2 on a non-tail-expressible push (forced full reload). Reset to 0
    /// on History / start_candle.
    pub realtime_seq: u64,
    /// Total bars cached after the merge. Native reconciles its local count.
    pub candle_count: u64,
    /// Legacy transport field retained for UniFFI compatibility. CandleStore is
    /// now always authoritative, so this stays empty.
    pub lean_history_push: Vec<KlineQuoteCandleVo>,
    /// Earliest cached bar ts, retained as a diagnostic/compatibility cursor.
    /// Pagination reads the CandleStore directly.
    pub oldest_ts: String,
    pub is_simulator: bool,
}

/// Funding rate snapshot (SWAP only). OC: `context.fundingRate`.
#[derive(Debug, Clone, PartialEq)]
pub struct KLineFundingRate {
    pub funding_rate: String,
    pub realized_rate: String,
    /// Next funding time (ms timestamp)
    pub funding_time: String,
    pub next_funding_rate: String,
    pub min_funding_rate: String,
    pub max_funding_rate: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct KLineHalfScreenState {
    pub is_initialized: bool,
    pub is_loading: bool,
    pub init_error: Option<String>,
    pub inst_id: String,
    pub biz_type: String,
    pub inst_family: String,
    pub base_ccy: String,
    pub quote_ccy: String,
    pub coin_name: String,
    pub tick_sz: String,
    pub ct_val: String,
    pub is_option: bool,
    pub inst_title: String,
    pub amount_unit: String,
    pub quote_symbol: String,
    pub price_title_unit: String,
    pub ticker_version: u64,
    pub candle_version: u64,
    /// Identity fencing indicator results across coin/period/price-mode restarts.
    pub indicator_subscription_id: u32,
    /// Exact CandleStore revision consumed by IndicatorRuntime.
    pub candle_revision: u64,
    pub candle_update_type: HalfScreenCandleUpdateType,
    pub candle_loading: bool,
    pub candle_error: Option<String>,
    pub finished_first_candle_load: bool,
    pub has_more_history: bool,
    pub current_period: String,
    /// True when stock-split adjustment is on (non-empty and != "none"). Mirrors
    /// `KlineCandlesState::is_stock_adjusted` so native can drive an "adjusted"
    /// indicator badge without re-querying the raw `adjustment` field.
    pub is_stock_adjusted: bool,
    /// Active price mode / candle source: "kline" (Last) | "mark" | "index".
    /// Drives `kline_type` selection for candle + load_more requests, volume
    /// visibility, and order/position/asset overlay suppression (only Default
    /// shows overlay). Initialized by biz (option → "mark", else "kline").
    pub price_type: String,
    /// Bars accepted this WS push (cleared on History / start_candle).
    pub realtime_batch: Vec<KlineQuoteCandleVo>,
    /// Bumps once per pushed realtime batch AND once per load_more
    /// (MoreHistory). load_more participates in the sequence so that if a
    /// MoreHistory is coalesced away (watch keeps only the latest value), the
    /// next realtime tick sees a seq gap and falls back to a full reload —
    /// preventing the prepended history from being silently missed. Reset to
    /// 0 on History / start_candle.
    pub realtime_seq: u64,
    /// Total bars cached after the merge. Native reconciles its local count.
    pub candle_count: u64,
    /// Compatibility field retained for older generated bindings. CandleStore is
    /// now always authoritative, so new clients pull structural updates through
    /// `get_candles()` and this field remains empty.
    pub lean_history_push: Vec<KlineQuoteCandleVo>,
    /// Compatibility pagination cursor. New code reads the head directly from
    /// CandleStore; this remains populated for older clients and empty-store fallback.
    pub oldest_ts: String,
    pub order_position_version: u64,
    pub asset_version: u64,
    pub indicator_version: u64,
    pub is_auction_staging: bool,
    pub is_pre_pending_order_staging: bool,
    pub digit: u32,
    pub data_source_type: u64,
    pub is_simulator: bool,
    /// Whether the Last / Mark / Index chip is available for this instrument.
    /// Precomputed on the Rust side via `compute_price_type_availability`.
    pub price_type_switch_visible: bool,
    /// Whether the Mark option is selectable in the chip menu (false for USDS
    /// spot pairs whose lever-tag is set — they have no mark price).
    pub mark_price_supported: bool,
}

/// A single item in the head-pop (biz line switcher) dropdown.
/// 
/// OC: `HeadPopItemModel`.
#[derive(Debug, Clone, PartialEq)]
pub struct KLineHeadPopItem {
    /// Business line type string: `"SPOT"`, `"MARGIN"`, `"SWAP"`, `"FUTURES"`, `"OPTION"`.
    pub biz_type: String,
    /// Display label (e.g. `"Spot"`, `"Perpetual"`).
    pub label: String,
    /// Whether this biz line is currently selected.
    pub is_selected: bool,
    /// The destination instrument ID when switching to this biz line.
    /// Empty if the biz is not loaded or has no instruments.
    pub dest_inst_id: String,
    /// Whether this biz line is available (loaded and has instruments).
    pub is_available: bool,
}

/// Lightweight instrument ID record for list queries.
/// Full details can be fetched via `kline_get_instrument_info(inst_id)`.
#[derive(Debug, Clone, PartialEq)]
pub struct KLineInstrumentBrief {
    pub inst_id: String,
    pub inst_type: String,
    pub biz_type: String,
    pub base_ccy: String,
    pub quote_ccy: String,
    pub inst_family: String,
    pub is_pre_market: bool,
}

/// Instrument snapshot — merges `OKTInstrument` + `OKTInstrumentContext` metadata
/// so K-line can render UI without a second round-trip to core-trading.
#[derive(Debug, Clone, PartialEq)]
pub struct KLineInstrumentInfo {
    /// e.g. "BTC-USDT", "BTC-USD-SWAP"
    pub inst_id: String,
    /// `"SPOT"` / `"SWAP"` / `"FUTURES"` / `"OPTION"` / `"MARGIN"`
    pub inst_type: String,
    /// Business line type (may differ from inst_type, e.g. MARGIN's inst_type could be SPOT)
    pub biz_type: String,
    /// Base currency (instBaseCoin)
    pub base_ccy: String,
    /// Real trade symbol (`instrument.get_trade_symbol()`) — the coin-table lookup
    /// key (e.g. TradFi detection). `base_ccy` is the display symbol and may be
    /// remapped for UI; never use it for data lookups.
    pub trade_symbol: String,
    /// Quote currency (instQuoteCoin)
    pub quote_ccy: String,
    /// e.g. "BTC-USD" for derivatives
    pub inst_family: String,
    /// Display name (instTitle)
    pub inst_title: String,
    /// Inverse/coin-margined contract (instrument.isNegative)
    pub is_negative: bool,
    /// USDT-margined contract
    pub is_positive: bool,
    /// Supports leverage (instrument.isSupportLever, for 10x spot)
    pub is_support_lever: bool,
    /// Minimum price increment
    pub tick_sz: String,
    /// Minimum order quantity increment
    pub lot_sz: String,
    pub ct_val: String,
    pub ct_mult: String,
    /// this_week / next_week / quarter / bi_quarter
    pub alias: String,
    /// Used by IndexDataCenter
    pub mk_uly: String,
    /// chargeConvertor.quotePrecision
    pub quote_precision: u32,
    /// tradeConvertor.targetUnitPrecision
    pub trade_precision: u32,
    /// chargeConvertor.quoteSymbol
    pub quote_symbol: String,
    /// tradeConvertor.targetUnit (amount unit)
    pub amount_unit: String,
    /// bizHandler.depthPriceTitleUnit
    pub price_title_unit: String,
    /// Leverage multiplier string (e.g. `"10"`). OC: `instrument.lever`.
    pub lever: String,
    /// Minimum order size. OC: `instrument.minSz`.
    pub min_sz: String,
    /// Whether the instrument is a pre-market pair. OC: `instrument.isPreMarket`.
    pub is_pre_market: bool,
    /// Settlement currency for derivatives. OC: `instrument.settleCcy`.
    pub settle_ccy: String,
    /// Coin display name (e.g. `"Bitcoin"`). OC: `instrument.coinName`.
    pub coin_name: String,
    /// Display symbol for quote currency. OC: `instrument.displayQuoteCcy`.
    pub display_quote_ccy: String,
    /// Contract value currency (e.g. `"BTC"` or `"USD"`). OC: `instrument.ctValCcy`.
    pub ct_val_ccy: String,
    /// Option strike price. OC: `instrument.stk`. Empty for non-option.
    pub stk: String,
    /// Option type: `"C"` (call) or `"P"` (put). OC: `instrument.optType`. Empty for non-option.
    pub opt_type: String,
    /// Option/Futures expiry time. OC: `instrument.expTime`. Empty for spot/swap.
    pub exp_time: String,
    /// Concatenated display symbol (e.g. `"BTCUSDT"`, `"BTCUSD CM"`).
    /// OC: `instrument.instSymbol` via `InstIdModel`.
    pub inst_symbol: String,
    /// Coin-level leverage (fallback when `lever` is empty, Spot only).
    /// OC: `instrument.coinLever`.
    pub coin_lever: String,
    /// Instrument family quote currency (e.g. `"USD"`, `"USDT"`).
    /// OC: `instrument.instFamilyQuoteCcy`.
    pub inst_family_quote_ccy: String,
    /// User-facing instrument identifier. OC: `instrument.displayId`.
    /// 
    /// For Spot, this is the configured `display_id` when present, otherwise
    /// `inst_id`. All other variants fall back to `inst_id`. Used by
    /// `OKKLineContractInfoVC` / `OKKLineContractInfoNativeDataSource` to feed
    /// `OKTInstIDModel`.
    pub display_id: String,
    /// Limit price parameters (Spot/Margin, Swap, Futures).
    pub lp_x1: String,
    pub lp_x2: String,
    pub lp_y1: String,
    pub lp_y2: String,
    pub lp_z1: String,
    pub lp_z2: String,
    /// Futures-only: delivery limit price (30 min before delivery).
    pub lp_c1: String,
    pub lp_c2: String,
    /// Spot-only limit price parameters.
    pub spot_lp_h1: String,
    pub spot_lp_h2: String,
    pub spot_lp_h3: String,
    pub spot_lp_j1: String,
    pub spot_lp_j2: String,
    /// Price limit status (OC: `OKTPriceLimitStatusType`). Spot only.
    pub price_limit_status: String,
    /// Max limit order value (USD). OC: `fullInstrument.maxLmtAmt`.
    pub max_lmt_amt: String,
    /// Max market order size. OC: `fullInstrument.maxMktSz`.
    pub max_mkt_sz: String,
    /// Max user position percent (Swap/Futures only). OC: `fullInstrument.userPositionMaxPercent`.
    pub user_position_max_percent: String,
    /// Max user position value USD (Swap/Futures only). OC: `fullInstrument.userPositionMaxValueUSD`.
    pub user_position_max_value_usd: String,
    /// Platform max total position value USD (Swap/Futures only).
    pub platform_side_position_max_value_usd: String,
    /// Whether the Last / Mark / Index price-type switch entry is available for
    /// this instrument. Derivatives are always true; spot is true only for
    /// margin-supported pairs or USDS pairs with a lever tag.
    pub price_type_switch_visible: bool,
    /// Whether the Mark price option is selectable in the price-type menu. Spot
    /// pairs whose USDS lever-tag is set have no mark price and set this to
    /// false; all other switchable instruments set this to true.
    pub mark_price_supported: bool,
}

/// Option summary snapshot (Greeks). Only meaningful for OPTION instruments.
#[derive(Debug, Clone, PartialEq)]
pub struct KLineOptionSummary {
    pub delta: String,
    pub gamma: String,
    pub vega: String,
    pub theta: String,
    /// Black-Scholes variants
    pub delta_bs: String,
    pub gamma_bs: String,
    pub vega_bs: String,
    pub theta_bs: String,
    pub lever: String,
    /// Implied vol used for mark price
    pub mark_vol: String,
}

/// Business gates used by K-line order write operations.
/// 
/// This is deliberately a separate UniFFI record: `RetailKLineOrder` is an
/// already-released record whose field list must remain frozen.
#[derive(Debug, Clone, PartialEq)]
pub struct KLineOrderWriteCapabilities {
    /// Whether the source order is reduce-only.
    pub is_reduce_only: bool,
    /// Whether a TP/SL chart row belongs to a standalone OCO strategy.
    pub is_standalone_oco: bool,
    /// Whether that OCO can use the current fixed-price single-side cancel API.
    pub supports_standalone_oco_single_side_cancel: bool,
    /// Whether the fixed trigger-order amend contract can preserve the order.
    pub supports_fixed_trigger_amend: bool,
}

/// Atomic snapshot of current open orders and positions.
#[derive(Debug, Clone, PartialEq)]
pub struct KLineOrdersAndPositions {
    pub orders: Vec<RetailKLineOrder>,
    pub positions: Vec<RetailKLinePosition>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct KLinePipState {
    pub is_initialized: bool,
    pub is_loading: bool,
    pub init_error: Option<String>,
    pub inst_id: String,
    pub biz_type: String,
    pub is_option: bool,
    pub data_source_type: u64,
    pub digit: u32,
    pub is_simulator: bool,
    /// Display name (instTitle) — PIP header title on the session path.
    pub inst_title: String,
    /// Structured localized title (core-trading `trade_title_utils`, same formatter as
    /// swiping_title/price_window). Native resolves the `DisplayString` segments and
    /// falls back to `inst_title` when a segment is unresolvable.
    pub trade_title: CTTradeTitle,
    /// Amount unit (tradeConvertor.targetUnit) — chart bridge model.
    pub amount_unit: String,
    /// Quote symbol (chargeConvertor.quoteSymbol) — chart bridge model turnover unit.
    pub quote_symbol: String,
    /// Instrument family (e.g. "BTC-USD") — native price-formatter convertor lookup.
    pub inst_family: String,
    /// USDT-margined (linear) contract — price-format fork for swap/futures.
    pub is_positive: bool,
    /// Bumped on every post-init session coin switch. Native uses this (not an
    /// inst_id diff) as the rebuild signal: state notifications keep only the
    /// latest value, so an init-completion frame and a switch flip can merge
    /// into one — a plain inst_id comparison would miss that and leave stale
    /// metadata forever. A monotonic seq survives frame merging.
    pub session_switch_seq: u64,
    pub ticker_version: u64,
    pub candle_version: u64,
    pub candle_update_type: PipCandleUpdateType,
    pub candle_loading: bool,
    pub candle_error: Option<String>,
    pub finished_first_candle_load: bool,
    pub current_period: String,
    /// True when stock-split adjustment is on (non-empty and != "none"). Mirrors
    /// kline_half_screen: native gates the effectiveTimes side-car and the adjusted
    /// realtime-merge reaction on this flag instead of re-deriving the adjustment.
    pub is_stock_adjusted: bool,
    pub realtime_batch: Vec<KlineQuoteCandleVo>,
    /// Bumps once per pushed batch; native detects a gap (state notifications
    /// keep only the latest value, so back-to-back batches can be missed) and
    /// falls back to a full pull. Reset to 0 on History / start_candle.
    pub realtime_seq: u64,
    /// Total bars cached after the merge (and cap drain). Native reconciles
    /// its local count against this after applying the batch.
    pub candle_count: u64,
    /// OFF (lean-cache) only: the History page pushed to native when Rust keeps
    /// no cache. ON: always empty (native pulls getCandles). MUST be reset to
    /// empty on every realtime bump (lean-cache design §4.4). pip has no load_more.
    pub lean_history_push: Vec<KlineQuoteCandleVo>,
}

/// Ticker display recipes (V1 `OKKLinePriceFormatterHelper` parity, native renders
/// via the shared recipe renderer).
/// 
/// - `last_price` mirrors `formatPrice:` — fiat-scaled + fiat-prefixed for inverse
/// contracts, quote-sign-prefixed for options, plain otherwise.
/// - `mark_price` mirrors `formatGroupZero:` — fiat-scaled for inverse but NEVER
/// prefixed. Empty recipes render the native placeholder.
/// - `change` is the raw ratio passthrough (same tick as the recipes, so native
/// never reads price and change from two different snapshots); `has_ticker`
/// is false until the first snapshot exists — native skips the render then.
#[derive(Debug, Clone, PartialEq)]
pub struct KLinePipTickerRecipes {
    pub last_price: KlineFormatRecipe,
    pub mark_price: KlineFormatRecipe,
    pub change: String,
    pub has_ticker: bool,
}

/// One authoritative preparation result shared by the direct and confirmation
/// paths. The enum makes an invalid zero-input/two-input state unrepresentable.
#[derive(Debug, Clone, PartialEq)]
pub struct KLinePreparedOrderAmend {
    pub family: KLineOrderAmendFamily,
    pub submit: KLinePreparedOrderSubmit,
    pub amended_raw_data: String,
}

/// Result of validating one fresh K-line order snapshot.
#[derive(Debug, Clone, PartialEq)]
pub struct KLinePreparedOrderCancel {
    pub submit: KLineOrderCancelSubmit,
    pub confirmation: KLineOrderCancelConfirmation,
}

/// Risk reserve fund trend item. OC: `OKTRiskTrendItem`.
#[derive(Debug, Clone, PartialEq)]
pub struct KLineRiskTrendItem {
    pub amount: String,
    /// Unix timestamp in milliseconds.
    pub date: String,
}

/// UniFFI Record wrapper for Vec<KLineRiskTrendItem> (needed by declare_ok_rust_future).
#[derive(Debug, Clone, PartialEq)]
pub struct KLineRiskTrendItems {
    pub items: Vec<KLineRiskTrendItem>,
}

/// TradingView OHLCV candle after index-price conversion.
/// All numeric values are strings to preserve precision across FFI.
#[derive(Debug, Clone, PartialEq)]
pub struct KLineTVCandle {
    pub time: String,
    pub open: String,
    pub high: String,
    pub low: String,
    pub close: String,
    pub volume: String,
}

/// Ticker snapshot — real-time market data K-line needs from `OKTInstrumentContext`.
#[derive(Debug, Clone, PartialEq)]
pub struct KLineTickerSnapshot {
    pub last: String,
    pub high24h: String,
    pub low24h: String,
    /// Volume in contracts or base currency
    pub vol24h: String,
    /// Volume in quote currency
    pub vol_ccy24h: String,
    /// Change ratio as decimal (e.g. "0.05" = 5%)
    pub change: String,
    pub mark_price: String,
    pub index_price: String,
    pub open_interest_oi: String,
    /// context.coinUSDPrice (from CupTickers3s)
    pub coin_usd_price: String,
    /// context.estimatedPrice (Option only)
    pub estimated_price: String,
}

/// Public tier info item. OC: `OKTTier`.
#[derive(Debug, Clone, PartialEq)]
pub struct KLineTierItem {
    pub tier: String,
    pub min_amount: String,
    pub max_amount: String,
    pub max_leverage: String,
    pub imr: String,
    pub mmr: String,
}

/// UniFFI Record wrapper for Vec<KLineTierItem> (needed by declare_ok_rust_future).
#[derive(Debug, Clone, PartialEq)]
pub struct KLineTierItems {
    pub items: Vec<KLineTierItem>,
}

/// DTO for `OKTradeDataTypeModel` — describes which trading data types
/// K-line subscribes to and whether a forced refresh is needed.
#[derive(Debug, Clone, PartialEq)]
pub struct KLineTradeDataTypeModel {
    /// Main data type key, e.g. `OKTradeDataTypeOfInstGroup`
    pub data_type: String,
    pub sub_data_types: Vec<String>,
    /// Whether this change should force an immediate refresh
    pub is_force_refresh: bool,
}

/// User-level trading preferences that influence K-line display.
#[derive(Debug, Clone, PartialEq)]
pub struct KLineTradeSetting {
    /// `"0"` = Price-Adjusted (PA), `"1"` = Black-Scholes (BS)
    pub greek_unit: String,
    /// TM sub-account: hide recent transaction list
    pub is_tm_sub_account_need_hidden_transaction: bool,
    /// TM sub-account: hide current position panel
    pub is_tm_sub_account_need_hidden_cur_pos: bool,
    /// Trade unit for linear (USDT-margined) contracts
    pub linear_trade_unit: String,
    /// Trade unit for inverse (coin-margined) contracts
    pub invert_trade_unit: String,
    /// `0` = total cost, `1` = average cost
    pub spot_average_config: u32,
}

/// A single recent-trade item for the "recent trades" list on K-line page.
#[derive(Debug, Clone, PartialEq)]
pub struct KLineTransaction {
    pub px: String,
    pub sz: String,
    /// `"buy"` or `"sell"`
    pub side: String,
    /// Timestamp in milliseconds (string)
    pub ts: String,
}

/// Investment agency information
/// 
/// Represents an investment agency or venture capital firm.
#[derive(Debug, Clone, PartialEq)]
pub struct KlineAgencyInfo {
    /// Name of the investment agency
    pub name: String,
    /// Logo URL of the investment agency
    pub logo: String,
}

/// Inputs for one arbitrage `ensure` call — the selected (left) leg.
#[derive(Debug, Clone, PartialEq)]
pub struct KlineArbBottomParams {
    pub inst_id: String,
    pub inst_type: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct KlineArbBottomState {
    pub is_ready: bool,
    pub is_loading: bool,
    pub error: Option<String>,
    pub inst_id: String,
    pub inst_type: String,
    /// SPOT quick-jump icon (first).
    pub first_icon: ArbBottomIconPlan,
    /// MARGIN quick-jump icon (second).
    pub second_icon: ArbBottomIconPlan,
}

/// Base coin info data
/// 
/// Represents comprehensive information about a cryptocurrency.
/// This corresponds to OKKLineCoinBaseInfoModel in iOS.
#[derive(Debug, Clone, PartialEq)]
pub struct KlineBaseCoinInfoData {
    /// Hot rank (popularity ranking)
    pub hot_rank: String,
    /// Market cap percentage of total crypto market
    pub mar_cap_per: String,
    /// Percentage of users who favorited this coin
    pub sel_num_per: String,
    /// Market cap rank
    pub market_cap_rank: String,
    /// Volume to market cap ratio
    pub volume_market_cap_ratio: String,
    /// All-time high price
    pub his_high_price: String,
    /// Timestamp of all-time high price
    pub his_high_price_ts: String,
    /// All-time low price
    pub his_low_price: String,
    /// Timestamp of all-time low price
    pub his_low_price_ts: String,
    /// Public blockchain (may be empty if multiple or none)
    pub pub_chain: String,
    /// Maximum supply
    pub max_sup: String,
    /// Maximum supply market value
    pub max_sup_mar: String,
    /// Whether token unlock information is supported
    pub support_token_unlock: bool,
    /// Coin introduction/description
    pub introduce: String,
    /// Coin icon URL
    pub icon: String,
    /// Full name of the coin
    pub full_name: String,
    /// Project symbol (e.g., "BTC")
    pub project: String,
    /// Token allocation information
    pub coin_allot: Vec<KlineCoinAllotInfo>,
    /// Sector/category tags
    pub selector: Vec<String>,
    /// Investment agencies/VCs
    pub agency: Vec<KlineAgencyInfo>,
    /// Social media links (official website, whitepaper, github, twitter, etc.)
    pub social_media_list: Vec<KlineSocialMediaInfo>,
    /// Team members
    pub members: Vec<KlineCoinMemberInfo>,
    /// Total supply (new field from coin-info API, FR-1)
    pub total_supply: String,
    /// Fully diluted valuation from CoinGecko (new field from coin-info API, FR-1)
    pub fdv_coingecko: String,
}

/// Base coin info request parameters
/// 
/// This struct represents the input parameters for fetching base coin information.
/// URL: /priapi/v5/rubik/public/coin-info
/// 
/// # Fields
/// 
/// - `ccy`: Currency symbol (e.g., "BTC", "ETH")
/// - `cache_expire_seconds`: HTTP cache expiration time in seconds (0 = no cache)
#[derive(Debug, Clone, PartialEq)]
pub struct KlineBaseCoinInfoReq {
    /// Currency symbol (e.g., "BTC", "ETH")
    pub ccy: String,
    /// HTTP cache expiration time in seconds (0 = no cache)
    pub cache_expire_seconds: u64,
}

/// Base coin info result
/// 
/// The response structure containing base coin information.
#[derive(Debug, Clone, PartialEq)]
pub struct KlineBaseCoinInfoResult {
    /// Base coin info data (single item, not an array)
    pub data: Option<KlineBaseCoinInfoData>,
}

/// Request parameters for batch tickers API
/// 
/// URL: /priapi/v5/market/mult-tickers
#[derive(Debug, Clone, PartialEq)]
pub struct KlineBatchTickersReq {
    /// Instrument IDs (will be joined with comma when making request)
    pub inst_ids: Vec<String>,
    /// Cache expiration time in seconds (0 = no cache)
    pub cache_expire_seconds: u64,
}

/// Result wrapper for batch tickers API response
#[derive(Debug, Clone, PartialEq)]
pub struct KlineBatchTickersResult {
    /// List of ticker data
    pub data: Vec<KlineTickerData>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct KlineBigDataInstMeta {
    /// `getTradeSymbol()` — inst_family/base_ccy derived on every inst type (for
    /// inverse contracts the value equals the settle coin because settle == base).
    pub trade_symbol: String,
    /// `getDisplayTradeSymbol()` — spot with a displayId returns
    /// displayBaseCcy; otherwise falls back to `getTradeSymbol()`.
    pub display_trade_symbol: String,
    /// `getQuoteSymbol()` — non-empty unlike `KLineInstrumentInfo.quote_symbol`.
    pub quote_symbol: String,
    /// `getDisplayQuoteSymbol()`.
    pub display_quote_symbol: String,
    /// `getInstFamily()`.
    pub inst_family: String,
    /// `isPositiveContract()` — false for SPOT/MARGIN; only meaningful for
    /// SWAP/FUTURES/OPTION.
    pub is_positive_contract: bool,
    /// `biz.getPriceDigit(instFamily)` parsed as u8, defaults to 2 on parse
    /// failure or missing biz. Native uses a `-1` sentinel when empty.
    pub price_digit: u8,
    /// Index underlying used by basis candles. Mirrors Native `instrument.mkUly`;
    /// when the server omits `uly`, falls back to the parsed family underlying.
    pub mk_uly: String,
    /// Spot/margin only (`false` for every other inst type). Mirrors iOS
    /// `OKTInstrument.isSupportLever` (`lever.doubleValue > 0`): float parse, NOT
    /// `check_margin_enable`'s `parse::<i32>` (which rejects non-integer strings).
    pub is_support_lever: bool,
    /// Explicit readiness bit (design §2.2): true iff the instrument was actually
    /// found (cache hit or hydrated). Hosts MUST gate on this — NOT on "symbol
    /// fields non-empty" (a legitimately resolved instrument with an abnormal
    /// inst_family can have empty derived symbols; legacy still renders it).
    pub resolved: bool,
}

/// Inputs for one `ensure` call. `is_support_bot` is a native-side config /
/// feature flag (`tradeConfig.isSupportBot()` plus e2e override) that Rust
/// cannot resolve, so it is passed in.
#[derive(Debug, Clone, PartialEq)]
pub struct KlineBottomParams {
    pub inst_id: String,
    pub inst_type: String,
    pub is_support_bot: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct KlineBottomState {
    pub is_ready: bool,
    pub is_loading: bool,
    pub error: Option<String>,
    pub inst_id: String,
    pub inst_type: String,
    /// New-coin lifecycle status (drives the new-coin tab / countdown UI).
    pub coin_status: CoinStatus,
    /// `resetFirstIcon`'s isNewCoin (countdown OR auction OR pre-quote).
    pub is_new_coin: bool,
    /// `isSupportRemind`: price-alert icon visibility.
    pub support_remind: bool,
    /// Whether the bottom strategy/biz icons are interactive (normal coin only).
    pub bottom_clickable: bool,
    /// Bottom second icon (spot / perp) display + routing.
    pub second_icon: SecondIconPlan,
    /// Bottom first icon (strategy / bot) display + routing.
    pub first_icon: FirstIconPlan,
}

#[derive(Debug, Clone, PartialEq)]
pub struct KlineBuySellPointsState {
    pub points: Vec<BuySellPointDisplay>,
}

/// Campaign list data
/// 
/// Represents a campaign list with its items.
/// Corresponds to OKMKLineOverviewCampaignListModel in iOS.
#[derive(Debug, Clone, PartialEq)]
pub struct KlineCampaignListData {
    /// List of campaign items
    pub campaign: Vec<KlineCampaignListItemData>,
}

/// Campaign list item
/// 
/// Represents a single campaign in the list.
/// Corresponds to OKMKLineOverviewCampaignListItemModel in iOS.
#[derive(Debug, Clone, PartialEq)]
pub struct KlineCampaignListItemData {
    /// Activity ID
    pub active_id: String,
    /// Activity URL slug
    pub url_slug: String,
    /// Activity language
    pub language: String,
    /// SEO title
    pub seo_title: String,
    /// SEO description
    pub seo_description: String,
}

/// Campaign list request parameters
/// 
/// # Fields
/// 
/// - `ccy`: Currency symbol (e.g., "BTC", "ETH")
/// - `location_code`: Location code (default: "over_view")
/// - `inst_name`: Instrument name
/// - `biz_type`: Business type
/// - `cache_expire_seconds`: HTTP cache expiration time in seconds (0 = no cache)
#[derive(Debug, Clone, PartialEq)]
pub struct KlineCampaignListReq {
    /// Currency symbol (e.g., "BTC", "ETH")
    pub ccy: String,
    /// Location code (default: "over_view")
    pub location_code: String,
    /// Instrument name
    pub inst_name: String,
    /// Business type
    pub biz_type: String,
    /// HTTP cache expiration time in seconds (0 = no cache)
    pub cache_expire_seconds: u64,
}

/// Campaign list result
/// 
/// The response structure containing campaign list data.
/// API returns an array of OKMKLineOverviewCampaignListModel.
#[derive(Debug, Clone, PartialEq)]
pub struct KlineCampaignListResult {
    /// List of campaign data
    pub data: Vec<KlineCampaignListData>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct KlineCandlesState {
    pub subscription_id: u32,
    pub candles_list: Vec<KlineQuoteCandleVo>,
    pub candle_revision: u64,
    /// Kind of the most recent change to `candles_list`. Native uses this
    /// to pick the matching chart-lib API instead of inferring from list shape.
    pub last_update_kind: CandlesUpdateKind,
    pub current_bar: String,
    pub current_price_type: String,
    pub is_stock_adjusted: bool,
    pub is_loading: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct KlineCexStockHoursState {
    /// Monotonic id of the latest subscribe; every async write drops itself if
    /// stale so a rapid re-subscribe can't be clobbered by an old fetch/tick.
    pub subscription_id: u32,
    /// Whether the target renders as a stock. `false` hides the tag (fetch
    /// error, non-stock instId, or empty schedule).
    pub is_stock: bool,
    /// Fully rendered sheet content; `None` until the first successful fetch.
    pub vo: Option<StockHoursVo>,
    pub is_loading: bool,
    pub error: Option<String>,
}

/// Token allocation information
/// 
/// Represents the distribution of tokens to different parties.
#[derive(Debug, Clone, PartialEq)]
pub struct KlineCoinAllotInfo {
    /// Name of the allocation category (e.g., "Team", "Investors", "Community")
    pub name: String,
    /// Percentage of total supply allocated
    pub percent: String,
}

/// Coin data info data
/// 
/// Represents market data information about a cryptocurrency.
/// Corresponds to OKGMCoinInfoModel in iOS.
#[derive(Debug, Clone, PartialEq)]
pub struct KlineCoinDataInfoData {
    /// Circulating supply
    pub flow_total: String,
    /// All-time high price
    pub high_price: String,
    /// Issue/ICO price
    pub issue_price: String,
    /// Issue time/date
    pub issue_time: String,
    /// Market capitalization
    pub market_cap: String,
    /// Token name
    pub name: String,
    /// 24h trading volume
    pub vol: String,
    /// Hold time
    pub hold_time: String,
    /// Fully diluted valuation
    pub fully_diluted_valuation: String,
}

/// Coin data info request parameters
/// 
/// # Fields
/// 
/// - `project_name`: Project name (e.g., "BTC", "ETH")
/// - `cache_expire_seconds`: HTTP cache expiration time in seconds (0 = no cache)
#[derive(Debug, Clone, PartialEq)]
pub struct KlineCoinDataInfoReq {
    /// Project name (e.g., "BTC", "ETH")
    pub project_name: String,
    /// HTTP cache expiration time in seconds (0 = no cache)
    pub cache_expire_seconds: u64,
}

/// Coin data info result
/// 
/// The response structure containing coin data information.
#[derive(Debug, Clone, PartialEq)]
pub struct KlineCoinDataInfoResult {
    /// Coin data info, None if not found
    pub data: Option<KlineCoinDataInfoData>,
}

/// Coin detail data
/// 
/// Represents detailed information about a cryptocurrency project.
/// This corresponds to OKGMCoinDetailModel in iOS.
#[derive(Debug, Clone, PartialEq)]
pub struct KlineCoinDetailData {
    /// Block explorer URL
    pub block_browser: String,
    /// Brief description
    pub brief: String,
    /// Classification/category (e.g., "decentralized_storage")
    pub classification: String,
    /// Currency ID
    pub currency_id: i64,
    /// Coin introduction/description
    pub introduce: String,
    /// Coin icon URL
    pub icon: String,
    /// Full name of the coin
    pub full_name: String,
    /// Total issue amount
    pub issue_total: String,
    /// Issue time (timestamp in milliseconds, e.g., 1225382400000)
    pub issue_time: String,
    /// Investor information
    pub investor: String,
    /// Investors information
    pub investors: String,
    /// Project symbol (e.g., "BTC")
    pub project: String,
    /// Project ID
    pub project_id: i64,
    /// Status
    pub status: i64,
    /// Circulating supply
    pub flow_total: String,
    /// Facebook URL
    pub facebook: String,
    /// GitHub URL
    pub github: String,
    /// Official website URL
    pub official_url: String,
    /// Telegram URL
    pub telegram: String,
    /// Twitter URL
    pub twitter: String,
    /// WeChat URL
    pub we_chat: String,
    /// Whitepaper URL
    pub white_paper: String,
    /// Whether the coin is in user's collection/favorites
    pub is_collection: bool,
    /// Team members
    pub members: Vec<KlineCoinDetailMemberInfo>,
    /// Video URLs
    pub videos: Vec<String>,
}

/// Coin detail member information
/// 
/// Represents a team member (corresponds to OKGMCoinMember).
#[derive(Debug, Clone, PartialEq)]
pub struct KlineCoinDetailMemberInfo {
    /// Member's avatar/profile image URL
    pub head_image: String,
    /// Member's introduction/bio
    pub introduce: String,
    /// Member's name
    pub name: String,
    /// Member's position/role
    pub position: String,
}

/// Coin detail request parameters
/// 
/// This struct represents the input parameters for fetching coin detail information.
/// URL: /v2/support/info/announce/detail
/// 
/// # Fields
/// 
/// - `project_name`: Project name (e.g., "BTC", "ETH")
/// - `cache_expire_seconds`: HTTP cache expiration time in seconds (0 = no cache)
#[derive(Debug, Clone, PartialEq)]
pub struct KlineCoinDetailReq {
    /// Project name (e.g., "BTC", "ETH")
    pub project_name: String,
    /// HTTP cache expiration time in seconds (0 = no cache)
    pub cache_expire_seconds: u64,
}

/// Coin detail result
/// 
/// The response structure containing coin detail information.
#[derive(Debug, Clone, PartialEq)]
pub struct KlineCoinDetailResult {
    /// Coin detail data (single item)
    pub data: Option<KlineCoinDetailData>,
}

/// Coin member information
/// 
/// Represents a team member or contributor.
#[derive(Debug, Clone, PartialEq)]
pub struct KlineCoinMemberInfo {
    /// Name of the member
    pub name: String,
    /// Title or role of the member
    pub title: String,
    /// Avatar URL of the member
    pub avatar: String,
}

/// A single formatted depth level for native UI rendering.
/// Maps from core-trading's `OKTFormattedDepthItem`.
#[derive(Debug, Clone, PartialEq)]
pub struct KlineDepthDisplayItem {
    /// Raw decimal price (native applies thousands grouping).
    pub price: String,
    /// Raw decimal amount (native applies K/M/B/T compact).
    pub amount: String,
    /// Raw price for click-fill
    pub origin_price: f64,
    /// Raw amount for bar width calculation
    pub origin_amount: f64,
    /// Cumulative amount up to and including this price level.
    /// Used by the depth chart (MapDepthView) for the area-curve total.
    pub sum_amount: f64,
    /// Bar fill ratio 0.0..1.0
    pub ratio: f64,
    /// Decimal precision for amount display.
    /// SPOT/MARGIN: amount<1000 -> max(2, min_size_scale); else 3
    /// Derivatives: from core-trading's get_depth_amt_digit
    pub amount_digit: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct KlineDepthInteractorState {
    pub bids: Vec<KlineDepthDisplayItem>,
    pub asks: Vec<KlineDepthDisplayItem>,
    pub ladders: Vec<String>,
    pub current_ladder: String,
    pub price_unit: String,
    pub amount_unit: String,
    pub is_loading: bool,
    pub is_empty: bool,
    pub error: Option<String>,
    /// Coordinate-axis price scale for native: inverse-contract fiat rate,
    /// otherwise 1.0. Native multiplies the current-price line by this before
    /// display so it matches the converted bid/ask levels.
    pub price_scale: f64,
    /// Price decimal precision for the coordinate-axis text. -1 when unknown,
    /// in which case native falls back to its default fraction handling.
    pub price_digit: i32,
}

/// Hypothetical PnL for a chart drag at the current dragged price.
#[derive(Debug, Clone, PartialEq)]
pub struct KlineDragPnlVo {
    /// `true` when the input was complete and the formulas could be
    /// evaluated; `false` when the drag target carries no PnL preview
    /// (main-order drag, or the dragged session is empty).
    pub has_value: bool,
    /// Formatted PnL amount with sign prefix (e.g. "+12.34", "-5.67").
    /// Empty when `has_value` is false.
    pub pnl: String,
    /// Formatted PnL ratio with sign prefix and percent suffix
    /// (e.g. "+0.82%", "-5.10%"). Empty when `has_value` is false.
    pub pnl_ratio: String,
    /// Unit / currency label paired with `pnl` (e.g. "USDT", "BTC").
    /// Empty when no unit is applicable.
    pub pnl_sign: String,
    /// Convenience flag so the chart can pick the up / down colour
    /// without re-parsing the formatted string.
    pub is_positive: bool,
    /// The dragged price mapped to order-base terms (V1
    /// `getOrderUPrice` / `OKTChargeConvertor.toBaseForOrder`): identity for
    /// linear / spot / margin, `price ÷ rate` for inverse contracts. Native
    /// writes this onto `bean.uPrice` so the modify dialog pre-fills the
    /// dragged main-order price or TP / SL trigger. Populated whenever the
    /// session carries a current price, independently of `has_value` — a
    /// main-order drag (no PnL preview) still needs this to pre-fill the
    /// amend dialog. Only genuinely empty when the drag session itself has
    /// no current price (e.g. the default/uninitialized VO).
    pub u_price: String,
}

/// The one event that fills the K-line page's earnings banner slot.
/// 
/// The banner shows at most one event at a time (PRD entry 2), so picking it is
/// a business rule and lives here rather than in each native client.
#[derive(Debug, Clone, PartialEq)]
pub struct KlineEarningsBannerVo {
    /// The chosen event, whole — the banner copy reads several of its fields.
    pub event: KlineEarningsEventVo,
    /// `true` once the actual values have landed: picks the published copy
    /// ([`KlineEarningsEventVo::summary_text`]) over the upcoming one, and the
    /// tap target that goes with it.
    pub is_published: bool,
    /// Whether the calendar button is offered. Only estimated / confirmed
    /// events can be added to the system calendar, so a published or canceled
    /// event has no button.
    pub can_subscribe: bool,
    /// Display abbreviation for [`KlineEarningsEventVo::market_timezone`], e.g.
    /// `ET` — the upcoming copy reads "MM/DD (ET) …". Empty when the market is
    /// not one we have an abbreviation for, and native then drops the
    /// parenthetical rather than printing a raw IANA id.
    /// 
    /// Derived here rather than on each client: it is a pure lookup with no
    /// locale involved, so iOS and Android must not each keep their own table.
    /// Generic (`ET`), not DST-specific (`EST`/`EDT`) — that is what the design
    /// asks for.
    pub market_timezone_abbr: String,
}

/// What native supplies for one instrument's K-line earnings request.
/// 
/// Deliberately only the page-scoped facts. The request window and the UTC
/// offset are **not** here: both are data rules, resolved Rust-side in
/// [`crate::core::earnings_calendar`] so the two platforms cannot drift and so
/// the offset is read from one place (the user setting) rather than mirrored.
#[derive(Debug, Clone, PartialEq)]
pub struct KlineEarningsEventReq {
    /// OKX instrument the chart is showing, e.g. `XTSLA-USDT`. BE resolves it to
    /// the canonical stock identity through the authoritative mapping; an unknown
    /// instrument is a parameter error, so never derive it by trimming a prefix.
    pub inst_id: String,
    /// Paired with [`Self::inst_id`]. Earnings currently covers `SPOT` / `SWAP`.
    pub inst_type: String,
    /// HTTP cache TTL in seconds; `0` bypasses the cache.
    pub cache_expire_seconds: u64,
}

/// Result of one earnings-event fetch, in the order BE returned (`date`, then
/// `calendar_id`).
#[derive(Debug, Clone, PartialEq)]
pub struct KlineEarningsEventResult {
    pub events: Vec<KlineEarningsEventVo>,
    /// `None` when no event is inside a banner window.
    pub banner: Option<KlineEarningsBannerVo>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct KlineEarningsEventState {
    /// Events from the most recent fetch, in the order BE returned them (`date`,
    /// then `calendar_id`). Native draws each as its own chart marker — they are
    /// never folded into the economic-event group count.
    pub raw_events: Vec<KlineEarningsEventVo>,
    /// The one event filling the page's banner slot, `None` when none is inside
    /// a banner window. Native renders it; the rule that picks it lives in
    /// `select_banner`.
    pub banner: Option<KlineEarningsBannerVo>,
    pub is_loading: bool,
    pub error: Option<String>,
}

/// One earnings event placed on the K-line time axis.
#[derive(Debug, Clone, PartialEq)]
pub struct KlineEarningsEventVo {
    /// Stable per instrument + fiscal period + event type, e.g. `earnings:9823349901`.
    /// A re-schedule keeps the id and bumps [`Self::version`].
    pub calendar_id: String,
    /// Always `earnings` for this endpoint.
    pub category: String,
    pub country: String,
    /// **The K-line anchor**, Unix-ms as a string — where the icon is drawn.
    /// BE derives it from `earnings_date` + `release_session` + the market's IANA
    /// zone + the trading calendar, so it already accounts for DST, holidays and
    /// early closes. Not interchangeable with [`Self::earnings_date`].
    pub date: String,
    /// Always `earnings` for this endpoint.
    pub event: String,
    /// i18n key, e.g. `trading_calendar_earnings`.
    pub event_key: String,
    /// Display title, e.g. `Tesla FY2026 Q2 earnings`.
    pub event_name: String,
    /// Bumped whenever the same event is re-scheduled.
    pub version: String,
    /// Announcement date **in the listing market's local time**, e.g. `2026-08-10`.
    pub earnings_date: String,
    /// IANA zone of the listing market, e.g. `America/New_York`.
    pub market_timezone: String,
    /// When in the session the report lands: `BEFORE_MARKET_OPEN` / `DURING_MARKET`
    /// / `AFTER_MARKET_CLOSE` / `TBD`. This is the *scheduled slot*, not a live
    /// market-open/closed status.
    pub release_session: String,
    /// `ESTIMATED` / `CONFIRMED` / `CANCELED`.
    pub schedule_status: String,
    /// `PENDING` / `PUBLISHED`. Set by BE the first time an authoritative actual
    /// arrives — never inferred from the clock passing the scheduled time.
    pub publication_status: String,
    /// BE's verdict on whether this event owns the banner slot above the chart
    /// (PRD entry 2). **Authoritative** — the window and priority rules moved to
    /// the backend, so native must not re-derive them from dates.
    /// 
    /// It does *not* subsume `schedule_status`: a canceled event can still carry
    /// `true`, so the cancel check stays on this side.
    pub show_in_above: bool,
    /// Whether analysis data is ready. Independent of [`Self::publication_status`].
    pub analyzed: bool,
    /// Derived from [`Self::publication_status`] by the use case, never read off
    /// the wire (`serde(skip)`).
    /// 
    /// Exists so the "what counts as published" rule stays in one place: the
    /// chart's published/upcoming marker filter and the banner both need it, and
    /// native must not re-compare the status string for itself.
    pub is_published: bool,
    /// Accounting period, e.g. `FY2026 Q2`. `None` when BE could not align it.
    pub fiscal_period: Option<String>,
    /// Reported currency **of this fiscal period**, e.g. `USD`.
    pub currency: String,
    pub currency_symbol: String,
    /// Raw EPS estimate (decimal verbatim).
    pub eps_estimated: Option<String>,
    /// Raw EPS actual (decimal verbatim).
    pub eps_actual: Option<String>,
    /// BE-formatted EPS estimate, e.g. `$1.68`. Display this as-is.
    pub eps_estimated_display: Option<String>,
    /// BE-formatted EPS actual, e.g. `$1.72`. Display this as-is.
    pub eps_actual_display: Option<String>,
    /// `BEAT` / `MISSED` / `MET`, or `None` when either side is missing.
    pub eps_beat_miss: Option<String>,
    /// Raw revenue estimate (decimal verbatim).
    pub revenue_estimated: Option<String>,
    /// Raw revenue actual (decimal verbatim).
    pub revenue_actual: Option<String>,
    /// BE-formatted revenue estimate, e.g. `$107.4B`. Display this as-is.
    pub revenue_estimated_display: Option<String>,
    /// BE-formatted revenue actual, e.g. `$108.1B`. Display this as-is.
    pub revenue_actual_display: Option<String>,
    /// `BEAT` / `MISSED` / `MET`, or `None` when either side is missing.
    pub revenue_beat_miss: Option<String>,
    /// Event-level summary BE generates for the requested locale. `None` when
    /// there was nothing to summarise — never an empty string to paper over it.
    pub summary_text: Option<String>,
    /// Unix-ms of the first publication; frozen once set, `None` while pending.
    pub published_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct KlineEcoCalendarState {
    /// Raw events returned by the most recent fetch, in date-ascending order.
    /// Native converts each to a `KLineEventBean` via the existing extension
    /// and feeds them to `chartConfig.klineEventList`.
    pub raw_events: Vec<KlineEconEventVo>,
    pub is_loading: bool,
    pub error: Option<String>,
}

/// Economic calendar request parameters
/// 
/// # Time Range
/// 
/// The time range is defined by `before` (start time) and `after` (end time):
/// - `before`: Start of the time range (earlier time, must be LESS than `after`)
/// - `after`: End of the time range (later time, must be GREATER than `before`)
/// - Time range must not exceed 1 year
/// 
/// # Example Time Range
/// 
/// To query events from one year ago to now:
/// ```ignore
/// let now = 1735660800000_i64;  // Current time
/// let one_year_ago = now - (365 * 24 * 60 * 60 * 1000);  // One year ago
/// 
/// let req = KlineEconEventReq {
/// before: one_year_ago.to_string(),  // Start: one year ago (earlier)
/// after: now.to_string(),            // End: now (later)
/// utc_zone: "0".to_string(),
/// cache_expire_seconds: 60,
/// };
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct KlineEconEventReq {
    /// Start timestamp of the time range (in milliseconds)
    /// 
    /// This represents the START of the query period (earlier time).
    /// Must be LESS than `after` (end time).
    /// 
    /// Example: "1704067200000" (one year ago)
    pub before: String,
    /// End timestamp of the time range (in milliseconds)
    /// 
    /// This represents the END of the query period (later time).
    /// Must be GREATER than `before` (start time).
    /// 
    /// Example: "1735660800000" (now)
    pub after: String,
    /// UTC timezone offset (e.g., "0" for UTC+0, "8" for UTC+8)
    pub utc_zone: String,
    /// HTTP cache expiration time in seconds
    /// 
    /// Controls how long the HTTP response should be cached:
    /// - `0`: No caching (always fetch fresh data from server)
    /// - `> 0`: Cache the response for the specified number of seconds
    /// 
    /// Default: 0 (no cache)
    /// 
    /// # Example
    /// ```ignore
    /// // Query events from one year ago to now, no cache
    /// let now = 1735660800000_i64;
    /// let one_year_ago = now - (365 * 24 * 60 * 60 * 1000);
    /// 
    /// let req = KlineEconEventReq {
    /// before: one_year_ago.to_string(),  // Start: one year ago
    /// after: now.to_string(),            // End: now
    /// utc_zone: "0".to_string(),
    /// cache_expire_seconds: 0,
    /// };
    /// 
    /// // Query events from one year ago to now, cache for 60 seconds
    /// let req = KlineEconEventReq {
    /// before: one_year_ago.to_string(),  // Start: one year ago
    /// after: now.to_string(),            // End: now
    /// utc_zone: "0".to_string(),
    /// cache_expire_seconds: 60,
    /// };
    /// ```
    pub cache_expire_seconds: u64,
}

/// Economic calendar result
#[derive(Debug, Clone, PartialEq)]
pub struct KlineEconEventResult {
    /// List of economic calendar events
    pub events: Vec<KlineEconEventVo>,
}

/// Economic calendar event VO
#[derive(Debug, Clone, PartialEq)]
pub struct KlineEconEventVo {
    /// Actual value (if available)
    pub actual: String,
    /// Calendar event ID
    pub calendar_id: String,
    /// Event category/type
    pub category: String,
    /// Country code or name
    pub country: String,
    /// Event date timestamp (in milliseconds)
    pub date: String,
    /// Event name
    pub event: String,
    /// Event key for localization
    pub event_key: String,
    /// Event name/title
    pub event_name: String,
    /// Forecast value (if available)
    pub forecast: String,
    /// Importance level ("1"=Low, "2"=Medium, "3"=High)
    pub importance: String,
    /// Previous value (if available)
    pub previous: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct KlineFavoriteState {
    /// Monotonic id of the latest `subscribe`. Every async write checks it and
    /// drops itself if stale, so a rapid re-subscribe (e.g. swiping coins) can't
    /// have a cancelled subscription's late `load_and_check` overwrite the new
    /// coin's favorite status (interactor-spec §J.1).
    pub subscription_id: u32,
    /// Raw `subscribe` inputs — the source of truth. `toggle()` rebuilds the
    /// `FavoriteItem` from these instead of a shadow struct field. All empty
    /// before the first `subscribe()`.
    pub inst_id: String,
    pub inst_type: String,
    pub ca: String,
    pub chain_id: String,
    /// DEX token symbol — required by the favorite `setting` API for DEX adds
    /// (empty for CeFi). Stored so `toggle()` can rebuild the full DEX item.
    pub token_symbol: String,
    pub is_favorite: bool,
    pub is_loading: bool,
    pub error: Option<String>,
    /// ExchangeOS DEX perp/spot identity (`subscribe_exos`) — the WHOLE key for that channel.
    /// Empty for CeFi / legacy DEX Swap. Appended last so existing native readers of this
    /// Record are unaffected.
    pub asset_id: String,
    /// `SPOT` / `PERP` for an ExchangeOS row; empty for CeFi / legacy DEX Swap (which
    /// normalizes to `SWAP` downstream). Selects the `FavoriteItem::DEX` shape on rebuild.
    pub dex_sub_type: String,
    /// ExchangeOS display name echoed back to the write endpoints as `name`
    /// (e.g. "BTC-PERP"); empty when the caller has none, letting the server backfill it.
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct KlineFlashNewsState {
    pub items: Vec<FlashNewsItemVo>,
    pub is_loading: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct KlineFormatRecipe {
    /// Raw numeric value as plain string (always uses "." as decimal,
    /// no grouping). `""` means no data — native should render a
    /// placeholder.
    pub value: String,
    /// Decimal precision for display.
    pub precision: u8,
    /// Display sign or unit (e.g. "$", "¥", "BTC", "USDT", "%").
    pub sign: String,
    /// Where to place the sign: "prefix" | "suffix" | "none".
    pub sign_position: String,
    /// Rounding mode hint: "down" | "up" | "half_up".
    pub rounding: String,
    /// Whether to keep trailing zeros.
    pub keep_trailing_zeros: bool,
    /// Whether to prepend `+` for positive values.
    pub prepend_plus: bool,
    /// Whether native should abbreviate (K/M/B and Chinese compact units).
    pub abbreviate: bool,
    /// Whether the value is already in percentage units (append `%`).
    pub is_percentage: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct KlineHeaderState {
    pub top_info: Option<HeadInfoVo>,
    pub multi_period_changes: Option<MultiPeriodChangeVo>,
    pub is_loading: bool,
    pub error: Option<String>,
}

/// Index ticker data
/// 
/// Contains index ticker information
#[derive(Debug, Clone, PartialEq)]
pub struct KlineIndexTickerData {
    /// Instrument ID (e.g., "BTC-USD")
    pub inst_id: String,
    /// Latest index price
    pub idx_px: String,
    /// 24h highest index price
    pub high_24h: String,
    /// 24h lowest index price
    pub low_24h: String,
    /// 24h opening index price
    pub open_24h: String,
    /// Timestamp in milliseconds
    pub ts: String,
    /// Start of day UTC+0
    pub sod_utc0: String,
    /// Start of day UTC+8
    pub sod_utc8: String,
    /// Icon URL
    pub icon: String,
    /// Full name
    pub full_name: String,
}

/// Request parameters for index tickers API
/// 
/// URL: /priapi/v5/market/index-tickers
#[derive(Debug, Clone, PartialEq)]
pub struct KlineIndexTickersReq {
    /// Quote currency (e.g., "USD", "USDT")
    pub quote_ccy: String,
    /// Instrument ID (e.g., "BTC-USD")
    pub inst_id: String,
    /// Cache expiration time in seconds (0 = no cache)
    pub cache_expire_seconds: u64,
}

/// Result wrapper for index tickers API response
#[derive(Debug, Clone, PartialEq)]
pub struct KlineIndexTickersResult {
    /// List of index ticker data
    pub data: Vec<KlineIndexTickerData>,
}

/// High-frequency precomputed indicator values for one CandleStore revision.
/// 
/// `payload` uses the versioned little-endian OKID protocol. Keeping the event
/// identity typed lets Native reject stale frames without parsing or allocating
/// the numeric body. The bytes lower to Kotlin `ByteArray`, avoiding a boxed
/// `List<Double>` for every output value.
#[derive(Debug, Clone, PartialEq)]
pub struct KlineIndicatorDataFrameVo {
    pub inst_id: String,
    pub inst_type: String,
    pub bar: String,
    pub price_type: String,
    pub candle_revision: u64,
    pub source_update_kind: CandlesUpdateKind,
    pub candle_count: u64,
    pub params_version: u64,
    pub is_full_result: bool,
    pub calculation_time_micros: u64,
    pub payload_version: u16,
    pub payload: Vec<u8>,
    pub warnings: Vec<String>,
}

/// Optional fill between two named output lines (currently used by BOLL).
#[derive(Debug, Clone, PartialEq)]
pub struct KlineIndicatorFillSchemaVo {
    pub upper_line_name: String,
    pub lower_line_name: String,
    pub color: String,
    pub color_block: String,
    pub alpha: f64,
    pub is_visible: bool,
}

/// Announces that Rust has committed a coherent full indicator ledger for one
/// candle generation. Numeric values are intentionally absent: Native uses
/// this identity to request only the chunks around its current viewport.
#[derive(Debug, Clone, PartialEq)]
pub struct KlineIndicatorGenerationVo {
    pub inst_id: String,
    pub inst_type: String,
    pub bar: String,
    pub price_type: String,
    pub candle_revision: u64,
    pub source_update_kind: CandlesUpdateKind,
    pub candle_count: u64,
    pub params_version: u64,
    pub mode: KlineIndicatorGenerationMode,
    /// Required cache revision for `PrependIncremental`; absent for a full
    /// rebase. Native falls back to a full window whenever this base is gone.
    pub base_candle_revision: Option<u64>,
    pub changed_ranges: Vec<KlineIndicatorLineRangeVo>,
    pub calculation_time_micros: u64,
    pub warnings: Vec<String>,
}

/// Absolute half-open value range changed for one stable schema line id.
/// The same record is also used by Native when it asks Rust for per-line
/// windows, keeping indicator formula knowledge out of the bridge layer.
#[derive(Debug, Clone, PartialEq)]
pub struct KlineIndicatorLineRangeVo {
    pub line_id: u32,
    pub start_index: u64,
    pub end_index: u64,
}

/// Low-frequency metadata for one precomputed indicator line.
#[derive(Debug, Clone, PartialEq)]
pub struct KlineIndicatorLineSchemaVo {
    /// Compact id used by the binary data-frame payload. Stable for one
    /// `params_version` and unique within the schema.
    pub line_id: u16,
    pub name: String,
    pub info_name: String,
    pub style: IndicatorOutputStyle,
    /// Rust-owned render configuration. Empty strings mean the chart theme
    /// fallback should be used.
    pub color: String,
    pub color_block: String,
    /// Logical dp width; Native performs the density conversion.
    pub line_width: f64,
    pub alpha: f64,
    pub is_visible: bool,
    pub color_role: IndicatorColorRole,
    /// Legend color is independent from the primitive color. This is required
    /// by mixed outputs such as colored Net L/S bars with a neutral shared label.
    pub info_color_role: IndicatorColorRole,
    pub include_in_info: bool,
    pub value_format: IndicatorValueFormat,
    pub info_unit: IndicatorInfoUnit,
    pub zero_is_no_data: bool,
}

/// Low-frequency metadata for one selected indicator instance.
#[derive(Debug, Clone, PartialEq)]
pub struct KlineIndicatorOutputSchemaVo {
    /// Stable within one chart session. Main/sub are intentionally part of the
    /// id because the same symbol can be installed in both areas.
    pub instance_id: String,
    /// Canonical calculator symbol owned by candle-support (for example `MA`,
    /// `SAR`, `SUPERTREND`, or `DONCHIAN_CHANNEL`).
    pub symbol: String,
    pub is_main: bool,
    pub viewport_dependent: bool,
    pub backdrop_style: IndicatorBackdropStyle,
    pub availability: KlineIndicatorAvailability,
    pub lines: Vec<KlineIndicatorLineSchemaVo>,
    pub fills: Vec<KlineIndicatorFillSchemaVo>,
}

/// Low-frequency indicator structure. Numeric arrays are intentionally absent;
/// they are carried by `KlineIndicatorDataFrameVo.payload`.
#[derive(Debug, Clone, PartialEq)]
pub struct KlineIndicatorSchemaVo {
    pub params_version: u64,
    pub outputs: Vec<KlineIndicatorOutputSchemaVo>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct KlineInitState {
    /// `true` once `ensure_initialized_base_info` + `load_<biz>_biz` have
    /// both succeeded for the current `biz_type`.
    pub is_ready: bool,
    pub is_loading: bool,
    pub error: Option<String>,
    pub inst_id: String,
    pub biz_type: String,
    /// Call auction in progress (`getBizContinuousStatus == "1"`).
    pub auction_status: bool,
    /// Pre-quote listing active (`getBizPreQuoteStatus == "ACTIVE"`).
    pub pre_quote_active: bool,
    /// New-coin countdown active (`KlineUtilsManager.isNewCoin`): a countdown
    /// exists and is not an already-online type-0 countdown. Native portrait
    /// routing gates the new-coin overview page on this (`isNewCoinCountDownState`).
    /// The combined countdown-OR-auction-OR-pre-quote flag native uses for the
    /// pre-listing analytics tag is derived host-side from this plus
    /// `auction_status` / `pre_quote_active`.
    pub new_coin_countdown: bool,
    /// Delivery-futures alias (`this_week` / `quarter` / ...); empty otherwise.
    pub futures_alias: String,
    /// Instrument trade symbol (base ccy display), e.g. "BTC".
    pub trade_symbol: String,
    /// Display trade symbol (displayBaseCcy, empty→tradeSymbol). Amount/VOL unit source.
    pub display_trade_symbol: String,
    /// Instrument quote symbol, e.g. "USDT".
    pub quote_symbol: String,
    /// TradFi coin (`getTradeCoinInfo(trade_symbol).tradFi`).
    pub is_trade_fi: bool,
    /// Commodity coin (non-empty `getTradeCoinInfo(trade_symbol).commodityTag`).
    pub is_commodity: bool,
    /// Whether this instrument has an index sample (SPOT with margin, or
    /// SWAP / FUTURES with an underlying); gates the index-sample tab.
    pub is_support_index_sample: bool,
    /// Pre-market pair.
    pub is_pre_market: bool,
    /// User trade-unit setting for this inst type ("0" coin / "1" contract / "2" USDT).
    pub trade_unit: String,
    /// Recommended price display digits (`biz.get_price_digit(inst_family)`,
    /// same source as V1 `commonInitChartView`). Native feeds the chart
    /// Y-axis precision from this before the first candle frame renders.
    pub price_digit: String,
    /// Account level string ("1" Simple / "2" SCC / "3" MCC / "4" PM),
    /// from `trade_setting().get_account_level()`.
    pub account_level: String,
    /// Raw product classification from the instrument detail response.
    pub inst_category: String,
    /// Raw trading-area groups from the current instrument's coin metadata.
    pub coin_groups: Vec<String>,
    /// `true` once `run_ensure` alone has succeeded — biz + base data are in, the
    /// instrument detail may still be in flight. Consumers that only need biz-derived
    /// facts (tab list, instrument identity) can start on this instead of `is_ready`.
    pub is_biz_ready: bool,
    /// Whether the loaded Instrument is live, resolved from its lifecycle/list-stage metadata.
    /// This does not wait for instrument-detail data; non-live instruments await `is_ready`.
    pub is_live: bool,
}

/// Per-tab metadata for the introduce-tab dispatcher.
/// 
/// Empty strings encode "no data" (instrument not found, or branch not
/// applicable to the inst type) — the native side already filters with
/// `isNotEmpty()` checks identical to the V1 `isNotNullOrEmpty()` checks.
#[derive(Debug, Clone, PartialEq)]
pub struct KlineIntroduceTabMeta {
    pub trade_symbol: String,
    pub display_quote_symbol: String,
    /// V1 `getIndexComponentsParam`:
    /// - SPOT: `inst_id` iff Spot biz reports `isSupportMargin(inst_id)`, else `""`
    /// - SWAP / FUTURES: `instrument.get_underlying()` (a.k.a. `mkUly`)
    /// - other: `""`
    pub index_components_param: String,
}

/// Lever interval data
/// 
/// Contains leverage configuration for an instrument
#[derive(Debug, Clone, PartialEq)]
pub struct KlineLeverIntervalData {
    /// Minimum leverage level
    pub min_level: String,
    /// Maximum leverage level
    pub max_level: String,
    /// Whether leverage can exceed maximum
    /// - true: UAE entities can use leverage beyond max, shows warning for higher leverage
    /// - false: Cannot exceed max leverage
    pub is_beyond_max: bool,
    /// Leverage selection percentages (comma-separated, e.g., "0.1,0.2,0.5,0.8,1")
    pub percents: String,
    /// Parsed leverage selection percentages as array
    pub percents_arr: Vec<String>,
}

/// Request parameters for lever interval API
/// 
/// URL: /priapi/v5/public/lever-interval
#[derive(Debug, Clone, PartialEq)]
pub struct KlineLeverIntervalReq {
    /// Instrument type
    pub inst_type: String,
    /// Underlying asset
    pub uly: String,
    /// Instrument family
    pub inst_family: String,
    /// Instrument ID
    pub inst_id: String,
    /// Currency
    pub ccy: String,
    /// Margin mode
    pub mgn_mode: String,
    /// Cache expiration time in seconds (0 = no cache)
    pub cache_expire_seconds: u64,
}

/// Result wrapper for lever interval API response
#[derive(Debug, Clone, PartialEq)]
pub struct KlineLeverIntervalResult {
    /// List of lever interval data
    pub data: Vec<KlineLeverIntervalData>,
}

/// Mark price data
/// 
/// Contains mark price information for an instrument
#[derive(Debug, Clone, PartialEq)]
pub struct KlineMarkPriceData {
    /// Instrument type
    pub inst_type: String,
    /// Instrument ID
    pub inst_id: String,
    /// Mark price
    pub mark_px: String,
    /// Timestamp in milliseconds
    pub ts: String,
}

/// Request parameters for mark price API
/// 
/// URL: /priapi/v5/public/mark-price
#[derive(Debug, Clone, PartialEq)]
pub struct KlineMarkPriceReq {
    /// Instrument type
    pub inst_type: String,
    /// Underlying asset
    pub uly: String,
    /// Instrument family
    pub inst_family: String,
    /// Instrument ID
    pub inst_id: String,
    /// Cache expiration time in seconds (0 = no cache)
    pub cache_expire_seconds: u64,
}

/// Result wrapper for mark price API response
#[derive(Debug, Clone, PartialEq)]
pub struct KlineMarkPriceResult {
    /// List of mark price data
    pub data: Vec<KlineMarkPriceData>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct KlineMcapV2State {
    /// Latest `subscription_id` accepted; native passes a fresh id per request
    /// so stale callbacks are filtered out.
    pub subscription_id: u32,
    /// Trend points (timestamp + market_cap_value), ascending by time.
    pub points: Vec<MarketCapPoint>,
    /// Data source (FMP / CoinGecko / Unknown).
    pub source: MarketCapV2Source,
    /// Market-cap caliber (underlying-asset / circulating / Unknown).
    pub caliber: MarketCapV2Caliber,
    /// Quote currency, verbatim.
    pub quote_ccy: String,
    /// Granularity echoed by the server, verbatim.
    pub granularity: String,
    /// Timestamp (ms) of the last valid point; `0` when there are none.
    pub latest_effective_time: i64,
    /// Server-reported staleness flag.
    pub is_stale: bool,
    /// Derived from `points` being non-empty. Drives toggle-button visibility
    /// only — it does NOT mean "market cap is zero" and does not replace the
    /// semantic fields above.
    pub has_data: bool,
    pub is_loading: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct KlineNewCoinIntroState {
    pub trade_symbol: String,
    pub quote_symbol: String,
    pub has_tradable: bool,
    pub is_watched: bool,
    /// Pre-composed display string (e.g. `"$0.0000"`, `"0.0000 €"`). No
    /// thousands grouping — the only value composed here is the zero state,
    /// where grouping is a no-op.
    pub zero_local_price: String,
    pub is_loading: bool,
    pub error: Option<String>,
}

/// UI-ready state for the new-coin page. The three flows are mutually exclusive
/// (selected by `mode`); only the matching group of fields is meaningful.
#[derive(Debug, Clone, PartialEq)]
pub struct KlineNewCoinState {
    /// Which pre-listing flow this instrument is in.
    pub mode: KlineNewCoinMode,
    /// Whether the new-coin banner should still be shown.
    pub should_show: bool,
    /// Whitelist (market-making) user — gets realtime header + real depth.
    pub is_white_user: bool,
    /// Whether the depth tab should load real depth (white user || mode != New).
    pub is_need_load_depth: bool,
    /// Whether the real-time ticker header should be shown. Per the unified
    /// (iOS) rule: only when the countdown open method is "opening price"
    /// (`open_method == "1"`). False for auction / pre-quote / other methods.
    pub should_show_header: bool,
    /// Coin icon URL from the coins cache (mirrors V1 `MarketConfig.getCoinUrl`).
    /// Used by the auction / pre-quote banner views. Empty if coins not loaded.
    pub coin_icon_url: String,
    /// Static all-zero header placeholder for non-whitelist users (aligns V1's non-white
    /// `showTopInfo`). Only set when `should_show_header && !is_white_user`; rendered via the
    /// same converter as the real header. None for whitelist (real header) / hidden header.
    pub placeholder_head_info: Option<HeadInfoVo>,
    pub stage: KlineNewCoinStage,
    pub countdown_seconds: i64,
    pub icon: String,
    pub display_id: String,
    pub online_ts: String,
    pub open_method: String,
    /// Whether the post-countdown next stage is call-auction / pre-quote
    /// (open_method "0" / "2"). Drives the countdown subtitle text, mirroring V1
    /// `CountDownInfo.getNextStage()` + `getAvailableTitle()`: true → "available in
    /// call auction", false → "available in charts" (open_method "1" or anything
    /// else). Only meaningful for mode == New.
    pub available_in_auction: bool,
    pub group: String,
    pub notice_url_cn: String,
    pub notice_url_en: String,
    pub auction_eq_px: String,
    pub auction_matched_sz: String,
    pub auction_unmatched_sz: String,
    pub auction_end_time: String,
    /// Server-generated data time — the "now" anchor the countdown view ticks from.
    pub auction_ts: String,
    pub auction_state: String,
    pub auction_finished: bool,
    pub pre_quote_end_timestamp: String,
    /// Server data modification time — the "now" anchor for the pre-quote countdown.
    pub pre_quote_modification_timestamp: String,
    pub pre_quote_finished: bool,
    /// Index price — `PRE_QUOTE_IND` stage only; empty at `PRE_QUOTE_EST`.
    pub pre_quote_index_price: String,
    /// Estimated opening price (same value as `auction_eq_px`) — `PRE_QUOTE_EST`
    /// stage only; empty at `PRE_QUOTE_IND`. May legitimately stay empty even
    /// at EST stage if the backend hasn't flagged the instrument `continuous`
    /// yet — the View renders a placeholder for empty values.
    pub pre_quote_estimated_open_price: String,
    /// Max buy price bound — `PRE_QUOTE_IND` stage only; empty at `PRE_QUOTE_EST`.
    pub pre_quote_max_buy_price: String,
    /// Min sell price bound — `PRE_QUOTE_IND` stage only; empty at `PRE_QUOTE_EST`.
    pub pre_quote_min_sell_price: String,
    /// True at `PRE_QUOTE_EST`, false at `PRE_QUOTE_IND` (and false before the
    /// one-shot list-stage fetch resolves).
    pub pre_quote_is_estimated_stage: bool,
    pub is_loading: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct KlineOndoStockHoursState {
    /// Monotonic id of the latest subscribe; every async write drops itself if
    /// stale so a rapid re-subscribe can't be clobbered by an old fetch/tick.
    pub subscription_id: u32,
    /// Whether the target is an Ondo RWA (renderable). False hides the tag.
    pub is_stock: bool,
    pub session: StockSessionStatus,
    pub schedule: Vec<StockSessionRowVo>,
    pub current_start_ms: i64,
    pub current_end_ms: i64,
    /// Server-anchored "now" cursor, advanced once per minute.
    pub effective_ts_ms: i64,
    /// Ondo RWA full name (displayed in DEX top bar RWA mode).
    pub full_name: String,
    /// Ondo zero-fee flag.
    pub ondo_zero_fee: bool,
    pub is_loading: bool,
    pub error: Option<String>,
}

/// Opening info data
/// 
/// Contains opening/listing information for an instrument
/// Corresponds to OKTSpotOpeningInfoModel in iOS.
#[derive(Debug, Clone, PartialEq)]
pub struct KlineOpeningInfoData {
    /// Opening/listing mode
    pub list_mode: KlineOpeningModeType,
    /// Opening/listing price
    pub list_price: String,
    /// Matched auction volume
    pub matched_auction_volume: String,
    /// Base currency for matched auction volume unit
    pub base_ccy: String,
}

/// Request parameters for opening info API
/// 
/// URL: /priapi/v5/public/trading-rules/list-info
#[derive(Debug, Clone, PartialEq)]
pub struct KlineOpeningInfoReq {
    /// Instrument ID
    pub inst_id: String,
    /// Cache expiration time in seconds (0 = no cache)
    pub cache_expire_seconds: u64,
}

/// Result wrapper for opening info API response
#[derive(Debug, Clone, PartialEq)]
pub struct KlineOpeningInfoResult {
    /// Opening info data (single item)
    pub data: Option<KlineOpeningInfoData>,
}

/// Pie chart data
/// 
/// Represents the token unlock allocation pie chart.
/// Corresponds to OKMKLineTUPieChartModel in iOS.
#[derive(Debug, Clone, PartialEq)]
pub struct KlinePieChartData {
    /// Currency symbol (e.g., "BTC", "ETH")
    pub ccy: String,
    /// Maximum supply
    pub max_supply: String,
    /// Unlock timestamp
    pub unlocked_ts: String,
    /// List of pie portions
    pub list: Vec<KlinePiePortionData>,
}

/// Pie portion data
/// 
/// Represents each part/slice in the pie chart view.
/// Corresponds to OKMKLineTUPiePortionModel in iOS.
#[derive(Debug, Clone, PartialEq)]
pub struct KlinePiePortionData {
    /// Label type
    pub label: String,
    /// Percentage of this label's allocation vs total allocation
    pub perc_of_label: String,
    /// Percentage of unlocked tokens in this label vs total
    pub perc_of_unlocked: String,
    /// Unlock progress of this label's allocation
    pub progress: String,
    /// Whether this slice is selected
    pub is_selected: bool,
    /// Angle value for pie chart rendering (min 0.05 for visibility)
    pub angle_value: String,
}

/// Display-ready position record for the K-line trade overlay.
/// 
/// Field groups:
/// - **Identity**: ids and metadata used for lookups and chart linkage
/// - **Raw**: untouched values forwarded from
/// [`super::kline_position::RetailKLinePosition`] (needed by drag PnL math)
/// - **Display**: locale-aware strings consumed directly by the chart
/// - **K-line specific**: attach-order linkage emitted by the pipeline
#[derive(Debug, Clone, PartialEq)]
pub struct KlinePositionDisplay {
    /// Position ID
    pub id: String,
    /// Instrument ID (e.g. "BTC-USDT-SWAP")
    pub inst_id: String,
    /// Instrument type ("SWAP" / "FUTURES" / "MARGIN" / "OPTION")
    pub inst_type: String,
    /// Position side
    pub pos_side: KLinePositionSide,
    /// Margin mode ("cross" / "isolated")
    pub mgn_mode: String,
    /// Whether this is a long position
    pub is_long: bool,
    /// Whether this position supports the one-click revert action
    pub support_revert: bool,
    /// Whether this is an EEA timed-growth-voucher (experience-coupon) position.
    /// The chart hides close/reverse/TP/SL for it (V1 isExperienceCoupon parity).
    pub is_experience_coupon: bool,
    /// Untouched average / open price from upstream
    pub origin_avg_px: String,
    /// Untouched liquidation price from upstream (if any)
    pub origin_liq_px: Option<String>,
    /// Untouched break-even price from upstream (if any)
    pub origin_balance_px: Option<String>,
    /// Position quantity (raw contract count for swap/futures, raw size for margin)
    pub pos: String,
    /// Leverage (empty for options/spot)
    pub lever: String,
    /// Formatted average / open price for chart display
    pub avg_px: String,
    /// Formatted unrealised PnL (with sign)
    pub upl: String,
    /// Unit / currency label paired with [`upl`]
    pub upl_sign: String,
    /// Formatted unrealised PnL ratio (e.g. "+0.82%")
    pub upl_ratio: String,
    /// Formatted position size for display
    pub pos_display: String,
    /// Unit label paired with [`pos_display`] ("BTC" / "Cont" / "USDT")
    pub pos_sign: String,
    /// Formatted initial-margin amount
    pub imr: String,
    /// Unit label paired with [`imr`]
    pub imr_sign: String,
    /// Formatted margin balance
    pub margin: String,
    /// Unit label paired with [`margin`]
    pub margin_sign: String,
    /// Formatted liquidation price
    pub liq_px: Option<String>,
    /// Formatted break-even price
    pub balance_px: Option<String>,
    /// IDs of orders attached to this position (TP / SL).
    /// Populated by `linkage.rs`.
    pub linked_attached_order_ids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct KlineQuoteCandleVo {
    pub ts: i64,
    pub o: String,
    pub h: String,
    pub l: String,
    pub c: String,
    pub vol: String,
    pub vol_ccy: String,
    pub vol_ccy_quote: String,
    /// Exchange candle status (`"0"` forming, `"1"` confirmed). Appended as
    /// an optional field to preserve source semantics without breaking older
    /// native constructors of this UniFFI record.
    pub confirm: Option<String>, // default = None
}

/// Social media information
/// 
/// Represents a social media link associated with the cryptocurrency.
#[derive(Debug, Clone, PartialEq)]
pub struct KlineSocialMediaInfo {
    /// Social media type (e.g., "officialUrl", "whitePaper", "github", "twitter")
    pub name: String,
    /// Icon URL for the social media platform
    pub icon: String,
    /// URL to the social media page
    pub url: String,
    /// Display title for the social media link
    pub title: String,
}

/// Display-ready spot cost-line record.
/// 
/// Only one such record is shown at a time — the pipeline picks the asset
/// matching the active `ccy` from the format context (mirroring V1
/// `assets.find { it.ccy == ccy }`).
#[derive(Debug, Clone, PartialEq)]
pub struct KlineSpotAssetDisplay {
    /// Currency code (matches the active `ccy` in the format context)
    pub ccy: String,
    /// Quote / settlement unit symbol (forwarded from format context)
    pub quote_unit: String,
    /// Untouched open average price from upstream
    pub origin_open_avg_px: String,
    /// Untouched accumulated average price from upstream
    pub origin_acc_avg_px: String,
    /// Untouched total PnL from upstream
    pub origin_total_pnl: String,
    /// Untouched spot unrealised PnL from upstream
    pub origin_spot_upl: String,
    /// Open-average price (numeric form, quote-currency precision)
    pub open_avg_px: String,
    /// Accumulated-average price (numeric form, quote-currency precision)
    pub acc_avg_px: String,
    /// Open-average price (display string, includes quote-currency symbol)
    pub format_open_avg_px: String,
    /// Accumulated-average price (display string, includes quote-currency symbol)
    pub format_acc_avg_px: String,
    /// Total PnL (display string)
    pub total_pnl: String,
    /// Total PnL ratio (display string, e.g. "+0.82%")
    pub total_pnl_ratio: String,
    /// Spot unrealised PnL (display string)
    pub spot_upl: String,
    /// Spot unrealised PnL ratio (display string)
    pub spot_upl_ratio: String,
    /// Cash balance (numeric form, locale-aware compact-scaled — native
    /// appends the localized suffix selected via [`cash_bal_unit`]).
    pub cash_bal: String,
    /// Spot balance (numeric form, locale-aware compact-scaled — native
    /// appends the localized suffix selected via [`spot_bal_unit`]).
    pub spot_bal: String,
    /// Compact-unit tag for [`cash_bal`]. Native maps to the localized
    /// suffix string at render time so no natural-language text crosses FFI.
    pub cash_bal_unit: KlineCompactUnit,
    /// Compact-unit tag for [`spot_bal`]. Same contract as
    /// [`cash_bal_unit`].
    pub spot_bal_unit: KlineCompactUnit,
    /// `true` → render the *open* average price line; `false` → render the
    /// accumulated average. Controlled by the spot-average-type user pref.
    pub is_open_avg_px: bool,
    /// `true` when the cost line should be rendered on the chart. Driven by
    /// the active price type (only "last price" / [`super::KLinePriceMode::Default`]).
    pub show_spot_average_line: bool,
}

/// K-line start time data item
/// 
/// Represents the initial time for each bar/timeframe of a specific instrument
#[derive(Debug, Clone, PartialEq)]
pub struct KlineStartTimeVo {
    /// Bar/timeframe (e.g., "1m", "5m", "15m", "1H", "1D")
    pub bar: String,
    /// Timestamp in milliseconds - the initial time for this bar
    pub ts: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct KlineSwipingTitleState {
    pub items: Vec<SwipingInstrumentVo>,
    pub is_loading: bool,
    pub error: Option<String>,
}

/// Ticker data
/// 
/// Contains comprehensive ticker information for an instrument
#[derive(Debug, Clone, PartialEq)]
pub struct KlineTickerData {
    /// Instrument type
    pub inst_type: String,
    /// Instrument ID
    pub inst_id: String,
    /// Latest trade price
    pub last: String,
    /// Latest trade size
    pub last_sz: String,
    /// Best ask price
    pub ask_px: String,
    /// Best ask size
    pub ask_sz: String,
    /// Best bid price
    pub bid_px: String,
    /// Best bid size
    pub bid_sz: String,
    /// 24h opening price
    pub open_24h: String,
    /// 24h highest price
    pub high_24h: String,
    /// 24h lowest price
    pub low_24h: String,
    /// 24h trading volume in quote currency
    pub vol_ccy_24h: String,
    /// 24h trading volume in contracts
    pub vol_24h: String,
    /// Ticker timestamp in milliseconds
    pub ts: String,
    /// Start of day UTC+0
    pub sod_utc0: String,
    /// Start of day UTC+8
    pub sod_utc8: String,
}

/// Token unlock event data
/// 
/// Represents a single token unlock event.
/// Corresponds to OKMKLineTokenUnlockEventModel in iOS.
#[derive(Debug, Clone, PartialEq)]
pub struct KlineTokenUnlockEventData {
    /// Currency symbol
    pub ccy: String,
    /// Unlock timestamp
    pub unlocked_ts: String,
    /// Unlocked token amount
    pub unlocked: String,
    /// Unlocked value in USD
    pub unlocked_value: String,
    /// Percentage of unlocked tokens vs total supply
    pub perc_of_unlocked: String,
    /// Percentage of unlocked value vs total market cap
    pub perc_of_unlocked_value: String,
    /// Allocation labels for this event
    pub allocation: Vec<String>,
}

/// Token unlock events request parameters
/// 
/// # Fields
/// 
/// - `ccy`: Currency symbol (e.g., "BTC", "ETH")
/// - `event_type`: Event type filter
/// - `cache_expire_seconds`: HTTP cache expiration time in seconds (0 = no cache)
#[derive(Debug, Clone, PartialEq)]
pub struct KlineTokenUnlockEventsReq {
    /// Currency symbol (e.g., "BTC", "ETH")
    pub ccy: String,
    /// Event type filter
    pub event_type: u64,
    /// HTTP cache expiration time in seconds (0 = no cache)
    pub cache_expire_seconds: u64,
}

/// Token unlock events result
/// 
/// The response structure containing token unlock events.
#[derive(Debug, Clone, PartialEq)]
pub struct KlineTokenUnlockEventsResult {
    /// List of token unlock events
    pub data: Vec<KlineTokenUnlockEventData>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct KlineTradeOverlayState {
    pub open_orders_list: Vec<RetailKLineOrder>,
    pub positions_list: Vec<RetailKLinePosition>,
    /// Spot account positions / holdings — derived to "spot average cost"
    /// overlay on the chart by native side.
    pub spot_assets: Vec<RetailKLineAsset>,
    /// **Deprecated** — kept for ABI compatibility. The new drag-time PnL
    /// flow returns [`KlineDragPnlVo`] directly from `compute_drag_pnl`
    /// without going through state, so this field is always `None`.
    pub pending_pnl: Option<PendingPnlVo>,
    /// When true, manager callbacks keep raw orders / positions / assets fresh
    /// but leave `view_state` frozen — used while the user is actively dragging
    /// an order line so the overlay does not jump.
    pub is_drag_paused: bool,
    pub is_loading: bool,
    pub error: Option<String>,
    /// Display-ready aggregate for the chart overlay.
    pub view_state: KlineTradeOverlayViewState,
    /// Active price source (last / mark / index).
    pub price_mode: KLinePriceMode,
}

/// Display-ready trade-overlay state for the K-line quote page.
/// 
/// Field flow:
/// - `positions` / `orders` / `spot_asset` come from
/// `trade_overlay_pipeline::build_view_state`; display strings are
/// produced via `KlineFormatter` (`core::service::display`).
/// - `linkage_map` (`order_id` → `position_id`) is emitted by `linkage.rs`.
/// - `pending_pnl` is set by `drag_session.rs` while a drag is in progress.
/// - `revision` increments on every push so Native can drop stale frames.
#[derive(Debug, Clone, PartialEq)]
pub struct KlineTradeOverlayViewState {
    pub positions: Vec<KlinePositionDisplay>,
    pub orders: Vec<DisplayOrderVo>,
    /// Asset matching the active `ccy`, or `None` when the spot cost line
    /// should be hidden (non-spot price type or no holdings).
    pub spot_asset: Option<KlineSpotAssetDisplay>,
    /// `order_id` → `position_id`. Filled by `linkage.rs`.
    pub linkage_map: HashMap<String, String>,
    /// Hypothetical PnL while a chart drag is in progress.
    pub pending_pnl: Option<PendingPnlVo>,
    /// `true` while the upstream callback is being suppressed because the
    /// user is dragging an order / cost / spot line.
    pub is_drag_paused: bool,
    /// Mirrors the loading flag on the raw state.
    pub is_loading: bool,
    /// Error message propagated from upstream callback, if any.
    pub error: Option<String>,
    /// Monotonic counter, incremented on every push. Allows Native to skip
    /// stale frames; resets only when the interactor is reconstructed.
    pub revision: u64,
    /// Whether the current SWAP/FUTURES instrument is inside a contract
    /// novice-cooling-period for the active user. Always `false` for
    /// SPOT/MARGIN/OPTION or before the first fetch completes.
    /// 
    /// Native quick-order gate reads this to decide whether to block the
    /// order panel with a cooling-period toast.
    pub is_in_cooling_period: bool,
    /// Cooling-period end time in milliseconds (Unix epoch). `0` when no
    /// cooling period is active. Native formats this for the toast message.
    pub cooling_end_time_ms: i64,
}

/// Unlock allocation request parameters
/// 
/// # Fields
/// 
/// - `ccy`: Currency symbol (e.g., "BTC", "ETH")
/// - `cache_expire_seconds`: HTTP cache expiration time in seconds (0 = no cache)
#[derive(Debug, Clone, PartialEq)]
pub struct KlineUnlockAllocationReq {
    /// Currency symbol (e.g., "BTC", "ETH")
    pub ccy: String,
    /// HTTP cache expiration time in seconds (0 = no cache)
    pub cache_expire_seconds: u64,
}

/// Unlock allocation result
/// 
/// The response structure containing pie chart data.
#[derive(Debug, Clone, PartialEq)]
pub struct KlineUnlockAllocationResult {
    /// Pie chart data (may be None if no data available)
    pub data: Option<KlinePieChartData>,
}

/// Unlock progress data
/// 
/// Represents the token unlock progress information.
/// Corresponds to OKMKLineTokenProgressModel in iOS.
#[derive(Debug, Clone, PartialEq)]
pub struct KlineUnlockProgressData {
    /// Currency symbol
    pub ccy: String,
    /// Maximum supply
    pub max_supply: String,
    /// Market cap
    pub mcap: String,
    /// Unlocked tokens
    pub unlocked: String,
    /// Locked tokens
    pub locked: String,
    /// Untracked tokens
    pub untracked: String,
    /// Percentage of unlocked tokens
    pub perc_of_unlocked: String,
    /// Percentage of locked tokens
    pub perc_of_locked: String,
    /// Percentage of untracked tokens
    pub perc_of_untracked: String,
    /// Vest info notes
    pub notes: String,
    /// Data source info
    pub data_source_info: String,
}

/// Unlock progress request parameters
/// 
/// # Fields
/// 
/// - `ccy`: Currency symbol (e.g., "BTC", "ETH")
/// - `cache_expire_seconds`: HTTP cache expiration time in seconds (0 = no cache)
#[derive(Debug, Clone, PartialEq)]
pub struct KlineUnlockProgressReq {
    /// Currency symbol (e.g., "BTC", "ETH")
    pub ccy: String,
    /// HTTP cache expiration time in seconds (0 = no cache)
    pub cache_expire_seconds: u64,
}

/// Unlock progress result
/// 
/// The response structure containing unlock progress data.
#[derive(Debug, Clone, PartialEq)]
pub struct KlineUnlockProgressResult {
    /// Unlock progress data (may be None if no data available)
    pub data: Option<KlineUnlockProgressData>,
}

/// Unlock schedule data
/// 
/// Represents the token unlock schedule.
/// Corresponds to OKMKLineTokenUnlocksScheduleModel in iOS.
#[derive(Debug, Clone, PartialEq)]
pub struct KlineUnlockScheduleData {
    /// Timestamp
    pub ts: String,
    /// List of unlock schedule items
    pub data: Vec<KlineUnlockScheduleSingleData>,
    /// Whether this is the default data to display (closest to today)
    pub near_today: bool,
}

/// Unlock schedule request parameters
/// 
/// # Fields
/// 
/// - `ccy`: Currency symbol (e.g., "BTC", "ETH")
/// - `period`: Time period filter (required by API)
/// - `cache_expire_seconds`: HTTP cache expiration time in seconds (0 = no cache)
/// 
/// # Period Values (from OKMOverviewPeriodType)
/// 
/// | Value | Description |
/// |-------|-------------|
/// | "1D"  | 1 day       |
/// | "1W"  | 1 week      |
/// | "1M"  | 1 month     |
/// | "3M"  | 3 months    |
/// | "1Y"  | 1 year      |
/// | "ALL" | All data    |
#[derive(Debug, Clone, PartialEq)]
pub struct KlineUnlockScheduleReq {
    /// Currency symbol (e.g., "BTC", "ETH")
    pub ccy: String,
    /// Time period filter
    /// - "1D": 1 day
    /// - "1W": 1 week (default)
    /// - "1M": 1 month
    /// - "3M": 3 months
    /// - "1Y": 1 year
    /// - "ALL": All data
    pub period: String,
    /// HTTP cache expiration time in seconds (0 = no cache)
    pub cache_expire_seconds: u64,
}

/// Unlock schedule result
/// 
/// The response structure containing unlock schedule data.
/// API returns an array of OKMKLineTokenUnlocksScheduleModel.
#[derive(Debug, Clone, PartialEq)]
pub struct KlineUnlockScheduleResult {
    /// List of unlock schedule data (array of OKMKLineTokenUnlocksScheduleModel)
    pub data: Vec<KlineUnlockScheduleData>,
}

/// Single unlock schedule item
/// 
/// Represents a single unlock event in the schedule.
/// Corresponds to OKMKLineTUSingleModel in iOS.
#[derive(Debug, Clone, PartialEq)]
pub struct KlineUnlockScheduleSingleData {
    /// Timestamp
    pub ts: String,
    /// Label type
    pub label: String,
    /// Unlocked token amount
    pub unlocked: String,
    /// Unlocked percentage
    pub unlocked_percent: String,
    /// Cumulative unlocked amount
    pub cul_unlocked: String,
    /// Unlocked value in USD
    pub unlocked_value: String,
    /// Cumulative unlocked value in USD
    pub cul_unlocked_value: String,
    /// Whether this item is selected
    pub is_selected: bool,
}

/// Describes the corporate action that produced the cumulative factors.
#[derive(Debug, Clone, PartialEq)]
pub struct LastUpdatedEvent {
    /// SPLIT / REVERSE_SPLIT / CASH_DIVIDEND / CASH_DIVIDEND_AND_STOCK_SPLIT / ...
    pub event_type: Option<String>,
    /// 0 = revoked, 1 = effective. Forwarded as-is so native can reuse the
    /// same constants without an extra enum layer.
    /// Tolerant deserializer: backend may send as int or string.
    pub status: Option<i32>,
    /// Ex-dividend / split-effective timestamp in milliseconds.
    /// Tolerant deserializer: backend currently sends as JSON number
    /// (1779783422728) but earlier contracts said string; accept both.
    pub effective_time: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LatestDealInteractorState {
    /// Filtered deal list for display (newest first, max 25 items).
    /// px and sz are pre-formatted for display.
    pub filtered_deals: Vec<BizLatestDeal>,
    pub filter_min_size: String,
    pub is_filter_active: bool,
    pub is_loading: bool,
    pub is_empty: bool,
    pub error: Option<String>,
    /// Header price unit (e.g., "USDT" for SPOT, "₮" sign for SWAP/FUTURES via getPriceDisplaySign).
    pub price_unit: String,
    /// Header amount unit. Driven by user's trade_unit setting via getAmountDisplaySymbol.
    /// May be a CD key (e.g., "pr_options_footer_switch_amount") that native resolves
    /// to the localized contract-unit label.
    pub amount_unit: String,
}

/// HTTP request parameters for latest deal list
#[derive(Debug, Clone, PartialEq)]
pub struct LatestDealReq {
    /// Instrument ID (e.g., "BTC-USDT", "BTC-USDT-SWAP")
    pub inst_id: String,
    /// Maximum number of entries to pull (OKX default 100, max 500)
    pub limit: Option<String>,
}

/// Request for leverage ratio data (Margin)
/// API: GET /priapi/v5/rubik/stat/margin/{instName}/loan-ratio
#[derive(Debug, Clone, PartialEq)]
pub struct LeverageRatioReq {
    /// Instrument ID (e.g. "BTC-USDT")
    pub inst_id: String,
    /// Period: "5m", "15m", "1H", "4H", "1D"
    pub period: String,
}

/// Long/Short ratio data for margin trading
#[derive(Debug, Clone, PartialEq)]
pub struct LongShortRatioData {
    /// Timestamp
    pub ts: String,
    /// Long amount
    pub long_amt: String,
    /// Short amount
    pub short_amt: String,
    /// Ratio
    pub ratio: Option<String>,
}

/// A single market-cap sample: a timestamp and the market-cap value recorded at
/// that time, both carried as strings to avoid floating-point precision loss.
/// 
/// Formerly `crate::types::market_cap::MarketCapPoint` (v1); relocated here as a
/// v2-owned type when the v1 domain was retired. The shape and UniFFI `Record`
/// contract are unchanged.
#[derive(Debug, Clone, PartialEq)]
pub struct MarketCapPoint {
    /// Unix timestamp in milliseconds, e.g. `"1704067200000"`.
    pub timestamp: String,
    /// Market-cap value in the quote currency, e.g. `"850000000000"`.
    pub market_cap_value: String,
}

/// Market cap v2 request parameters.
/// 
/// Only `inst_id`, `inst_type`, `from` and `to` are sent to the server;
/// `cache_expire_seconds` is a client-local HTTP cache control (same meaning as
/// v1), not an HTTP parameter. Granularity is intentionally not a field — v2
/// does not send it (the server applies its default, currently `1D`).
#[derive(Debug, Clone, PartialEq)]
pub struct MarketCapV2Req {
    /// Instrument id, e.g. `BTC-USDT`, `xTSLA-USDT`. Required, non-empty. Sent
    /// as the `instId` query parameter. Note this differs from v1, which sent a
    /// bare coin symbol (e.g. `BTC`).
    pub inst_id: String,
    /// Instrument type. Required, non-empty. Sent as the `instType` query
    /// parameter.
    pub inst_type: String,
    /// Start time in milliseconds. `0` means unspecified — the parameter is
    /// omitted from the URL and the server applies its own window. `0` is a
    /// sentinel, not the year 1970.
    /// 
    /// Measured 2026-09-19 (BTC-USDT), as none of it is in a written contract: the
    /// omitted default is `to - 365d`, `from` is exclusive like `to`, and a window
    /// holding more than 1000 points is capped at 1000 with the OLD end truncated.
    /// A literal `from=0` on the wire hits that cap, which is why 0 must mean "omit"
    /// here and never reach the URL.
    pub from: i64,
    /// End time in milliseconds. `0` means unspecified — the parameter is
    /// omitted from the URL and the server applies its default (now). `0` is a
    /// sentinel, not the year 1970.
    pub to: i64,
    /// HTTP cache expiration in seconds. `0` means no caching. Same meaning as
    /// v1; this is client-local and is not sent as an HTTP parameter.
    pub cache_expire_seconds: u64,
}

/// Market cap v2 result.
/// 
/// Carries all seven server semantics without lossy compression.
#[derive(Debug, Clone, PartialEq)]
pub struct MarketCapV2Result {
    /// Time series points, ascending by timestamp. An empty vector means
    /// **no data available** — it must never be interpreted as a market cap of
    /// zero. Uses the v2-owned `MarketCapPoint` type (timestamp + value strings).
    pub points: Vec<MarketCapPoint>,
    /// Data source (FMP / CoinGecko / Unknown).
    pub source: MarketCapV2Source,
    /// Market-cap caliber (underlying-asset total / circulating / Unknown).
    pub caliber: MarketCapV2Caliber,
    /// Quote currency, passed through verbatim (currently USD or whatever the
    /// upstream returns).
    pub quote_ccy: String,
    /// Granularity echoed by the server, passed through verbatim (currently
    /// `1D`).
    pub granularity: String,
    /// Timestamp (ms) of the last valid point in this series; `0` when there
    /// are no valid points.
    pub latest_effective_time: i64,
    /// Whether the latest sample in the current window exceeds the freshness
    /// threshold, as reported by the server.
    pub is_stale: bool,
}

/// Request parameters for market events
#[derive(Debug, Clone, PartialEq)]
pub struct MarketEventReq {
    /// Instrument ID (e.g., "BTC-USDT")
    pub inst_id: String,
    /// Cursor for pagination
    pub cursor: Option<String>,
    /// Page size
    pub page_size: Option<i32>,
    /// Unit type
    pub unit: Option<String>,
    /// Display instrument name
    pub display_inst_name: Option<String>,
    /// Whether this is a WebSocket force request
    /// When true, UseCase will filter events newer than `latest_timestamp`
    pub is_ws_force_request: bool,
    /// Latest event timestamp from current list (milliseconds)
    /// Used with `is_ws_force_request = true` to filter new events
    pub latest_timestamp: Option<i64>,
    /// Use new template (default: 1)
    pub use_new_temp: i32,
}

/// Section-grouped market events (preferred Vo for clients >= v6.169).
/// 
/// `bucket_ts` is currently bucketed by UTC day 00:00:00 millis. After the
/// native side receives it, it formats the section header text according to
/// the local locale ("Today" / "Yesterday" / absolute date); the Rust layer
/// does not care about timezone or i18n.
#[derive(Debug, Clone, PartialEq)]
pub struct MarketEventSectionVo {
    /// Bucket timestamp of this section (UTC day 00:00:00 millis)
    pub bucket_ts: i64,
    pub items: Vec<MarketEventsVo>,
}

/// Result returned from market events use case
#[derive(Debug, Clone, PartialEq)]
pub struct MarketEventsResult {
    /// Next cursor for pagination
    pub next_cursor: Option<String>,
    /// Number of new/updated events (for WebSocket updates)
    pub update_count: i32,
    /// List of all events to display
    pub event_list: Vec<MarketEventsVo>,
}

/// State for market events interactor
/// 
/// Note: `events` is preserved for backward compatibility (clients < v6.169).
/// New consumers should read from `sections` + `events_version` + `last_added_count`.
/// 
/// `#[allow(deprecated)]` is applied at the struct level because the derives
/// generated by `#[interactor::state]` (Clone/Debug/PartialEq) internally
/// reference `events`, which would otherwise trigger a self-deprecation warning
/// under the workspace `warnings = "deny"` policy.
#[derive(Debug, Clone, PartialEq)]
pub struct MarketEventsState {
    /// New: section-grouped events (preferred by clients >= v6.169).
    pub sections: Vec<MarketEventSectionVo>,
    /// New: monotonically increasing on every change, so native can diff first-load vs incremental.
    pub events_version: u64,
    /// New: last incremental added count, paired with `events_version`.
    pub last_added_count: i32,
    pub events: Vec<MarketEventsVo>,
    pub is_loading: bool,
    pub has_more: Option<bool>,
    pub error: Option<String>,
}

/// Mention detail information
#[derive(Debug, Clone, PartialEq)]
pub struct MentionDetail {
    /// Authors list
    pub authors: Vec<String>,
    /// Mention number
    pub mention_num: String,
    /// Mention number V1
    pub mention_num_v1: String,
}

/// Configuration for MoreTradeManager
#[derive(Debug, Clone, PartialEq)]
pub struct MoreTradeConfig {
    /// Current instrument ID (e.g., "BTC-USDT")
    pub current_inst_id: String,
    /// Current business type (e.g., "SPOT", "SWAP")
    pub current_biz_type: String,
    /// Whether options feature is restricted
    pub is_options_restricted: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MoreTradeInteractorState {
    pub items: Vec<MoreTradeItem>,
    pub is_loading: bool,
    pub is_empty: bool,
    pub error: Option<String>,
    /// Whether the user is in demo/simulate trading mode.
    /// Read from `CTTradeAccountManager` on subscribe — used by native to
    /// hide the favorite star icon in simulate mode.
    pub is_demo_trading: bool,
}

/// More Trade Item
/// 
/// Data structure for displaying a single trading instrument in the K-line
/// page's trade tab. Contains all the information needed for UI rendering.
/// 
/// Corresponds to Native's `OKKMoreTradeMarketTableViewCellItem`.
/// 
/// # Fields
/// - `inst_id`: Instrument identifier (e.g., "BTC-USDT", "BTC-USDT-SWAP")
/// - `biz_type`: Business line type (Spot/Swap/Futures/Option)
/// - `title`: Localized business line name
/// - `price`: Current price string
/// - `change_value`: Price change percentage string (e.g., "+2.35%", "-1.20%")
/// - `is_positive_change`: Whether the change is positive (for color display)
/// - `is_favorite`: Whether the instrument is in user's favorites
/// - `is_options`: Whether this is an options instrument (for special UI)
/// - `is_show_lever`: Whether to show leverage info
/// - `flag_title`: Contract flag label (e.g., "Perp", "0327")
/// - `icon_url`: Coin icon URL (options only)
/// - `base_ccy`: Base currency (e.g., "BTC")
/// - `quote_ccy`: Quote currency (e.g., "USDT")
/// - `inst_family`: Instrument family (e.g., "BTC-USDT")
/// - `is_positive_contract`: Whether this is a linear/positive contract
#[derive(Debug, Clone, PartialEq)]
pub struct MoreTradeItem {
    /// Instrument identifier
    pub inst_id: String,
    /// Business line type
    pub biz_type: MoreTradeBizType,
    /// Localized business line title (e.g., "Spot", "Perpetual", "Futures", "Options")
    pub title: String,
    /// Current price string (formatted)
    pub price: String,
    /// Price change percentage string (e.g., "+2.35%", "-1.20%")
    pub change_value: String,
    /// Whether the price change is positive (for color display)
    pub is_positive_change: bool,
    /// Whether the instrument is in user's favorites
    pub is_favorite: bool,
    /// Whether this is an options instrument (for special UI treatment)
    pub is_options: bool,
    /// Whether to show leverage information
    pub is_show_lever: bool,
    /// Contract flag label (e.g., "Perp" for swap, "0327" for futures expiry)
    pub flag_title: String,
    /// Coin icon URL (used for options display)
    pub icon_url: String,
    /// Base currency symbol (e.g., "BTC")
    pub base_ccy: String,
    /// Quote currency symbol (e.g., "USDT")
    pub quote_ccy: String,
    /// Instrument family (e.g., "BTC-USDT")
    pub inst_family: String,
    /// Whether this is a linear/positive contract (affects options simple entry display)
    pub is_positive_contract: bool,
    /// Currency type for contract group lookup (e.g., "USDT", "USD").
    /// Used by native binder to call TradeFacade.getFutureGroupInfo(bizType, ccyType)
    /// for fresh localized margin-type text on every render (V1 parity).
    pub ccy_type: String,
    /// Display symbol for the instrument (e.g., "BTC/USDT", "BTCUSDT")
    pub display_symbol: String,
    /// Whether to show the simple-option entry for this Options item.
    /// True for eligible inverse Options after the upstream compliance gate.
    /// Always false for non-Options items.
    pub is_show_simple_option: bool,
    /// Instrument alias (delivery futures only).
    /// 
    /// For FUTURES contracts this carries the API alias (e.g. "this_week",
    /// "next_week", "quarter", "next_quarter") which — together with
    /// `inst_family` — uniquely identifies a delivery contract on the
    /// server. Add-to-favorite requires this pair for FUTURES; without it
    /// the server rejects with `code=3902 "Enter the required info."`.
    /// 
    /// Empty for SPOT / SWAP / OPTION.
    pub alias: String,
    /// Expiry time in milliseconds (OPTION only).
    /// 
    /// Option identity on the favorite API = (inst_id, exp_time); omitting
    /// exp_time serializes as "0" and the server rejects with
    /// `code=3902 "Enter the required info."` (same failure mode as a
    /// FUTURES item without its alias).
    /// 
    /// Empty for SPOT / SWAP / FUTURES.
    pub exp_time: String,
    /// Backend tradability flag for this instrument (`simpleProduct.isTradable`).
    /// 
    /// Threaded through so the V2 Rust Trade-tab binder can gate the favorite
    /// star and the row-click destination for the "visible-but-not-tradable"
    /// compliance scenario (TEPT-6547). Populated from the resolved
    /// `Instrument.is_tradable()` (core-trading, reference-only).
    /// 
    /// `Some(false)` -> backend explicitly marked not tradable.
    /// `Some(true)`  -> tradable.
    /// `None`        -> backend sent nothing -> treated as tradable downstream
    /// (backward compatible; old backend / old app parity, AC-7).
    pub tradable: Option<bool>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MultiPeriodChangeVo {
    /// Always 5 items in declaration order: today, 7d, 30d, 90d, 180d.
    pub items: Vec<PeriodChangeItemVo>,
    /// Last successful update timestamp (ms). Native uses for staleness detection.
    pub last_updated_ts: i64,
}

/// News carousel item
/// 
/// Represents a single news or campaign item in the carousel.
/// 
/// # Fields
/// 
/// - `active_id`: Unique identifier for the active item
/// - `url_slug`: URL slug for the item (for deep linking)
/// - `language`: Language code of the content
/// - `seo_title`: SEO-optimized title
/// - `seo_description`: SEO-optimized description
/// - `flash_type`: Type of flash content
/// 
/// # Why String Types?
/// 
/// All fields use `String` to:
/// - Handle null values from backend (converted to empty strings)
/// - Support internationalization
/// - Maintain consistent serialization across platforms
/// - Avoid type conversion issues
/// 
/// # Example
/// 
/// ```ignore
/// let item = NewsCarouselItem {
/// active_id: "63206156575808".to_string(),
/// url_slug: "btc-news".to_string(),
/// language: "en".to_string(),
/// seo_title: "Bitcoin News".to_string(),
/// seo_description: "Latest Bitcoin updates".to_string(),
/// flash_type: "news".to_string(),
/// };
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct NewsCarouselItem {
    /// Unique identifier for the active item
    /// 
    /// Format: String representation of a numeric ID
    /// 
    /// Example: "63206156575808"
    pub active_id: String,
    /// URL slug for the item
    /// 
    /// Used for deep linking and web navigation
    /// 
    /// Example: "btc-news-2024"
    pub url_slug: String,
    /// Language code of the content
    /// 
    /// Examples: "en", "zh", "ja"
    pub language: String,
    /// SEO-optimized title
    /// 
    /// The title of the news or campaign item
    pub seo_title: String,
    /// SEO-optimized description
    /// 
    /// Brief description of the news or campaign item
    pub seo_description: String,
    /// Type of flash content
    /// 
    /// Examples: "news", "campaign", "announcement"
    pub flash_type: String,
}

/// News carousel request parameters
/// 
/// This struct represents the input parameters for fetching news carousel data.
/// It supports both CeFi and CeDeFi use cases.
/// 
/// # Fields
/// 
/// ## CeFi Fields
/// - `location_code`: Page location identifier indicating where the carousel is displayed (e.g., "kline_page", "home_page", "trade_page")
/// - `ccy`: Currency filter (e.g., "BTC", "ETH")
/// - `inst_name`: Instrument name filter (e.g., "BTC-USDT")
/// - `biz_type`: Business type filter (e.g., "spot", "swap")
/// 
/// ## CeDeFi Fields
/// - `contract_address`: Smart contract address for DeFi tokens
/// - `chain_id`: Blockchain chain identifier (e.g., "1" for Ethereum mainnet)
/// - `token`: Token identifier for DeFi assets
/// 
/// ## Common Fields
/// - `url`: API endpoint URL for the request
/// - `size`: Number of items to return per page (default: 20)
/// - `cache_expire_seconds`: HTTP cache expiration time in seconds (0 = no cache)
/// 
/// # Example
/// 
/// ```ignore
/// // CeFi example
/// let req = NewsCarouselReq {
/// location_code: "kline_page".to_string(),
/// ccy: "BTC".to_string(),
/// inst_name: "BTC-USDT".to_string(),
/// biz_type: "spot".to_string(),
/// contract_address: String::new(),
/// chain_id: String::new(),
/// token: String::new(),
/// url: "/priapi/v5/rubik/public/active-list".to_string(),
/// size: 20,
/// cache_expire_seconds: 60,
/// };
/// 
/// // CeDeFi example
/// let req = NewsCarouselReq {
/// location_code: String::new(),
/// ccy: String::new(),
/// inst_name: String::new(),
/// biz_type: String::new(),
/// contract_address: "0x1234...".to_string(),
/// chain_id: "1".to_string(),
/// token: "USDT".to_string(),
/// url: "/priapi/v5/rubik/content/dex/flash/list/1".to_string(),
/// size: 20,
/// cache_expire_seconds: 60,
/// };
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct NewsCarouselReq {
    /// Page location identifier (CeFi)
    /// 
    /// Indicates which page the news carousel is displayed on.
    /// This allows the server to return page-specific content.
    /// 
    /// Examples: "kline_page", "home_page", "trade_page", "market_page"
    /// 
    /// Default: "kline_page"
    pub location_code: String,
    /// Currency filter (CeFi)
    /// 
    /// Examples: "BTC", "ETH", "USDT"
    /// 
    /// Default: empty string (no filter)
    pub ccy: String,
    /// Instrument name filter (CeFi)
    /// 
    /// Examples: "BTC-USDT", "ETH-USDT"
    /// 
    /// Default: empty string (no filter)
    pub inst_name: String,
    /// Business type filter (CeFi)
    /// 
    /// Examples: "spot", "swap", "futures"
    /// 
    /// Default: empty string (no filter)
    pub biz_type: String,
    /// Smart contract address (CeDeFi)
    /// 
    /// The blockchain contract address for DeFi tokens.
    /// 
    /// Examples: "0x1234567890abcdef...", "So11111111111111111111111111111111111111112"
    /// 
    /// Default: empty string (no filter)
    pub contract_address: String,
    /// Blockchain chain identifier (CeDeFi)
    /// 
    /// Identifies which blockchain network the token is on.
    /// 
    /// Examples: "1" (Ethereum), "56" (BSC), "137" (Polygon)
    /// 
    /// Default: empty string (no filter)
    pub chain_id: String,
    /// Token identifier (CeDeFi)
    /// 
    /// The token symbol or identifier for DeFi assets.
    /// 
    /// Examples: "USDT", "WETH", "DAI"
    /// 
    /// Default: empty string (no filter)
    pub token: String,
    /// API endpoint URL
    /// 
    /// The base URL path for the API request.
    /// 
    /// Examples:
    /// - CeFi: "/priapi/v5/rubik/public/active-list"
    /// - CeDeFi: "/priapi/v5/rubik/content/dex/flash/list/1"
    /// 
    /// Default: empty string
    pub url: String,
    /// Number of items to return per page
    /// 
    /// Controls the maximum number of news carousel items returned in the response.
    /// 
    /// Default: 20
    pub size: u64,
    /// HTTP cache expiration time in seconds
    /// 
    /// Controls how long the HTTP response should be cached:
    /// - `0`: No caching (always fetch fresh data from server)
    /// - `> 0`: Cache the response for the specified number of seconds
    /// 
    /// Default: 0 (no cache)
    pub cache_expire_seconds: u64,
}

/// News carousel result
/// 
/// The response structure containing news carousel data.
/// 
/// # Fields
/// 
/// - `content_flash`: Vector of content flash items (news, announcements)
/// - `campaign`: Vector of campaign items (promotions, events)
/// 
/// # Usage
/// 
/// The `content_flash` and `campaign` fields contain the parsed and validated data items.
/// 
/// # Example
/// 
/// ```ignore
/// let result = get_news_carousel(req).await?;
/// 
/// // Use parsed content flash data
/// for item in result.content_flash {
/// println!("Title: {}, Type: {}", item.seo_title, item.flash_type);
/// }
/// 
/// // Use parsed campaign data
/// for item in result.campaign {
/// println!("Campaign: {}", item.seo_title);
/// }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct NewsCarouselResult {
    /// List of content flash items
    /// 
    /// Contains news, announcements, and other flash content.
    /// May be empty if no content is available.
    pub content_flash: Vec<NewsCarouselItem>,
    /// List of campaign items
    /// 
    /// Contains promotional campaigns, events, and special offers.
    /// May be empty if no campaigns are available.
    pub campaign: Vec<NewsCarouselItem>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OrderLineVo {
    pub order_id: String,
    pub side: String,
    pub price: String,
    pub size: String,
    pub state: String,
}

/// Result of getting orders and positions together
/// 
/// Contains both orders and positions with order-position association applied.
/// TP/SL orders are attached to their corresponding positions.
#[derive(Debug, Clone, PartialEq)]
pub struct OrdersAndPositionsResult {
    /// Remaining orders not attached to any position
    pub orders: Vec<RetailKLineOrder>,
    /// Positions with attached TP/SL orders
    pub positions: Vec<RetailKLinePosition>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PendingPnlVo {
    pub target_id: String,
    pub hypothetical_price: String,
    pub pnl_amount: String,
    pub pnl_rate: String,
    pub is_positive: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PeriodChangeItemVo {
    /// Period key: "today" | "7d" | "30d" | "90d" | "180d". Native maps to localized label.
    pub label_key: String,
    /// Truncated to 2 decimals, signed. Empty string indicates no data for the period.
    /// Examples: "5.00", "-10.00", "" (no candle).
    pub change_percent_raw: String,
    /// Whether the ratio was strictly positive. Native uses for "+" prefix and color.
    pub is_positive: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PositionLineVo {
    pub pos_id: String,
    pub pos_side: String,
    pub avg_entry_price: String,
    pub size: String,
    pub upl: String,
    pub liq_price: String,
}

/// Request for position volume & trading volume data (Swap/Futures)
/// API: GET /priapi/v5/rubik/stat/contracts/{instName}/open-interest-volume-union
#[derive(Debug, Clone, PartialEq)]
pub struct PositionVolumeReq {
    /// Instrument ID (e.g. "BTC-USDT-SWAP")
    pub inst_id: String,
    /// Period: "5m", "15m", "1H", "4H", "1D"
    pub period: String,
}

/// One sourceType's worth of rule IDs for a batch delete. The action takes a list
/// of these so a mixed-type selection (CeFi + CeDeFi + indicators) is ONE effect —
/// looping `batch_delete_alerts` per type from native got all but the first call
/// silently dropped by `SpawnPolicy::SkipIfRunning`.
#[derive(Debug, Clone, PartialEq)]
pub struct PriceAlertBatchDeleteGroup {
    pub source_type: RetailKLinePriceAlertSourceType,
    pub rule_ids: Vec<u64>,
}

/// State for price alert interactor
#[derive(Debug, Clone, PartialEq)]
pub struct PriceAlertState {
    /// All price alerts for the current instrument/token
    pub alerts: Vec<RetailKLinePriceAlert>,
    /// Grouped price alerts (for list view)
    pub groups: Vec<RetailKLinePriceAlertGroup>,
    /// True while loading alerts or groups
    pub is_loading: bool,
    /// True while a mutation (add/update/delete/activate) is in progress
    pub is_mutating: bool,
    /// Last loading error message
    pub error: Option<String>,
    /// Last mutation error message
    pub mutation_error: Option<String>,
    /// Current alert count
    pub alert_count: u32,
    /// Maximum alert count (typically 50)
    pub maximum_count: u32,
    /// Monotonic counter bumped every time an alerts load COMPLETES (success or
    /// error). Native consumes its pending list callbacks on generation change —
    /// edge-detecting `is_loading` alone can miss edges: the state observer reads
    /// state at callback time (not a snapshot), so a fast load may show `false`
    /// for both notifications and the pending callback would hang forever.
    pub alerts_generation: u64,
    /// Same as `alerts_generation`, for groups loads.
    pub groups_generation: u64,
    /// Metadata readiness latch. `false` until either the exact instrument or a
    /// parser-backed domain fallback has resolved. Raw instId quote segments are
    /// never exposed as units.
    pub is_metadata_ready: bool,
    /// Instrument ID ("BTC-USDT"). Set synchronously at CeFi factory time.
    pub inst_id: String,
    /// Biz line ("SPOT"/"MARGIN"/"SWAP"/"FUTURES"). Set synchronously at CeFi factory time.
    pub biz_type: String,
    /// Display title. OC: `instrument.instTitle`.
    pub inst_title: String,
    /// Base currency symbol ("BTC"). OC: `instrument.baseCcy`.
    pub base_ccy: String,
    /// Coin ccy symbol ("BTC", NOT the "Bitcoin" display name). OC: `instrument.coinName`.
    pub coin_name: String,
    /// Display price unit. iOS V1 `OKKPriceAlertCeFiHandler.unit()`:
    /// spot/margin → quote symbol; inverse contract → USD; positive contract →
    /// quote symbol. Settlement/margin currency is deliberately not a price unit.
    pub price_unit: String,
    /// Inverse (coin-margined) contract. OC: `instrument.isNegative`.
    /// Drives the inverse USD→fiat conversion branch in the price input container.
    pub is_negative: bool,
    /// Instrument family ("BTC-USD"). Feeds `kline_inverse_price_format`'s
    /// per-(inst_family, iso_code) fiat precision lookup.
    pub inst_family: String,
    /// Charge-currency price precision. OC handler `priceDigit()` =
    /// `createChargeConvertor(withInstFamily:).quotePrecision`. NOT tickSz-derived.
    pub quote_precision: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PriceAlertVo {
    pub alert_id: String,
    pub trigger_price: String,
    pub direction: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PriceFloatWindowState {
    /// Currently visible items, with live price_recipe / change_percent_recipe
    /// (empty recipes until the first ticker frame for that inst_id arrives).
    pub visible_items: Vec<PriceWindowItemVo>,
    /// True between subscribe_visible call and the moment the Manager has
    /// finished setting up the consumer task (NOT when first frame arrives).
    pub is_subscribing: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PriceLineVo {
    pub price: String,
    pub kind: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PriceWindowItemVo {
    /// Instrument ID, e.g. "BTC-USDT".
    pub inst_id: String,
    /// Instrument type: "SPOT" | "SWAP" | "FUTURES" | "OPTION".
    /// V1 MARGIN is mapped to "SPOT" by the Android caller before reaching Rust.
    pub inst_type: String,
    /// Display label shown in the floating window; may differ from inst_id.
    pub display_id: String,
    /// Pre-formatted display title (canonical `trade_title_utils` rules);
    /// native resolves the `DisplayString` segments (same as the V2 header).
    /// The variant depends on the producing interactor — the two screens
    /// never share items: the FloatWindow Interactor fills the float variant
    /// (no type suffix — the window renders its own type label), the List
    /// Interactor fills the management variant (FUTURES expiry / X-Perps
    /// label, OPTION coin+expiry, SWAP `pr_market_label_swap` suffix cdkey).
    pub title: CTTradeTitle,
    /// Pre-formatted type label (V1 `KlineType.getTypeName`): SWAP perp cdkey,
    /// FUTURES delivery date / expiry, OPTION expiry date. Empty for SPOT /
    /// MARGIN (the float window gates those natively). Only the FloatWindow
    /// Interactor fills it; the List Interactor leaves it empty.
    pub type_label: CTTradeTitle,
    /// Leverage multiplier as a string; empty when not applicable.
    pub leverage: String,
    /// True if this instrument is in pre-market trading.
    pub is_pre_market: bool,
    /// True if this is an X-Perps (extended) contract.
    pub is_x_perps: bool,
    /// Price formatting recipe. Empty (MarketFormatRecipe::empty()) until first ticker tick.
    /// On the List Interactor (management page), always empty.
    pub price_recipe: MarketFormatRecipe,
    /// 24h change percentage formatting recipe. Empty until first ticker tick.
    pub change_percent_recipe: MarketFormatRecipe,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PriceWindowListState {
    /// Full list of instruments. price_recipe / change_percent_recipe are
    /// always empty in the List Interactor (it does not subscribe tickers).
    pub items: Vec<PriceWindowItemVo>,
    pub is_loading: bool,
    pub error: Option<String>,
}

/// Single probability point exposed to the UI.
#[derive(Debug, Clone, PartialEq)]
pub struct ProbabilityPoint {
    /// Probability value, 0.0..1.0.
    pub value: f32,
    /// Candle open timestamp (milliseconds).
    pub timestamp: i64,
    /// Whether the candle is confirmed (closed). Unconfirmed pushes drive the
    /// pulsing dot; confirmed ones append to the list.
    pub confirmed: bool,
}

/// Retail K-line asset data for spot cost line (spot only)
/// 
/// Converted from `AccountDetailData`.
/// Used to display cost line on spot K-line charts.
/// 
/// # OC Mapping
/// Maps to `OKKLineAssetModel` class
#[derive(Debug, Clone, PartialEq)]
pub struct RetailKLineAsset {
    /// Currency (e.g., "BTC", "ETH")
    pub ccy: String,
    /// Average holding price (raw value for calculations)
    pub avg_px: String,
    /// Whether this is long (spot is always long)
    pub is_long: bool,
    /// Profit amount (raw value)
    pub profit: String,
    /// Profit rate (raw value, percentage)
    pub profit_rate: String,
    /// Available balance (raw value)
    pub avail_bal: String,
    /// Frozen balance (raw value)
    pub frozen_bal: String,
    /// Cash balance (holdings, raw value from AccountDetailData.cash_bal)
    pub cash_bal: String,
    /// Spot balance (net purchase, raw value from AccountDetailData.spot_bal)
    pub spot_bal: String,
    /// Open average price (raw, from `AccountDetailData.open_avg_px`, divided by
    /// `quote_coin_last` the same way as `avg_px`).
    /// 
    /// Additive typed field (A2 gap closure): `avg_px` only carries whichever of
    /// open/accumulated cost the user's cost-price-type setting selected (see
    /// `KLineAssetConvertor::convert_single_asset`), so a consumer that needs BOTH
    /// bases has no typed source today. OKKLine's `TradeAssetDataConverter.convert`
    /// currently maps the single `avg_px` onto both `KlineSpotAverageBean.openAvgPx`
    /// and `.accAvgPx` (its own comment: "carries a single average price ...
    /// map the single value onto both"). This field lets that mapping carry the
    /// real open-cost value instead of duplicating whichever basis was selected.
    pub open_avg_px: String,
    /// Accumulated average price (raw, from `AccountDetailData.acc_avg_px`,
    /// divided by `quote_coin_last` the same way as `avg_px`).
    /// 
    /// Additive typed field (A2 gap closure): counterpart of `open_avg_px` — see
    /// its doc comment for the consumer gap this closes.
    pub acc_avg_px: String,
    /// Raw asset data (JSON string for business operations)
    pub raw_data: Option<String>,
    /// Formatted cash balance (truncated by trade_precision / lotSz)
    /// OC: `[instCtx.tradeConvertor targetFormatter:cashBal]`
    pub formatted_cash_bal: String,
    /// Formatted spot balance (truncated by trade_precision / lotSz)
    pub formatted_spot_bal: String,
    /// Formatted available balance (truncated by trade_precision / lotSz)
    pub formatted_avail_bal: String,
    /// Formatted frozen balance (truncated by trade_precision / lotSz)
    pub formatted_frozen_bal: String,
    /// Formatted average price (rounded by quote_precision / tickSz)
    /// OC: `[instCtx.chargeConvertor toQuoteFormated:avgPx]`
    pub formatted_avg_px: String,
    /// Formatted profit with sign prefix (+/-)
    /// OC: `[assetConvertor toQuoteFormated:pnl]`
    pub formatted_profit: String,
    /// Formatted profit rate for display (percentage string, e.g. "-0.82%").
    /// Populated by interactor, empty from convertor.
    pub formatted_profit_rate: String,
}

/// Retail K-line attach order (TP/SL order)
/// 
/// Represents a take-profit or stop-loss order attached to a main order or position.
/// 
/// # OC Mapping
/// Maps to `OKKLineAttachOrder` class
#[derive(Debug, Clone, PartialEq)]
pub struct RetailKLineAttachOrder {
    /// Attach order ID
    pub id: String,
    /// Attach order type (TP/SL)
    pub attach_type: AttachOrderType,
    /// Order side (opposite to master order)
    /// 
    /// OC: attachOrder.side = (order.side == Buy) ? Sell : Buy
    pub side: KLineOrderSide,
    /// Trigger price (raw value)
    pub trigger_px: String,
    /// Trigger price type (last/index/mark)
    pub trigger_px_type: Option<TriggerPriceType>,
    /// Order price (raw value, None for market price)
    /// 
    /// OC: tpOrdPx / slOrdPx
    pub price: Option<String>,
    /// Order price (formatted, None for market price)
    /// TODO: YHB - formatting should be done in native/CLI layer
    pub ord_px: Option<String>,
    /// Order size (raw value)
    pub size: String,
    /// Formatted size for display (sz * ctVal for swap/futures).
    /// Populated by interactor, empty from convertor.
    pub formatted_size: String,
    /// Whether this is an all-position TP/SL order
    /// 
    /// OC: isAllPositionTPSL
    pub is_all_position_tpsl: bool,
    /// Master order ID that this attach order belongs to
    /// 
    /// Used for reverse lookup. Native side should manage the actual reference.
    /// 
    /// OC: masterOrder.id
    pub master_order_id: Option<String>,
    /// Profit amount when triggered (raw value)
    /// TODO: YHB - formatting should be done in native/CLI layer
    pub profit_amount: Option<String>,
    /// Profit rate when triggered (raw value, percentage)
    /// TODO: YHB - formatting should be done in native/CLI layer
    pub profit_rate: Option<String>,
    /// Raw order data (JSON string for business operations)
    pub raw_data: Option<String>,
}

/// Retail K-line order data for chart order line display
/// 
/// Converted from `UnifiedOrder` (GroupOrderRecord/TradeOrderData/StrategyOrderData).
/// Fields are pre-formatted for direct UI display.
/// 
/// # OC Mapping
/// Maps to `OKKLineOrder` class
#[derive(Debug, Clone, PartialEq)]
pub struct RetailKLineOrder {
    /// Order ID (unified: ord_id for normal orders, algo_id for algo orders)
    pub id: String,
    /// Instrument ID (e.g., "BTC-USDT-SWAP")
    pub inst_id: String,
    /// Instrument type (e.g., "SPOT", "SWAP", "FUTURES", "MARGIN", "OPTION")
    pub inst_type: String,
    /// Order type
    /// 
    /// K-line specific order type
    pub order_type: KLineOrderType,
    /// Order side
    /// 
    /// K-line specific order side
    pub side: KLineOrderSide,
    /// Position side
    /// 
    /// K-line specific position side
    pub pos_side: KLinePositionSide,
    /// Order price (formatted, "-1" means market price)
    pub price: String,
    /// Order size (raw contract count for swap/futures)
    pub size: String,
    /// Order leverage (drives the drag-time earning/margin PnL ratio).
    /// Empty for spot.
    pub lever: String,
    /// Formatted size for display (sz * ctVal for swap/futures, raw for spot).
    /// Populated by interactor, empty from convertor.
    pub formatted_size: String,
    /// Trading unit (e.g., "BTC", "Cont", "USDT")
    pub unit: Option<String>,
    /// Raw notional USD value from upstream order payload.
    pub notional_usd: String,
    /// Target currency of a spot market order's `sz` (`"base_ccy"` /
    /// `"quote_ccy"`, raw `tgtCcy` from the source order). Selects the
    /// base-vs-quote convertor when formatting the order amount, mirroring OC
    /// `OKTCOrdersUtil.spotOrMarginHYHAmountInMarket:...tgtCcy:`.
    /// None/empty for contract orders and non-market spot orders.
    /// `#[uniffi(default = None)]` so existing native constructors of this
    /// Record stay source-compatible (the field was added after the fact).
    pub tgt_ccy: Option<String>, // default = None
    /// Trigger price (for TP/SL/Trigger/Trailing orders)
    /// 
    /// Used for drawing on K-line chart
    pub trigger_px: Option<String>,
    /// Trigger price mode (last/index/mark)
    pub trigger_px_mode: KLinePriceMode,
    /// Raw trigger price type from upstream (`last`/`index`/`mark`).
    /// Empty means V1 does not append a trigger price label.
    pub trigger_px_type: String,
    /// Active price (for trailing stop orders)
    pub active_px: Option<String>,
    /// Callback ratio for trailing stop orders (`callbackRatio`).
    pub callback_ratio: String,
    /// Callback spread for trailing stop orders (`callbackSpread`).
    pub callback_spread: String,
    /// Whether this is a long position
    /// 
    /// Determined by side and pos_side:
    /// - Spot: side == Buy
    /// - Contract net mode: side == Buy
    /// - Contract long/short mode: pos_side == Long
    pub is_long: bool,
    /// Whether this is an all-position TP/SL order
    pub is_all_position_tpsl: bool,
    /// Whether this trigger order uses Chase Limit after activation.
    /// Derived from the chase strategy type or non-empty `advChaseParams`.
    pub is_plan_chase: bool,
    /// Whether this order already has a take-profit leg attached
    /// (feeds the chart's `haveTP`).
    pub support_tp: bool,
    /// Whether this order already has a stop-loss leg attached
    /// (feeds the chart's `haveSL`).
    pub support_sl: bool,
    /// Attached TP/SL orders (not including temporary ones)
    /// 
    /// OC: attachOrders
    pub attach_orders: Vec<RetailKLineAttachOrder>,
    /// Position ID that this order is associated with (for position-linked TP/SL orders)
    /// 
    /// Used for reverse lookup. Native side should manage the actual reference.
    /// 
    /// OC: position.id
    pub position_id: Option<String>,
    /// Asset ID that this order is associated with (for spot cost line orders)
    /// 
    /// Used for reverse lookup. Native side should manage the actual reference.
    /// 
    /// OC: asset.id
    pub asset_id: Option<String>,
    /// Profit amount when triggered (formatted)
    pub profit_amount: Option<String>,
    /// Profit rate when triggered (formatted, percentage)
    pub profit_rate: Option<String>,
    /// Margin collateral currency (raw, from `TradeOrderData.ccy` /
    /// `GroupOrderRecord.ccy`). Selects the base-vs-quote branch when computing
    /// a MARGIN attach-order's PnL amount. None/empty for non-margin orders.
    pub margin_ccy: Option<String>,
    /// Order creation time (Unix timestamp in milliseconds)
    pub c_time: Option<String>,
    /// Raw order data (JSON string for business operations)
    pub raw_data: Option<String>,
    /// Fixed-income dynamic-trigger-price fields, mirrored from the source algo order
    /// (`StrategyOrderData` / `GroupOrderRecord`). A non-empty `*_type` marks a fixed-income TP/SL
    /// order whose chart line must not be drag-modified. The `*_val` fields are value-only and must
    /// not drive that decision. Empty for non-TP/SL orders.
    pub tp_dynamic_trigger_px_type: Option<String>,
    pub tp_dynamic_trigger_px_val: Option<String>,
    pub sl_dynamic_trigger_px_type: Option<String>,
    pub sl_dynamic_trigger_px_val: Option<String>,
}

/// Retail K-line position data for cost line display
/// 
/// Converted from `PositionData`.
/// Fields are pre-formatted for direct UI display.
/// 
/// # OC Mapping
/// Maps to `OKKLinePosition` class
#[derive(Debug, Clone, PartialEq)]
pub struct RetailKLinePosition {
    /// Position ID
    pub id: String,
    /// Instrument ID (e.g., "BTC-USDT-SWAP")
    pub inst_id: String,
    /// Instrument type (e.g., "SWAP", "FUTURES", "MARGIN", "OPTION")
    pub inst_type: String,
    /// Position side
    /// 
    /// K-line specific position side
    pub pos_side: KLinePositionSide,
    /// Position quantity (raw contract count for swap/futures)
    pub pos: String,
    /// Formatted position size for display (pos * ctVal for swap/futures, raw for margin).
    /// Populated by interactor, empty from convertor.
    pub formatted_size: String,
    /// Average open price / cost (formatted)
    pub avg_px: String,
    /// Break-even price (formatted)
    pub balance_px: Option<String>,
    /// Liquidation price (formatted)
    pub liq_px: Option<String>,
    /// Last traded price (formatted)
    pub last: Option<String>,
    /// Unrealized PnL (formatted)
    pub upl: String,
    /// Unrealized PnL ratio (raw ratio, e.g. "0.0082" for 0.82%)
    pub upl_ratio: String,
    /// Formatted PnL ratio for display (percentage string, e.g. "+0.82%").
    /// Populated by interactor, empty from convertor.
    pub formatted_profit_rate: String,
    /// Formatted PnL amount for display (precision-formatted with sign, e.g. "+1.23").
    /// Populated by interactor, empty from convertor.
    /// Native: profitWithTrade → digitConvertor → prependPlusSign
    pub formatted_profit: String,
    /// Formatted average cost price for display (precision-formatted, e.g. "0.09170").
    /// Populated by interactor, empty from convertor.
    /// Native: OKChartDigitSeparatorPlain(cost, quotePrecision)
    pub formatted_cost: String,
    /// Margin mode: "cross" or "isolated"
    /// 
    /// Needed by Android's `connectCostAndOrder` to determine cost-order association.
    /// Maps from `PositionData.mgn_mode`.
    pub mgn_mode: String,
    /// Leverage (not applicable for options)
    /// 
    /// Needed by Android's `calculatePnlPos` for PnL calculation.
    /// Maps from `PositionData.lever`.
    pub lever: String,
    /// Initial margin requirement (raw, from `PositionData.imr`).
    /// 
    /// Formatted downstream by the trade-overlay pipeline; shown in the K-line
    /// position tip panel for cross-margin positions.
    pub imr: String,
    /// Margin balance (raw, from `PositionData.margin`).
    /// 
    /// Formatted downstream by the trade-overlay pipeline; shown in the K-line
    /// position tip panel for isolated-margin positions.
    pub margin: String,
    /// Settlement currency (raw, from `PositionData.ccy`).
    /// 
    /// Selects the margin display symbol (trade vs quote) for MARGIN positions.
    pub ccy: String,
    /// Position currency for margin trading (raw, from `PositionData.pos_ccy`).
    /// 
    /// Consumed by the K-line "add TP/SL without confirm" Rust bridge
    /// (`KLineAddPositionTpslInputVo.posCcy`, MARGIN max-available context) and by
    /// the Pure position-TPSL platform adapter (`PositionTpslLegacyPlatformInput.posCcy`).
    pub pos_ccy: String,
    /// Interest accrued but not yet deducted (raw, from `PositionData.interest`).
    /// 
    /// Consumed by `RetailKLinePosition -> OktPositionData.interest` (the shared
    /// standalone-position mapper for close / add-TP&SL / reverse) and by
    /// `PositionTpslLegacyPlatformInput.interest`.
    pub interest: String,
    /// Liabilities for margin trading (raw, from `PositionData.liab`).
    /// 
    /// Consumed by the Pure position-TPSL platform adapter
    /// (`PositionTpslLegacyPlatformInput.liab`).
    pub liab: String,
    /// Available position size for closing, buy/sell mode only (raw, from
    /// `PositionData.avail_pos`).
    /// 
    /// Consumed by `RetailKLinePosition -> OktPositionData.availPos` (the shared
    /// standalone-position mapper for close / add-TP&SL / reverse).
    pub avail_pos: String,
    /// Underlying (raw, from `PositionData.uly`).
    /// 
    /// Consumed by `RetailKLinePosition -> OktPositionData.uly` (the shared
    /// standalone-position mapper for close / add-TP&SL / reverse). The Rust
    /// interactor itself passes `underlying = ""` and resolves the index-
    /// subscription underlying via `uly -> inst_family -> biz lookup`, so `uly`
    /// must be populated for that fallback chain to work standalone.
    pub uly: String,
    /// Instrument family (raw, from `PositionData.inst_family`).
    /// 
    /// Consumed by `RetailKLinePosition -> OktPositionData.instFamily` (the shared
    /// standalone-position mapper for close / add-TP&SL / reverse).
    pub inst_family: String,
    /// Whether this is a long position
    /// 
    /// Determined by pos_side: pos_side == Long
    pub is_long: bool,
    /// Whether this position supports take-profit
    pub support_tp: bool,
    /// Whether this position supports stop-loss
    pub support_sl: bool,
    /// Whether this position supports reverse (open opposite position)
    pub support_revert: bool,
    /// Whether this is an EEA timed-growth-voucher position (biz_ref_type ==
    /// "timed_growth_coupon"). The chart hides close/reverse/TP/SL for it.
    pub is_experience_coupon: bool,
    /// Associated TP/SL orders for this position (not including temporary ones)
    /// 
    /// Orders that are attached to this position (e.g., take-profit, stop-loss).
    /// These orders will be displayed alongside the position on the K-line chart.
    /// 
    /// OC: attachOrders
    pub attach_orders: Vec<RetailKLineOrder>,
    /// Raw position data (JSON string for business operations)
    pub raw_data: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RetailKLinePriceAlert {
    /// rule id
    pub rule_id: Option<u64>,
    /// source type
    pub source_type: Option<RetailKLinePriceAlertSourceType>,
    /// prompt type
    pub prompt_type: Option<RetailKLinePriceAlertPromptType>,
    /// repeat type
    pub repeat_type: Option<RetailKLinePriceAlertRepeatType>,
    /// biz type
    pub biz_type: Option<RetailKLinePriceAlertBizType>,
    /// active
    pub active: bool,
    /// prompt threshold
    pub prompt_threshold: Option<String>,
    /// customized message
    pub customized_message: Option<String>,
    /// timestamp of modify time, unit is milliseconds
    pub modify_time: Option<u64>,
    /// timestamp of create time, unit is milliseconds
    pub create_time: Option<u64>,
    /// instrument name
    pub instrument_name: Option<String>,
    /// token
    pub token: Option<String>,
    /// token address
    pub token_address: Option<String>,
    /// chain id
    pub chain_id: Option<String>,
    /// chain logo url
    pub chain_logo_url: Option<String>,
    /// token logo url
    pub token_logo_url: Option<String>,
    /// asset id — ExchangeOS DEX perp/spot identity (sourceType=5). Uniquely identifies a
    /// spot/perp; the read-side ("has alert") key. Empty for CeFi / legacy DEX swap.
    /// `#[uniffi(default = None)]` so foreign constructors of this Record need not pass it
    /// (matches `name` / `exchange_os_biz_type`).
    pub asset_id: Option<String>, // default = None
    /// entity id for indicator alerts
    pub entity_id: Option<u64>,
    /// display id
    pub display_id: Option<String>,
    /// condition 1 (e.g., "above", "below")
    pub condition1: Option<String>,
    /// condition 2 (e.g., threshold value)
    pub condition2: Option<String>,
    /// time period for indicator (e.g., MA period)
    pub time_period: Option<u32>,
    /// indicator type (e.g., "MA", "RSI")
    pub indicator_type: Option<String>,
    /// preset condition: "0"=Custom, "1"=MA-Bullish, "2"=MA-Bearish
    pub preset_condition: Option<String>,
    /// signal type
    pub signal_type: Option<String>,
    /// display name for ExchangeOS DEX perp/spot (sourceType=5); server backfills if absent.
    /// `#[uniffi(default = None)]` so existing native constructors of this Record stay
    /// source-compatible (this field is appended last — ABI-safe, no call-site churn).
    pub name: Option<String>, // default = None
    /// dexps biz type for ExchangeOS write path (0=spot, 1=perp). Separate dict from PriceAlertBizType.
    /// `#[uniffi(default = None)]` so existing native constructors of this Record stay
    /// source-compatible (appended last — ABI-safe).
    pub exchange_os_biz_type: Option<u32>, // default = None
}

#[derive(Debug, Clone, PartialEq)]
pub struct RetailKLinePriceAlertGroup {
    /// Name of the inst name for CeFi,
    /// or token symbol for CeDeFi
    pub inst_id: Option<String>,
    /// Alerts in the group
    pub alerts: Vec<RetailKLinePriceAlert>,
    /// For CeFi, it is the biz type
    /// For CeDeFi, it is None
    pub biz_type: Option<RetailKLinePriceAlertBizType>,
    /// For CeFi, it is the inst_id
    /// For CeDeFi, it is the chain_id + token_address
    pub unique_id: Option<String>,
    /// For CeDeFi, it is the chain_id
    pub chain_id: Option<String>,
    /// For CeDeFi, it is the token_address
    pub token_address: Option<String>,
    /// For CeFi, it is None
    /// For CeDeFi, it is the token_logo_url
    pub token_logo_url: Option<String>,
    /// For CeDeFi, it is the chain_logo_url
    pub chain_logo_url: Option<String>,
}

/// Resolved display + routing for the bottom second icon.
#[derive(Debug, Clone, PartialEq)]
pub struct SecondIconPlan {
    pub kind: SecondIconKind,
    pub visible: bool,
    pub clickable: bool,
    /// Jump target biz type; empty when no entry matched.
    pub jump_inst_type: String,
    /// Jump target inst id; empty when no entry matched.
    pub jump_inst_id: String,
    /// Backend tradability flag of the jump target (`simpleProduct.isTradable`),
    /// threaded onto the Perp/Contracts (SWAP) entry so `KlineBottomViewModel`
    /// (V2) can hide the bottom Contracts entry for the "visible-but-not-tradable"
    /// compliance scenario (B-1 / TEPT-6547).
    /// 
    /// `Some(false)` -> target SWAP explicitly not tradable.
    /// `Some(true)`  -> tradable. `None` -> unknown -> treated as tradable (AC-7).
    /// Always `None` for the Spot entry (only the Perp/Contracts entry is gated).
    pub tradable: Option<bool>,
}

/// One row of the session list (drawn in the middle of the bottom sheet).
#[derive(Debug, Clone, PartialEq)]
pub struct SessionVo {
    /// Backend-hosted icon URL for this row (LIGHT/day theme asset); empty
    /// when the backend omitted it. Native loads the URL when non-empty,
    /// otherwise keeps the icon slot invisible (layout preserved) — there is
    /// no local drawable fallback.
    pub icon_url: String,
    /// DARK/night-theme icon URL. Native picks this one when the app is in
    /// dark mode; empty → fall back to [`Self::icon_url`] for both themes.
    pub icon_url_dark: String,
    /// Backend-localized display name, e.g. "Pre-market".
    pub name: String,
    /// Backend-localized liquidity label, e.g. "Moderate liquidity".
    pub liquidity_label: String,
    /// Enum → Native looks up the coloured bar icon.
    pub liquidity_level: LiquidityLevel,
    /// "08:00 - 13:30" in device-local time.
    pub time_range: String,
    /// Session window start, epoch ms. Geometry data for the progress bar —
    /// segment widths are proportional to (end - start).
    pub start_ms: i64,
    /// Session window end, epoch ms (exclusive).
    pub end_ms: i64,
    /// Raw backend `sessionCode` string, passed through verbatim. TRANSITIONAL:
    /// only consumed by the native progress-bar color bridge until the backend
    /// ships `colorType` (see [`Self::color_type`]); no client-side semantics
    /// are attached to specific values.
    pub session_code: String,
    /// Per-session color selector for the progress-bar segment, passed through
    /// verbatim. Backend enum: `YELLOW` / `GREEN` / `PURPLE` / `BLUE` / `GRAY`.
    /// Native maps it to a theme color resource; empty/unknown values fall back
    /// to the transitional sessionCode-based color bridge (delete the bridge
    /// once the field is fully rolled out and the renderers are VO-driven).
    pub color_type: String,
    /// Whether this row is the currently-active session.
    pub is_active: bool,
}

/// `GET /priapi/v5/rubik/public/tradfi/shareholders/stats?ticker=`.
#[derive(Debug, Clone, PartialEq)]
pub struct ShareholdersStats {
    pub symbol: Option<String>,
    pub period: Option<String>,
    pub summary: Option<ShareholdersSummary>,
    /// Every holder, in API (rank) order — backs the holders table.
    pub top_holders: Vec<TopHolder>,
    /// Donut breakdown (top-N named + "Others"). Derived by [`Self::finalize`].
    pub donut: Vec<DonutSlice>,
    /// Ownership history, normalized oldest-first by [`Self::finalize`] (the API
    /// returns latest-first). Chart plots left→right = oldest→newest.
    pub inst_history: Vec<InstHistoryPoint>,
}

/// Institution ownership summary (three header metrics + their QoQ changes).
#[derive(Debug, Clone, PartialEq)]
pub struct ShareholdersSummary {
    /// Institutional ownership (raw decimal, `"0.6346"` = 63.46%).
    pub inst_pct: Option<String>,
    pub inst_pct_change: Option<String>,
    pub inst_shares: Option<String>,
    pub inst_shares_change: Option<String>,
    pub inst_holders: Option<String>,
    pub inst_holders_change: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SpotAverageVo {
    pub avg_price: String,
    pub size: String,
}

/// Result type for start operation
#[derive(Debug, Clone, PartialEq)]
pub struct StartResult {
    /// Indicates if the operation was successful
    pub success: bool,
}

/// Top-level VO for the CEX stock-hours bottom sheet.
#[derive(Debug, Clone, PartialEq)]
pub struct StockHoursVo {
    /// Sheet title, e.g. "US stocks stock trading hours" — backend-localized.
    pub title: String,
    /// Sheet body copy, fully rendered (no placeholders) — backend-localized.
    pub description: String,
    /// Timezone footer, e.g. "Times shown in your local time (UTC+8)." — Rust
    /// fills the device-local offset into the backend template.
    pub timezone_footer: String,
    /// One row per session (Pre-market / Open / After-hours / Overnight / …).
    pub sessions: Vec<SessionVo>,
    /// Coarse state of the current window — drives banner + holiday layout.
    pub current_state: CurrentState,
    /// Session label for the currently-active row (e.g. "Pre-market").
    pub current_session_name: String,
    /// "08:00 - 13:30" in device-local time.
    pub current_time_range: String,
    /// Progress-bar cursor position within the active session, `[0.0, 1.0]`.
    pub progress_ratio: f32,
    /// Fully-rendered subtitle countdown, e.g. "US stocks market opens in 3h 19m".
    pub countdown_text: String,
    /// Populated only when the current window announces an early close.
    pub early_close: Option<EarlyCloseVo>,
    /// Populated only during a holiday closure.
    pub holiday: Option<HolidayVo>,
}

/// One session row for the bottom-sheet schedule. Timestamps are Unix ms;
/// native formats them (UTC `HH:mm`) — formatting stays native.
#[derive(Debug, Clone, PartialEq)]
pub struct StockSessionRowVo {
    pub status: StockSessionStatus,
    pub start_ms: i64,
    pub end_ms: i64,
}

/// One share split / reverse split entry.
#[derive(Debug, Clone, PartialEq)]
pub struct StockSplit {
    pub ex_date: Option<String>,
    /// Denominator, e.g. `1` in a 4-for-1 split.
    pub split_from: Option<String>,
    /// Numerator, e.g. `4` in a 4-for-1 split.
    pub split_to: Option<String>,
    /// `STOCK_SPLIT` / `REVERSE_SPLIT`.
    pub split_type: Option<String>,
}

/// One swiping-ribbon entry: identity plus the instrument metadata needed to
/// render its tag / leverage / company-name / stock-hours strip. `meta` is
/// resolved per item so native renders each ribbon cell without any extra
/// instrument lookup. `title` is the pre-formatted display title (canonical
/// `TradeTitleUtils` rules); native resolves its `DisplayString` segments.
#[derive(Debug, Clone, PartialEq)]
pub struct SwipingInstrumentVo {
    pub inst_id: String,
    pub inst_type: String,
    pub meta: HeadMetaVo,
    pub title: CTTradeTitle,
}

/// One top-holder row (the holders table shows every entry, in API order).
#[derive(Debug, Clone, PartialEq)]
pub struct TopHolder {
    pub rank: Option<String>,
    pub holder_name: Option<String>,
    /// Share of total (raw decimal, `"0.0914"` = 9.14%).
    pub shares_pct: Option<String>,
    /// QoQ change in shares held (signed; `+` increase / `-` decrease).
    pub change_shares: Option<String>,
}

/// Trading indicator data point DTO (single data point - DTO)
/// 
/// Corresponds to HTTP API response data structure
/// 
/// # Data Format Description
/// Different indicators have different data formats:
/// - `oi`: [timestamp, positionOI]
/// - `eliteLSOIRatio`: [timestamp, eliteLongOI, eliteShortOI, eliteOIRatio]
/// - `eliteLSAccountRatio`: [timestamp, eliteLongAccount, eliteShortAccount, eliteAccountRatio]
/// - `LSAccountRatio`: [timestamp, longAccountRatio, shortAccountRatio, lsAccountRatio]
/// - `takerBuySellVol`: [timestamp, takerBuyVol, takerSellVol]
/// 
/// # Performance Note
/// The `timestamp` field uses `i64` for optimal performance in merge operations.
/// 
/// **Design:**
/// - Uses `serde_with::PickFirst` to handle both string and number timestamps from backend
/// - First tries to parse as `i64` directly (when backend sends number)
/// - Falls back to `DisplayFromStr` (when backend sends string like `"1640000000000"`)
/// - UniFFI supports `i64` natively for Swift/Kotlin FFI
/// - Eliminates repeated `parse::<i64>()` calls in `merge_data_points()`
/// 
/// **Example JSON (both supported):**
/// ```json
/// ["1640000000000", "12345.67"]  // timestamp as string
/// [1640000000000, "12345.67"]    // timestamp as number
/// ```
/// After deserialization: `timestamp: 1640000000000_i64`
#[derive(Debug, Clone, PartialEq)]
pub struct TradingIndicatorDataPoint {
    /// Timestamp in milliseconds (accepts both string and number from JSON)
    pub timestamp: i64,
    /// Open interest in the user-selected unit (only oi indicator has value)
    pub position_oi: String,
    /// Open interest denominated in USDT, always populated alongside `position_oi`
    /// regardless of the user-selected unit. Native side displays this as the
    /// secondary USDT value in the OI info popup.
    pub position_oi_usdt: String,
    /// Elite long position (only eliteLSOIRatio indicator has value)
    pub elite_long_oi: String,
    /// Elite short position (only eliteLSOIRatio indicator has value)
    pub elite_short_oi: String,
    /// Elite position ratio (only eliteLSOIRatio indicator has value)
    pub elite_oi_ratio: String,
    /// Elite long account count (only eliteLSAccountRatio indicator has value)
    pub elite_long_account: String,
    /// Elite short account count (only eliteLSAccountRatio indicator has value)
    pub elite_short_account: String,
    /// Elite account ratio (only eliteLSAccountRatio indicator has value)
    pub elite_account_ratio: String,
    /// Long account percentage (only LSAccountRatio indicator has value)
    pub long_account_ratio: String,
    /// Short account percentage (only LSAccountRatio indicator has value)
    pub short_account_ratio: String,
    /// Long/short account ratio value (only LSAccountRatio indicator has value)
    pub ls_account_ratio: String,
    /// Taker buy volume (only takerBuySellVol indicator has value)
    pub taker_buy_vol: String,
    /// Taker sell volume (only takerBuySellVol indicator has value)
    pub taker_sell_vol: String,
}

/// Trading indicator result DTO (DTO returned to Swift/Kotlin)
/// 
/// This is the final result type exposed to UniFFI
/// 
/// # Design Notes
/// Grouped by indicator type, each indicator has its own array, avoiding mixed storage
#[derive(Debug, Clone, PartialEq)]
pub struct TradingIndicatorResult {
    /// Trading pair ID
    pub inst_id: String,
    /// Open interest data
    pub open_interest: Vec<TradingIndicatorDataPoint>,
    /// Elite long/short account ratio data
    pub elite_long_short_account_ratio: Vec<TradingIndicatorDataPoint>,
    /// Elite long/short position ratio data
    pub elite_long_short_position_ratio: Vec<TradingIndicatorDataPoint>,
    /// Long/short account ratio data
    pub long_short_account_ratio: Vec<TradingIndicatorDataPoint>,
    /// Taker buy/sell volume data
    pub taker_buy_sell_volume: Vec<TradingIndicatorDataPoint>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TradingIndicatorState {
    /// Version counter for data cache — bumped on every data update.
    /// Native observes this and calls `get_data()` when it changes.
    pub data_version: u64,
    /// Whether initial HTTP data is loading
    pub is_loading: bool,
    /// Error message (if any)
    pub error: Option<String>,
}

// -----------------------------------------------------------------------------
// 3. Interfaces / Interactors / Services (161)
// -----------------------------------------------------------------------------

/// Generic single-chart interactor. Drives exactly one chart, identified by the
/// `chart_id` captured in state. Runtime instrument context (`inst_id` /
/// `index_inst_id`) arrives via [`BigDataChartActions::configure`] and lives in
/// state (see those fields).
/// 
/// `configure` is one-shot (Native rebuilds the card to switch instrument), so
/// there is intentionally **no** `context_id` generation guard here — the
/// `AlwaysCancel` `Load` effect alone serialises period switches.
pub struct BigDataChartInteractor;
impl BigDataChartInteractor {
    /// Pull the cached payload (call after a `version` bump).
    pub fn chart_data(&self) -> Option<BigDataChartResult> { todo!() }
    /// Set the runtime instrument context used by later loads to build requests.
    /// 
    /// `inst_id`: current instrument id (e.g. "BTC-USDT-SWAP").
    /// `index_inst_id`: index instrument id (e.g. "BTC-USD"), needed for the
    /// FuturesSpotGap request.
    /// 
    /// Biz-line / visible-charts / supports-lever are intentionally NOT here —
    /// those live in the Native layer for the single-chart design.
    pub fn configure(&self, inst_id: String, index_inst_id: String) { todo!() }
    /// Feed an already-fetched `result` in directly, **without** issuing a
    /// network request. Writes the payload to the cache, records `period` (when
    /// `Some`), clears `loading` / `failed`, and bumps `version` — exactly the
    /// success path of a load, minus the effect/use-case.
    /// 
    /// Enables sharing one request across charts: e.g. the two leverage charts
    /// (`MarginLongShortRatio` / `MarginBorrowVolume`) share one loan-ratio
    /// response — one loads it, then injects the same result into the other so
    /// the second chart skips its request.
    pub fn inject(&self, result: BigDataChartResult, period: Option<BigDataChartPeriod>) { todo!() }
    /// Unified load entry. Sets `current_period` (when a period is given) and
    /// `loading`, then dispatches on `chart_id` to the matching use case via the
    /// `Load` effect (`AlwaysCancel`, so a fresh load cancels the in-flight one).
    /// 
    /// `period` is `None` for charts without a period selector (`FundingRate`);
    /// it must otherwise be one of [`BigDataChartState::periods`]. For the
    /// BuySellVolume chart, prefer [`BigDataChartActions::load_with_unit`] (this
    /// entry uses the default `Coin` unit).
    pub fn load(&self, period: Option<BigDataChartPeriod>) { todo!() }
    /// BuySellVolume-only load that also selects the volume `unit`. For every
    /// other chart this behaves like [`BigDataChartActions::load`] (the `unit` is
    /// ignored).
    pub fn load_with_unit(&self, period: Option<BigDataChartPeriod>, unit: BuySellUnit) { todo!() }
    /// Cancel the in-flight load and clear the cached payload.
    pub fn on_clear(&self) { todo!() }
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
    /// Enable or disable debug logging.
    /// 
    /// In release builds this is a no-op, but the symbol must exist
    /// so that UniFFI bindings generated in debug mode continue to
    /// link successfully against the release library.
    pub fn set_debug(&self, _enabled: bool) { todo!() }
    /// Opt in to Rust-side quote scaling for `FuturesSpotGap`.
    /// 
    /// Default `false` emits raw server close prices. When `true`, inverse
    /// SWAP / FUTURES get both series multiplied by the fiat rate before they
    /// reach the cache, so native renders them as-is. No-op for other charts;
    /// takes effect on the next `load`.
    pub fn set_quote_scaling(&self, enabled: bool) { todo!() }
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
    pub fn state(&self) -> BigDataChartState { todo!() }
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
}

pub struct BigDataPageInteractor;
impl BigDataPageInteractor {
    /// Resolve business line, visible charts, instrument metadata and recipes.
    pub fn configure(&self, inst_id: String, inst_type: String) { todo!() }
    /// Detach the state observer so it can't keep the host alive.
    pub fn on_clear(&self) { todo!() }
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
    pub fn state(&self) -> BigDataPageState { todo!() }
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
}

/// Business Line Instrument Provider Bridge
/// 
/// Exposes UniFFI interface for loading instruments across all business lines
/// and retrieving instruments by business type.
/// 
/// # Example (Swift)
/// ```swift
/// // Create bridge
/// let bridge = BizInstrumentProviderBridge.new(instId: "BTC-USDT", bizType: "SPOT")
/// 
/// // Set callback (optional)
/// bridge.setCallback(callback: myCallback)
/// 
/// // Load all business lines
/// let future = bridge.load()
/// try await future.poll()
/// 
/// // Get instrument ID for SWAP
/// if let swapId = bridge.getInstId(bizType: "SWAP") {
/// print("SWAP: \(swapId)") // "BTC-USDT-SWAP"
/// }
/// 
/// // Get full instrument record
/// if let record = bridge.getInstrumentRecord(bizType: "SWAP") {
/// print("inst_id: \(record.instId)")
/// print("tick_sz: \(record.tickSz)")
/// }
/// ```
pub struct BizInstrumentProviderBridge;
impl BizInstrumentProviderBridge {
    /// Create a new bridge instance
    /// 
    /// # Parameters
    /// - `inst_id`: Instrument ID (e.g., "BTC-USDT")
    /// - `biz_type`: Business type (e.g., "SPOT", "SWAP", "FUTURES", "OPTION", "MARGIN")
    /// 
    /// # Returns
    /// An Arc-wrapped bridge instance
    pub fn new(inst_id: String, biz_type: String) -> Arc<BizInstrumentProviderBridge> { todo!() }
    /// Get instrument ID for a specific business type
    /// 
    /// # Parameters
    /// - `biz_type`: Business type (e.g., "SPOT", "SWAP", "FUTURES", "OPTION", "MARGIN")
    /// 
    /// # Returns
    /// - `Some(String)`: The instrument ID for the given business type
    /// - `None`: Not loaded yet or no instrument for this type
    pub fn get_inst_id(&self, biz_type: String) -> Option<String> { todo!() }
    /// Get current loading state
    /// 
    /// # Returns
    /// The current `BizLoadStateBridge`
    pub fn get_state(&self) -> BizLoadStateBridge { todo!() }
    /// Asynchronously load all business lines
    /// 
    /// Loads instruments for all 5 core business types:
    /// SPOT, MARGIN, SWAP, FUTURES, OPTION
    /// 
    /// # Returns
    /// A cancellable future that resolves to true when loading completes successfully
    pub fn load(&self) -> Arc<OKRustFutureLoadResultWithOKFFIError> { todo!() }
    /// Set callback for loading completion
    /// 
    /// # Parameters
    /// - `callback`: Implementation of `BizInstrumentProviderBridgeCallback`
    pub fn set_callback(&self, callback: Arc<dyn BizInstrumentProviderBridgeCallback>) { todo!() }
}

/// Callback interface for BizInstrumentProviderBridge (FFI-compatible)
/// 
/// Implement this trait in Swift/Kotlin to receive loading completion notifications.
/// 
/// # Swift Usage
/// ```swift
/// class MyCallback: BizInstrumentProviderBridgeCallback {
/// func onLoadCompleted(state: BizLoadStateBridge) {
/// switch state {
/// case .loaded: print("Loaded successfully")
/// case .error(let message): print("Error: \(message)")
/// default: break
/// }
/// }
/// }
/// ```
pub struct BizInstrumentProviderBridgeCallback;
impl BizInstrumentProviderBridgeCallback {
    /// Called when loading completes (success or failure)
    pub fn on_load_completed(&self, state: BizLoadStateBridge) { todo!() }
}

pub struct CandleAdjustmentInteractor;
impl CandleAdjustmentInteractor {
    /// Start the global candle-adjustment subscription. The channel is a full
    /// broadcast (no instId), so callers subscribe once and filter the emitted
    /// items by instId on the native side. Safe to call repeatedly.
    pub fn initialize(&self) { todo!() }
    /// Tear down: cancel all effects and detach observers. Called from
    /// native dealloc / page leave.
    pub fn on_clear(&self) { todo!() }
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
    /// DEPRECATED. Use `ui_event_stream()`; drop the returned stream
    /// wrapper to unsubscribe.
    pub fn remove_ui_event_observer(&self) { todo!() }
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
    /// DEPRECATED: callback-based observer. Use `ui_event_stream()` —
    /// the callback observer pins the foreign object in UniFFI's
    /// static handleMap and produces a cross-language retain cycle
    /// (Android GC + iOS ARC).
    pub fn set_ui_event_observer(&self, observer: Arc<dyn CandleAdjustmentUiEventObserver>) { todo!() }
    /// Start the interactor lifecycle.
    /// 
    /// Calls the user's `InteractorLifecycle::start()` implementation.
    /// Override that method to add custom startup behavior (e.g., initial data loads).
    pub fn start(&self) { todo!() }
    /// Fire a batch candle-adjustments HTTP query for multiple instIds (≤20).
    /// The in-flight batch request (if any) is cancelled first. Result arrives
    /// as `MultiRequestSucceeded` / `MultiRequestFailed`.
    pub fn start_multi_request(&self, queries: Vec<CandleAdjustmentQuery>) { todo!() }
    /// Stock-gated variant of `initialize` + [`start_multi_request`]: keeps
    /// only the stock legs (per-leg gate as [`start_request_for_stock`]) and
    /// stays fully inert when none survive — callers pass ALL legs unfiltered.
    pub fn start_multi_request_for_stock(&self, queries: Vec<CandleAdjustmentQuery>, stock_adjust_ab: bool) { todo!() }
    /// Fire a single-instId candle-adjustments HTTP query. The in-flight request
    /// (if any) is cancelled first, so calling this repeatedly on instrument
    /// switch is safe — only the latest result is emitted. Result arrives as
    /// `RequestSucceeded` / `RequestFailed`.
    pub fn start_request(&self, query: CandleAdjustmentQuery) { todo!() }
    /// Stock-gated variant of `initialize` + [`start_request`]: the business
    /// gate (stock-adjust AB gray && the leg resolves to an active stock
    /// adjustment — TradFi coin + persisted forward choice) lives HERE, so
    /// native attaches unconditionally with no `isTradFi` knowledge. When the
    /// gate fails the interactor stays fully inert: no WS subscription, no
    /// HTTP, no events — same cost as native not attaching at all.
    /// `stock_adjust_ab` is the native gray switch (not readable from Rust).
    pub fn start_request_for_stock(&self, query: CandleAdjustmentQuery, stock_adjust_ab: bool) { todo!() }
    /// Get the current state (UniFFI export).
    /// 
    /// Returns the External type (UniFFI-compatible).
    pub fn state(&self) -> CandleAdjustmentState { todo!() }
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
    /// Cancel the in-flight batch HTTP query, if any. Does not affect the WS
    /// subscription or the single query.
    pub fn stop_multi_request(&self) { todo!() }
    /// Cancel the in-flight single-instId HTTP query, if any. Does not affect
    /// the WS subscription or the batch query.
    pub fn stop_request(&self) { todo!() }
    /// Opt-in async UI event stream (cycle-free). Single-subscriber.
    pub fn ui_event_stream(&self) -> Arc<CandleAdjustmentUiEventStream> { todo!() }
}

/// Deprecated UniFFI callback interface for observing UI events. Prefer the cycle-free `CandleAdjustmentUiEventStream` returned by `interactor.ui_event_stream()`.
pub struct CandleAdjustmentUiEventObserver;
impl CandleAdjustmentUiEventObserver {
    pub fn on_ui_event(&self, event: CandleAdjustmentUiEvent) { todo!() }
}

/// Opt-in async event stream from `interactor.ui_event_stream()`.
/// Single-subscriber, latest-wins. `next()` resolves to `None` when the
/// interactor is dropped or a new subscription replaces this one.
pub struct CandleAdjustmentUiEventStream;
impl CandleAdjustmentUiEventStream {
    pub async fn next(&self) -> Option<CandleAdjustmentUiEvent> { todo!() }
}

/// Callback for candles stream updates
pub struct CandlesStreamCallback;
impl CandlesStreamCallback {
    /// Called when candles data is updated (from HTTP or WebSocket)
    /// 
    /// # Arguments
    /// * `candles` - List of candle data (None if no data available)
    /// * `is_from_http` - `true` if data is from HTTP initial request, `false` if from WebSocket
    pub fn on_update_candles(&self, candles: Option<Vec<BizKLineData>>, is_from_http: bool) { todo!() }
    /// Called when an error occurs during subscription
    pub fn on_update_error(&self, error: OKFFIError) { todo!() }
}

/// CeDeFi buy-sell marker bridge
/// 
/// Provides UniFFI-compatible API for subscribing to CeDeFi buy-sell marker data.
pub struct CedefiBsMarkerBridge;
impl CedefiBsMarkerBridge {
    /// Create a new bridge instance
    pub fn new() -> Arc<CedefiBsMarkerBridge> { todo!() }
    /// Subscribe to CeDeFi buy-sell marker stream
    /// 
    /// # Parameters
    /// - `request`: CeDeFi buy-sell marker request parameters
    /// - `ws_only`: Whether to skip HTTP and only subscribe to WebSocket
    /// - `false`: Fetch HTTP initial data first, then subscribe to WS.
    /// Each callback receives the full accumulated data list.
    /// - `true`: Only subscribe to WS push, skip HTTP.
    /// Each callback receives a single-element Vec with the new WS point.
    /// Native merges into its existing data. Use this for foreground resume.
    /// - `callback`: Callback to receive stream updates
    /// 
    /// # Returns
    /// A future that completes when the stream ends. Cancel this future to stop WS.
    /// 
    /// # Native Lifecycle Pattern
    /// 
    /// ```text
    /// // Initial load
    /// self.bsMarkerFuture = bridge.subscribe(req, ws_only: false, callback: self)
    /// 
    /// // Background (stop WS)
    /// self.bsMarkerFuture.cancel()
    /// 
    /// // Foreground resume (after candle HTTP ready, restart WS only)
    /// self.bsMarkerFuture = bridge.subscribe(req, ws_only: true, callback: self)
    /// ```
    pub fn subscribe_cedefi_bs_marker_stream(&self, request: CedefiBsMarkerReq, ws_only: bool, callback: Arc<dyn CedefiBsMarkerStreamCallback>) -> Arc<OKRustFutureboolWithOKFFIError> { todo!() }
}

/// Callback for CeDeFi buy-sell marker stream updates
/// 
/// Native platforms implement this callback to receive CeDeFi BS marker data.
/// 
/// # Full mode (`ws_only = false`)
/// 
/// 1. `on_update_data` is called first with the full HTTP initial data
/// 2. Subsequently, `on_update_data` is called with accumulated data (HTTP + WS merged)
/// 3. `on_update_completed` is called when the stream ends
/// 4. `on_update_error` is called if an error occurs
/// 
/// # WS-only mode (`ws_only = true`)
/// 
/// 1. `on_update_data` is called for each WS push with a single-element Vec
/// 2. Native should merge this point into its existing `buySellMarkersModelAry`
/// 3. `on_update_completed` / `on_update_error` work the same way
pub struct CedefiBsMarkerStreamCallback;
impl CedefiBsMarkerStreamCallback {
    /// Called when the stream is completed
    pub fn on_update_completed(&self) { todo!() }
    /// Called when CeDeFi BS marker data is updated
    /// 
    /// # Arguments
    /// * `data` - BS marker data points:
    /// - Full mode: Complete accumulated list
    /// - WS-only mode: Single-element Vec with just the new point
    pub fn on_update_data(&self, data: Vec<CedefiBsMarkerPoint>) { todo!() }
    /// Called when an error occurs during the stream
    pub fn on_update_error(&self, error: OKFFIError) { todo!() }
}

pub struct CompanyDividendInteractor;
impl CompanyDividendInteractor {
    /// Start a dividend-splits load for `ticker`. The in-flight load (if any) is
    /// cancelled first, so calling repeatedly on instrument switch is safe — only
    /// the latest result is emitted.
    /// 
    /// `inst_type` is optional: when present it is forwarded to the API as the
    /// `instType` query param, and passing `None` preserves the previous request
    /// shape.
    pub fn load(&self, ticker: String, inst_type: Option<String>) { todo!() }
    /// Tear down: cancel the load and detach observers (native dealloc / leave).
    pub fn on_clear(&self) { todo!() }
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
    /// DEPRECATED. Use `ui_event_stream()`; drop the returned stream
    /// wrapper to unsubscribe.
    pub fn remove_ui_event_observer(&self) { todo!() }
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
    /// DEPRECATED: callback-based observer. Use `ui_event_stream()` —
    /// the callback observer pins the foreign object in UniFFI's
    /// static handleMap and produces a cross-language retain cycle
    /// (Android GC + iOS ARC).
    pub fn set_ui_event_observer(&self, observer: Arc<dyn CompanyDividendUiEventObserver>) { todo!() }
    /// Start the interactor lifecycle.
    /// 
    /// Calls the user's `InteractorLifecycle::start()` implementation.
    /// Override that method to add custom startup behavior (e.g., initial data loads).
    pub fn start(&self) { todo!() }
    /// Get the current state (UniFFI export).
    /// 
    /// Returns the External type (UniFFI-compatible).
    pub fn state(&self) -> CompanyDividendState { todo!() }
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
    /// Opt-in async UI event stream (cycle-free). Single-subscriber.
    pub fn ui_event_stream(&self) -> Arc<CompanyDividendUiEventStream> { todo!() }
}

/// Deprecated UniFFI callback interface for observing UI events. Prefer the cycle-free `CompanyDividendUiEventStream` returned by `interactor.ui_event_stream()`.
pub struct CompanyDividendUiEventObserver;
impl CompanyDividendUiEventObserver {
    pub fn on_ui_event(&self, event: CompanyDividendUiEvent) { todo!() }
}

/// Opt-in async event stream from `interactor.ui_event_stream()`.
/// Single-subscriber, latest-wins. `next()` resolves to `None` when the
/// interactor is dropped or a new subscription replaces this one.
pub struct CompanyDividendUiEventStream;
impl CompanyDividendUiEventStream {
    pub async fn next(&self) -> Option<CompanyDividendUiEvent> { todo!() }
}

pub struct CompanyExperienceInteractor;
impl CompanyExperienceInteractor {
    /// Invalidate every section cache and outstanding response while preserving the subject.
    pub fn clear_cache(&self) { todo!() }
    /// Release the active subject and invalidate every section without detaching observers.
    pub fn clear_subject(&self) { todo!() }
    /// Pull the current generation's dividends and splits projection.
    pub fn dividends_presentation(&self) -> Option<CompanyDividendsPresentation> { todo!() }
    pub fn dividends_request_generation(&self) -> u64 { todo!() }
    pub fn etf_allocations_request_generation(&self) -> u64 { todo!() }
    /// Pull the current generation's ETF Constituent-Changes (M5) presentation.
    pub fn etf_constituent_changes_presentation(&self) -> Option<CompanyEtfConstituentChangesPresentation> { todo!() }
    pub fn etf_constituent_changes_request_generation(&self) -> u64 { todo!() }
    /// Pull the current generation's COMPLETE Rust-merged ETF Holdings (M2) list.
    pub fn etf_holdings_presentation(&self) -> Option<CompanyEtfHoldingsPagePresentation> { todo!() }
    pub fn etf_holdings_request_generation(&self) -> u64 { todo!() }
    pub fn etf_performance_request_generation(&self) -> u64 { todo!() }
    /// Pull the current generation's ETF Top-Holdings (M1) presentation.
    pub fn etf_top_holdings_presentation(&self) -> Option<CompanyEtfTopHoldingsPresentation> { todo!() }
    pub fn etf_top_request_generation(&self) -> u64 { todo!() }
    /// Pull the current generation's partial-success Financials projection.
    pub fn financials_presentation(&self) -> Option<CompanyFinancialsPresentation> { todo!() }
    pub fn financials_request_generation(&self) -> u64 { todo!() }
    /// Load the dividends and stock-splits projection.
    pub fn load_dividends(&self, force_refresh: bool) { todo!() }
    /// Load the ETF M3 Allocations (country + sector).
    pub fn load_etf_allocations(&self) { todo!() }
    /// Load the ETF M5 Constituent Changes, first page. `limit` is the page size
    /// (preview passes `Some(3)`; the detail page passes `Some(50)` / `None` for
    /// the backend default). Resets the Rust-owned merged list.
    pub fn load_etf_constituent_changes(&self, limit: Option<u32>) { todo!() }
    /// Load the ETF M2 Holdings list, page 1 (a first load, refresh, or the
    /// current sort re-applied). Resets the Rust-owned merged list.
    pub fn load_etf_holdings(&self) { todo!() }
    /// Load the ETF M4 Performance (curve + stage/annual returns). `period`
    /// selects the curve window (`1M`/`3M`/`6M`/`1Y`/`3Y`; `None` = backend
    /// default `1Y`). Benchmarks are not requested per-type — the response
    /// carries all of them and the client selects among them.
    pub fn load_etf_performance(&self, period: Option<String>) { todo!() }
    /// Load the ETF M1 Top-Holdings card.
    pub fn load_etf_top_holdings(&self) { todo!() }
    /// Load the partial-success earnings and quarterly statement projections.
    pub fn load_financials(&self, force_refresh: bool) { todo!() }
    /// Fetch the next M5 Constituent-Changes page and append it inside Rust.
    /// No-op when there are no more pages or a page load is already in flight.
    pub fn load_more_etf_constituent_changes(&self) { todo!() }
    /// Fetch the next M2 Holdings page and append it inside Rust. No-op when
    /// there are no more pages or a page load is already in flight.
    pub fn load_more_etf_holdings(&self) { todo!() }
    /// Load the profile and optional quote. `force_refresh` bypasses the successful cache.
    pub fn load_profile(&self, force_refresh: bool) { todo!() }
    /// Load the shared shareholder stats projection. `force_refresh` bypasses the cache.
    pub fn load_shareholders(&self, force_refresh: bool) { todo!() }
    /// Load or re-project one statement section and selected detail range.
    pub fn load_statement(&self, kind: StatementKind, timeframe: String, range_kind: String, range_value: Option<i32>, force_refresh: bool) { todo!() }
    /// Release all subject data, cancel effects, and detach observers.
    pub fn on_clear(&self) { todo!() }
    /// Prepare or change the active subject. Preparing an identical subject is idempotent.
    pub fn prepare(&self, subject: CompanySubject) { todo!() }
    /// Prepare a DEX/CeDeFi subject without applying CEX instrument assumptions.
    pub fn prepare_external(&self, ticker: String, icon_url: String) { todo!() }
    /// Rust-resolved ticker for the active subject. Native must not derive it from `inst_id`.
    pub fn prepared_ticker(&self) -> Option<String> { todo!() }
    /// Pull the current generation's profile and quote-derived metrics.
    pub fn profile_presentation(&self) -> Option<CompanyProfileExperiencePresentation> { todo!() }
    /// Current profile request epoch. Consumers use it to reject superseded synchronous events.
    pub fn profile_request_generation(&self) -> u64 { todo!() }
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
    /// DEPRECATED. Use `ui_event_stream()`; drop the returned stream
    /// wrapper to unsubscribe.
    pub fn remove_ui_event_observer(&self) { todo!() }
    /// Enable or disable debug logging.
    /// 
    /// In release builds this is a no-op, but the symbol must exist
    /// so that UniFFI bindings generated in debug mode continue to
    /// link successfully against the release library.
    pub fn set_debug(&self, _enabled: bool) { todo!() }
    /// Re-load the M2 Holdings list under a new sort. Resets pagination.
    pub fn set_etf_holdings_sort(&self, sort_by: Option<String>, order: Option<String>) { todo!() }
    /// Set a state observer (UniFFI export).
    /// 
    /// DEPRECATED: callback-based observer. Use `state_stream()` —
    /// the callback observer pins the foreign object in UniFFI's
    /// static handleMap and produces a cross-language retain cycle
    /// (Android GC + iOS ARC).
    pub fn set_state_observer(&self, observer: Arc<dyn InteractorStateChangeObserver>) { todo!() }
    /// DEPRECATED: callback-based observer. Use `ui_event_stream()` —
    /// the callback observer pins the foreign object in UniFFI's
    /// static handleMap and produces a cross-language retain cycle
    /// (Android GC + iOS ARC).
    pub fn set_ui_event_observer(&self, observer: Arc<dyn CompanyExperienceUiEventObserver>) { todo!() }
    /// Pull the current generation's shareholder and institution projections.
    pub fn shareholders_presentation(&self) -> Option<CompanyShareholdersPresentation> { todo!() }
    /// Current shareholder request epoch. Consumers use it to reject superseded events.
    pub fn shareholders_request_generation(&self) -> u64 { todo!() }
    /// Start the interactor lifecycle.
    /// 
    /// Calls the user's `InteractorLifecycle::start()` implementation.
    /// Override that method to add custom startup behavior (e.g., initial data loads).
    pub fn start(&self) { todo!() }
    /// Get the current state (UniFFI export).
    /// 
    /// Returns the External type (UniFFI-compatible).
    pub fn state(&self) -> CompanyExperienceState { todo!() }
    /// Opt-in async state-change stream (cycle-free, signal-only).
    /// Single-subscriber, latest-wins. `changed()` resolves `true` on
    /// change (pull `state()` for the snapshot), `false` when the
    /// interactor is released or a newer subscription replaces this one.
    pub fn state_stream(&self) -> Arc<InteractorStateStream> { todo!() }
    /// Re-project a cached statement without making another endpoint request.
    pub fn statement_presentation(&self, kind: StatementKind, timeframe: String, range_kind: String, range_value: Option<i32>) -> Option<CompanyStatementPresentation> { todo!() }
    pub fn statement_request_generation(&self) -> u64 { todo!() }
    /// Stop the interactor and cancel all effects.
    /// 
    /// Calls the user's `InteractorLifecycle::stop()` implementation first,
    /// then cancels all effects as a framework guarantee.
    pub fn stop(&self) { todo!() }
    /// The currently prepared subject, if it was resolved and has a ticker.
    pub fn subject(&self) -> Option<CompanySubject> { todo!() }
    /// Opt-in async UI event stream (cycle-free). Single-subscriber.
    pub fn ui_event_stream(&self) -> Arc<CompanyExperienceUiEventStream> { todo!() }
}

/// Deprecated UniFFI callback interface for observing UI events. Prefer the cycle-free `CompanyExperienceUiEventStream` returned by `interactor.ui_event_stream()`.
pub struct CompanyExperienceUiEventObserver;
impl CompanyExperienceUiEventObserver {
    pub fn on_ui_event(&self, event: CompanyExperienceUiEvent) { todo!() }
}

/// Opt-in async event stream from `interactor.ui_event_stream()`.
/// Single-subscriber, latest-wins. `next()` resolves to `None` when the
/// interactor is dropped or a new subscription replaces this one.
pub struct CompanyExperienceUiEventStream;
impl CompanyExperienceUiEventStream {
    pub async fn next(&self) -> Option<CompanyExperienceUiEvent> { todo!() }
}

pub struct CompanyFinancialsInteractor;
impl CompanyFinancialsInteractor {
    /// Start the initial parallel load of all four sections for `ticker`.
    /// 
    /// `inst_type` is optional: when present it is forwarded to the API as the
    /// `instType` query param (on every section request), and passing `None`
    /// preserves the previous request shape.
    pub fn load(&self, ticker: String, inst_type: Option<String>) { todo!() }
    /// Tear down: cancel effects and detach observers (native dealloc / leave).
    pub fn on_clear(&self) { todo!() }
    /// Stop-gap teardown (TEMPORARY — target state is the stream APIs,
    /// which need no teardown call).
    /// 
    /// One idempotent call at host lifecycle end that breaks the
    /// UniFFI handleMap cycle for any registered callback observers,
    /// ends active streams, and stops effects. On Kotlin call BEFORE
    /// `destroy()`. Includes `stop()`.
    pub fn release(&self) { todo!() }
    /// Reload one statement section at `timeframe`. Supersedes any in-flight
    /// reload.
    /// 
    /// `inst_type` is optional: when present it is forwarded to the API as the
    /// `instType` query param, and passing `None` preserves the previous request
    /// shape.
    pub fn reload(&self, ticker: String, kind: StatementKind, timeframe: String, inst_type: Option<String>) { todo!() }
    /// Remove the state observer (UniFFI export).
    /// 
    /// DEPRECATED: callback-based observation. Use `state_stream()` —
    /// it needs no removal call. Until migrated, every
    /// `set_state_observer` MUST be paired with this (or `release()`)
    /// at host lifecycle end, or the UniFFI handleMap pins the
    /// observer and its host forever.
    pub fn remove_state_observer(&self) { todo!() }
    /// DEPRECATED. Use `ui_event_stream()`; drop the returned stream
    /// wrapper to unsubscribe.
    pub fn remove_ui_event_observer(&self) { todo!() }
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
    /// DEPRECATED: callback-based observer. Use `ui_event_stream()` —
    /// the callback observer pins the foreign object in UniFFI's
    /// static handleMap and produces a cross-language retain cycle
    /// (Android GC + iOS ARC).
    pub fn set_ui_event_observer(&self, observer: Arc<dyn CompanyFinancialsUiEventObserver>) { todo!() }
    /// Start the interactor lifecycle.
    /// 
    /// Calls the user's `InteractorLifecycle::start()` implementation.
    /// Override that method to add custom startup behavior (e.g., initial data loads).
    pub fn start(&self) { todo!() }
    /// Get the current state (UniFFI export).
    /// 
    /// Returns the External type (UniFFI-compatible).
    pub fn state(&self) -> CompanyFinancialsState { todo!() }
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
    /// Opt-in async UI event stream (cycle-free). Single-subscriber.
    pub fn ui_event_stream(&self) -> Arc<CompanyFinancialsUiEventStream> { todo!() }
}

/// Deprecated UniFFI callback interface for observing UI events. Prefer the cycle-free `CompanyFinancialsUiEventStream` returned by `interactor.ui_event_stream()`.
pub struct CompanyFinancialsUiEventObserver;
impl CompanyFinancialsUiEventObserver {
    pub fn on_ui_event(&self, event: CompanyFinancialsUiEvent) { todo!() }
}

/// Opt-in async event stream from `interactor.ui_event_stream()`.
/// Single-subscriber, latest-wins. `next()` resolves to `None` when the
/// interactor is dropped or a new subscription replaces this one.
pub struct CompanyFinancialsUiEventStream;
impl CompanyFinancialsUiEventStream {
    pub async fn next(&self) -> Option<CompanyFinancialsUiEvent> { todo!() }
}

pub struct CompanyProfileInteractor;
impl CompanyProfileInteractor {
    /// Start a concurrent profile + quote load for `ticker`. The in-flight load
    /// (if any) is cancelled first, so calling repeatedly on instrument switch is
    /// safe — only the latest result is emitted.
    /// 
    /// `inst_type` is optional: when present it is forwarded to the API as the
    /// `instType` query param, and passing `None` preserves the previous request
    /// shape.
    pub fn load(&self, ticker: String, inst_type: Option<String>) { todo!() }
    /// Tear down: cancel the load and detach observers (native dealloc / leave).
    pub fn on_clear(&self) { todo!() }
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
    /// DEPRECATED. Use `ui_event_stream()`; drop the returned stream
    /// wrapper to unsubscribe.
    pub fn remove_ui_event_observer(&self) { todo!() }
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
    /// DEPRECATED: callback-based observer. Use `ui_event_stream()` —
    /// the callback observer pins the foreign object in UniFFI's
    /// static handleMap and produces a cross-language retain cycle
    /// (Android GC + iOS ARC).
    pub fn set_ui_event_observer(&self, observer: Arc<dyn CompanyProfileUiEventObserver>) { todo!() }
    /// Start the interactor lifecycle.
    /// 
    /// Calls the user's `InteractorLifecycle::start()` implementation.
    /// Override that method to add custom startup behavior (e.g., initial data loads).
    pub fn start(&self) { todo!() }
    /// Get the current state (UniFFI export).
    /// 
    /// Returns the External type (UniFFI-compatible).
    pub fn state(&self) -> CompanyProfileState { todo!() }
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
    /// Opt-in async UI event stream (cycle-free). Single-subscriber.
    pub fn ui_event_stream(&self) -> Arc<CompanyProfileUiEventStream> { todo!() }
}

/// Deprecated UniFFI callback interface for observing UI events. Prefer the cycle-free `CompanyProfileUiEventStream` returned by `interactor.ui_event_stream()`.
pub struct CompanyProfileUiEventObserver;
impl CompanyProfileUiEventObserver {
    pub fn on_ui_event(&self, event: CompanyProfileUiEvent) { todo!() }
}

/// Opt-in async event stream from `interactor.ui_event_stream()`.
/// Single-subscriber, latest-wins. `next()` resolves to `None` when the
/// interactor is dropped or a new subscription replaces this one.
pub struct CompanyProfileUiEventStream;
impl CompanyProfileUiEventStream {
    pub async fn next(&self) -> Option<CompanyProfileUiEvent> { todo!() }
}

pub struct CompanyShareholdersInteractor;
impl CompanyShareholdersInteractor {
    /// Start a shareholders-stats load for `ticker`. The in-flight load (if any)
    /// is cancelled first, so calling repeatedly on instrument switch is safe —
    /// only the latest result is emitted.
    /// 
    /// `inst_type` is optional: when present it is forwarded to the API as the
    /// `instType` query param, and passing `None` preserves the previous request
    /// shape.
    pub fn load(&self, ticker: String, inst_type: Option<String>) { todo!() }
    /// Tear down: cancel the load and detach observers (native dealloc / leave).
    pub fn on_clear(&self) { todo!() }
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
    /// DEPRECATED. Use `ui_event_stream()`; drop the returned stream
    /// wrapper to unsubscribe.
    pub fn remove_ui_event_observer(&self) { todo!() }
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
    /// DEPRECATED: callback-based observer. Use `ui_event_stream()` —
    /// the callback observer pins the foreign object in UniFFI's
    /// static handleMap and produces a cross-language retain cycle
    /// (Android GC + iOS ARC).
    pub fn set_ui_event_observer(&self, observer: Arc<dyn CompanyShareholdersUiEventObserver>) { todo!() }
    /// Start the interactor lifecycle.
    /// 
    /// Calls the user's `InteractorLifecycle::start()` implementation.
    /// Override that method to add custom startup behavior (e.g., initial data loads).
    pub fn start(&self) { todo!() }
    /// Get the current state (UniFFI export).
    /// 
    /// Returns the External type (UniFFI-compatible).
    pub fn state(&self) -> CompanyShareholdersState { todo!() }
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
    /// Opt-in async UI event stream (cycle-free). Single-subscriber.
    pub fn ui_event_stream(&self) -> Arc<CompanyShareholdersUiEventStream> { todo!() }
}

/// Deprecated UniFFI callback interface for observing UI events. Prefer the cycle-free `CompanyShareholdersUiEventStream` returned by `interactor.ui_event_stream()`.
pub struct CompanyShareholdersUiEventObserver;
impl CompanyShareholdersUiEventObserver {
    pub fn on_ui_event(&self, event: CompanyShareholdersUiEvent) { todo!() }
}

/// Opt-in async event stream from `interactor.ui_event_stream()`.
/// Single-subscriber, latest-wins. `next()` resolves to `None` when the
/// interactor is dropped or a new subscription replaces this one.
pub struct CompanyShareholdersUiEventStream;
impl CompanyShareholdersUiEventStream {
    pub async fn next(&self) -> Option<CompanyShareholdersUiEvent> { todo!() }
}

/// Callback for compare candles stream updates
pub struct CompareCandlesStreamCallback;
impl CompareCandlesStreamCallback {
    /// Called when the subscription is completed
    pub fn on_completed(&self) { todo!() }
    /// Called when an error occurs during subscription
    pub fn on_error(&self, error: OKFFIError) { todo!() }
    /// Called when compare candles data is updated (from WebSocket)
    /// 
    /// # Arguments
    /// * `update` - Batch update containing K-line data for multiple instruments
    pub fn on_update(&self, update: CompareCandlesStreamUpdate) { todo!() }
}

/// Deprecated UniFFI callback interface for observing UI events. Prefer the cycle-free `CompareIndicatorUiEventStream` returned by `interactor.ui_event_stream()`.
pub struct CompareIndicatorUiEventObserver;
impl CompareIndicatorUiEventObserver {
    pub fn on_ui_event(&self, event: CompareIndicatorUiEvent) { todo!() }
}

/// Opt-in async event stream from `interactor.ui_event_stream()`.
/// Single-subscriber, latest-wins. `next()` resolves to `None` when the
/// interactor is dropped or a new subscription replaces this one.
pub struct CompareIndicatorUiEventStream;
impl CompareIndicatorUiEventStream {
    pub async fn next(&self) -> Option<CompareIndicatorUiEvent> { todo!() }
}

/// Callback for depth stream updates
pub struct DepthStreamCallback;
impl DepthStreamCallback {
    /// Called when the subscription is completed
    pub fn on_update_completed(&self) { todo!() }
    /// Called when depth data is updated (from WebSocket)
    /// 
    /// # Arguments
    /// * `depth` - Order book data (snapshot or incremental update)
    pub fn on_update_depth(&self, depth: BizDepth) { todo!() }
    /// Called when an error occurs during subscription
    pub fn on_update_error(&self, error: OKFFIError) { todo!() }
}

/// Drawing line interactor
pub struct DrawingLineInteractor;
impl DrawingLineInteractor {
    /// Delete drawing lines by IDs for an explicit instrument.
    pub fn delete_lines(&self, line_ids: Vec<String>, inst_id: String) { todo!() }
    /// Reload drawing lines from server/local
    pub fn fetch_lines(&self) { todo!() }
    /// Initialize interactor with an instrument ID (called when chart loads)
    pub fn initialize(&self, inst_id: String) { todo!() }
    /// Called when chart is destroyed
    pub fn on_clear(&self) { todo!() }
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
    /// DEPRECATED. Use `ui_event_stream()`; drop the returned stream
    /// wrapper to unsubscribe.
    pub fn remove_ui_event_observer(&self) { todo!() }
    /// Save a drawing line (create or update)
    pub fn save_line(&self, line: OKKLineDrawingData) { todo!() }
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
    /// DEPRECATED: callback-based observer. Use `ui_event_stream()` —
    /// the callback observer pins the foreign object in UniFFI's
    /// static handleMap and produces a cross-language retain cycle
    /// (Android GC + iOS ARC).
    pub fn set_ui_event_observer(&self, observer: Arc<dyn DrawingLineUiEventObserver>) { todo!() }
    /// Start the interactor lifecycle.
    /// 
    /// Calls the user's `InteractorLifecycle::start()` implementation.
    /// Override that method to add custom startup behavior (e.g., initial data loads).
    pub fn start(&self) { todo!() }
    /// Get the current state (UniFFI export).
    /// 
    /// Returns the External type (UniFFI-compatible).
    pub fn state(&self) -> DrawingLineState { todo!() }
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
    /// Opt-in async UI event stream (cycle-free). Single-subscriber.
    pub fn ui_event_stream(&self) -> Arc<DrawingLineUiEventStream> { todo!() }
    /// Upload all unsynced lines to server
    pub fn upload_lines(&self) { todo!() }
}

/// Deprecated UniFFI callback interface for observing UI events. Prefer the cycle-free `DrawingLineUiEventStream` returned by `interactor.ui_event_stream()`.
pub struct DrawingLineUiEventObserver;
impl DrawingLineUiEventObserver {
    pub fn on_ui_event(&self, event: DrawingLineUiEvent) { todo!() }
}

/// Opt-in async event stream from `interactor.ui_event_stream()`.
/// Single-subscriber, latest-wins. `next()` resolves to `None` when the
/// interactor is dropped or a new subscription replaces this one.
pub struct DrawingLineUiEventStream;
impl DrawingLineUiEventStream {
    pub async fn next(&self) -> Option<DrawingLineUiEvent> { todo!() }
}

pub struct EventIndexPriceInteractor;
impl EventIndexPriceInteractor {
    pub fn cancel_ws(&self) { todo!() }
    pub fn load_data(&self, config: EventChartConfigVo) { todo!() }
    pub fn on_clear(&self) { todo!() }
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
    pub fn state(&self) -> EventIndexPriceState { todo!() }
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
}

pub struct EventProbabilityInteractor;
impl EventProbabilityInteractor {
    /// Cancel the running HTTP/WS effect. Preserves state.
    pub fn cancel_ws(&self) { todo!() }
    /// Reload with a new config: cancels any running HTTP/WS, fetches HTTP,
    /// then subscribes to the candle WS when `!is_settled`.
    pub fn load_data(&self, config: EventChartConfigVo) { todo!() }
    /// Called when the owning ViewModel is cleared.
    pub fn on_clear(&self) { todo!() }
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
    pub fn state(&self) -> EventProbabilityState { todo!() }
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
}

/// Simplified WebSocket subscription for formula candles
/// 
/// Provides a simplified interface for subscribing to real-time formula candles
/// via WebSocket using the combine-candles endpoint.
pub struct FormulaCandlesSimplified;
impl FormulaCandlesSimplified {
    /// Create a new FormulaCandlesSimplified instance with individual parameters
    /// 
    /// This constructor builds the WebSocket URL internally, eliminating the need
    /// for callers to construct the URL themselves.
    /// 
    /// # Parameters
    /// * `calc_expression` - Calculation expression string (e.g., "V0 + V1 - V2")
    /// * `inst_id_array` - Instrument ID array as JSON string
    /// (e.g., `[["BTC-USDT","SPOT"],["ETH-USDT","SPOT"]]`)
    /// * `bar` - Bar/timeframe (e.g., "1m", "5m", "15m", "1H", "1D")
    /// * `is_demo_trading` - Whether the user is in demo trading mode
    pub fn new(calc_expression: String, inst_id_array: String, bar: String, is_demo_trading: bool) -> Arc<FormulaCandlesSimplified> { todo!() }
    /// Get the WebSocket path (for debugging)
    /// 
    /// # Returns
    /// The WebSocket path including query parameters
    pub fn get_ws_path(&self) -> String { todo!() }
    /// Check if the subscription is currently running
    /// 
    /// # Returns
    /// `true` if the subscription is active, `false` otherwise
    pub fn is_running(&self) -> bool { todo!() }
    /// Set the data callback
    /// 
    /// # Parameters
    /// * `callback` - Callback implementation for receiving candle data
    pub fn set_callback(&self, callback: Arc<dyn FormulaCandlesSimplifiedCallback>) { todo!() }
    /// Start the WebSocket subscription
    /// 
    /// # Returns
    /// A future that resolves when the subscription is started
    pub fn start(&self) -> Arc<OKRustFutureStartResultWithOKFFIError> { todo!() }
    /// Stop the WebSocket subscription
    pub fn stop(&self) { todo!() }
}

/// Callback trait for formula candles WebSocket data
/// 
/// Implement this trait to receive real-time formula candles data updates.
pub struct FormulaCandlesSimplifiedCallback;
impl FormulaCandlesSimplifiedCallback {
    /// Called when an error occurs
    /// 
    /// # Parameters
    /// * `error_message` - Error description
    pub fn on_error(&self, error_message: String) { todo!() }
    /// Called when WebSocket message is received with candle data
    /// 
    /// # Parameters
    /// * `candles` - List of candle data items received from WebSocket
    pub fn on_ws_message(&self, candles: Vec<BizKLineData>) { todo!() }
}

/// Deprecated UniFFI callback interface for observing UI events. Prefer the cycle-free `FormulaIndicatorUiEventStream` returned by `interactor.ui_event_stream()`.
pub struct FormulaIndicatorUiEventObserver;
impl FormulaIndicatorUiEventObserver {
    pub fn on_ui_event(&self, event: FormulaIndicatorUiEvent) { todo!() }
}

/// Opt-in async event stream from `interactor.ui_event_stream()`.
/// Single-subscriber, latest-wins. `next()` resolves to `None` when the
/// interactor is dropped or a new subscription replaces this one.
pub struct FormulaIndicatorUiEventStream;
impl FormulaIndicatorUiEventStream {
    pub async fn next(&self) -> Option<FormulaIndicatorUiEvent> { todo!() }
}

/// Deprecated UniFFI callback interface for observing UI events. Prefer the cycle-free `HalfScreenIndicatorUiEventStream` returned by `interactor.ui_event_stream()`.
pub struct HalfScreenIndicatorUiEventObserver;
impl HalfScreenIndicatorUiEventObserver {
    pub fn on_ui_event(&self, event: HalfScreenIndicatorUiEvent) { todo!() }
}

/// Opt-in async event stream from `interactor.ui_event_stream()`.
/// Single-subscriber, latest-wins. `next()` resolves to `None` when the
/// interactor is dropped or a new subscription replaces this one.
pub struct HalfScreenIndicatorUiEventStream;
impl HalfScreenIndicatorUiEventStream {
    pub async fn next(&self) -> Option<HalfScreenIndicatorUiEvent> { todo!() }
}

pub struct IndicatorSyncInteractor;
impl IndicatorSyncInteractor {
    pub fn on_clear(&self) { todo!() }
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
    /// DEPRECATED. Use `ui_event_stream()`; drop the returned stream
    /// wrapper to unsubscribe.
    pub fn remove_ui_event_observer(&self) { todo!() }
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
    /// DEPRECATED: callback-based observer. Use `ui_event_stream()` —
    /// the callback observer pins the foreign object in UniFFI's
    /// static handleMap and produces a cross-language retain cycle
    /// (Android GC + iOS ARC).
    pub fn set_ui_event_observer(&self, observer: Arc<dyn IndicatorSyncUiEventObserver>) { todo!() }
    /// Start the interactor lifecycle.
    /// 
    /// Calls the user's `InteractorLifecycle::start()` implementation.
    /// Override that method to add custom startup behavior (e.g., initial data loads).
    pub fn start(&self) { todo!() }
    /// Get the current state (UniFFI export).
    /// 
    /// Returns the External type (UniFFI-compatible).
    pub fn state(&self) -> IndicatorSyncState { todo!() }
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
    /// Reconcile platform storage, retry the durable outbox, then fetch the cloud delta.
    pub fn sync_download(&self) { todo!() }
    pub fn try_upload_diff(&self, old: Vec<String>, new: Vec<String>, is_main: bool) { todo!() }
    pub fn try_upload_single_indicator(&self, indicator_name: IndicatorName, indicator_type: IndicatorType, is_selected: bool) { todo!() }
    /// Compatibility triggers. The payload is intentionally not uploaded directly: the shared
    /// Repository is the source of truth and already recorded the exact changed server keys.
    pub fn try_upload_toggle(&self, indicator_name: String, is_main: bool, enabled: bool) { todo!() }
    /// Opt-in async UI event stream (cycle-free). Single-subscriber.
    pub fn ui_event_stream(&self) -> Arc<IndicatorSyncUiEventStream> { todo!() }
}

/// Deprecated UniFFI callback interface for observing UI events. Prefer the cycle-free `IndicatorSyncUiEventStream` returned by `interactor.ui_event_stream()`.
pub struct IndicatorSyncUiEventObserver;
impl IndicatorSyncUiEventObserver {
    pub fn on_ui_event(&self, event: IndicatorSyncUiEvent) { todo!() }
}

/// Opt-in async event stream from `interactor.ui_event_stream()`.
/// Single-subscriber, latest-wins. `next()` resolves to `None` when the
/// interactor is dropped or a new subscription replaces this one.
pub struct IndicatorSyncUiEventStream;
impl IndicatorSyncUiEventStream {
    pub async fn next(&self) -> Option<IndicatorSyncUiEvent> { todo!() }
}

pub struct KLineCompareInteractor;
impl KLineCompareInteractor {
    pub fn new_default() -> Arc<KLineCompareInteractor> { todo!() }
    pub fn check_switch_period(&self, period: String) -> bool { todo!() }
    pub async fn fetch_indicator_line_ranges(&self, subscription_id: u32, expected_candle_revision: u64, expected_params_version: u64, ranges: Vec<KlineIndicatorLineRangeVo>) -> Result<KlineIndicatorDataFrameVo, OKFFIError> { todo!() }
    pub async fn fetch_indicator_viewport(&self, subscription_id: u32, expected_candle_revision: u64, expected_params_version: u64, start_index: u64, end_index: u64) -> Result<KlineIndicatorDataFrameVo, OKFFIError> { todo!() }
    pub async fn fetch_indicator_window(&self, subscription_id: u32, expected_candle_revision: u64, expected_params_version: u64, start_index: u64, end_index: u64) -> Result<KlineIndicatorDataFrameVo, OKFFIError> { todo!() }
    /// Pull all candle data as a flat list of (instId, candles) pairs.
    /// UniFFI cannot export HashMap directly, so we use a Vec of records.
    pub fn get_all_candle_inst_ids(&self) -> Vec<String> { todo!() }
    /// Pull candle data for a specific instrument.
    pub fn get_candles_for_inst(&self, inst_id: String) -> Vec<BizKLineData> { todo!() }
    pub fn get_funding_rate(&self) -> Option<KLineFundingRate> { todo!() }
    /// Returns the exact encoded page for one observed structural state.
    /// Identity is checked before cloning the single byte buffer so a stale or
    /// coalesced callback falls back to a full snapshot instead of applying a
    /// page to the wrong Native candle cache.
    pub fn get_history_delta_frame(&self, subscription_id: u32, candle_version: u64, candle_revision: u64) -> Option<CompareHistoryDeltaFrame> { todo!() }
    pub fn get_ticker(&self) -> Option<KLineTickerSnapshot> { todo!() }
    /// Initialize with primary instrument. Triggers ensureInit → loadBiz → context start.
    pub fn initialize(&self, inst_id: String, biz_type: String) { todo!() }
    pub fn is_auction_staging(&self) -> bool { todo!() }
    /// Load more history candles for all instruments.
    /// `inst_ids` should match the current subscription set.
    pub fn load_more(&self, inst_ids: Vec<String>) { todo!() }
    /// Full stop and cleanup.
    pub fn on_clear(&self) { todo!() }
    /// Stop-gap teardown (TEMPORARY — target state is the stream APIs,
    /// which need no teardown call).
    /// 
    /// One idempotent call at host lifecycle end that breaks the
    /// UniFFI handleMap cycle for any registered callback observers,
    /// ends active streams, and stops effects. On Kotlin call BEFORE
    /// `destroy()`. Includes `stop()`.
    pub fn release(&self) { todo!() }
    /// Remove a compare instrument's candle data from cache.
    /// Does NOT cancel WS — the spawned task for this instId will be
    /// cleaned up when the parent CandleSubscription effect is cancelled.
    pub fn remove_inst(&self, inst_id: String) { todo!() }
    /// Remove the state observer (UniFFI export).
    /// 
    /// DEPRECATED: callback-based observation. Use `state_stream()` —
    /// it needs no removal call. Until migrated, every
    /// `set_state_observer` MUST be paired with this (or `release()`)
    /// at host lifecycle end, or the UniFFI handleMap pins the
    /// observer and its host forever.
    pub fn remove_state_observer(&self) { todo!() }
    /// DEPRECATED. Use `ui_event_stream()`; drop the returned stream
    /// wrapper to unsubscribe.
    pub fn remove_ui_event_observer(&self) { todo!() }
    /// Self-resolved variant of [`set_adjustments`] (Android V2): judges every
    /// leg internally (TradFi coin-table gate + persisted forward choice, same
    /// leg judgment as the candles request layer) and stores only the stock
    /// legs, so native carries no `isTradFi` knowledge. `stock_adjust_ab` is
    /// the native gray switch; `false` clears all adjustments. Pure setter like
    /// [`set_adjustments`] — call before `start_candle` with the same leg set.
    pub fn resolve_stock_adjustments(&self, inst_ids: Vec<String>, stock_adjust_ab: bool) { todo!() }
    /// Set per-inst corporate-action effectiveTimes (from the batch adjustment
    /// HTTP). Pure setter — applies live to subsequent WS merges, no restart.
    pub fn set_adjustment_effective_times(&self, times: Vec<InstEffectiveTimes>) { todo!() }
    /// Set per-inst stock-adjustment parameters (from the Swift side-car's
    /// any-stock computation; only stock insts that need adjustment appear).
    /// Pure setter — does NOT restart. The Swift side drives the restart
    /// (calls `start_candle` after this so the first-frame HTTP carries `adjust`).
    pub fn set_adjustments(&self, adjustments: Vec<InstAdjustment>) { todo!() }
    /// Deprecated compatibility hook. CandleStore is now always enabled because
    /// the primary store is shared with IndicatorRuntime.
    pub fn set_candle_cache_full(&self, enabled: bool) { todo!() }
    /// Enable or disable debug logging.
    /// 
    /// In release builds this is a no-op, but the symbol must exist
    /// so that UniFFI bindings generated in debug mode continue to
    /// link successfully against the release library.
    pub fn set_debug(&self, _enabled: bool) { todo!() }
    pub fn set_indicator_render_context(&self, surface_active: bool, main_enabled: bool, sub_enabled: bool) { todo!() }
    pub fn set_indicator_trading_unit(&self, unit: String) { todo!() }
    /// Set a state observer (UniFFI export).
    /// 
    /// DEPRECATED: callback-based observer. Use `state_stream()` —
    /// the callback observer pins the foreign object in UniFFI's
    /// static handleMap and produces a cross-language retain cycle
    /// (Android GC + iOS ARC).
    pub fn set_state_observer(&self, observer: Arc<dyn InteractorStateChangeObserver>) { todo!() }
    /// DEPRECATED: callback-based observer. Use `ui_event_stream()` —
    /// the callback observer pins the foreign object in UniFFI's
    /// static handleMap and produces a cross-language retain cycle
    /// (Android GC + iOS ARC).
    pub fn set_ui_event_observer(&self, observer: Arc<dyn CompareIndicatorUiEventObserver>) { todo!() }
    /// Start the interactor lifecycle.
    /// 
    /// Calls the user's `InteractorLifecycle::start()` implementation.
    /// Override that method to add custom startup behavior (e.g., initial data loads).
    pub fn start(&self) { todo!() }
    /// Start candle subscription for all instruments at a given period.
    /// `period` must be API bar format.
    /// `inst_ids` is a list of ALL instrument IDs (primary + compare targets).
    /// `preserve_cache`: foreground / same-period resume keeps the existing
    /// per-leg CandleStores (fresh HTTP frames replace each leg atomically as data
    /// arrives) so the comparison series never go partial mid-reload — mirrors
    /// ObjC switchPeriod reusing `barDataById`. A real period switch passes
    /// `false` to clear and reload from scratch.
    pub fn start_candle(&self, period: String, inst_ids: Vec<String>, preserve_cache: bool) { todo!() }
    /// Restart context subscription.
    pub fn start_context(&self) { todo!() }
    /// Get the current state (UniFFI export).
    /// 
    /// Returns the External type (UniFFI-compatible).
    pub fn state(&self) -> KLineCompareState { todo!() }
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
    /// Stop candle subscription.
    pub fn stop_candle(&self) { todo!() }
    /// Stop context subscription (ticker WS).
    pub fn stop_context(&self) { todo!() }
    /// Opt-in async UI event stream (cycle-free). Single-subscriber.
    pub fn ui_event_stream(&self) -> Arc<CompareIndicatorUiEventStream> { todo!() }
}

/// Main context bridge managing an `InstContext` lifecycle and real-time market data delivery.
/// 
/// OC: Replaces `OKTInstrumentContext`.
/// 
/// # Threading
/// 
/// - The bridge is `Send + Sync` (all state behind `Mutex`).
/// - `start()` spawns work on `GlobalRuntime`. Callbacks fire on Tokio workers.
/// - Data accessors use `try_lock` — safe to call from callbacks (returns default on contention).
pub struct KLineContextBridge;
impl KLineContextBridge {
    /// Create a new idle `KLineContextBridge`.
    /// OC: `[[OKTInstrumentContext alloc] init]`
    pub fn new() -> Arc<KLineContextBridge> { todo!() }
    /// Get the latest coin-to-USD price (from CupTickers3s).
    /// OC: `context.coinUsdPrice`
    pub fn get_coin_usd_price(&self) -> Option<String> { todo!() }
    /// Delivery/funding countdown timer for derivatives.
    /// OC: `context.deliveryTimer`
    pub fn get_delivery_timer(&self) -> Option<KLineDeliveryTimer> { todo!() }
    /// OC: `context.estimatedPrice`
    pub fn get_estimated_price(&self) -> Option<String> { todo!() }
    /// Current funding rate for perpetual swaps. Reads from live `InstContext`.
    /// OC: `context.fundingRate`
    pub fn get_funding_rate(&self) -> Option<KLineFundingRate> { todo!() }
    /// Get the latest cached option summary (Greeks).
    /// OC: `context.optionSummary`
    pub fn get_option_summary(&self) -> Option<KLineOptionSummary> { todo!() }
    /// Get quote coin last price. Uses try_lock for callback safety.
    /// OC: `context.quoteCoinLast(forCcy:)`
    pub fn get_quote_coin_last(&self, quote_ccy: String) -> Option<String> { todo!() }
    /// Quote currency display precision (e.g. `"0.01"` → `2`).
    /// Returns [`DEFAULT_PRECISION`] before `start()` or on lock contention.
    pub fn get_quote_precision(&self) -> u32 { todo!() }
    /// Fiat currency symbol (e.g. `"$"`, `"¥"`).
    pub fn get_quote_symbol(&self) -> String { todo!() }
    /// Trade target unit (e.g. `"BTC"`, `"Cont"`).
    pub fn get_target_unit(&self) -> String { todo!() }
    /// Get the latest cached ticker snapshot. Safe to call from callbacks (uses try_lock).
    /// OC: `context.tickerData`
    pub fn get_ticker(&self) -> Option<KLineTickerSnapshot> { todo!() }
    /// Trade quantity display precision (e.g. `"0.0001"` → `4`).
    /// Returns [`DEFAULT_PRECISION`] before `start()` or on lock contention.
    pub fn get_trade_precision(&self) -> u32 { todo!() }
    /// Whether the instrument is in call-auction staging. `false` if no context is active.
    /// OC: `context.isCallAuctionStaging`
    pub fn is_auction_staging(&self) -> bool { todo!() }
    /// Whether the instrument is in pre-pending order staging.
    /// OC: `context.isPrePendingOrderStaging`
    pub fn is_pre_pending_order_staging(&self) -> bool { todo!() }
    /// OC: `context.isStarted`
    pub fn is_started(&self) -> bool { todo!() }
    /// Remove the current callback. No further notifications until a new one is set.
    /// OC: `context.delegate = nil`
    pub fn remove_callback(&self) { todo!() }
    /// Replace the current callback with a new one.
    /// OC: `context.delegate = newDelegate`
    /// 
    /// **Retain cycle note:** see [`start`](Self::start) doc — the callback must not
    /// hold a strong reference to this bridge instance.
    pub fn set_callback(&self, callback: Arc<dyn KLineContextCallback>) { todo!() }
    /// Start the context for a given instrument.
    /// 
    /// Asynchronously: ensures trade core init → loads biz → resolves instrument →
    /// creates `InstContext` → `start_all()` → emits initial ticker → enters stream loop.
    /// 
    /// Calling `start()` while already started implicitly calls `stop()` first.
    /// 
    /// OC: Replaces `[[OKTInstrumentContext alloc] initWithTrade:... dataSourceType:]` + `[context start]`.
    /// 
    /// # Retain Cycle Safety
    /// 
    /// The bridge holds `Arc<dyn KLineContextCallback>` internally. To avoid retain cycles,
    /// the native callback implementation **must not** hold a strong reference back to
    /// the bridge. The Swift wrapper layer (`OKKLineCTContextWrapper`) uses `weak var delegate`
    /// to guarantee this. `stop()` and `Drop` both clear the stored callback.
    /// 
    /// # Rapid restart safety
    /// 
    /// Calling `start()` twice in quick succession is safe. The `stop()` call bumps
    /// the generation counter, and the new `start()` bumps it again. Old spawned tasks
    /// detect staleness via `LifecycleGuard::is_current()` and exit silently.
    /// The `cancel_token` in state may diverge from the one captured by the old task,
    /// but the generation guard is the primary staleness check.
    /// 
    /// # Threading
    /// 
    /// Returns immediately. All async work spawns on `GlobalRuntime`.
    /// Callbacks fire on Tokio worker threads.
    pub fn start(&self, inst_id: String, biz_type: String, ds_type: u64, callback: Arc<dyn KLineContextCallback>) { todo!() }
    /// No-op — exists for API compatibility with OC's `[context startWS]`.
    /// In core-trading, HTTP and WS subscriptions are started together by `start_all()`.
    pub fn start_ws(&self) { todo!() }
    /// Stop all data sources, clear cached state, and release the callback.
    /// Safe to call multiple times — subsequent calls are no-ops.
    /// OC: Replaces `[context stop]` + manual ivar cleanup.
    pub fn stop(&self) { todo!() }
}

/// Callback trait for receiving real-time context data changes.
/// 
/// OC: Replaces `OKTInstrumentContextDelegate`.
/// 
/// # Threading & Safety
/// 
/// - Callbacks fire on a **background thread** (Tokio worker).
/// - Data accessors (e.g. `get_ticker`) use `try_lock`, so they ARE safe to call
/// from callbacks. However, mutating methods (`start`/`stop`) must NOT be called
/// from callbacks — dispatch to another thread/queue instead.
/// - **Retain cycle:** Implementations must NOT hold a strong reference to the
/// `KLineContextBridge` that owns this callback. Use weak references on the native side.
/// - Dispatch to the main thread on the native side for UI updates.
pub struct KLineContextCallback;
impl KLineContextCallback {
    /// Call-auction staging state changed (SPOT/MARGIN pre-market session).
    pub fn on_call_auction_staging_changed(&self, is_staging: bool) { todo!() }
    /// Coin-to-USD price changed (from CupTickers3s, used for fiat display).
    pub fn on_coin_usd_price_changed(&self, price: String) { todo!() }
    /// Delivery/funding timer countdown updated (~every second for SWAP/FUTURES/OPTION).
    pub fn on_delivery_timer_changed(&self, timer: KLineDeliveryTimer) { todo!() }
    /// An error occurred during the context lifecycle.
    pub fn on_error(&self, message: String) { todo!() }
    /// Estimated settlement price changed (options).
    pub fn on_estimated_price_changed(&self, price: String) { todo!() }
    /// Funding rate changed (SWAP / perpetual contracts only).
    pub fn on_funding_rate_changed(&self, funding_rate: KLineFundingRate) { todo!() }
    /// Index price changed (composite price from multiple exchanges).
    pub fn on_index_price_changed(&self, index_price: String) { todo!() }
    /// Instrument details have been fetched or changed; re-read metadata if needed.
    pub fn on_instrument_changed(&self) { todo!() }
    /// Mark price changed (smoothed price for margin calculations on derivatives).
    pub fn on_mark_price_changed(&self, mark_price: String) { todo!() }
    /// Open interest changed (total outstanding derivative contracts).
    pub fn on_open_interest_changed(&self, oi: String) { todo!() }
    /// Option summary (Greeks) changed. Only fired for option instruments.
    pub fn on_option_summary_changed(&self, summary: KLineOptionSummary) { todo!() }
    /// Pre-pending order staging state changed (SPOT/MARGIN).
    pub fn on_pre_pending_order_staging_changed(&self, is_staging: bool) { todo!() }
    /// Full ticker snapshot has changed (primary data delivery callback).
    pub fn on_ticker_changed(&self, ticker: KLineTickerSnapshot) { todo!() }
}

/// Bridge for real-time depth (order book) data.
/// OC: Replaces `OKTDepthDataSource`.
pub struct KLineDepthBridge;
impl KLineDepthBridge {
    pub fn new() -> Arc<KLineDepthBridge> { todo!() }
    /// Get current depth snapshot.
    pub fn get_snapshot(&self) -> Option<KLineDepthSnapshot> { todo!() }
    /// Start depth subscription for an instrument.
    /// `grouping`: depth grouping level (e.g. `"0.01"`).
    pub fn start(&self, inst_id: String, grouping: String, callback: Arc<dyn KLineDepthBridgeCallback>) { todo!() }
    /// Stop depth subscription.
    pub fn stop(&self) { todo!() }
}

/// Callback trait for depth data changes.
pub struct KLineDepthBridgeCallback;
impl KLineDepthBridgeCallback {
    pub fn on_depth_error(&self, message: String) { todo!() }
    pub fn on_depth_snapshot(&self, snapshot: KLineDepthSnapshot) { todo!() }
    pub fn on_depth_update(&self, snapshot: KLineDepthSnapshot) { todo!() }
}

pub struct KLineExploreBotsHalfScreenInteractor;
impl KLineExploreBotsHalfScreenInteractor {
    /// Session-driven constructor (mirrors kline_half_screen). Ticker/context still
    /// runs on this interactor's own per-inst KLineContextBridge.
    pub fn create_from_session(handle: Arc<TradeSessionHandle>) -> Arc<KLineExploreBotsHalfScreenInteractor> { todo!() }
    pub fn new_default() -> Arc<KLineExploreBotsHalfScreenInteractor> { todo!() }
    /// Returns `true` if switching to this period requires a new candle request.
    pub fn check_switch_period(&self, period: String) -> bool { todo!() }
    pub fn get_candle_count(&self) -> u32 { todo!() }
    /// Pull candle list. Native calls when `candle_version` changes.
    pub fn get_candles(&self) -> Vec<BizKLineData> { todo!() }
    /// Pull delivery countdown (futures only).
    pub fn get_delivery_timer(&self) -> Option<KLineDeliveryTimer> { todo!() }
    /// Pull funding rate (swap only).
    pub fn get_funding_rate(&self) -> Option<KLineFundingRate> { todo!() }
    /// Pull ticker snapshot. Native calls when `ticker_version` changes.
    pub fn get_ticker(&self) -> Option<KLineTickerSnapshot> { todo!() }
    /// Initialize with instrument. Triggers ensureInit → loadBiz → context start.
    pub fn initialize(&self, inst_id: String, biz_type: String, subscribe_ctx: bool) { todo!() }
    /// Session-driven init: resolve the instrument from the injected session, run the
    /// `initialize` chain, and follow coin switches. Unlike kline_half_screen the first
    /// candle start stays native-driven; `period` is only stored for switch restarts.
    pub fn initialize_from_session(&self, period: String, subscribe_ctx: bool) { todo!() }
    pub fn is_auction_staging(&self) -> bool { todo!() }
    /// Load more history candles (pagination).
    pub fn load_more(&self) { todo!() }
    /// Full stop and cleanup.
    pub fn on_clear(&self) { todo!() }
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
    /// Stock-split adjustment ("forward"/"backward"/""/"none"). Empty/"none" =
    /// no adjustment. Restarts the candle subscription so subsequent pushes carry
    /// the new adjustment (mirrors half-screen). No-op when unchanged.
    pub fn set_adjustment(&self, value: String) { todo!() }
    /// Corporate-action effectiveTime(ms) list for the adjusted realtime merge
    /// window check. Empty = fetched, no events; not called = None (not fetched →
    /// merge by default). Live — applies to subsequent WS batches, no restart.
    pub fn set_adjustment_effective_times(&self, times: Vec<i64>) { todo!() }
    /// Candle-cache mode switch (lean-cache design). `false` (default) = OFF:
    /// Rust keeps no cache/merge, forwards to native. `true` = ON: full cache +
    /// merge. MUST be called before `start_candle`; effective next `start_candle`.
    pub fn set_candle_cache_full(&self, enabled: bool) { todo!() }
    /// Enable or disable debug logging.
    /// 
    /// In release builds this is a no-op, but the symbol must exist
    /// so that UniFFI bindings generated in debug mode continue to
    /// link successfully against the release library.
    pub fn set_debug(&self, _enabled: bool) { todo!() }
    /// Override the default page size for load-more pagination.
    /// Must be called before `load_more`. Default is 300 (aligned with native `kPageLimit`).
    pub fn set_page_limit(&self, limit: u32) { todo!() }
    /// Set a state observer (UniFFI export).
    /// 
    /// DEPRECATED: callback-based observer. Use `state_stream()` —
    /// the callback observer pins the foreign object in UniFFI's
    /// static handleMap and produces a cross-language retain cycle
    /// (Android GC + iOS ARC).
    pub fn set_state_observer(&self, observer: Arc<dyn InteractorStateChangeObserver>) { todo!() }
    /// Self-resolved variant of [`set_adjustment`] (mirrors kline_half_screen):
    /// derives the stock-split adjustment for the CURRENT instrument internally
    /// (TradFi coin-table gate + persisted forward choice) so native carries no
    /// `isTradFi` knowledge. `stock_adjust_ab` is the native gray switch; `false`
    /// clears the adjustment. Routes through [`set_adjustment`] (no-op when unchanged).
    pub fn set_stock_adjust_auto(&self, stock_adjust_ab: bool) { todo!() }
    /// Start the interactor lifecycle.
    /// 
    /// Calls the user's `InteractorLifecycle::start()` implementation.
    /// Override that method to add custom startup behavior (e.g., initial data loads).
    pub fn start(&self) { todo!() }
    /// Start candle subscription for a period. Also used for period switching.
    /// `period` must be an API bar format (e.g. "1m", "5m", "1H", "1D"),
    /// NOT a UI period constant. Native caller must convert via `convertPeriodToType:` first.
    pub fn start_candle(&self, period: String) { todo!() }
    /// Restart context subscription (e.g. viewWillAppear after stopContext).
    pub fn start_context(&self) { todo!() }
    /// Get the current state (UniFFI export).
    /// 
    /// Returns the External type (UniFFI-compatible).
    pub fn state(&self) -> KLineExploreBotsHalfScreenState { todo!() }
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
    /// Stop candle subscription.
    pub fn stop_candle(&self) { todo!() }
    /// Stop context subscription (ticker WS).
    pub fn stop_context(&self) { todo!() }
}

pub struct KLineFormulaInteractor;
impl KLineFormulaInteractor {
    pub fn new_default() -> Arc<KLineFormulaInteractor> { todo!() }
    /// Returns `true` if switching to this period requires a new candle request.
    pub fn check_switch_period(&self, period: String) -> bool { todo!() }
    pub async fn fetch_indicator_line_ranges(&self, subscription_id: u32, expected_candle_revision: u64, expected_params_version: u64, ranges: Vec<KlineIndicatorLineRangeVo>) -> Result<KlineIndicatorDataFrameVo, OKFFIError> { todo!() }
    pub async fn fetch_indicator_viewport(&self, subscription_id: u32, expected_candle_revision: u64, expected_params_version: u64, start_index: u64, end_index: u64) -> Result<KlineIndicatorDataFrameVo, OKFFIError> { todo!() }
    pub async fn fetch_indicator_window(&self, subscription_id: u32, expected_candle_revision: u64, expected_params_version: u64, start_index: u64, end_index: u64) -> Result<KlineIndicatorDataFrameVo, OKFFIError> { todo!() }
    pub fn get_candle_count(&self) -> u32 { todo!() }
    /// Pull candle list. Native calls when `candle_version` changes.
    pub fn get_candles(&self) -> Vec<BizKLineData> { todo!() }
    /// Initialize with formula expression and coins.
    /// `formula`: expression like "$0+$1-$2"
    /// `formula_coins`: JSON string of coin params (inst_id_array format)
    /// `formula_title`: display title for uniqueID
    pub fn initialize(&self, formula: String, formula_coins: String, formula_title: String) { todo!() }
    /// Load more history candles (pagination).
    pub fn load_more(&self) { todo!() }
    /// Full stop and cleanup.
    pub fn on_clear(&self) { todo!() }
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
    /// DEPRECATED. Use `ui_event_stream()`; drop the returned stream
    /// wrapper to unsubscribe.
    pub fn remove_ui_event_observer(&self) { todo!() }
    /// Stock-split adjustment ("forward"/""/"none"). Empty/"none" = no adjustment.
    /// Restarts the candle subscription so subsequent HTTP history is re-pulled
    /// adjusted (mirrors half_screen/explore_bots). No-op when unchanged.
    pub fn set_adjustment(&self, value: String) { todo!() }
    /// Corporate-action effectiveTime(ms) union over the formula's stock instIds,
    /// used by the adjusted realtime merge window. Empty = fetched, no events; not
    /// called = None (not fetched → merge by default). Live — no candle restart.
    pub fn set_adjustment_effective_times(&self, times: Vec<i64>) { todo!() }
    /// Deprecated compatibility hook. CandleStore is now always enabled because
    /// it is shared with IndicatorRuntime.
    pub fn set_candle_cache_full(&self, enabled: bool) { todo!() }
    /// Enable or disable debug logging.
    /// 
    /// In release builds this is a no-op, but the symbol must exist
    /// so that UniFFI bindings generated in debug mode continue to
    /// link successfully against the release library.
    pub fn set_debug(&self, _enabled: bool) { todo!() }
    pub fn set_indicator_render_context(&self, surface_active: bool, main_enabled: bool, sub_enabled: bool) { todo!() }
    /// Timeline-only AVG is temporary and does not replace persisted selection.
    /// Keep the original three-argument entry point for existing native callers.
    pub fn set_indicator_render_context_with_timeline(&self, surface_active: bool, main_enabled: bool, sub_enabled: bool, timeline_average_enabled: bool) { todo!() }
    /// Set a state observer (UniFFI export).
    /// 
    /// DEPRECATED: callback-based observer. Use `state_stream()` —
    /// the callback observer pins the foreign object in UniFFI's
    /// static handleMap and produces a cross-language retain cycle
    /// (Android GC + iOS ARC).
    pub fn set_state_observer(&self, observer: Arc<dyn InteractorStateChangeObserver>) { todo!() }
    /// Self-resolved variant of [`set_adjustment`] (Android V2): collapses the
    /// formula's legs into one adjust decision internally — any TradFi leg with
    /// the persisted forward choice → "forward", otherwise none (the backend
    /// applies forward to stock legs only, so the collapse is safe).
    /// `formula_coins` is the same inst_id_array JSON passed to `initialize`
    /// (passed explicitly because `initialize`'s state write is async and may
    /// not be visible yet); the per-leg TradFi judgment stays in Rust — native
    /// carries no `isTradFi` knowledge. `stock_adjust_ab` is the native gray
    /// switch; `false` clears the adjustment. Routes through
    /// [`set_adjustment`], so unchanged values stay no-ops. Re-call to
    /// recompute (e.g. after the user flips the stock-price-adjustment setting).
    pub fn set_stock_adjust_auto(&self, formula_coins: String, stock_adjust_ab: bool) { todo!() }
    /// DEPRECATED: callback-based observer. Use `ui_event_stream()` —
    /// the callback observer pins the foreign object in UniFFI's
    /// static handleMap and produces a cross-language retain cycle
    /// (Android GC + iOS ARC).
    pub fn set_ui_event_observer(&self, observer: Arc<dyn FormulaIndicatorUiEventObserver>) { todo!() }
    /// Start the interactor lifecycle.
    /// 
    /// Calls the user's `InteractorLifecycle::start()` implementation.
    /// Override that method to add custom startup behavior (e.g., initial data loads).
    pub fn start(&self) { todo!() }
    /// Start candle subscription for a period.
    /// `period` must be API bar format (e.g. "1m", "5m", "1H", "1D").
    pub fn start_candle(&self, period: String) { todo!() }
    /// Get the current state (UniFFI export).
    /// 
    /// Returns the External type (UniFFI-compatible).
    pub fn state(&self) -> KLineFormulaState { todo!() }
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
    /// Stop candle subscription (WS only).
    pub fn stop_candle(&self) { todo!() }
    /// Opt-in async UI event stream (cycle-free). Single-subscriber.
    pub fn ui_event_stream(&self) -> Arc<FormulaIndicatorUiEventStream> { todo!() }
}

pub struct KLineHalfScreenInteractor;
impl KLineHalfScreenInteractor {
    /// V2 session-driven constructor: builds the interactor and injects the trade
    /// page's session as a `Weak` (via the opaque [`TradeSessionHandle`] that crossed
    /// FFI). The interactor then reads the current instrument + subscribes
    /// ticker/mark/index/funding + instrument switches from the session, instead of
    /// the native-fed `KLineContextBridge`.
    pub fn create_from_session(handle: Arc<TradeSessionHandle>) -> Arc<KLineHalfScreenInteractor> { todo!() }
    pub fn new_default() -> Arc<KLineHalfScreenInteractor> { todo!() }
    /// Returns `true` if switching to this period requires a new candle request.
    pub fn check_switch_period(&self, period: String) -> bool { todo!() }
    pub async fn fetch_indicator_line_ranges(&self, subscription_id: u32, expected_candle_revision: u64, expected_params_version: u64, ranges: Vec<KlineIndicatorLineRangeVo>) -> Result<KlineIndicatorDataFrameVo, OKFFIError> { todo!() }
    pub async fn fetch_indicator_viewport(&self, subscription_id: u32, expected_candle_revision: u64, expected_params_version: u64, start_index: u64, end_index: u64) -> Result<KlineIndicatorDataFrameVo, OKFFIError> { todo!() }
    pub async fn fetch_indicator_window(&self, subscription_id: u32, expected_candle_revision: u64, expected_params_version: u64, start_index: u64, end_index: u64) -> Result<KlineIndicatorDataFrameVo, OKFFIError> { todo!() }
    /// Earliest available candle ts for the current bar — the X-perp backfill
    /// boundary for the half-screen tint region. Cache hit on (inst_id,
    /// kline_type) returns immediately; miss fetches the initial-time list once.
    /// Mirrors `KlineCandlesInteractor::fetch_start_time_for_current_bar`.
    pub async fn fetch_start_time_for_current_bar(&self) -> i64 { todo!() }
    /// Pull current assets from OPA manager. Native calls when `asset_version` changes.
    /// Raw fields are preserved; formatted_* fields are populated using
    /// base_dig (from IndexConfig) for balance fields and digit for price/profit fields.
    /// 
    /// OC: `[instCtx.tradeConvertor targetFormatter:cashBal].toLocalizedString`
    /// where tradeConvertor.targetUnitPrecision = baseDig (NOT lot_sz).
    /// baseDig is the base currency display precision from `/priapi/v5/public/config`.
    pub fn get_assets(&self) -> Vec<RetailKLineAsset> { todo!() }
    pub fn get_candle_count(&self) -> u32 { todo!() }
    /// Pull candle list. Native calls when `candle_version` changes.
    pub fn get_candles(&self) -> Vec<BizKLineData> { todo!() }
    /// Pull delivery countdown (futures only). Read live from the session's `InstContext`.
    pub fn get_delivery_timer(&self) -> Option<KLineDeliveryTimer> { todo!() }
    /// Pull funding rate (swap only). Read live from the session's `InstContext`.
    pub fn get_funding_rate(&self) -> Option<KLineFundingRate> { todo!() }
    /// Pull current orders from OPA manager. Native calls when `order_position_version` changes.
    /// Contract order sizes follow the same trade-convertor path as native OC convertors.
    pub fn get_orders(&self) -> Vec<RetailKLineOrder> { todo!() }
    /// Pull current positions from OPA manager. Native calls when `order_position_version` changes.
    /// Position size follows native cross/isolated account gating and trade-convertor rounding.
    /// `formatted_profit_rate` = ratio * 100 as "X.XX%".
    pub fn get_positions(&self) -> Vec<RetailKLinePosition> { todo!() }
    /// Pull quote coin price from context (for asset calculation).
    pub fn get_quote_coin_price(&self) -> Option<String> { todo!() }
    /// Pull ticker snapshot. Native calls when `ticker_version` changes.
    /// Assembled live from the shared session's `InstContext` (reuses the bridge's
    /// `build_ticker_snapshot`); no separate KLine context subscription.
    pub fn get_ticker(&self) -> Option<KLineTickerSnapshot> { todo!() }
    /// Full display-ready trade overlay (orders + positions + spot) via the shared
    /// [`build_view_state`] pipeline — renders identically to the full-screen quotes
    /// page. Called on `order_position_version` / `asset_version` bump.
    pub fn get_trade_overlay_view_state(&self) -> KlineTradeOverlayViewState { todo!() }
    pub fn initialize(&self, inst_id: String, biz_type: String, subscribe_ctx: bool) { todo!() }
    pub fn initialize_from_session(&self, period: String, subscribe_ctx: bool) { todo!() }
    /// Fixed-instrument init (bot host): use the passed inst directly, no session / coin-switch.
    /// `forced_price_type` pins the price mode + hides the chip; `series_id` (event instFamily)
    /// resolves the underlying index (settlement-safe). Both None for the normal bot path.
    pub fn initialize_with_instrument(&self, inst_id: String, biz_type: String, period: String, subscribe_ctx: bool, forced_price_type: Option<String>, series_id: Option<String>) { todo!() }
    pub fn is_auction_staging(&self) -> bool { todo!() }
    /// True when the active price mode is Default (Last) — drives overlay
    /// suppression in the order/position/asset getters. Mark/Index → false.
    pub fn is_default_price_mode(&self) -> bool { todo!() }
    pub fn is_pre_pending_order_staging(&self) -> bool { todo!() }
    /// Prepend older candles for pagination. SkipIfRunning to prevent duplicates.
    pub fn load_more(&self) { todo!() }
    pub fn on_clear(&self) { todo!() }
    /// Pause every page-owned asynchronous source without destroying cached state or the observer.
    /// A later initialize call performs a fresh, race-free restart.
    pub fn pause(&self) { todo!() }
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
    /// DEPRECATED. Use `ui_event_stream()`; drop the returned stream
    /// wrapper to unsubscribe.
    pub fn remove_ui_event_observer(&self) { todo!() }
    /// Stock-split adjustment ("forward" / "backward" / empty / "none" all
    /// supported; empty and "none" both map to "no adjustment"). Mirrors
    /// `quote/kline_candles::set_stock_adjustment` — invalidates the candle
    /// cache and re-spawns the subscription so subsequent pushes carry the
    /// new adjustment. No-op when value is unchanged.
    pub fn set_adjustment(&self, value: String) { todo!() }
    /// Set the corporate-action effectiveTime(ms) list for the current instId,
    /// used by the adjusted realtime merge window check. Passing an empty list
    /// means "fetched, no events"; not calling it at all leaves the internal
    /// state at `None` ("not fetched" → merge by default). Live — applies to
    /// subsequent WS batches without restarting the candle subscription.
    pub fn set_adjustment_effective_times(&self, times: Vec<i64>) { todo!() }
    /// Compatibility no-op. Half-screen always uses CandleStore as the single
    /// Rust candle authority; retained temporarily for existing generated clients.
    pub fn set_candle_cache_full(&self, enabled: bool) { todo!() }
    /// Enable or disable debug logging.
    /// 
    /// In release builds this is a no-op, but the symbol must exist
    /// so that UniFFI bindings generated in debug mode continue to
    /// link successfully against the release library.
    pub fn set_debug(&self, _enabled: bool) { todo!() }
    /// Temporary availability for this half-screen surface. Persistent
    /// selection remains owned by candle-support settings.
    pub fn set_indicator_render_context(&self, surface_active: bool, main_enabled: bool, sub_enabled: bool, timeline_average_enabled: bool) { todo!() }
    pub fn set_indicator_trading_unit(&self, unit: String) { todo!() }
    pub fn set_page_limit(&self, limit: u32) { todo!() }
    /// Switch the candle price mode: "kline" (Last) | "mark" | "index".
    /// Re-spawns the candle subscription with the matching kline_type (Index
    /// resolves to the underlying); bumps order/asset versions so native re-pulls
    /// the overlay (cleared for Mark/Index, restored for Default). No-op when
    /// unchanged.
    pub fn set_price_mode(&self, price_type: String) { todo!() }
    /// Set a state observer (UniFFI export).
    /// 
    /// DEPRECATED: callback-based observer. Use `state_stream()` —
    /// the callback observer pins the foreign object in UniFFI's
    /// static handleMap and produces a cross-language retain cycle
    /// (Android GC + iOS ARC).
    pub fn set_state_observer(&self, observer: Arc<dyn InteractorStateChangeObserver>) { todo!() }
    /// Self-resolved variant of [`set_adjustment`]: derives the stock-split
    /// adjustment for the CURRENT instrument internally (TradFi coin-table gate
    /// + persisted forward choice, same judgment as the main-K candles request)
    /// so native carries no `isTradFi` knowledge. `stock_adjust_ab` is the
    /// native gray switch for the stock-adjust feature; `false` clears the
    /// adjustment. Call on attach and after every instrument switch; routes
    /// through [`set_adjustment`], so unchanged values stay no-ops.
    pub fn set_stock_adjust_auto(&self, stock_adjust_ab: bool) { todo!() }
    /// DEPRECATED: callback-based observer. Use `ui_event_stream()` —
    /// the callback observer pins the foreign object in UniFFI's
    /// static handleMap and produces a cross-language retain cycle
    /// (Android GC + iOS ARC).
    pub fn set_ui_event_observer(&self, observer: Arc<dyn HalfScreenIndicatorUiEventObserver>) { todo!() }
    /// Start the interactor lifecycle.
    /// 
    /// Calls the user's `InteractorLifecycle::start()` implementation.
    /// Override that method to add custom startup behavior (e.g., initial data loads).
    pub fn start(&self) { todo!() }
    /// Subscribe candles for period. `period` must be API bar format (e.g. "1H").
    pub fn start_candle(&self, period: String) { todo!() }
    /// Restart context WS. Stops existing bridge first.
    pub fn start_context(&self) { todo!() }
    /// Start order/position/asset subscriptions.
    pub fn start_order_position_asset(&self) { todo!() }
    /// Get the current state (UniFFI export).
    /// 
    /// Returns the External type (UniFFI-compatible).
    pub fn state(&self) -> KLineHalfScreenState { todo!() }
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
    pub fn stop_candle(&self) { todo!() }
    /// Stop context WS. Uses write().take() to prevent races.
    pub fn stop_context(&self) { todo!() }
    /// Stop order/position/asset subscriptions.
    pub fn stop_order_position_asset(&self) { todo!() }
    /// Opt-in async UI event stream (cycle-free). Single-subscriber.
    pub fn ui_event_stream(&self) -> Arc<HalfScreenIndicatorUiEventStream> { todo!() }
}

/// Unified bridge for order, position, and asset data sources.
/// 
/// OC: Replaces `OKTLiveManualOrderDataSource`, `OKTPositionLiveManualDataSource`,
/// `OKTAssetLiveDataSource`, and `OKKAssetPositionOrderManager`.
/// 
/// A generation counter with [`LifecycleGuard`] ensures stale tasks from a previous
/// `start()` cycle do not deliver callbacks after `stop()`.
pub struct KLineOrderPositionAssetBridge;
impl KLineOrderPositionAssetBridge {
    /// Create a new, idle bridge. No subscriptions until [`start`](Self::start).
    pub fn new() -> Arc<KLineOrderPositionAssetBridge> { todo!() }
    /// Get current assets (spot trading only).
    /// OC: `OKTAssetLiveDataSource.assets`
    pub fn get_assets(&self) -> Vec<RetailKLineAsset> { todo!() }
    /// Get current open orders. Use `get_orders_and_positions` for atomic read of both.
    pub fn get_orders(&self) -> Vec<RetailKLineOrder> { todo!() }
    /// Get orders and positions as a single atomic snapshot.
    /// OC: Replaces separate `orderDataSource.orders` + `positionDataSource.positions`.
    pub fn get_orders_and_positions(&self) -> KLineOrdersAndPositions { todo!() }
    /// Get current open positions. Use `get_orders_and_positions` for atomic read of both.
    pub fn get_positions(&self) -> Vec<RetailKLinePosition> { todo!() }
    /// Get last price of the quote coin (spot only, used for cost line calculation).
    /// OC: `OKTAssetLiveDataSource.quoteCoinPrice`
    pub fn get_quote_coin_price(&self) -> Option<String> { todo!() }
    /// Start all data sources (order + position + asset) for the given instrument.
    /// If already running, the previous subscription is stopped first (implicit restart).
    /// 
    /// OC: Replaces multiple `[[OKTXxxDataSource alloc] init... start]` calls.
    pub fn start(&self, inst_id: String, biz_type: String, callback: Arc<dyn KLineOrderPositionAssetBridgeCallback>) { todo!() }
    /// Stop all data sources and release resources.
    /// OC: Replaces scattered `[dataSource stop]` calls.
    pub fn stop(&self) { todo!() }
}

/// Callback trait for order/position/asset data change notifications.
/// 
/// OC: Replaces `OKTLiveListDataSourceDelegate`.
/// 
/// # Threading
/// 
/// All methods fire on a **background thread**. Dispatch to main thread for UI updates.
pub struct KLineOrderPositionAssetBridgeCallback;
impl KLineOrderPositionAssetBridgeCallback {
    /// Asset/balance list has changed (spot trading only).
    pub fn on_assets_changed(&self, assets: Vec<RetailKLineAsset>) { todo!() }
    /// Error in an underlying data source (`source`: `"order"`, `"position"`, `"asset"`, `"quote_coin_price"`).
    pub fn on_error(&self, source: String, message: String) { todo!() }
    /// Orders and/or positions have changed.
    pub fn on_orders_and_positions_changed(&self, orders: Vec<RetailKLineOrder>, positions: Vec<RetailKLinePosition>) { todo!() }
}

/// PIP Context bridge — lightweight ticker/markPrice subscription for the floating PIP chart.
/// OC: replaces `OKTPipContext`.
pub struct KLinePipContextBridge;
impl KLinePipContextBridge {
    pub fn new() -> Arc<KLinePipContextBridge> { todo!() }
    /// Get the cached mark price string. Uses try_lock — safe to call from callbacks.
    /// OC: replaces `self.pipContext.markPrice`.
    pub fn get_mark_price(&self) -> Option<String> { todo!() }
    /// Get the cached ticker snapshot. Uses try_lock — safe to call from callbacks.
    /// OC: replaces `self.pipContext.ticker`.
    pub fn get_ticker(&self) -> Option<KLineTickerSnapshot> { todo!() }
    /// Start PIP context with ticker (+ optional mark price) subscription.
    /// OC: replaces `[[OKTPipContext alloc] initWithTrade:instrument:dataType:]` + `[start]`.
    /// 
    /// If already running, stops the previous WS subscription first but preserves
    /// cached ticker data until new data arrives. Safe to call from any thread.
    pub fn start(&self, inst_id: String, biz_type: String, data_source_type: u64, callback: Arc<dyn KLinePipContextCallback>) { todo!() }
    /// Type-safe variant of [`start`](Self::start) using [`KLinePipDataType`] enum instead of raw bitflags.
    pub fn start_typed(&self, inst_id: String, biz_type: String, data_type: KLinePipDataType, callback: Arc<dyn KLinePipContextCallback>) { todo!() }
    /// Stop all subscriptions and clear cached data. OC: replaces `[_pipContext stop]`.
    pub fn stop(&self) { todo!() }
    /// Stop WebSocket only, keeping cached ticker data for display.
    /// OC: replaces `[_pipContext stopWS]`.
    pub fn stop_ws(&self) { todo!() }
}

/// Callback trait for PIP context updates. OC: replaces `OKTPIPContextDelegate`.
/// 
/// Invoked on a **background thread**. Data accessors (`get_ticker`, `get_mark_price`)
/// use `try_lock` and are safe to call from callbacks. Mutating methods (`start`/`stop`)
/// should NOT be called from callbacks — dispatch to another queue instead.
pub struct KLinePipContextCallback;
impl KLinePipContextCallback {
    pub fn on_error(&self, message: String) { todo!() }
    /// OC: replaces `pipContextMarkPriceChanged:`.
    pub fn on_mark_price_changed(&self, mark_price: String) { todo!() }
    /// OC: replaces `pipContextTickerChanged:`.
    pub fn on_ticker_changed(&self, ticker: KLineTickerSnapshot) { todo!() }
}

pub struct KLinePipInteractor;
impl KLinePipInteractor {
    /// V2 session-driven constructor: builds the interactor and injects the trade
    /// page's session as a `Weak` (via the opaque [`TradeSessionHandle`] that
    /// crossed FFI). Follow with `initialize_from_session`.
    pub fn create_from_session(handle: Arc<TradeSessionHandle>) -> Arc<KLinePipInteractor> { todo!() }
    pub fn new_default() -> Arc<KLinePipInteractor> { todo!() }
    pub fn get_candle_count(&self) -> u32 { todo!() }
    /// Pull all candles. Native calls when `candle_version` changes.
    pub fn get_candles(&self) -> Vec<BizKLineData> { todo!() }
    /// Pull the latest mark price string (option PIP uses this).
    pub fn get_mark_price(&self) -> Option<String> { todo!() }
    /// Pull the latest ticker snapshot. Native calls when `ticker_version` changes.
    pub fn get_ticker(&self) -> Option<KLineTickerSnapshot> { todo!() }
    /// Build display recipes for the current ticker snapshot. Value selection
    /// mirrors the native VM (option main price = markPrice); formatting mirrors
    /// the head-info recipe assembly (same KlineFormatter SSOT). Pull alongside
    /// `get_ticker()` on every `ticker_version` bump.
    /// 
    /// Coin-switch window: post-switch state may pair the new inst_id with stale
    /// is_option/is_positive for one frame — native rebuilds the VM on seq bump.
    pub fn get_ticker_recipes(&self) -> KLinePipTickerRecipes { todo!() }
    /// ensureInit → loadBiz → instrument info → start PIP context.
    pub fn initialize(&self, inst_id: String, biz_type: String) { todo!() }
    /// Session-driven init (V2): resolve inst_id/biz_type from the injected
    /// `TradeSessionContext` and run `initialize`; watch `InstrumentChanged`
    /// for coin switches. Requires `create_from_session`. Candle start stays
    /// native-driven (after `is_initialized`), same as the legacy path.
    pub fn initialize_from_session(&self) { todo!() }
    pub fn on_clear(&self) { todo!() }
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
    /// Stock-split adjustment forwarded into subsequent candle requests.
    /// Pass empty string to clear (OC `nil`).
    pub fn set_adjustment(&self, value: String) { todo!() }
    /// Set the corporate-action effectiveTime(ms) list for the current instId,
    /// used by the adjusted realtime merge window check. Empty list = fetched,
    /// no events; not calling it leaves state at `None` (not fetched → merge by
    /// default). Live — applies to subsequent WS batches without a candle restart.
    pub fn set_adjustment_effective_times(&self, times: Vec<i64>) { todo!() }
    /// Candle-cache mode switch (lean-cache design). `false` (default) = OFF:
    /// Rust keeps no cache/merge, forwards to native. `true` = ON: full cache +
    /// merge. MUST be called before `start_candle`; effective next `start_candle`.
    pub fn set_candle_cache_full(&self, enabled: bool) { todo!() }
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
    /// Self-resolved variant of [`set_adjustment`] (mirrors kline_half_screen):
    /// derives the stock-split adjustment for the CURRENT instrument internally
    /// (TradFi coin-table gate + persisted forward choice) so native carries no
    /// `isTradFi` knowledge. `stock_adjust_ab` is the native gray switch; `false`
    /// clears the adjustment. Routes through [`set_adjustment`] (no-op when unchanged).
    pub fn set_stock_adjust_auto(&self, stock_adjust_ab: bool) { todo!() }
    /// Start the interactor lifecycle.
    /// 
    /// Calls the user's `InteractorLifecycle::start()` implementation.
    /// Override that method to add custom startup behavior (e.g., initial data loads).
    pub fn start(&self) { todo!() }
    /// Start candle WS for `period` (API bar format, e.g. "1H").
    pub fn start_candle(&self, period: String) { todo!() }
    /// (Re)start PIP context WS (ticker / markPrice). No-op if not initialized.
    pub fn start_pip_context(&self) { todo!() }
    /// Get the current state (UniFFI export).
    /// 
    /// Returns the External type (UniFFI-compatible).
    pub fn state(&self) -> KLinePipState { todo!() }
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
    /// Stop candle WS + tear down PIP context (mirrors OC `candleDataShouldEndLoad:`).
    pub fn stop_candle(&self) { todo!() }
}

/// Transaction (latest deal) data source bridge.
/// OC: replaces `OKTTransactionDataSource`.
pub struct KLineTransactionBridge;
impl KLineTransactionBridge {
    pub fn new() -> Arc<KLineTransactionBridge> { todo!() }
    /// Get the cached list of transaction items. Uses try_lock — safe to call from callbacks.
    /// OC: replaces `transactionDataSource.items`.
    pub fn get_items(&self) -> Vec<KLineTransaction> { todo!() }
    /// Start the transaction data source for the given instrument.
    /// OC: replaces `[[OKTTransactionDataSource alloc] initWithInstId:instId]` + `[start]`.
    /// 
    /// If already running, stops the previous subscription first (implicit restart).
    /// Safe to call from any thread; callbacks arrive on a background thread.
    pub fn start(&self, inst_id: String, callback: Arc<dyn KLineTransactionCallback>) { todo!() }
    /// Stop the transaction data source and clear cached data.
    /// OC: replaces `[transactionDataSource stop]`.
    pub fn stop(&self) { todo!() }
}

/// Callback trait for transaction data source notifications.
/// OC: replaces `OKTTransactionDataSourceDelegate`.
/// 
/// Invoked on a **background thread**. Dispatch to main thread for UI updates.
pub struct KLineTransactionCallback;
impl KLineTransactionCallback {
    /// Full snapshot replacement of the transaction list.
    pub fn on_refreshed(&self, items: Vec<KLineTransaction>) { todo!() }
    /// Called when start completes. `error` is `None` on success, `Some(message)` on failure.
    pub fn on_start_finished(&self, error: Option<String>) { todo!() }
}

pub struct KlineArbBottomInteractor;
impl KlineArbBottomInteractor {
    /// Load all biz lines for `params.inst_id` and publish the SPOT / MARGIN
    /// icon decision. Cancels any in-flight load on re-entry.
    pub fn ensure(&self, params: KlineArbBottomParams) { todo!() }
    pub fn on_clear(&self) { todo!() }
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
    pub fn state(&self) -> KlineArbBottomState { todo!() }
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
}

pub struct KlineBottomInteractor;
impl KlineBottomInteractor {
    /// Load all five biz lines for `params.inst_id` and publish the derived
    /// portrait decisions. Cancels any in-flight load on re-entry.
    pub fn ensure(&self, params: KlineBottomParams) { todo!() }
    pub fn on_clear(&self) { todo!() }
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
    pub fn state(&self) -> KlineBottomState { todo!() }
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
}

pub struct KlineBuySellPointsInteractor;
impl KlineBuySellPointsInteractor {
    pub fn on_clear(&self) { todo!() }
    /// Refresh markers for the visible candle window. Cancels any in-flight
    /// fetch. Logged-out callers get an empty list (V1 `isLogin()` gate).
    /// `price_mode` selects the V1 price-conversion branch (index mode scales
    /// inverse / option prices to local fiat).
    /// `sid` scopes the query to one strategy (Bot K-line); `None` / empty
    /// omits the param (V1 `historyOrderSid` behaviour).
    pub fn refresh(&self, inst_id: String, inst_type: String, bar: String, type_time: String, start: String, end: String, price_mode: KLinePriceMode, sid: Option<String>) { todo!() }
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
    pub fn state(&self) -> KlineBuySellPointsState { todo!() }
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
}

pub struct KlineCandlesInteractor;
impl KlineCandlesInteractor {
    pub fn change_period(&self, subscription_id: u32, bar: String) { todo!() }
    pub fn change_price_type(&self, subscription_id: u32, price_type: String) { todo!() }
    pub fn current_candle_revision(&self) -> u64 { todo!() }
    /// One-shot history-window fetch for a manager-driven jump. The shared
    /// native KlineFastJumpManager owns jump orchestration (in-range decision,
    /// scroll animation — chart geometry) and requests the window via this call.
    /// `bar` is explicit because a period-switch anchor jumps on the new bar
    /// before the ws-only resume updates `metadata.bar`; underlying / price-type
    /// / adjustment resolution stays in Rust (read from metadata). The call replaces the store
    /// window but does not start a streaming subscription or emit transport frames; the following
    /// ws-only resume installs the new observer and explicitly replays the cached generation.
    pub async fn fetch_history_window(&self, bar: String, after_ts: i64, limit: u32) -> Result<Vec<KlineQuoteCandleVo>, OKFFIError> { todo!() }
    /// Binary counterpart of `fetch_history_window` for consumers that opted into binary frames.
    /// The existing string-based API remains available to iOS and legacy clients.
    pub async fn fetch_history_window_native(&self, bar: String, after_ts: i64, limit: u32) -> Result<Vec<u8>, OKFFIError> { todo!() }
    /// Returns one OKID packet whose individual lines may use different
    /// absolute windows. This is the prepend/fling path: Native decides which
    /// cached chunks are missing, while Rust remains the sole owner of values.
    pub async fn fetch_indicator_line_ranges(&self, subscription_id: u32, expected_candle_revision: u64, expected_params_version: u64, ranges: Vec<KlineIndicatorLineRangeVo>) -> Result<KlineIndicatorDataFrameVo, OKFFIError> { todo!() }
    /// Returns exact values for indicators whose formula depends on the
    /// visible candle interval (currently SR). Ordinary lines remain empty in
    /// the packet so their stable global ids are preserved without retransmit.
    pub async fn fetch_indicator_viewport(&self, subscription_id: u32, expected_candle_revision: u64, expected_params_version: u64, start_index: u64, end_index: u64) -> Result<KlineIndicatorDataFrameVo, OKFFIError> { todo!() }
    /// Returns one combined OKID packet for every active indicator line in the
    /// requested absolute candle range. Formula calculation and the full
    /// ledger remain internal to Rust; Native only sees this Interactor API.
    pub async fn fetch_indicator_window(&self, subscription_id: u32, expected_candle_revision: u64, expected_params_version: u64, start_index: u64, end_index: u64) -> Result<KlineIndicatorDataFrameVo, OKFFIError> { todo!() }
    /// Cache hit returns immediately; cache miss does a synchronous HTTP fetch.
    /// Used by native to clamp the jump target before starting a jump.
    pub async fn fetch_start_time_for_current_bar(&self) -> i64 { todo!() }
    pub fn get_start_time_for_current_bar(&self) -> i64 { todo!() }
    /// Invalidates direct time-anchor HTTP results that have not committed yet. Native calls this
    /// synchronously before it publishes a new full-series command, closing the small gap before
    /// the replacement async subscription enters Rust.
    pub fn invalidate_history_window_requests(&self) { todo!() }
    pub fn load_history_left(&self, before_ts: i64) { todo!() }
    pub fn load_history_right(&self, after_ts: i64) { todo!() }
    pub fn on_clear(&self) { todo!() }
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
    /// DEPRECATED. Use `ui_event_stream()`; drop the returned stream
    /// wrapper to unsubscribe.
    pub fn remove_ui_event_observer(&self) { todo!() }
    /// Corporate-action effectiveTime(ms) list for the active instId. Fed by
    /// native after `CandleAdjustmentInteractor` REST resolves; consulted live
    /// per WS batch to gate the close-only adjusted merge window. Mirrors
    /// `KLineHalfScreenActions::set_adjustment_effective_times`.
    pub fn set_adjustment_effective_times(&self, times: Vec<i64>) { todo!() }
    /// Sync the chart's fast-K (anchored historical browse) state in from native. While set,
    /// live WS ticks are not merged into `candle_store` (keeps it ≡ the displayed series).
    pub fn set_anchored(&self, anchored: bool) { todo!() }
    /// Set the active bar in metadata only (no subscription / store side effects). Used before a
    /// period-switch anchor jump so `fetch_start_time_for_current_bar` resolves the NEW bar's
    /// earliest candle for the clamp / fallback decision; the ws-only resume then subscribes it.
    pub fn set_current_bar(&self, bar: String) { todo!() }
    /// Enable or disable debug logging.
    /// 
    /// In release builds this is a no-op, but the symbol must exist
    /// so that UniFFI bindings generated in debug mode continue to
    /// link successfully against the release library.
    pub fn set_debug(&self, _enabled: bool) { todo!() }
    /// Set temporary indicator availability for this page. This never writes
    /// persistent selection; Rust recomputes the effective schema instead.
    pub fn set_indicator_render_context(&self, surface_active: bool, main_enabled: bool, sub_enabled: bool, timeline_average_enabled: bool) { todo!() }
    /// Set the display unit used by server-backed trading indicators.
    /// The runtime owns the feed; Native only forwards the page preference.
    pub fn set_indicator_trading_unit(&self, unit: String) { todo!() }
    /// Deprecated; prefetch fills the table automatically.
    pub fn set_kline_start_time(&self, ts: i64) { todo!() }
    /// Enable versioned binary candle callbacks for this interactor instance.
    /// Call before subscribing. The default remains disabled for legacy consumers.
    pub fn set_native_frame_enabled(&self, enabled: bool) { todo!() }
    /// Register the binary candle callback used when NativeFrame mode is enabled.
    pub fn set_native_frame_observer(&self, observer: Box<dyn KlineCandlesNativeFrameObserver>) { todo!() }
    /// Set price-type ("kline" / "mark" / "index" / `TYPE_*_KLINE` variants)
    /// without spawning a subscription. Native uses this in the streaming-
    /// subscribe pattern: write metadata, then drive a fresh subscription via
    /// `subscribe_streaming` (which reads `metadata.price_type` internally).
    pub fn set_price_type(&self, price_type: String) { todo!() }
    /// Set a state observer (UniFFI export).
    /// 
    /// DEPRECATED: callback-based observer. Use `state_stream()` —
    /// the callback observer pins the foreign object in UniFFI's
    /// static handleMap and produces a cross-language retain cycle
    /// (Android GC + iOS ARC).
    pub fn set_state_observer(&self, observer: Arc<dyn InteractorStateChangeObserver>) { todo!() }
    /// DEPRECATED: callback-based observer. Use `ui_event_stream()` —
    /// the callback observer pins the foreign object in UniFFI's
    /// static handleMap and produces a cross-language retain cycle
    /// (Android GC + iOS ARC).
    pub fn set_ui_event_observer(&self, observer: Arc<dyn KlineCandlesUiEventObserver>) { todo!() }
    /// Start the interactor lifecycle.
    /// 
    /// Calls the user's `InteractorLifecycle::start()` implementation.
    /// Override that method to add custom startup behavior (e.g., initial data loads).
    pub fn start(&self) { todo!() }
    /// Get the current state (UniFFI export).
    /// 
    /// Returns the External type (UniFFI-compatible).
    pub fn state(&self) -> KlineCandlesState { todo!() }
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
    /// Start (or restart) the candle subscription. `subscription_id` lets old
    /// pushes be discarded after a period / price-type / adjustment change.
    pub fn subscribe(&self, subscription_id: u32, inst_id: String, inst_type: String, bar: String) { todo!() }
    pub async fn subscribe_streaming(&self, subscription_id: u32, inst_id: String, inst_type: String, bar: String, cancel_token: Arc<OKCancellationToken>) -> Result<(), OKFFIError> { todo!() }
    /// WS-only variant: skips the HTTP seed and preserves the current render series.
    /// CandleStore retains the series while `candles_list` stays empty (both frame modes), so
    /// no duplicate candle data crosses UniFFI.
    pub async fn subscribe_streaming_ws_only(&self, subscription_id: u32, inst_id: String, inst_type: String, bar: String, cancel_token: Arc<OKCancellationToken>) -> Result<(), OKFFIError> { todo!() }
    /// Opt-in async UI event stream (cycle-free). Single-subscriber.
    pub fn ui_event_stream(&self) -> Arc<KlineCandlesUiEventStream> { todo!() }
}

/// Per-frame candle delivery. `state.candles_list` is latest-value: a WS tick
/// written between two native pulls silently overwrites the previous frame
/// (fatal when it erases the HTTP seed → blank chart / single candle). This
/// Legacy event consumers receive every frame in write order. Android's opt-in binary callback
/// receives equivalent frames while `candles_list` stays empty to avoid a second FFI transfer.
pub struct KlineCandlesNativeFrameObserver;
impl KlineCandlesNativeFrameObserver {
    pub fn on_native_frame(&self, subscription_id: u32, payload: Vec<u8>, kind: CandlesUpdateKind, candle_revision: u64) { todo!() }
}

/// Deprecated UniFFI callback interface for observing UI events. Prefer the cycle-free `KlineCandlesUiEventStream` returned by `interactor.ui_event_stream()`.
pub struct KlineCandlesUiEventObserver;
impl KlineCandlesUiEventObserver {
    pub fn on_ui_event(&self, event: KlineCandlesUiEvent) { todo!() }
}

/// Opt-in async event stream from `interactor.ui_event_stream()`.
/// Single-subscriber, latest-wins. `next()` resolves to `None` when the
/// interactor is dropped or a new subscription replaces this one.
pub struct KlineCandlesUiEventStream;
impl KlineCandlesUiEventStream {
    pub async fn next(&self) -> Option<KlineCandlesUiEvent> { todo!() }
}

pub struct KlineCexStockHoursInteractor;
impl KlineCexStockHoursInteractor {
    pub fn on_clear(&self) { todo!() }
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
    pub fn state(&self) -> KlineCexStockHoursState { todo!() }
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
    /// Subscribe the region-aware CEX market status. `inst_id` may be empty —
    /// the backend then defaults to US_STOCK. Re-invoke after a device
    /// timezone or app language change so the VO re-renders with fresh
    /// locale output.
    pub fn subscribe(&self, inst_id: String) { todo!() }
}

/// CryptoInfoService - Unified service for cryptocurrency information
/// 
/// This service aggregates all crypto info related APIs into a single object,
/// providing a cleaner API for mobile developers.
/// 
/// # Available Methods
/// 
/// - `get_base_coin_info`: Fetch base coin information (market ranking, prices, etc.)
/// - `get_coin_data_info`: Fetch coin data information (market cap, volume, etc.)
/// - `get_coin_detail`: Fetch detailed project information (team, social media, etc.)
/// - `get_campaign_list`: Fetch campaign list (active promotions, events, etc.)
/// 
/// # Example (Swift)
/// 
/// ```swift
/// let service = CryptoInfoService()
/// let req = service.createBaseCoinInfoReq(ccy: "BTC", cacheExpireSeconds: 300)
/// let result = try await service.getBaseCoinInfo(req: req)
/// ```
/// 
/// # Example (Kotlin)
/// 
/// ```kotlin
/// val service = CryptoInfoService()
/// val req = service.createBaseCoinInfoReq(ccy = "BTC", cacheExpireSeconds = 300)
/// val result = service.getBaseCoinInfo(req)
/// ```
pub struct KlineCryptoInfoService;
impl KlineCryptoInfoService {
    /// Creates a new KlineCryptoInfoService instance.
    pub fn new() -> Arc<KlineCryptoInfoService> { todo!() }
    /// Creates a new `BaseCoinInfoReq` with the specified parameters.
    /// 
    /// # Parameters
    /// - `ccy`: Currency symbol (e.g., "BTC", "ETH")
    /// - `cache_expire_seconds`: Cache expiration time in seconds (0 = no cache)
    /// 
    /// # Returns
    /// A new `BaseCoinInfoReq` instance
    pub fn create_base_coin_info_req(&self, ccy: String, cache_expire_seconds: u64) -> KlineBaseCoinInfoReq { todo!() }
    /// Creates a new `CampaignListReq` with the specified parameters.
    /// 
    /// # Parameters
    /// - `ccy`: Currency symbol (e.g., "BTC", "ETH")
    /// - `location_code`: Location code (default: "over_view")
    /// - `inst_name`: Instrument name
    /// - `biz_type`: Business type
    /// - `cache_expire_seconds`: Cache expiration time in seconds (0 = no cache)
    /// 
    /// # Returns
    /// A new `CampaignListReq` instance
    pub fn create_campaign_list_req(&self, ccy: String, location_code: String, inst_name: String, biz_type: String, cache_expire_seconds: u64) -> KlineCampaignListReq { todo!() }
    /// Creates a new `CampaignListReq` with default location_code.
    /// 
    /// # Parameters
    /// - `ccy`: Currency symbol (e.g., "BTC", "ETH")
    /// - `inst_name`: Instrument name
    /// - `biz_type`: Business type
    /// - `cache_expire_seconds`: Cache expiration time in seconds (0 = no cache)
    /// 
    /// # Returns
    /// A new `CampaignListReq` instance with location_code set to "over_view"
    pub fn create_campaign_list_req_with_defaults(&self, ccy: String, inst_name: String, biz_type: String, cache_expire_seconds: u64) -> KlineCampaignListReq { todo!() }
    /// Creates a new `CoinDataInfoReq` with the specified parameters.
    /// 
    /// # Parameters
    /// - `project_name`: Project name (e.g., "BTC", "ETH")
    /// - `cache_expire_seconds`: Cache expiration time in seconds (0 = no cache)
    /// 
    /// # Returns
    /// A new `CoinDataInfoReq` instance
    pub fn create_coin_data_info_req(&self, project_name: String, cache_expire_seconds: u64) -> KlineCoinDataInfoReq { todo!() }
    /// Creates a new `CoinDetailReq` with the specified parameters.
    /// 
    /// # Parameters
    /// - `project_name`: Project name (e.g., "BTC", "ETH")
    /// - `cache_expire_seconds`: Cache expiration time in seconds (0 = no cache)
    /// 
    /// # Returns
    /// A new `CoinDetailReq` instance
    pub fn create_coin_detail_req(&self, project_name: String, cache_expire_seconds: u64) -> KlineCoinDetailReq { todo!() }
    /// Get base coin info data
    /// 
    /// Fetches comprehensive cryptocurrency information including market ranking,
    /// historical prices, supply info, token allocation, social media links, etc.
    /// 
    /// URL: /priapi/v5/rubik/public/coin-info
    /// 
    /// # Parameters
    /// - `req`: Base coin info request with currency symbol and cache settings
    /// 
    /// # Returns
    /// A cancellable future that resolves to `BaseCoinInfoResult`
    pub fn get_base_coin_info(&self, req: KlineBaseCoinInfoReq) -> Arc<OKRustFutureBaseCoinInfoResultWithOKFFIError> { todo!() }
    /// Get campaign list
    /// 
    /// Fetches campaign list including active promotions and events.
    /// 
    /// URL: /priapi/v5/rubik/public/active-list
    /// 
    /// # Parameters
    /// - `req`: Campaign list request with parameters and cache settings
    /// 
    /// # Returns
    /// A cancellable future that resolves to `CampaignListResult`
    pub fn get_campaign_list(&self, req: KlineCampaignListReq) -> Arc<OKRustFutureCampaignListResultWithOKFFIError> { todo!() }
    /// Get coin data info
    /// 
    /// Fetches market data information including circulating supply, market cap,
    /// trading volume, issue price, etc.
    /// 
    /// URL: /v2/support/info/announce/coinDataInfo
    /// 
    /// # Parameters
    /// - `req`: Coin data info request with project name and cache settings
    /// 
    /// # Returns
    /// A cancellable future that resolves to `CoinDataInfoResult`
    pub fn get_coin_data_info(&self, req: KlineCoinDataInfoReq) -> Arc<OKRustFutureCoinDataInfoResultWithOKFFIError> { todo!() }
    /// Get coin detail data
    /// 
    /// Fetches detailed cryptocurrency project information including team members,
    /// social media links, issue info, etc.
    /// 
    /// URL: /v2/support/info/announce/detail
    /// 
    /// # Parameters
    /// - `req`: Coin detail request with project name and cache settings
    /// 
    /// # Returns
    /// A cancellable future that resolves to `CoinDetailResult`
    pub fn get_coin_detail(&self, req: KlineCoinDetailReq) -> Arc<OKRustFutureCoinDetailResultWithOKFFIError> { todo!() }
}

pub struct KlineDepthInteractor;
impl KlineDepthInteractor {
    pub fn on_clear(&self) { todo!() }
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
    /// Enable or disable debug logging.
    /// 
    /// In release builds this is a no-op, but the symbol must exist
    /// so that UniFFI bindings generated in debug mode continue to
    /// link successfully against the release library.
    pub fn set_debug(&self, _enabled: bool) { todo!() }
    /// Switch depth ladder/grouping. Re-subscribes WS.
    pub fn set_ladder(&self, ladder: String) { todo!() }
    /// Set a state observer (UniFFI export).
    /// 
    /// DEPRECATED: callback-based observer. Use `state_stream()` —
    /// the callback observer pins the foreign object in UniFFI's
    /// static handleMap and produces a cross-language retain cycle
    /// (Android GC + iOS ARC).
    pub fn set_state_observer(&self, observer: Arc<dyn InteractorStateChangeObserver>) { todo!() }
    /// Switch trade unit display: "0"=coin, "1"=contract, "2"=USDT.
    pub fn set_trade_unit(&self, trade_unit: String) { todo!() }
    /// Start the interactor lifecycle.
    /// 
    /// Calls the user's `InteractorLifecycle::start()` implementation.
    /// Override that method to add custom startup behavior (e.g., initial data loads).
    pub fn start(&self) { todo!() }
    /// Get the current state (UniFFI export).
    /// 
    /// Returns the External type (UniFFI-compatible).
    pub fn state(&self) -> KlineDepthInteractorState { todo!() }
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
    /// Subscribe with optional preferred ladder, a per-page levels cap and surface.
    /// `preferred_ladder`: selected ladder; empty means Rust resolves by surface.
    /// `max_depth_levels`: 50 for order-book table, all available levels for the depth chart.
    pub fn subscribe(&self, inst_id: String, inst_type: String, preferred_ladder: String, max_depth_levels: u32, surface: KlineDepthSurface) { todo!() }
}

pub struct KlineEarningsEventInteractor;
impl KlineEarningsEventInteractor {
    /// Fetch the earnings events for `req`'s instrument.
    /// 
    /// Takes the whole record rather than positional arguments: `inst_id` and
    /// `inst_type` are adjacent strings, which positional arguments make easy to
    /// transpose silently. The window and the UTC offset are not native's to
    /// pass — the use case resolves both.
    pub fn fetch(&self, req: KlineEarningsEventReq) { todo!() }
    pub fn on_clear(&self) { todo!() }
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
    pub fn state(&self) -> KlineEarningsEventState { todo!() }
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
}

pub struct KlineEcoCalendarInteractor;
impl KlineEcoCalendarInteractor {
    /// Fetch events covering `[before_ms, after_ms]` (Unix-ms). `utc_zone`
    /// is the user's offset (e.g. "0" / "8"). `cache_expire_secs = 0`
    /// bypasses the HTTP cache; pass `> 0` to reuse the cached response.
    pub fn fetch(&self, before_ms: i64, after_ms: i64, utc_zone: String, cache_expire_secs: u64) { todo!() }
    pub fn on_clear(&self) { todo!() }
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
    pub fn state(&self) -> KlineEcoCalendarState { todo!() }
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
}

pub struct KlineFavoriteInteractor;
impl KlineFavoriteInteractor {
    pub fn on_clear(&self) { todo!() }
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
    /// DEPRECATED. Use `ui_event_stream()`; drop the returned stream
    /// wrapper to unsubscribe.
    pub fn remove_ui_event_observer(&self) { todo!() }
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
    /// DEPRECATED: callback-based observer. Use `ui_event_stream()` —
    /// the callback observer pins the foreign object in UniFFI's
    /// static handleMap and produces a cross-language retain cycle
    /// (Android GC + iOS ARC).
    pub fn set_ui_event_observer(&self, observer: Arc<dyn KlineFavoriteUxEventObserver>) { todo!() }
    /// Start the interactor lifecycle.
    /// 
    /// Calls the user's `InteractorLifecycle::start()` implementation.
    /// Override that method to add custom startup behavior (e.g., initial data loads).
    pub fn start(&self) { todo!() }
    /// Get the current state (UniFFI export).
    /// 
    /// Returns the External type (UniFFI-compatible).
    pub fn state(&self) -> KlineFavoriteState { todo!() }
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
    /// Bind to a CeFi instrument (SPOT / MARGIN / SWAP / FUTURES / OPTION /
    /// PRE_MARKET) keyed by `(inst_id, inst_type)`. The concrete `FavoriteItem`
    /// variant is inferred from `inst_type`; MARGIN folds into SPOT.
    /// Re-entering (either subscribe) cancels the previous subscription.
    pub fn subscribe_cefi(&self, inst_id: String, inst_type: String) { todo!() }
    /// Bind to a DEX token keyed by `(chain_id, ca)` (token contract address).
    /// `token_symbol` is required by the favorite `setting` API on add (the
    /// server rejects a DEX add without it); pass the page's current token
    /// symbol. Re-entering (either subscribe) cancels the previous subscription.
    pub fn subscribe_dex(&self, chain_id: String, ca: String, token_symbol: String) { todo!() }
    /// Bind to an ExchangeOS DEX instrument keyed by `asset_id` ALONE. A perp and a spot
    /// instrument of the same coin carry DIFFERENT `asset_id`s, so `is_perp` never
    /// participates in identity — it only picks the `dexSubType` the write endpoints expect
    /// (`PERP` / `SPOT`). `name` is the display name echoed back as `name` on add; pass an
    /// empty string to let the server backfill it from `asset_id`.
    /// Re-entering (any subscribe) cancels the previous subscription.
    pub fn subscribe_exos(&self, asset_id: String, is_perp: bool, name: String) { todo!() }
    /// Toggle the favorite status of the most recently subscribed item.
    /// No-op if no subscribe was called.
    pub fn toggle(&self) { todo!() }
    /// Opt-in async UI event stream (cycle-free). Single-subscriber.
    pub fn ui_event_stream(&self) -> Arc<KlineFavoriteUxEventStream> { todo!() }
}

/// Deprecated UniFFI callback interface for observing UI events. Prefer the cycle-free `KlineFavoriteUxEventStream` returned by `interactor.ui_event_stream()`.
pub struct KlineFavoriteUxEventObserver;
impl KlineFavoriteUxEventObserver {
    pub fn on_ui_event(&self, event: KlineFavoriteUxEvent) { todo!() }
}

/// Opt-in async event stream from `interactor.ui_event_stream()`.
/// Single-subscriber, latest-wins. `next()` resolves to `None` when the
/// interactor is dropped or a new subscription replaces this one.
pub struct KlineFavoriteUxEventStream;
impl KlineFavoriteUxEventStream {
    pub async fn next(&self) -> Option<KlineFavoriteUxEvent> { todo!() }
}

pub struct KlineFlashNewsInteractor;
impl KlineFlashNewsInteractor {
    pub fn on_clear(&self) { todo!() }
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
    pub fn state(&self) -> KlineFlashNewsState { todo!() }
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
    pub fn subscribe(&self, inst_id: String, inst_type: String) { todo!() }
}

/// UniFFI-exported K-line number formatter.
/// 
/// # Construction
/// 
/// ```ignore
/// // Pulls all biz references from CTRSTrade global singleton.
/// // This is the default for pipeline + native callers.
/// let formatter = KlineFormatter::from_trading_core();
/// ```
/// 
/// # Biz routing
/// 
/// Every method that takes `inst_type` routes precision/instrument
/// lookups to the matching biz line. Methods that do not take
/// `inst_type` (`format_percentage`, `format_compact`,
/// `format_spot_pnl_value`, `position_unit_for`) operate on the input
/// value alone and need no biz.
pub struct KlineFormatter;
impl KlineFormatter {
    /// Pull biz references from the `CTRSTrade` singleton. Captures all
    /// five biz lines so any K-line instrument is covered.
    pub fn from_trading_core() -> Arc<KlineFormatter> { todo!() }
    /// Balance (cash / spot / avail / frozen) at amount precision, truncated
    /// toward zero, trailing zeros kept. Mirrors V1 `formatCashBal`
    /// (`setPrecision(amountDigit).setKeepZero(true)`).
    pub fn format_balance(&self, value: String, inst_id: String, inst_type: String) -> String { todo!() }
    /// Single-value buy/sell marker price conversion (V1 `convertHistoryOrderData`
    /// + `OKTChargeConvertor.toQuoteNumber`). For a batch, build a
    /// [`Self::buy_sell_price_scaler`] once and call [`BuySellPriceScaler::apply`]
    /// per point — this avoids the per-point instrument / rate lookup.
    pub fn format_buy_sell_price(&self, value: String, inst_id: String, inst_type: String, price_mode: KLinePriceMode) -> String { todo!() }
    /// Returns the scaled numeric body + a [`KlineCompactUnit`] tag.
    /// Native renders the localized suffix string from the enum.
    /// trade_overlay's existing behaviour was HALF_UP — preserved here.
    pub fn format_compact(&self, value: String) -> CompactValue { todo!() }
    /// Fiat valuation ("≈$" row) Recipe: USD value -> current fiat (× rate, USD -> ×1), precision
    /// always uses V1 `recommendPrecision` (bucketed by fiat value magnitude, independent of quote
    /// currency precision), HALF_UP + keep trailing zeros + fiat symbol prefix. Aligns with V1
    /// `createUSDToLocalConvertor().setRecommendPrecision().prefix()`.
    /// Difference from [`format_local_fiat`]: the latter's USD branch uses quote currency precision
    /// (reused by inverse main price / high-low).
    pub fn format_fiat_valuation(&self, value: String, _inst_id: String, _inst_type: String) -> KlineFormatRecipe { todo!() }
    /// Spot index price Recipe (option header "spot index" row): precision via recommend
    /// (larger value -> fewer digits), DOWN, no sign, keep trailing zeros, thousands by native.
    pub fn format_index_price_recipe(&self, value: String) -> KlineFormatRecipe { todo!() }
    /// USD-quoted price → local fiat Recipe with auto-precision.
    /// USD currency short-circuits to biz quote precision (no rate multiplication),
    /// sign = fiat symbol prefix. Premium currencies prefer
    /// `usd_to_this_rate_premium` over `exchange_rate`.
    pub fn format_local_fiat(&self, value: String, inst_id: String, inst_type: String) -> KlineFormatRecipe { todo!() }
    /// Open-interest Recipe (already multiplied by contract multiplier upstream).
    /// Natural precision, DOWN, trailing zeros trimmed, grouped by native.
    /// Aligns with V1 `tradeConvertor.toTargetNumber(posAmount).formatLocalized()`.
    pub fn format_open_interest_recipe(&self, value: String) -> KlineFormatRecipe { todo!() }
    /// Main price / high-low for option index mode: aligns with V1 `createIndexPriceConverterWithIndex`
    /// (= `OkOptionBiz::create_index_price_convertor`) — value = index price (USD, identity),
    /// precision + sign taken from the **index quote currency** (coin-margined option = USD -> "$"),
    /// not the option premium symbol (₿).
    /// `with_sign=false` is for high-low (V1 high/low does not call `.prefix()`).
    pub fn format_option_index_price_recipe(&self, value: String, inst_id: String, inst_type: String, with_sign: bool) -> KlineFormatRecipe { todo!() }
    /// Tooltip order notional, aligned with V1 `getNotionalUsd()`: convert USD
    /// to the current fiat, keep 2 decimals, round down, group thousands, and
    /// prefix the fiat sign. Empty/zero renders `--`.
    pub fn format_order_notional_usd(&self, value: String) -> String { todo!() }
    /// Order limit / trigger / TP / SL price. Buy rounds DOWN, sell rounds UP.
    pub fn format_order_price(&self, value: String, inst_id: String, inst_type: String, side: KLineOrderSide) -> String { todo!() }
    /// Ratio (`"0.0082"`) → signed percentage (`"+0.82%"`).
    /// Scale ×100, 2 decimals truncated toward zero, non-zero values prepend sign.
    pub fn format_percentage(&self, ratio: String) -> String { todo!() }
    /// PnL value at the instrument's `profit_dig` precision, truncated toward zero.
    pub fn format_pnl_value(&self, value: String, inst_id: String, inst_type: String) -> String { todo!() }
    /// Position cost-line quantity, aligned with V1
    /// `KlinePositionsUnitUtils.fmtAmt` / `fmtMarginPos`: rendered in the user's
    /// selected trade unit (contracts / coin / quote) via the instrument's
    /// unit-aware quantity formatter — NOT a hardcoded notional. `price` (live
    /// last, falling back to avg) drives the unit conversions that need it (coin
    /// display, inverse contracts); for the contracts unit it is ignored, so 5
    /// contracts render as "5", matching V1.
    pub fn format_position_quantity(&self, pos: String, inst_id: String, inst_type: String, price: Option<String>) -> String { todo!() }
    /// Price at biz precision, rounded HALF_UP (position avg / liq / break-even /
    /// chart label). Pass-through when biz / instrument is unavailable.
    pub fn format_price(&self, value: String, inst_id: String, inst_type: String) -> String { todo!() }
    /// Server price → chart Y-axis quote-coin units at biz precision.
    /// Includes inverse SWAP/FUTURES + OPTION USD→quote-coin remap via the
    /// biz charge convertor; raw numeric out, native adds thousands.
    pub fn format_quote_price(&self, value: String, inst_id: String, inst_type: String) -> String { todo!() }
    /// Signed 2-decimal percentage Recipe. `is_percentage = true` — native
    /// appends `%`. Caller sets `prepend_plus` if needed.
    pub fn format_signed_percentage(&self, value: String) -> KlineFormatRecipe { todo!() }
    /// Signed numeric at biz precision, DOWN rounded. Primitive Recipe — sign
    /// / prepend_plus left untouched; caller decorates per field.
    pub fn format_signed_price(&self, value: String, inst_id: String, inst_type: String) -> KlineFormatRecipe { todo!() }
    /// Like [`format_signed_price`](Self::format_signed_price) but HALF_UP rounded.
    pub fn format_signed_price_rounded(&self, value: String, inst_id: String, inst_type: String) -> KlineFormatRecipe { todo!() }
    /// Spot cost-line price. V1 (`formatCostPriceUSDValueToQuotePrice` =
    /// `value.divS(lastPrice, max = priceDigit)`) TRUNCATES (DOWN) to price
    /// precision, unlike the HALF_UP `format_price` — so a cost of 0.093255
    /// shows 0.09325, not 0.09326. Pass-through when biz is unavailable.
    pub fn format_spot_cost_price(&self, value: String, inst_id: String, inst_type: String) -> String { todo!() }
    /// Spot average-line PnL value. V1 (`formatCostPriceUSDValueToQuotePrice`)
    /// renders it at the instrument's PRICE precision (e.g. 5 dp for DOGE), and
    /// the chart adds the sign + thousands separator via `formatLocalized`. So
    /// emit a price-precision number with NO sign prefix here — a `+` would
    /// double up against the chart's own sign logic.
    pub fn format_spot_pnl_value(&self, value: String, inst_id: String, inst_type: String) -> String { todo!() }
    pub fn format_trailing_callback(&self, callback_ratio: String, callback_spread: String, inst_id: String, inst_type: String, side: KLineOrderSide) -> String { todo!() }
    pub fn format_trailing_callback_spread(&self, callback_spread: String, inst_id: String, inst_type: String, side: KLineOrderSide) -> String { todo!() }
    /// 24h turnover Recipe (quote ccy amount) — 2-decimal, HALF_UP, abbreviate.
    /// Aligns with V1 `fmtKMBnWithLanguageWithoutSign(2, false, HALF_UP)`.
    pub fn format_turnover_recipe(&self, value: String) -> KlineFormatRecipe { todo!() }
    /// 24h volume Recipe (base ccy amount) — 2-decimal, DOWN, abbreviate.
    /// Aligns with V1 `Range.fmtVolumeInCN / fmtCjlVolume`.
    pub fn format_volume_recipe(&self, value: String) -> KlineFormatRecipe { todo!() }
    /// Base currency parsed from `inst_id` ("BTC-USDT" → "BTC").
    pub fn position_unit_for(&self, inst_id: String) -> String { todo!() }
    /// Biz price precision digit count for `inst_id` / `inst_type`.
    /// Consumed by chart library's `setPriceFractionDigits` short-circuit.
    pub fn price_fraction_digits(&self, inst_id: String, inst_type: String) -> u8 { todo!() }
}

pub struct KlineHeaderInteractor;
impl KlineHeaderInteractor {
    /// Switch the visible price source: "kline" (latest), "index", or "mark".
    /// Reuses cached values from each subscribed stream — does not re-subscribe.
    pub fn change_price_type(&self, price_type: String) { todo!() }
    pub fn on_clear(&self) { todo!() }
    /// Re-run instrument-detail fetch for the currently subscribed inst, then
    /// republish the header (drives implied-valuation refresh on rebase events).
    /// No-op when inst_id is empty (no active subscription). Does not restart
    /// WS streams. Mirrors V1 `MarketDetailPresent.refreshImpliedValuation`.
    pub fn refresh_instrument_detail(&self) { todo!() }
    /// Refetch the daily candles and recompute. Wired to RxBus timezone /
    /// adjustment changes on native side.
    pub fn refresh_multi_period(&self) { todo!() }
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
    /// Replace the daily-bar context without refetching. The stock adjustment
    /// param is re-resolved internally from the current instrument.
    pub fn set_daily_candle_context(&self, daily_bar: String) { todo!() }
    /// Direct push of the 200 daily candles. Fallback / test seam.
    pub fn set_daily_candles(&self, raw_candles: Vec<Vec<String>>) { todo!() }
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
    pub fn state(&self) -> KlineHeaderState { todo!() }
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
    /// `daily_bar` is the multi-period candle bar (e.g. "1D" / "1Dutc"),
    /// resolved native-side from timezone settings. The stock-style adjustment
    /// query param is resolved internally (Rust SSOT, same as the candles
    /// interactor) — never passed from native.
    /// `price_type` is the initial visible source ("kline" / "mark" / "index");
    /// native passes the per-instType default (options default to "mark"), so the
    /// header starts in the right mode without a follow-up `change_price_type`.
    pub fn subscribe(&self, inst_id: String, inst_type: String, daily_bar: String, price_type: String) { todo!() }
    /// Push the latest ticker price. Fallback / test seam.
    pub fn update_current_price(&self, price: String) { todo!() }
}

pub struct KlineInitInteractor;
impl KlineInitInteractor {
    /// Canonical Company decision for the latest published init state.
    /// Kept outside `KlineInitState` so the established state record remains ABI-stable.
    pub fn company_entry_policy(&self) -> CompanyEntryPolicy { todo!() }
    /// Prepare trade core + the biz line for `biz_type`. Cancels any
    /// in-flight ensure on re-entry (AlwaysCancel). Caller should pass the
    /// instrument's biz type string (SPOT / MARGIN / SWAP / FUTURES /
    /// OPTION); unknown strings surface as a state error.
    pub fn ensure(&self, inst_id: String, biz_type: String) { todo!() }
    pub fn on_clear(&self) { todo!() }
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
    pub fn state(&self) -> KlineInitState { todo!() }
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
}

pub struct KlineMcapV2Interactor;
impl KlineMcapV2Interactor {
    pub async fn load_future(&self, subscription_id: u32, req: MarketCapV2Req, cancel_token: Arc<OKCancellationToken>) -> Result<(), OKFFIError> { todo!() }
    pub fn on_clear(&self) { todo!() }
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
    pub fn state(&self) -> KlineMcapV2State { todo!() }
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
}

/// New-coin interactor.
pub struct KlineNewCoinInteractor;
impl KlineNewCoinInteractor {
    /// Initialize / re-init for an instrument. `mode` is decided by the host from
    /// the listing list. The whitelist flag (depth) and header visibility are
    /// derived inside Rust — no native judgement needed.
    pub fn initialize(&self, inst_id: String, inst_type: String, mode: KlineNewCoinMode) { todo!() }
    /// Native forwards the auction / pre-quote banner's countdown-reached-zero UI event.
    /// The displayed countdown ticks from the real server end time, so reaching zero means
    /// the pre-listing window elapsed — finish the flow. Mirrors V1 `checkAuctionState` /
    /// `checkPreQuoteState` hiding on `serverTs >= endTime`; the decision stays in Rust.
    pub fn on_banner_countdown_finished(&self) { todo!() }
    /// Called when the page is destroyed.
    pub fn on_clear(&self) { todo!() }
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
    /// DEPRECATED. Use `ui_event_stream()`; drop the returned stream
    /// wrapper to unsubscribe.
    pub fn remove_ui_event_observer(&self) { todo!() }
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
    /// DEPRECATED: callback-based observer. Use `ui_event_stream()` —
    /// the callback observer pins the foreign object in UniFFI's
    /// static handleMap and produces a cross-language retain cycle
    /// (Android GC + iOS ARC).
    pub fn set_ui_event_observer(&self, observer: Arc<dyn KlineNewCoinUiEventObserver>) { todo!() }
    /// Start the interactor lifecycle.
    /// 
    /// Calls the user's `InteractorLifecycle::start()` implementation.
    /// Override that method to add custom startup behavior (e.g., initial data loads).
    pub fn start(&self) { todo!() }
    /// Get the current state (UniFFI export).
    /// 
    /// Returns the External type (UniFFI-compatible).
    pub fn state(&self) -> KlineNewCoinState { todo!() }
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
    /// Opt-in async UI event stream (cycle-free). Single-subscriber.
    pub fn ui_event_stream(&self) -> Arc<KlineNewCoinUiEventStream> { todo!() }
}

pub struct KlineNewCoinIntroInteractor;
impl KlineNewCoinIntroInteractor {
    pub fn on_clear(&self) { todo!() }
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
    pub fn state(&self) -> KlineNewCoinIntroState { todo!() }
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
    pub fn subscribe(&self, inst_id: String, inst_type: String) { todo!() }
}

/// Deprecated UniFFI callback interface for observing UI events. Prefer the cycle-free `KlineNewCoinUiEventStream` returned by `interactor.ui_event_stream()`.
pub struct KlineNewCoinUiEventObserver;
impl KlineNewCoinUiEventObserver {
    pub fn on_ui_event(&self, event: KlineNewCoinUiEvent) { todo!() }
}

/// Opt-in async event stream from `interactor.ui_event_stream()`.
/// Single-subscriber, latest-wins. `next()` resolves to `None` when the
/// interactor is dropped or a new subscription replaces this one.
pub struct KlineNewCoinUiEventStream;
impl KlineNewCoinUiEventStream {
    pub async fn next(&self) -> Option<KlineNewCoinUiEvent> { todo!() }
}

pub struct KlineOndoStockHoursInteractor;
impl KlineOndoStockHoursInteractor {
    pub fn on_clear(&self) { todo!() }
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
    pub fn state(&self) -> KlineOndoStockHoursState { todo!() }
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
    /// Subscribe the CeDeFi/Ondo per-token status keyed by `(chain_id, ca)`.
    pub fn subscribe(&self, chain_id: String, ca: String) { todo!() }
}

pub struct KlineSwipingTitleInteractor;
impl KlineSwipingTitleInteractor {
    pub fn on_clear(&self) { todo!() }
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
    pub fn state(&self) -> KlineSwipingTitleState { todo!() }
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
    /// Build the swiping-title list for `(inst_id, inst_type)`.
    /// 
    /// When `use_favorites` is true and the page is not OPTION, the watchlist
    /// is consulted first; falls back to market-sort if fewer than 2 entries.
    pub fn subscribe(&self, inst_id: String, inst_type: String, use_favorites: bool) { todo!() }
}

/// TokenUnlockService - Unified service for token unlock
/// 
/// This service aggregates all token unlock related APIs into a single object,
/// providing a cleaner API for mobile developers.
/// 
/// # Available Methods
/// 
/// - `get_token_unlock_events`: Fetch token unlock events (unlock schedule, amounts, etc.)
/// - `get_unlock_progress`: Fetch token unlock progress (unlocked/locked/untracked percentages)
/// - `get_unlock_allocation`: Fetch token unlock allocation (pie chart data)
/// - `get_unlock_schedule`: Fetch token unlock schedule (timeline of unlock events)
/// 
/// # Example (Swift)
/// 
/// ```swift
/// let service = TokenUnlockService()
/// let req = service.createTokenUnlockEventsReq(
/// ccy: "ARB",
/// eventType: 1,
/// cacheExpireSeconds: 300
/// )
/// let result = try await service.getTokenUnlockEvents(req: req)
/// ```
pub struct KlineTokenUnlockService;
impl KlineTokenUnlockService {
    /// Creates a new KlineTokenUnlockService instance.
    pub fn new() -> Arc<KlineTokenUnlockService> { todo!() }
    /// Creates a new `TokenUnlockEventsReq` with the specified parameters.
    /// 
    /// # Parameters
    /// - `ccy`: Currency symbol (e.g., "ARB", "OP")
    /// - `event_type`: Event type filter
    /// - `cache_expire_seconds`: Cache expiration time in seconds (0 = no cache)
    /// 
    /// # Returns
    /// A new `TokenUnlockEventsReq` instance
    pub fn create_token_unlock_events_req(&self, ccy: String, event_type: u64, cache_expire_seconds: u64) -> KlineTokenUnlockEventsReq { todo!() }
    /// Creates a new `UnlockAllocationReq` with the specified parameters.
    /// 
    /// # Parameters
    /// - `ccy`: Currency symbol (e.g., "ARB", "OP")
    /// - `cache_expire_seconds`: Cache expiration time in seconds (0 = no cache)
    /// 
    /// # Returns
    /// A new `UnlockAllocationReq` instance
    pub fn create_unlock_allocation_req(&self, ccy: String, cache_expire_seconds: u64) -> KlineUnlockAllocationReq { todo!() }
    /// Creates a new `UnlockProgressReq` with the specified parameters.
    /// 
    /// # Parameters
    /// - `ccy`: Currency symbol (e.g., "ARB", "OP")
    /// - `cache_expire_seconds`: Cache expiration time in seconds (0 = no cache)
    /// 
    /// # Returns
    /// A new `UnlockProgressReq` instance
    pub fn create_unlock_progress_req(&self, ccy: String, cache_expire_seconds: u64) -> KlineUnlockProgressReq { todo!() }
    /// Creates a new `UnlockScheduleReq` with the specified parameters.
    /// 
    /// # Parameters
    /// - `ccy`: Currency symbol (e.g., "ARB", "OP")
    /// - `period`: Time period filter ("year" or "all")
    /// - `cache_expire_seconds`: Cache expiration time in seconds (0 = no cache)
    /// 
    /// # Returns
    /// A new `UnlockScheduleReq` instance
    pub fn create_unlock_schedule_req(&self, ccy: String, period: String, cache_expire_seconds: u64) -> KlineUnlockScheduleReq { todo!() }
    /// Get token unlock events
    /// 
    /// Fetches token unlock events including unlock schedule, amounts,
    /// percentages, and allocation labels.
    /// 
    /// URL: /priapi/v5/rubik/token-unlock/events
    /// 
    /// # Parameters
    /// - `req`: Token unlock events request with currency symbol, type, and cache settings
    /// 
    /// # Returns
    /// A cancellable future that resolves to `TokenUnlockEventsResult`
    pub fn get_token_unlock_events(&self, req: KlineTokenUnlockEventsReq) -> Arc<OKRustFutureTokenUnlockEventsResultWithOKFFIError> { todo!() }
    /// Get token unlock allocation
    /// 
    /// Fetches token unlock allocation pie chart data including portions
    /// for different allocation types (Team, Investors, etc.).
    /// 
    /// URL: /priapi/v5/rubik/token-unlock/allocation
    /// 
    /// # Parameters
    /// - `req`: Unlock allocation request with currency symbol and cache settings
    /// 
    /// # Returns
    /// A cancellable future that resolves to `UnlockAllocationResult`
    pub fn get_unlock_allocation(&self, req: KlineUnlockAllocationReq) -> Arc<OKRustFutureUnlockAllocationResultWithOKFFIError> { todo!() }
    /// Get token unlock progress
    /// 
    /// Fetches token unlock progress information including maximum supply,
    /// market cap, unlocked/locked/untracked amounts and percentages.
    /// 
    /// URL: /priapi/v5/rubik/token-unlock/progress
    /// 
    /// # Parameters
    /// - `req`: Unlock progress request with currency symbol and cache settings
    /// 
    /// # Returns
    /// A cancellable future that resolves to `UnlockProgressResult`
    pub fn get_unlock_progress(&self, req: KlineUnlockProgressReq) -> Arc<OKRustFutureUnlockProgressResultWithOKFFIError> { todo!() }
    /// Get token unlock schedule
    /// 
    /// Fetches token unlock schedule showing timeline of unlock events
    /// with cumulative unlocked amounts and values.
    /// 
    /// URL: /priapi/v5/rubik/token-unlock/schedule
    /// 
    /// # Parameters
    /// - `req`: Unlock schedule request with currency symbol and cache settings
    /// 
    /// # Returns
    /// A cancellable future that resolves to `UnlockScheduleResult`
    pub fn get_unlock_schedule(&self, req: KlineUnlockScheduleReq) -> Arc<OKRustFutureUnlockScheduleResultWithOKFFIError> { todo!() }
}

pub struct KlineTradeOverlayInteractor;
impl KlineTradeOverlayInteractor {
    /// Open a drag session for a chart line. `target_kind` is one of:
    /// `"order_main"`, `"order_tp"`, `"order_sl"`, `"position_tp"`,
    /// `"position_sl"`, `"spot_tp"`, `"spot_sl"`. Sets `is_drag_paused`
    /// to true; upstream callbacks stop touching state until the matching
    /// `end_drag` call.
    pub fn begin_drag(&self, target_id: String, target_kind: String) { todo!() }
    /// **Deprecated** — use [`KlineTradeOverlayInteractor::compute_drag_pnl`]
    /// (pure-function entry on the impl block). Retained as a no-op to
    /// preserve the FFI ABI for older Native bindings.
    pub fn calculate_hypothetical_pnl(&self, target_id: String, hypothetical_price: String) { todo!() }
    /// Pure compute — reads the active drag session snapshot and returns
    /// the hypothetical PnL formatted as display strings.
    /// 
    /// Does **not** mutate state. Does **not** trigger the state observer.
    /// Safe to call from the chart-drag listener on every frame (60Hz) —
    /// the call only takes a read lock on `self.inner` and runs Decimal
    /// arithmetic; no FFI roundtrip via the state-update channel.
    pub fn compute_drag_pnl(&self, current_price: String) -> KlineDragPnlVo { todo!() }
    /// Convert a chart/display price back to the order-base price used by
    /// amend-order APIs.
    /// 
    /// Main-order drags do not create a Rust PnL session, so callers cannot
    /// rely on `compute_drag_pnl` to provide `u_price` for that path. Keep the
    /// inverse conversion here so Native uses the same scaler as the overlay
    /// pipeline and the TP/SL drag session.
    pub fn display_price_to_order_base(&self, inst_id: String, inst_type: String, display_price: String) -> String { todo!() }
    /// Close the drag session. When `committed` is true the dialog has
    /// already submitted the order, so the pipeline re-renders from the latest
    /// cached upstream snapshot. While drag is paused, callbacks keep raw
    /// orders / positions fresh but leave `view_state` frozen. When false the
    /// chart re-renders from the same cached snapshot to restore the old line.
    pub fn end_drag(&self, committed: bool) { todo!() }
    /// Tell the interactor that a newly submitted TP / SL order should be
    /// pre-attached to the given target on its first appearance.
    /// `target_kind` carries V1's `orderStatusBean.isCost` semantics:
    /// `"position"` → attach to a position id; `"order"` → attach to a
    /// main order id. Any other value is ignored.
    pub fn mark_new_order_linkage(&self, ord_id: String, target_id: String, target_kind: String) { todo!() }
    pub fn on_clear(&self) { todo!() }
    pub fn pause_refresh_during_drag(&self) { todo!() }
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
    pub fn resume_refresh(&self) { todo!() }
    /// Enable or disable debug logging.
    /// 
    /// In release builds this is a no-op, but the symbol must exist
    /// so that UniFFI bindings generated in debug mode continue to
    /// link successfully against the release library.
    pub fn set_debug(&self, _enabled: bool) { todo!() }
    /// Switch the active price source. Non-`Default` stops the manager;
    /// returning to `Default` restarts it.
    pub fn set_price_mode(&self, mode: KLinePriceMode) { todo!() }
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
    pub fn state(&self) -> KlineTradeOverlayState { todo!() }
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
    pub fn subscribe(&self, inst_id: String, inst_type: String) { todo!() }
    /// Update the dragged price every chart-move frame. Recomputes
    /// `pending_pnl` immediately so the chart can show the hypothetical
    /// PnL number next to the line.
    pub fn update_drag_price(&self, new_price: String) { todo!() }
}

/// TradingRulesService - Unified service for trading rules
/// 
/// This service aggregates all trading rules related APIs into a single object,
/// providing a cleaner API for mobile developers.
/// 
/// # Available Methods
/// 
/// - `get_lever_interval`: Fetch leverage interval configuration
/// - `get_mark_price`: Fetch mark price information
/// - `get_index_tickers`: Fetch index tickers information
/// - `get_batch_tickers`: Fetch batch tickers information
/// - `get_opening_info`: Fetch opening/listing info
/// 
/// # Example (Swift)
/// 
/// ```swift
/// let service = TradingRulesService()
/// 
/// // Opening info
/// let openingReq = service.createOpeningInfoReq(
/// instId: "BTC-USDT",
/// cacheExpireSeconds: 300
/// )
/// let openingResult = try await service.getOpeningInfo(req: openingReq)
/// ```
pub struct KlineTradingRulesService;
impl KlineTradingRulesService {
    /// Creates a new KlineTradingRulesService instance.
    pub fn new() -> Arc<KlineTradingRulesService> { todo!() }
    /// Creates a new `BatchTickersReq` with the specified parameters.
    /// 
    /// # Parameters
    /// - `inst_ids`: Instrument IDs (e.g., ["BTC-USDT", "ETH-USDT"])
    /// - `cache_expire_seconds`: Cache expiration time in seconds (0 = no cache)
    /// 
    /// # Returns
    /// A new `BatchTickersReq` instance
    pub fn create_batch_tickers_req(&self, inst_ids: Vec<String>, cache_expire_seconds: u64) -> KlineBatchTickersReq { todo!() }
    /// Creates a new `IndexTickersReq` with the specified parameters.
    /// 
    /// # Parameters
    /// - `quote_ccy`: Quote currency (e.g., "USD", "USDT")
    /// - `inst_id`: Instrument ID (e.g., "BTC-USD")
    /// - `cache_expire_seconds`: Cache expiration time in seconds (0 = no cache)
    /// 
    /// # Returns
    /// A new `IndexTickersReq` instance
    pub fn create_index_tickers_req(&self, quote_ccy: String, inst_id: String, cache_expire_seconds: u64) -> KlineIndexTickersReq { todo!() }
    /// Creates a new `LeverIntervalReq` with the specified parameters.
    /// 
    /// # Parameters
    /// - `inst_type`: Instrument type (e.g., "SWAP", "FUTURES", "MARGIN")
    /// - `uly`: Underlying asset
    /// - `inst_family`: Instrument family
    /// - `inst_id`: Instrument ID (e.g., "BTC-USDT-SWAP")
    /// - `ccy`: Currency
    /// - `mgn_mode`: Margin mode (e.g., "cross", "isolated")
    /// - `cache_expire_seconds`: Cache expiration time in seconds (0 = no cache)
    /// 
    /// # Returns
    /// A new `LeverIntervalReq` instance
    pub fn create_lever_interval_req(&self, inst_type: String, uly: String, inst_family: String, inst_id: String, ccy: String, mgn_mode: String, cache_expire_seconds: u64) -> KlineLeverIntervalReq { todo!() }
    /// Creates a new `MarkPriceReq` with the specified parameters.
    /// 
    /// # Parameters
    /// - `inst_type`: Instrument type (e.g., "SWAP", "FUTURES", "MARGIN")
    /// - `uly`: Underlying asset
    /// - `inst_family`: Instrument family
    /// - `inst_id`: Instrument ID (e.g., "BTC-USDT-SWAP")
    /// - `cache_expire_seconds`: Cache expiration time in seconds (0 = no cache)
    /// 
    /// # Returns
    /// A new `MarkPriceReq` instance
    pub fn create_mark_price_req(&self, inst_type: String, uly: String, inst_family: String, inst_id: String, cache_expire_seconds: u64) -> KlineMarkPriceReq { todo!() }
    /// Creates a new `OpeningInfoReq` with the specified parameters.
    /// 
    /// # Parameters
    /// - `inst_id`: Instrument ID (e.g., "BTC-USDT")
    /// - `cache_expire_seconds`: Cache expiration time in seconds (0 = no cache)
    /// 
    /// # Returns
    /// A new `OpeningInfoReq` instance
    pub fn create_opening_info_req(&self, inst_id: String, cache_expire_seconds: u64) -> KlineOpeningInfoReq { todo!() }
    /// Get batch tickers data
    /// 
    /// Fetches ticker information for multiple instruments in a single request.
    /// The inst_ids are joined with comma when making the HTTP request.
    /// 
    /// URL: /priapi/v5/market/mult-tickers
    /// 
    /// # Parameters
    /// - `req`: Batch tickers request with instrument IDs and cache settings
    /// 
    /// # Returns
    /// A cancellable future that resolves to `BatchTickersResult`
    pub fn get_batch_tickers(&self, req: KlineBatchTickersReq) -> Arc<OKRustFutureBatchTickersResultWithOKFFIError> { todo!() }
    /// Get index tickers data
    /// 
    /// Fetches index ticker information including latest price, 24h high/low/open,
    /// and other related data.
    /// 
    /// URL: /priapi/v5/market/index-tickers
    /// 
    /// # Parameters
    /// - `req`: Index tickers request with quote currency, instrument ID, and cache settings
    /// 
    /// # Returns
    /// A cancellable future that resolves to `IndexTickersResult`
    pub fn get_index_tickers(&self, req: KlineIndexTickersReq) -> Arc<OKRustFutureIndexTickersResultWithOKFFIError> { todo!() }
    /// Get lever interval data
    /// 
    /// Fetches leverage interval configuration including min/max leverage,
    /// whether max can be exceeded, and percentage options.
    /// 
    /// URL: /priapi/v5/public/lever-interval
    /// 
    /// # Parameters
    /// - `req`: Lever interval request with instrument info and cache settings
    /// 
    /// # Returns
    /// A cancellable future that resolves to `LeverIntervalResult`
    pub fn get_lever_interval(&self, req: KlineLeverIntervalReq) -> Arc<OKRustFutureLeverIntervalResultWithOKFFIError> { todo!() }
    /// Get mark price data
    /// 
    /// Fetches mark price information for instruments.
    /// 
    /// URL: /priapi/v5/public/mark-price
    /// 
    /// # Parameters
    /// - `req`: Mark price request with instrument info and cache settings
    /// 
    /// # Returns
    /// A cancellable future that resolves to `MarkPriceResult`
    pub fn get_mark_price(&self, req: KlineMarkPriceReq) -> Arc<OKRustFutureMarkPriceResultWithOKFFIError> { todo!() }
    /// Get opening info data
    /// 
    /// Fetches opening/listing information for an instrument including
    /// opening mode (auction/fixed price/pre-quote), opening price,
    /// matched auction volume, and base currency.
    /// 
    /// URL: /priapi/v5/public/trading-rules/list-info
    /// 
    /// # Parameters
    /// - `req`: Opening info request with instrument ID and cache settings
    /// 
    /// # Returns
    /// A cancellable future that resolves to `OpeningInfoResult`
    pub fn get_opening_info(&self, req: KlineOpeningInfoReq) -> Arc<OKRustFutureOpeningInfoResultWithOKFFIError> { todo!() }
}

pub struct LatestDealInteractor;
impl LatestDealInteractor {
    /// Stop subscription and detach the state observer.
    pub fn on_clear(&self) { todo!() }
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
    /// Enable or disable debug logging.
    /// 
    /// In release builds this is a no-op, but the symbol must exist
    /// so that UniFFI bindings generated in debug mode continue to
    /// link successfully against the release library.
    pub fn set_debug(&self, _enabled: bool) { todo!() }
    /// Set filter threshold. Empty string clears the filter.
    pub fn set_filter(&self, min_size: String) { todo!() }
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
    pub fn state(&self) -> LatestDealInteractorState { todo!() }
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
    /// Stop subscription and clear cached data, keeping the state observer for future re-subscribe.
    pub fn stop_subscription(&self) { todo!() }
    /// Start subscribing to latest deals for the given instrument.
    pub fn subscribe(&self, inst_id: String, inst_type: String) { todo!() }
}

/// Callback for latest deal stream updates
pub struct LatestDealStreamCallback;
impl LatestDealStreamCallback {
    /// Called when the subscription is completed
    pub fn on_update_completed(&self) { todo!() }
    /// Called when deal data is updated (from HTTP or WebSocket)
    /// 
    /// # Arguments
    /// * `deals` - List of deal data (None if no data available)
    pub fn on_update_deals(&self, deals: Option<Vec<BizLatestDeal>>) { todo!() }
    /// Called when an error occurs during subscription
    pub fn on_update_error(&self, error: OKFFIError) { todo!() }
}

/// Market events interactor
pub struct MarketEventsInteractor;
impl MarketEventsInteractor {
    /// Initialize interactor with inst_id (called when page is created/rebuilt)
    pub fn initialize(&self, inst_id: String, unit: String, display_inst_name: String) { todo!() }
    /// Load more (pagination)
    pub fn load_more(&self) { todo!() }
    /// Called when page destroy
    pub fn on_clear(&self) { todo!() }
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
    /// DEPRECATED. Use `ui_event_stream()`; drop the returned stream
    /// wrapper to unsubscribe.
    pub fn remove_ui_event_observer(&self) { todo!() }
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
    /// DEPRECATED: callback-based observer. Use `ui_event_stream()` —
    /// the callback observer pins the foreign object in UniFFI's
    /// static handleMap and produces a cross-language retain cycle
    /// (Android GC + iOS ARC).
    pub fn set_ui_event_observer(&self, observer: Arc<dyn MarketEventsUiEventObserver>) { todo!() }
    /// Start the interactor lifecycle.
    /// 
    /// Calls the user's `InteractorLifecycle::start()` implementation.
    /// Override that method to add custom startup behavior (e.g., initial data loads).
    pub fn start(&self) { todo!() }
    /// Get the current state (UniFFI export).
    /// 
    /// Returns the External type (UniFFI-compatible).
    pub fn state(&self) -> MarketEventsState { todo!() }
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
    /// Opt-in async UI event stream (cycle-free). Single-subscriber.
    pub fn ui_event_stream(&self) -> Arc<MarketEventsUiEventStream> { todo!() }
}

/// Deprecated UniFFI callback interface for observing UI events. Prefer the cycle-free `MarketEventsUiEventStream` returned by `interactor.ui_event_stream()`.
pub struct MarketEventsUiEventObserver;
impl MarketEventsUiEventObserver {
    pub fn on_ui_event(&self, event: MarketEventsUiEvent) { todo!() }
}

/// Opt-in async event stream from `interactor.ui_event_stream()`.
/// Single-subscriber, latest-wins. `next()` resolves to `None` when the
/// interactor is dropped or a new subscription replaces this one.
pub struct MarketEventsUiEventStream;
impl MarketEventsUiEventStream {
    pub async fn next(&self) -> Option<MarketEventsUiEvent> { todo!() }
}

pub struct MoreTradeInteractor;
impl MoreTradeInteractor {
    pub fn on_clear(&self) { todo!() }
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
    /// DEPRECATED. Use `ui_event_stream()`; drop the returned stream
    /// wrapper to unsubscribe.
    pub fn remove_ui_event_observer(&self) { todo!() }
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
    /// DEPRECATED: callback-based observer. Use `ui_event_stream()` —
    /// the callback observer pins the foreign object in UniFFI's
    /// static handleMap and produces a cross-language retain cycle
    /// (Android GC + iOS ARC).
    pub fn set_ui_event_observer(&self, observer: Arc<dyn MoreTradeUiEventObserver>) { todo!() }
    /// Start the interactor lifecycle.
    /// 
    /// Calls the user's `InteractorLifecycle::start()` implementation.
    /// Override that method to add custom startup behavior (e.g., initial data loads).
    pub fn start(&self) { todo!() }
    /// Get the current state (UniFFI export).
    /// 
    /// Returns the External type (UniFFI-compatible).
    pub fn state(&self) -> MoreTradeInteractorState { todo!() }
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
    pub fn subscribe(&self, inst_id: String, biz_type: String, is_options_restricted: bool) { todo!() }
    /// Toggle favorite status for an instrument (API + UI update).
    /// Handles both add and remove based on current `is_favorite` state.
    pub fn toggle_favorite(&self, inst_id: String) { todo!() }
    /// Opt-in async UI event stream (cycle-free). Single-subscriber.
    pub fn ui_event_stream(&self) -> Arc<MoreTradeUiEventStream> { todo!() }
}

/// Deprecated UniFFI callback interface for observing UI events. Prefer the cycle-free `MoreTradeUiEventStream` returned by `interactor.ui_event_stream()`.
pub struct MoreTradeUiEventObserver;
impl MoreTradeUiEventObserver {
    pub fn on_ui_event(&self, event: MoreTradeUiEvent) { todo!() }
}

/// Opt-in async event stream from `interactor.ui_event_stream()`.
/// Single-subscriber, latest-wins. `next()` resolves to `None` when the
/// interactor is dropped or a new subscription replaces this one.
pub struct MoreTradeUiEventStream;
impl MoreTradeUiEventStream {
    pub async fn next(&self) -> Option<MoreTradeUiEvent> { todo!() }
}

/// Navigation Bar Favorite Bridge
/// 
/// Exposes UniFFI interface for K-line navigation bar favorite operations.
/// Automatically selects appropriate backend based on login status:
/// - **Logged-in users**: Network + cache (FavoritesCombinedRepo)
/// - **Guest users**: Local SQLite (FavoritesDbRepoImpl)
/// 
/// # Example (Swift)
/// ```swift
/// // Create bridge (synchronous constructor returns a future)
/// let future = NavFavoriteBridge.create()
/// try future.startWithCallback(callback: MyCallback())
/// // In callback: let bridge = future.getSuccessResult()
/// 
/// // Check if favorite
/// let isFav = try bridge.isFavorite(item: spotItem)
/// 
/// // Toggle favorite (add if not favorite, remove if favorite)
/// let future = bridge.toggleFavorite(item: spotItem)
/// let isNowFavorite = try await future.poll()
/// ```
pub struct NavFavoriteBridge;
impl NavFavoriteBridge {
    /// Check if the specified item is favorite
    /// 
    /// Synchronous method that checks local cache directly.
    /// 
    /// # Parameters
    /// - `item`: The favorite item to check
    /// 
    /// # Returns
    /// `true` if the item is in favorites
    pub fn is_favorite(&self, item: RetailTradingFavoriteItem) -> Result<bool, OKFFIError> { todo!() }
    /// Load favorites and check item status
    /// 
    /// Used for page initialization: refresh list first, then check status.
    /// 
    /// # Parameters
    /// - `item`: The favorite item to check
    /// 
    /// # Returns
    /// A future that resolves to whether the item is in favorites
    pub fn load_and_check_favorite(&self, item: RetailTradingFavoriteItem) -> Arc<OKRustFutureboolWithOKFFIError> { todo!() }
    /// Toggle favorite status
    /// 
    /// Automatically adds or removes based on current status.
    /// 
    /// # Parameters
    /// - `item`: The favorite item to toggle
    /// 
    /// # Returns
    /// A future that resolves to `true` if item is now favorite,
    /// `false` if item is now not favorite
    pub fn toggle_favorite(&self, item: RetailTradingFavoriteItem) -> Arc<OKRustFutureboolWithOKFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureArcNavFavoriteBridgeWithOKFFIError;
impl OKRustFutureArcNavFavoriteBridgeWithOKFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<Arc<NavFavoriteBridge>> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureAssetDistributionDataWithOKFFIError;
impl OKRustFutureAssetDistributionDataWithOKFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<AssetDistributionData> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureBaseCoinInfoResultWithOKFFIError;
impl OKRustFutureBaseCoinInfoResultWithOKFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<KlineBaseCoinInfoResult> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureBatchTickersResultWithOKFFIError;
impl OKRustFutureBatchTickersResultWithOKFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<KlineBatchTickersResult> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureBuySellMarkerResultWithOKFFIError;
impl OKRustFutureBuySellMarkerResultWithOKFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<BuySellMarkerResult> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureBuySellVolumeDataWithOKFFIError;
impl OKRustFutureBuySellVolumeDataWithOKFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<BuySellVolumeData> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureCampaignListResultWithOKFFIError;
impl OKRustFutureCampaignListResultWithOKFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<KlineCampaignListResult> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureCandlesDataListWithOKFFIError;
impl OKRustFutureCandlesDataListWithOKFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<Vec<BizKLineData>> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureCapitalFlowDataListWithOKFFIError;
impl OKRustFutureCapitalFlowDataListWithOKFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<Vec<CapitalFlowData>> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureCoinDataInfoResultWithOKFFIError;
impl OKRustFutureCoinDataInfoResultWithOKFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<KlineCoinDataInfoResult> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureCoinDetailResultWithOKFFIError;
impl OKRustFutureCoinDetailResultWithOKFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<KlineCoinDetailResult> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureCoinNewsResultWithOKFFIError;
impl OKRustFutureCoinNewsResultWithOKFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<CoinNewsResult> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureCompanyAvailabilityCategoriesWithOKFFIError;
impl OKRustFutureCompanyAvailabilityCategoriesWithOKFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<CompanyAvailabilityCategories> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureCompareCandlesRespListWithOKFFIError;
impl OKRustFutureCompareCandlesRespListWithOKFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<Vec<CompareKLineResp>> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureContractLongShortRatioDataListWithOKFFIError;
impl OKRustFutureContractLongShortRatioDataListWithOKFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<Vec<ContractLongShortRatioData>> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureDepthResultWithOKFFIError;
impl OKRustFutureDepthResultWithOKFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<BizDepth> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureFormulaCandlesDataListWithOKFFIError;
impl OKRustFutureFormulaCandlesDataListWithOKFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<Vec<BizKLineData>> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureFundingRateDataListWithOKFFIError;
impl OKRustFutureFundingRateDataListWithOKFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<Vec<FundingRateData>> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureFuturesSpotGapDataWithOKFFIError;
impl OKRustFutureFuturesSpotGapDataWithOKFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<FuturesSpotGapData> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureIndexTickersResultWithOKFFIError;
impl OKRustFutureIndexTickersResultWithOKFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<KlineIndexTickersResult> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureIndicatorSyncDownloadResultWithOKFFIError;
impl OKRustFutureIndicatorSyncDownloadResultWithOKFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<IndicatorSyncDownloadResult> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureIndicatorSyncUploadResultWithOKFFIError;
impl OKRustFutureIndicatorSyncUploadResultWithOKFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<IndicatorSyncUploadResult> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureInterestAndTradingUnionItemListWithOKFFIError;
impl OKRustFutureInterestAndTradingUnionItemListWithOKFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<Vec<InterestAndTradingUnionItem>> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureKLineRiskTrendItemsWithOKFFIError;
impl OKRustFutureKLineRiskTrendItemsWithOKFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<KLineRiskTrendItems> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureKLineTierItemsWithOKFFIError;
impl OKRustFutureKLineTierItemsWithOKFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<KLineTierItems> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureKlineBigDataInstMetaWithOKFFIError;
impl OKRustFutureKlineBigDataInstMetaWithOKFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<KlineBigDataInstMeta> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureKlineEconEventResultWithOKFFIError;
impl OKRustFutureKlineEconEventResultWithOKFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<KlineEconEventResult> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureKlineStartTimeListWithOKFFIError;
impl OKRustFutureKlineStartTimeListWithOKFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<Vec<KlineStartTimeVo>> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureLeverIntervalResultWithOKFFIError;
impl OKRustFutureLeverIntervalResultWithOKFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<KlineLeverIntervalResult> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureLoadResultWithOKFFIError;
impl OKRustFutureLoadResultWithOKFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<bool> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureLongShortRatioDataListWithOKFFIError;
impl OKRustFutureLongShortRatioDataListWithOKFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<Vec<LongShortRatioData>> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureMarkPriceResultWithOKFFIError;
impl OKRustFutureMarkPriceResultWithOKFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<KlineMarkPriceResult> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureMarketCapV2ResultWithOKFFIError;
impl OKRustFutureMarketCapV2ResultWithOKFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<MarketCapV2Result> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureNewsCarouselResultWithOKFFIError;
impl OKRustFutureNewsCarouselResultWithOKFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<NewsCarouselResult> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureOpeningInfoResultWithOKFFIError;
impl OKRustFutureOpeningInfoResultWithOKFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<KlineOpeningInfoResult> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFuturePriceAlertGroupsWithOKFFIError;
impl OKRustFuturePriceAlertGroupsWithOKFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<Vec<RetailKLinePriceAlertGroup>> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFuturePriceAlertsWithOKFFIError;
impl OKRustFuturePriceAlertsWithOKFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<Vec<RetailKLinePriceAlert>> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureStartResultWithOKFFIError;
impl OKRustFutureStartResultWithOKFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<StartResult> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureTokenUnlockEventsResultWithOKFFIError;
impl OKRustFutureTokenUnlockEventsResultWithOKFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<KlineTokenUnlockEventsResult> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureTradingIndicatorResultWithRetailKlineError;
impl OKRustFutureTradingIndicatorResultWithRetailKlineError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<RetailKlineError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<TradingIndicatorResult> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), RetailKlineError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureUnlockAllocationResultWithOKFFIError;
impl OKRustFutureUnlockAllocationResultWithOKFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<KlineUnlockAllocationResult> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureUnlockProgressResultWithOKFFIError;
impl OKRustFutureUnlockProgressResultWithOKFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<KlineUnlockProgressResult> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureUnlockScheduleResultWithOKFFIError;
impl OKRustFutureUnlockScheduleResultWithOKFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<KlineUnlockScheduleResult> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKFFIError> { todo!() }
}

/// K-line order, position, and asset data bridge
/// 
/// Manages the lifecycle of order, position, and asset data sources,
/// and pushes converted data to the native layer via callbacks.
/// 
/// # Lifecycle
/// 
/// ```text
/// new() -> set_callback() -> start() -> [data push via callback] -> stop()
/// ```
pub struct OrderPositionAssetBridge;
impl OrderPositionAssetBridge {
    /// Create a new bridge
    /// 
    /// # Arguments
    /// * `inst_type` - Instrument type (e.g., "SPOT", "SWAP", "FUTURES", "MARGIN")
    /// * `inst_id` - Instrument ID (e.g., "BTC-USDT", "BTC-USDT-SWAP")
    pub fn new(inst_type: String, inst_id: String) -> Arc<OrderPositionAssetBridge> { todo!() }
    /// Clear callback
    pub fn clear_callback(&self) { todo!() }
    /// Get current assets (spot only)
    pub fn get_assets(&self) -> Vec<RetailKLineAsset> { todo!() }
    /// Get current orders (without position association)
    pub fn get_orders(&self) -> Vec<RetailKLineOrder> { todo!() }
    /// Get current orders and positions with order-position association
    /// 
    /// TP/SL orders are attached to their corresponding positions.
    pub fn get_orders_and_positions(&self) -> OrdersAndPositionsResult { todo!() }
    /// Get current positions (without order association)
    pub fn get_positions(&self) -> Vec<RetailKLinePosition> { todo!() }
    /// Get current quote coin price (spot only)
    pub fn get_quote_coin_price(&self) -> Option<String> { todo!() }
    /// Set callback for data change notifications
    pub fn set_callback(&self, callback: Arc<dyn OrderPositionAssetCallback>) { todo!() }
    /// Start all data sources
    /// 
    /// Each data source starts independently - one failure won't block others.
    /// Errors are reported via the callback's `on_error` method.
    pub fn start(&self) -> Arc<OKRustFutureboolWithOKFFIError> { todo!() }
    /// Stop all data sources and clear cache
    pub fn stop(&self) { todo!() }
    /// Update configuration (used when switching instruments)
    /// 
    /// Stops current data sources, updates config, and restarts.
    pub fn update_config(&self, inst_type: String, inst_id: String) -> Arc<OKRustFutureboolWithOKFFIError> { todo!() }
}

/// Callback for order, position, and asset data changes
/// 
/// Implement this trait in Swift/Kotlin to receive real-time data updates.
pub struct OrderPositionAssetCallback;
impl OrderPositionAssetCallback {
    /// Called when asset data changes (spot only)
    pub fn on_assets_changed(&self, assets: Vec<RetailKLineAsset>) { todo!() }
    /// Called when an error occurs in a data source
    pub fn on_error(&self, source: KLineDataSourceErrorKind, error_message: String) { todo!() }
    /// Called when order or position data changes
    /// 
    /// Orders and positions are provided together for order-position association.
    pub fn on_orders_and_positions_changed(&self, orders: Vec<RetailKLineOrder>, positions: Vec<RetailKLinePosition>) { todo!() }
}

pub struct PriceAlertBridge;
impl PriceAlertBridge {
    pub fn create_for_cedefi(chain_id: String, token_address: String) -> Arc<PriceAlertBridge> { todo!() }
    pub fn create_for_cefi(inst_id: String, biz_type: String) -> Arc<PriceAlertBridge> { todo!() }
    pub fn default() -> Arc<PriceAlertBridge> { todo!() }
    pub fn activate_price_alert(&self, rule_id: u64, source_type: RetailKLinePriceAlertSourceType, active: bool) -> Arc<OKRustFutureboolWithOKFFIError> { todo!() }
    pub fn add_price_alert(&self, alert: RetailKLinePriceAlert) -> Arc<OKRustFutureboolWithOKFFIError> { todo!() }
    pub fn delete_price_alert(&self, rule_id: u64, source_type: RetailKLinePriceAlertSourceType) -> Arc<OKRustFutureboolWithOKFFIError> { todo!() }
    pub fn delete_price_alerts(&self, rule_ids_by_source_type: HashMap<RetailKLinePriceAlertSourceType, Vec<u64>>) -> Arc<OKRustFutureboolWithOKFFIError> { todo!() }
    pub fn get_price_alert_groups(&self) -> Arc<OKRustFuturePriceAlertGroupsWithOKFFIError> { todo!() }
    pub fn get_price_alerts(&self) -> Arc<OKRustFuturePriceAlertsWithOKFFIError> { todo!() }
    pub fn get_unique_id(&self) -> Option<String> { todo!() }
    pub fn price_alert_maximum_count(&self) -> u32 { todo!() }
    pub fn set_ticker_ws_callback(&self, callback: Arc<dyn PriceTickerCallback>) { todo!() }
    pub fn start_ticker_ws(&self) { todo!() }
    pub fn stop_ticker_ws(&self) { todo!() }
    pub fn update_price_alert(&self, alert: RetailKLinePriceAlert) -> Arc<OKRustFutureboolWithOKFFIError> { todo!() }
}

/// Price alert interactor
pub struct PriceAlertInteractor;
impl PriceAlertInteractor {
    /// Activate or deactivate a price alert
    pub fn activate_alert(&self, rule_id: u64, source_type: RetailKLinePriceAlertSourceType, active: bool) { todo!() }
    /// Add a new price alert
    pub fn add_alert(&self, alert: RetailKLinePriceAlert) { todo!() }
    /// Delete multiple price alerts in ONE effect (mixed source types supported)
    pub fn batch_delete_alert_groups(&self, groups: Vec<PriceAlertBatchDeleteGroup>) { todo!() }
    /// Delete multiple price alerts of a single source type.
    /// 
    /// Deprecated: prefer [`Self::batch_delete_alert_groups`] — calling this repeatedly
    /// for mixed source types loses batches (the MutateAlert effect is SkipIfRunning).
    /// Kept with the original signature for UniFFI ABI compatibility.
    pub fn batch_delete_alerts(&self, source_type: RetailKLinePriceAlertSourceType, rule_ids: Vec<u64>) { todo!() }
    /// Delete a single price alert
    pub fn delete_alert(&self, rule_id: u64, source_type: RetailKLinePriceAlertSourceType) { todo!() }
    /// Initialize interactor and load alerts (called when page is created)
    pub fn initialize(&self) { todo!() }
    /// Reload all alerts
    pub fn load_alerts(&self) { todo!() }
    /// Load alert groups
    pub fn load_groups(&self) { todo!() }
    /// Called when page is destroyed
    pub fn on_clear(&self) { todo!() }
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
    /// DEPRECATED. Use `ui_event_stream()`; drop the returned stream
    /// wrapper to unsubscribe.
    pub fn remove_ui_event_observer(&self) { todo!() }
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
    /// Set a callback to receive real-time price updates from the ticker WebSocket.
    /// 
    /// Must be called before `start_ticker_ws()`. The callback's `on_price_update(price)`
    /// method will be called on a background thread whenever a new ticker price is received.
    /// Native callers must dispatch to the main thread if needed.
    pub fn set_ticker_ws_callback(&self, callback: Arc<dyn PriceTickerCallback>) { todo!() }
    /// DEPRECATED: callback-based observer. Use `ui_event_stream()` —
    /// the callback observer pins the foreign object in UniFFI's
    /// static handleMap and produces a cross-language retain cycle
    /// (Android GC + iOS ARC).
    pub fn set_ui_event_observer(&self, observer: Arc<dyn PriceAlertUiEventObserver>) { todo!() }
    /// Start the interactor lifecycle.
    /// 
    /// Calls the user's `InteractorLifecycle::start()` implementation.
    /// Override that method to add custom startup behavior (e.g., initial data loads).
    pub fn start(&self) { todo!() }
    /// Start the ticker WebSocket subscription as a persistent effect.
    /// 
    /// The WS runs until `stop_ticker_ws()` is called, or the interactor is stopped/cleared.
    /// Uses `AlwaysCancel` policy: calling `start_ticker_ws()` again will cancel the previous
    /// subscription and start a new one.
    pub fn start_ticker_ws(&self) { todo!() }
    /// Start the interactor lifecycle WITHOUT loading alerts. Lets the consumer own the
    /// initial load (e.g. a page that loads on resume) so it isn't fetched twice on first entry.
    pub fn start_without_load(&self) { todo!() }
    /// Get the current state (UniFFI export).
    /// 
    /// Returns the External type (UniFFI-compatible).
    pub fn state(&self) -> PriceAlertState { todo!() }
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
    /// Stop the ticker WebSocket subscription by cancelling the persistent effect.
    pub fn stop_ticker_ws(&self) { todo!() }
    /// Opt-in async UI event stream (cycle-free). Single-subscriber.
    pub fn ui_event_stream(&self) -> Arc<PriceAlertUiEventStream> { todo!() }
    /// Update an existing price alert
    pub fn update_alert(&self, alert: RetailKLinePriceAlert) { todo!() }
}

/// Deprecated UniFFI callback interface for observing UI events. Prefer the cycle-free `PriceAlertUiEventStream` returned by `interactor.ui_event_stream()`.
pub struct PriceAlertUiEventObserver;
impl PriceAlertUiEventObserver {
    pub fn on_ui_event(&self, event: PriceAlertUiEvent) { todo!() }
}

/// Opt-in async event stream from `interactor.ui_event_stream()`.
/// Single-subscriber, latest-wins. `next()` resolves to `None` when the
/// interactor is dropped or a new subscription replaces this one.
pub struct PriceAlertUiEventStream;
impl PriceAlertUiEventStream {
    pub async fn next(&self) -> Option<PriceAlertUiEvent> { todo!() }
}

pub struct PriceFloatWindowInteractor;
impl PriceFloatWindowInteractor {
    /// Called when the page / window is destroyed.
    pub fn on_clear(&self) { todo!() }
    /// Force re-subscribe with the (caller-supplied) current visible set.
    /// Semantically identical to `subscribe_visible`; kept as a distinct
    /// action solely so callers (and analytics/log readers) can differentiate
    /// initial-subscribe from explicit recovery-refresh. Same SubscribeStart
    /// effect slot, same placeholder reseed.
    pub fn refresh(&self, visible: Vec<InstrumentRef>) { todo!() }
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
    pub fn state(&self) -> PriceFloatWindowState { todo!() }
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
    /// Subscribe to tickers for the given visible set. Replaces any prior
    /// subscription. State.visible_items is re-initialized with empty recipes
    /// so the UI can render placeholders for any new items.
    pub fn subscribe_visible(&self, visible: Vec<InstrumentRef>) { todo!() }
    /// Cancel the current subscription (window dismissed or screen off).
    pub fn unsubscribe(&self) { todo!() }
}

/// Callback interface for ticker websocket price updates
/// 
/// This trait is designed to be implemented by foreign code (Swift/Kotlin)
/// to receive real-time price updates from the websocket.
pub struct PriceTickerCallback;
impl PriceTickerCallback {
    /// Called when a new price update is received from the websocket
    /// 
    /// # Arguments
    /// * `price` - The latest price as a string
    pub fn on_price_update(&self, price: String) { todo!() }
}

pub struct PriceWindowListInteractor;
impl PriceWindowListInteractor {
    /// Add an instrument (deduped by inst_id + inst_type).
    pub fn add_instrument(&self, item: PriceWindowItemVo) { todo!() }
    /// Batch-add multiple instruments in a single effect. Use this instead of
    /// calling `add_instrument` in a loop — `SpawnPolicy::SkipIfRunning` on the
    /// shared `Mutate` effect id would drop all but the first call.
    pub fn add_instruments(&self, items: Vec<PriceWindowItemVo>) { todo!() }
    /// Initial load + start lifecycle. Idempotent — safe to call again.
    pub fn load_list(&self) { todo!() }
    /// Called when the page is destroyed.
    pub fn on_clear(&self) { todo!() }
    /// Stop-gap teardown (TEMPORARY — target state is the stream APIs,
    /// which need no teardown call).
    /// 
    /// One idempotent call at host lifecycle end that breaks the
    /// UniFFI handleMap cycle for any registered callback observers,
    /// ends active streams, and stops effects. On Kotlin call BEFORE
    /// `destroy()`. Includes `stop()`.
    pub fn release(&self) { todo!() }
    /// Remove an instrument by (inst_id, inst_type).
    pub fn remove_instrument(&self, inst_id: String, inst_type: String) { todo!() }
    /// Remove the state observer (UniFFI export).
    /// 
    /// DEPRECATED: callback-based observation. Use `state_stream()` —
    /// it needs no removal call. Until migrated, every
    /// `set_state_observer` MUST be paired with this (or `release()`)
    /// at host lifecycle end, or the UniFFI handleMap pins the
    /// observer and its host forever.
    pub fn remove_state_observer(&self) { todo!() }
    /// Reorder the full list. Items not present in `order` are appended at end.
    pub fn reorder(&self, order: Vec<InstrumentRef>) { todo!() }
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
    pub fn state(&self) -> PriceWindowListState { todo!() }
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
}

pub struct RetailBigDataBridge;
impl RetailBigDataBridge {
    pub fn new() -> Arc<RetailBigDataBridge> { todo!() }
    /// Get asset distribution data (Spot)
    /// 
    /// Fetches asset distribution data for spot trading, showing how different sizes
    /// of traders (extra large, large, mid, small) are buying and selling.
    /// 
    /// # Parameters
    /// - `req`: Asset distribution request parameters (inst_id, period)
    /// 
    /// # Returns
    /// A future that resolves to asset distribution data
    pub fn get_asset_distribution(&self, req: AssetDistributionReq) -> Arc<OKRustFutureAssetDistributionDataWithOKFFIError> { todo!() }
    /// Get buy & sell volume data (Swap/Futures)
    /// 
    /// Fetches the aggregated taker buy and sell volume for perpetual swap and futures contracts.
    /// 
    /// # Parameters
    /// - `req`: Buy/sell volume request parameters (inst_id, period, unit)
    /// 
    /// # Returns
    /// A future that resolves to buy/sell volume data (including cont, crypto, usds)
    pub fn get_buy_sell_volume(&self, req: BuySellVolumeReq) -> Arc<OKRustFutureBuySellVolumeDataWithOKFFIError> { todo!() }
    /// Get capital flow data (Spot)
    /// 
    /// Fetches capital flow (inflow/outflow) data for spot trading.
    /// 
    /// # Parameters
    /// - `req`: Capital flow request parameters (inst_id, period)
    /// 
    /// # Returns
    /// A future that resolves to a list of capital flow data
    pub fn get_capital_flow(&self, req: CapitalFlowReq) -> Arc<OKRustFutureCapitalFlowDataListWithOKFFIError> { todo!() }
    /// Get contract long/short ratio data (Swap/Futures)
    /// 
    /// Fetches the ratio of long vs short positions for perpetual swap and futures contracts.
    /// 
    /// # Parameters
    /// - `req`: Contract long/short ratio request parameters (inst_id, period)
    /// 
    /// # Returns
    /// A future that resolves to a list of long/short ratio data
    pub fn get_contract_long_short_ratio(&self, req: ContractLongShortRatioReq) -> Arc<OKRustFutureContractLongShortRatioDataListWithOKFFIError> { todo!() }
    /// Get funding rate history data (Swap)
    /// 
    /// Fetches historical funding rate data for perpetual swaps.
    /// 
    /// # Parameters
    /// - `req`: Funding rate request parameters (inst_id)
    /// 
    /// # Returns
    /// A future that resolves to a list of funding rate data
    pub fn get_funding_rate(&self, req: FundingRateReq) -> Arc<OKRustFutureFundingRateDataListWithOKFFIError> { todo!() }
    /// Get futures-spot gap data
    /// 
    /// Fetches both futures candles and spot index candles, then combines them
    /// to show the basis (gap) between futures and spot prices over time.
    /// 
    /// # Parameters
    /// - `req`: Futures-spot gap request parameters (futures_inst_id, index_inst_id, bar, limit)
    /// 
    /// # Returns
    /// A future that resolves to futures-spot gap data (dates, futures_data, index_data)
    pub fn get_futures_spot_gap(&self, req: FuturesSpotGapReq) -> Arc<OKRustFutureFuturesSpotGapDataWithOKFFIError> { todo!() }
    /// Get leverage ratio data (Margin)
    /// 
    /// Fetches leverage ratio data for margin trading, showing the long/short
    /// ratio for borrowed assets.
    /// 
    /// # Parameters
    /// - `req`: Leverage ratio request parameters (inst_id, period)
    /// 
    /// # Returns
    /// A future that resolves to a list of long/short ratio data
    pub fn get_leverage_ratio(&self, req: LeverageRatioReq) -> Arc<OKRustFutureLongShortRatioDataListWithOKFFIError> { todo!() }
    /// Get position volume & trading volume data (Swap/Futures)
    /// 
    /// Fetches open interest and trading volume data for perpetual swap and futures contracts.
    /// Uses the union API which returns combined data with both crypto and USD values.
    /// 
    /// # Parameters
    /// - `req`: Position volume request parameters (inst_id, period)
    /// 
    /// # Returns
    /// A future that resolves to a list of InterestAndTradingUnionItem
    pub fn get_position_volume(&self, req: PositionVolumeReq) -> Arc<OKRustFutureInterestAndTradingUnionItemListWithOKFFIError> { todo!() }
}

pub struct RetailCandlesBridge;
impl RetailCandlesBridge {
    pub fn new() -> Arc<RetailCandlesBridge> { todo!() }
    /// Fetch candles initial time for each bar/timeframe
    /// 
    /// This method returns the initial timestamp for each bar/timeframe of a specific instrument.
    /// Useful for determining the earliest available data point for historical data queries.
    /// 
    /// # Parameters
    /// - `request`: Request parameters containing kline_type and inst_id
    /// 
    /// # Returns
    /// A future that resolves to a list of KlineStartTimeVo (bar, timestamp pairs)
    pub fn get_candles_initial_time(&self, request: CandlesInitialTimeReq) -> Arc<OKRustFutureKlineStartTimeListWithOKFFIError> { todo!() }
    /// Fetch history candles (historical K-line data)
    /// 
    /// This method fetches historical candlestick data for a specific instrument.
    /// Used for loading historical data for charting or analysis.
    /// 
    /// # Parameters
    /// - `request`: Request parameters containing inst_id, bar, before, after, limit
    /// 
    /// # Returns
    /// A future that resolves to a list of BizKLineData (candlestick data)
    pub fn get_history_candles(&self, request: HistoryCandlesReq) -> Arc<OKRustFutureCandlesDataListWithOKFFIError> { todo!() }
    /// Subscribe to candles stream (HTTP initial data + WebSocket updates)
    /// 
    /// This method provides a stream-based API for candle data, starting with HTTP
    /// initial data followed by WebSocket real-time updates.
    /// 
    /// # Parameters
    /// - `request`: Candles request parameters (kline_type, inst_id, bar, after, limit)
    /// - `ws_only`: Whether to skip HTTP request and only subscribe to WebSocket
    /// - `true`: Only subscribe to WS push, skip HTTP request
    /// - `false`: Fetch HTTP initial data first, then subscribe to WS
    /// - `callback`: Callback to receive candle updates
    /// 
    /// # Returns
    /// A future that completes when subscription is established
    pub fn subscribe_candles_stream(&self, request: CandlesReq, ws_only: bool, callback: Arc<dyn CandlesStreamCallback>) -> Arc<OKRustFutureboolWithOKFFIError> { todo!() }
}

pub struct RetailCompareCandlesBridge;
impl RetailCompareCandlesBridge {
    pub fn new() -> Arc<RetailCompareCandlesBridge> { todo!() }
    /// Get compare candles data
    /// 
    /// Fetches K-line data for multiple instruments in a single request.
    /// This allows comparing K-line data across different instruments simultaneously.
    /// This is the initial fetch (not historical pagination).
    /// 
    /// # Parameters
    /// - `req_list`: List of compare candles request parameters, one for each instrument to compare.
    /// Each request contains:
    /// - `inst_id`: Instrument ID (e.g., "BTC-USDT", "BTC-USDT-SWAP")
    /// - `candle_type`: K-line type, one of: "TRADE", "INDEX", "MARK" (default: "TRADE")
    /// - `bar`: Bar/timeframe (e.g., "1m", "5m", "15m", "1H", "1D")
    /// - `limit`: Limit number of results (optional, default: 100, max: 100)
    /// 
    /// # Returns
    /// A future that resolves to a list of compare candle responses, one per instrument.
    /// Each response contains the instrument ID, candle type, bar/timeframe, and the K-line data.
    pub fn get_compare_candles(&self, req_list: Vec<CompareKlineReq>) -> Arc<OKRustFutureCompareCandlesRespListWithOKFFIError> { todo!() }
    /// Get history compare candles data
    /// 
    /// Fetches historical K-line data for multiple instruments in a single request.
    /// This allows comparing K-line data across different instruments simultaneously.
    /// 
    /// # Parameters
    /// - `req_list`: List of history compare candles request parameters, one for each instrument to compare.
    /// Each request contains:
    /// - `inst_id`: Instrument ID (e.g., "BTC-USDT", "BTC-USDT-SWAP")
    /// - `candle_type`: K-line type, one of: "TRADE", "INDEX", "MARK" (default: "TRADE")
    /// - `bar`: Bar/timeframe (e.g., "1m", "5m", "15m", "1H", "1D")
    /// - `before`: Timestamp before which to fetch data (optional, for pagination)
    /// - `after`: Timestamp after which to fetch data (optional, for pagination)
    /// - `limit`: Limit number of results (optional, default: 100, max: 100)
    /// 
    /// # Returns
    /// A future that resolves to a list of compare candle responses, one per instrument.
    /// Each response contains the instrument ID, candle type, bar/timeframe, and the K-line data.
    pub fn get_history_compare_candles(&self, req_list: Vec<CompareKlineReq>) -> Arc<OKRustFutureCompareCandlesRespListWithOKFFIError> { todo!() }
    /// Subscribe to compare candles stream (WebSocket)
    /// 
    /// Subscribes to WebSocket streams for multiple instruments simultaneously.
    /// Updates are batched every 200ms to reduce UI refresh frequency.
    /// Uses TRADE (Latest) K-line type.
    /// 
    /// # Parameters
    /// - `inst_ids`: List of instrument IDs (e.g., ["BTC-USDT", "ETH-USDT"])
    /// - `bar`: Bar/timeframe for all instruments (e.g., "1m", "5m", "15m", "1H", "1D")
    /// - `callback`: Callback to receive stream updates
    /// 
    /// # Returns
    /// A future that completes when subscription is established
    pub fn subscribe_compare_candles_stream(&self, inst_ids: Vec<String>, bar: String, callback: Arc<dyn CompareCandlesStreamCallback>) -> Arc<OKRustFutureboolWithOKFFIError> { todo!() }
}

pub struct RetailDepthBridge;
impl RetailDepthBridge {
    /// Create depth bridge
    pub fn new() -> Arc<RetailDepthBridge> { todo!() }
    /// Subscribe to depth stream (WebSocket only)
    /// 
    /// This method provides a stream-based API for order book data via WebSocket.
    /// Following Kotlin DepthResultListener:
    /// - No HTTP request (depth is real-time only)
    /// - First push: snapshot with full depth (400 levels)
    /// - Subsequent pushes: incremental updates every 100ms
    /// 
    /// # Parameters
    /// - `request`: Depth request parameters (inst_id, grouping)
    /// - `callback`: Callback to receive depth updates
    /// 
    /// # Returns
    /// A future that completes when subscription is established
    pub fn subscribe_depth_stream(&self, request: DepthReq, callback: Arc<dyn DepthStreamCallback>) -> Arc<OKRustFutureboolWithOKFFIError> { todo!() }
}

pub struct RetailFormulaCandlesBridge;
impl RetailFormulaCandlesBridge {
    pub fn new() -> Arc<RetailFormulaCandlesBridge> { todo!() }
    /// Create FormulaCandlesSimplified instance for WebSocket subscription
    /// 
    /// # Arguments
    /// * `calc_expression` - Calculation expression string (e.g., "V0 + V1 - V2")
    /// * `inst_id_array` - Instrument ID array as JSON string
    /// (e.g., `[["BTC-USDT","SPOT"],["ETH-USDT","SPOT"]]`)
    /// * `bar` - Bar/timeframe (e.g., "1m", "5m", "15m", "1H", "1D")
    /// * `is_demo_trading` - Whether the user is in demo trading mode
    pub fn create_formula_candles_simplified(&self, calc_expression: String, inst_id_array: String, bar: String, is_demo_trading: bool) -> Arc<FormulaCandlesSimplified> { todo!() }
    /// Get history formula candles data
    /// 
    /// Fetches historical K-line candle data calculated from formula expressions.
    /// Formula candles allow combining multiple instruments using mathematical expressions.
    /// 
    /// # Parameters
    /// - `req`: History formula candles request parameters containing:
    /// - `calc_expression`: Calculation expression string (e.g., "V0 + V1 - V2")
    /// - `inst_id_array`: Instrument ID array as JSON string (e.g., `[["BTC-USDT","SPOT"],["ETH-USDT","SPOT"]]`)
    /// - `bar`: Bar/timeframe (e.g., "1m", "5m", "15m", "1H", "1D")
    /// - `before`: Timestamp before which to fetch data (optional, for pagination)
    /// - `after`: Timestamp after which to fetch data (optional, for pagination)
    /// - `limit`: Limit number of results (optional, default: 100)
    /// 
    /// # Returns
    /// A future that resolves to a list of historical formula candle data
    pub fn get_history_formula_candles(&self, req: HistoryFormulaCandlesReq) -> Arc<OKRustFutureFormulaCandlesDataListWithOKFFIError> { todo!() }
}

/// Native platform API interface for retail Kline module
/// 
/// Implementations are injected from the native platform (Android/iOS) at runtime.
/// All implementations must be `Send + Sync` for thread-safe access.
pub struct RetailKlineNativeApi;
impl RetailKlineNativeApi {
    /// Get the up/down time zone configuration
    pub fn get_up_down_time_zone(&self) -> KlineUpDownTimeZone { todo!() }
}

pub struct RetailLatestDealBridge;
impl RetailLatestDealBridge {
    pub fn new() -> Arc<RetailLatestDealBridge> { todo!() }
    /// Subscribe to latest deals stream (HTTP initial data + WebSocket updates)
    /// 
    /// This method provides a stream-based API for latest deal data, starting with HTTP
    /// initial data followed by WebSocket real-time updates.
    /// 
    /// # Parameters
    /// - `request`: Latest deal request parameters (inst_id, limit)
    /// - `callback`: Callback to receive deal updates
    /// 
    /// # Returns
    /// A future that completes when subscription is established
    pub fn subscribe_latest_deals_stream(&self, request: LatestDealReq, callback: Arc<dyn LatestDealStreamCallback>) -> Arc<OKRustFutureboolWithOKFFIError> { todo!() }
}

/// Implemented by the shell (Android/iOS). All inputs are absolute epoch ms.
pub struct StockHoursLocaleFormatter;
impl StockHoursLocaleFormatter {
    /// Localized short duration (largest two units, minute-ceiled), e.g.
    /// "3h 19m" / "1d 2h" (unit names follow the app locale, incl. CJK).
    /// Fills the countdown `{time}`.
    pub fn format_duration(&self, duration_ms: i64) -> String { todo!() }
    /// Localized "EEE MM/dd HH:mm" in device tz, e.g. "Thu 11/26 09:00" (the
    /// weekday name follows the app locale, incl. CJK). Fills the closure
    /// `{startDate}` / `{endDate}`.
    pub fn format_weekday_datetime(&self, timestamp_ms: i64) -> String { todo!() }
    /// Localized "EEE HH:mm" in device tz, e.g. "Sat 00:00" (locale-dependent
    /// weekday). Used for the weekend/holiday schedule-row time range.
    pub fn format_weekday_time(&self, timestamp_ms: i64) -> String { todo!() }
}

/// Callback interface for TradingIndicatorDataSourceBridge (UniFFI export)
/// 
/// Swift/Kotlin implements this interface to receive parsed TradingIndicatorResult.
/// 
/// # Memory Management for Native Implementations
/// 
/// **Your callback implementation can safely hold a strong reference to the DataSource:**
/// 
/// ```swift
/// class MyCallback: TradingIndicatorCallbackBridge {
/// let dataSource: TradingIndicatorDataSourceBridge  // ✅ Safe! No weak needed
/// 
/// init(dataSource: TradingIndicatorDataSourceBridge) {
/// self.dataSource = dataSource
/// }
/// }
/// ```
/// 
/// **Why this is safe:**
/// - The internal `TradingIndicatorDataSource` stores the wrapper as a `Weak` reference
/// - Even if Swift callback → DataSource → internal → Weak(wrapper), the cycle is broken
/// - No memory leaks will occur
/// - Native code doesn't need to use weak references
pub struct TradingIndicatorCallbackBridge;
impl TradingIndicatorCallbackBridge {
    /// Called when HTTP request fails
    pub fn on_error(&self, error_message: String) { todo!() }
    /// Called when HTTP request succeeds with parsed data
    pub fn on_success(&self, data: TradingIndicatorResult) { todo!() }
    /// Called when WebSocket message is received with parsed data
    /// 
    /// # Parameters
    /// - `merged_data`: Complete merged data after combining with cache (all indicators).
    /// Native uses this to update the UI directly.
    pub fn on_ws_message(&self, merged_data: TradingIndicatorResult) { todo!() }
}

/// Trading indicator data source - UniFFI export wrapper
/// 
/// This wrapper implements `TradingIndicatorCallback` trait and forwards events to native callback.
/// 
/// # Reference Chain (Fix #8: Accurate documentation)
/// 
/// ```text
/// wrapper.inner (Arc<TradingIndicatorDataSource>)
/// └── stores wrapper as Weak<dyn TradingIndicatorCallback>  ← This breaks the cycle!
/// 
/// wrapper.native_callback (Arc<dyn TradingIndicatorCallbackBridge>)
/// └── Strong reference to Swift/Kotlin callback (safe, no back-reference)
/// ```
/// 
/// # Memory Safety Analysis
/// 
/// - `inner → wrapper`: **Weak** reference (breaks cycle)
/// - `wrapper → native_callback`: **Strong** reference (Swift callback lifetime managed by Rust)
/// - Swift callback → wrapper: Can be **Strong** (cycle already broken by inner's Weak)
/// 
/// Corresponds to OC's OKTKLineTradingDataSource.
pub struct TradingIndicatorDataSourceBridge;
impl TradingIndicatorDataSourceBridge {
    /// Create a new trading indicator data source
    /// 
    /// # Parameters
    /// - `inst_id`: Contract ID (must be in contract format, not spot)
    /// - ✅ Valid: "BTC-USDT-SWAP", "ETH-USDT-SWAP", "BTC-USD-SWAP"
    /// - ❌ Invalid: "BTC-USDT" (spot format not supported)
    /// - `indicators`: List of indicators (corresponds to OC's tradingDataTypes)
    /// - `unit`: Trading unit (corresponds to `core-trading`'s `TRADE_UNIT_*` constants)
    /// - `"0"` = `TRADE_UNIT_COIN` (coin-margined)
    /// - `"1"` = `TRADE_UNIT_CONTRACT` (contracts)
    /// - `"2"` = `TRADE_UNIT_U` (USDT/USDC/USD)
    /// - `bar`: K-line period (e.g., "1H", "1D")
    /// - `limit`: Data limit
    /// - `after`: Request timestamp (milliseconds), used for pagination. If empty string, uses current timestamp
    /// 
    /// # Example (Swift)
    /// ```swift
    /// let dataSource = TradingIndicatorDataSourceBridge(
    /// instId: "BTC-USDT-SWAP",  // Must be in contract format
    /// indicators: ["oi", "takerBuySellVol"],
    /// unit: "0",  // TRADE_UNIT_COIN (coin-margined)
    /// bar: "1H",
    /// limit: 100,
    /// after: ""   // Use current timestamp
    /// )
    /// ```
    pub fn new(inst_id: String, indicators: Vec<String>, unit: String, bar: String, limit: i64, after: String) -> Arc<TradingIndicatorDataSourceBridge> { todo!() }
    /// Get cached trading indicator data (returns clone, None if not fetched yet)
    /// 
    /// # Returns
    /// - `Some(TradingIndicatorResult)`: Cloned cached data
    /// - `None`: No data cached yet
    pub fn get_trading_indicator_data(&self) -> Option<TradingIndicatorResult> { todo!() }
    /// Reset data source: clears all data and stops all connections
    /// 
    /// This is a complete reset operation that:
    /// 1. Clears cached indicator data
    /// 2. Stops all active connections (HTTP/WebSocket)
    /// 3. Cancels pending WebSocket updates
    /// 4. Resets WebSocket runtime state
    /// 
    /// Equivalent to calling `clear_data()` + `stop()`.
    /// Use when leaving the page or when a complete reset is needed.
    pub fn reset(&self) { todo!() }
    /// Restart WebSocket only (without HTTP request)
    /// 
    /// Use cases: app foreground, reconnect after disconnection/network switch
    pub fn restart_websocket(&self) { todo!() }
    /// Set callback interface for receiving trading indicator data updates
    /// 
    /// Registers a callback that receives HTTP and WebSocket trading indicator data.
    /// All updates are delivered through this callback interface.
    /// 
    /// # Memory Safety
    /// 
    /// This method stores a **Strong** reference to the callback. This is safe because:
    /// 1. The internal `TradingIndicatorDataSource` stores `self` (wrapper) as `Weak`
    /// 2. This prevents circular references: wrapper → inner → Weak(wrapper)
    /// 3. Swift/Kotlin callback can safely hold strong ref to wrapper
    /// 
    /// # Example (Swift)
    /// 
    /// ```swift
    /// class CallbackImpl: TradingIndicatorCallbackBridge {
    /// let dataSource: TradingIndicatorDataSourceBridge  // ✅ Safe! Strong ref is OK
    /// 
    /// init(dataSource: TradingIndicatorDataSourceBridge) {
    /// self.dataSource = dataSource
    /// }
    /// 
    /// func onSuccess(data: TradingIndicatorResult) {
    /// // Handle data
    /// }
    /// }
    /// 
    /// dataSource.setCallback(callback: callbackImpl)
    /// ```
    pub fn set_callback(&self, callback: Arc<dyn TradingIndicatorCallbackBridge>) { todo!() }
    /// Start fetching data (fire-and-forget).
    /// 
    /// Initiates data fetching in the background.
    /// All data and errors are delivered via the callback interface:
    /// - `callback.on_success()` - Initial HTTP data
    /// - `callback.on_ws_message()` - Real-time WebSocket updates
    /// - `callback.on_error()` - Any errors during operation
    /// 
    /// # Lifecycle
    /// 
    /// - HTTP request executes first (Sequential strategy)
    /// - WebSocket subscribes after HTTP completes
    /// - Call `stop()` to terminate WebSocket when leaving page
    /// 
    /// # Example
    /// 
    /// ```swift
    /// dataSource.start()
    /// 
    /// // Data arrives via callback.onSuccess / callback.onWsMessage
    /// // Stop when done
    /// dataSource.stop()
    /// ```
    pub fn start(&self) { todo!() }
    /// Stop fetching data
    /// 
    /// Terminates all active connections and resets internal state.
    /// Call this when leaving the page or when data is no longer needed.
    /// 
    /// # Cleanup Actions
    /// 1. Clears native callback (releases Swift/Kotlin reference)
    /// 2. Stops internal implementation (closes WebSocket, clears state)
    pub fn stop(&self) { todo!() }
}

pub struct TradingIndicatorInteractor;
impl TradingIndicatorInteractor {
    /// Get cached trading indicator data. Call after `data_version` changes.
    /// 
    /// Returns `None` if no data has been fetched yet.
    pub fn get_data(&self) -> Option<TradingIndicatorResult> { todo!() }
    /// Initialize: create datasource and start fetching data.
    /// 
    /// Cancels any previous datasource and creates a new one.
    /// 
    /// # Parameters
    /// - `inst_id`: Contract ID (must be contract format, e.g. "BTC-USDT-SWAP")
    /// - `indicators`: List of indicator names (e.g. ["oi", "takerBuySellVol"])
    /// - `unit`: Trading unit ("0"=coin, "1"=contract, "2"=USDT)
    /// - `bar`: K-line period (e.g. "1H", "1D")
    /// - `limit`: Data point limit (1-1440)
    pub fn initialize(&self, inst_id: String, indicators: Vec<String>, unit: String, bar: String, limit: i64) { todo!() }
    /// Cleanup when page is destroyed.
    pub fn on_clear(&self) { todo!() }
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
    /// DEPRECATED. Use `ui_event_stream()`; drop the returned stream
    /// wrapper to unsubscribe.
    pub fn remove_ui_event_observer(&self) { todo!() }
    /// Restart WebSocket only (e.g. app foreground, reconnect)
    pub fn restart_websocket(&self) { todo!() }
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
    /// DEPRECATED: callback-based observer. Use `ui_event_stream()` —
    /// the callback observer pins the foreign object in UniFFI's
    /// static handleMap and produces a cross-language retain cycle
    /// (Android GC + iOS ARC).
    pub fn set_ui_event_observer(&self, observer: Arc<dyn TradingIndicatorUiEventObserver>) { todo!() }
    /// Start the interactor lifecycle.
    /// 
    /// Calls the user's `InteractorLifecycle::start()` implementation.
    /// Override that method to add custom startup behavior (e.g., initial data loads).
    pub fn start(&self) { todo!() }
    /// Get the current state (UniFFI export).
    /// 
    /// Returns the External type (UniFFI-compatible).
    pub fn state(&self) -> TradingIndicatorState { todo!() }
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
    /// Opt-in async UI event stream (cycle-free). Single-subscriber.
    pub fn ui_event_stream(&self) -> Arc<TradingIndicatorUiEventStream> { todo!() }
}

/// Deprecated UniFFI callback interface for observing UI events. Prefer the cycle-free `TradingIndicatorUiEventStream` returned by `interactor.ui_event_stream()`.
pub struct TradingIndicatorUiEventObserver;
impl TradingIndicatorUiEventObserver {
    pub fn on_ui_event(&self, event: TradingIndicatorUiEvent) { todo!() }
}

/// Opt-in async event stream from `interactor.ui_event_stream()`.
/// Single-subscriber, latest-wins. `next()` resolves to `None` when the
/// interactor is dropped or a new subscription replaces this one.
pub struct TradingIndicatorUiEventStream;
impl TradingIndicatorUiEventStream {
    pub async fn next(&self) -> Option<TradingIndicatorUiEvent> { todo!() }
}

// -----------------------------------------------------------------------------
// 4. Top-Level Exported Functions (195)
// -----------------------------------------------------------------------------

pub fn alert_price_change_type(alert: RetailKLinePriceAlert) -> RetailKLinePriceAlertChangeType { todo!() }

pub fn alert_price_period(alert: RetailKLinePriceAlert) -> RetailKLinePriceAlertPeriodType { todo!() }

/// Creates a `CoinNewsReq` for CeDeFi scenarios.
/// 
/// This is a UniFFI-exported convenience factory wrapping `CoinNewsReq::new_cedefi`,
/// so Swift/Kotlin can create the request without manually filling CeFi fields.
/// 
/// # Parameters
/// - `contract_address`: Smart contract address for DeFi tokens
/// - `chain_id`: Blockchain chain identifier (e.g., "1" for Ethereum)
/// - `token_symbol`: Token symbol for DeFi assets (e.g., "USDT", "WETH")
/// - `size`: Number of items to return (default: 20 if None)
/// - `cache_expire_seconds`: Cache expiration time in seconds (default: 0 if None)
/// - `url`: API endpoint URL (uses default CeDeFi URL if empty)
/// 
/// # Returns
/// A new `CoinNewsReq` configured for CeDeFi.
pub fn create_coin_news_req_cedefi(contract_address: String, chain_id: String, token_symbol: String, size: Option<u64>, cache_expire_seconds: Option<u64>, url: String) -> CoinNewsReq { todo!() }

/// Creates a `CoinNewsReq` for CeFi scenarios.
/// 
/// This is a UniFFI-exported convenience factory wrapping `CoinNewsReq::new_cefi`,
/// so Swift/Kotlin can create the request without manually filling CeDeFi fields.
/// 
/// # Parameters
/// - `size`: Number of items to return (default: 20 if None)
/// - `token`: Pagination token (CeFi)
/// - `group`: Group/category filter (CeFi)
/// - `important`: Importance filter (default: "true" if None)
/// - `cache_expire_seconds`: Cache expiration time in seconds (default: 0 if None)
/// - `url`: API endpoint URL (uses default CeFi URL if empty)
/// 
/// # Returns
/// A new `CoinNewsReq` configured for CeFi.
pub fn create_coin_news_req_cefi(size: Option<u64>, token: String, group: String, important: Option<String>, cache_expire_seconds: Option<u64>, url: String) -> CoinNewsReq { todo!() }

pub fn create_indicator_sync_interactor(setting_case: ChartSettingCase) -> Arc<IndicatorSyncInteractor> { todo!() }

/// Factory function to create NavFavoriteBridge asynchronously.
/// 
/// Returns an OKRustFuture that resolves to `Arc<NavFavoriteBridge>` on success.
/// Use `OkRustFutureWrapper` on the Kotlin side to poll the result.
pub fn create_nav_favorite_bridge() -> Arc<OKRustFutureArcNavFavoriteBridgeWithOKFFIError> { todo!() }

/// Creates a `NewsCarouselReq` for CeDeFi scenarios.
/// 
/// This is a UniFFI-exported convenience factory wrapping `NewsCarouselReq::new_cedefi`,
/// so Swift/Kotlin can create the request without manually filling CeFi fields.
/// 
/// # Parameters
/// - `contract_address`: Smart contract address for DeFi tokens
/// - `chain_id`: Blockchain chain identifier (e.g., "1" for Ethereum)
/// - `token`: Token identifier for DeFi assets (e.g., "USDT", "WETH")
/// - `size`: Number of items to return (default: 20 if None)
/// - `cache_expire_seconds`: Cache expiration time in seconds (default: 0 if None)
/// - `url`: API endpoint URL (uses default CeDeFi URL if empty)
/// 
/// # Returns
/// A new `NewsCarouselReq` configured for CeDeFi.
pub fn create_news_carousel_req_cedefi(contract_address: String, chain_id: String, token: String, size: Option<u64>, cache_expire_seconds: Option<u64>, url: String) -> NewsCarouselReq { todo!() }

/// Creates a `NewsCarouselReq` for CeFi scenarios.
/// 
/// This is a UniFFI-exported convenience factory wrapping `NewsCarouselReq::new_cefi`,
/// so Swift/Kotlin can create the request without manually filling CeDeFi fields.
/// 
/// # Parameters
/// - `location_code`: Page location identifier (e.g., "kline_page", "home_page")
/// - `ccy`: Currency filter (e.g., "BTC", "ETH")
/// - `inst_name`: Instrument name filter (e.g., "BTC-USDT")
/// - `biz_type`: Business type filter (e.g., "spot", "swap")
/// - `size`: Number of items to return (default: 20 if None)
/// - `cache_expire_seconds`: Cache expiration time in seconds (default: 0 if None)
/// - `url`: API endpoint URL (uses default CeFi URL if empty)
/// 
/// # Returns
/// A new `NewsCarouselReq` configured for CeFi.
pub fn create_news_carousel_req_cefi(location_code: String, ccy: String, inst_name: String, biz_type: String, size: Option<u64>, cache_expire_seconds: Option<u64>, url: String) -> NewsCarouselReq { todo!() }

pub fn create_retail_kline_arb_bottom_interactor() -> Arc<KlineArbBottomInteractor> { todo!() }

/// Create a single-chart interactor bound to `chart_id`. The initial state has
/// `periods` from [`periods_for_chart`], `current_period` defaulted to the first
/// period (or `None` for charts without a selector), and everything else at its
/// zero value (`loading = false`, `failed = false`, `version = 0`).
pub fn create_retail_kline_big_data_chart_interactor(chart_id: BigDataChartId) -> Arc<BigDataChartInteractor> { todo!() }

/// State is empty until [`BigDataPageActions::configure`] runs.
pub fn create_retail_kline_big_data_page_interactor() -> Arc<BigDataPageInteractor> { todo!() }

pub fn create_retail_kline_bottom_interactor() -> Arc<KlineBottomInteractor> { todo!() }

pub fn create_retail_kline_buy_sell_points_interactor() -> Arc<KlineBuySellPointsInteractor> { todo!() }

/// Factory exposed to native callers.
/// 
/// Constructs the interactor only — it starts NO request and opens NO
/// subscription. Native explicitly calls `initialize` (WS), `start_request`
/// (single HTTP), or `start_multi_request` (batch HTTP) when needed.
pub fn create_retail_kline_candle_adjustment_interactor() -> Arc<CandleAdjustmentInteractor> { todo!() }

pub fn create_retail_kline_candles_interactor() -> Arc<KlineCandlesInteractor> { todo!() }

/// Candle-only page factory. It shares the same candle implementation but
/// does not construct or schedule IndicatorRuntime and emits no indicator
/// schema/data events.
pub fn create_retail_kline_candles_interactor_without_indicators() -> Arc<KlineCandlesInteractor> { todo!() }

pub fn create_retail_kline_cex_stock_hours_interactor(locale_formatter: Arc<dyn StockHoursLocaleFormatter>) -> Arc<KlineCexStockHoursInteractor> { todo!() }

/// Factory exposed to native. Starts nothing — native calls `load(ticker)`.
pub fn create_retail_kline_company_dividend_interactor() -> Arc<CompanyDividendInteractor> { todo!() }

/// Build a subject-scoped facade. Native must call `prepare` before loading a surface.
pub fn create_retail_kline_company_experience_interactor() -> Arc<CompanyExperienceInteractor> { todo!() }

/// Factory exposed to native. Starts nothing — native calls `load(ticker)`.
pub fn create_retail_kline_company_financials_interactor() -> Arc<CompanyFinancialsInteractor> { todo!() }

/// Factory exposed to native. Shares one request center between the two use
/// cases (single HTTP client). Starts nothing — native calls `load(ticker)`.
pub fn create_retail_kline_company_profile_interactor() -> Arc<CompanyProfileInteractor> { todo!() }

/// Factory exposed to native. Starts nothing — native calls `load(ticker)`.
pub fn create_retail_kline_company_shareholders_interactor() -> Arc<CompanyShareholdersInteractor> { todo!() }

pub fn create_retail_kline_depth_interactor() -> Arc<KlineDepthInteractor> { todo!() }

/// Create a DrawingLineInteractor for CeFi mode
pub fn create_retail_kline_drawing_line_interactor(max_lines: u32) -> Arc<DrawingLineInteractor> { todo!() }

/// Create a DrawingLineInteractor for CeDeFi mode
pub fn create_retail_kline_drawing_line_interactor_cedefi(max_lines: u32) -> Arc<DrawingLineInteractor> { todo!() }

/// Create a DrawingLineInteractor for an explicit drawing-line store.
pub fn create_retail_kline_drawing_line_interactor_for_store(store_case: DrawingLineStoreCase, max_lines: u32) -> Arc<DrawingLineInteractor> { todo!() }

pub fn create_retail_kline_earnings_event_interactor() -> Arc<KlineEarningsEventInteractor> { todo!() }

pub fn create_retail_kline_eco_calendar_interactor() -> Arc<KlineEcoCalendarInteractor> { todo!() }

pub fn create_retail_kline_event_index_price_interactor() -> Arc<EventIndexPriceInteractor> { todo!() }

pub fn create_retail_kline_event_probability_interactor() -> Arc<EventProbabilityInteractor> { todo!() }

pub fn create_retail_kline_favorite_interactor() -> Arc<KlineFavoriteInteractor> { todo!() }

pub fn create_retail_kline_flash_news_interactor() -> Arc<KlineFlashNewsInteractor> { todo!() }

pub fn create_retail_kline_header_interactor() -> Arc<KlineHeaderInteractor> { todo!() }

pub fn create_retail_kline_init_interactor() -> Arc<KlineInitInteractor> { todo!() }

pub fn create_retail_kline_latest_deal_interactor() -> Arc<LatestDealInteractor> { todo!() }

/// Create a new MarketEventsInteractor instance
pub fn create_retail_kline_market_events_interactor() -> Arc<MarketEventsInteractor> { todo!() }

pub fn create_retail_kline_mcap_v2_interactor() -> Arc<KlineMcapV2Interactor> { todo!() }

pub fn create_retail_kline_more_trade_interactor() -> Arc<MoreTradeInteractor> { todo!() }

/// Create a new `KlineNewCoinInteractor` instance.
pub fn create_retail_kline_new_coin_interactor() -> Arc<KlineNewCoinInteractor> { todo!() }

pub fn create_retail_kline_new_coin_intro_interactor() -> Arc<KlineNewCoinIntroInteractor> { todo!() }

pub fn create_retail_kline_ondo_stock_hours_interactor() -> Arc<KlineOndoStockHoursInteractor> { todo!() }

/// Create a default PriceAlertInteractor (for alert list page without specific instrument)
pub fn create_retail_kline_price_alert_interactor() -> Arc<PriceAlertInteractor> { todo!() }

/// Create a PriceAlertInteractor for CeDeFi tokens
pub fn create_retail_kline_price_alert_interactor_for_cedefi(chain_id: String, token_address: String) -> Arc<PriceAlertInteractor> { todo!() }

/// Create a PriceAlertInteractor for CeFi instruments
pub fn create_retail_kline_price_alert_interactor_for_cefi(inst_id: String, biz_type: String) -> Arc<PriceAlertInteractor> { todo!() }

pub fn create_retail_kline_price_float_window_interactor() -> Arc<PriceFloatWindowInteractor> { todo!() }

pub fn create_retail_kline_price_window_list_interactor() -> Arc<PriceWindowListInteractor> { todo!() }

pub fn create_retail_kline_swiping_title_interactor() -> Arc<KlineSwipingTitleInteractor> { todo!() }

pub fn create_retail_kline_trade_overlay_interactor() -> Arc<KlineTradeOverlayInteractor> { todo!() }

pub fn create_temporary_alert(source_type: RetailKLinePriceAlertSourceType) -> RetailKLinePriceAlert { todo!() }

pub fn create_trading_indicator_interactor() -> Arc<TradingIndicatorInteractor> { todo!() }

/// Legacy compatibility entry point. New surfaces must use
/// `create_indicator_sync_interactor`, whose durable coordinator owns conflict resolution.
pub fn download_indicator_settings(setting_case: ChartSettingCase, time_version: String) -> Arc<OKRustFutureIndicatorSyncDownloadResultWithOKFFIError> { todo!() }

/// Get buy-sell marker data
/// 
/// This function fetches and processes buy-sell marker data for trading analysis,
/// returning a cancellable future that can be integrated with native async systems.
/// 
/// # Parameters
/// - `req`: Buy-sell marker request parameters including instrument ID, time range, and filters
/// 
/// # Returns
/// A cancellable future that resolves to `BuySellMarkerResult`
/// 
/// # Example
/// ```ignore
/// let req = BuySellMarkerReq {
/// inst_id: "BTC-USDT".to_string(),
/// bar: Some("1m".to_string()),
/// type_time: None,
/// after: None,
/// inst_type: "SPOT".to_string(),
/// ord_type: "limit,market,post_only,fok,ioc,optimal_limit_ioc".to_string(),
/// start: "1704067200000".to_string(),
/// end: "1704153600000".to_string(),
/// state: "filled".to_string(),
/// limit: 100,
/// cache_expire_seconds: 60,
/// };
/// let future = get_buy_sell_marker(req);
/// let result = future.await?;
/// ```
pub fn get_buy_sell_marker(req: BuySellMarkerReq) -> Arc<OKRustFutureBuySellMarkerResultWithOKFFIError> { todo!() }

/// Get coin news data
/// 
/// This function fetches and processes coin news content data, returning a cancellable
/// future that can be integrated with native async systems.
/// 
/// # Parameters
/// - `req`: Coin news request parameters including size, token, group, and importance filters
/// 
/// # Returns
/// A cancellable future that resolves to `CoinNewsResult`
/// 
/// # Example
/// ```ignore
/// let req = CoinNewsReq {
/// size: 20,
/// token: "".to_string(),
/// group: "crypto".to_string(),
/// important: "true".to_string(),
/// cache_expire_seconds: 60,
/// };
/// let future = get_coin_news(req);
/// let result = future.await?;
/// ```
pub fn get_coin_news(req: CoinNewsReq) -> Arc<OKRustFutureCoinNewsResultWithOKFFIError> { todo!() }

/// Fetch the TradFi availability categories for one API `ticker`.
/// 
/// `inst_type` is optional (e.g. "SPOT") and follows the other TradFi data
/// endpoints' contract: appended to the request only when non-empty.
/// 
/// A missing record resolves to an all-`false` projection; only transport /
/// server errors reject the future, and the caller degrades those to
/// all-`false` too (silent, no on-screen error).
pub fn get_company_availability(ticker: String, inst_type: Option<String>) -> Arc<OKRustFutureCompanyAvailabilityCategoriesWithOKFFIError> { todo!() }

/// Get economic calendar events
/// 
/// This function fetches and processes economic calendar events data,
/// returning a cancellable future that can be integrated with native async systems.
/// 
/// # Parameters
/// - `req`: Economic calendar request parameters including date range and timezone
/// 
/// # Returns
/// A cancellable future that resolves to `KlineEconEventResult`
pub fn get_economic_calendar_events(req: KlineEconEventReq) -> Arc<OKRustFutureKlineEconEventResultWithOKFFIError> { todo!() }

/// Get market cap v2 trend data.
/// 
/// Fetches and assembles the semantic v2 result for the given request,
/// returning a cancellable future for native async integration. An empty point
/// list resolves as a successful, empty result — never an error.
pub fn get_market_cap_trend_v2(req: MarketCapV2Req) -> Arc<OKRustFutureMarketCapV2ResultWithOKFFIError> { todo!() }

/// Get news carousel data
/// 
/// This function fetches and processes news carousel data including coin news
/// items and campaign items, returning a cancellable future that can be integrated
/// with native async systems.
/// 
/// # Parameters
/// - `req`: News carousel request parameters including location, currency, instrument, and business type filters
/// 
/// # Returns
/// A cancellable future that resolves to `NewsCarouselResult`
/// 
/// # Example
/// ```ignore
/// let req = NewsCarouselReq {
/// location_code: "en-US".to_string(),
/// ccy: "BTC".to_string(),
/// inst_name: "BTC-USDT".to_string(),
/// biz_type: "spot".to_string(),
/// cache_expire_seconds: 60,
/// };
/// let future = get_news_carousel(req);
/// let result = future.await?;
/// ```
pub fn get_news_carousel(req: NewsCarouselReq) -> Arc<OKRustFutureNewsCarouselResultWithOKFFIError> { todo!() }

/// X-perp backfill instrument eligibility for native callers: the underlying
/// index instrumentName (e.g. "BTC-USD") when `inst_id` is an X-perp with a
/// crypto underlying, else null. Toggle-independent — the settings sheet uses
/// it for the "Backfill history" row visibility; the request-time toggle gate
/// lives inside the candle request builders.
pub fn kline_backfill_index_name_for_instrument(inst_type: String, inst_id: String) -> Option<String> { todo!() }

/// Get data type models for base + strategy trading modes.
/// Includes both basic types (open orders, positions) and strategy-specific types (grid, DCA).
/// OC: `[OKTradeDataConfigFactory baseAndStgyDataTypeModelArr]`
pub fn kline_base_and_stgy_data_type_model_arr() -> Vec<KLineTradeDataTypeModel> { todo!() }

/// Get base data type models only (without strategy types).
/// OC: `[OKTradeDataConfigFactory baseDataTypeModelArr]`
pub fn kline_base_data_type_model_arr() -> Vec<KLineTradeDataTypeModel> { todo!() }

/// Sync cache read (Android/OKMarket contract — signature untouched).
pub fn kline_big_data_inst_meta(inst_id: String, inst_type: String) -> KlineBigDataInstMeta { todo!() }

/// Async ensured variant (iOS TradingData container — design §2.1): ensure + load
/// the exact biz, then cache-lookup with a single-instrument hydration fallback.
/// All "not found" shapes return `resolved == false` instead of erroring, so the
/// host's retry/degrade ladder owns the terminal behavior.
pub fn kline_big_data_inst_meta_ensured(inst_id: String, inst_type: String) -> Arc<OKRustFutureKlineBigDataInstMetaWithOKFFIError> { todo!() }

/// Convert a [`KLineBizType`] to its core-trading string representation.
pub fn kline_biz_type_string(biz_type: KLineBizType) -> String { todo!() }

/// Calculate estimated P&L amount from entry price, trigger price, and quantity.
/// 
/// OC: `[biz profitForInstrument:...]` / `OKTCTPSLCalculateTool`.
/// 
/// Formulas:
/// - **Linear**: `pnl = (trigger - entry) * size * ct_val` (long), or `(entry - trigger) * size * ct_val` (short)
/// - **Inverse**: `pnl = size * ct_val * (1/entry - 1/trigger)` (long), or `size * ct_val * (1/trigger - 1/entry)` (short)
/// - **Spot**: `pnl = (trigger - entry) * size` (ct_val = 1)
/// 
/// `ct_val`: contract value (e.g. `"0.01"` for BTC-USDT-SWAP). Use `"1"` for spot.
pub fn kline_calculate_pnl_amount(entry_price: String, trigger_price: String, size: String, ct_val: String, is_long: bool, is_inverse: bool, precision: u32) -> KLineFormattedPrice { todo!() }

/// Calculate P&L rate from entry price and trigger price.
/// 
/// OC: `OKTStopTpSlFuturesUSDTCalculator` / `OKTStopTpSlFuturesCoinCalculator`.
/// 
/// Formulas:
/// - **Linear, Long**: `rate = (trigger/entry - 1) * leverage`
/// - **Linear, Short**: `rate = (1 - trigger/entry) * leverage`
/// - **Inverse, Long**: `rate = (1 - entry/trigger) * leverage`
/// - **Inverse, Short**: `rate = (entry/trigger - 1) * leverage`
/// 
/// Returns rate as decimal string (e.g. `"0.1"` = 10%).
pub fn kline_calculate_pnl_rate(entry_price: String, trigger_price: String, leverage: String, is_long: bool, is_inverse: bool) -> String { todo!() }

/// Calculate stop-loss trigger price from target rate.
/// 
/// OC: `rateWayRateCalculateGetTriggerPrice` in `OKTStopTpSlFutures*Calculator`.
/// 
/// Formulas:
/// - **Linear, Long SL**: `trigger = entry * (1 - rate/lever)`
/// - **Linear, Short SL**: `trigger = entry * (1 + rate/lever)`
/// - **Inverse, Long SL**: `trigger = entry / (1 + rate/lever)`
/// - **Inverse, Short SL**: `trigger = entry / (1 - rate/lever)`
pub fn kline_calculate_sl_price(entry_price: String, rate: String, leverage: String, is_long: bool, is_inverse: bool, precision: u32) -> KLineFormattedPrice { todo!() }

/// Calculate take-profit trigger price from target rate.
/// 
/// OC: `rateWayRateCalculateGetTriggerPrice` in `OKTStopTpSlFutures*Calculator`.
/// 
/// Formulas:
/// - **Linear, Long TP**: `trigger = entry * (1 + rate/lever)`
/// - **Linear, Short TP**: `trigger = entry * (1 - rate/lever)`
/// - **Inverse, Long TP**: `trigger = entry / (1 - rate/lever)`
/// - **Inverse, Short TP**: `trigger = entry / (1 + rate/lever)`
/// 
/// `rate` is a decimal (e.g. `"0.1"` = 10%). `leverage` is e.g. `"10"`.
pub fn kline_calculate_tp_price(entry_price: String, rate: String, leverage: String, is_long: bool, is_inverse: bool, precision: u32) -> KLineFormattedPrice { todo!() }

/// Whether the selected K-line instrument is allowed to show the quick-order entry.
/// 
/// Mirrors V1 `KlineUserExt.canShowQuickOrder(BizInstrument)` for business
/// eligibility only. This synchronous bridge reads current in-memory snapshots;
/// `false` also covers cold caches, so callers must recompute after trade base
/// info or the requested biz line becomes available.
pub fn kline_can_show_quick_order(inst_type: String, inst_id: String) -> bool { todo!() }

/// Compatibility export retained only for trigger-parent attached TP/SL cancellation.
/// Normal parents and standalone OCO orders are owned by OKTrade.
pub fn kline_cancel_single_tpsl(parent: RetailKLineOrder, side: AttachOrderType, target_attach_algo_id: Option<String>) -> Arc<OKRustFutureboolWithOKFFIError> { todo!() }

/// Clear the cached contract user state (e.g. on logout).
pub fn kline_clear_contract_user_state() { todo!() }

/// Compute data-source bit flags for a given biz type.
/// 
/// Composition per biz type:
/// - **Spot (no lever):** Ticker + USD Price + Call Auction + Pre-Pending Order
/// - **Spot (with lever):** + Mark Price
/// - **Margin:** Ticker + Mark Price + USD Price + Call Auction + Pre-Pending Order
/// - **Swap:** Ticker + Mark Price + Open Interest + Pre-Pending Order
/// - **Futures:** Ticker + Mark Price + Open Interest
/// - **Option:** Ticker + Mark Price + Option Summary + Index Price + Estimated Price + OI
/// - **Unknown:** Ticker + Mark Price (safe default)
/// 
/// OC: replaces per-subclass `dataSourceType:` overrides.
pub fn kline_data_source_type_for_biz(biz_type: String, is_support_lever: bool) -> KLineDataSourceType { todo!() }

/// Build a data type model from a type key string. Returns `None` for unknown keys.
/// Useful when reconstructing a model from a persisted or server-provided key.
/// OC: `[OKTradeDataTypeModel modelWithMainDataType:]`
pub fn kline_data_type_model_with_main_data_type(data_type: String) -> Option<KLineTradeDataTypeModel> { todo!() }

/// Divides two decimal strings (`a / b`), returning `"0"` if `b` is zero.
/// 
/// Common use: `openPrice = lastPrice / (changeRatio + 1)`.
/// 
/// OC: `lastPrice.doubleValue / ((change.doubleValue) + 1)`
/// Used by: LiteProvider:1192, Provider:2459/2476
pub fn kline_decimal_divide(a: String, b: String) -> KLineDecimalResult { todo!() }

/// Formats a decimal with max precision, rounding **down** (truncation), trailing zeros removed.
/// E.g. `"1.2300"` with `max_precision=4` → `"1.23"`.
/// 
/// OC: `[[NSDecimalNumber fromPlainString:value] toLocalizedStringDownToMax:precision]`
/// Used by: OptionProvider:60-67/73/81, HalfScreenProvider:335/354
pub fn kline_decimal_format_down_to_max(value: String, max_precision: u32) -> String { todo!() }

/// Formats a decimal with max precision, rounding **half-up** (standard rounding), trailing zeros removed.
/// Uses standard mathematical rounding instead of conservative truncation.
/// 
/// OC: `[[NSDecimalNumber fromPlainString:value] toLocalizedStringRoundToMax:precision]`
/// Used by: TradeDataHandler:95
pub fn kline_decimal_format_round_to_max(value: String, max_precision: u32) -> String { todo!() }

/// Multiplies two decimal strings and returns the exact product.
/// 
/// OC: `[[NSDecimalNumber fromPlainString:a] multiply:[NSDecimalNumber fromPlainString:b]] toPlainString]`
/// Used by: SpotProvider:89, Provider:3380, OptionProvider:107, IndexDataCenter:508
pub fn kline_decimal_multiply(a: String, b: String) -> KLineDecimalResult { todo!() }

/// Subtracts two decimal strings (`a - b`). Commonly used for `change = close - open`.
/// 
/// OC: `[[lastPriceDecimal sub:openDecimal] toPlainString]`
/// Used by: LiteProvider:1195, Provider:2462/2479, IndexDataCenter:423
pub fn kline_decimal_subtract(a: String, b: String) -> KLineDecimalResult { todo!() }

/// Extract the precision digit count from a tick-size / ladder-step string.
/// e.g. `"0.01"` → `2`, `"0.0001"` → `4`, `"1"` → `0`.
/// 
/// OC: `[OKTCDepthUtil extractLadderDigit:]` / `[OKTCDepthUtil precisionDigWithString:]`.
pub fn kline_depth_ladder_digit(step: String) -> u32 { todo!() }

/// Sync helper exposed to native: query the available depth ladders for an
/// instrument before subscribing.
pub fn kline_depth_list_ladders(inst_id: String, inst_type: String) -> Vec<String> { todo!() }

/// Get data type models for DEX + CEX base trading context.
/// OC: `[OKTradeDataConfigFactory dexAndTradeCexBaseDataTypeModelArr]`
pub fn kline_dex_and_trade_cex_base_data_type_model_arr() -> Vec<KLineTradeDataTypeModel> { todo!() }

/// Ensure trade core is initialized (sync, blocking).
/// OC: `[self.trade ensureInitialized]`
/// 
/// **DEPRECATED**: Blocks the calling thread up to 30s. Do NOT call on the main thread.
/// Use `kline_ensure_initialized_future` instead.
pub fn kline_ensure_initialized() -> Result<(), OKFFIError> { todo!() }

/// Ensure trade core is initialized with base info AND strategy data (async).
/// OC: `[self.trade ensureInitializedAndStgy]`
/// 
/// This calls `ensure_initialized_and_stgy` which loads both base info and
/// strategy-related data types (e.g. SettingOfStgy).
pub fn kline_ensure_initialized_and_stgy_future() -> Arc<OKRustFutureboolWithOKFFIError> { todo!() }

/// Ensure trade core is initialized (async variant, preferred for new code).
/// OC: `[self.trade ensureInitialized]`
pub fn kline_ensure_initialized_future() -> Arc<OKRustFutureboolWithOKFFIError> { todo!() }

/// Ensure trade core is initialized with specific data types (async variant).
/// OC: `[self.trade ensureInitializedWithDTModelArr:]`
/// 
/// Unlike `kline_ensure_initialized_future` which only loads base info, this variant
/// accepts a list of data type models to preload specific data (e.g. DEXChainInfo, Coins).
pub fn kline_ensure_initialized_with_dtmodels_future(dt_models: Vec<KLineTradeDataTypeModel>) -> Arc<OKRustFutureboolWithOKFFIError> { todo!() }

/// Fetch public tier info for the given instrument family (async).
/// Uses `mgnMode=cross` as the default (matches native ContractInfoVC usage).
/// OC: `[biz loadTierInfoWithMgnMode:OKTMarginModeCross instFamily:]`
pub fn kline_fetch_public_tiers(biz_type: String, inst_family: String) -> Arc<OKRustFutureKLineTierItemsWithOKFFIError> { todo!() }

/// Fetch risk reserve fund trend items for the given instrument family (async).
/// OC: `[biz loadTrendItemsWithIndexOrCoin:instFamily day:@"7"]`
pub fn kline_fetch_risk_trend_items(biz_type: String, inst_family: String) -> Arc<OKRustFutureKLineRiskTrendItemsWithOKFFIError> { todo!() }

/// Find instrument by ID within a specific business line.
/// 
/// Unlike [`kline_get_instrument_info`] which searches all loaded biz lines
/// (Spot → Margin → Swap → Futures → Option), this function only queries
/// the specified `biz_type`.
/// 
/// OC: `[[self.trade getBiz:bizType] instrumentsById[instId]]`
pub fn kline_find_instrument_in_biz(biz_type: String, inst_id: String) -> Option<KLineInstrumentInfo> { todo!() }

/// Format an asset's balance/quantity using trade precision (base currency).
/// 
/// OC: `[instCtx.tradeConvertor toTargetFormatted:value]`
/// OC default rounding: HalfUp (`OKTFRoundModeRound`), trailing zeros: Remove.
pub fn kline_format_asset_balance(value: String, trade_precision: u32) -> KLineFormattedPrice { todo!() }

/// Format an asset's cost/price using quote precision (quote currency).
/// 
/// OC: `[instCtx.chargeConvertor toQuoteFormated:value].keepZero`
/// OC default rounding: HalfUp, trailing zeros: Keep (via `.keepZero` chain).
pub fn kline_format_asset_quote(value: String, quote_precision: u32) -> KLineFormattedPrice { todo!() }

/// Formats a price change value, prepending `"+"` for positive values.
/// Positive sign is added manually since CTFormatter doesn't support `prepend_plus_sign` yet.
/// 
/// OC: `[chargeConvertor kl_formatGroupZeroPlusSign:value]`
/// Used by: SpotProvider:100, SwapProvider:105, FuturesProvider:74, OptionProvider:126
pub fn kline_format_change_value(value: String, precision: u32) -> KLineFormattedPrice { todo!() }

/// Stateless compact-number formatter. Exposed as a uniffi free function
/// so callers don't need a `KlineFormatter` instance — formatting doesn't
/// read any biz state, only the current locale.
/// 
/// `half_up`: rounding mode for the scaled value. `true` matches V1
/// `OKTFormatter.round()` (HALF_UP); `false` matches V1 default (DOWN).
pub fn kline_format_compact(value: String, half_up: bool) -> CompactValue { todo!() }

/// Formats with grouping separators, removing trailing zeros.
/// Used for volume and open interest display.
/// 
/// NOTE: Implementation is identical to `kline_format_turnover`. Both are kept as separate
/// UniFFI exports because they map to different OC call sites (`tradeConvertor` vs `chargeConvertor`)
/// and may diverge in future formatting requirements.
/// 
/// OC: `[tradeConvertor kl_formatGroup:value]`
/// Used by: OptionProvider:89, HalfScreenProvider:335
pub fn kline_format_group(value: String, precision: u32) -> KLineFormattedPrice { todo!() }

/// Format an order quantity/amount with the instrument's lot_sz precision.
/// 
/// OC: `[OKTradingConvertor convertHYHAmount:order:]`.
/// `with_unit`: if `true`, appends the trade unit (e.g. `"BTC"`, `"Cont"`).
pub fn kline_format_order_amount(amount: String, inst_id: String, with_unit: bool) -> KLineFormattedPrice { todo!() }

/// Format an order price with the instrument's tick_sz precision.
/// 
/// OC: `[OKTradingConvertor convertPrice:order:]`.
/// `side`: `"buy"` → rounds Down, `"sell"` → rounds Up (matches OC `priceRoundMode:`).
pub fn kline_format_order_price(price: String, inst_id: String, side: String) -> KLineFormattedPrice { todo!() }

/// Format a USD value to local fiat currency for order display.
/// 
/// OC: `[OKTradingConvertor convertUsdToLocalCurrency:order:]`.
pub fn kline_format_order_usd_to_fiat(usd_value: String) -> KLineFormattedPrice { todo!() }

/// Format a ratio value as a signed percentage string (e.g. `"+0.82%"`).
/// 
/// Input is a raw ratio such as `"0.0082"` for `0.82%`.
/// Uses HalfUp rounding and keeps fixed trailing zeros at the requested precision.
pub fn kline_format_percentage_with_sign(value: String, precision: u32) -> String { todo!() }

/// Format a P&L value with sign prefix (`+`/`-`).
/// 
/// OC: `[assetConvertor toQuoteFormated:costValue]` then sign check on `.number`.
/// Returns formatted value with `+` prefix for positive, `-` preserved for negative.
pub fn kline_format_pnl_with_sign(value: String, quote_precision: u32) -> KLineFormattedPrice { todo!() }

/// Formats a price with fixed decimal precision, keeping trailing zeros. Rounds down.
/// 
/// OC: `[chargeConvertor kl_formatGroupZero:price]`
/// Used by: SpotProvider:64, LiteProvider:1205, Provider:2386
pub fn kline_format_price(price: String, precision: u32) -> KLineFormattedPrice { todo!() }

/// Like `kline_format_price` but prepends a currency symbol (e.g. `"$ 29,345.10"`).
/// 
/// OC: `[chargeConvertor kl_formatPrefixGroupZero:price]`
/// Used by: SwapProvider:53/69, FuturesProvider:60/91, OptionProvider:115, PIPProvider:348
pub fn kline_format_price_with_prefix(price: String, precision: u32, symbol: String) -> KLineFormattedPrice { todo!() }

/// Format risk reserve fund amount using DigitConvertor.
/// OC: `[[biz createRiskConvertorForInstFamily:] convert:amount]`
/// 
/// Returns `(rounded_value, suffix_value)`. The convertor uses `marginDig` precision
/// from IndexConfig, and `settleCcy` as symbol (or `quoteCcy` for UM contracts).
pub fn kline_format_risk_reserve(biz_type: String, inst_family: String, amount: String) -> KLineFormattedRiskReserve { todo!() }

/// Formats a turnover value, removing trailing zeros for compact display. Rounds down.
/// 
/// OC: `[chargeConvertor toQuoteNumber:value]`
/// Used by: SwapProvider:117/127, FuturesProvider:124/134, Provider:2693/2730
pub fn kline_format_turnover(value: String, precision: u32) -> KLineFormattedPrice { todo!() }

/// Resolve the `alias` field of a FUTURES instrument by inst_id.
/// Returns empty string for non-FUTURES instruments or unknown inst_ids.
/// V1 reference: `getMinSuggestedInstrument(inst_type, inst_id) as? FutureInstrument`
/// in MarketEventsViewModel.getKlineShareLink for futures-share-URL alias mapping.
pub fn kline_futures_alias(inst_id: String) -> String { todo!() }

/// Get a snapshot of all account configuration relevant to K-line display.
/// 
/// OC: Replaces `OKTContractUserState`, `OKTCangModeConfigCenter`,
/// `tradeSetting.acctLv`, `tradeSetting.posMode`, `tradeSetting.pnlPriceType`.
/// 
/// # Known Limitation: Cooling Period (core-trading gap)
/// 
/// OC's `OKTContractUserState.isDuringCoolingPeriod` is a separate state
/// that temporarily blocks reverse-position operations after contract type
/// switch. **core-trading does not expose a cooling-period API.** The field
/// `is_account_level_support_reverse` uses `get_reverse_pos_enable()` as an
/// approximation, which checks account-level permission but does NOT check
/// the transient cooling-period state. This means K-line may show the
/// reverse button during cooling period when it should be hidden.
/// 
/// **TODO**: Once core-trading adds a `is_during_cooling_period()` API,
/// combine it: `supports_reverse = get_reverse_pos_enable() && !is_during_cooling_period()`.
pub fn kline_get_account_config() -> KLineAccountConfig { todo!() }

/// Get the account level string.
/// Values: `"1"` (Simple), `"2"` (SCC), `"3"` (MCC), `"4"` (PM).
/// OC: `tradeSetting.acctLv`
pub fn kline_get_account_level() -> String { todo!() }

/// Get amount precision digit for an instrument family.
/// OC: `[biz getAmountDigit:instFamily]`.
pub fn kline_get_amount_digit(biz_type: String, inst_family: String) -> String { todo!() }

/// Get amount precision digit by unit type for an instrument family.
/// OC: `[biz getAmountDigitByUnit:instFamily unitType:]`.
pub fn kline_get_amount_digit_by_unit(biz_type: String, inst_family: String, unit_type: String) -> String { todo!() }

/// Get amount display symbol for an instrument.
/// OC: `[bizHandler amountDisplaySymbol:instrument]`.
pub fn kline_get_amount_display_symbol(inst_id: String) -> String { todo!() }

/// Get the charge currency config for a given inst-family.
/// 
/// OC: `[biz getChargeCurrencyForInstFamily:instFamily]` — returns the
/// quote/charge currency metadata (sign, display name, EEA display name,
/// etc.) used by K-line price labels.
/// 
/// Returns `None` when:
/// - `biz_type` is unknown or its biz is not initialised,
/// - `inst_family` is empty or unknown.
/// 
/// Delegates to `OkBizTrait::get_charge_currency_for_inst_family(Some(&inst_family))`
/// and converts to the FFI-friendly [`ChargeCurrencyRecord`].
pub fn kline_get_charge_currency_for_inst_family(biz_type: String, inst_family: String) -> Option<ChargeCurrencyRecord> { todo!() }

/// Get the price-digit precision for the charge currency of a given instrument family.
/// Used primarily by Option K-line for index price display.
/// 
/// Lookup order: Option biz → Futures → Swap → Margin → Spot → `DEFAULT_PRICE_DIGITS`.
/// Synchronous & best-effort — only reads from already-loaded biz caches.
/// 
/// OC: `[self.biz getChargeCurrencyForInstFamily:...].priceDig`.
pub fn kline_get_charge_currency_price_dig(inst_id: String, inst_family: String) -> u32 { todo!() }

/// Charge-currency price precision resolved through the instrument's own biz line
/// (price-alert handler semantics, `OKKPriceAlertCeFiHandlerRust.priceDigit()`).
/// OC parity target (post TEPT-6424):
/// - swap/futures: `createChargeConvertorNegativeFixUSDWithInstFamily.quotePrecision`
/// (currency fixed to USD — inverse families store per-ccy composite keys)
/// - others: `createChargeConvertorWithInstFamily.quotePrecision`
/// 
/// Unlike [`kline_get_charge_currency_price_dig`] (fixed Option→Futures→Swap→Margin→Spot
/// scan serving the Option K-line index price), this resolves by `biz_type`, so
/// same-named inst families across biz lines (spot "BTC-USDT" vs swap "BTC-USDT")
/// cannot cross-contaminate.
/// 
/// Fallback matrix mirrors OC exactly — deliberately NOT `get_price_digit()`, whose
/// built-in fallbacks (negative swap → 2, spot → 8) diverge from OC:
/// - biz not loaded → 0 (OC handler: `biz == nil` → `?? 0`)
/// - swap/futures/option config missing → 8 (`kOKTContractDefaultPriceDig`,
/// `OKTBiz.m:1312-1321/1352-1356`, both positive & negative branches)
/// - spot/margin config missing → 0 (`cc.priceDig.integerValue` on nil cc,
/// `OKTBiz.m:1549-1555`, no default protection)
/// 
/// `inst_family` may be empty for spot/margin call sites (OC passes
/// `instrument.instFamily`, which for spot equals `inst_id`); falls back to `inst_id`.
pub fn kline_get_charge_currency_price_dig_for_biz(biz_type: String, inst_id: String, inst_family: String) -> u32 { todo!() }

/// Get the display name of a coin by its symbol (e.g. `"BTC"` → `"Bitcoin"`).
/// OC: `instrument.coinName` / `[OKTrade imageURLForCoin:]`.
/// Returns empty string if the coin is not found.
pub fn kline_get_coin_name(symbol: String) -> String { todo!() }

/// Get the coin icon URL by symbol (e.g. `"BTC"` → `"https://..."`).
/// OC: `[trade imageURLForCoin:]` — coin-cache url first; a miss falls back to the
/// asset-team CDN template (premarket coins are absent from public/coins). Native
/// prefixes a dynamic accelerator domain when available; Rust has no accelerator
/// facility, so it uses the canonical static domain (native's own final fallback).
pub fn kline_get_coin_url(symbol: String) -> String { todo!() }

/// Get contract digit (contract-to-coin conversion precision) for an instrument family.
/// OC: `[[biz getTradeConfig:instFamily] contractDig]`.
/// Different from `kline_get_amount_digit` which returns `baseDig`.
pub fn kline_get_contract_digit(biz_type: String, inst_family: String) -> String { todo!() }

/// Get countdown model for a specific instrument (new coin listing).
/// OC: `[trade countDownModelWithInstID:]`.
/// Returns `None` if no countdown data or instrument not found.
pub fn kline_get_countdown_model(inst_id: String) -> Option<KLineCountdownModel> { todo!() }

/// Get the user's fiat currency ISO code (e.g. "USD", "CNY"). Falls back to "USD".
/// OC: `[OKTrade sharedTrade].currencyDataHandler.currentModel.isoCode`.
pub fn kline_get_currency_iso_code() -> String { todo!() }

/// Get the fiat currency display symbol (e.g. `"$"`, `"¥"`, `"₺"`).
/// OC: `currencyDataHandler.currentModel.symbol`.
pub fn kline_get_currency_symbol() -> String { todo!() }

/// Get the list of available depth precision levels for an instrument.
/// e.g. `["0.01", "0.1", "1", "10"]` for BTC-USDT.
/// 
/// OC: `[self.biz getDepthLevelListWithInstrument:]`.
/// Returns empty vector if the biz is not loaded or instrument not found.
pub fn kline_get_depth_level_list(inst_id: String) -> Vec<String> { todo!() }

/// Get the exchange rate from USD to the user's selected fiat currency.
/// OC: `currencyDataHandler.currentModel.exchangeRate`.
pub fn kline_get_exchange_rate() -> String { todo!() }

/// Get the first trade time (timestamp string, used for "new user" badge).
/// OC: `tradeSetting.firstTradeTime`
pub fn kline_get_first_trade_time() -> String { todo!() }

/// Get option greek unit: `"0"` = PA (Price Adjusted), `"1"` = BS (Black-Scholes).
/// OC: `context.trade.tradeSetting.greekUnit`
pub fn kline_get_greek_unit() -> String { todo!() }

/// Get the list of head-pop items for the biz line switcher.
/// 
/// OC: Replaces `[OKTHeadTradePopDataSource getTopModelArray]` +
/// `[OKTHeadTradePopDataSource getBottomModelArray]`.
/// 
/// `current_inst_id`: The currently displayed instrument ID (for `is_selected` marking).
/// `current_biz_type`: The currently selected biz type.
/// 
/// Returns a list of available biz lines. Each item indicates whether it's
/// selected, available, and what instrument to navigate to.
pub fn kline_get_head_pop_items(current_inst_id: String, current_biz_type: String) -> Vec<KLineHeadPopItem> { todo!() }

/// Primary instrument query for K-line. Returns a flat [`KLineInstrumentInfo`]
/// replacing all scattered `self.instrument.xxx` reads across 11 OC provider files.
/// 
/// Returns `None` if the instrument isn't found (biz not yet loaded).
/// 
/// **Note:** instrument-owned symbols (including `quote_symbol`) are populated
/// synchronously here. Context-dependent amount units still refresh through
/// `KLineContextBridge`. `quote_precision` and `trade_precision` are derived
/// from `tick_sz` / `lot_sz` when available, falling back to
/// `DEFAULT_PRICE_DIGITS` (1) when ticker_size is empty.
pub fn kline_get_instrument_info(inst_id: String) -> Option<KLineInstrumentInfo> { todo!() }

/// Get the list of all instruments for a given biz type.
/// 
/// OC: `biz.instruments` / `[biz instrumentsWithInstFamily:]`.
/// 
/// Returns a lightweight list. Call `kline_get_instrument_info(inst_id)`
/// for full details on a specific instrument.
pub fn kline_get_instrument_list(biz_type: String) -> Vec<KLineInstrumentBrief> { todo!() }

/// Get instruments filtered by inst_family within a biz type.
/// OC: `[biz instrumentsWithInstFamily:]`.
pub fn kline_get_instruments_by_family(biz_type: String, inst_family: String) -> Vec<KLineInstrumentBrief> { todo!() }

/// Get the position mode string: `"long_short_mode"` (hedge) or `"net_mode"` (one-way).
/// OC: `OKTCangModeConfigCenter.posMode`
pub fn kline_get_position_mode() -> String { todo!() }

/// Get pre-market instruments filtered by inst_family within a biz type.
/// OC: `[biz preMarketInstrumentsWithInstFamily:]`.
/// Only applicable to Futures/Swap; returns empty for Spot/Margin/Option.
pub fn kline_get_pre_market_instruments_by_family(biz_type: String, inst_family: String) -> Vec<KLineInstrumentBrief> { todo!() }

/// Get price precision digit for an instrument family.
/// OC: `[biz getPriceDigit:instFamily]` / `[bizHandler decimalNumOfDepthAmount]`.
/// Returns the digit count as string (e.g. `"2"`), or empty if not found.
pub fn kline_get_price_digit(biz_type: String, inst_family: String) -> String { todo!() }

/// Get price display symbol for an instrument.
/// OC: `[bizHandler priceDisplaySymbol:instrument]`.
/// Returns e.g. `"USDT"`, `"USD"`, or empty if not found.
pub fn kline_get_price_display_symbol(inst_id: String) -> String { todo!() }

/// Get sheet digit for an instrument family (used for tier max amount formatting).
/// OC: `[[biz getTradeConfig:instFamily] sheetDig]`.
pub fn kline_get_sheet_digit(biz_type: String, inst_family: String) -> String { todo!() }

/// Get the trade (index) config for a given inst-family.
/// 
/// OC: `[biz getTradeConfig:instFamily]` — returns precision/config block
/// used by K-line for amount rendering, fee labels, etc.
/// 
/// Returns `None` when:
/// - `biz_type` is unknown or its biz is not initialised,
/// - `inst_family` is empty or unknown.
/// 
/// Delegates to `OkBizTrait::get_trade_config(Some(&inst_family))` and
/// converts the internal `IndexConfig` to the FFI-friendly
/// [`IndexConfigRecord`] (already `uniffi::Record`).
pub fn kline_get_trade_config(biz_type: String, inst_family: String) -> Option<IndexConfigRecord> { todo!() }

/// Get a full snapshot of the current trade settings (UI preferences).
/// OC: `self.trade.tradeSetting`
pub fn kline_get_trade_setting() -> KLineTradeSetting { todo!() }

/// Get the trade unit string based on contract direction.
/// `is_negative`: `true` → inverse (coin-margined) unit, `false` → linear (USDT-margined) unit.
/// OC: `tradeSetting.linearTradeUnit` / `tradeSetting.invertTradeUnit`
pub fn kline_get_trade_unit(is_negative: bool) -> String { todo!() }

/// Batch-converts TradingView candles from USD to local fiat for index price mode.
/// 
/// **Precision selection:** The last candle's close price is used as the reference
/// for auto-precision, ensuring stable decimal places across the series.
/// 
/// `time` and `volume` are passed through unchanged (not price fields).
/// 
/// OC: Replaces `createUSDChargeConvertor` + per-candle `convert:` loop pattern.
pub fn kline_index_convert_trading_view_candles(candles: Vec<KLineTVCandle>) -> Vec<KLineTVCandle> { todo!() }

/// Whether `inst_id`'s inst-family quote ccy is `USD_UM` (US-region UM contract).
/// 
/// OC: `[inst isInstFamilyQuoteCcyBelongUS_UM]`. Returns `false` when the biz
/// isn't loaded or the inst isn't found. Matches the `== "USD_UM"` convention
/// already used in `biz_query_bridge`.
pub fn kline_inst_family_quote_ccy_belong_us_um(inst_id: String) -> bool { todo!() }

/// Whether `inst_id` is a SPOT instrument that supports margin trading.
/// 
/// Mirrors V1 `(trade.getBiz(SPOT) as? ISpotBiz)?.isSupportMargin(instId)`
/// — `false` when biz isn't loaded or the inst isn't in the SPOT cache.
pub fn kline_inst_supports_margin(inst_id: String) -> bool { todo!() }

/// Zero-fee badge flag (entity-level: maker & taker fees both "0").
/// Returns `false` when the biz isn't loaded or the inst isn't found.
/// 
/// OC: `instrument.zeroFee` (numeric-zero compare; the entity feed emits the
/// literal "0", so the canonical string compare matches cross-platform).
pub fn kline_instrument_is_zero_fee(biz_type: String, inst_id: String) -> bool { todo!() }

pub fn kline_introduce_tab_meta(inst_id: String, inst_type: String) -> KlineIntroduceTabMeta { todo!() }

/// Formats a change value for inverse contracts: converts USD to fiat,
/// keeps trailing zeros, and prepends `"+"` for positive values.
/// 
/// OC: `[[[[context.usdConvertor quoteFormatterForPrice:] keepZero] prependPlusSign] toLocalizedString]`
/// Used by: SwapProvider:101, FuturesProvider:70
pub fn kline_inverse_format_change(value: String) -> KLineFormattedPrice { todo!() }

/// Formats a turnover value for inverse contracts: converts USD to fiat, removes trailing zeros.
/// 
/// OC: `[context.usdConvertor toQuoteNumber:value]`
/// Used by: SwapProvider:129, FuturesProvider:136, OptionProvider:131
pub fn kline_inverse_format_turnover(value: String) -> KLineFormattedPrice { todo!() }

/// Formats an inverse-contract (coin-margined) price: converts raw USD → user's
/// current fiat and rounds to the per-(inst_family, iso_code) `priceDig`.
/// 
/// OC: `[instCtx.chargeConvertor toQuoteFormated:price].keepZero` where the convertor
/// is `OKTNegativeChargeConvertor` (`OKTBiz.m:1308-1338`): quotePrecision =
/// `ChargeCurrency.priceDig` (config miss → 8), multiplier = glb exchange rate
/// (standard rate, NOT the premium-adjusted one), no currency-sign prefix.
/// Used by: price-alert edit page (`OKKPAPriceInputContainerRust`).
/// 
/// Deliberately NOT `kline_usd_to_fiat_auto` (auto-precision by magnitude + premium
/// rate — a different OC convertor) and NOT `InverseToFiatChargeConvertor::
/// from_inst_family_config` as-is (its config-miss fallback is the global
/// `currency.precision`, OC's is 8). Thousands separator is native's responsibility.
pub fn kline_inverse_price_format(price: String, biz_type: String, inst_family: String) -> KLineFormattedPrice { todo!() }

/// Reverse of [`kline_inverse_price_format`]: converts an inverse-contract fiat
/// display price back to the raw USD quote (display ÷ glb exchange rate), then
/// TRUNCATES to the USD-side priceDig.
/// 
/// OC: `[chargeConvertor toBase:]` on `OKTNegativeChargeConvertor` = division +
/// `toFixed:basePrecision` (NSRoundDown), where basePrecision = usdPrecision
/// (`createNegativeChargeConvertorWithInstFamily`, config-miss default 8).
/// The truncation is load-bearing: the chart drag path writes an axis-precision
/// display value into the threshold — without this clamp the unrounded tail is
/// uploaded verbatim (observed: `64743.255615234375` stored server-side; A-path
/// is protected by the same clamp inside `toBase:`, TEPT-6424).
/// 
/// Contract: call ONLY for inverse (coin-margined) instruments — the caller gates
/// on `is_negative` (linear/spot display price IS the USD quote, identity).
/// Rate <= 0 (config not loaded) returns the truncated input — never divides by zero.
pub fn kline_inverse_price_reverse(price: String, biz_type: String, inst_family: String) -> String { todo!() }

/// Check if a business line is initialized (instruments loaded).
/// OC: `[biz isInitialized]`.
pub fn kline_is_biz_initialized(biz_type: String) -> bool { todo!() }

/// Check if the current fiat currency is USD.
/// OC: `currencyDataHandler.currentIsUSD`.
pub fn kline_is_current_currency_usd() -> bool { todo!() }

/// Check whether `inst_type` is a derivative (Swap, Futures, or Option).
/// OC: replaces `!isSpotOrMargin(bizType)`.
pub fn kline_is_derivative(inst_type: String) -> bool { todo!() }

/// Check whether the given coin is currently inside its cooling period.
/// 
/// Returns `false` when:
/// - No state has been pushed yet (native hasn't called
/// [`kline_update_contract_user_state`]),
/// - Cooling remind is off (`cooling_remind != "1"`),
/// - `now` is outside `[cooling_start_time, cooling_end_time)`,
/// - `ccy` is not in `coin_scope` (when scope is non-empty).
/// 
/// OC: `[OKTContractUserState isDuringCoolingPeriodWithCcy:ccy]`.
/// Delegates to
/// `core_trading::business::calm_period::ContractUserState::is_during_cooling_period_with_ccy`.
pub fn kline_is_during_cooling_period(ccy: String) -> bool { todo!() }

/// Check whether `inst_type` is Futures.
/// OC: replaces `bizType == OKTBizTypeFutures`.
pub fn kline_is_futures(inst_type: String) -> bool { todo!() }

/// Whether a managed (TM) sub-account needs to hide current positions.
/// OC: `tradeSetting.isTmSubAccountNeedHiddenCurPos`
/// 
/// The historical "Rust true / native false" divergence for normal accounts was
/// a missing `hosting_status` guard in core-trading, fixed in `7ad83b7cb2`.
pub fn kline_is_hidden_cur_pos() -> bool { todo!() }

/// Whether a managed (TM) sub-account needs to hide transaction records.
/// OC: `tradeSetting.isTmSubAccountNeedHiddenTransaction`
pub fn kline_is_hidden_transaction() -> bool { todo!() }

/// Check if trade base info has been initialized (sync).
/// OC: `[self.trade isInitializedBaseInfo]`
/// 
/// Returns `true` if the account config has been loaded (i.e., trade_setting
/// has data), which is the primary indicator that base info initialization
/// has completed.
pub fn kline_is_initialized_base_info() -> bool { todo!() }

/// Check if K-line display is enabled for the trade panel.
/// OC: `tradeSetting.isKLineEnabled`
pub fn kline_is_kline_enabled() -> bool { todo!() }

/// Check if the account is MCC (Multi-Currency Cross) or PM (Portfolio Margin).
/// OC: `tradeSetting.isMCCOrPMAccount` (acctLv == "3" || acctLv == "4").
/// Used in ~10 files for UI visibility: quick order, position display, navigation.
pub fn kline_is_mcc_or_pm_account() -> bool { todo!() }

/// Check whether position mode is net (one-way) mode.
/// OC: `tradeSetting.isNetMode`
pub fn kline_is_net_position_mode() -> bool { todo!() }

/// Check whether `inst_type` is Option.
/// OC: replaces `bizType == OKTBizTypeOption`.
pub fn kline_is_option(inst_type: String) -> bool { todo!() }

/// Whether the current account is a PM (Portfolio Margin, lv4) or Trade (lv1) account.
/// 
/// OC: `acctLv == OKTAccountLevelTypePMC || acctLv == OKTAccountLevelTypeTrade`
/// (`is_pm_account` == iOS `isPMAccount`, `is_trade_account` == iOS `isTradeAccount`).
/// Unloaded config → level defaults to lv1 → returns `true`.
pub fn kline_is_pm_or_trade_account() -> bool { todo!() }

/// Check if PnL display uses last price (vs mark price).
/// OC: `tradeSetting.isLastPriceTypePositionPnl`
/// 
/// Semantic complement of [`kline_is_pnl_mark_price`]; both are exposed
/// because OC code reads either directly. core-trading provides
/// `is_pnl_price_type_last()` on `trade_setting()`; both flags are
/// independent (a third price type can be active and make both `false`).
pub fn kline_is_pnl_last_price() -> bool { todo!() }

/// Check if PnL display uses mark price (vs last price).
/// OC: `tradeSetting.isPnlPriceTypeMark`
pub fn kline_is_pnl_mark_price() -> bool { todo!() }

/// Check whether the account level supports portfolio margin (level 4).
/// OC: `OKTAccountLevelType == OKTAccountLevelTypePMC`
pub fn kline_is_portfolio_margin() -> bool { todo!() }

/// Check if the account is SCC (Single-Currency Cross, level 2).
/// OC: `tradeSetting.isSCCAccount`
pub fn kline_is_scc_account() -> bool { todo!() }

/// Check whether the app is in simulator (demo) trading mode.
/// OC: replaces `OKBTSCIsSimulatorTrading`.
pub fn kline_is_simulator_trading() -> bool { todo!() }

/// Check whether `inst_type` is Spot.
/// OC: replaces `bizType == OKTBizTypeSpot`.
pub fn kline_is_spot(inst_type: String) -> bool { todo!() }

/// Check whether `inst_type` is Spot or Margin.
/// OC: replaces `isSpotOrMargin(bizType)`.
pub fn kline_is_spot_or_margin(inst_type: String) -> bool { todo!() }

/// Whether the trade entity supports all five business lines
/// (spot / margin / swap / futures / option).
/// 
/// OC: `[OKTrade isSupportAllBizTypes]` (OKTrade+restriction) —
/// `supportedBizTypeSet` contains all five `OKTBizType` tokens. The Rust set
/// (`InstGroupData.supported_biz_type_set`, built from the inst-group
/// `available_type[].inst_type`) carries the same wire tokens, so membership
/// checks are 1:1.
/// 
/// Feeds the session-global `trade_facts.is_support_all_biz_types` (newcomer
/// guide gate, `derived_enable_guide`) via retail-kline's trade-facts sync;
/// also exported over FFI as a general-purpose query.
/// 
/// # Not-yet-loaded behavior
/// Inst-group data is part of `ensure_initialized_base_info`'s base data
/// types; before it loads this returns `false`. That matches the native call
/// site (`trade?.isSupportAllBizTypes() ?? false` → guide hidden until trade
/// core is ready). The trade-facts sync re-pushes on kline-init success and
/// on settings-change broadcasts, which un-hides the guide.
pub fn kline_is_support_all_biz_types() -> bool { todo!() }

/// Check whether `inst_type` is Swap (perpetual).
/// OC: replaces `bizType == OKTBizTypeSwap`.
pub fn kline_is_swap(inst_type: String) -> bool { todo!() }

/// Check whether `inst_type` is Swap or Futures.
/// OC: replaces `isSwapOrFutures(bizType)`.
pub fn kline_is_swap_or_futures(inst_type: String) -> bool { todo!() }

/// TradFi (US-stock / RWA) flag for a coin by trade symbol (base ccy, e.g. "AAPL").
/// Mirrors native `getTradeCoinInfo(tradeSymbol).tradFi`; unloaded table / unknown coin /
/// absent flag all degrade to `false` (native nil-read parity).
pub fn kline_is_trade_fi_coin(trade_symbol: String) -> bool { todo!() }

/// Load all business lines concurrently (async).
/// OC: `[self.trade loadAllBizs]`
/// 
/// Loads Spot, Margin, Swap, Futures, and Option biz concurrently via
/// `CTRSTrade::load_all_biz()`.
pub fn kline_load_all_bizs_future() -> Arc<OKRustFutureboolWithOKFFIError> { todo!() }

/// Load the business line for the given biz type (sync, blocking).
/// OC: `[self.trade loadBiz:self.instrument.bizType]`
/// 
/// **DEPRECATED**: Blocks the calling thread up to 30s. Do NOT call on the main thread.
/// Use `kline_load_biz_future` instead.
pub fn kline_load_biz(biz_type: String) -> Result<(), OKFFIError> { todo!() }

/// Load the business line (async variant, preferred for new code).
/// OC: `[self.trade loadBiz:]`
pub fn kline_load_biz_future(biz_type: String) -> Arc<OKRustFutureboolWithOKFFIError> { todo!() }

/// Preflight validation: init → parse biz → load biz → validate instrument (sync, blocking).
/// Does **NOT** create an `InstContext` — use `KLineContextBridge.start()` for that.
/// OC: `[self.trade loadContextWithInstrument:instrument dataSourceType:type]`
/// 
/// `ds_type` is kept for API compatibility but not used here.
/// 
/// **DEPRECATED**: Blocks the calling thread up to 30s. Do NOT call on the main thread.
/// Use `kline_load_context_future` instead.
pub fn kline_load_context(inst_id: String, biz_type: String, ds_type: u64) -> Result<(), OKFFIError> { todo!() }

/// Preflight validation (async variant, preferred for new code).
/// OC: `[self.trade loadContextWithInstrument:dataSourceType:]`
/// 
/// `ds_type` is kept for API compatibility but not used here.
pub fn kline_load_context_future(inst_id: String, biz_type: String, ds_type: u64) -> Arc<OKRustFutureboolWithOKFFIError> { todo!() }

/// USD → user local-currency rate (raw `usdToThisRate` field of the active
/// currency preference). Defaults to `"1"` when settings hasn't loaded a
/// currency yet (matches V1's `nativeRateDefault`).
/// 
/// **Does not consult premium-rate fields.** TRY-style premium users see the
/// same rate V1 shows; the inverse-contract futures-basis chart historically
/// depends on this exact path. The premium-aware convertor is reachable via
/// `build_usd_to_fiat_convertor` for callers that want it.
pub fn kline_native_usd_to_local_rate_raw() -> String { todo!() }

/// Determine the rounding mode for an order price.
/// 
/// OC: `[OKTCOrderUtil priceRoundMode:order:]`.
/// 
/// - Buy-side: round **up** (pay more, conservative for buyer).
/// - Sell-side: round **down** (receive less, conservative for seller).
/// 
/// Returns `"up"` or `"down"`.
pub fn kline_order_price_round_mode(is_buy: bool) -> String { todo!() }

/// Returns value-only write gates for a frozen K-line order snapshot.
/// 
/// This additive API keeps the released `RetailKLineOrder` wire layout intact.
pub fn kline_order_write_capabilities(parent: RetailKLineOrder) -> KLineOrderWriteCapabilities { todo!() }

/// General title = V1 `getTitleByIdAndType(instId, instType)` with default
/// args. Used by new-coin / compare / market-events. SWAP appends the
/// management-label cdkey; everything else uses the default options (short swap
/// suffix, read only by the SWAP / X-Perps branches — FUTURES expiry ignores
/// it).
pub fn kline_plain_title(inst_id: String, inst_type: String) -> CTTradeTitle { todo!() }

/// Prepares both K-line order-amend consumers from one fresh typed snapshot.
/// 
/// Native passes only semantic intent and a new price. Rust owns raw-data
/// parsing, identity checks, leg mutation and DTO assembly; no request is
/// submitted here, so the existing core-trading interactor keeps its warning,
/// proposed-price and submit lifecycle.
pub fn kline_prepare_order_amend(parent: RetailKLineOrder, action: KLineOrderAmendAction, new_price: String) -> Result<KLinePreparedOrderAmend, OKFFIError> { todo!() }

/// Validates a fresh chart order and resolves its typed owner handoff identity.
/// Native uses that identity to call the existing trade cancellation owner.
pub fn kline_prepare_order_cancel(parent: RetailKLineOrder) -> Result<KLinePreparedOrderCancel, OKFFIError> { todo!() }

/// Port of V1 `PrecisionUtils.recommendPrecision(Double): Int`. Returns the
/// recommended fractional-digit count for a numeric value based on its
/// magnitude. Used by chart marker views (e.g. futures-basis hover) to keep
/// the displayed price compact as the value grows.
/// 
/// Buckets (V1 source of truth):
/// - `>= 1,000,000` → 0
/// - `>= 100,000`   → 1
/// - `>= 10,000`    → 2
/// - `>= 1,000`     → 2
/// - `>= 100`       → 2
/// - `>= 10`        → 4
/// - `>= 1`         → 4
/// - `< 1` → V1 `calPrecisionByEffectiveDigit(value, 5)` =
/// `BigDecimal(v).round(MathContext(5, HALF_UP)).scale()`.
/// Mathematically equivalent to `max(0, 5 - 1 - floor(log10(|v|)))`.
pub fn kline_recommend_price_precision(value: f64) -> u32 { todo!() }

/// K-line header/ribbon title (V1 `simpleSwap = true` / `swapWithSuffix = false`):
/// the header renders its own contract-type tag next to the title, so the title
/// itself carries no type suffix. Same formatter/options as the swiping-list
/// `ribbon_title`, so a pre-VO placeholder built from this never flashes to a
/// different string once the ribbon VOs land.
pub fn kline_ribbon_title(inst_id: String, inst_type: String) -> CTTradeTitle { todo!() }

/// Landscape standalone title = V1 `setupCoinName`: the title carries its own
/// type suffix (no separate header type tag). INDEX → raw inst_id; SWAP → base
/// + management label; everything else uses a `None` swap suffix so an X-Perps
/// contract shows the bare name (V1 `simpleSwap = true`), a non-X-Perps future
/// still shows its expiry (the Normal rule ignores the suffix), and pre-market
/// / SPOT route through the formatter's own branches (SPOT uses the normalized
/// display symbols).
pub fn kline_standalone_title(inst_id: String, inst_type: String) -> CTTradeTitle { todo!() }

/// Get all supported trade data type keys as strings.
/// OC: `[OKTradeDataConfigFactory supportedDataTypeArr]`
pub fn kline_supported_data_type_arr() -> Vec<String> { todo!() }

/// Check whether the account supports reverse (close) positions.
/// When `false`, position lines on K-line should not show reverse button.
/// 
/// OC: `!OKTContractUserState.isDuringCoolingPeriod && acctLv supports reverse`
/// 
/// # Known Limitation (core-trading gap)
/// 
/// Only checks `get_reverse_pos_enable()` (account-level permission).
/// Does NOT check `OKTContractUserState.isDuringCoolingPeriod` because
/// core-trading has no cooling-period API. See `kline_get_account_config()` doc.
pub fn kline_supports_reverse_position() -> bool { todo!() }

/// Check whether the reverse-position button should be enabled for the
/// given coin, combining account-level permission with the cooling-period
/// guard.
/// 
/// OC: `!OKTContractUserState.isDuringCoolingPeriod && acctLv supports reverse`
/// 
/// This is the cooling-period-aware variant of
/// [`kline_supports_reverse_position`]; the original is kept for callers
/// that do not have a coin context.
pub fn kline_supports_reverse_position_for_ccy(ccy: String) -> bool { todo!() }

/// Push the latest `ContractUserState` from native into the bridge.
/// 
/// Call this whenever OC receives an updated `OKTContractUserState`
/// (e.g. on trade-setting refresh or contract-mode switch). All fields
/// mirror the OC `OKTContractUserState` properties as strings, matching
/// the server-side wire format.
/// 
/// Pass empty strings for unknown numeric fields (they will be treated
/// as `0` by the underlying parser).
/// 
/// OC: native side already maintains `OKTContractUserState`; this just
/// forwards the same data to Rust.
pub fn kline_update_contract_user_state(cooling_remind: String, cooling_start_time: String, cooling_end_time: String, cooling_period: String, silent_end_time: String, ts: String, max_coin_size: String, coin_scope: String) { todo!() }

/// Converts a USD price to local fiat with auto-determined precision.
/// Precision is chosen based on the converted value's magnitude.
/// 
/// OC: `[[OKTUSDFiatChargeConvertor alloc] initWithUSDPrice:price]`
/// Used by: SpotProvider:70, SwapProvider:45, FuturesProvider:52, OptionProvider:93,
/// LiteProvider:1227, IndexDataCenter:497
/// 
/// # TRY-style premium rate
/// 
/// OC's `OKTUSDFiatChargeConvertor` applies a per-currency premium rate when
/// the user's fiat is flagged premium (e.g. Turkish Lira's
/// `usdToThisRatePremium`). core-trading's
/// `USDToFiatAutoPrecisionConvertor::from_global_config` only reads
/// `exchange_rate`, so this bridge constructs the convertor through
/// [`super::lifecycle::build_usd_to_fiat_convertor`], which picks
/// `usd_to_this_rate_premium` when `is_premium == true`. Output matches the
/// OC path for premium currencies.
/// 
/// # Known Limitations
/// 
/// **`isOldExchangeRate`**: OC supports a legacy exchange rate fallback via
/// `initWithUSDPrice:isOldExchangeRate:`. This is **not** supported in the
/// Rust convertor. Only the current exchange rate is used.
pub fn kline_usd_to_fiat_auto(usd_price: String) -> KLineFormattedPrice { todo!() }

/// Converts USD to fiat with `"≈"` prefix (e.g. `"≈¥203,456.78"`).
/// Returns plain `String` since the caller only needs the display string.
/// 
/// OC: `[usdFiatConvertor toQuoteStandard:price]`
/// Used by: SpotProvider:84/91, FuturesProvider:102
pub fn kline_usd_to_fiat_auto_standard(usd_price: String) -> String { todo!() }

/// Converts USD to fiat with currency symbol as prefix, keeping trailing zeros.
/// 
/// OC: `[usdFiatConvertor kl_formatPrefixGroupZero:price]`
/// Used by: SwapProvider:80, OptionProvider:109
pub fn kline_usd_to_fiat_auto_with_prefix(usd_price: String) -> KLineFormattedPrice { todo!() }

/// Map the backend `method` field straight to the chart contract kind.
/// 
/// Vocabulary and strict parsing are delegated to retail-trading's
/// [`EventContractMethod::parse_method`] (the method type's home) — this
/// function only translates that domain enum into the chart enum, so the
/// wire vocabulary exists in exactly one place. Returns `None` for unknown
/// method strings; native callers should alert in debug and fall back
/// conservatively (15-minute UpDown) instead of mislabelling.
pub fn parse_event_chart_contract_kind(method: String) -> Option<EventChartContractKind> { todo!() }

/// Register the global native API implementation
/// 
/// Called from the native platform (Android/iOS) during app initialization.
/// If called multiple times, only the first call succeeds.
pub fn register_retail_kline_global_native_api(api: Arc<dyn RetailKlineNativeApi>) { todo!() }

pub fn resolve_retail_kline_company_entry_policy(subject: CompanySubject) -> CompanyEntryPolicy { todo!() }

/// Hands logged-out CeFi drawing lines to the account that is logged in when
/// K-line is next opened. Safe to call on every entry; an empty guest store is
/// a no-op and source data is cleared only after the target transaction commits.
pub fn retail_kline_claim_guest_drawing_lines(max_lines: u32) -> Arc<OKRustFutureboolWithOKFFIError> { todo!() }

/// Identity of the selected drawing-line store (CeFi uid; empty when logged
/// out or for local-only stores). Platform migration owns the separate policy
/// for assigning legacy or guest data to an account.
pub fn retail_kline_drawing_store_uid(is_cedefi: bool) -> String { todo!() }

/// Identity of the selected drawing-line store.
pub fn retail_kline_drawing_store_uid_for_store(store_case: DrawingLineStoreCase) -> String { todo!() }

/// Batch import for the one-time platform migration (native SP/Room → Rust).
/// 
/// One call, one SQLite transaction, all-or-nothing — set the platform's
/// migration flag only when this resolves true. Do NOT migrate through
/// `save_line`: actions are fire-and-forget (no per-line result) and the save
/// queue gives no batch atomicity.
pub fn retail_kline_import_drawing_lines(lines: Vec<OKKLineDrawingData>, is_cedefi: bool, max_lines: u32) -> Arc<OKRustFutureboolWithOKFFIError> { todo!() }

/// Batch import into an explicit drawing-line store.
pub fn retail_kline_import_drawing_lines_for_store(lines: Vec<OKKLineDrawingData>, store_case: DrawingLineStoreCase, max_lines: u32) -> Arc<OKRustFutureboolWithOKFFIError> { todo!() }

/// Legacy compatibility entry point. New surfaces must use
/// `create_indicator_sync_interactor`, whose durable coordinator owns retry ordering.
pub fn upload_indicator_settings(setting_case: ChartSettingCase, server_keys: Vec<String>, show_states: Vec<IndicatorUploadShowState>) -> Arc<OKRustFutureIndicatorSyncUploadResultWithOKFFIError> { todo!() }
