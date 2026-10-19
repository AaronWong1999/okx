// =============================================================================
// Reconstructed Rust UniFFI IDL & Schema for crate: `retail_trading`
// Extracted from OKX Android v6.194.0 (lib/arm64-v8a/libOKCore.so)
// Total symbols decoded: 2226
//   Records: 323 | Enums: 117 | Interfaces: 137
//   Constructors: 95 | Methods: 1545 | Functions: 6
// =============================================================================

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

// -----------------------------------------------------------------------------
// 1. Enums (117)
// -----------------------------------------------------------------------------

/// Result of adding an instrument to the formula
#[derive(Debug, Clone, PartialEq)]
pub enum AddInstrumentResult {
    /// Successfully added
    Success,
    /// Formula is full (reached max elements)
    FormulaFull,
}

/// Result of adding an instrument to the formula (UniFFI-compatible)
#[derive(Debug, Clone, PartialEq)]
pub enum AddInstrumentResultVo {
    /// Successfully added
    Success,
    /// Formula is full (reached max elements)
    FormulaFull,
}

/// Enum wrapping all possible All Tab module types (UniFFI export)
/// Modules are sorted by weight descending in AllTabState.modules
#[derive(Debug, Clone, PartialEq)]
pub enum AllTabModule {
    Spot(AllTabSpotModule),
    Futures(AllTabFuturesModule),
    Options(AllTabOptionsModule),
    EventContract(AllTabEventContractModule),
    Dex(AllTabDexModule),
    Functions(AllTabFunctionsModule),
    P2p(AllTabP2pModule),
    Earn(AllTabEarnModule),
    Bots(AllTabBotsModule),
    Trader(AllTabTraderModule),
    Content(AllTabContentModule),
    Campaign(AllTabCampaignModule),
    Announcement(AllTabAnnouncementModule),
}

/// Time period for asset analysis queries.
#[derive(Debug, Clone, PartialEq)]
pub enum AssetAnalysisPeriod {
    /// 7-day period
    SevenDays,
    /// 30-day period
    ThirtyDays,
    /// 90-day period
    NinetyDays,
    /// All time
    All,
}

/// Type of a calendar event.
/// 
/// Used to distinguish economic, token unlock, and earnings events.
#[derive(Debug, Clone, PartialEq)]
pub enum CalendarEventType {
    /// Economic/macro event (default)
    Economic,
    /// Token unlock event
    TokenUnlock,
    /// Company earnings event
    Earnings,
}

/// Filter type for the trading calendar list page.
#[derive(Debug, Clone, PartialEq)]
pub enum CalendarFilterType {
    /// Show all event types
    All,
    /// Show only economic events
    Economic,
    /// Show only token unlock events
    TokenUnlock,
    /// Show only earnings events
    Earnings,
}

/// Category group type enumeration
/// 
/// It is used to filter out the raw vo list for displaying
#[derive(Debug, Clone, PartialEq)]
pub enum CategoryGroupType {
    /// Predefined category group for all categories
    All,
    /// Normal category group from backend response
    Normal,
    /// Deprecated/unused: Top (id=1) is now a `Normal` server-driven category classified by id; kept only for UniFFI ABI stability (do not remove).
    RankingTop,
    /// Predefined new ranking category group
    RankingNew,
    /// Predefined hot ranking category group
    RankingHot,
    /// Predefined gainers ranking category group
    RankingGainers,
    /// Predefined losers ranking category group
    RankingLosers,
    /// Predefined market cap ranking category group
    RankingMarketCap,
    /// Predefined turnover ranking category group
    RankingTurnover,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CategoryMixedItem {
    Spot(QuotationSpotItemVo),
    Futures(QuotationFuturesItemVo),
}

/// Charge group type enumeration
/// 
/// It is used to filter out the raw vo list for displaying
#[derive(Debug, Clone, PartialEq)]
pub enum ChargeGroupType {
    /// Normal charge group like (USD, USDS, Crypto Pair)
    Normal,
    /// Special charge group for Coin (Crypto)
    Crypto,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ChartPeriod {
    Hour1,
    Hour24,
    Week1,
    Month1,
    Year1,
    All,
}

/// About section type.
#[derive(Debug, Clone, PartialEq)]
pub enum CiAboutSectionType {
    /// "About {CCY}" introduction text
    About,
    /// Social media links and official URLs
    LinkAndSocial,
    /// Team members
    Members,
}

/// Semantic type for overview items.
/// 
/// Determines tap behavior and potential navigation targets.
#[derive(Debug, Clone, PartialEq)]
pub enum CiOverviewItemType {
    /// Default: no special behavior
    Normal,
    /// Hot rank item -- tap navigates to hot rank page (jump_index=100)
    HotRank,
    /// Market cap rank item -- tap navigates to market rank page (jump_index=101)
    MarketCapRank,
}

/// Social media type for coin info links.
/// 
/// Maps to API `name` field values. Used for filtering (weChat excluded)
/// and for resolving display title localization keys.
#[derive(Debug, Clone, PartialEq)]
pub enum CiSocialMediaType {
    OfficialUrl,
    WhitePaper,
    BlockBrowser,
    Github,
    Facebook,
    Twitter,
    Telegram,
    WeChat,
    Unknown,
}

/// 6-state async data state for commodity rank list (contains both spot and futures)
#[derive(Debug, Clone, PartialEq)]
pub enum CommodityRankDataState {
    Idle,
    InitialLoading,
    Refreshing {
        stale_data: StockRankListPo,
    },
    Success {
        data: StockRankListPo,
    },
    InitialError {
        error: HomeCommodityError,
    },
    RefreshError {
        error: HomeCommodityError,
        stale_data: StockRankListPo,
    },
}

/// Banner click action type. Maps to Android `BannerClickType`.
#[derive(Debug, Clone, PartialEq)]
pub enum CountingBannerClickType {
    /// Navigate to trade page (canTrade == true)
    Trade,
    /// Navigate to kline/chart page (canTrade == false)
    Kline,
    /// Navigate via deeplink URL (OKP banners)
    Deeplink,
}

/// 6-state async data state for counting banner list.
#[derive(Debug, Clone, PartialEq)]
pub enum CountingBannerDataState {
    Idle,
    InitialLoading,
    Refreshing {
        stale_data: Vec<CountingBannerVo>,
    },
    Success {
        data: Vec<CountingBannerVo>,
    },
    InitialError {
        error: CountingBannerError,
    },
    RefreshError {
        error: CountingBannerError,
        stale_data: Vec<CountingBannerVo>,
    },
}

/// Banner type enum. Maps to Android `BannerType`.
#[derive(Debug, Clone, PartialEq)]
pub enum CountingBannerType {
    /// New coin listing banner
    NewCoin,
    /// OKP promotion banner
    OkpBanner,
}

/// One-shot UX events emitted by the interactor.
/// 
/// `FavoriteToggled` carries both the requested direction (`to_watch`)
/// and the outcome (`success`) so native can branch into the four
/// add/remove × success/failure toast variants without parsing state.
#[derive(Debug, Clone, PartialEq)]
pub enum CryptoTradeListUxEvent {
    FavoriteToggled {
        to_watch: bool,
        success: bool,
    },
}

/// Business type for contract info tabs.
/// 
/// Maps to iOS OKTBizType subset used in OKTContractInfoRootVC.
#[derive(Debug, Clone, PartialEq)]
pub enum CtrBizType {
    /// Spot + Margin (combined tab)
    Spot,
    /// Perpetual swap
    Swap,
    /// Expiry futures
    Futures,
    /// Options
    Options,
}

/// Action associated with a field tap.
/// 
/// Native uses this to determine what happens when the user taps
/// the info icon or arrow on a field row.
#[derive(Debug, Clone, PartialEq)]
pub enum CtrFieldAction {
    /// No action (plain text display)
    None,
    /// Show limit price dialog
    ShowLimitPriceDialog,
    /// Show order rules dialog
    ShowOrderRulesDialog,
    /// Show position limit dialog
    ShowPositionLimitDialog,
    /// Show delivery profit dialog
    ShowDeliveryProfitDialog,
    /// Show closing profit dialog
    ShowClosingProfitDialog,
    /// Show settlement profit dialog
    ShowSettlementProfitDialog,
    /// Show exercise profit dialog (options)
    ShowExerciseProfitDialog,
    /// Navigate to instrument intro URL
    OpenInstrumentIntro,
    /// Navigate to ADL mechanism URL
    OpenAdlMechanism,
    /// Navigate to fee rate URL (options)
    OpenFeeRate,
    /// Show settlement time dialog
    ShowSettlementTimeDialog,
    /// Show Pre-IPO Face Value (Contract Size) rebase info dialog.
    /// Only emitted when the instrument is both `is_trad_fi` and `is_pre_market`.
    ShowPreIpoFaceValueDialog,
    /// Show Price Band hard-cap explanation dialog.
    /// Only emitted on Pre-IPO TradFi contracts (the Price Band row itself is
    /// already gated by the same condition in `build_swap_fields`).
    ShowPriceBandDialog,
}

/// Semantic type of a contract info display field.
/// 
/// Native uses this to determine rendering style (plain text, link, info icon, etc.)
#[derive(Debug, Clone, PartialEq)]
pub enum CtrFieldType {
    /// Plain text value (e.g., tick size, face value)
    PlainText,
    /// Text with info icon that opens a dialog (e.g., limit price, order rules)
    InfoDialog,
    /// Tappable row that navigates to a URL (e.g., instrument intro, ADL mechanism)
    LinkArrow,
}

/// How Native reassembles the localized numeric segments of a field value.
#[derive(Debug, Clone, PartialEq)]
pub enum CtrValueKind {
    /// A single localized number: `raw_numbers[0]`.
    Number,
    /// A percentage. `raw_numbers[0]` is the raw RATIO (e.g. "0.3" meaning 30%);
    /// Native multiplies by 100, formats per Locale, and appends the locale
    /// percent sign. Rust must NOT scale or include a `%` character.
    Percent,
    /// A closed range `[raw_numbers[0], raw_numbers[1]]`; Native joins the two
    /// localized numbers with the locale range separator.
    Range,
    /// `symbol` followed by `raw_numbers[0]` (e.g. index price "BTCUSDT 65432.1").
    SymbolNumber,
    /// `raw_numbers[0]` followed by `unit` (number + currency/unit suffix).
    NumberUnit,
    /// Price band: two "number unit" pairs (`raw_numbers[0]`/`raw_numbers[1]` with
    /// `unit`) that Native joins into a localized lower/upper range template.
    RangeWithUnit,
    /// Min order quantity composite: main quantity `raw_numbers[0]` (with the
    /// `{contracts_unit}` placeholder Native replaces) plus a parenthesized
    /// currency amount `raw_numbers[1]` with `unit`.
    MinOrderQty,
    /// Composite option tick size: `raw_numbers` are the tick sizes for each tier
    /// and `boundaries` describe the tiers. `unit` is empty because option tick
    /// size is displayed as a value only. Native reassembles the localized string.
    OptionTick,
}

/// 6-state async data state for a backend-configured home rank list.
#[derive(Debug, Clone, PartialEq)]
pub enum CustomHomeRankDataState {
    Idle,
    InitialLoading,
    Refreshing {
        stale_data: CustomHomeRankListPo,
    },
    Success {
        data: CustomHomeRankListPo,
    },
    InitialError {
        error: HomeCustomRankError,
    },
    RefreshError {
        error: HomeCustomRankError,
        stale_data: CustomHomeRankListPo,
    },
}

/// Visual state of a calendar date cell in the month picker.
#[derive(Debug, Clone, PartialEq)]
pub enum DateItemState {
    /// The date is the currently selected date
    Selected,
    /// The date has events and can be selected (default)
    Normal,
    /// The date has no events and cannot be selected
    Disable,
}

/// DEX time period for data aggregation
/// 
/// Uses `strum` for numeric string conversion:
/// - `Display`: `to_string()` yields the period value as string (`"1"`, `"2"`, `"3"`, `"4"`).
/// - `EnumString`: parse from string via `FromStr` (e.g. `"1"`, `"2"`, `"3"`, `"4"`).
/// - From integer: use `value.to_string().parse::<DexPeriod>().ok()`.
/// 
/// Corresponds to:
/// `com.okinc.market.quotation.ui.model.dex.DexPeriodEnum`
#[derive(Debug, Clone, PartialEq)]
pub enum DexPeriod {
    /// 5 minutes (value 1)
    FiveMinutes,
    /// 1 hour (value 2)
    OneHour,
    /// 4 hours (value 3)
    FourHours,
    /// 1 day (24 hours) (value 4) - Default
    OneDay,
    /// 24 hours, same value as OneDay (value 4)
    TwentyFourHours,
}

/// DEX sorting options enumeration
/// 
/// Defines the available sorting criteria for DEX token lists. Each variant
/// corresponds to a different sorting method that can be applied to DEX token
/// data for display purposes.
/// 
/// # Variants
/// 
/// - `Price`: Sort tokens by their current price value
/// - `Change`: Sort tokens by their price change ratio (24h percentage)
/// - `Turnover`: Sort tokens by their 24-hour trading volume (turnover)
/// - `MarketCap`: Sort tokens by their market capitalization
/// - `None`: No specific sorting (default), typically uses search volume or natural order
/// 
/// # Usage
/// 
/// This enum is used in conjunction with `SortOrder` to specify both the
/// sorting criteria and direction (ascending/descending) for DEX token lists.
/// 
/// # Example
/// 
/// ```ignore
/// use retail_trading::types::quotation::dex::DexSortBy;
/// use retail_trading::types::quotation::sorter::SortOrder;
/// let sort_by = DexSortBy::Turnover;
/// let sort_order = SortOrder::Descending;
/// // Sort DEX tokens by turnover in descending order
/// ```
/// 
/// Corresponds to `com.okinc.market.quotation.ui.model.dex.DexSortByEnum` in OKMarket.
#[derive(Debug, Clone, PartialEq)]
pub enum DexSortBy {
    /// Sort by price
    Price,
    /// Sort by change ratio
    Change,
    /// Sort by turnover (trading volume)
    Turnover,
    /// Sort by market capitalization
    MarketCap,
    /// No sorting / Search volume (default)
    None,
}

/// V2 Discover L1 page model. Fixed pages keep the existing enum and dynamic
/// Finance categories carry their complete cloud payload. Keeping the wrapper
/// separate avoids changing the ABI shape of [DiscoverPage].
#[derive(Debug, Clone, PartialEq)]
pub enum DiscoverL1Page {
    Fixed {
        page: DiscoverPage,
    },
    Category {
        category: FinanceCategoryTabVo,
    },
}

/// Final Discover Markets L1 page identity delivered to the host app.
/// Unlike the raw cloud [L1TabName](crate::types::market_tabs::L1TabName),
/// this is POST-gate. `stock` / `commodity` never appear: they have no
/// standalone L1 page and exist only as L2 categories. `trad_fi` DOES appear —
/// it is a business line of its own, whose L2 tabs come from the
/// `market_trad_fi` cloud scene, and must not be confused with the TradFi
/// category group (id 49) that still renders as a Category page.
#[derive(Debug, Clone, PartialEq)]
pub enum DiscoverPage {
    Favorites,
    Spot,
    Futures,
    /// Event-contract tab (may host the Options chain alongside — the
    /// hosting Fragment decides from its own service reads).
    Event,
    /// Legacy standalone Options tab. Retained for forward-compat and host-side
    /// `when` exhaustiveness only — no assembly path produces it now that the
    /// event AB is retired (the event slot yields `Event` or nothing).
    Options,
    Dex,
    Overview,
    /// TradFi business line. Its L2 tabs come from the `market_trad_fi` cloud
    /// scene; the page is only offered when that scene has usable categories.
    TradFi,
}

/// Earnings estimate comparison as supplied by the calendar API.
#[derive(Debug, Clone, PartialEq)]
pub enum EarningsBeatMiss {
    Beat,
    Missed,
    Met,
    /// Unrecognized wire value — a new server enum value degrades to this
    /// known-unknown instead of failing the whole calendar/market-data parse.
    Unknown,
}

/// Native destination for an earnings event's authoritative jump target.
#[derive(Debug, Clone, PartialEq)]
pub enum EarningsJumpDestination {
    Kline,
    KlineNews,
    CompanyFinancials,
}

/// Earnings publication status as supplied by the calendar API.
#[derive(Debug, Clone, PartialEq)]
pub enum EarningsPublicationStatus {
    Pending,
    Published,
    /// Unrecognized wire value — a new server enum value degrades to this
    /// known-unknown instead of failing the whole calendar/market-data parse.
    Unknown,
}

/// Earnings release session as supplied by the calendar API.
#[derive(Debug, Clone, PartialEq)]
pub enum EarningsReleaseSession {
    BeforeMarketOpen,
    DuringMarket,
    AfterMarketClose,
    Tbd,
    /// Unrecognized wire value — a new server enum value degrades to this
    /// known-unknown instead of failing the whole calendar/market-data parse.
    Unknown,
}

/// Earnings scheduling status as supplied by the calendar API.
#[derive(Debug, Clone, PartialEq)]
pub enum EarningsScheduleStatus {
    Estimated,
    Confirmed,
    Canceled,
    /// Unrecognized wire value — a new server enum value degrades to this
    /// known-unknown instead of failing the whole calendar/market-data parse.
    Unknown,
}

/// 6-state async data state for equity rank list (contains both spot and futures)
#[derive(Debug, Clone, PartialEq)]
pub enum EquityRankDataState {
    Idle,
    InitialLoading,
    Refreshing {
        stale_data: StockRankListPo,
    },
    Success {
        data: StockRankListPo,
    },
    InitialError {
        error: HomeEquityError,
    },
    RefreshError {
        error: HomeEquityError,
        stale_data: StockRankListPo,
    },
}

/// Event contract prediction method (FFI export)
#[derive(Debug, Clone, PartialEq)]
pub enum EventContractMethod {
    PriceUpDown,
    PriceAbove,
    /// Price-hit: settles if the index price touches the strike before delivery.
    PriceHit,
    /// Price-between: settles if the index price is within [floor, cap] at delivery.
    PriceBetween,
}

/// The NavSearch DEX business-mode sub-tabs.
/// 
/// Corresponds to the top-level business tabs consumed by the
/// `revamp/exos/dex` package on the Kotlin side.
#[derive(Debug, Clone, PartialEq)]
pub enum ExchangeOsBizTab {
    /// Perpetual futures ranking.
    Perp,
    /// Spot ranking.
    Spot,
    /// On-chain swap ranking (reuses the legacy `/dex/rank` data path).
    Swap,
}

/// 6-state async data state for explore boost cards.
#[derive(Debug, Clone, PartialEq)]
pub enum ExploreBoostDataState {
    Idle,
    InitialLoading,
    Refreshing {
        stale_data: Vec<ExploreBoostCardVo>,
    },
    Success {
        data: Vec<ExploreBoostCardVo>,
    },
    InitialError {
        error: ExploreBoostError,
    },
    RefreshError {
        error: ExploreBoostError,
        stale_data: Vec<ExploreBoostCardVo>,
    },
}

/// Stage of a boost activity, computed from timestamps + caller-supplied trusted now.
/// 
/// Variants map directly from the V1 Kotlin `@BoostType` int constants:
/// 0 = WaitingInfo (all timestamps zero, or unclassified fallback)
/// 1 = WaitingJoin (now < joinStartTime)
/// 2 = Joining     (joinStartTime <= now < joinEndTime)
/// 3 = WaitClaim   (joinEndTime <= now < claimStartTime)
/// 4 = Claiming    (claimStartTime <= now < claimEndTime)
/// 6 = Ended       (now >= claimEndTime)
/// 
/// Note: V1 skips int 5; Rust uses named variants, making the gap implicit.
#[derive(Debug, Clone, PartialEq)]
pub enum ExploreBoostStage {
    /// All five timestamps are zero — activity details not yet available. (V1 int 0)
    WaitingInfo,
    /// Countdown to join open. (V1 int 1)
    WaitingJoin,
    /// Join window is open. (V1 int 2)
    Joining,
    /// Join closed, waiting for claim window. (V1 int 3)
    WaitClaim,
    /// Claim window is open. (V1 int 4)
    Claiming,
    /// Activity has ended. (V1 int 6)
    Ended,
}

/// V2 Explore L1 model. Dynamic Finance pages keep the server title and
/// Category Group ID without adding TradFi/Stocks/ETF-specific enums.
#[derive(Debug, Clone, PartialEq)]
pub enum ExploreL1Page {
    Fixed {
        page: ExplorePage,
    },
    Category {
        category: FinanceCategoryTabVo,
    },
}

/// Fixed Explore card pages supported by the current Android host.
#[derive(Debug, Clone, PartialEq)]
pub enum ExplorePage {
    Favorites,
    Spot,
    Futures,
    Event,
    Dex,
}

/// Sort direction for funding fee queries.
#[derive(Debug, Clone, PartialEq)]
pub enum FaFundingSortDirection {
    /// Sort by estimated funding fee ascending
    EstimateAscending,
    /// Sort by estimated funding fee descending
    EstimateDescending,
    /// Sort by real-time funding fee ascending
    RealTimeAscending,
    /// Sort by real-time funding fee descending
    RealTimeDescending,
}

/// PnL type for futures analysis ticker queries.
#[derive(Debug, Clone, PartialEq)]
pub enum FaPnlType {
    /// Net PnL
    Net,
    /// Long PnL
    Long,
    /// Short PnL
    Short,
}

/// Business line type for favorite selection (UniFFI-compatible)
#[derive(Debug, Clone, PartialEq)]
pub enum FavoriteSelectionBusinessLineVo {
    Crypto,
    Spot,
    Futures,
    Dex,
    Options,
    /// exos DEX (NavSearch ExchangeOS pipeline). Selection ids carry the biz-tab
    /// discriminator in the item-id encoding (see the exos list interactor).
    ExosDex,
}

/// Three-state selection status (UniFFI-compatible)
#[derive(Debug, Clone, PartialEq)]
pub enum FavoriteSelectionStateVo {
    /// Already in the target group (non-cancelable)
    Existing,
    /// Newly selected by user (can be toggled off)
    Selected,
    /// Not selected
    Unselected,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FavoriteServiceError {
    NetworkFailed {
        error_message: String,
    },
    ServerResponseError {
        error_message: String,
    },
    JSONParseError {
        error_message: String,
    },
    InternalError {
        error_message: String,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum FavoritesAddToGroupUxEffect {
    /// Cryptos successfully added to groups
    AddToGroupsSuccess {
        /// Number of cryptos added
        crypto_count: u32,
        /// Names of the groups cryptos were added to
        group_names: Vec<String>,
    },
    /// Add-to-groups operation failed
    AddToGroupsError {
        /// Error message
        error: String,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum FavoritesCrudError {
    OperationFailed {
        msg: String,
    },
    Cancelled,
    /// The operation needs a logged-in session. Native layers should react by
    /// prompting the user to log in (matching iOS guest watchlist behaviour).
    LoginRequired,
}

/// UX Effects for favorites cryptos edit operations
#[derive(Debug, Clone, PartialEq)]
pub enum FavoritesCryptosEditUxEffect {
    /// Items removed successfully
    RemoveSuccess {
        /// Number of items removed
        count: i32,
    },
    /// Items remove failed
    RemoveError {
        error: String,
    },
    /// Items rearranged successfully
    RearrangeSuccess,
    /// Items rearrange failed
    RearrangeError {
        error: String,
    },
}

/// UX Effects for favorites groups edit operations
#[derive(Debug, Clone, PartialEq)]
pub enum FavoritesGroupsEditUxEffect {
    /// Group created successfully
    CreateGroupSuccess {
        group_name: String,
    },
    /// Group creation failed
    CreateGroupError {
        group_name: String,
        error: String,
    },
    /// Group deleted successfully
    DeleteGroupSuccess {
        group_name: String,
    },
    /// Group deletion failed
    DeleteGroupError {
        group_name: String,
        error: String,
    },
    /// Group renamed successfully
    RenameGroupSuccess {
        old_group_name: String,
        new_group_name: String,
    },
    /// Group rename failed
    RenameGroupError {
        old_group_name: String,
        new_group_name: String,
        error: String,
    },
    /// Groups rearranged successfully
    RearrangeGroupsSuccess,
    /// Groups rearrange failed
    RearrangeGroupsError {
        error: String,
    },
    /// Group visibility updated successfully
    UpdateVisibilitySuccess {
        group_name: String,
        hidden: bool,
    },
    /// Group visibility update failed
    UpdateVisibilityError {
        group_name: String,
        hidden: bool,
        error: String,
    },
    /// Group name validation result
    GroupNameValidation {
        result: GroupNameValidationResult,
    },
}

/// Formula operators
#[derive(Debug, Clone, PartialEq)]
pub enum FormulaOperator {
    /// Addition (+)
    Add,
    /// Subtraction (-)
    Subtract,
    /// Multiplication (× or *)
    Multiply,
    /// Division (÷ or /)
    Divide,
    /// Left parenthesis (
    LeftParen,
    /// Right parenthesis )
    RightParen,
}

/// Formula validation error types
#[derive(Debug, Clone, PartialEq)]
pub enum FormulaValidationError {
    /// Formula is empty
    EmptyFormula,
    /// Brackets are not matched
    UnmatchedBrackets,
    /// Empty brackets (e.g., "()")
    EmptyBrackets,
    /// Invalid variable (not wrapped in {})
    InvalidVariable,
    /// Constant value exceeds maximum (1,000,000)
    ConstantTooLarge,
    /// Negative constant not allowed
    NegativeConstant,
    /// Too many elements (> 10)
    TooManyElements,
    /// Too few elements (< 2)
    TooFewElements,
    /// No variables in formula (at least 1 required)
    NoVariables,
    /// Operator at invalid position (start/end)
    OperatorAtInvalidPosition,
    /// Consecutive operands without operator
    ConsecutiveOperands,
    /// Consecutive operators
    ConsecutiveOperators,
    /// Invalid expression structure
    InvalidExpression,
}

/// Futures tab page model.
/// 
/// Each variant maps to one tab in Discover-Futures or Nav-Search-Futures.
#[derive(Debug, Clone, PartialEq)]
pub enum FuturesPageVo {
    /// "All" tab — shows all instruments; `available_units` is the full unit list.
    All {
        available_units: Vec<FuturesUnitVo>,
        /// User-selected secondary unit filter on All tab.
        selected_unit: Option<FuturesUnitVo>,
    },
    /// Ranking tab (Hot / Gainers / Losers).
    Ranking {
        kind: RankingPageKind,
        category_group: CategoryGroupVo,
        default_unit: Option<FuturesUnitVo>,
    },
    /// Category tab (e.g., New, TradFi, or any BOSS-driven category with showInMarketNav).
    Category {
        category_group: CategoryGroupVo,
        default_unit: Option<FuturesUnitVo>,
    },
    /// Unit-driven tab (USDT-M / USD-S-M / Crypto-M) — shows ALL contract types.
    Unit {
        unit: FuturesUnitVo,
    },
    /// Contract-type-driven tab (X-Perp / Expiry) — shows all units.
    ContractType {
        futures_type: FuturesTypeKind,
    },
}

/// Futures contract type classification
/// 
/// Corresponds to: `com.okinc.market.quotation.ui.model.FuturesTypeEnum`
#[derive(Debug, Clone, PartialEq)]
pub enum FuturesType {
    /// All contract types
    All,
    /// Perpetual swap only
    PerpetualOnly,
    /// X-Perp contracts only
    XPerpsOnly,
    /// Delivery futures only
    ExpiryOnly,
    /// Perpetual swap + Delivery futures
    PerpetualAndExpiryOnly,
    /// New Coins + Pre-market only
    NewCoinsAndPremarketOnly,
}

/// Futures contract type kind for type-driven tabs.
#[derive(Debug, Clone, PartialEq)]
pub enum FuturesTypeKind {
    /// X-Perp contracts.
    XPerps,
    /// Delivery / Expiry contracts.
    Expiry,
}

/// 6-state async data state for gainer futures rank list
#[derive(Debug, Clone, PartialEq)]
pub enum GainerFuturesRankDataState {
    Idle,
    InitialLoading,
    Refreshing {
        stale_data: Vec<FuturesRankItemPo>,
    },
    Success {
        data: Vec<FuturesRankItemPo>,
    },
    InitialError {
        error: HomeGainerError,
    },
    RefreshError {
        error: HomeGainerError,
        stale_data: Vec<FuturesRankItemPo>,
    },
}

/// Instrument type for ranking tab selection
#[derive(Debug, Clone, PartialEq)]
pub enum GainerRankingInstType {
    /// Spot instruments
    Spot,
    /// Futures instruments (SWAP)
    Futures,
}

/// 6-state async data state for gainer spot rank list
#[derive(Debug, Clone, PartialEq)]
pub enum GainerSpotRankDataState {
    Idle,
    InitialLoading,
    Refreshing {
        stale_data: Vec<UpDownSpotRank>,
    },
    Success {
        data: Vec<UpDownSpotRank>,
    },
    InitialError {
        error: HomeGainerError,
    },
    RefreshError {
        error: HomeGainerError,
        stale_data: Vec<UpDownSpotRank>,
    },
}

/// Group name validation result
/// 
/// Matches OC `checkGroupName:` return values for consistency.
#[derive(Debug, Clone, PartialEq)]
#[repr(i32)]
pub enum GroupNameValidationResult {
    /// The group name is valid and available (OC: 0)
    Valid = 0,
    /// The group name exceeds the maximum length of 26 characters (OC: 1)
    TooLong = 1,
    /// The group name contains invalid characters (not matching `^[\p{L}0-9_\-\s]+$`) or is empty (OC: 2)
    InvalidCharacters = 2,
    /// The group name already exists (case-insensitive comparison) (OC: 3)
    AlreadyExists = 3,
    /// The group name consists only of whitespace (OC: 4)
    OnlyWhitespace = 4,
}

/// Indicates the source of the displayed favorites data
/// 
/// This enum helps the UI layer distinguish between actual user favorites
/// and recommended coins shown as a fallback when the user has no favorites.
#[derive(Debug, Clone, PartialEq)]
pub enum HomeFavoritesDataSource {
    /// Data is from user's actual favorites
    Favorites,
    /// Data is from recommended coins (shown when ALL group is empty)
    Recommended,
}

/// 6-state async data state for favorites list
#[derive(Debug, Clone, PartialEq)]
pub enum HomeFavoritesDataState {
    Idle,
    InitialLoading,
    Refreshing {
        stale_data: Vec<QuotationFavoriteVo>,
    },
    Success {
        data: Vec<QuotationFavoriteVo>,
    },
    InitialError {
        error: HomeFavoritesError,
    },
    RefreshError {
        error: HomeFavoritesError,
        stale_data: Vec<QuotationFavoriteVo>,
    },
}

/// UX Effects for home favorites
/// 
/// These are one-shot UI events that the native layer should handle
#[derive(Debug, Clone, PartialEq)]
pub enum HomeFavoritesUxEffect {
    /// Favorite added successfully
    AddFavSuccess {
        item: QuotationFavoriteVo,
    },
    /// Favorite add failed
    AddFavError {
        item: QuotationFavoriteVo,
        error: String,
    },
    /// Favorite removed successfully
    RemoveFavSuccess {
        item: QuotationFavoriteVo,
    },
    /// Favorite remove failed
    RemoveFavError {
        item: QuotationFavoriteVo,
        error: String,
    },
    /// Favorites rearranged successfully
    RearrangeSuccess {
        count: i32,
    },
    /// Favorites rearrange failed
    RearrangeError {
        error: String,
    },
    /// Multiple favorites added successfully
    AddFavoritesSuccess {
        items: Vec<QuotationFavoriteVo>,
        group_name: String,
    },
    /// Multiple favorites add failed
    AddFavoritesError {
        items: Vec<QuotationFavoriteVo>,
        group_name: String,
        error: String,
    },
}

/// 6-state async data state for hot futures rank list
#[derive(Debug, Clone, PartialEq)]
pub enum HotFuturesRankDataState {
    Idle,
    InitialLoading,
    Refreshing {
        stale_data: Vec<FuturesRankItemPo>,
    },
    Success {
        data: Vec<FuturesRankItemPo>,
    },
    InitialError {
        error: HomeHotError,
    },
    RefreshError {
        error: HomeHotError,
        stale_data: Vec<FuturesRankItemPo>,
    },
}

/// Instrument type for ranking tab selection
#[derive(Debug, Clone, PartialEq)]
pub enum HotRankingInstType {
    /// Spot instruments
    Spot,
    /// Futures instruments (SWAP)
    Futures,
}

/// 6-state async data state for hot spot rank list
#[derive(Debug, Clone, PartialEq)]
pub enum HotSpotRankDataState {
    Idle,
    InitialLoading,
    Refreshing {
        stale_data: Vec<HotSpotRankItemPo>,
    },
    Success {
        data: Vec<HotSpotRankItemPo>,
    },
    InitialError {
        error: HomeHotError,
    },
    RefreshError {
        error: HomeHotError,
        stale_data: Vec<HotSpotRankItemPo>,
    },
}

/// 6-state async data state for the home index coin quotes list
#[derive(Debug, Clone, PartialEq)]
pub enum IndexQuoteDataState {
    Idle,
    InitialLoading,
    Refreshing {
        stale_data: Vec<IndexQuoteVo>,
    },
    Success {
        data: Vec<IndexQuoteVo>,
    },
    InitialError {
        error: HomeIndexError,
    },
    RefreshError {
        error: HomeIndexError,
        stale_data: Vec<IndexQuoteVo>,
    },
}

/// Instrument type for filtering (UniFFI-compatible)
#[derive(Debug, Clone, PartialEq)]
pub enum InstTypeVo {
    /// Spot trading pairs
    Spot,
    /// Futures/Swap contracts
    Futures,
    /// Options contracts
    Options,
    /// DEX tokens
    Dex,
    /// Coins/Currencies
    Coin,
}

/// Margin type for an options instrument.
/// 
/// Represents the collateral / margining model the option uses:
/// - `Cm` (Coin-Margined): traditional crypto-margined options, e.g. `BTC-USD-...`
/// - `Um` (USDⓈ-Margined / Linear): USDⓈ-settled linear options, e.g. `BTC-USD-...-UM`
/// 
/// Mirrors OKMarket Kotlin `MarginType` enum and is the single source of truth for
/// margin-type discrimination across UniFFI VOs (`OptionsCategoryGroupVo`,
/// `QuotationOptionsItemVo`, `OptionsChainItemVo`, `QuotationFavoriteVo::Options`).
/// 
/// Flat enum (no payload) for minimal UniFFI binding overhead.
#[derive(Debug, Clone, PartialEq)]
pub enum MarginType {
    /// Coin-Margined option (CM).
    Cm,
    /// USDⓈ-Margined / linear option (UM).
    Um,
}

/// Time window for the market-overview chart query.
/// 
/// API parameter name: `period`
/// Values confirmed from iOS `OKMOverviewUtil.m`:
/// `"1D"`, `"1W"`, `"1M"`, `"3M"`, `"1Y"`, `"ALL"`
#[derive(Debug, Clone, PartialEq)]
pub enum MarketChartPeriod {
    /// 1 day — API value `"1D"`
    OneDay,
    /// 1 week (7 days) — API value `"1W"`
    SevenDay,
    /// 1 month (30 days) — API value `"1M"`
    ThirtyDay,
    /// 3 months (90 days) — API value `"3M"`
    NinetyDay,
    /// 1 year — API value `"1y"`
    OneYear,
    /// All available history — API value `"ALL"`
    All,
}

/// Which metric to display in the market-overview chart.
/// 
/// API parameter name: `type`
/// Values confirmed from iOS `OKMOverviewUtil.m`:
/// MarketCap → `"1"`, Ratio → `"2"`, Volume → `"3"`
#[derive(Debug, Clone, PartialEq)]
pub enum MarketChartType {
    /// Total market capitalisation — API value `"1"`
    MarketCap,
    /// Coin dominance ratio — API value `"2"` (requires `coin` param)
    Ratio,
    /// 24 h trading volume — API value `"3"`
    Volume,
}

/// Time-zone convention used to compute 24 h price changes.
/// 
/// The caller reads the user's preference from `UserPreferencesManager` and
/// passes the corresponding variant; the Rust layer never reads user preferences
/// directly.
#[derive(Debug, Clone, PartialEq)]
pub enum MarketFlowZone {
    /// Rolling 24 h window — API value `"24h"` (default)
    H24,
    /// UTC+0 daily window — API value `"utc0"`
    Utc0,
    /// UTC+8 daily window — API value `"utc8"`
    Utc8,
}

/// Public FFI result contract for the navigation account derivative-config exits.
/// 
/// A feature-specific `uniffi::Error` (NOT the global message-only `OKFFIError`)
/// so native identifies the outcome by variant, not by matching a string. Success
/// stays `Ok(Vec<NavDerivativeConfigVo>)` (an empty list is a valid success). The
/// crate-internal `RetailTradingError` is unchanged; this type is only the final
/// public mapping.
/// 
/// The payload field is named `msg`, NOT `message`: the generated Kotlin error
/// class extends `Exception`, so a `message` field collides with
/// `Throwable.message` and fails the Kotlin build with conflicting declarations
/// and a missing `override`. This matches the crate's existing
/// `FavoritesCrudError`, which uses `msg` for the same reason.
#[derive(Debug, Clone, PartialEq)]
pub enum NavAccountConfigError {
    /// A normal HTTP response that the server rejected (non-zero business code),
    /// or a success code that carried no config data. A business-level failure —
    /// never conflated with a transport/parse exception.
    LogicalFailure {
        msg: String,
    },
    /// The operation was cancelled via its per-request `OKCancellationToken`.
    Cancelled,
    /// Transport failure (HTTP non-2xx / connection) or an unparseable response —
    /// never reported as a success.
    RequestException {
        msg: String,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum NavSearchFavoritesGroupUxEffect {
    /// Favorite added successfully
    AddFavSuccess {
        item: QuotationFavoriteVo,
        group_name: String,
    },
    /// Favorite add failed
    AddFavError {
        item: QuotationFavoriteVo,
        group_name: String,
        error: String,
    },
    /// Favorite removed successfully
    RemoveFavSuccess {
        item: QuotationFavoriteVo,
        group_name: String,
    },
    /// Favorite remove failed
    RemoveFavError {
        item: QuotationFavoriteVo,
        group_name: String,
        error: String,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum NavSearchTabPage {
    Favorites,
    Spot,
    Futures,
    Dex,
    Options,
    /// Combined Events & Options tab. Emitted when AB on + Event FR on + Option FR on.
    /// Replaces the plain `Options` tab in that combination. Android renders
    /// `NavSearchEventContractOptionsGroupFragmentV2(showEvents=true, showOptionChain=true)`,
    /// which exposes Event sub-tabs (All / UpDown / Above) plus **only** the
    /// Options Chain sub-page (no Quote / Simple / Copilot). See
    /// `root_tab_assembler::assemble_biz_root_tabs` for the full AB/FR truth table.
    EventContractAndOptions,
    /// Events-only tab. Emitted when AB on + Event FR on + Option FR off.
    /// Android renders
    /// `NavSearchEventContractOptionsGroupFragmentV2(showEvents=true, showOptionChain=false)`,
    /// which shows only the Event sub-tabs.
    EventContractOnly,
    Crypto {
        category_group: CategoryGroupVo,
    },
    /// Dynamic Finance business-line page from navL1Tabs. The Category owns
    /// Spot/Futures L2 pages; it is not part of coin mode.
    Category {
        category: FinanceCategoryTabVo,
    },
    /// TradFi business line from navL1Tabs. Its L2 tabs come from the
    /// `nav_trad_fi` cloud scene. Not to be confused with the TradFi CATEGORY
    /// (id 49), which still arrives as [NavSearchTabPage::Category].
    TradFi,
}

/// Navigation parameters for different record types
/// 
/// Contains the minimal information needed to navigate to the detail page.
/// Stored as serializable struct for persistence.
#[derive(Debug, Clone, PartialEq)]
pub enum NavigationParams {
    /// Spot: inst_id, inst_type
    Spot {
        inst_id: String,
        inst_type: String,
    },
    /// Futures: inst_id, inst_type
    Futures {
        inst_id: String,
        inst_type: String,
    },
    /// Options: inst_id, inst_type, exp_time
    Options {
        inst_id: String,
        inst_type: String,
        exp_time: Option<String>,
    },
    /// Event: inst_id, inst_type. `method`/`freq` are optional and only used to
    /// render a richer history title without rehydrating the instrument — reopen
    /// needs only `inst_id` (+ inst_type = "EVENTS"), matching V1 parity.
    Event {
        inst_id: String,
        inst_type: String,
        method: Option<String>,
        freq: Option<String>,
    },
    /// Dex: chain_id, chain_name, token_contract_address, token_symbol, cedefi_support, community_recognized, boosted
    Dex {
        chain_id: String,
        chain_name: String,
        token_contract_address: String,
        token_name: String,
        token_symbol: String,
        cedefi_support: bool,
        community_recognized: bool,
        boosted: bool,
    },
    /// Earn: currency_id, subscription_url, currency_name, product_type_name
    Earn {
        currency_id: i32,
        subscription_url: String,
        currency_name: String,
        product_type_name: String,
    },
    /// Trader: jump_param (contains trader routing info)
    Trader {
        jump_param: String,
        nick_name_cn: String,
        nick_name_en: String,
        trader_type: String,
    },
    /// Bots: display_id, biz_type, strategy_type
    Bots {
        display_id: String,
        biz_type: String,
        strategy_type: String,
        inst_name: String,
    },
    /// P2P: currency, currency_id, support_p2p, is_p2p
    P2p {
        currency: String,
        currency_id: String,
        support_p2p: String,
        is_p2p: bool,
    },
    /// Campaign: activity_id, link
    Campaign {
        activity_id: i32,
        link: String,
    },
    /// Announcement: link
    Announcement {
        link: String,
    },
    /// Content: raw_po_json for passing to client
    Content {
        raw_po_json: String,
    },
    /// ContentKol: kol_id — the only field needed to reopen a Planet resident
    /// profile on revisit. Additive sibling of `Content`; do not reorder.
    ContentKol {
        kol_id: String,
    },
}

/// 6-state async data state for new coin futures rank list
#[derive(Debug, Clone, PartialEq)]
pub enum NewCoinFuturesRankDataState {
    Idle,
    InitialLoading,
    Refreshing {
        stale_data: Vec<NewCoinFuturesRankPo>,
    },
    Success {
        data: Vec<NewCoinFuturesRankPo>,
    },
    InitialError {
        error: HomeNewCoinError,
    },
    RefreshError {
        error: HomeNewCoinError,
        stale_data: Vec<NewCoinFuturesRankPo>,
    },
}

/// 6-state async data state for unified new coin rank (includes spot + futures + wait online)
/// Uses HomeNewCoinGroupVo which contains: online, wait_online_new, wait_online, futures_online
#[derive(Debug, Clone, PartialEq)]
pub enum NewCoinRankDataState {
    Idle,
    InitialLoading,
    Refreshing {
        stale_data: HomeNewCoinGroupVo,
    },
    Success {
        data: HomeNewCoinGroupVo,
    },
    InitialError {
        error: HomeNewCoinError,
    },
    RefreshError {
        error: HomeNewCoinError,
        stale_data: HomeNewCoinGroupVo,
    },
}

/// Instrument type for new coin ranking tab selection
#[derive(Debug, Clone, PartialEq)]
pub enum NewCoinRankingInstType {
    /// Spot instruments
    Spot,
    /// Futures instruments (SWAP)
    Futures,
}

/// 6-state async data state for new coin spot rank list
#[derive(Debug, Clone, PartialEq)]
pub enum NewCoinSpotRankDataState {
    Idle,
    InitialLoading,
    Refreshing {
        stale_data: Vec<NewCoinRankPo>,
    },
    Success {
        data: Vec<NewCoinRankPo>,
    },
    InitialError {
        error: HomeNewCoinError,
    },
    RefreshError {
        error: HomeNewCoinError,
        stale_data: Vec<NewCoinRankPo>,
    },
}

#[derive(Debug, Clone, PartialEq)]
#[repr(i32)]
pub enum NewCoinStateType {
    CountDown = 0,
    WillOnline = 1,
    Cancel = 2,
    Auction = 3,
    PreQuote = 4,
    Trading = 5,
}

/// Options direction (Call/Put)
/// 
/// Uses `strum` for string conversion:
/// - `Display` trait for converting to string (via `to_string()`)
/// - `EnumString` trait for parsing from string (via `FromStr`)
#[derive(Debug, Clone, PartialEq)]
pub enum OptionsDirection {
    /// All options
    All,
    /// Call option (bullish)
    Call,
    /// Put option (bearish)
    Put,
}

/// Pre-market token lifecycle state.
/// 
/// The API returns one of `"live"`, `"close"`, `"preopen"`, `"converted"`.
/// Any unrecognised value is captured in `Unknown` for forward compatibility.
#[derive(Debug, Clone, PartialEq)]
pub enum PreMarketState {
    /// Trading is active.
    Live,
    /// Trading has closed.
    Close,
    /// Token is upcoming (API value: `"preopen"`).
    Upcoming,
    /// Token has been converted to a regular listing.
    Converted,
    /// Unrecognised state — kept for forward compatibility.
    Unknown {
        value: String,
    },
}

/// Policy for pre-selected instruments (UniFFI-compatible)
#[derive(Debug, Clone, PartialEq)]
pub enum PreSelectedPolicyVo {
    /// Pre-selected instruments can be canceled/deselected
    Cancelable,
    /// Pre-selected instruments cannot be canceled/deselected
    NonCancelable,
}

/// Quotation data source change events
/// 
/// Corresponds to:
/// `com.okinc.market.quotation.ui.model.QuotationDataSourceChangedEvent`
#[derive(Debug, Clone, PartialEq)]
pub enum QuotationDataSourceChangedEvent {
    /// Business line data source changed
    InstrumentBizChanged {
        /// Instrument type
        inst_type: String,
        /// Timestamp in milliseconds
        ts: i64,
    },
    /// Countdown ended event
    CountDownChange {
        /// Timestamp in milliseconds
        ts: i64,
    },
    /// New coin online event
    NewCoinOnline {
        /// Timestamp in milliseconds
        ts: i64,
    },
    /// Account changed event (after account switch)
    AccountChanged {
        /// Timestamp in milliseconds
        ts: i64,
    },
}

/// Quotation Favorite View Object
/// 
/// This enum wraps different quotation item types for unified display
/// in the favorites/watchlist screen.
/// 
/// Each variant contains the full VO for that business line, preserving
/// all specific fields and behaviors.
#[derive(Debug, Clone, PartialEq)]
pub enum QuotationFavoriteVo {
    /// Spot trading pair (e.g., BTC-USDT spot)
    Spot(QuotationSpotItemVo),
    /// Futures/Swap trading pair (e.g., BTC-USDT-SWAP)
    Futures(QuotationFuturesItemVo),
    /// Options contract (e.g., BTC-USD-230630-30000-C)
    Options(QuotationOptionsItemVo),
    /// DEX token (e.g., Solana on-chain token)
    Dex(QuotationDexItemVo),
    /// Coin/Currency (e.g., BTC as a coin, not a trading pair)
    Coin(QuotationCryptoItemVo),
}

/// Unified output for all ranking detail types.
/// 
/// Each variant carries typed items for both spot and futures lists.
/// For `NewCoin`, a single API call returns both in `NewCoinGroupResponse`
/// (fields `online`/`wait_online_new` for spot; `futures_online`/`futures_wait_online`
/// for futures).
#[derive(Debug, Clone, PartialEq)]
pub enum RankingDetailData {
    /// Hot rank: spot items + futures items
    Hot {
        spot: Vec<HotSpotRankItemPo>,
        futures: Vec<FuturesRankItemPo>,
    },
    /// New coin rank: single response containing both spot and futures data
    NewCoin {
        data: NewCoinGroupResponse,
    },
    /// Gainers rank: spot items + futures items
    Gainers {
        spot: Vec<UpDownSpotRank>,
        futures: Vec<FuturesRankItemPo>,
    },
    /// Losers rank: spot items + futures items
    Losers {
        spot: Vec<UpDownSpotRank>,
        futures: Vec<FuturesRankItemPo>,
    },
    /// Market cap rank: spot items + futures items
    MarketCap {
        spot: Vec<MarketCapRankItemPo>,
        futures: Vec<FuturesRankItemPo>,
    },
    /// Turnover rank: spot items + futures items
    Turnover {
        spot: Vec<TurnoverRankItemPo>,
        futures: Vec<FuturesRankItemPo>,
    },
}

/// Sub-tab market type for ranking detail.
/// 
/// Controls whether spot, futures, or both are fetched.
/// V1 parity: spot and futures tabs load independently —
/// only the visible sub-tab triggers an API call.
#[derive(Debug, Clone, PartialEq)]
pub enum RankingDetailMarketType {
    /// Fetch only spot data (futures list will be empty).
    Spot,
    /// Fetch only futures data (spot list will be empty).
    Futures,
}

/// Ranking detail category type.
/// 
/// Maps to iOS `OKGDTopListType` (`newRankListAry`). Each variant has both
/// a spot list and a futures list.
/// 
/// | Variant   | Time filter | Spot API                              | Futures API                       |
/// |-----------|-------------|---------------------------------------|-----------------------------------|
/// | Hot       | ✅           | /public/hot-rank                      | /futures/ranking (rankType=hot)   |
/// | NewCoin   | ❌           | /public/new-coin-rank (spot+futures)  | (included in spot response)       |
/// | Gainers   | ✅           | /public/up-down-rank (rank=0)         | /futures/up-down-rank             |
/// | Losers    | ✅           | /public/up-down-rank (rank=1)         | /futures/ranking (rankType=losers)|
/// | MarketCap | ❌           | /public/market-cap-rank               | /futures/ranking (rankType=marketCap)|
/// | Turnover  | ✅           | /public/turn-over-rank                | /futures/ranking (rankType=volume)|
#[derive(Debug, Clone, PartialEq)]
pub enum RankingDetailType {
    /// Hot rank - ranked by heat index.
    Hot,
    /// New coin rank - recently listed instruments.
    /// Single API call returns both spot and futures data.
    NewCoin,
    /// Gainers - instruments with the highest price increase.
    Gainers,
    /// Losers - instruments with the largest price decrease.
    Losers,
    /// Market cap rank - ranked by market capitalization.
    MarketCap,
    /// Turnover rank - ranked by trading volume/turnover.
    Turnover,
}

/// Ranking page kind — shared by Spot and Futures pages.
#[derive(Debug, Clone, PartialEq)]
pub enum RankingPageKind {
    Hot,
    Gainers,
    Losers,
}

/// Favorite item enum that can represent different types of favorites
#[derive(Debug, Clone, PartialEq)]
pub enum RetailTradingFavoriteItem {
    SPOT(FavoriteSpotItem),
    FUTURE(FavoriteFutureItem),
    OPTION(FavoriteOptionItem),
    DEX(FavoriteDEXItem),
    COIN(FavoriteCoinItem),
}

/// Chart date format derived from API response timeStep.
#[derive(Debug, Clone, PartialEq)]
pub enum SaChartDateFormat {
    /// Daily data: localized month-day format
    MonthDay,
    /// Monthly data: localized year-month format
    MonthYear,
}

/// Cost price calculation type.
#[derive(Debug, Clone, PartialEq)]
pub enum SaCostPriceType {
    /// Average cost
    Average,
    /// FIFO cost
    Fifo,
}

/// Sort column for the coin list.
#[derive(Debug, Clone, PartialEq)]
pub enum SaPnlFilterType {
    /// No sorting (server default order)
    None,
    /// Sort by coin name (alphabetical)
    Name,
    /// Sort by holding quantity (numeric)
    Balance,
    /// Sort by PnL value (numeric)
    Pnl,
}

/// Sort direction for the coin list.
/// Cycles: None -> Desc -> Asc -> None (via toggle_sort).
#[derive(Debug, Clone, PartialEq)]
pub enum SaPnlSortOrder {
    /// No sorting
    None,
    /// Descending
    Desc,
    /// Ascending
    Asc,
}

/// Tab segment type for the coin list.
#[derive(Debug, Clone, PartialEq)]
pub enum SaSegmentType {
    /// Current holdings tab
    CurrentHoldings,
    /// All-time PnL tab
    AllTime,
}

/// Search entry point - where the search was initiated
#[derive(Debug, Clone, PartialEq)]
pub enum SearchEntry {
    /// Global search (main search page)
    Global,
    /// Navigation search (nav bar search)
    Navigation,
    /// Landscape search (landscape mode)
    Landscape,
}

/// Search record type for categorization and filtering
/// 
/// Corresponds to data types returned from `MainSearchManager`.
#[derive(Debug, Clone, PartialEq)]
pub enum SearchRecordType {
    /// Spot trading pairs - `QuotationSpotItemVo`
    Spot,
    /// Futures/Swap contracts - `QuotationFuturesItemVo`
    Futures,
    /// Options contracts - `QuotationOptionsItemVo`
    Options,
    /// Event contracts - `QuotationEventContractItemVo`
    Event,
    /// DEX tokens - `SearchDexItem`
    Dex,
    /// Earn products (savings, staking) - `SearchEarnItem`
    Earn,
    /// Copy trading leaders - `SearchTraderItem`
    Trader,
    /// Trading bots (grid, DCA) - `SearchBotsItem`
    Bots,
    /// P2P trading - `SearchP2pItem`
    P2p,
    /// Promotional campaigns - `SearchCampaignItem`
    Campaign,
    /// Announcements - `SearchAnnouncementItem`
    Announcement,
    /// Content (Feed/KOL) - `SearchContentVo`
    Content,
    /// Planet resident (KOL) - `SearchContentKolVo`. Additive sibling of `Content`
    /// keyed by `kol_id` (dedup key `content_kol_{kol_id}`); do not reorder or
    /// merge with `Content` (legacy record deserialization depends on `Content`).
    /// Never remove this variant either: `SearchHistoryStorage::load` deserializes
    /// the whole persisted sequence in a single pass, so one unknown variant fails
    /// the entire parse and silently drops a user's ENTIRE search history.
    ContentKol,
}

/// Futures type for filtering (UniFFI-compatible)
#[derive(Debug, Clone, PartialEq)]
pub enum SelectionFuturesTypeVo {
    All,
    PerpetualOnly,
    XPerpsOnly,
    ExpiryOnly,
    PerpetualAndExpiryOnly,
    NewCoinsAndPremarketOnly,
}

/// Selection mode for instrument selection (UniFFI-compatible)
#[derive(Debug, Clone, PartialEq)]
pub enum SelectionModeVo {
    /// Single selection - selecting new item replaces previous selection
    Single,
    /// Multiple selection - can select many items
    Multi,
    /// Single selection with immediate dismiss - selecting item immediately returns result
    SingleImmediate,
}

/// Validation result for selection operations (UniFFI-compatible)
/// Named differently from old interactor's ValidationResultVo to allow coexistence.
#[derive(Debug, Clone, PartialEq)]
pub enum SelectionValidationResultVo {
    Valid,
    BelowMinimum {
        current: u32,
        required: u32,
    },
    AboveMaximum {
        current: u32,
        limit: u32,
    },
    CannotCancelPreSelected,
    NotInAllowedList,
    CustomError {
        message: String,
    },
}

/// Sorting field selection
/// 
/// Corresponds to:
/// `com.okinc.market.quotation.ui.model.SortBy`
#[derive(Debug, Clone, PartialEq)]
pub enum SortBy {
    /// Sort by name (alphabetical)
    Name,
    /// Sort by volume (24h trading volume)
    Volume,
    /// Sort by turnover (24h trading value)
    Turnover,
    /// Sort by price
    Price,
    /// Sort by price change (percentage)
    Change,
    /// No sorting (default order)
    None,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SortConfig {
    Default,
    PriceAsc,
    PriceDesc,
    ChangeRatioAsc,
    ChangeRatioDesc,
}

/// Sorting order
/// 
/// Corresponds to:
/// `com.okinc.market.quotation.ui.model.SortOrder`
#[derive(Debug, Clone, PartialEq)]
pub enum SortOrderEnum {
    /// Ascending order (0-9, A-Z, low to high)
    Asc,
    /// Descending order (9-0, Z-A, high to low)
    Desc,
    /// No specific order
    None,
}

/// Spot tab page model.
/// 
/// Each variant maps to one tab in Discover-Spot or Nav-Search-Spot.
#[derive(Debug, Clone, PartialEq)]
pub enum SpotPageVo {
    /// "All" tab — shows all instruments; `available_charge_groups` lists every
    /// quote group (USDT / USDⓈ / CryptoⓈ) for the quote-currency chooser.
    All {
        available_charge_groups: Vec<ChargeGroupVo>,
        /// User-selected secondary filter on All tab (None = no filter applied).
        selected_charge_group: Option<ChargeGroupVo>,
    },
    /// Ranking tab (Hot / Gainers / Losers).
    Ranking {
        kind: RankingPageKind,
        category_group: CategoryGroupVo,
        default_charge_group: Option<ChargeGroupVo>,
    },
    /// Category tab (e.g., New, or any BOSS-driven category passing the spot filter).
    Category {
        category_group: CategoryGroupVo,
        default_charge_group: Option<ChargeGroupVo>,
    },
    /// Currency-specific tab (e.g., USDT, USDC).
    Ccy {
        charge_group: ChargeGroupVo,
    },
    /// Crypto (coin-margined) tab.
    Crypto,
}

/// Typed stock-rank lifecycle state, derived in Rust from the V2 wire fields.
/// Native renders from this enum only — never from raw display text or local
/// inference (FR-012).
/// 
/// Forward-compat `Unknown` fallback: an unrecognized wire `state` degrades to a
/// neutral wait-online row (no crash, no local inference).
/// 
/// Derives `Serialize, Deserialize` because it is a field of the serialized
/// `StockRankWaitOnlineItemPo` — a `uniffi::Enum` embedded in a serde-serialized
/// struct must also derive serde or the struct fails to compile.
#[derive(Debug, Clone, PartialEq)]
pub enum StockLifecycleState {
    /// Unrecognized / forward-compat. Only malformed wait-online rows use this.
    Unknown,
    /// state empty AND type == "0" AND now < onlineTs. Countdown label + de-emphasized price/change.
    Countdown,
    /// type == "1" / state == "STAY_TUNED" — no concrete listing time.
    StayTuned,
    /// state == "CALL_AUCTION". Call-auction status text; tappable.
    CallAuction,
    /// state == "PRE_QUOTE". Pre-market / pre-quote status text; tappable.
    PreQuote,
    /// Online item is in normal continuous trading.
    Trading,
}

/// 6-state async data state for stock rank list (contains both spot and futures)
#[derive(Debug, Clone, PartialEq)]
pub enum StockRankDataState {
    Idle,
    InitialLoading,
    Refreshing {
        stale_data: StockRankListPo,
    },
    Success {
        data: StockRankListPo,
    },
    InitialError {
        error: HomeStockError,
    },
    RefreshError {
        error: HomeStockError,
        stale_data: StockRankListPo,
    },
}

/// Ticker subscription mode
/// 
/// Controls how ticker data is subscribed:
/// - `None`: No ticker data subscription
/// - `HttpOnly`: Only HTTP ticker requests (periodic polling)
/// - `HttpAndWebSocket`: Both HTTP and WebSocket ticker subscriptions
#[derive(Debug, Clone, PartialEq)]
pub enum TickersMode {
    /// No ticker data subscription
    None,
    /// Only HTTP ticker requests (periodic polling)
    HttpOnly,
    /// Both HTTP and WebSocket ticker subscriptions
    HttpAndWebSocket,
}

/// Time period enum for the period-based tickers feature.
/// 
/// Used as the `tickers_period` parameter throughout the
/// Payload → Manager → Interactor chain.
/// 
/// `strum::AsRefStr` provides zero-cost string access via `.as_ref()`,
/// e.g. `TickersPeriodEnum::Period1D.as_ref()` returns `"1D"`.
/// This mirrors the Kotlin `PeriodEnum.inputPara` convention.
#[derive(Debug, Clone, PartialEq)]
pub enum TickersPeriodEnum {
    /// 1-day period (default)
    Period1D,
    /// 5-minute period
    Period5M,
    /// 1-hour period
    Period1H,
    /// 1-week period
    Period1W,
    /// 1-month period
    Period1M,
}

/// Time period filter for top movers in a category detail page.
/// 
/// Maps to iOS `OKGDTLUpDownTimeType` constants.
#[derive(Debug, Clone, PartialEq)]
pub enum TopMoversPeriod {
    /// Today (1D)
    Today,
    /// Last 5 minutes
    FiveMinutes,
    /// Last 1 hour
    OneHour,
    /// This week
    ThisWeek,
    /// This month
    ThisMonth,
}

/// Radar signal type filter.
/// 
/// Corresponds to iOS `OKPTradeRadarSelectType`.
#[derive(Debug, Clone, PartialEq)]
pub enum TradeRadarType {
    /// All signal types
    All,
    /// Whale transactions (large buys/sells)
    Whale,
    /// 5-minute price swing alerts
    PriceSwing,
    /// Token unlock events
    Unlocks,
    /// Most-traded coins in 24h
    Traders,
}

/// FFI mirror of core-trading's internal `FutureTitleIntent` (which is NOT
/// uniffi-exported upstream — retail-trading owns its own FFI enum and maps).
/// Kotlin `ruleType` mapping: `TRADE_FUTURE_NORMAL` / null -> `Normal`;
/// `TITLE_RULE_TYPE_GROUP_FILTER` -> `GroupFilter`.
/// 
/// TODO(core-trading): `FutureTitleIntent` needs an `Xperps` variant (Kotlin
/// explicit `ruleType = TRADE_XPERPS`; OKMarket production sites
/// `HomeFuturesMarketItemTemplate` / `CardFuturesMarketItemTemplate` /
/// `FormatFuturesVoFieldsUseCaseV3` / `MarketFavoritesCryptosEditViewModelV2`,
/// all with `swapWithSuffix = false`). Explicit XPERPS is NOT equivalent to
/// `Normal` auto-detection: applied to a regular dated future it renders the
/// bare family name with no expiry date. Once upstream adds the variant, add
/// `Xperps` HERE as a NEW variant (additive = FFI-safe; never rename / remove
/// / reorder existing variants) and map it through.
#[derive(Debug, Clone, PartialEq)]
pub enum TradeTitleRuleType {
    Normal,
    GroupFilter,
}

/// FFI mirror of core-trading's internal `SwapSuffixMode`. Kotlin mapping:
/// `swapWithSuffix = false` -> `None`; default (`true`) -> `Short`;
/// `swapUseLongSuffix = true` -> `Long` (OKTrade share-poster path only —
/// no OKMarket caller uses it today). FFI red line: additive changes only.
#[derive(Debug, Clone, PartialEq)]
pub enum TradeTitleSwapSuffix {
    None,
    Short,
    Long,
}

/// Section-level UI state (drives visible / hidden). The trending section is
/// shown only on `Success`; `Empty`/`Error` hide ONLY this section (NFR-002).
#[derive(Debug, Clone, PartialEq)]
pub enum TrendingTopicsSectionState {
    /// One-shot request in flight (initial).
    Loading,
    /// Loaded with at least one topic — section visible.
    Success,
    /// Loaded but no topics — section hidden.
    Empty,
    /// Request failed/timed out — section hidden.
    Error,
}

/// Event list type filter.
#[derive(Debug, Clone, PartialEq)]
pub enum UnlockEventType {
    Upcoming,
    Past,
    Recently,
}

/// Schedule chart period filter.
#[derive(Debug, Clone, PartialEq)]
pub enum UnlockSchedulePeriod {
    Week,
    Month,
    Year,
    All,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UpDownTimeZone {
    /// 24-hour mode (no time zone conversion)
    Mode24h,
    /// UTC+0 time zone
    ModeUtc0,
    /// UTC+8 time zone (e.g., China Standard Time)
    ModeUtc8,
}

/// Validation result for selection operations (UniFFI-compatible)
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationResultVo {
    /// Selection is valid
    Valid,
    /// Below minimum required
    BelowMinimum {
        current: u32,
        required: u32,
    },
    /// Above maximum allowed
    AboveMaximum {
        current: u32,
        limit: u32,
    },
    /// Attempting to cancel a non-cancelable pre-selected instrument
    CannotCancelPreSelected,
    /// Instrument not in allowed whitelist
    NotInAllowedList,
    /// Custom validation error
    CustomError {
        message: String,
    },
}

/// Instrument type filter for watchlist/favorites
#[derive(Debug, Clone, PartialEq)]
pub enum WatchlistInstType {
    /// All instrument types
    All,
    /// Spot trading pairs
    Spot,
    /// Futures/Swap contracts
    Future,
    /// Options contracts
    Option,
    /// DEX tokens
    Dex,
    /// Crypto/Coin mode
    Coin,
}

// -----------------------------------------------------------------------------
// 2. Records / Data Models (323)
// -----------------------------------------------------------------------------

/// Announcement module data for All Tab (UniFFI export)
#[derive(Debug, Clone, PartialEq)]
pub struct AllTabAnnouncementModule {
    pub items: Vec<SearchAnnouncementItem>,
    pub total_count: u32,
    pub has_more: bool,
}

/// Bots module data for All Tab (UniFFI export)
#[derive(Debug, Clone, PartialEq)]
pub struct AllTabBotsModule {
    pub items: Vec<SearchBotsItem>,
    pub total_count: u32,
    pub has_more: bool,
}

/// Campaign module data for All Tab (UniFFI export)
#[derive(Debug, Clone, PartialEq)]
pub struct AllTabCampaignModule {
    pub items: Vec<SearchCampaignItem>,
    pub total_count: u32,
    pub has_more: bool,
}

/// Content module data for All Tab overview (UniFFI export)
/// Contains only the overview subset (max 4 items: 2 KOL + 2 Feed)
#[derive(Debug, Clone, PartialEq)]
pub struct AllTabContentModule {
    pub overview_kol_list: Vec<SearchContentKolVo>,
    pub overview_feed_list: Vec<SearchContentFeedVo>,
    pub has_more_data: bool,
}

/// DEX module data for All Tab (UniFFI export)
#[derive(Debug, Clone, PartialEq)]
pub struct AllTabDexModule {
    pub items: Vec<SearchDexItem>,
    pub total_count: u32,
    pub has_more: bool,
}

/// Earn module data for All Tab (UniFFI export)
#[derive(Debug, Clone, PartialEq)]
pub struct AllTabEarnModule {
    pub items: Vec<SearchEarnItem>,
    pub total_count: u32,
    pub has_more: bool,
}

/// Event contract module data for All Tab (UniFFI export)
/// 
/// Used when AB+FR+data truth table is satisfied (signal A1 + F1 + non-empty items).
/// `has_options` mirrors V1 chip-switch logic in `SearchResultOverviewFragment.addEventAndOptionSection`:
/// when both events and options exist in the same ALL tab section, Android renders chips to
/// toggle between Events and Options sub-lists.
#[derive(Debug, Clone, PartialEq)]
pub struct AllTabEventContractModule {
    pub items: Vec<QuotationEventContractItemVo>,
    pub total_count: u32,
    pub has_more: bool,
    /// Whether the current search also produced non-empty Options results.
    /// Android uses this flag to decide whether to render Events/Options chip switcher.
    pub has_options: bool,
    /// Whether the current Options results contain any USDⓈ-margined (UM) options.
    /// 
    /// Mirrors OKMarket Kotlin `AllTabEventContractModule.hasUmOptions`. Used by Android
    /// to decide whether to render an additional CM/UM sub-tab in the Events/Options chip
    /// area when the user toggles to the Options side.
    pub has_um_options: bool,
}

/// Functions module data for All Tab (UniFFI export)
/// 
/// The "Functions" group in the ALL-tab overview (weighted immediately after Dex).
/// `items` is the preview (take 3) of the compliance-filtered `functions_list`
/// produced in the SSOT; Android renders these rows directly. `items` /
/// `total_count` / `has_more` mirror the sibling modules so the group participates
/// in weight-sorted ordering and drives its own "show more".
#[derive(Debug, Clone, PartialEq)]
pub struct AllTabFunctionsModule {
    pub items: Vec<SearchFunctionItem>,
    pub total_count: u32,
    pub has_more: bool,
}

/// Futures module data for All Tab (UniFFI export)
#[derive(Debug, Clone, PartialEq)]
pub struct AllTabFuturesModule {
    pub items: Vec<QuotationFuturesItemVo>,
    pub total_count: u32,
    pub has_more: bool,
}

/// Options module data for All Tab (UniFFI export)
/// Items are grouped by expiration date (chronologically sorted, max 3 items per date)
#[derive(Debug, Clone, PartialEq)]
pub struct AllTabOptionsModule {
    /// Options chain entries (canonical UM-first order)
    pub items: Vec<OptionsChainItemVo>,
    pub total_count: u32,
}

/// P2P module data for All Tab (UniFFI export)
#[derive(Debug, Clone, PartialEq)]
pub struct AllTabP2pModule {
    pub items: Vec<SearchP2pItem>,
    pub total_count: u32,
    pub has_more: bool,
}

/// Spot module data for All Tab (UniFFI export)
#[derive(Debug, Clone, PartialEq)]
pub struct AllTabSpotModule {
    pub items: Vec<QuotationSpotItemVo>,
    pub total_count: u32,
    pub has_more: bool,
}

/// All Tab state containing all modules (UniFFI export)
/// This is the UniFFI-compatible version of AllTabResult from aggregate_all_tab_use_case.
#[derive(Debug, Clone, PartialEq)]
pub struct AllTabState {
    /// Modules sorted by weight descending (only non-empty modules)
    pub modules: Vec<AllTabModule>,
    /// Full content data for Content tab (overview subset is in AllTabModule::Content)
    pub content_vo: Option<SearchContentVo>,
}

/// Trader module data for All Tab (UniFFI export)
#[derive(Debug, Clone, PartialEq)]
pub struct AllTabTraderModule {
    pub items: Vec<SearchTraderItem>,
    pub total_count: u32,
    pub has_more: bool,
}

/// A single date cell in the calendar month view.
#[derive(Debug, Clone, PartialEq)]
pub struct CalendarDateItem {
    /// Date key formatted as "MM/DD/YYYY"
    pub date_key: String,
    /// Visual state of this date cell
    pub state: DateItemState,
    /// Whether this date represents today
    pub is_today: bool,
}

/// A group of financial events occurring on the same calendar date.
#[derive(Debug, Clone, PartialEq)]
pub struct CalendarGroupedEvents {
    /// Date key formatted as "MM/DD/YYYY"
    pub date_key: String,
    /// Events scheduled for this date
    pub events: Vec<FinancialEventPo>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CandleVo {
    /// Millisecond timestamp string (e.g. "1700000000000").
    pub timestamp_ms: String,
    /// Raw price string — Android handles formatting.
    pub price: String,
}

/// Category detail response.
/// 
/// Corresponds to iOS `OKMCategoryDetailTopInfoModel`.
/// 
/// API: GET /priapi/v5/rubik/discover2/categories-detail
#[derive(Debug, Clone, PartialEq)]
pub struct CategoryDetailPo {
    /// Category ID
    pub category_id: String,
    /// Category name
    pub category_name: String,
    /// 24h average change percentage
    pub change_percent: String,
    /// Category description text
    pub category_desc: String,
    /// Market capitalization stats
    pub market_cap: CategoryDetailStatPo,
    /// 24h trading volume stats
    pub volume24h: CategoryDetailStatPo,
    /// Total social mentions stats
    pub total_mentions: CategoryDetailStatPo,
}

/// Market stat item (marketCap, volume24h, totalMentions).
/// 
/// Corresponds to iOS `OKMCategoryDetailTopInfoItemModel`.
#[derive(Debug, Clone, PartialEq)]
pub struct CategoryDetailStatPo {
    /// Stat value (e.g., "1234567890")
    pub volume: String,
    /// Change percentage (e.g., "0.45")
    pub change_percent: String,
    /// Trend chart data points as [timestamp, value] pairs.
    pub chart: Vec<Vec<String>>,
}

/// Category Group View Object
/// 
/// Corresponds to:
/// `com.okinc.market.quotation.ui.model.CategoryGroupVo`
/// 
/// Used for market category filtering (mainstream, new, hot, gainers, etc.)
#[derive(Debug, Clone, PartialEq)]
pub struct CategoryGroupVo {
    /// Raw category group data from trading API
    pub raw_category_group_data: Option<TradeGroupData>,
    /// Category group type for filtering out the raw vo list for displaying
    pub group_type: CategoryGroupType,
}

/// DEX Swap item view object — `CedefiSearchResultVO` (§3.3).
#[derive(Debug, Clone, PartialEq)]
pub struct CedefiSearchResultVo {
    pub chain_id: String,
    pub chain_name: String,
    pub chain_logo: String,
    pub token_symbol: String,
    pub token_name: String,
    pub token_contract_address: String,
    pub token_logo: String,
    /// Raw latest price string.
    pub price: String,
    /// Raw 24h change string.
    pub price_change_24h: String,
    pub price_change_utc0: String,
    pub price_change_utc8: String,
    pub liquidity: String,
    pub turn_over: String,
    pub market_cap: String,
    pub community_recognized: bool,
    pub risk_level: i32,
    pub cex_listing: bool,
    pub cex_full_name: String,
    pub cex_listing_name: String,
    pub verified: bool,
    pub boosted: bool,
    pub boost_percentage: String,
    pub boost_rate: String,
    pub hit_tags: Vec<i32>,
}

/// KOL certification badge VO (Orbit KOL identifier) (v6.116.0+)
/// Corresponds to CertBadgePo in Android.
/// 
/// Only fields consumed by the search cell are exposed (PRD §5: topTrader* fields are
/// intentionally NOT carried here — out of scope for this iteration, see PRD §3.2).
#[derive(Debug, Clone, PartialEq)]
pub struct CertBadgeVo {
    /// KOL certification status (NONE / ACTIVE)
    pub kol_cert_status: String,
    /// KOL certification source (TWITTER / OKX / orbit)
    pub kol_cert_source: String,
    /// Identity certifications (pre-translated, priority-sorted; client takes first 3 non-blank titles)
    pub identity_certs: Vec<IdentityCertVo>,
}

/// Charge Group View Object
/// 
/// Corresponds to:
/// `com.okinc.market.quotation.ui.model.ChargeGroupVo`
/// 
/// Used for fee/charge type filtering
#[derive(Debug, Clone, PartialEq)]
pub struct ChargeGroupVo {
    /// Group id
    pub id: String,
    /// Group key
    pub key: String,
    /// Group name
    pub name: String,
    /// Display name (for EEA compliance, USD may display as USDC)
    pub display_name: Option<String>,
    /// Sub-types in the group
    pub types: Vec<String>,
    /// Whether trade receive banner is displayed
    pub display_trade_receive_banner: bool,
    /// Charge group type for filtering out the raw vo list for displaying
    pub group_type: ChargeGroupType,
}

/// A section in the "About" area of the coin info page.
/// 
/// Sections are: introduction text, social media links, team members.
/// Native handles text truncation, layout heights, and rendering.
#[derive(Debug, Clone, PartialEq)]
pub struct CiAboutSection {
    /// Section type determines rendering strategy
    pub section_type: CiAboutSectionType,
    /// Introduction text (only for About section type)
    pub introduction: String,
    /// Social media links (only for LinkAndSocial section type)
    pub social_links: Vec<CiSocialLinkVo>,
    /// Team members (only for Members section type)
    pub members: Vec<CiCoinMemberVo>,
}

/// Investment organization / agency entry.
#[derive(Debug, Clone, PartialEq)]
pub struct CiAgencyPo {
    /// Organization name
    pub name: String,
    /// Logo URL
    pub logo: String,
}

/// Audit data from /priapi/v3/growth/price/currency-detail → data.tokenInfo.
/// 
/// All fields default to empty string on missing/null to distinguish
/// "not returned" from "returned with a value". Callers check
/// `certik_url.is_empty()` to decide whether to show the Audit section.
/// 
/// Every field goes through [`StringOrNumber`] because the upstream contract
/// types these as `string | number | null`: `certikLastAuditTime` is a JSON
/// number (BTC returns `0`), and any of them can be an explicit `null`.
/// `#[serde(default)]` alone only covers an absent key, so without the adapter
/// a number or `null` fails the whole `data.tokenInfo` decode and the Audit
/// section is silently hidden.
#[derive(Debug, Clone, PartialEq)]
pub struct CiAuditPo {
    /// CertiK audit report URL. Non-empty = show Audit section.
    pub certik_url: String,
    /// CertiK icon URL for light theme.
    pub certik_icon_light_url: String,
    /// Last audit timestamp in milliseconds as a string.
    /// "0" or empty = no audit date (display "--").
    pub certik_last_audit_time: String,
}

/// Token distribution allocation entry.
#[derive(Debug, Clone, PartialEq)]
pub struct CiCoinAllotPo {
    /// Allocation name (e.g. "Team", "Investors")
    pub name: String,
    /// Percentage string (e.g. "15.5")
    pub percent: String,
}

/// Response from GET /priapi/v5/rubik/public/coin-info
/// 
/// Contains coin fundamentals: rankings, ATH/ATL, supply, token distribution,
/// social media links, team members, and introduction text.
#[derive(Debug, Clone, PartialEq)]
pub struct CiCoinBaseInfoPo {
    /// Popularity ranking (e.g. "5")
    pub hot_rank: String,
    /// Market cap ranking (e.g. "1")
    pub market_cap_rank: String,
    /// Percentage of users who favorited (whole number, e.g. "5.23" = 5.23%)
    pub sel_num_per: String,
    /// All-time high price (USD)
    pub his_high_price: String,
    /// ATH timestamp in milliseconds
    pub his_high_price_ts: String,
    /// All-time low price (USD)
    pub his_low_price: String,
    /// ATL timestamp in milliseconds
    pub his_low_price_ts: String,
    /// Max supply ("-1" = infinite)
    pub max_sup: String,
    /// Max supply market cap (USD)
    pub max_sup_mar: String,
    /// Token distribution allocations
    pub coin_allot: Vec<CiCoinAllotPo>,
    /// Investment organizations
    pub agency: Vec<CiAgencyPo>,
    /// Whether token unlock info is available
    pub support_token_unlock: bool,
    /// Volume-to-market-cap ratio
    pub volume_market_cap_ratio: String,
    /// Industry classification
    pub industry: String,
    /// Coin description / introduction text
    pub introduce: String,
    /// Coin icon URL
    pub icon: String,
    /// Full name (e.g. "Bitcoin")
    pub full_name: String,
    /// Ticker symbol (e.g. "BTC")
    pub project: String,
    /// Social media links
    pub social_media_list: Vec<CiSocialMediaPo>,
    /// Team members
    pub members: Vec<CiCoinMemberPo>,
    /// Total supply (new field from coin-info API, FR-1)
    pub total_supply: String,
    /// Fully diluted valuation from CoinGecko (new field from coin-info API, FR-1)
    pub fdv_coingecko: String,
}

/// Response from GET /v2/support/info/announce/coinDataInfo
/// 
/// Contains market data: circulating supply, market cap, FDV, issue info.
/// 
/// The v2 API returns mixed field naming (camelCase + snake_case) and
/// mixed types (some fields are numbers, others are strings). All fields
/// are normalized to String for consistent downstream processing.
#[derive(Debug, Clone, PartialEq)]
pub struct CiCoinDataInfoPo {
    /// Circulating supply
    pub flow_total: String,
    /// Historical high price (API returns as `high_price` with underscore)
    pub high_price: String,
    /// Issue/launch price (API may return as number)
    pub issue_price: String,
    /// Issue timestamp in milliseconds (API may return as number)
    pub issue_time: String,
    /// Market capitalization (USD, API may return as number)
    pub market_cap: String,
    /// Coin name
    pub name: String,
    /// 24h volume (API may return as number)
    pub vol: String,
    /// Average hold time (API may return as number)
    pub hold_time: String,
    /// Fully diluted valuation (USD)
    pub fully_diluted_valuation: String,
}

/// Team member entry.
#[derive(Debug, Clone, PartialEq)]
pub struct CiCoinMemberPo {
    /// Avatar URL
    pub head_image: String,
    /// Bio text
    pub introduce: String,
    /// Member name
    pub name: String,
    /// Job title / position
    pub position: String,
}

/// Processed team member ready for display.
#[derive(Debug, Clone, PartialEq)]
pub struct CiCoinMemberVo {
    /// Avatar URL
    pub avatar_url: String,
    /// Member name
    pub name: String,
    /// Job title
    pub position: String,
    /// Bio text
    pub introduction: String,
}

/// Aggregated coin info data from all API responses.
/// 
/// Produced by CoinInfoManager::query_all() after parallel fetching.
/// Partial data is acceptable (any field can be default if its API failed).
#[derive(Debug, Clone, PartialEq)]
pub struct CiCompositeData {
    /// From /priapi/v5/rubik/public/coin-info
    pub base_info: CiCoinBaseInfoPo,
    /// From /v2/support/info/announce/coinDataInfo
    pub data_info: CiCoinDataInfoPo,
    /// From /priapi/v3/growth/price/currency-detail → data.tokenInfo.
    /// None when the audit request was not made or failed.
    pub audit_info: Option<CiAuditPo>,
}

/// A single display item in the coin overview metrics grid.
/// 
/// Produced by CoinInfoManager::build_overview_items(). Each item represents
/// one metric row (e.g. "Market Cap", "ATH", "Max Supply").
/// 
/// Native uses label_key/explanation_key for localization and value_recipe
/// for number formatting. The native side handles 2-column layout and heights.
#[derive(Debug, Clone, PartialEq)]
pub struct CiOverviewItem {
    /// Localization key for the item label (e.g. "pr_demotrade_text_market_cap")
    pub label_key: String,
    /// Localization key for tooltip explanation. Empty = no tooltip.
    pub explanation_key: String,
    /// Formatted primary display value as a recipe.
    /// Native applies locale-aware formatting using this recipe.
    pub value_recipe: MarketFormatRecipe,
    /// Item semantic type (controls jump behavior on tap)
    pub item_type: CiOverviewItemType,
    /// Jump index: 0=none, 100=hot rank page, 101=market cap rank page
    pub jump_index: u8,
    /// Secondary text (e.g. date for ATH/ATL, coin symbol for supply).
    /// Empty = no secondary text.
    pub secondary_text: String,
    /// Whether secondary_text represents a date (for ATH/ATL price+date display)
    pub secondary_is_date: bool,
    /// Whether this is the last item (native may apply special styling)
    pub is_last: bool,
}

/// State for the coin info overview page.
#[derive(Debug, Clone, PartialEq)]
pub struct CiOverviewState {
    /// Composite data from both APIs
    pub data: Option<CiCompositeData>,
    /// Overview metrics items (formatted, ordered, ready for 2-column grid)
    pub overview_items: Vec<CiOverviewItem>,
    /// About sections (introduction, social links, team members)
    pub about_sections: Vec<CiAboutSection>,
    /// Token distribution sorted by percent descending
    pub coin_allot_sorted: Vec<CiCoinAllotPo>,
    /// Investment organizations
    pub agencies: Vec<CiAgencyPo>,
    /// Whether token unlock section should be shown
    pub should_show_token_unlock: bool,
    /// Whether the coin symbol disables trading pair features
    pub is_coin_symbol_disabled: bool,
    /// Coin icon URL (for header display)
    pub icon: String,
    /// Coin full name (e.g. "Bitcoin")
    pub full_name: String,
    /// Coin project symbol (e.g. "BTC")
    pub project: String,
    /// Whether a network request is in progress
    pub is_loading: bool,
    /// Error message if the request failed
    pub error: Option<String>,
    /// Audit data from /priapi/v3/growth/price/currency-detail.
    /// None until load_audit() completes or when audit data is unavailable.
    pub audit_info: Option<CiAuditPo>,
    /// Why `audit_info` is absent, when its absence is NOT a business answer.
    /// 
    /// `None` together with an absent `audit_info` means the upstream answered
    /// and this coin genuinely has no audit, which is the only case that may be
    /// presented as "no Audit section". A `Some` value means no usable answer was
    /// obtained — currently `missing_price_identity` (the request was rejected
    /// before dispatch because no usable Price identity was available) or
    /// `request_failed`. Those are recoverable conditions: report them and allow
    /// a retry rather than treating them as a permanent absence of audit data.
    pub audit_unavailable_reason: Option<String>,
}

/// Processed social media link ready for display.
/// 
/// Filtered to supported types only (weChat excluded).
/// Contains localization key for the display title.
#[derive(Debug, Clone, PartialEq)]
pub struct CiSocialLinkVo {
    /// Social media type
    pub media_type: CiSocialMediaType,
    /// Display title localization key (e.g. "pr_demotrade_anchor_official_web")
    pub title_key: String,
    /// URL to open
    pub url: String,
}

/// Social media link from the coin-info API.
#[derive(Debug, Clone, PartialEq)]
pub struct CiSocialMediaPo {
    /// Type key (e.g. "officialUrl", "twitter", "github")
    pub name: String,
    /// Social media URL
    pub url: String,
    /// Display name
    pub title: String,
}

/// Error type for CountingBannerDataState
#[derive(Debug, Clone, PartialEq)]
pub struct CountingBannerError {
    pub message: String,
}

/// State for CountingBannerInteractor
#[derive(Debug, Clone, PartialEq)]
pub struct CountingBannerState {
    /// Banner carousel data (6-state async data state)
    pub banner_state: CountingBannerDataState,
    /// New coin info for parent tab badge
    pub new_coin_info: Option<NewCoinInfoVo>,
}

/// Cross-platform banner VO.
/// 
/// Provides raw data for platform-side rendering. Unlike the Android
/// `CountingBannerVo` which contains localized strings, this struct
/// provides the raw fields needed for the platform to construct the UI.
#[derive(Debug, Clone, PartialEq)]
pub struct CountingBannerVo {
    /// Instrument ID (e.g., "BTC-USDT")
    pub inst_id: String,
    /// Instrument type (e.g., "SPOT", "SWAP")
    pub inst_type: String,
    /// Display ID (alternative display name)
    pub display_id: String,
    /// Icon URL for the coin
    pub icon_url: String,
    /// Online timestamp in milliseconds
    pub online_time: i64,
    /// Wait-online countdown seconds (server-provided initial value)
    pub wait_online_time: Option<String>,
    /// Whether the instrument is ready for trading
    pub is_trade_ready: bool,
    /// Whether this is a pre-market item
    pub is_pre_market: bool,
    /// Card order in the banner carousel (1-indexed)
    pub card_order: i32,
    /// Click action type
    pub click_type: CountingBannerClickType,
    /// Deeplink URL (only for OKP banners)
    pub deeplink_url: Option<String>,
    /// OKP banner ID (only for OKP banners)
    pub okp_id: Option<i32>,
    /// Banner type discriminator
    pub banner_type: CountingBannerType,
    /// OKP banner title (for NewCoin, platform formats from instId)
    pub okp_title: Option<String>,
    /// OKP banner subtitle
    pub okp_subtitle: Option<String>,
    /// OKP icon for light theme
    pub icon_light: Option<String>,
    /// OKP icon for dark theme
    pub icon_dark: Option<String>,
    /// Open method for trading
    pub open_method: Option<String>,
    /// Notice URL (Chinese)
    pub notice_url_cn: Option<String>,
    /// Notice URL (English)
    pub notice_url_en: Option<String>,
    /// Whether canTrade field from API (for platform-side logic)
    pub can_trade: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CryptoChartState {
    pub candles: Vec<CandleVo>,
    /// Latest ticker value object (price + change_pct + base_price).
    pub ticker: Option<CryptoTickerVo>,
    pub is_loading_candles: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CryptoDetailState {
    pub ccy: String,
    pub symbol: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CryptoEarnState {
    pub categories: Vec<EarnCategoryVo>,
    pub is_loading: bool,
    pub error: Option<String>,
}

/// Raw inputs for a Crypto list item (from `QuotationCryptoItemVo`).
#[derive(Debug, Clone, PartialEq)]
pub struct CryptoItemFormatInput {
    /// Raw change ratio decimal. `None` -> empty change recipe.
    pub change_ratio: Option<String>,
    /// USD price to convert to fiat. `None` -> no price recipe.
    pub price_usd: Option<String>,
}

/// Recipes for a Crypto list item.
#[derive(Debug, Clone, PartialEq)]
pub struct CryptoItemRecipes {
    /// Change ratio recipe — always present (may be an empty recipe).
    pub change: MarketFormatRecipe,
    /// Fiat price recipe, or `None`.
    pub price: Option<MarketFormatRecipe>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CryptoProfileState {
    pub profile: Option<CryptoProfileVo>,
    pub is_loading: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CryptoProfileVo {
    pub icon: String,
    /// Short name from API (e.g. "BTC", "ETH"). Corresponds to V1 `detail.project`.
    pub project: String,
    /// User-facing symbol from TradingCore (e.g. "xAMAT"), falling back to `project`.
    pub display_project: String,
    pub full_name: String,
    /// Plain text project introduction. May be long — Android truncates at 4 lines.
    pub introduce: String,
    pub social_media_list: Vec<SocialMediaVo>,
    pub members: Vec<MemberVo>,
    /// Unix timestamp in milliseconds, formatted as String (e.g. "1231006505000").
    pub issue_time: String,
    /// Issue price in USD, raw string (e.g. "0.0001").
    pub issue_price: String,
    /// All-time high price in USD, raw string.
    pub high_price: String,
    /// 24-hour trading volume in USD, raw string.
    pub vol_24h: String,
    /// Circulating market cap in USD, raw string.
    pub market_cap: String,
    /// Circulating supply (token count), raw string.
    pub flow_total: String,
    /// Issue price converted to the user's valuation currency (fiat price recipe).
    pub issue_price_recipe: MarketFormatRecipe,
    /// All-time high converted to the user's valuation currency (fiat price recipe).
    pub high_price_recipe: MarketFormatRecipe,
    /// 24h volume converted to the user's valuation currency (abbreviated amount recipe).
    pub vol_24h_recipe: MarketFormatRecipe,
    /// Market cap converted to the user's valuation currency (abbreviated amount recipe).
    pub market_cap_recipe: MarketFormatRecipe,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CryptoTickerVo {
    /// Raw price string, e.g. "70214.5"
    pub last: String,
    /// Raw change percentage as decimal string, e.g. "-0.0177" (UI formats as -1.77%)
    pub change_pct: String,
    /// Base price used for change calculation, e.g. "71468"
    pub base_price: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CryptoTradeListState {
    pub trade_products: Option<TradeProductsVo>,
    pub is_loading: bool,
    pub error: Option<String>,
}

/// A single display row in the contract info view.
/// 
/// Represents one key-value row. Native renders these in order,
/// using field_type to determine the visual style and action
/// to determine tap behavior.
#[derive(Debug, Clone, PartialEq)]
pub struct CtrContractField {
    /// Localization key for the row label (e.g., "pr_trade_info_list_tick_size")
    pub label_key: String,
    /// Pre-formatted display value (e.g., "0.01", "1-125", "BTC-USD 65432.1")
    pub value: String,
    /// Secondary value or unit suffix (e.g., "BTC", "sheets")
    /// Empty string if not applicable.
    pub suffix: String,
    /// Rendering style for this field
    pub field_type: CtrFieldType,
    /// Tap action for this field
    pub action: CtrFieldAction,
    /// Whether this field should be visible.
    /// Rust computes visibility based on business rules (L005-L012);
    /// native hides the row when false.
    pub is_visible: bool,
    /// Optional dialog content (populated when action is a dialog type).
    /// Native reads this to build the dialog/alert when user taps.
    pub dialog: Option<CtrDialogContent>,
    /// Optional raw-value numeric format descriptor (CR-3). When present, Native
    /// localizes each raw numeric segment and reassembles per `kind`; the
    /// pre-existing `value` is a forward-compatible fallback. Rust bakes NO locale
    /// grouping/decimal/percent/range separators into the raw numbers (CC-1).
    /// Additive optional field — never retype/remove existing fields (CC-2).
    pub format: Option<CtrValueFormat>,
}

/// Ordered list of fields for a biz type, ready for native rendering.
#[derive(Debug, Clone, PartialEq)]
pub struct CtrContractFieldList {
    /// Business type this field list belongs to
    pub biz_type: CtrBizType,
    /// Ordered fields (native renders top-to-bottom)
    pub fields: Vec<CtrContractField>,
}

/// State for the contract info page.
#[derive(Debug, Clone, PartialEq)]
pub struct CtrContractInfoState {
    /// Active business type
    pub biz_type: CtrBizType,
    /// Ordered, visibility-annotated field list for native rendering
    pub field_list: CtrContractFieldList,
    /// Whether a network request is in progress
    pub is_loading: bool,
    /// Error message if the last request failed
    pub error: Option<String>,
}

/// Structured content for a dialog/alert triggered by an info icon tap.
#[derive(Debug, Clone, PartialEq)]
pub struct CtrDialogContent {
    /// Dialog title localization key
    pub title_key: String,
    /// Ordered sections within the dialog
    pub sections: Vec<CtrDialogSection>,
    /// "Learn more" URL localization key. Empty when the dialog has no Learn more
    /// action. Appended last to keep the UniFFI Record positionally forward-compatible
    /// (native only reads this field; it never constructs the record).
    pub learn_more_url_key: String,
}

/// A section within a dialog.
/// 
/// Each section has a localization key for the label
/// and a pre-formatted value string. Complex formulas
/// are expressed as localized template keys with parameter
/// substitution done in Rust.
#[derive(Debug, Clone, PartialEq)]
pub struct CtrDialogSection {
    /// Localization key for section label. Empty for unlabeled content.
    pub label_key: String,
    /// Pre-formatted value text. May contain line breaks.
    pub value: String,
    /// Optional raw-value numeric format descriptor (CR-5). When present, Native
    /// localizes the raw numeric segment(s) of this section instead of rendering a
    /// Rust-baked format. Additive optional field (CC-2); Rust bakes no locale
    /// grouping/percent/range separators into the raw numbers (CC-1).
    pub format: Option<CtrValueFormat>,
}

/// Static instrument fields provided by native.
/// 
/// These come from OKTInstrument + OKTFullInstrument on iOS,
/// or the equivalent TradingCore cache on Android.
/// Rust does NOT fetch these; native passes them in.
#[derive(Debug, Clone, PartialEq)]
pub struct CtrInstrumentInput {
    pub inst_id: String,
    pub inst_type: String,
    pub inst_family: String,
    pub base_ccy: String,
    pub settle_ccy: String,
    pub ct_val: String,
    pub ct_val_ccy: String,
    pub tick_sz: String,
    pub lot_sz: String,
    pub min_sz: String,
    pub max_mkt_sz: String,
    pub max_mkt_amt: String,
    pub max_lmt_amt: String,
    pub ct_mult: String,
    /// true for USDT-margined (positive), false for coin-margined (negative)
    pub is_positive: bool,
    /// Delivery/expiry time in epoch milliseconds (as string). Empty for Spot/Swap.
    pub exp_time: String,
    /// "C" or "P", empty for non-options
    pub opt_type: String,
    /// Strike price, empty for non-options
    pub stk: String,
    pub price_limit_status: String,
    pub spot_lp_h1: String,
    pub spot_lp_h2: String,
    pub spot_lp_h3: String,
    pub spot_lp_j1: String,
    pub spot_lp_j2: String,
    pub lp_x1: String,
    pub lp_x2: String,
    pub lp_y1: String,
    pub lp_y2: String,
    pub lp_z1: String,
    pub lp_z2: String,
    pub lp_c1: String,
    pub lp_c2: String,
    pub within_ten_mins_of_contract_generation: String,
    pub ten_mins_after_contract_generation: String,
    pub index_max_threshold_rate: String,
    pub px_limit_factor: String,
    pub mas_pa_delta_threshold: String,
    pub spread_width_obl_min_base_spread_v2: String,
    pub spread_width_obl_delta_factor_gt_haft_v2: String,
    pub spread_width_obl_delta_factor_lt_haft_v2: String,
    pub spread_width_obl_min_base_spread_v3: String,
    pub spread_width_obl_delta_factor_gt_haft_v3: String,
    pub spread_width_obl_delta_factor_lt_haft_v3: String,
    pub future_settlement: bool,
    pub inst_family_quote_ccy: String,
    pub user_position_max_percent: String,
    pub user_position_max_value_usd: String,
    pub platform_side_position_max_value_usd: String,
    /// mkUly field for index-tickers API call
    pub mk_uly: String,
    /// Alias for futures (e.g., "this_week", "next_week", "quarter")
    pub alias: String,
    /// Whether this instrument supports leverage (isSupportLever)
    pub is_support_lever: bool,
    /// Leverage value from native instrument (e.g. "100" for SwapInstrument.lever).
    /// Used as fallback when the lever interval API doesn't return data (Swap/Futures).
    pub lever: String,
    /// Whether this instrument is a tradFi (equity/stock) contract
    pub is_trad_fi: bool,
    /// Whether this instrument is a pre-market pair
    pub is_pre_market: bool,
    /// Lower price limit band (from products API). Empty when absent.
    pub lower_price_limit_band: String,
    /// Higher price limit band (from products API). Empty when absent.
    pub higher_price_limit_band: String,
    /// Serialized tick config entries: Vec of (max_price, tick_size) pairs
    /// Native builds this from biz.getTradeConfig(instFamily).tickConfig
    pub tick_config: Vec<CtrTickConfigEntry>,
}

/// A single tick size configuration entry for options.
#[derive(Debug, Clone, PartialEq)]
pub struct CtrTickConfigEntry {
    /// Upper price boundary for this tick size tier
    pub max_price: String,
    /// Tick size for prices below max_price
    pub tick_size: String,
}

/// Structured numeric-format descriptor for a contract-info field value.
/// 
/// All numbers in `raw_numbers` are raw (no locale grouping/decimal, no percent
/// sign, no range separator). `precisions[i]` (when present and >= 0) is the
/// fraction-digit count Native should render for `raw_numbers[i]`; -1 or a
/// missing entry means "render at the raw precision". `symbol` carries a
/// non-numeric prefix (index symbol, currency sign) and `unit` a suffix
/// (currency/unit). `boundaries` carries any extra literal segments needed to
/// reassemble a composite value (e.g. option-tick tier boundaries).
#[derive(Debug, Clone, PartialEq)]
pub struct CtrValueFormat {
    /// Raw numeric segments in display order (no locale formatting applied).
    pub raw_numbers: Vec<String>,
    /// How Native reassembles the localized segments.
    pub kind: CtrValueKind,
    /// Per-number fraction digits (parallel to `raw_numbers`; -1 or empty = raw
    /// precision).
    pub precisions: Vec<i32>,
    /// Non-numeric prefix/symbol (index symbol, currency sign). Empty if none.
    pub symbol: String,
    /// Unit/currency suffix (e.g. base_ccy, ct_val_ccy). Empty if none.
    pub unit: String,
    /// Extra literal boundary segments for composite kinds (e.g. option-tick tier
    /// max prices). Empty for simple kinds.
    pub boundaries: Vec<String>,
}

/// FFI-flat current-currency record (settings currency preference snapshot).
/// Flat FFI-safe scalars only (retail-trading exported-record red line).
#[derive(Debug, Clone, PartialEq)]
pub struct CurrencyInfoRecord {
    /// Server-assigned currency id.
    pub currency_id: i32,
    /// ISO code, e.g. `"USD"` / `"TRY"`.
    pub iso_code: String,
    /// Localized display name, e.g. `"US Dollar"`.
    pub display_name: String,
    /// Currency symbol, e.g. `"$"`.
    pub symbol: String,
    /// Fiat display precision (decimal places).
    pub precision: i32,
    /// Whether the currency uses the premium (index-rate) channel.
    pub is_premium: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CustomHomeRankItemPo {
    pub turn_over: String,
    pub volume: String,
    pub display_id: String,
    pub icon: String,
    pub change_per_day24h: String,
    pub change_per_day_utc0: String,
    pub change_per_day_utc8: String,
    pub inst_id: String,
    pub inst_type: String,
    pub last_price: String,
    pub last_usd_price: String,
    pub commodity_tag: String,
    pub pre_ipo_tag: bool,
    pub has_recent_earnings: bool, // default = false
}

#[derive(Debug, Clone, PartialEq)]
pub struct CustomHomeRankListPo {
    pub spot: Vec<CustomHomeRankItemPo>,
    pub futures: Vec<CustomHomeRankItemPo>,
}

/// DEX Chain Group View Object - Chain information wrapper
/// 
/// Corresponds to:
/// `com.okinc.market.quotation.ui.model.dex.DexChainGroupVo`
#[derive(Debug, Clone, PartialEq)]
pub struct DexChainGroupVo {
    /// Chain ID (e.g., "501", "8453", "192")
    /// Note: Boosted chain ID is "0"
    pub chain_id: String,
    /// Chain name (e.g., "Solana", "Base", "X Layer")
    pub chain_name: String,
    /// Chain logo URL
    pub chain_logo_url: String,
    /// Whether this chain is boosted
    pub is_boost: bool,
    /// Whether this represents all chains
    pub is_all_group: bool,
    /// Whether this chain is RWA (Real-World Assets / TradFi)
    pub is_rwa: bool,
    /// Whether to show zero fees
    pub show_zero_fee: bool,
}

/// DEX Perp item view object (§2.3.1).
#[derive(Debug, Clone, PartialEq)]
pub struct DexPerpItemVo {
    pub asset_id: String,
    pub display_symbol: String,
    pub token_name: String,
    pub token_logo: String,
    pub tag: String,
    /// Raw latest price string.
    pub price: String,
    /// Raw signed 24h change percent string (no `%`).
    pub price_change_24h: String,
    pub change_utc0: String,
    pub change_utc8: String,
    pub chain_id: String,
    pub chain_name: String,
    pub token_contract_address: String,
    pub is_favorite: bool,
    pub deeplink: String,
    /// Full token name for the subtitle, e.g. `Bitcoin`.
    pub full_name: Option<String>, // default = None
    /// 24h turnover as a raw USD numeric string for the subtitle.
    pub turn_over_24h: Option<String>, // default = None
}

/// DEX Spot item view object (§2.3.2). Same shape as [`DexPerpItemVo`],
/// distinct type so the Android layer can bind the two sections separately.
#[derive(Debug, Clone, PartialEq)]
pub struct DexSpotItemVo {
    pub asset_id: String,
    pub display_symbol: String,
    pub token_name: String,
    pub token_logo: String,
    pub tag: String,
    pub price: String,
    pub price_change_24h: String,
    pub change_utc0: String,
    pub change_utc8: String,
    pub chain_id: String,
    pub chain_name: String,
    pub token_contract_address: String,
    pub is_favorite: bool,
    pub deeplink: String,
    /// Full token name for the subtitle, e.g. `Bitcoin`.
    pub full_name: Option<String>, // default = None
    /// 24h turnover as a raw USD numeric string for the subtitle.
    pub turn_over_24h: Option<String>, // default = None
}

#[derive(Debug, Clone, PartialEq)]
pub struct DiscoverCryptoListState {
    pub subscription_id: u32,
    pub crypto_list: Vec<QuotationCryptoItemVo>,
    pub is_loading_list: bool,
    pub list_error: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DiscoverCryptoState {
    pub category_groups: Vec<CategoryGroupVo>,
    pub is_loading_groups: bool,
    pub groups_error: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DiscoverDexListState {
    pub subscription_id: u32,
    pub dex_list: Vec<QuotationDexItemVo>,
    pub is_loading_list: bool,
    pub list_error: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DiscoverDexState {
    pub chain_groups: Vec<DexChainGroupVo>,
    pub is_loading_groups: bool,
    pub groups_error: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DiscoverEventContractGroupState {
    /// Full set of available (method, freq) groups (replaces the old
    /// `available_methods: Vec<EventContractMethod>`). Native UIs build one
    /// tab per entry so the freq dimension is preserved end-to-end.
    pub available_method_groups: Vec<EventMethodGroupVo>,
    pub available_currencies: Vec<String>,
    pub is_loading_groups: bool,
    pub groups_error: Option<String>,
    /// 3-list `[freqs, methods, coins]` filter-dimension snapshot — drives the
    /// V2 dynamic chip row (mirrors V1 Opt's `cachedDimensions` produced by
    /// the Kotlin `OkEventsBiz.getEventFilterDimensions()` helper). Appended
    /// at the end of the struct so the UniFFI field layout stays
    /// backward-compatible with the previous state ABI; existing native
    /// bindings continue to deserialise the earlier fields unchanged.
    pub event_filter_dimensions: Vec<Vec<EventFilterDimVo>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DiscoverEventContractListState {
    pub subscription_id: u32,
    pub updown_cards: Vec<EventContractUpDownCardVo>,
    pub above_groups: Vec<EventContractAboveGroupVo>,
    pub is_loading_list: bool,
    pub list_error: Option<String>,
    /// Currently selected date per base currency (mirrors Manager state).
    /// Native consumers use this to render the selected chip; updates flow
    /// through the `select_date_for_currency` action.
    pub selected_date_map: Vec<SelectedDateVo>,
    /// Distinct series keys in HTTP inst-list response order, refreshed each
    /// time the subscribe pipeline emits. Drives entry-B Event tab card
    /// order on the native side (§2.2.1.5 6.9). Empty when the HTTP fetch
    /// failed during the last cache refresh — consumers fall back to the
    /// legacy alphabetical builder. Appended at the end of the struct to
    /// keep the UniFFI field layout backward-compatible with the previous
    /// state ABI; existing native bindings continue to deserialise the
    /// earlier fields unchanged.
    pub series_keys_ordered_by_http: Vec<SeriesKeyVo>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DiscoverFuturesListState {
    pub subscription_id: u32,
    pub futures_list: Vec<QuotationFuturesItemVo>,
    pub is_loading_list: bool,
    pub list_error: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DiscoverFuturesPageState {
    pub page_list: Vec<FuturesPageVo>,
    pub is_loading: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DiscoverMarketsRootState {
    /// Final gated L1 page list (cloud or local — both post-gate; the
    /// cloud-vs-local outcome is a Rust-side log, not UI data).
    pub tabs: Vec<DiscoverL1Page>,
    pub is_loading: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DiscoverOptionsChainState {
    pub options_chain_list: Vec<OptionsChainItemVo>,
    pub is_loading: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DiscoverOptionsQuoteGroupState {
    pub category_groups: Vec<OptionsCategoryGroupVo>,
    pub exp_time_periods: Vec<String>,
    pub is_loading_groups: bool,
    pub groups_error: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DiscoverOptionsQuoteListState {
    pub subscription_id: u32,
    pub options_list: Vec<QuotationOptionsItemVo>,
    pub is_loading_list: bool,
    pub list_error: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DiscoverOptionsSimpleState {
    pub simple_item_list: Vec<OptionsSimpleItemVo>,
    pub is_loading: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DiscoverSpotListState {
    pub subscription_id: u32,
    pub spot_list: Vec<QuotationSpotItemVo>,
    pub is_loading_list: bool,
    pub list_error: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DiscoverSpotPageState {
    pub page_list: Vec<SpotPageVo>,
    pub is_loading: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DiscoverTradFiPageState {
    /// L2 tabs in cloud order, already narrowed by the category data gate. Empty
    /// while loading, and also when the cloud scene carries nothing usable (the
    /// L1 tab is not offered in that case). One entry means the host shows the
    /// business line as the tab strip; several means the categories are the strip
    /// and the business line moves into a dropdown.
    pub tabs: Vec<TradFiL2TabVo>,
    /// Page-level business-line availability: both layouts render the other
    /// dimension from these two flags (tab strip, or the top-right dropdown).
    pub has_spot_tab: bool,
    pub has_futures_tab: bool,
    pub is_loading: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EarnCategoryVo {
    /// Display name of the earn product category (e.g. "Savings", "Staking").
    pub name: String,
    pub products: Vec<EarnProductVo>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EarnProductVo {
    pub rate: String,
    /// Server integer IDs: 1=New, 2=Hot, 3=Bonus.
    pub labels: Vec<u32>,
    /// Display subtitles. First element is primary subtitle; second (if present) is secondary.
    pub subtitle: Vec<String>,
    /// Complete deeplink URL — Android passes directly to DeeplinkService.
    pub subscription_url: String,
}

/// Stable UniFFI presentation record for an earnings calendar event.
/// 
/// Business derivation from the wire DTO is intentionally owned by the calendar domain.
#[derive(Debug, Clone, PartialEq)]
pub struct EarningsDataPo {
    pub calendar_id: String,
    pub version: i64,
    pub icon: String,
    pub display_symbol: String,
    pub event_timestamp: String,
    pub earnings_date: String,
    /// Market-local release clock from the service (HH:mm). This is a display value,
    /// so native clients must not apply a device-timezone conversion to it.
    pub release_time: String,
    pub market_timezone: String,
    pub release_session: EarningsReleaseSession,
    pub schedule_status: EarningsScheduleStatus,
    pub publication_status: EarningsPublicationStatus,
    pub analyzed: bool,
    pub market_status: Option<String>,
    pub fiscal_period: Option<String>,
    pub eps_estimated_display: Option<String>,
    pub eps_actual_display: Option<String>,
    pub revenue_estimated_display: Option<String>,
    pub revenue_actual_display: Option<String>,
    pub eps_beat_miss: Option<EarningsBeatMiss>,
    pub revenue_beat_miss: Option<EarningsBeatMiss>,
    pub summary_text: Option<String>,
    pub can_add_to_calendar: bool,
    pub reminder_title: String,
    pub reminder_timestamp: i64,
    pub jump_target: Option<EarningsJumpTargetVo>,
}

/// Authoritative instrument identity and destination for earnings navigation.
#[derive(Debug, Clone, PartialEq)]
pub struct EarningsJumpTargetVo {
    pub inst_id: String,
    pub inst_type: String,
    pub destination: EarningsJumpDestination,
}

/// Earnings events grouped for the compact Home calendar presentation.
#[derive(Debug, Clone, PartialEq)]
pub struct EarningsPreviewDatePo {
    pub date_key: String,
    pub events: Vec<EarningsDataPo>,
}

/// Economic/macro calendar event data.
/// 
/// Corresponds to a single economic indicator release or scheduled event.
#[derive(Debug, Clone, PartialEq)]
pub struct EcoDataPo {
    /// Unique calendar entry ID
    pub calendar_id: String,
    /// Event template ID
    pub event_id: String,
    /// Event timestamp (13-digit milliseconds, may arrive as number or string)
    pub time_stamp: String,
    /// Icon URL for the event
    pub icon: String,
    /// Description / event name
    pub desc: String,
    /// Actual reported value
    pub actual: String,
    /// Previous value
    pub previous: String,
    /// Consensus / forecast value
    pub consensus: String,
}

/// Above group with items (FFI).
/// 
/// The semantic depends on which builder produced the group:
/// - `build_above_groups` (Explore path): one group per (ccy, date). `items`
/// contains all items for that (ccy, date); `available_dates_ts` is a single-
/// element list `[date_label_ts]` so consumers can treat both shapes uniformly.
/// - `build_above_groups_per_currency` (Discover / NavSearch path): one group
/// per ccy; `date_label_ts` is the currently selected date (or the earliest
/// chronologically if none was selected), `available_dates_ts` is the full
/// list of day-start timestamps for that ccy sorted ascending, `items`
/// contains only the selected-date items, and `total_vol` is aggregated over
/// those items.
/// Native UIs render date chips from `available_dates_ts` (formatting each
/// timestamp via their own locale-aware date formatter) and highlight the
/// chip whose timestamp matches `date_label_ts`.
/// 
/// Both `date_label_ts` and entries in `available_dates_ts` are UTC day-start
/// timestamps in milliseconds (i.e. 00:00:00 UTC of the calendar day on which
/// the underlying `exp_time_ms` falls). `0` indicates "no date".
#[derive(Debug, Clone, PartialEq)]
pub struct EventContractAboveGroupVo {
    pub base_currency: String,
    pub date_label_ts: i64,
    pub group_key: String,
    pub total_vol: String,
    pub items: Vec<QuotationEventContractItemVo>,
    /// Available day-start timestamps (UTC, ms) for this base currency,
    /// sorted ascending.
    pub available_dates_ts: Vec<i64>,
}

/// UpDown aggregated card (FFI) — one per (ccy, freq) pair
#[derive(Debug, Clone, PartialEq)]
pub struct EventContractUpDownCardVo {
    pub inst_id: String,
    pub base_currency: String,
    /// Frequency of the underlying items (e.g. "FIVE_MIN" / "FIFTEEN_MIN" /
    /// "HOURLY"). Native UIs render this directly instead of inferring it
    /// from a hardcoded tab label.
    pub freq: String,
    pub probability: String,
    /// Server-clamped UI display price (FSP-9179) sourced from the first item in
    /// the (ccy, freq) group. Falls back to `"--"` when the underlying value is
    /// absent. Subject to the same invalid → `"0.5"` clamp applied per-item by
    /// `ApplyEventContractTickersUseCase` (FSP-9205).
    pub display_price: String,
    pub aggregated_vol: String,
}

/// One filter-dimension entry, used in the 3-list `[freqs, methods, coins]`
/// payload returned by `events_biz.get_event_filter_dimensions()` and exposed
/// through `DiscoverEventContractGroupState.event_filter_dimensions`.
/// 
/// Each dimension's element carries:
/// - the raw filter value for that dimension (only one of `freq`/`method`/`coin`
/// is the dimension key; the other two reflect the representative instrument's
/// identity so native consumers can render labels uniformly)
/// - `representative_inst_id` for V1 native title helper lookup
/// (`EventContractTitleHelper.displayForFreq/Method/Coin(instId)`).
/// 
/// Empty sub-list at a given dimension index means no data for that dimension
/// and the native side should hide the corresponding chip.
#[derive(Debug, Clone, PartialEq)]
pub struct EventFilterDimVo {
    /// Event method, e.g. "PRICE_UP_DOWN" / "PRICE_ABOVE".
    pub method: String,
    /// Frequency, e.g. "FIVE_MIN", "FIFTEEN_MIN", "DAILY".
    pub freq: String,
    /// Underlying coin, e.g. "BTC". Empty when not applicable for this dim.
    pub coin: String,
    /// inst_id of one representative instrument for label resolution via
    /// native title helpers.
    pub representative_inst_id: String,
    /// UI-ready chip label, i18n-resolved in Rust via core-trading's
    /// `display_for_freq/method/coin` (`CDKey`/`FormattedCDKey` carry the
    /// localization key + params; native renders them against string
    /// resources). Removes the last dependency on the V1
    /// `getEventFilterDimensions(biz)` label path, which returned empty when
    /// the Kotlin events biz was not initialized on the Rust/AB path.
    pub display: DisplayString,
}

/// Method+freq group VO that mirrors core-trading's `EventMethodGroupRecord`
/// inside retail-trading, plus a [`representative_inst_id`] for downstream
/// rendering.
/// 
/// `representative_inst_id` carries the inst_id of any one instrument within
/// the (method, freq) group so native consumers can call V1 helpers like
/// `OKTradeCoreModule.eventService.getGroupName(instId)` without synthesising
/// a fake instId. The "first instrument seen during dedup" is used (callers
/// must construct the VO from a real instrument list); unknown method
/// strings are skipped at the construction boundary so this VO only ever
/// contains methods retail-trading natively understands.
#[derive(Debug, Clone, PartialEq)]
pub struct EventMethodGroupVo {
    pub method: EventContractMethod,
    /// Frequency, e.g. "FIVE_MIN", "FIFTEEN_MIN", "FORTY_FIVE_MIN",
    /// "HOURLY", "TWO_HOURLY", "DAILY".
    pub freq: String,
    /// inst_id of any one instrument from this (method, freq) group.
    /// Used as the input to V1 native title helpers (Kotlin
    /// `OKTradeCoreModule.eventService.getGroupName(instId)` /
    /// `getEventShortName(instId)`).
    pub representative_inst_id: String,
}

/// Flat, FFI-safe filter option exposed to the UniFFI Group interactor.
/// 
/// The crate-internal [`ExchangeOsFilterPo`] is not FFI-safe (it is a `Po`), so the
/// Group interactor exposes the DEX filters as this exported `uniffi::Record`
/// (mirrors how swap chain groups are exposed via the exported [`DexChainGroupVo`]).
#[derive(Debug, Clone, PartialEq)]
pub struct ExchangeOsFilterVo {
    /// Filter key (passed back as the `filter` subscribe argument).
    pub key: String,
    /// Filter display name.
    pub name: String,
    /// Swap-only boost tab flag (routes to `/dex/boost/rank`).
    pub boost: bool,
}

/// Flat, FFI-safe market item. Fields mirror Kotlin `NavSearchExosDexItemVo`.
/// 
/// Price / turnover / market cap stay raw strings — the Kotlin side formats them
/// (`FormatDexVoFieldsUseCaseV3`), matching the DEX V2 behavior.
#[derive(Debug, Clone, PartialEq)]
pub struct ExchangeOsMarketItemVo {
    /// Stable identity for perp/spot rows.
    pub asset_id: String,
    /// Chain ID (swap only; empty for perp/spot).
    pub chain_id: String,
    /// Token contract address (swap only; empty for perp/spot).
    pub token_contract_address: String,
    /// Display symbol, e.g. "BTC-USDT".
    pub symbol: String,
    /// Route/title name. For perp/spot search rows this follows Kotlin V1 and uses
    /// `tokenName` when present; ranking rows fall back to API `name`.
    pub name: String,
    /// Full token name, e.g. "Bitcoin". Perp/spot list rows use this as subtitle copy.
    pub full_name: String,
    /// Token logo URL.
    pub logo_url: String,
    /// Chain logo URL (swap only).
    pub chain_logo_url: String,
    /// Current price (raw string; formatted by native).
    pub price: String,
    /// Timezone-picked change (raw string; formatted by native).
    pub price_change: String,
    /// 24h turnover (raw string; formatted by native).
    pub turn_over: String,
    /// Market cap (raw string; formatted by native).
    pub market_cap: String,
    /// Swap boost badge flag.
    pub boosted: bool,
    /// Swap boost rate string.
    pub boost_rate: String,
    /// Business-line tag copy ("DEX" / "DEX Perp").
    pub tag: String,
    /// Whether this item is in the user's watchlist (favorites). Set by the
    /// manager's favorites-merge stream (key: `chainId:contract` for swap,
    /// `assetId` for perp/spot); the raw `From<&InnerVo>` mapping leaves it
    /// `false` (favorites are joined at the manager layer, not at Po->Vo).
    pub is_watching: bool,
}

/// State for ExploreBoostCardInteractor.
/// 
/// No `should_show` field — the `isShowDexTrade()` gate is an Android platform
/// service with no Rust equivalent; the V2 Fragment applies it before subscribing.
#[derive(Debug, Clone, PartialEq)]
pub struct ExploreBoostCardState {
    pub boost_state: ExploreBoostDataState,
}

/// Flat, FFI-safe record for one boost activity card.
/// 
/// Android derives localized subtitle and button text from (stage, joined,
/// claimed). No color/direction semantics — only raw data.
#[derive(Debug, Clone, PartialEq)]
pub struct ExploreBoostCardVo {
    /// Activity identifier (maps to V1 BoostInfoVo.activityId).
    pub activity_id: i64,
    /// Activity title (may be empty string).
    pub title: String,
    /// Wide banner image URL (maps to wideBanner field).
    pub banner_url: String,
    /// Logo image URL (may be empty string).
    pub logo_url: String,
    /// Chain identifier for the activity.
    pub chain_id: i32,
    /// Computed stage (pure function of trusted_now_ms + timestamps).
    pub stage: ExploreBoostStage,
    /// Whether the current user has joined this activity.
    pub joined: bool,
    /// Whether the current user has claimed rewards.
    pub claimed: bool,
    /// Raw join-window open timestamp (milliseconds epoch).
    pub join_start_time: i64,
    /// Raw join-window close timestamp (milliseconds epoch).
    pub join_end_time: i64,
    /// Raw claim-window open timestamp (milliseconds epoch).
    pub claim_start_time: i64,
    /// Raw claim-window close timestamp (milliseconds epoch).
    pub claim_end_time: i64,
    /// Raw activity end timestamp (milliseconds epoch). Display-only; not used
    /// for stage classification (V1 uses claim_end_time as the Ended boundary).
    pub end_time: i64,
    /// Absolute wall-clock ms of the NEXT stage boundary for this card.
    /// Android takes min(next_refresh_time) across all cards to schedule the
    /// reload. 0 means no next boundary (WaitingInfo or Ended).
    pub next_refresh_time: i64,
    /// The server timestamp (sever_timestamp) used for stage classification.
    /// Android passes this back as context for debugging and audit.
    pub server_timestamp: i64,
}

/// Error type for ExploreBoostDataState.
#[derive(Debug, Clone, PartialEq)]
pub struct ExploreBoostError {
    pub message: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExploreCardDexState {
    pub dex_list: Vec<QuotationDexItemVo>,
    pub is_loading: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExploreCardFuturesState {
    pub futures_list: Vec<QuotationFuturesItemVo>,
    pub is_loading: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExploreCardOptionsState {
    pub options_chain_list: Vec<OptionsChainItemVo>,
    pub is_loading: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExploreCardSpotState {
    pub spot_list: Vec<QuotationSpotItemVo>,
    pub is_loading: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExploreEventContractState {
    /// Monotonic id of the subscription that last wrote this state. Used to drop
    /// stale pushes from a cancelled stream (see `process_stream`).
    pub subscription_id: u32,
    pub updown_cards: Vec<EventContractUpDownCardVo>,
    pub above_groups: Vec<EventContractAboveGroupVo>,
    pub is_loading: bool,
    pub error: Option<String>,
    /// Capped preview rows (<= MAX_OVERVIEW_ITEMS): the SINGLE selection that
    /// drives BOTH this card's display and its WS subscription. Native renders
    /// these directly (per-row title/icon/vol); no client-side cap/selection.
    pub overview_rows: Vec<ExploreEventOverviewRowVo>,
}

/// One row of the Explore event-contract preview card (the capped <=6 overview).
/// 
/// This is the single source of truth for BOTH the Explore card display AND its
/// WS subscription set: Rust selects the preview rows once (see
/// `event_contract_format_utils::compute_overview_selection`) and the Explore
/// interactor (a) emits these rows for native rendering and (b) subscribes only
/// the inst_ids/ccys backing them — so display == subscription and no
/// off-preview series (e.g. HIT/BETWEEN not shown) get subscribed.
/// 
/// Presentation (title / icon / volume text / string resources) stays native:
/// titles come from `inst_id` (UP_DOWN, via the V1 short-name helper) or from
/// `base_currency` + `date_label_ts` (ABOVE-like), so this VO carries only the
/// raw fields native needs, not a pre-rendered label.
#[derive(Debug, Clone, PartialEq)]
pub struct ExploreEventOverviewRowVo {
    pub base_currency: String,
    pub method: EventContractMethod,
    /// Representative inst_id: the UP_DOWN card's inst_id, or the ABOVE-like
    /// group's smallest-strike item — the same inst_id used to subscribe the WS.
    pub inst_id: String,
    pub freq: String,
    /// UTC day-start ms for ABOVE-like rows (used for the "on <date>" label);
    /// `0` for UP_DOWN rows.
    pub date_label_ts: i64,
    /// Aggregated volume (UP_DOWN card) or group total volume (ABOVE-like), raw
    /// string; native formats it via `formatEventVolume`.
    pub vol: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExploreRootState {
    pub tabs: Vec<ExploreL1Page>,
    pub is_loading: bool,
}

/// A cumulative PnL data point in the calendar chart.
#[derive(Debug, Clone, PartialEq)]
pub struct FaCalendarCumulativePnlPointPo {
    /// Cumulative PnL at this point
    pub cumulative_pnl: String,
    /// Date timestamp in milliseconds
    pub date_time: i64,
    /// PnL for this period
    pub pnl: String,
    /// Time step identifier
    pub time_step: i32,
}

/// A single day entry in the futures analysis calendar.
#[derive(Debug, Clone, PartialEq)]
pub struct FaCalendarDayPo {
    /// Number of closed positions this day
    pub closed_positions: String,
    /// Cumulative PnL for this day
    pub cumulative_pnl: String,
    /// Date timestamp in milliseconds
    pub date_time: i64,
    /// Win rate for this day (decimal string, e.g. "0.50" = 50%)
    pub win_rate: String,
    /// Number of winning trades
    pub win_count: i32,
    /// Number of losing trades
    pub lose_count: i32,
    /// PnL on winning trades (API returns number)
    pub pnl_on_win: f64,
    /// PnL on losing trades (API returns number)
    pub pnl_on_loss: f64,
}

/// Calendar data for futures analysis PnL calendar view.
/// 
/// GET /priapi/v5/journal/app-calendar-highlight
#[derive(Debug, Clone, PartialEq)]
pub struct FaCalendarPo {
    /// Cumulative PnL for the period
    pub cumulative_pnl: String,
    /// Total loss PnL
    pub loss_pnl: String,
    /// Number of losing days
    pub loss_count: String,
    /// Total win PnL
    pub win_pnl: String,
    /// Number of winning days
    pub win_count: String,
    /// Daily calendar entries
    pub calendar_one_day_dtos: Vec<FaCalendarDayPo>,
    /// Daily cumulative PnL chart data
    pub every_day_cumulative_pnl_list: Vec<FaCalendarCumulativePnlPointPo>,
}

/// State for the futures analysis calendar view.
#[derive(Debug, Clone, PartialEq)]
pub struct FaCalendarState {
    /// Calendar PnL data
    pub data: Option<FaCalendarPo>,
    /// Whether a network request is in progress
    pub is_loading: bool,
    /// Error message if the last request failed
    pub error: Option<String>,
}

/// A single data point in the cumulative PnL chart.
#[derive(Debug, Clone, PartialEq)]
pub struct FaCumulativePnlPointPo {
    /// Cumulative PnL value at this point
    pub cumulative_pnl: String,
    /// Timestamp in milliseconds
    pub date_time: i64,
    /// PnL for this period
    pub pnl: String,
    /// Time step identifier
    pub time_step: i32,
}

/// Estimated funding fee for a swap instrument.
/// 
/// GET /priapi/v5/journal/estimated-funding-fee
#[derive(Debug, Clone, PartialEq)]
pub struct FaEstimatedFundingFeePo {
    /// Instrument ID
    pub inst_id: String,
    /// Instrument type (SWAP / FUTURES). Used for X-Perp title formatting.
    pub inst_type: Option<String>,
    /// Funding fee rate
    pub funding_fee_rate: String,
    /// Estimated funding fee
    pub estimated_funding_fee: String,
    /// Estimated fiat currency funding fee
    pub estimated_fiat_ccy_funding_fee: String,
    /// Currency
    pub ccy: String,
    /// Coin icon URL. Resolved via BizManagerCache using inst_id with resolved inst_type.
    /// Not present in the API response.
    pub icon_url: String,
}

/// Funding fee chart data for futures analysis.
/// 
/// GET /priapi/v5/journal/funding-fee
#[derive(Debug, Clone, PartialEq)]
pub struct FaFundingFeeChartPo {
    /// Total funding fee
    pub funding_fee: String,
    /// List of instruments with positive funding fees
    pub win_funding_fee_list: Vec<FaFundingFeeItemPo>,
    /// List of instruments with negative funding fees
    pub loss_funding_fee_list: Vec<FaFundingFeeItemPo>,
    /// Funding fee flow over time
    pub every_time_step_funding_fee_list: Vec<FaFundingFlowPointPo>,
    /// Total positive funding fee
    pub win_funding_fee: String,
    /// Total negative funding fee
    pub loss_funding_fee: String,
}

/// Funding fee breakdown per instrument.
#[derive(Debug, Clone, PartialEq)]
pub struct FaFundingFeeItemPo {
    /// Funding fee amount
    pub funding_fee: String,
    /// Instrument ID
    pub inst_id: String,
    /// Instrument type (SWAP / FUTURES). Used for X-Perp title formatting.
    pub inst_type: Option<String>,
}

/// State for the futures analysis funding fee page.
#[derive(Debug, Clone, PartialEq)]
pub struct FaFundingFeeState {
    /// Funding fee chart data
    pub chart_data: Option<FaFundingFeeChartPo>,
    /// Estimated funding fee list
    pub estimated_data: Vec<FaEstimatedFundingFeePo>,
    /// Available swap instrument IDs for coin pair selection
    pub swap_inst_ids: Vec<String>,
    /// Whether the coin pair selector request is in progress
    pub swap_inst_ids_loading: bool,
    /// Error message if the last coin pair selector request failed
    pub swap_inst_ids_error: Option<String>,
    /// Whether a network request is in progress
    pub is_loading: bool,
    /// Error message if the last request failed
    pub error: Option<String>,
}

/// A single data point in the funding fee flow chart.
#[derive(Debug, Clone, PartialEq)]
pub struct FaFundingFlowPointPo {
    /// Timestamp in milliseconds (string from API, e.g. "1771516800000")
    pub date_time: String,
    /// Funding fee value at this point
    pub funding_fee: String,
    /// Time step identifier (string from API, e.g. "1" = month, "2" = day)
    pub time_step: String,
}

/// Overview performance data for futures analysis.
/// 
/// GET /priapi/v5/journal/app-highlight
#[derive(Debug, Clone, PartialEq)]
pub struct FaOverviewPo {
    /// Average hold time
    pub avg_hold_time: String,
    /// Average PnL per trade
    pub avg_pnl: String,
    /// Average PnL on losing trades
    pub avg_pnl_on_losers: String,
    /// Average PnL on winning trades
    pub avg_pnl_on_winners: String,
    /// Cumulative PnL
    pub cumulative_pnl: String,
    /// Current day PnL
    pub current_day_pnl: String,
    /// Average hold time for losing trades
    pub lose_avg_hold_time: String,
    /// Average hold time for winning trades
    pub win_avg_hold_time: String,
    /// Total loss PnL
    pub loss_pnl: String,
    /// Number of losing trades
    pub loss_count: String,
    /// Profit factor
    pub profit_factor: String,
    /// Risk/reward ratio
    pub risk_reward_ratio: String,
    /// Total win PnL
    pub win_pnl: String,
    /// Win rate
    pub win_rate: String,
    /// Number of winning trades
    pub win_count: String,
    /// Total number of trades
    pub total_count: String,
    /// Trading fee
    pub trading_fee: String,
    /// Cumulative PnL chart data points
    pub cumulative_pnl_list: Vec<FaCumulativePnlPointPo>,
    /// Win/loss PnL ratio as a percentage (0–100): |winPnl| / (|winPnl| + |lossPnl|) * 100.
    /// Defaults to 50 when both are zero. Computed by the interactor; not in the API response.
    pub pnl_percentage: i32,
}

/// State for the futures analysis overview.
#[derive(Debug, Clone, PartialEq)]
pub struct FaOverviewState {
    /// Overview performance data
    pub data: Option<FaOverviewPo>,
    /// Whether a network request is in progress
    pub is_loading: bool,
    /// Error message if the last request failed
    pub error: Option<String>,
}

/// PnL breakdown by instrument for futures analysis.
/// 
/// GET /priapi/v5/journal/performance/app-ticker
#[derive(Debug, Clone, PartialEq)]
pub struct FaTickerItemPo {
    /// Business type identifier
    pub biz_type: i32,
    /// Instrument type (SWAP / FUTURES). Used for X-Perp title formatting.
    /// Preferred over biz_type for display, falls back to biz_type-derived type when absent.
    pub inst_type: Option<String>,
    /// Instrument name
    pub inst_name: String,
    /// Long position PnL
    pub long_pnl: String,
    /// Net PnL
    pub net_pnl: String,
    /// Short position PnL
    pub short_pnl: String,
    /// Progress bar percentage (0–100) relative to the item with the largest absolute PnL
    /// in the current list. Computed by the interactor; not present in the API response.
    pub progress_percent: i32,
    /// Coin icon URL. Resolved via BizManagerCache using inst_name as instId and inst_type
    /// derived from biz_type. Not present in the API response.
    pub icon_url: String,
}

/// State for the futures analysis ticker list.
#[derive(Debug, Clone, PartialEq)]
pub struct FaTickerState {
    /// PnL data per instrument
    pub data: Vec<FaTickerItemPo>,
    /// Whether a network request is in progress
    pub is_loading: bool,
    /// Error message if the last request failed
    pub error: Option<String>,
}

/// Favorite coin item
#[derive(Debug, Clone, PartialEq)]
pub struct FavoriteCoinItem {
    pub inst_id: Option<String>,
    pub inst_type: Option<String>,
    pub exp_time: Option<String>,
    pub alias: Option<String>,
    pub icon: Option<String>,
    pub full_name: Option<String>,
    /// Display symbol (e.g. "xMU" where the raw `inst_id`/symbol is "XMU").
    /// 
    /// The watchlist payload omits this, so it deserializes to `None`; consumers that
    /// need it (e.g. the favorites-editing screen) enrich it from the coins map. Kept
    /// optional so a blank/absent value falls back to `inst_id` on the UI side.
    pub display_symbol: Option<String>,
}

/// Favorite DEX item
#[derive(Debug, Clone, PartialEq)]
pub struct FavoriteDEXItem {
    pub inst_type: Option<String>,
    pub chain_id: Option<String>,
    pub token_contract_address: Option<String>,
    pub token_symbol: Option<String>,
    pub boost_rate: Option<String>,
    pub token_logo_url: Option<String>,
    pub chain_logo_url: Option<String>,
    pub asset_id: Option<String>,
    pub dex_sub_type: Option<String>,
    /// Display name of a DEX Spot / Perp row, echoed back to the server as `name` on the
    /// write endpoints (the read model spells it `tokenName`).
    /// 
    /// Only meaningful when `dex_sub_type` is SPOT / PERP — a legacy Swap row is identified by
    /// chain + contract and must not send it (see `to_dict`). `None` where the source has no
    /// name (DB rows, the legacy Swap VO), in which case `to_dict` falls back to the token
    /// symbol.
    /// 
    /// Appended last with `#[uniffi(default = None)]` so existing native constructors of this
    /// Record stay source-compatible.
    pub name: Option<String>, // default = None
}

/// Favorite future item
#[derive(Debug, Clone, PartialEq)]
pub struct FavoriteFutureItem {
    pub inst_id: Option<String>,
    pub inst_type: Option<String>,
    pub exp_time: Option<String>,
    pub alias: Option<String>,
    pub icon: Option<String>,
    pub full_name: Option<String>,
    pub coin_lever: Option<String>,
    /// Resolved real instrument id for delivery futures.
    /// For delivery futures, `inst_id` stores the inst_family (e.g., "ETH-USDT"),
    /// while this field stores the actual contract id (e.g., "ETH-USDT-250328")
    /// resolved via BizManager.smart_lookup(inst_family, alias).
    /// `None` for perpetual swaps or when resolution is not yet performed.
    pub resolved_inst_id: Option<String>,
}

/// Favorite option item
#[derive(Debug, Clone, PartialEq)]
pub struct FavoriteOptionItem {
    pub inst_id: Option<String>,
    pub inst_type: Option<String>,
    pub exp_time: Option<String>,
    pub alias: Option<String>,
    pub icon: Option<String>,
    pub full_name: Option<String>,
    pub coin_lever: Option<String>,
}

/// Crypto config (UniFFI-compatible)
#[derive(Debug, Clone, PartialEq)]
pub struct FavoriteSelectionCryptoConfigVo {
    pub category_groups: Vec<CategoryGroupVo>,
}

/// Crypto filter parameters (UniFFI-compatible)
#[derive(Debug, Clone, PartialEq)]
pub struct FavoriteSelectionCryptoFilterVo {
    pub category_group_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FavoriteSelectionCryptoListState {
    /// Subscription identifier for filtering stale callbacks
    pub subscription_id: u32,
    /// Active keyword used to filter stale callbacks
    pub keyword: Option<String>,
    /// Crypto config (category groups)
    pub config: Option<FavoriteSelectionCryptoConfigVo>,
    /// Search result items
    pub results: Vec<QuotationCryptoItemVo>,
    /// Whether config is loading
    pub is_loading_config: bool,
    /// Whether search results are loading
    pub is_loading_results: bool,
    /// Error message
    pub error: Option<String>,
}

/// DEX config (UniFFI-compatible)
#[derive(Debug, Clone, PartialEq)]
pub struct FavoriteSelectionDexConfigVo {
    pub chain_groups: Vec<DexChainGroupVo>,
    pub periods: Vec<DexPeriod>,
    pub sort_options: Vec<DexSortBy>,
}

/// DEX filter parameters (UniFFI-compatible)
#[derive(Debug, Clone, PartialEq)]
pub struct FavoriteSelectionDexFilterVo {
    pub chain_id: Option<String>,
    pub period: Option<DexPeriod>,
    pub sort_by: Option<DexSortBy>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FavoriteSelectionDexListState {
    /// Subscription identifier for filtering stale callbacks
    pub subscription_id: u32,
    /// Active keyword used to filter stale callbacks
    pub keyword: Option<String>,
    /// DEX config (chain groups, periods, sort options)
    pub config: Option<FavoriteSelectionDexConfigVo>,
    /// Search result items
    pub results: Vec<QuotationDexItemVo>,
    /// Whether config is loading
    pub is_loading_config: bool,
    /// Whether search results are loading
    pub is_loading_results: bool,
    /// Error message
    pub error: Option<String>,
}

/// exos DEX config (UniFFI-compatible).
/// 
/// Mirrors the exos manager's `load_data` result: the four biz tabs, swap chain
/// sub-tabs, and (keyword mode) the total search-hit count.
#[derive(Debug, Clone, PartialEq)]
pub struct FavoriteSelectionExosDexConfigVo {
    pub biz_tabs: Vec<ExchangeOsBizTab>,
    pub swap_chain_groups: Vec<DexChainGroupVo>,
    pub total_count: u32,
}

/// exos DEX filter parameters (UniFFI-compatible).
/// 
/// The biz tab, sort field, and sort order are passed as separate `subscribe`
/// arguments (mirroring the NavSearch exos market-list interactor); this VO
/// carries the remaining per-tab dimensions.
#[derive(Debug, Clone, PartialEq)]
pub struct FavoriteSelectionExosDexFilterVo {
    /// Swap chain sub-tab id (swap only; perp/spot ignore it).
    pub chain_id: Option<String>,
    /// Internal filter key. No-op for perp/spot/swap.
    pub filter: Option<String>,
    /// Market-list period (perp/spot/swap). `None` defaults to 1 day.
    pub period: Option<DexPeriod>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FavoriteSelectionExosDexListState {
    /// Subscription identifier for filtering stale callbacks
    pub subscription_id: u32,
    /// Active keyword used to filter stale callbacks
    pub keyword: Option<String>,
    /// exos DEX config (biz tabs, swap chain sub-tabs, filter options)
    pub config: Option<FavoriteSelectionExosDexConfigVo>,
    /// Market rows (Perp / Spot / Swap)
    pub market_results: Vec<ExchangeOsMarketItemVo>,
    /// Whether config is loading
    pub is_loading_config: bool,
    /// Whether search results are loading
    pub is_loading_results: bool,
    /// Error message
    pub error: Option<String>,
}

/// Futures config (UniFFI-compatible)
/// 
/// Same page-list contract as [FavoriteSelectionSpotConfigVo]; unit chooser
/// data rides on `FuturesPageVo::All`.
#[derive(Debug, Clone, PartialEq)]
pub struct FavoriteSelectionFuturesConfigVo {
    pub page_list: Vec<FuturesPageVo>,
}

/// Futures filter parameters (UniFFI-compatible)
#[derive(Debug, Clone, PartialEq)]
pub struct FavoriteSelectionFuturesFilterVo {
    pub category_group_id: Option<String>,
    pub futures_type: Option<FuturesType>,
    pub unit: Option<String>,
    /// When `Some(true)`, the futures list is sourced from the hot swap ranking
    /// data source instead of the regular search/category list. In hot mode the
    /// other dimensions (category_group_id / futures_type / unit) are ignored —
    /// hot is a single ranked list of perpetual swaps. Defaults to regular
    /// (non-hot) behavior. Added as a trailing Option field to preserve UniFFI
    /// Record ABI.
    pub is_hot: Option<bool>,
    /// Ranking period for ranking tabs (Hot / Gainers / Losers). Drives which
    /// period's change% the list is ranked by. `None` keeps the previous default
    /// (Period1D for hot, unranked otherwise). Trailing Option preserves the
    /// UniFFI Record ABI.
    pub tickers_period: Option<TickersPeriodEnum>,
    /// When `Some(true)`, source the futures list as the Gainers ranking
    /// (`CategoryGroups::gainers()`) with the default unit and period — same
    /// mechanism as `is_hot`. Trailing Option preserves the UniFFI Record ABI.
    pub is_gainers: Option<bool>,
    /// When `Some(true)`, source the futures list as the Losers ranking
    /// (`CategoryGroups::losers()`). See `is_gainers`.
    pub is_losers: Option<bool>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FavoriteSelectionFuturesListState {
    /// Subscription identifier for filtering stale callbacks
    pub subscription_id: u32,
    /// Active keyword used to filter stale callbacks
    pub keyword: Option<String>,
    /// Futures config (cloud-aware tab page list)
    pub config: Option<FavoriteSelectionFuturesConfigVo>,
    /// Search result items
    pub results: Vec<QuotationFuturesItemVo>,
    /// Whether config is loading
    pub is_loading_config: bool,
    /// Whether search results are loading
    pub is_loading_results: bool,
    /// Error message
    pub error: Option<String>,
}

/// Search hit statistics (UniFFI-compatible)
#[derive(Debug, Clone, PartialEq)]
pub struct FavoriteSelectionHitStatisticsVo {
    pub crypto_count: u32,
    pub spot_count: u32,
    pub futures_count: u32,
    pub dex_count: u32,
    pub options_count: u32,
}

/// Options config (UniFFI-compatible)
#[derive(Debug, Clone, PartialEq)]
pub struct FavoriteSelectionOptionsConfigVo {
    pub category_groups: Vec<OptionsCategoryGroupVo>,
    pub directions: Vec<OptionsDirection>,
    /// R9a: per-`inst_family` expiry-date dropdown options.
    /// 
    /// Key: `OptionsCategoryGroupVo.inst_family` (literal, includes `_UM` suffix for
    /// USDⓈ-margined groups). Value: chronologically sorted millisecond timestamp
    /// strings.
    pub exp_time_periods: HashMap<String, Vec<String>>,
}

/// Options filter parameters (UniFFI-compatible)
#[derive(Debug, Clone, PartialEq)]
pub struct FavoriteSelectionOptionsFilterVo {
    pub category_group_id: Option<String>,
    pub direction: Option<OptionsDirection>,
    pub exp_time: Option<String>,
    pub strike_price: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FavoriteSelectionOptionsListState {
    /// Subscription identifier for filtering stale callbacks
    pub subscription_id: u32,
    /// Active keyword used to fetch this config/results
    pub keyword: Option<String>,
    /// Options config (category groups, directions)
    pub config: Option<FavoriteSelectionOptionsConfigVo>,
    /// Search result items
    pub results: Vec<QuotationOptionsItemVo>,
    /// Whether config is loading
    pub is_loading_config: bool,
    /// Whether search results are loading
    pub is_loading_results: bool,
    /// Error message
    pub error: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FavoriteSelectionRootState {
    /// Search hit statistics per business line
    pub hit_statistics: Option<FavoriteSelectionHitStatisticsVo>,
    /// Currently selected IDs per business line
    pub selected_ids: FavoriteSelectionSelectedIdsVo,
    /// Total number of selected items across all business lines
    pub total_selected_count: u32,
    /// Result of the last toggle operation (true = selected, false = deselected)
    pub last_toggle_result: Option<bool>,
    /// Number of items added in the last confirm operation
    pub last_confirm_count: Option<u32>,
    /// Whether a statistics search is in progress
    pub is_loading_statistics: bool,
    /// Whether a confirm operation is in progress
    pub is_confirming: bool,
    /// Error message from the last operation
    pub error: Option<String>,
    /// Whether the interactor has been initialized with group info
    pub is_initialized: bool,
}

/// Selected IDs per business line (UniFFI-compatible)
#[derive(Debug, Clone, PartialEq)]
pub struct FavoriteSelectionSelectedIdsVo {
    pub crypto_ids: Vec<String>,
    pub spot_ids: Vec<String>,
    pub futures_ids: Vec<String>,
    pub dex_ids: Vec<String>,
    pub options_ids: Vec<String>,
    /// exos DEX selection ids (encoded `PERP:|SPOT:|SWAP:` item ids). Trailing
    /// field preserves the UniFFI Record ABI.
    pub exos_dex_ids: Vec<String>,
}

/// Spot config (UniFFI-compatible)
/// 
/// The secondary tab row is delivered as a ready-built page list (cloud
/// nav_spot scene aware, Rust SSOT) — the native side only renders it.
/// Chooser data (quote-currency options) rides on `SpotPageVo::All`.
#[derive(Debug, Clone, PartialEq)]
pub struct FavoriteSelectionSpotConfigVo {
    pub page_list: Vec<SpotPageVo>,
}

/// Spot filter parameters (UniFFI-compatible)
#[derive(Debug, Clone, PartialEq)]
pub struct FavoriteSelectionSpotFilterVo {
    pub category_group_id: Option<String>,
    pub charge_group_id: Option<String>,
    /// When `Some(true)`, the spot list is sourced from the hot ranking data
    /// source instead of the regular search/category list. In hot mode the
    /// other dimensions (category_group_id / charge_group_id) are ignored —
    /// hot is a single ranked list. Defaults to regular (non-hot) behavior.
    /// Added as a trailing Option field to preserve UniFFI Record ABI.
    pub is_hot: Option<bool>,
    /// Ranking period for ranking tabs (Hot / Gainers / Losers). Drives which
    /// period's change% the list is ranked by. `None` keeps the previous default
    /// (Period1D for hot, unranked otherwise). Trailing Option preserves the
    /// UniFFI Record ABI.
    pub tickers_period: Option<TickersPeriodEnum>,
    /// When `Some(true)`, source the spot list as the Gainers ranking
    /// (`CategoryGroups::gainers()`) with the default charge group and period —
    /// same mechanism as `is_hot`. Trailing Option preserves the UniFFI Record ABI.
    pub is_gainers: Option<bool>,
    /// When `Some(true)`, source the spot list as the Losers ranking
    /// (`CategoryGroups::losers()`). See `is_gainers`.
    pub is_losers: Option<bool>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FavoriteSelectionSpotListState {
    /// Subscription identifier for filtering stale callbacks
    pub subscription_id: u32,
    /// Active keyword used to filter stale callbacks
    pub keyword: Option<String>,
    /// Spot config (cloud-aware tab page list)
    pub config: Option<FavoriteSelectionSpotConfigVo>,
    /// Search result items
    pub results: Vec<QuotationSpotItemVo>,
    /// Whether config is loading
    pub is_loading_config: bool,
    /// Whether search results are loading
    pub is_loading_results: bool,
    /// Error message
    pub error: Option<String>,
}

/// Favorite spot item
#[derive(Debug, Clone, PartialEq)]
pub struct FavoriteSpotItem {
    pub inst_id: Option<String>,
    pub inst_type: Option<String>,
    pub exp_time: Option<String>,
    pub alias: Option<String>,
    pub icon: Option<String>,
    pub full_name: Option<String>,
    pub coin_lever: Option<String>,
    pub display_quote_symbol: Option<String>,
    /// Pre-computed leverage display tag ("10x") with USDⓈ multi-stablecoin +
    /// single-currency-account suppression and margin feature-restriction
    /// already applied (via `core_trading::calc_spot_lever_tag`, wrapped by
    /// `lever_tag_for_vo`). `None` / empty means "do not display". Only
    /// populated where a `SpotInstrument` is available for the join (e.g. the
    /// favorites edit interactor); raw HTTP/DB construction leaves it `None`.
    /// Display-only: intentionally excluded from `to_dict` / persistence and
    /// from `get_unique_id`, so it never affects identity or serialization.
    pub lever_display: Option<String>,
    /// Pre-computed display base symbol (e.g. "BTC", or a lowercase/custom
    /// displayId base) resolved from the instrument via
    /// `core_trading` `BizInstrument::get_display_trade_symbol` (displayBaseCcy,
    /// falls back to tradeSymbol). `None` / empty means "not resolved". Only
    /// populated where a `SpotInstrument` is available for the join (e.g. the
    /// favorites edit interactor); raw HTTP/DB construction leaves it `None`.
    /// Display-only: intentionally excluded from `to_dict` / persistence and
    /// from `get_unique_id`, so it never affects identity or serialization.
    pub display_trade_symbol: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FavoritesAddToGroupState {
    /// Filtered groups ready for display (non-preset, non-hidden, DEX filtered)
    pub available_groups: Vec<RetailTradingGroupFavorite>,
    /// All group names (including hidden) for create-group duplicate validation
    pub all_group_names: Vec<String>,
    /// Whether groups are being loaded
    pub is_loading_groups: bool,
    /// Error from loading groups
    pub groups_error: Option<String>,
    /// Group names currently selected by the user
    pub selected_group_names: Vec<String>,
    /// Whether an add operation is in progress
    pub is_adding: bool,
    /// Error from the add operation
    pub add_error: Option<String>,
}

/// State for FavoritesCryptosEditInteractor
#[derive(Debug, Clone, PartialEq)]
pub struct FavoritesCryptosEditState {
    /// All available groups
    pub groups: Vec<RetailTradingGroupFavorite>,
    /// Whether groups are being loaded
    pub is_loading_groups: bool,
    /// Error from loading groups, if any
    pub groups_error: Option<String>,
    /// Name of the currently displayed group
    pub group_name: String,
    /// Items in the currently displayed group
    pub group_items: Vec<RetailTradingFavoriteItem>,
    /// Whether group items are being loaded
    pub is_loading_items: bool,
    /// Error from loading items, if any
    pub items_error: Option<String>,
    /// Whether a remove operation is in progress
    pub is_removing: bool,
    /// Whether a rearrange operation is in progress
    pub is_rearranging: bool,
}

/// State for FavoritesGroupsEditInteractor
#[derive(Debug, Clone, PartialEq)]
pub struct FavoritesGroupsEditState {
    /// Visible (non-hidden) groups, displayed in the main list
    pub showing_groups: Vec<RetailTradingGroupFavorite>,
    /// Hidden groups, displayed in the hidden section
    pub hidden_groups: Vec<RetailTradingGroupFavorite>,
    /// Whether groups are being loaded
    pub is_loading_groups: bool,
    /// Error from loading groups, if any
    pub groups_error: Option<String>,
    /// Whether a delete operation is in progress
    pub is_deleting: bool,
    /// Whether a rearrange operation is in progress
    pub is_rearranging: bool,
    /// Whether a visibility update operation is in progress
    pub is_updating_visibility: bool,
}

/// Per-inst-type item count for filter tab display
#[derive(Debug, Clone, PartialEq)]
pub struct FavoritesInstTypeCount {
    pub inst_type: WatchlistInstType,
    pub count: u32,
}

/// Dynamic Finance Category payload shared by Explore, Discover and Nav
/// Search. L2Scene-specific page enums wrap this record later; this type only
/// carries stable Category identity and display data.
#[derive(Debug, Clone, PartialEq)]
pub struct FinanceCategoryTabVo {
    pub key: String,
    pub title: String,
    pub category_group: CategoryGroupVo,
}

/// A single financial calendar event from the API.
/// 
/// May represent an economic, token unlock, or earnings event.
/// The `event_type` field (renamed from `type` in JSON) identifies which variant is populated.
#[derive(Debug, Clone, PartialEq)]
pub struct FinancialEventPo {
    /// Raw event type string from the API: "economic" | "tokenUnlock" | "earnings"
    pub event_type: String,
    /// Economic event data; present when `event_type` is "economic"
    pub economic_data: Option<EcoDataPo>,
    /// Token unlock data; present when `event_type` is "tokenUnlock"
    pub token_unlock_data: Option<TokenUnlockDataPo>,
    /// Earnings data; present when the source item has an Earnings payload
    pub earnings_data: Option<EarningsDataPo>, // default = None
}

/// Formula data for output
#[derive(Debug, Clone, PartialEq)]
pub struct FormulaData {
    /// Display formula string (e.g., "{BTC-USDT&SPOT}+{ETH-USDT&SWAP}")
    pub display_formula: String,
    /// Calculation expression (e.g., "V0+V1*2")
    pub calc_expression: String,
    /// Instrument ID array as JSON string
    /// Format: `[["TRADE","BTC-USDT"],["TRADE","ETH-USDT"],["CONST","2"]]`
    pub inst_id_array: String,
    /// Number of variables (instruments) in the formula
    pub variable_count: u32,
    /// Number of constants in the formula
    pub constant_count: u32,
    /// Total element count (variables + constants)
    pub total_element_count: u32,
}

/// State for FormulaNavInteractor
#[derive(Debug, Clone, PartialEq)]
pub struct FormulaNavState {
    pub formula_data: FormulaData,
    pub can_save: bool,
    pub remaining_capacity: u32,
    pub is_empty: bool,
    pub validation_error: Option<FormulaValidationError>,
    pub last_add_result: Option<AddInstrumentResultVo>,
    pub variable_count: u32,
    pub constant_count: u32,
    pub total_element_count: u32,
    pub error: Option<String>,
}

/// Formula search manager state
#[derive(Debug, Clone, PartialEq)]
pub struct FormulaSearchState {
    /// Current formula data
    pub formula_data: FormulaData,
    /// Whether the formula can be saved (valid)
    pub can_save: bool,
    /// Number of remaining elements that can be added
    pub remaining_capacity: u32,
    /// Whether the formula is empty
    pub is_empty: bool,
    /// Validation error if any
    pub validation_error: Option<FormulaValidationError>,
}

/// Raw inputs for a Futures/Swap list item (from `QuotationFuturesItemVo`).
#[derive(Debug, Clone, PartialEq)]
pub struct FuturesItemFormatInput {
    /// Raw change ratio decimal. `None` -> empty change recipe.
    pub change_ratio: Option<String>,
    /// Instrument price. `None` -> no price recipe.
    pub price: Option<String>,
    /// Raw 24h high price, formatted with the same instrument precision. `None` -> no recipe.
    pub high_24h: Option<String>, // default = None
    /// Raw 24h low price, formatted with the same instrument precision. `None` -> no recipe.
    pub low_24h: Option<String>, // default = None
    /// Instrument family (e.g. "BTC-USDT").
    pub inst_family: String,
    /// Instrument type ("SWAP" | "FUTURES").
    pub inst_type: String,
    /// True for linear (USDT/USDC) contracts, false for inverse (coin-margined).
    pub is_positive_contract: bool,
    /// USD sub-price to convert to fiat. `None` -> no sub-price recipe.
    pub sub_price_usd: Option<String>,
    /// 24h volume (base-coin). `None` -> no volume recipe.
    pub volume: Option<String>,
    /// USD turnover to abbreviate into fiat. `None` -> no turnover recipe.
    pub turnover_usd: Option<String>,
    /// USD market cap to abbreviate into a fiat recipe; None -> no recipe.
    pub market_cap_usd: Option<String>, // default = None
}

/// Futures rank item response from API
/// 
/// Represents a single item in the futures rank list.
/// This corresponds to the Kotlin `FuturesRankItemPo` data class.
#[derive(Debug, Clone, PartialEq)]
pub struct FuturesRankItemPo {
    /// Instrument ID (e.g., "BTC-USDT-SWAP")
    pub inst_id: String,
    /// Instrument type (e.g., "SWAP", "FUTURES")
    pub inst_type: String,
    /// Icon URL
    pub icon: String,
    /// Currency code (e.g., "USDT", "BTC")
    pub ccy: String,
    /// Full name
    pub full_name: String,
    /// Last price
    pub last_price: String,
    /// Change percentage
    pub change_per: String,
    /// Volume
    pub volume: String,
    /// Turnover
    pub turn_over: String,
    /// Market capitalization
    pub market_cap: String,
    /// Alias
    pub alias: String,
    /// Expiration time
    pub exp_time: String,
    /// Online timestamp
    pub online_ts: String,
    /// Change percentage in 24 hours
    pub change_24h: String,
    /// Change percentage at UTC 0
    pub change_utc0: String,
    /// Change percentage at UTC 8
    pub change_utc8: String,
    /// Low price (24h)
    pub low: String,
    /// High price (24h)
    pub high: String,
    /// Funding rate
    pub funding_rate: String,
    /// Open interest
    pub open_interest: String,
    /// Whether the instrument has a recently published earnings event.
    pub has_recent_earnings: bool, // default = false
    /// USD price computed from last_price x cup ticker rate
    pub last_usd_price: String,
}

/// Futures trading unit configuration
/// 
/// Corresponds to:
/// `com.okinc.market.quotation.ui.model.FuturesUnitVo`
/// 
/// This is copied from `FutureGroupInfo` from trading core but made Parcelable.
/// Used for futures/swap contract unit selection and filtering.
#[derive(Debug, Clone, PartialEq)]
pub struct FuturesUnitVo {
    /// Display text (e.g., "USDT Perpetual", "USD Perpetual", "Delivery Contract")
    pub text: String,
    /// Currency type (e.g., "USDT", "USD", "USDC")
    pub ccy_type: String,
    /// Contract type (e.g., "SWAP", "FUTURES")
    pub ct_type: String,
}

/// Error type for CommodityRankDataState
#[derive(Debug, Clone, PartialEq)]
pub struct HomeCommodityError {
    pub message: String,
}

/// State for HomeCommodityInteractor
#[derive(Debug, Clone, PartialEq)]
pub struct HomeCommodityState {
    /// Commodity rank state (contains both spot and futures lists)
    pub commodity_rank_state: CommodityRankDataState,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HomeCustomRankError {
    pub message: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HomeCustomRankState {
    pub custom_rank_state: CustomHomeRankDataState,
}

/// Error type for EquityRankDataState
#[derive(Debug, Clone, PartialEq)]
pub struct HomeEquityError {
    pub message: String,
}

/// State for HomeEquityInteractor
#[derive(Debug, Clone, PartialEq)]
pub struct HomeEquityState {
    /// Equity rank state (contains both spot and futures lists)
    pub equity_rank_state: EquityRankDataState,
}

/// Error type for HomeFavoritesDataState
#[derive(Debug, Clone, PartialEq)]
pub struct HomeFavoritesError {
    pub message: String,
}

/// State for HomeFavoritesInteractor
#[derive(Debug, Clone, PartialEq)]
pub struct HomeFavoritesState {
    pub favorites_state: HomeFavoritesDataState,
    pub current_group_name: String,
    pub sort_by: SortBy,
    pub sort_order: SortOrderEnum,
    pub data_source: HomeFavoritesDataSource,
    /// Monotonically increasing counter incremented when the first new data arrives from
    /// the subscription stream after a `load_data()` call.
    /// Ticker updates do NOT increment this value.
    /// Android uses this to detect structural data reloads vs. price-only ticker updates.
    pub data_version: u32,
}

/// Error type for HomeGainerDataState
#[derive(Debug, Clone, PartialEq)]
pub struct HomeGainerError {
    pub message: String,
}

/// State for HomeGainerInteractor
#[derive(Debug, Clone, PartialEq)]
pub struct HomeGainerState {
    /// Gainer rank state (Spot version)
    pub spot_rank_state: GainerSpotRankDataState,
    /// Gainer rank state (Futures version)
    pub futures_rank_state: GainerFuturesRankDataState,
    /// Currently selected instrument type
    pub inst_type: GainerRankingInstType,
}

/// Error type for HomeHotDataState
#[derive(Debug, Clone, PartialEq)]
pub struct HomeHotError {
    pub message: String,
}

/// State for HomeHotInteractor
#[derive(Debug, Clone, PartialEq)]
pub struct HomeHotState {
    /// Hot rank state (Spot version)
    pub spot_rank_state: HotSpotRankDataState,
    /// Hot rank state (Futures version)
    pub futures_rank_state: HotFuturesRankDataState,
    /// Currently selected instrument type
    pub inst_type: HotRankingInstType,
}

/// Error type for IndexQuoteDataState
#[derive(Debug, Clone, PartialEq)]
pub struct HomeIndexError {
    pub message: String,
}

/// State for HomeIndexInteractor
#[derive(Debug, Clone, PartialEq)]
pub struct HomeIndexState {
    /// Index coin quotes state (single curated list)
    pub index_state: IndexQuoteDataState,
}

/// Error type for HomeNewCoinDataState
#[derive(Debug, Clone, PartialEq)]
pub struct HomeNewCoinError {
    pub message: String,
}

/// Home page new coin group VO
/// 
/// Similar to NewCoinGroupResponse but each list is limited to 7 items for home page display.
#[derive(Debug, Clone, PartialEq)]
pub struct HomeNewCoinGroupVo {
    /// Online new coin rank list (max 7 items)
    pub online: Vec<NewCoinRankPo>,
    /// Wait online new coin rank list (max 7 items)
    pub wait_online_new: Vec<WaitOnlineRankPo>,
    /// Wait online coin rank list (max 7 items)
    pub wait_online: Vec<WaitOnlineRankPo>,
    /// Futures online list (max 7 items)
    pub futures_online: Option<Vec<NewCoinFuturesRankPo>>,
    /// Futures wait online list (max 7 items)
    pub futures_wait_online: Option<Vec<WaitOnlineRankPo>>,
}

/// State for HomeNewCoinInteractor
#[derive(Debug, Clone, PartialEq)]
pub struct HomeNewCoinState {
    /// New coin rank state (Spot version) - for individual spot subscription
    pub spot_rank_state: NewCoinSpotRankDataState,
    /// New coin rank state (Futures version) - for individual futures subscription
    pub futures_rank_state: NewCoinFuturesRankDataState,
    /// Unified new coin rank state (includes spot + futures + wait online data)
    /// Uses HomeNewCoinGroupVo which contains: online, wait_online_new, wait_online, futures_online
    pub unified_rank_state: NewCoinRankDataState,
    /// Currently selected instrument type
    pub inst_type: NewCoinRankingInstType,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HomeRankingDexGroupState {
    pub chain_groups: Vec<DexChainGroupVo>,
    pub is_loading: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HomeRankingDexState {
    pub dex_list: Vec<QuotationDexItemVo>,
    pub is_loading: bool,
    pub error: Option<String>,
}

/// State for HomeSelectedGroupInteractor
#[derive(Debug, Clone, PartialEq)]
pub struct HomeSelectedGroupState {
    pub all_groups: Vec<RetailTradingGroupFavorite>,
    pub selected_group: Option<RetailTradingGroupFavorite>,
    pub has_favorites: bool,
    pub is_loading: bool,
    pub is_rearranging: bool,
    pub error: Option<String>,
}

/// Error type for HomeStockDataState
#[derive(Debug, Clone, PartialEq)]
pub struct HomeStockError {
    pub message: String,
}

/// State for HomeStockInteractor
#[derive(Debug, Clone, PartialEq)]
pub struct HomeStockState {
    /// Stock rank state (contains both spot and futures lists)
    pub stock_rank_state: StockRankDataState,
}

/// Hot rank item response from API
/// 
/// Represents a single item in the hot rank list.
/// This corresponds to the Kotlin `HotRankResponse` data class.
#[derive(Debug, Clone, PartialEq)]
pub struct HotSpotRankItemPo {
    /// Icon URL (version 2)
    pub icon_v2: String,
    /// Instrument ID (e.g., "BTC-USDT")
    pub inst_id: String,
    /// Instrument type (e.g., "SPOT", "SWAP")
    pub inst_type: String,
    /// Full name (version 2)
    pub full_name_v2: String,
    /// Change percentage (version 2)
    pub change_per_v2: String,
    /// Volume (version 2)
    pub volume_v2: String,
    /// Turnover (version 2)
    pub turn_over_v2: String,
    /// Hot index
    pub hot_index: String,
    /// Last price
    pub last_price: String,
    /// Currency code (version 2)
    pub ccy_v2: String,
    /// Change percentage in 24 hours
    pub change_per_day24h: String,
    /// Change percentage at UTC 0
    pub change_per_day_utc0: String,
    /// Change percentage at UTC 8
    pub change_per_day_utc8: String,
    /// Display ID - Display trading pair name (e.g., "BTC-USDT")
    pub display_id: Option<String>,
    /// Whether the instrument has a recently published earnings event.
    pub has_recent_earnings: bool, // default = false
    /// USD price computed from last_price x cup ticker rate
    pub last_usd_price: String,
    /// Call auction state: "" (normal), "CALL_AUCTION", "PRE_QUOTE"
    pub call_auction_state: String,
    /// Display base coin name derived from display_id (falls back to inst_id base).
    /// Display-only; ccy_v2/inst_id remain the raw keys for ticker/sort/dedup.
    pub display_coin_name: String,
}

/// Identity certification entry VO (v6.116.0+)
#[derive(Debug, Clone, PartialEq)]
pub struct IdentityCertVo {
    /// Display title (already translated by backend)
    pub title: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IndexComponentVo {
    pub exch: String,
    pub symbol: String,
    pub sym_px: String,
    pub wgt: String,
    pub cnv_px: String,
}

/// State for the index components page.
#[derive(Debug, Clone, PartialEq)]
pub struct IndexComponentsState {
    /// Loaded index components data, or None if not yet loaded.
    pub data: Option<IndexComponentsVo>,
    /// Whether a network request is in progress.
    pub is_loading: bool,
    /// Error message if the last request failed.
    pub error: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IndexComponentsVo {
    pub components: Vec<IndexComponentVo>,
    pub last: String,
    pub index: String,
    pub ts: String,
}

/// A single home index coin quote, exposed across UniFFI.
/// 
/// Holds only flat FFI-safe fields. Raw prices and the change ratio are kept as
/// `String` to preserve precision; the pre-parsed `change_ratio_value` lets native
/// derive color from the sign (per crate red-lines: no `is_red`/color fields).
#[derive(Debug, Clone, PartialEq)]
pub struct IndexQuoteVo {
    /// Index instId, e.g. "BTC-USD"; used as the K-line jump / tracking key.
    pub inst_id: String,
    /// Base coin symbol, e.g. "BTC"; derived from `inst_id` (split on '-', first segment).
    pub ccy: String,
    /// Coin full name; from coin-info enrichment, empty when not found.
    pub full_name: String,
    /// Coin icon URL; from coin-info enrichment, empty when not found.
    pub icon_url: String,
    /// Raw USD index price (`idxPx`), kept as String to preserve precision.
    pub index_price: String,
    /// Raw decimal change ratio from `cal_change_pct`, e.g. "0.0123".
    pub change_ratio: String,
    /// Pre-parsed change ratio value; native derives color from its sign.
    pub change_ratio_value: Option<f64>,
}

/// State for MainSearchInteractor
#[derive(Debug, Clone, PartialEq)]
pub struct MainSearchState {
    pub keyword: Option<String>,
    pub is_loading: bool,
    pub error: Option<String>,
    pub all_tab: Option<AllTabState>,
    pub spot: Vec<QuotationSpotItemVo>,
    pub futures: Vec<QuotationFuturesItemVo>,
    /// Matched options chain entries — one per `(inst_family, margin_type)`, in canonical
    /// per-coin UM-first order. Global search renders these as navigation cards
    /// ("BTC Options chain / USDⓈ-margined"); there are no per-instrument rows and no price,
    /// so consumers render the list directly.
    pub options: Vec<OptionsChainItemVo>,
    pub earn: Vec<SearchEarnItem>,
    pub trader: Vec<SearchTraderItem>,
    pub bots: Vec<SearchBotsItem>,
    pub dex: Vec<SearchDexItem>,
    /// Function search results (global-search "Functions" category). Holds the FULL
    /// BE-filtered list; device-side compliance (FR) filtering is applied Kotlin-side.
    pub functions: Vec<SearchFunctionItem>,
    pub p2p: Vec<SearchP2pItem>,
    pub campaign: Vec<SearchCampaignItem>,
    pub announcement: Vec<SearchAnnouncementItem>,
    pub content: Option<SearchContentVo>,
    /// Representative event-contract rows matched by keyword, already aggregated
    /// by [`aggregate_event_search_rows`] (one row per (ccy, freq) UP_DOWN and
    /// per (ccy, method) above-like = latest expiry; ordered UP_DOWN → ABOVE →
    /// HIT → BETWEEN). Native renders this list directly — no further dedup.
    /// The ALL tab preview (with AB/FR gating) lives in `all_tab.modules`;
    /// this list is the source of truth for the dedicated event contract tab
    /// and sub-pages (Events-only / Events + Options / Options-only).
    pub event_contract: Vec<QuotationEventContractItemVo>,
    /// DEX group search result (perp/spot/swap), sourced from
    /// the same `/search/result` response's `dex` field — populated as part
    /// of the existing `search()` Search effect, no dedicated Action/Effect.
    /// ABI only-add: appended after `event_contract`. Render gating lives on
    /// the Kotlin side (`CheckExchangeOsDexRestrictionUseCase`), same policy
    /// as the nav exchange-os group search.
    pub dex_perp: Vec<DexPerpItemVo>,
    pub dex_spot: Vec<DexSpotItemVo>,
    pub dex_swap: Vec<SearchDexItem>,
}

/// Market cap rank item (spot).
/// 
/// Represents an instrument ranked by its market capitalization.
/// Primary metric: `market_cap`.
/// 
/// API: GET /priapi/v5/rubik/app/public/market-cap-rank
#[derive(Debug, Clone, PartialEq)]
pub struct MarketCapRankItemPo {
    /// Instrument ID (e.g., "BTC-USDT")
    pub inst_id: String,
    /// Instrument type (e.g., "SPOT")
    pub inst_type: String,
    /// Market capitalization - primary metric for this ranking type
    pub market_cap: String,
    /// Latest price
    pub last_price: String,
    /// 24-hour price change percentage
    pub change_per_day24h: String,
    /// Price change percentage at UTC+0
    pub change_per_day_utc0: String,
    /// Price change percentage at UTC+8
    pub change_per_day_utc8: String,
    /// Icon URL (version 2)
    pub icon_v2: String,
    /// Full instrument name (version 2)
    pub full_name_v2: String,
    /// Currency code (version 2, e.g., "BTC")
    pub ccy_v2: String,
    /// Display ID - display trading pair name (e.g., "xSNDK-USDT")
    pub display_id: Option<String>,
    /// Whether the instrument has a recently published earnings event.
    pub has_recent_earnings: bool, // default = false
    /// USD price computed from last_price x cup ticker rate
    pub last_usd_price: String,
    /// Display base coin name derived from display_id (falls back to inst_id base).
    /// Display-only; ccy_v2/inst_id remain raw keys for ticker/sort/dedup.
    pub display_coin_name: String,
}

/// Market-capitalisation dominance for a single coin.
#[derive(Debug, Clone, PartialEq)]
pub struct MarketCapShareRatePo {
    /// Coin code, e.g. `"BTC"`, `"ETH"`
    pub coin: String,
    /// Icon URL for the coin
    pub coin_icon_url: String,
    /// Dominance ratio, e.g. `"45.2"` means 45.2 %
    pub ratio: String,
    /// Change in dominance, e.g. `"+0.5"` means +0.5 percentage points
    pub ratio_change: String,
}

/// A single data point in a market-overview chart.
#[derive(Debug, Clone, PartialEq)]
pub struct MarketChartPointPo {
    /// Unix timestamp in **milliseconds**, e.g. `"1710374400000"`
    pub ts: String,
    /// Raw numeric value for this data point
    pub value: String,
}

/// State for the historical chart below the overview card.
#[derive(Debug, Clone, PartialEq)]
pub struct MarketChartState {
    /// Chart data returned by the server.
    /// `None` before the first successful load.
    pub data: Option<MarketOverviewChartPo>,
    /// The metric currently displayed in the chart
    pub chart_type: MarketChartType,
    /// The time window currently displayed in the chart
    pub period: MarketChartPeriod,
    /// The coin used for dominance charts (only relevant when `chart_type == Ratio`)
    pub coin: String,
    /// Whether a network request is in progress
    pub is_loading: bool,
    /// Error message from the last failed request, if any
    pub error: Option<String>,
}

/// Full breakdown shown when the market-flow card is expanded.
#[derive(Debug, Clone, PartialEq)]
pub struct MarketFlowExpandedPo {
    /// Total number of rising coins
    pub up_num: String,
    /// Total number of falling coins
    pub down_num: String,
    /// Maximum absolute change-percent represented in the chart, e.g. `"20"` for 20 %
    pub max_range_num: String,
    /// Histogram buckets for rising coins, ordered from smallest to largest range
    pub up_list: Vec<MarketFlowRangeItemPo>,
    /// Histogram buckets for falling coins, ordered from smallest to largest range
    pub down_list: Vec<MarketFlowRangeItemPo>,
}

/// A single cell in the minimized market-flow strip.
#[derive(Debug, Clone, PartialEq)]
pub struct MarketFlowMinimizedCellPo {
    /// Average price change percent for this bucket, e.g. `"0.05"`
    pub average_price_change_percent: String,
}

/// Summary data shown when the market-flow card is collapsed.
#[derive(Debug, Clone, PartialEq)]
pub struct MarketFlowMinimizedPo {
    /// Number of coins with a positive 24 h change, e.g. `"1200"`
    pub up_num: String,
    /// Number of coins with a negative 24 h change, e.g. `"800"`
    pub down_num: String,
    /// Cell data for the minimized strip view (one cell per range bucket)
    pub cells: Vec<MarketFlowMinimizedCellPo>,
}

/// Top-level market flow response.
/// 
/// Contains both a compact summary (`minimized`) for the collapsed card view
/// and a full breakdown (`expanded`) for the expanded bar-chart view.
#[derive(Debug, Clone, PartialEq)]
pub struct MarketFlowPo {
    /// Data for the collapsed (minimised) view
    pub minimized: MarketFlowMinimizedPo,
    /// Data for the expanded bar-chart view
    pub expanded: MarketFlowExpandedPo,
}

/// A single bucket in the market-flow histogram.
#[derive(Debug, Clone, PartialEq)]
pub struct MarketFlowRangeItemPo {
    /// Human-readable range label, e.g. `"0-5%"`, `"5-10%"`
    pub range: String,
    /// Number of coins whose change falls within this range, e.g. `"200"`
    pub num: String,
}

/// State for the market flow card.
#[derive(Debug, Clone, PartialEq)]
pub struct MarketFlowState {
    /// Market flow data.
    /// `None` before the first successful load.
    pub data: Option<MarketFlowPo>,
    /// Whether a network request is in progress
    pub is_loading: bool,
    /// Error message from the last failed request, if any
    pub error: Option<String>,
}

/// Formatting recipe for native to execute with OKLocalization.
/// 
/// Rust computes the recipe (precision, sign, rounding, flags).
/// Native executes with locale-aware formatting (thousands separator, decimal point).
/// 
/// # Native Usage (Swift)
/// ```swift
/// let recipe = formatter.formatPrice(value: lastPrice, instFamily: "BTC-USDT", instType: "SPOT")
/// let number = NSDecimalNumber(string: recipe.value)
/// 
/// // Apply rounding + precision
/// var text = number.toLocalizedStringDownToFixed(Int(recipe.precision))
/// 
/// // Apply sign
/// if recipe.signPosition == "prefix" { text = recipe.sign + text }
/// if recipe.signPosition == "suffix" { text = text + " " + recipe.sign }
/// 
/// // Apply percentage
/// if recipe.isPercentage { text = text + "%" }
/// 
/// // Apply plus sign
/// if recipe.prependPlus && number.doubleValue > 0 { text = "+" + text }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct MarketFormatRecipe {
    /// Raw numeric value as plain string (always uses "." as decimal, no grouping).
    /// e.g., "50234.56", "3.45", "1234567890"
    /// Empty string means no data (native should show placeholder).
    pub value: String,
    /// Number of decimal places for display.
    pub precision: u8,
    /// Display sign or symbol (e.g., "$", "¥", "BTC", "USDT").
    pub sign: String,
    /// Where to place the sign: "prefix" | "suffix" | "none"
    pub sign_position: String,
    /// Rounding mode: "down" | "up" | "half_up" | "half_down"
    pub rounding: String,
    /// Whether to keep trailing zeros (e.g., "3.40" vs "3.4")
    pub keep_trailing_zeros: bool,
    /// Whether to prepend "+" for positive values
    pub prepend_plus: bool,
    /// Whether native should abbreviate large numbers (K/M/B/T)
    pub abbreviate: bool,
    /// Whether to treat as percentage (value already multiplied by 100, append "%")
    pub is_percentage: bool,
    /// Dynamic sign display policy: "auto" | "except_zero" | "always".
    /// 
    /// - `None` (legacy): derive from `prepend_plus` ("auto" when false,
    /// "except_zero" when true).
    /// - "auto": negative sign only (Kotlin `DisplaySign.AUTO`).
    /// - "except_zero": show "+"/"-" for any non-zero value.
    /// - "always": show sign even for zero.
    /// 
    /// `#[uniffi(default = None)]` so existing native constructors of this
    /// Record stay source-compatible.
    pub display_sign: Option<String>, // default = None
    /// Fiat currency ISO code (e.g. "TRY", "EUR") when `sign` is a fiat symbol.
    /// 
    /// When set, native SHOULD format via its locale-aware fiat primitive
    /// (Android `formatFiatSymbol(code, …)`) which places the currency symbol
    /// per locale (prefix in en-US, suffix in some EU locales), instead of the
    /// static `sign` + `sign_position` fallback. Rust cannot decide the
    /// position itself (no ICU/CLDR data), so the decision is delegated with
    /// real data rather than a guessed position.
    /// 
    /// `#[uniffi(default = None)]` so existing native constructors of this
    /// Record stay source-compatible.
    pub fiat_iso_code: Option<String>, // default = None
}

/// Lightweight snapshot of one instrument for FFI consumers (domain data,
/// not a render VO).
#[derive(Debug, Clone, PartialEq)]
pub struct MarketInstrumentBrief {
    /// Instrument ID (e.g., "BTC-USDT", "BTC-USDT-SWAP")
    pub inst_id: String,
    /// Instrument type (e.g., "SPOT", "SWAP", "FUTURES", "OPTION")
    pub inst_type: String,
    /// Raw trade (base) symbol
    pub trade_symbol: String,
    /// Raw quote symbol
    pub quote_symbol: String,
    /// Display trade (base) symbol (spot rename; falls back to the raw symbol)
    pub display_trade_symbol: String,
    /// Display quote symbol (spot display quote; falls back to the raw symbol)
    pub display_quote_symbol: String,
    /// Contract currency type (derivatives; empty for spot)
    pub ccy_type: String,
    /// Instrument family (derivatives; empty for spot)
    pub inst_family: String,
    /// Futures delivery alias (e.g., "this_week"); empty for non-futures
    pub alias: String,
    /// Whether this is a pre-market pair
    pub is_pre_market: bool,
    /// Whether this is an X-Perp pair (delivery-typed by underlying,
    /// user-perceived perpetual)
    pub is_x_perps: bool,
    /// Whether this is a positive (linear) contract
    pub is_positive_contract: bool,
    /// Whether this is a USD-margined (USD-UM) contract
    pub is_usd_um_contract: bool,
    /// Full coin name (e.g., "Bitcoin")
    pub full_name: String,
}

/// Recipes for a Spot or Futures list item (identical field shape).
#[derive(Debug, Clone, PartialEq)]
pub struct MarketListItemRecipes {
    /// Change ratio recipe — always present (may be an empty recipe).
    pub change: MarketFormatRecipe,
    /// Price recipe, or `None` when no price was supplied.
    pub price: Option<MarketFormatRecipe>,
    /// Signless 24h high price recipe, or `None` when no high was supplied.
    pub high_24h: Option<MarketFormatRecipe>, // default = None
    /// Signless 24h low price recipe, or `None` when no low was supplied.
    pub low_24h: Option<MarketFormatRecipe>, // default = None
    /// Fiat sub-price recipe, or `None`.
    pub sub_price: Option<MarketFormatRecipe>,
    /// 24h volume recipe, or `None`.
    pub volume: Option<MarketFormatRecipe>,
    /// Fiat turnover recipe, or `None`.
    pub turnover: Option<MarketFormatRecipe>,
    /// Fiat market-cap recipe, or `None` when no market cap was supplied.
    /// Defaulted so existing hand-written constructor call sites on both
    /// native platforms keep compiling without changes.
    pub market_cap: Option<MarketFormatRecipe>, // default = None
}

/// State for the market trends overview card (market cap / volume / dominance).
#[derive(Debug, Clone, PartialEq)]
pub struct MarketOverviewCardState {
    /// Market overview data (market cap, volume, dominance).
    /// `None` before the first successful load.
    pub data: Option<MarketOverviewPo>,
    /// Whether a network request is in progress
    pub is_loading: bool,
    /// Error message from the last failed request, if any
    pub error: Option<String>,
}

/// Historical chart data for market cap, volume, or dominance.
/// 
/// Returned by `/priapi/v5/rubik/public/overall-market-trend`.
/// The `trends` array is ordered **ascending** by timestamp (oldest first),
/// exactly as the server returns it; reordering for display is the caller's
/// responsibility.
#[derive(Debug, Clone, PartialEq)]
pub struct MarketOverviewChartPo {
    /// Data points in ascending time order
    pub trends: Vec<MarketChartPointPo>,
    /// Percentage change over the selected period, e.g. `"0.82"` means +0.82 %
    pub change_ratio: String,
}

/// Market statistics returned by the overview API.
/// 
/// Contains the aggregate market-cap, 24 h volume, and per-coin dominance ratios.
/// All numeric fields are raw strings exactly as returned by the server; formatting
/// (currency symbol, K/M/B/T suffix, percentage sign) is handled on the native side.
#[derive(Debug, Clone, PartialEq)]
pub struct MarketOverviewPo {
    /// Total crypto market capitalisation, e.g. `"2450000000000"` (USD)
    pub market_cap: String,
    /// 24 h market-cap change percentage, e.g. `"2.5"` means +2.5 %
    pub market_change: String,
    /// 24 h trading volume, e.g. `"95000000000"` (USD)
    pub volume: String,
    /// 24 h volume change percentage, e.g. `"-1.2"` means −1.2 %
    pub volume_change: String,
    /// Per-coin dominance list (BTC, ETH, …)
    pub dominance: Vec<MarketCapShareRatePo>,
}

/// Combined state for the full Trends card.
#[derive(Debug, Clone, PartialEq)]
pub struct MarketTrendsState {
    /// Overview card section (market cap, volume, dominance list)
    pub overview: MarketOverviewCardState,
    /// Chart section
    pub chart: MarketChartState,
    /// Upcoming calendar event from market-data API (used by OKMMVDEntranceVC).
    /// `None` when no event is returned or before first load.
    pub calendar_event: Option<FinancialEventPo>,
    /// All calendar events returned by the merged Home market-data request.
    /// 
    /// `calendar_event` remains as the first-item compatibility projection for
    /// existing consumers; Home uses this complete list to render both
    /// Economic and Earnings sections.
    pub calendar_events: Vec<FinancialEventPo>,
    /// Rust-owned compact Earnings grouping derived from [calendar_events].
    pub earnings_groups: Vec<EarningsPreviewDatePo>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MemberVo {
    pub name: String,
    pub role: String,
    pub avatar: String,
}

/// One navigation account derivative-config element.
/// 
/// Exactly three nullable raw-string fields mapped by serde to the backend wire
/// names `isSetNavConfig`, `derivativesNavConfig`, and `showConfigTimes` (matching
/// the fixed old-link `DeriConfigData`). Kept as `Option<String>` with no
/// `#[serde(default)]` coercion, so serde's built-in `Option` handling gives: a
/// not-provided field decodes to `None` (serialized absent), a wire `null`
/// decodes to `None`, and an empty wire string decodes to `Some("")` — the raw
/// meaning is preserved verbatim. Values (e.g. `derivativesNavConfig == "bizNAv"`)
/// are passed through verbatim: no case conversion, no closed-enum mapping.
#[derive(Debug, Clone, PartialEq)]
pub struct NavDerivativeConfigVo {
    /// Whether a navigation preference has been set — raw server string under the
    /// wire name `isSetNavConfig` (e.g. `"1"` / `"0"`); kept raw, never parsed to
    /// a bool.
    pub is_set_nav_config: Option<String>,
    /// The current navigation mode — raw server string under the wire name
    /// `derivativesNavConfig`. Its value is a mode string such as `bizNAv` (raw
    /// capital `A`, preserved) or `coinNav`, and any unknown mode string is passed
    /// through verbatim (never folded into a closed enum).
    pub derivatives_nav_config: Option<String>,
    /// The popup display count — raw server string under the wire name
    /// `showConfigTimes`. Kept raw; NOT converted to a numeric/computed value.
    pub show_config_times: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NavSearchCategoryMixedListState {
    pub subscription_id: u32,
    pub is_loading_list: bool,
    pub mixed_items: Vec<CategoryMixedItem>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NavSearchCategoryPageState {
    pub mixed_total_count: Option<u32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NavSearchCryptoGroupState {
    pub category_groups: Vec<CategoryGroupVo>,
    pub total_count: u32,
    pub is_loading_groups: bool,
    pub groups_error: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NavSearchCryptoListState {
    pub subscription_id: u32,
    pub is_loading_list: bool,
    pub list_error: Option<String>,
    pub crypto_list: Vec<QuotationCryptoItemVo>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NavSearchDexGroupState {
    pub chain_groups: Vec<DexChainGroupVo>,
    pub total_count: u32,
    pub is_loading_groups: bool,
    pub groups_error: Option<String>,
    pub keyword: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NavSearchDexListState {
    pub subscription_id: u32,
    pub is_loading_list: bool,
    pub list_error: Option<String>,
    pub dex_list: Vec<QuotationDexItemVo>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NavSearchEventContractGroupState {
    /// Available (method, freq) groups for tab construction. Replaces the old
    /// `available_methods` so tab keys carry both dimensions.
    pub available_method_groups: Vec<EventMethodGroupVo>,
    pub total_count: u32,
    pub is_loading: bool,
    pub error: Option<String>,
    pub keyword: Option<String>,
    /// 3-list `[freqs, methods, coins]` filter-dimension snapshot — drives the
    /// V2 dynamic chip row in NavSearch. Appended at the end of the struct so
    /// the UniFFI field layout stays backward-compatible with the previous
    /// state ABI; existing native bindings continue to deserialise the
    /// earlier fields unchanged.
    pub event_filter_dimensions: Vec<Vec<EventFilterDimVo>>,
    /// Coin-dimension chip gate — mirrors the V1
    /// `eventService.isShowEventCoinTabFilter()` AB read that NavSearch
    /// previously did native-side. Resolved Rust-side (`trade_ab`) so native
    /// no longer touches the V1 event service. When `false`, native drops the
    /// coin dim entirely. Appended last for UniFFI ABI back-compat.
    pub is_show_coin_tab_filter: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NavSearchEventContractListState {
    pub subscription_id: u32,
    pub is_loading: bool,
    pub error: Option<String>,
    pub updown_cards: Vec<EventContractUpDownCardVo>,
    pub above_groups: Vec<EventContractAboveGroupVo>,
    /// Currently selected date per base currency (mirrors Manager state).
    pub selected_date_map: Vec<SelectedDateVo>,
    /// Keyword echo of the most recent `subscribe` call. Consumers can use
    /// this to verify the state they observe matches the keyword they expect.
    pub keyword: Option<String>,
    /// Distinct series keys in HTTP inst-list response order, refreshed on
    /// every subscribe emit. Drives entry-C NavSearch card order across
    /// all three tabs (all / upDown / above) — they share the same HTTP
    /// invariant as the entry-B Event tab (§2.2.1.5 6.9). Empty when the
    /// last HTTP fetch failed; native falls back to alphabetical order.
    /// Appended at the end to keep the UniFFI field layout
    /// backward-compatible with the previous state ABI.
    pub series_keys_ordered_by_http: Vec<SeriesKeyVo>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NavSearchExchangeOsGroupState {
    /// Visible business tabs (Perp / Spot / Swap).
    pub biz_tabs: Vec<ExchangeOsBizTab>,
    /// Swap chain sub-tabs ("All" first).
    pub swap_chain_groups: Vec<DexChainGroupVo>,
    /// Keyword mode: total search hits across the groups. No-keyword mode: 0.
    pub total_count: u32,
    pub is_loading_groups: bool,
    pub groups_error: Option<String>,
    pub keyword: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NavSearchExchangeOsMarketListState {
    pub subscription_id: u32,
    pub is_loading_list: bool,
    pub list_error: Option<String>,
    pub items: Vec<ExchangeOsMarketItemVo>,
    /// Whether a further market-list page exists (drives the "load more" affordance).
    pub has_more: bool,
    /// Whether a load-more fetch is currently in flight.
    pub is_load_more_loading: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NavSearchFavoritesGroupState {
    pub inst_type_counts: Vec<FavoritesInstTypeCount>,
    pub total_count: u32,
    pub is_loading_groups: bool,
    pub groups_error: Option<String>,
    pub keyword: Option<String>,
    pub is_opt_loading: bool,
    pub opt_error: Option<String>,
    pub opt_success: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NavSearchFavoritesListState {
    pub subscription_id: u32,
    pub is_loading_list: bool,
    pub list_error: Option<String>,
    pub favorites_list: Vec<QuotationFavoriteVo>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NavSearchFuturesGroupState {
    pub category_groups: Vec<CategoryGroupVo>,
    pub units: Vec<FuturesUnitVo>,
    pub available_futures_types: Vec<FuturesType>,
    pub total_count: u32,
    pub is_loading_groups: bool,
    pub groups_error: Option<String>,
    pub keyword: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NavSearchFuturesListState {
    pub subscription_id: u32,
    pub is_loading_list: bool,
    pub list_error: Option<String>,
    pub futures_list: Vec<QuotationFuturesItemVo>,
    /// Comma-joined USD settlement currency list when the current list contains UM futures;
    /// None when no UM futures exist or no currencies are configured.
    pub display_usd_settlement_currency_reminder_text: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NavSearchFuturesPageState {
    pub page_list: Vec<FuturesPageVo>,
    pub is_loading: bool,
    pub error: Option<String>,
    pub keyword: Option<String>,
    pub total_count: u32,
}

/// State for NavSearchHistoryInteractor
#[derive(Debug, Clone, PartialEq)]
pub struct NavSearchHistoryState {
    pub history_list: Vec<SearchRecord>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NavSearchOptionsChainState {
    pub keyword: Option<String>,
    pub is_loading: bool,
    pub error: Option<String>,
    pub options_chain_list: Vec<OptionsChainItemVo>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NavSearchOptionsQuoteGroupState {
    pub category_groups: Vec<OptionsCategoryGroupVo>,
    /// R9a: per-`inst_family` expiry-date dropdown options.
    /// 
    /// Key: `OptionsCategoryGroupVo.inst_family` (literal, includes `_UM` suffix for
    /// USDⓈ-margined groups). Value: chronologically sorted millisecond timestamp
    /// strings.
    /// 
    /// **ABI break (Rust-side commit; Android consumers updated separately in R9b).**
    pub exp_time_periods: HashMap<String, Vec<String>>,
    pub total_count: u32,
    pub is_loading_groups: bool,
    pub groups_error: Option<String>,
    pub keyword: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NavSearchOptionsQuoteListState {
    pub subscription_id: u32,
    pub is_loading_list: bool,
    pub list_error: Option<String>,
    pub options_list: Vec<QuotationOptionsItemVo>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NavSearchOptionsSimpleState {
    pub keyword: Option<String>,
    pub is_loading: bool,
    pub error: Option<String>,
    pub simple_item_list: Vec<OptionsSimpleItemVo>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NavSearchResultState {
    pub is_loading: bool,
    pub error: Option<String>,
    pub keyword: Option<String>,
    pub spot_total_count: u32,
    pub futures_total_count: u32,
    pub options_total_count: u32,
    pub dex_total_count: u32,
    pub spot_list: Vec<QuotationSpotItemVo>,
    pub futures_list: Vec<QuotationFuturesItemVo>,
    pub options_quote_list: Vec<QuotationOptionsItemVo>,
    pub dex_list: Vec<QuotationDexItemVo>,
    pub events: Vec<QuotationEventContractItemVo>,
    pub events_total_count: u32,
    pub is_event_tab_active: bool,
    pub options_simple_list: Vec<OptionsSimpleItemVo>,
    pub is_simple_options_enabled: bool,
    /// R-FIX3: aggregated chain items shared between business-line mode and
    /// COIN_MODE search result (V2). Source: `GetOptionsChainsVoUseCase`,
    /// already Layer-1 ON/OFF aware (CM+UM or CM only) and post R-FIX2 +
    /// #36/#37/#38 carries the correct `margin_type` and ` CM` / ` UM`
    /// `deliver_name` suffix. Filtered case-insensitively against `keyword`
    /// on `inst_family` (V1 parity with `GetNavOptionsChainsVoUseCase`).
    pub options_chain_list: Vec<OptionsChainItemVo>,
    pub options_category_groups: Vec<OptionsCategoryGroupVo>,
    /// R9a: per-`inst_family` expiry-date dropdown options for the options tab.
    /// Key = `OptionsCategoryGroupVo.inst_family` (literal, with `_UM`), value =
    /// chronologically sorted millisecond timestamp strings.
    pub options_exp_time_periods: HashMap<String, Vec<String>>,
    pub display_usd_settlement_currency_reminder_text: Option<String>,
    pub is_show_all_spot_list: bool,
    pub is_futures_tab_active: bool,
    pub is_options_tab_active: bool,
    pub selected_options_category_group: Option<OptionsCategoryGroupVo>,
    pub selected_options_exp_time: Option<String>,
    pub options_strike_price: Option<String>,
    pub options_direction: Option<OptionsDirection>,
    /// DEX Perp sub-group.
    pub dex_perp: Vec<DexPerpItemVo>,
    /// DEX Spot sub-group.
    pub dex_spot: Vec<DexSpotItemVo>,
    /// DEX Swap sub-group (`CedefiSearchResultVo` elements).
    pub dex_swap: Vec<CedefiSearchResultVo>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NavSearchRootState {
    pub biz_tabs: Vec<NavSearchTabPage>,
    pub coin_tabs: Vec<NavSearchTabPage>,
    pub is_loading: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NavSearchSpotGroupState {
    pub category_groups: Vec<CategoryGroupVo>,
    pub charge_groups: Vec<ChargeGroupVo>,
    pub default_charge_group_id: Option<String>,
    pub total_count: u32,
    pub is_loading_groups: bool,
    pub groups_error: Option<String>,
    pub keyword: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NavSearchSpotListState {
    pub subscription_id: u32,
    pub is_loading_list: bool,
    pub list_error: Option<String>,
    pub spot_list: Vec<QuotationSpotItemVo>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NavSearchSpotPageState {
    pub page_list: Vec<SpotPageVo>,
    pub is_loading: bool,
    pub error: Option<String>,
    pub keyword: Option<String>,
    pub total_count: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NavSearchTradFiPageState {
    /// L2 tabs in cloud order, already narrowed by the category data gate. Empty
    /// while loading, and also when the cloud scene carries nothing usable (the
    /// tab is not offered in that case). One entry means the host shows the
    /// business line as the tab strip; several means the categories are the strip
    /// and the business line moves into a dropdown.
    pub tabs: Vec<TradFiL2TabVo>,
    /// Page-level business-line availability. Both layouts render the other
    /// dimension from these two flags (tab strip, or the top-right dropdown).
    pub has_spot_tab: bool,
    pub has_futures_tab: bool,
    pub is_loading: bool,
    pub mixed_total_count: Option<u32>,
}

/// New coin futures rank item response from API
/// 
/// Represents a single item in the new coin futures rank list.
/// This corresponds to the Kotlin `NewCoinFuturesRankPo` data class.
/// 
/// @since V6.102.0
#[derive(Debug, Clone, PartialEq)]
pub struct NewCoinFuturesRankPo {
    /// Last price - latest transaction price
    pub last_price: String,
    /// Change percentage in 24 hours
    pub change_per_day24h: String,
    /// Change percentage at UTC 0
    pub change_per_day_utc0: String,
    /// Change percentage at UTC 8
    pub change_per_day_utc8: String,
    /// Icon URL - currency icon
    pub icon: String,
    /// Online timestamp - listing time
    pub online_ts: String,
    /// Alias - alternative name
    pub alias: Option<String>,
    /// Expiration time
    pub exp_time: Option<String>,
    /// Rule type - pre_market means pre-market trading; normal means regular trading
    pub rule_type: Option<String>,
    /// Instrument ID (e.g., "BTC-USDT-SWAP")
    pub inst_id: String,
    /// Instrument type - business line, PERPETUAL, PREMARKET
    pub inst_type: String,
    /// Whether the instrument has a recently published earnings event.
    pub has_recent_earnings: bool, // default = false
    /// New coin state type (computed from CallAuction subscription)
    /// Represents the current trading state: CountDown, WillOnline, Cancel, Auction, PreQuote, Trading
    pub new_coin_state_type: NewCoinStateType,
    /// Countdown timestamp in milliseconds (computed from CallAuction subscription)
    /// Only valid when new_coin_state_type is CountDown or WillOnline
    pub new_coin_count_down_timestamp: i64,
    /// USD price computed from last_price x cup ticker rate
    pub last_usd_price: String,
}

/// New coin group response from API
/// 
/// Represents the complete response for new coin rank, including online, wait online, and futures lists.
/// This corresponds to the Kotlin `NewCoinGroupResponse` data class.
#[derive(Debug, Clone, PartialEq)]
pub struct NewCoinGroupResponse {
    /// Online new coin rank list
    pub online: Vec<NewCoinRankPo>,
    /// Wait online new coin rank list
    pub wait_online_new: Vec<WaitOnlineRankPo>,
    /// Wait online coin rank list
    pub wait_online: Vec<WaitOnlineRankPo>,
    /// Futures online list - new coin futures list
    /// @since V6.102.0
    pub futures_online: Option<Vec<NewCoinFuturesRankPo>>,
    /// Futures wait online list - new coin futures pending listing
    pub futures_wait_online: Option<Vec<WaitOnlineRankPo>>,
}

/// New coin info VO for parent tab badge data.
#[derive(Debug, Clone, PartialEq)]
pub struct NewCoinInfoVo {
    /// Number of upcoming new spot coins
    pub new_coin_count: i32,
    /// Names of upcoming new spot coins
    pub coin_names: Vec<String>,
    /// Number of upcoming new futures
    pub futures_count: i32,
    /// Names of upcoming new futures
    pub futures_names: Vec<String>,
}

/// New coin rank item response from API
/// 
/// Represents a single item in the new coin rank list.
/// This corresponds to the Kotlin `NewCoinRank` data class.
#[derive(Debug, Clone, PartialEq)]
pub struct NewCoinRankPo {
    /// Icon URL
    pub icon: String,
    /// Online timestamp
    pub online_ts: String,
    /// Last price
    pub last_price: String,
    /// Change percentage in 24 hours
    pub change_per_day24h: String,
    /// Change percentage at UTC 0
    pub change_per_day_utc0: String,
    /// Change percentage at UTC 8
    pub change_per_day_utc8: String,
    /// Display ID - display trading pair name (e.g., "BTC-USDT")
    pub display_id: Option<String>,
    /// Rule type - pre_market means pre-market trading; normal means regular trading
    pub rule_type: Option<String>,
    /// Instrument ID (e.g., "BTC-USDT")
    pub inst_id: String,
    /// Instrument type (e.g., "SPOT", "SWAP")
    pub inst_type: String,
    /// Whether the instrument has a recently published earnings event.
    pub has_recent_earnings: bool, // default = false
    /// New coin state type (computed from CallAuction subscription)
    /// Represents the current trading state: CountDown, WillOnline, Cancel, Auction, PreQuote, Trading
    pub new_coin_state_type: NewCoinStateType,
    /// Countdown timestamp in milliseconds (computed from CallAuction subscription)
    /// Only valid when new_coin_state_type is CountDown or WillOnline
    pub new_coin_count_down_timestamp: i64,
    /// USD price computed from last_price x cup ticker rate
    pub last_usd_price: String,
    /// Display base coin name derived from display_id (falls back to inst_id base).
    /// Display-only; inst_id remains the raw key for ticker/sort/dedup.
    pub display_coin_name: String,
}

/// Options Category Group View Object - Options family grouping
/// 
/// Corresponds to:
/// `com.okinc.market.quotation.ui.model.OptionsCategoryGroupVo`
/// 
/// Implements `Ord` for use as key in `BTreeMap` to ensure deterministic iteration order.
/// The ordering is lexicographic: first by `symbol`, then by `inst_family` (preserving the
/// original `_UM` suffix), then by `margin_type` when both are equal.
/// 
/// Note: `inst_family` keeps the raw `_UM` suffix for USDⓈ-margined groups (e.g.
/// `BTC-USD_UM`). Do NOT strip the suffix here — Android relies on the literal value to
/// disambiguate CM vs UM groups. Use `margin_type` for typed discrimination.
#[derive(Debug, Clone, PartialEq)]
pub struct OptionsCategoryGroupVo {
    /// Symbol (e.g., "BTC", "ETH")
    /// 
    /// Primary ordering field: categories are sorted first by symbol lexicographically.
    pub symbol: String,
    /// Instrument family (e.g., "BTC-USD", "BTC-USD_UM", "ETH-USDT")
    /// 
    /// Secondary ordering field: used for ordering when symbols are equal.
    /// IMPORTANT: keeps the original `_UM` suffix for USDⓈ-margined groups.
    pub inst_family: String,
    /// Margin type for this category group: `Cm` or `Um`.
    /// 
    /// Mirrors OKMarket Kotlin `OptionsCategoryGroupVo.marginType`. Native callers should
    /// rely on this field for branching, not on parsing `inst_family`.
    pub margin_type: MarginType,
}

/// Options chain item VO (for T-type quotation display)
/// 
/// Corresponds to:
/// - `com.okinc.market.quotation.ui.model.OptionsChainCoinItemVo` (Kotlin)
/// 
/// Represents a coin item in the options chain with deliver name and instrument family.
#[derive(Debug, Clone, PartialEq)]
pub struct OptionsChainItemVo {
    /// Display name for the coin (e.g., "BTCUSD CM" or "ETHUSD UM").
    /// 
    /// Built via `format_deliver_name(inst_family, margin_type)` to mirror V1
    /// Kotlin `CoinFormat.formatOptionInstFamilyDisplay()`. Always carries the
    /// trailing ` CM` / ` UM` tag — Android / iOS rely on the visible suffix to
    /// disambiguate coin-margined vs USDⓈ-margined chain rows.
    pub deliver_name: String,
    /// Instrument family (e.g., "BTC-USD", "BTC-USD_UM", "ETH-USDT")
    /// 
    /// IMPORTANT: keeps the original `_UM` suffix for USDⓈ-margined options.
    pub inst_family: String,
    /// Margin type for this options chain item: `Cm` or `Um`.
    /// 
    /// Mirrors OKMarket Kotlin `OptionsChainCoinItemVo.marginType`.
    pub margin_type: MarginType,
    /// Underlying-level tradability flag (TEPT-6221, added at struct end for ABI stability).
    /// 
    /// Aggregated from the FIRST instrument under this underlying (all instruments under one
    /// underlying share the same backend flag). `None` = backend sent nothing -> treat as
    /// tradable (default-off / EEA parity). Surfaces over UniFFI as Kotlin `isTradable: Boolean?`.
    pub is_tradable: Option<bool>,
}

/// Raw inputs for an Options list item (from `QuotationOptionsItemVo`).
#[derive(Debug, Clone, PartialEq)]
pub struct OptionsItemFormatInput {
    /// Mark price. `None` -> no mark-price recipe.
    pub mark_price: Option<String>,
    /// Instrument family for OPTION precision resolution.
    pub inst_family: String,
    /// Change ratio, already finite-checked by native. `None` -> no change
    /// recipe (native renders the "0.0%" neutral placeholder itself).
    pub change_ratio: Option<String>,
    /// USD turnover; native passes "0" when the server omits volume.
    pub turnover_usd: Option<String>,
}

/// Recipes for an Options list item.
#[derive(Debug, Clone, PartialEq)]
pub struct OptionsItemRecipes {
    /// Mark-price recipe, or `None`.
    pub mark_price: Option<MarketFormatRecipe>,
    /// Change ratio recipe, or `None` when native supplied no change ratio
    /// (native shows the "0.0%" neutral placeholder).
    pub change: Option<MarketFormatRecipe>,
    /// Turnover recipe — always present.
    pub turnover: MarketFormatRecipe,
}

/// Options chain simple item VO (for simple selection display)
/// 
/// Corresponds to:
/// `com.okinc.market.quotation.ui.model.OptionsSimpleItemVo` (Kotlin)
/// 
/// Used for displaying simplified options chain items with price information.
/// 
/// Note: `raw_index_tickers_data` from Kotlin is not included here as `IndexTickersData`
/// is not UniFFI-compatible. If needed, it should be handled in native code.
#[derive(Debug, Clone, PartialEq)]
pub struct OptionsSimpleItemVo {
    /// Coin name (e.g., "BTC", "ETH")
    pub name: String,
    /// Icon URL for the coin (optional)
    pub icon_url: Option<String>,
    /// Formatted price string (e.g., "$50000.00")
    pub price: String,
    /// Index instrument id (e.g., "BTC-USD") for opening simple option page
    pub inst_id: String,
    /// Underlying-level tradability flag (TEPT-6221, added at struct end for ABI stability).
    /// 
    /// Aggregated from the FIRST instrument under this underlying. `None` = backend sent
    /// nothing -> treat as tradable (default-off / EEA parity). Surfaces over UniFFI as Kotlin
    /// `isTradable: Boolean?`.
    pub is_tradable: Option<bool>,
}

/// A single position tier row for native rendering.
/// 
/// CR-4 / CC-1: OKCore returns RAW values only; OKKLine Native performs all
/// locale-aware formatting (grouping, percent sign). The legacy `max_amount` /
/// `imr` / `mmr` fields are kept (never retyped — CC-2) but now carry the RAW
/// value (no comma grouping, no percent sign). The additive `*_raw` fields make
/// the raw source explicit and carry the percent semantics: `imr_raw` / `mmr_raw`
/// are raw RATIOS (e.g. "0.01" = 1%) that Native scales x100 and formats with the
/// locale percent sign; `max_amount_raw` is the raw amount Native groups per Locale.
#[derive(Debug, Clone, PartialEq)]
pub struct PositionTierItemVo {
    /// Tier level ("1", "2", ...)
    pub tier: String,
    /// Raw max position amount (e.g. "10000"), no comma grouping.
    pub max_amount: String,
    /// Max leverage (e.g. "100")
    pub max_leverage: String,
    /// Initial margin ratio as a RAW ratio (e.g. "0.01"), no percent sign.
    pub imr: String,
    /// Maintenance margin ratio as a RAW ratio (e.g. "0.004"), no percent sign.
    pub mmr: String,
    /// Additive raw max position amount (Option for forward-compat; same value as
    /// `max_amount`). Native groups per Locale.
    pub max_amount_raw: Option<String>,
    /// Additive raw initial margin RATIO (e.g. "0.01"); Native x100 + locale %.
    pub imr_raw: Option<String>,
    /// Additive raw maintenance margin RATIO (e.g. "0.004"); Native x100 + locale %.
    pub mmr_raw: Option<String>,
}

/// State for the position tier page.
#[derive(Debug, Clone, PartialEq)]
pub struct PositionTierState {
    /// Formatted display items
    pub tiers: Vec<PositionTierItemVo>,
    /// Whether a network request is in progress
    pub is_loading: bool,
    /// Error message if the last request failed
    pub error: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PreMarketLaunchState {
    /// Preview items (max 4 on initial load).
    pub items: Vec<PreMarketTokenVo>,
    /// Whether a "View More" button should be shown.
    pub has_more: bool,
    /// Total count of filtered items.
    pub total_count: u32,
    /// Whether a network request is in progress.
    pub is_loading: bool,
    /// Error message from the last failed fetch.
    pub error: Option<String>,
}

/// Pre-market token view object.
#[derive(Debug, Clone, PartialEq)]
pub struct PreMarketTokenVo {
    pub inst_id: String,
    pub ccy: String,
    pub full_name: String,
    pub icon: String,
    /// L005: `bigIcon` preferred, fallback to `icon`.
    pub display_icon: String,
    pub state: PreMarketState,
    pub introduce: String,
    pub last_price: String,
    pub total_volume: String,
    pub official_url: String,
    pub twitter: String,
    pub telegram: String,
    /// Listing timestamp (ms since epoch). Formatting is done by native.
    pub list_time: String,
    /// `"SWAP"` or `"FUTURES"` — native determines biz type from this.
    pub inst_type: String,
    /// L006: `true` when `ruleType == "rebase_contract"`.
    pub is_rebase: bool,
}

/// Crypto Item View Object Snapshot for UniFFI export
/// 
/// This is the FFI-friendly version that excludes internal-only fields like `groups`.
/// Use this type when exporting to mobile platforms via UniFFI.
#[derive(Debug, Clone, PartialEq)]
pub struct QuotationCryptoItemVo {
    /// Trade symbol from TradeCoinInfo (e.g., "BTC", "ETH")
    /// 
    /// This is the raw/canonical symbol and MUST stay the logic key (navigation,
    /// watchlist matching, ticker subscription). For the user-facing symbol use
    /// `display_symbol` instead. Mirrors the spot VO split of `name` (logic key)
    /// vs `display_trade_symbol` (display).
    pub symbol: String,
    /// Display symbol from TradeCoinInfo, falling back to `symbol` when blank
    /// 
    /// Sourced from `TradeCoinInfo::get_display_symbol()` (e.g., "xMU" where the
    /// raw symbol is "XMU"). Use this for rendering the coin symbol; keep `symbol`
    /// for any lookup/matching. Empty only when the coin's `TradeCoinInfo` was
    /// unavailable (delisted / not-yet-loaded), in which case consumers fall back
    /// to `symbol`.
    pub display_symbol: String,
    /// Description (full name) from TradeCoinInfo (e.g., "bitcoin", "ethereum")
    pub desc: String,
    /// Single pre-resolved coin subtitle: trimmed commodity tag, else trimmed
    /// full name, else empty. Never a concatenation. `desc` stays the raw full
    /// name for other consumers; this is the SSOT subtitle for the V2 coin row.
    /// 
    /// `#[uniffi(default = "")]` keeps existing native (Kotlin/Swift) constructors
    /// of this Record source-compatible — callers may omit the new field — while the
    /// Rust `From`/default paths always populate it. Additive-only, ABI forward-safe.
    pub resolved_subtitle: String, // default = ""
    /// Icon URL from TradeCoinInfo
    pub icon_url: String,
    /// Price from TickersData.last
    pub price: Option<f64>,
    /// Change ratio value from TickersData (calculated as percentage)
    pub change_ratio_value: Option<f64>,
    /// Whether this item is in watchlist
    pub is_watching: bool,
    /// Whether this item is a hot coin
    pub is_hot: bool,
}

/// DEX Item View Object
#[derive(Debug, Clone, PartialEq)]
pub struct QuotationDexItemVo {
    /// Token symbol, e.g. "BTC"
    pub token_symbol: String,
    /// Token contract address, e.g. "0x123..."
    pub token_contract_address: String,
    /// Token logo URL, e.g. "https://static.coinall.ltd/cdn/wallet/logo/BTC-20220525.png"
    pub token_logo_url: String,
    /// Chain logo URL, e.g. "https://static.coinall.ltd/cdn/wallet/logo/SOL-20220525.png"
    pub chain_logo_url: String,
    /// Chain ID, e.g. "501"
    pub chain_id: String,
    /// Chain name, e.g. "X Layer"
    pub chain_name: String,
    /// Price change ratio value (percentage)
    pub change_ratio_value: Option<f64>,
    /// Current price value
    pub price_value: Option<f64>,
    /// 24h turnover (trading volume in USD)
    pub turnover_value: Option<String>,
    /// Market capitalization
    pub market_cap_value: Option<String>,
    /// Liquidity
    pub liquidity: Option<String>,
    /// Whether this item is in watchlist
    pub is_watching: bool,
    /// Whether verified by official
    pub is_verified: bool,
    /// Show boost rate when is Some, otherwise None
    pub boost_rate: Option<String>,
    /// Full name of the token (e.g., "iShares Ethereum Trust ETF" for RWA tokens)
    pub full_name: Option<String>,
    /// DEX Spot/Perp asset identity (only set when dex_sub_type is SPOT/PERP).
    /// None for the legacy on-chain Swap VO, which is keyed by chain_id + contract.
    pub asset_id: Option<String>, // default = None
    /// DEX sub-type: SWAP (legacy) / SPOT / PERP. None or empty means SWAP.
    pub dex_sub_type: Option<String>, // default = None
    /// Display name for a DEX Spot/Perp row. None for the legacy Swap VO
    /// (consumers fall back to token_symbol).
    pub name: Option<String>, // default = None
    /// Raw USD price for a DEX Spot/Perp row's secondary price line (converted to the
    /// user's local display currency by the native `MarketFormatter.formatFiatPrice`,
    /// same as `QuotationSpotItemVo`/`QuotationFuturesItemVo`'s `sub_price_value`).
    /// None for the legacy Swap VO (no USD-denominated ticker field to derive it from)
    /// and for a Spot/Perp row before its first ticker merge.
    pub sub_price_value: Option<f64>, // default = None
}

/// Single event contract item (FFI export — minimal fields for Android UI)
#[derive(Debug, Clone, PartialEq)]
pub struct QuotationEventContractItemVo {
    pub inst_id: String,
    pub base_currency: String,
    pub method: EventContractMethod,
    /// Frequency carried through from the underlying instrument so native
    /// rendering can display per-item freq instead of guessing from a tab key.
    pub freq: String,
    pub strike_price: String,
    /// BETWEEN upper bound (raw string). Empty for non-BETWEEN. Native composes
    /// the range label ("<cap" / "≥floor" / "floor-cap") from strike + cap.
    pub cap_strike_price: String,
    pub exp_time_ms: i64,
    pub probability: String,
    pub vol_share: String,
    /// Server-clamped UI display price (FSP-9179). Native list cards must read
    /// this field instead of `probability`. When upstream display_price is absent
    /// or invalid, `ApplyEventContractTickersUseCase` falls back to invalid/probability
    /// handling, then `"0.5"` so V2 native matches Kotlin V1.
    pub display_price: String,
    pub is_invalid: bool,
}

/// Futures Item View Object for UniFFI export
/// 
/// This is the FFI-friendly version that includes all fields needed for UI display.
/// Use this type when exporting to mobile platforms via UniFFI.
#[derive(Debug, Clone, PartialEq)]
pub struct QuotationFuturesItemVo {
    /// Instrument ID (e.g., "BTC-USDT-SWAP", "BTC-USDT-240101")
    pub inst_id: String,
    /// Instrument type (e.g., "SWAP", "FUTURES")
    pub inst_type: String,
    /// Trading symbol (e.g., "BTC", "ETH", "OKB")
    pub trade_symbol: String,
    /// Quote symbol (e.g., "USDT", "USDC", "USD")
    pub quote_symbol: String,
    /// Icon URL for the underlying asset
    pub icon_url: Option<String>,
    /// Price change ratio value (percentage)
    pub change_ratio_value: Option<f64>,
    /// Current price value
    pub price_value: Option<f64>,
    /// USD equivalent price value
    pub sub_price_value: Option<f64>,
    /// Raw 24-hour high price from the pair ticker.
    pub high_24h: Option<String>, // default = None
    /// Raw 24-hour low price from the pair ticker.
    pub low_24h: Option<String>, // default = None
    /// Volume value (using String to represent BigDecimal)
    pub volume_value: Option<String>,
    /// Turnover value (using String to represent BigDecimal)
    pub turnover_value: Option<String>,
    /// Whether this item is in watchlist
    pub is_watching: bool,
    /// Whether the underlying stock published earnings within the last 30 days.
    pub has_recent_earnings: bool, // default = false
    /// Currency type (replaces quoteSymbol for USD-margined contracts)
    pub ccy_type: Option<String>,
    /// Market capitalization value
    pub market_cap: Option<String>,
    /// List time (online date) - used for new coins category
    pub list_time: Option<String>,
    /// Instrument family (needed for price formatting)
    pub inst_family: String,
    /// Whether this item is in pre-market
    pub is_pre_market: bool,
    /// Whether this item is a rebase contract (ruleType = "rebase_contract")
    pub is_rebase: bool,
    /// Whether this item is an X-Perp contract (ruleType = "xperp")
    pub is_xperp: bool,
    /// Whether this item has zero trading fee (maker & taker both "0")
    pub is_zero_fee: bool,
    /// Whether this item is positive contract
    pub is_positive_contract: bool,
    /// Whether this item is usd um contract
    pub is_usd_um_contract: bool,
    /// New Coin State Type
    pub new_coin_state_type: NewCoinStateType,
    /// New Coin State Count Down Timestamp
    pub new_coin_count_down_timestamp: i64,
    /// Is new tag for the raw instrument
    pub is_new_tag: bool,
    /// Is new label for the raw instrument
    pub is_new_label: bool,
    /// Contract alias for delivery futures (e.g., "this_week", "next_week", "quarter")
    /// Empty string for perpetual swaps.
    pub alias: String,
    /// Whether this item is currently selected in a selection UI
    /// Default: false
    pub is_selected: bool,
    /// Whether this item is enabled for selection interaction
    /// Default: true (can be disabled for pre-selected items)
    pub is_enable: bool,
    /// Full coin name (e.g., "Bitcoin", "Ethereum")
    pub full_name: Option<String>,
    /// Whether this coin is a commodity/TradFi asset (e.g., stocks, commodities)
    /// Sourced from TradeCoinInfo.commodity_tag in coins_map
    pub commodity_tag: Option<String>,
    /// Contract value per contract (e.g., "100" for BTC-USD-SWAP = 100 USD per contract)
    /// Used for turnover calculation: vol_24h × contract_val × cup_rate
    pub contract_val: String,
    /// Whether this instrument is tradable for the current user.
    /// Entity 39 users may see X-Perps instruments that are visible but not tradable.
    /// None means tradable (backward compatible default).
    pub is_tradable: Option<bool>,
}

/// Options Item View Object for UniFFI export
/// 
/// This is the FFI-friendly version that includes all fields needed for UI display.
/// Use this type when exporting to mobile platforms via UniFFI.
#[derive(Debug, Clone, PartialEq)]
pub struct QuotationOptionsItemVo {
    /// Instrument ID (e.g., "BTC-USD-240101-30000-C")
    pub inst_id: String,
    /// Display name with formatted date (e.g., "BTCUSD CM-20240101-30000-C",
    /// "ETHUSD UM-20260702-1575-P"), aligned with the V1 watchlist format.
    /// Constructed from the deliver-name prefix (`{BASE}{QUOTE} {CM|UM}`,
    /// `_UM` stripped) + exp_time (YYYYMMDD) + strike + direction.
    pub name: String,
    /// Instrument type (e.g., "OPTION")
    pub inst_type: String,
    /// Instrument family (e.g., "BTC-USD") - used for mark price formatting on mobile
    pub inst_family: String,
    /// Expiration time (e.g., "240101")
    pub exp_time: String,
    pub strike_price: String,
    /// Mark price (underlying asset price)
    pub mark_price_value: Option<f64>,
    /// Options direction (Call/Put)
    pub direction: OptionsDirection,
    /// Price change ratio value (percentage)
    pub change_ratio_value: Option<f64>,
    /// 24h turnover value in USD (from vol_ccy_24h for derivatives)
    pub turnover_value: Option<String>,
    /// Whether this item is in watchlist
    pub is_watching: bool,
    /// Full coin name (e.g., "Bitcoin") — populated from BizManager coin data.
    /// `None` if BizManager has not enriched the instrument yet.
    pub full_name: Option<String>,
    /// Margin type for this options item: `Cm` or `Um`.
    /// 
    /// Mirrors OKMarket Kotlin `QuotationOptionsItemVo.marginType`. Derived from
    /// `instrument.is_usdum_contract()` at construction time.
    pub margin_type: MarginType,
    /// Instrument-level tradability flag (TEPT-6221, added at struct end for ABI stability).
    /// 
    /// Read directly from the backing instrument (`BizInstrument::is_tradable()`); the Options
    /// Quote surface renders at instrument granularity, so no underlying aggregation. `None` =
    /// backend sent nothing -> treat as tradable (default-off / EEA parity). Surfaces over UniFFI
    /// as Kotlin `isTradable: Boolean?`.
    pub is_tradable: Option<bool>,
}

/// Spot Item View Object for UniFFI export
/// 
/// This is the FFI-friendly version that includes all fields needed for UI display.
/// Use this type when exporting to mobile platforms via UniFFI.
#[derive(Debug, Clone, PartialEq)]
pub struct QuotationSpotItemVo {
    /// Instrument ID (e.g., "BTC-USDT")
    pub inst_id: String,
    /// Instrument type (e.g., "SPOT", "MARGIN")
    pub inst_type: String,
    /// Spot icon URL
    pub icon_url: Option<String>,
    /// Spot coin name (e.g., "BTC", "ETH")
    pub name: String,
    /// Display trade symbol (renamed base, display-only). Falls back to raw `name` when blank.
    /// `name` stays raw as the logic key (icon url / baseCcy); render sites read this instead.
    /// MOCK (revert: drop the "mk" prefix): value = "mk" + get_display_trade_symbol() for visible testing.
    pub display_trade_symbol: String,
    /// Quote currency (e.g., "USDT", "USDC")
    pub quote: String,
    /// Leverage multiplier (e.g., "3x", "5x")
    pub leverage: String,
    /// Price change ratio value (percentage)
    pub change_ratio_value: Option<f64>,
    /// Current price value
    pub price_value: Option<f64>,
    /// Raw 24-hour high price from the pair ticker.
    pub high_24h: Option<String>, // default = None
    /// Raw 24-hour low price from the pair ticker.
    pub low_24h: Option<String>, // default = None
    /// USD equivalent price value
    pub sub_price_value: Option<f64>,
    /// Volume value (using String to represent BigDecimal)
    pub volume_value: Option<String>,
    /// Turnover value (using String to represent BigDecimal)
    pub turnover_value: Option<String>,
    /// Whether this item is in watchlist
    pub is_watching: bool,
    /// Whether the underlying stock published earnings within the last 30 days.
    /// Favorites copy their grouped-API marker; market lists copy the captured
    /// TradFi base-info snapshot marker.
    pub has_recent_earnings: bool, // default = false
    /// Market capitalization value
    pub market_cap: Option<String>,
    /// List time (online date) - used for new coins category
    pub list_time: Option<String>,
    /// Instrument family (needed for price formatting)
    pub inst_family: String,
    /// New Coin State Type
    pub new_coin_state_type: NewCoinStateType,
    /// New Coin State Count Down Timestamp
    pub new_coin_count_down_timestamp: i64,
    /// Is new tag for the raw instrument
    pub is_new_tag: bool,
    /// Is new label for the raw instrument
    pub is_new_label: bool,
    /// Whether this item has zero trading fee (maker & taker both "0")
    pub is_zero_fee: bool,
    /// Whether this item is currently selected in a selection UI
    /// Default: false
    pub is_selected: bool,
    /// Whether this item is enabled for selection interaction
    /// Default: true (can be disabled for pre-selected items)
    pub is_enable: bool,
    /// Whether to display trade receive (USD pay-with) reminder.
    /// True when `trade_quote_ccy_list` has more than 1 entry.
    pub is_display_trade_receive: bool,
    /// Display quote symbol for the USD reminder banner title (e.g., "USDC")
    pub display_quote_symbol: String,
    /// Full coin name (e.g., "Bitcoin", "Ethereum")
    pub full_name: Option<String>,
    /// Whether this coin is a commodity/TradFi asset (e.g., stocks, commodities)
    /// Sourced from TradeCoinInfo.commodity_tag in coins_map
    pub commodity_tag: Option<String>,
    /// Whether this instrument is tradable for the current user.
    /// None means the backend did not provide the flag and is treated as tradable.
    pub is_tradable: Option<bool>,
}

/// Aggregated rank overview from the all-rank-list API.
/// 
/// A single HTTP request returns the top items for every ranking category.
/// Used for the horizontal card carousel at the top of the ranking page.
/// 
/// Corresponds to iOS `OKGDRankTopModel`.
/// 
/// API: GET /priapi/v5/rubik/app/public/all-rank-list
#[derive(Debug, Clone, PartialEq)]
pub struct RankOverviewData {
    /// Hot attention ranking
    pub hot_rank: Vec<RankOverviewItemPo>,
    /// New coin ranking
    pub coins: Vec<RankOverviewItemPo>,
    /// Top gainers
    pub top_gainer: Vec<RankOverviewItemPo>,
    /// Top losers
    pub top_loser: Vec<RankOverviewItemPo>,
    /// Market cap ranking
    pub market_cap: Vec<RankOverviewItemPo>,
    /// Turnover ranking
    pub turn_over: Vec<RankOverviewItemPo>,
    /// Most buyers ranking
    pub most_buyer: Vec<RankOverviewItemPo>,
    /// Most sellers ranking
    pub most_seller: Vec<RankOverviewItemPo>,
    /// Popular search ranking
    pub popular_search: Vec<RankOverviewItemPo>,
}

/// A single item in the rank overview response.
/// 
/// Unified item type returned by the all-rank-list API.
/// Each ranking category returns items with the same structure.
/// 
/// Corresponds to iOS `OKGDRankTopDetailModel`.
#[derive(Debug, Clone, PartialEq)]
pub struct RankOverviewItemPo {
    /// Instrument ID (e.g., "BTC-USDT")
    pub inst_id: String,
    /// Instrument/business type (e.g., "SPOT")
    pub inst_type: String,
    /// Coin icon URL
    pub icon: String,
    /// Coin icon URL (version 2)
    pub icon_v2: String,
    /// Full coin name (version 2, e.g., "Bitcoin")
    pub full_name_v2: String,
    /// Currency code (version 2, e.g., "BTC")
    pub ccy_v2: String,
    /// Whether the instrument has a recently published earnings event.
    pub has_recent_earnings: bool, // default = false
    /// Latest price
    pub last_price: String,
    /// Price change percentage (server-selected based on zone)
    pub change_per: String,
    /// 24h price change percentage
    pub change_per_day24h: String,
    /// Price change percentage at UTC+0
    pub change_per_day_utc0: String,
    /// Price change percentage at UTC+8
    pub change_per_day_utc8: String,
    /// Listing timestamp (for new coins)
    pub online_ts: String,
    /// Hot index (for hot ranking)
    pub hot_index: String,
    /// Search count (for popular search)
    pub search_count: String,
    /// Buyer count (for most-buyer ranking)
    pub buyer_count: String,
    /// Seller count (for most-seller ranking)
    pub seller_count: String,
    /// Market capitalization
    pub market_cap: String,
    /// Trading volume/turnover
    pub turn_over: String,
    /// USD price computed from last_price x cup ticker rate
    pub last_usd_price: String,
}

/// State of the ranking detail interactor.
/// 
/// All fields are automatically exposed to native code as a UniFFI record.
#[derive(Debug, Clone, PartialEq)]
pub struct RankingDetailState {
    /// Currently selected ranking type
    pub rank_type: RankingDetailType,
    /// Which sub-tab was requested (Spot or Futures)
    pub market_type: RankingDetailMarketType,
    /// Ranking data for the current `rank_type`, or `None` if not yet loaded
    pub data: Option<RankingDetailData>,
    /// Overview data: top items for all ranking categories (horizontal card carousel)
    pub overview: Option<RankOverviewData>,
    /// Whether a ranking fetch is in progress
    pub is_loading: bool,
    /// Whether the overview fetch is in progress
    pub is_overview_loading: bool,
    /// Error message from the last failed fetch, or `None` if no error
    pub error: Option<String>,
    /// Monotonically increasing version counter to ensure state change detection
    /// even when ranking data is structurally identical across period switches.
    pub version: u64,
}

/// Group favorite
#[derive(Debug, Clone, PartialEq)]
pub struct RetailTradingGroupFavorite {
    pub group_name: Option<String>,
    pub translated_group_name: Option<String>,
    pub is_preset: Option<bool>,
    pub is_hidden: Option<bool>,
    pub favorites: Option<Vec<RetailTradingFavoriteItem>>,
}

/// Cumulative PnL chart response.
/// 
/// GET /priapi/v5/journal/spot/cumulative-pnl
/// Server returns [[...]] array; take first element.
#[derive(Debug, Clone, PartialEq)]
pub struct SaChartPo {
    /// Today's PnL value
    pub today_pnl: String,
    /// Total cumulative PnL for the selected period
    pub pnl: String,
    /// Time-series data points
    pub every_time_step_pnl_list: Vec<SaChartPointPo>,
}

/// Individual data point in the chart time series.
#[derive(Debug, Clone, PartialEq)]
pub struct SaChartPointPo {
    /// Timestamp in milliseconds (as string)
    pub date_time: String,
    /// PnL value at this time step
    pub pnl: String,
    /// Granularity indicator: "2" = daily (MonthDay), other = monthly (MonthYear)
    pub time_step: String,
}

/// State for the spot analysis chart.
#[derive(Debug, Clone, PartialEq)]
pub struct SaChartState {
    /// Chart data from API
    pub data: Option<SaChartPo>,
    /// Chart date format derived from response
    pub date_format: SaChartDateFormat,
    /// Today's PnL as a formatted recipe
    pub today_pnl_recipe: MarketFormatRecipe,
    /// Total cumulative PnL as a formatted recipe
    pub total_pnl_recipe: MarketFormatRecipe,
    /// Whether a network request is in progress
    pub is_loading: bool,
    /// Error message if the last request failed
    pub error: Option<String>,
}

/// Formatted wrapper for a coin item, containing pre-computed recipes.
/// 
/// Produced by the coin list interactor for each SaCoinItemPo.
/// Native uses these recipes to format numbers with locale-aware formatting.
#[derive(Debug, Clone, PartialEq)]
pub struct SaCoinItemFormatted {
    /// Raw coin data from API
    pub raw: SaCoinItemPo,
    /// Formatted PnL value (precision from valuation unit, prepend_plus, color hint via sign)
    pub pnl_recipe: MarketFormatRecipe,
    /// Formatted PnL rate (uses MarketFormatter.format_change)
    pub pnl_rate_recipe: MarketFormatRecipe,
    /// Formatted balance (precision from OkSpotBiz sizeDig, HALF_DOWN rounding,
    /// trailing zeros kept)
    /// Only meaningful for CurrentHoldings tab; empty recipe for AllTime.
    pub balance_recipe: MarketFormatRecipe,
    /// Formatted position market value (valuation-currency sign, HALF_DOWN
    /// rounding, no "+" prefix)
    /// Only meaningful for CurrentHoldings tab; empty recipe for AllTime.
    pub position_value_recipe: MarketFormatRecipe,
}

/// Individual coin PnL entry.
#[derive(Debug, Clone, PartialEq)]
pub struct SaCoinItemPo {
    /// Coin symbol (e.g., "BTC")
    pub name: String,
    /// Instrument name
    pub insta_name: String,
    /// Holding quantity
    pub balance: String,
    /// PnL value in valuation currency
    pub pnl: String,
    /// Cost price
    pub cost_price: String,
    /// PnL ratio (decimal, e.g., "0.05" for 5%)
    pub pnl_rate: String,
    /// Current position market value
    pub position_market_value: String,
    /// Coin icon URL. Resolved from the global coins map using `name` as the symbol.
    /// Not present in the API response.
    pub icon_url: String,
}

/// State for the spot analysis coin list.
#[derive(Debug, Clone, PartialEq)]
pub struct SaCoinListState {
    /// Formatted coin items (sorted)
    pub items: Vec<SaCoinItemFormatted>,
    /// Holdings PnL summary (CurrentHoldings tab only)
    pub holding_pnl_recipe: MarketFormatRecipe,
    /// Current tab
    pub segment: SaSegmentType,
    /// Current sort state for CurrentHoldings tab
    pub current_holdings_filter: SaPnlFilterState,
    /// Current sort state for AllTime tab
    pub all_time_filter: SaPnlFilterState,
    /// Whether a network request is in progress
    pub is_loading: bool,
    /// Error message if the last request failed
    pub error: Option<String>,
}

/// Current sorting state for the coin list (per tab).
#[derive(Debug, Clone, PartialEq)]
pub struct SaPnlFilterState {
    /// Which column is being sorted
    pub filter_type: SaPnlFilterType,
    /// Sort direction
    pub order: SaPnlSortOrder,
}

/// Response envelope of the activity-recommend API (the `data` object).
/// 
/// API: GET /priapi/v5/rubik/search/activity-recommend (data)
/// All arrays default to empty so a missing field deserializes cleanly.
#[derive(Debug, Clone, PartialEq)]
pub struct SearchActivityRecommendData {
    /// Hot-search coin tokens (same shape as the legacy /recommend flat array)
    pub tokens: Vec<SearchEntryRecommendPo>,
    /// Campaign / activity items
    pub activity: Vec<SearchActivityRecommendPo>,
    /// Topic (content-trending) hints (NEW). `serde(default)` keeps older
    /// responses missing the key parseable (NFR-004 forward-compat).
    pub content_trendings: Vec<SearchContentTrendingPo>,
}

/// Activity (campaign) recommend item from the activity-recommend API.
/// 
/// API: GET /priapi/v5/rubik/search/activity-recommend (data.activity[])
/// Exported `uniffi::Record` — crosses FFI inside `SearchActivityRecommendData`;
/// the native side maps it into its own UI item type.
#[derive(Debug, Clone, PartialEq)]
pub struct SearchActivityRecommendPo {
    /// Icon image URL (empty => client-side fallback icon)
    pub url: String,
    /// Display copywriting text (shown in search bar)
    pub desc: String,
    /// Deeplink to the campaign landing page (e.g. "okx://...")
    pub deeplink: String,
    /// Campaign key/name (e.g. "world_cup_2026_outcome_campaign")
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SearchAnnouncementItem {
    pub title: String,
    pub publish_date: i64,
    pub link: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SearchBotsItem {
    pub apy: String,
    pub biz_type: String,
    pub inst_name: String,
    pub strategy_type: String,
    pub icon: String,
    pub display_id: String,
    /// Formatted instrument name (e.g., "BTC/USDT")
    pub formatted_inst_name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SearchCampaignItem {
    pub activity_id: i32,
    pub title: String,
    pub open_date: i64,
    pub campaign_status: String,
    pub icon: String,
    pub link: String,
}

/// Search Content Feed Item (v6.116.0+)
/// Corresponds to SearchResultContentFeedVo in Android
#[derive(Debug, Clone, PartialEq)]
pub struct SearchContentFeedVo {
    /// Author name
    pub author_name: String,
    /// Author avatar URL
    pub author_avatar_url: String,
    /// Raw publish time in milliseconds (formatted on client side for localization)
    pub publish_time: i64,
    /// Content ID
    pub content_id: String,
    /// Content text (use translated_content if available)
    pub content: String,
    /// Show news tag (true if not from twitter/okx)
    pub show_news_tag: bool,
    /// Like count
    pub like_count: i32,
    /// View count
    pub view_count: i32,
    /// Source platform (e.g., "twitter", "ok")
    pub source_platform: String,
    /// Raw JSON data for passing to client (optional)
    pub data_str: String,
    /// Whether the author is officially verified
    pub is_official: bool,
}

/// Search Content KOL Item (v6.116.0+)
/// Corresponds to SearchResultContentKolVo in Android
#[derive(Debug, Clone, PartialEq)]
pub struct SearchContentKolVo {
    /// KOL ID
    pub kol_id: String,
    /// KOL name
    pub kol_name: String,
    /// KOL avatar URL
    pub kol_avatar_url: String,
    /// KOL bio/description (use translated_bio if available)
    pub bio: String,
    /// Raw follower count
    pub follower_count: i32,
    /// Raw JSON data for passing to client (optional)
    pub data_str: String,
    /// Whether the KOL is officially verified
    pub is_official: bool,
    /// KOL certification badge (Orbit KOL identifier); None when backend omits it
    pub cert_badge: Option<CertBadgeVo>,
}

/// Topic (content-trending) recommend item from the activity-recommend API.
/// 
/// API: GET /priapi/v5/rubik/search/activity-recommend (data.contentTrendings[])
/// Carried into Global Search as the current search-bar hint (native maps it to
/// its own `SearchRecommendVo.Topic`). Exported `uniffi::Record` — crosses FFI
/// nested inside `SearchActivityRecommendData`.
/// 
/// Scope: only PRD-scoped fields (title / deeplink / icon). No tracking fields.
#[derive(Debug, Clone, PartialEq)]
pub struct SearchContentTrendingPo {
    /// Interface-provided title shown in the search-bar hint.
    pub title: String,
    /// Topic deeplink; a BLANK deeplink means the Topic is excluded (native).
    pub deeplink: String,
    /// Remote icon URL; empty/absent => native falls back to the carousel fire icon.
    pub icon: Option<String>,
}

/// Search Content Vo (v6.116.0+)
/// Corresponds to SearchResultContentVo in Android
#[derive(Debug, Clone, PartialEq)]
pub struct SearchContentVo {
    /// Raw JSON data (for passing to Lua/WebView)
    pub raw_po_json: String,
    /// Feed item list
    pub feed_vo_list: Vec<SearchContentFeedVo>,
    /// KOL item list
    pub kol_vo_list: Vec<SearchContentKolVo>,
    /// ALL tab Overview: pre-computed KOL subset (People first)
    pub overview_kol_list: Vec<SearchContentKolVo>,
    /// ALL tab Overview: pre-computed Feed subset
    pub overview_feed_list: Vec<SearchContentFeedVo>,
    /// Whether the content PO contains queryInfo (needed for Content tab navigation)
    pub has_query_info: bool,
    /// ALL tab Overview: whether total count exceeds the overview limit AND queryInfo exists
    pub has_more_data: bool,
}

/// Search DEX Item
#[derive(Debug, Clone, PartialEq)]
pub struct SearchDexItem {
    pub chain_id: String,
    pub chain_name: String,
    pub chain_logo: String,
    pub token_symbol: String,
    pub token_name: String,
    pub token_contract_address: String,
    pub token_logo: String,
    pub price: String,
    pub price_change_24h: String,
    pub price_change_4h: String,
    pub price_change_1h: String,
    pub price_change_utc0: String,
    pub price_change_utc8: String,
    pub liquidity: String,
    pub turn_over_24h: String,
    pub market_cap: String,
    pub community_recognized: bool,
    pub risk_level: String,
    pub cedefi_support: bool,
    pub verified: bool,
    pub boosted: bool,
    pub boost_percentage: String,
    pub boost_rate: String,
    /// Per-coin RWA flag (real-world asset). None preserves an unknown/omitted backend
    /// value, Some(false) means explicitly non-RWA, and Some(true) means RWA.
    pub rwa: Option<bool>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SearchEarnItem {
    pub currency_id: i32,
    pub currency_name: String,
    pub display_currency_name: String,
    pub currency_full_name: String,
    pub currency_icon: String,
    pub apy: String,
    /// Raw Rate.rateNum.value list (for client-side localized formatting)
    pub rate_values: Vec<String>,
    /// Rate.rateNum.type value ("0" or "1")
    pub rate_type: String,
    pub need_regional_display: bool,
    pub subscription_url: String,
    pub product_type_name: String,
}

/// Search entry recommend item response from API
/// 
/// Represents a single item in the search bar carousel.
/// API: GET /priapi/v5/rubik/search/recommend
/// 
/// This corresponds to the Kotlin `SearchRecommendPo` data class.
#[derive(Debug, Clone, PartialEq)]
pub struct SearchEntryRecommendPo {
    /// Display copywriting text (shown in search bar)
    pub copywriting: String,
    /// Instrument ID (e.g., "BTC-USDT")
    pub inst_id: String,
    /// Instrument type (e.g., "SPOT", "SWAP")
    pub inst_type: String,
    /// Recommend type (e.g., "coin", "trader", "strategy")
    pub recommend_type: String,
    /// Currency code (e.g., "BTC")
    pub ccy: String,
}

/// State for SearchEntryRecommendInteractor
#[derive(Debug, Clone, PartialEq)]
pub struct SearchEntryRecommendState {
    /// Raw {tokens, activity} envelope from the activity-recommend endpoint.
    /// The native side maps it into its own item types and owns the merge/order/index.
    pub activity_data: SearchActivityRecommendData,
    pub error: Option<String>,
}

/// Search Function Item (global-search "Functions" category)
/// 
/// A single function result row shared by the ALL-tab overview group and the
/// dedicated Functions sub-page. Flat FFI-safe fields only. Only the 5 fields
/// the client consumes are modeled; the web-only `webUrl` is intentionally not
/// carried on mobile.
#[derive(Debug, Clone, PartialEq)]
pub struct SearchFunctionItem {
    pub name: String,
    pub icon_light: String,
    pub icon_dark: String,
    pub app_deeplink: String,
    pub feature_restriction: String,
}

/// State for SearchHistoryInteractor
#[derive(Debug, Clone, PartialEq)]
pub struct SearchHistoryState {
    pub history_list: Vec<SearchRecord>,
    pub error: Option<String>,
}

/// State for SearchHotInteractor
#[derive(Debug, Clone, PartialEq)]
pub struct SearchHotState {
    pub hot_list: Vec<QuotationSpotItemVo>,
    pub is_loading: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SearchP2pItem {
    pub currency: String,
    pub currency_id: String,
    pub asset_level: i32,
    pub support_p2p: String,
    pub deposit_withdrawable: bool,
    pub deposit_rechargeable: bool,
}

/// Lightweight search record for persistence
/// 
/// Contains only essential fields for:
/// 1. UI display (`display_name`, `subtitle`, `icon_url`)
/// 2. Deduplication (`unique_key`)
/// 3. Navigation (`navigation_params`)
#[derive(Debug, Clone, PartialEq)]
pub struct SearchRecord {
    /// Unique key for deduplication (e.g., "spot_BTC-USDT")
    pub unique_key: String,
    /// Display name for UI
    pub display_name: String,
    /// Subtitle for UI (optional secondary info)
    pub subtitle: Option<String>,
    /// Icon URL for UI
    pub icon_url: Option<String>,
    /// Record type for categorization
    pub record_type: SearchRecordType,
    /// Search/click timestamp (Unix timestamp in milliseconds)
    pub timestamp: i64,
    /// Search entry point where this record was created
    pub source: Option<SearchEntry>,
    /// Navigation parameters for routing
    pub navigation_params: NavigationParams,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SearchTraderItem {
    pub income_rate: String,
    pub jump_param: String,
    pub nickname_cn: String,
    pub nickname_en: String,
    pub head_picture: String,
    pub default_head_picture_id: String,
    pub trader_type: String,
    pub aum: String,
    /// Whether this is a trader (type != "5" && type != "6")
    pub is_trader: bool,
}

/// Represents a user-selected date for a specific base currency in the ABOVE events UI.
/// 
/// Rust state holds a Vec<SelectedDateVo> as the single source of truth for which date
/// is currently active per base currency. Native (Android/iOS) consumers read this from
/// state and display the corresponding chip as selected; they dispatch
/// `select_date_for_currency` action when the user taps a different date chip.
/// 
/// `date_label_ts` is a UTC day-start timestamp in milliseconds.
#[derive(Debug, Clone, PartialEq)]
pub struct SelectedDateVo {
    pub base_currency: String,
    pub date_label_ts: i64,
    /// Optional method (as `EventContractMethod::as_str()`, e.g. "HIT" / "PRICE_ABOVE")
    /// and freq (e.g. "DAILY" / "MONTHLY") that scope the selection to ONE above-like
    /// card. When both are `Some`, the selection applies only to the matching
    /// (ccy, method, freq) group (each card remembers its own date independently).
    /// When both are `None` (legacy `select_date_for_currency` path), the selection
    /// is a currency-wide fallback shared by that currency's cards.
    pub method: Option<String>,
    pub freq: Option<String>,
}

/// Represents a selected instrument for comparison
#[derive(Debug, Clone, PartialEq)]
pub struct SelectedInstrument {
    /// Instrument ID (e.g., "BTC-USDT", "BTC-USDT-SWAP")
    pub inst_id: String,
    /// Instrument type (SPOT, SWAP, FUTURES, etc.)
    pub inst_type: String,
    /// Display name
    pub display_name: String,
}

/// Configuration for selection behavior (UniFFI-compatible)
#[derive(Debug, Clone, PartialEq)]
pub struct SelectionConfigVo {
    /// Selection mode
    pub selection_mode: SelectionModeVo,
    /// Supported instrument types for filtering
    pub supported_inst_types: Vec<InstTypeVo>,
    /// Minimum number of instruments required (0 = no minimum)
    pub min_selection: u32,
    /// Maximum number of instruments allowed (0 = unlimited)
    pub max_selection: u32,
    /// Policy for pre-selected instruments
    pub pre_selected_policy: PreSelectedPolicyVo,
    /// Whether to validate selection on toggle
    pub validate_on_toggle: bool,
    /// Allowed instrument IDs (empty = allow all)
    pub allowed_instrument_ids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SelectionNavSearchFavoritesState {
    pub keyword: Option<String>,
    pub results: Vec<QuotationFavoriteVo>,
    pub is_searching: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SelectionNavSearchFuturesGroupState {
    pub page_list: Vec<FuturesPageVo>,
    pub total_count: u32,
    pub is_loading_groups: bool,
    pub groups_error: Option<String>,
    pub keyword: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SelectionNavSearchFuturesListState {
    pub futures_type: Option<FuturesType>,
    pub category_group: Option<CategoryGroupVo>,
    pub unit: Option<FuturesUnitVo>,
    pub results: Vec<QuotationFuturesItemVo>,
    pub is_searching: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SelectionNavSearchSelectionState {
    pub selected_instruments: Vec<QuotationFavoriteVo>,
    pub selected_count: u32,
    pub can_save: bool,
    pub last_validation: Option<SelectionValidationResultVo>,
    pub last_toggle_result: Option<SelectionToggleResultVo>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SelectionNavSearchSpotGroupState {
    pub page_list: Vec<SpotPageVo>,
    pub total_count: u32,
    pub is_loading_groups: bool,
    pub groups_error: Option<String>,
    pub keyword: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SelectionNavSearchSpotListState {
    pub category_group: Option<CategoryGroupVo>,
    pub charge_group: Option<ChargeGroupVo>,
    pub results: Vec<QuotationSpotItemVo>,
    pub is_searching: bool,
    pub error: Option<String>,
}

/// Result of toggle selection operation (UniFFI-compatible)
/// Named differently from old interactor's ToggleResultVo to allow coexistence.
#[derive(Debug, Clone, PartialEq)]
pub struct SelectionToggleResultVo {
    pub is_selected: bool,
    pub validation: SelectionValidationResultVo,
    pub should_dismiss: bool,
}

/// Distinct (method, freq, base_currency) tuple identifying one event series.
/// 
/// Produced from the HTTP inst-list response order so the Android/iOS UI can
/// render Event Tab + NavSearch cards in HTTP-driven sequence (cf. §2.2.1.5
/// 6.9 batch). WS pushes only update vol/probability of existing rows — they
/// never reorder, since reorder would fight this HTTP-driven invariant.
#[derive(Debug, Clone, PartialEq)]
pub struct SeriesKeyVo {
    pub method: EventContractMethod,
    /// Frequency token, e.g. "FIVE_MIN", "FIFTEEN_MIN", "HOURLY", "DAILY".
    pub freq: String,
    /// Base currency derived from `series_id`, e.g. "BTC".
    pub base_currency: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SocialMediaVo {
    pub name: String,
    pub url: String,
}

/// Raw inputs for a Spot list item (native reads them from `QuotationSpotItemVo`).
#[derive(Debug, Clone, PartialEq)]
pub struct SpotItemFormatInput {
    /// Raw change ratio decimal (e.g. "0.0345"). `None` -> empty change recipe.
    pub change_ratio: Option<String>,
    /// Instrument price. `None` when native's priceValue is null or 0.
    pub price: Option<String>,
    /// Raw 24h high price, formatted with the same instrument precision. `None` -> no recipe.
    pub high_24h: Option<String>, // default = None
    /// Raw 24h low price, formatted with the same instrument precision. `None` -> no recipe.
    pub low_24h: Option<String>, // default = None
    /// Instrument family (e.g. "BTC-USDT") for precision/sign resolution.
    pub inst_family: String,
    /// USD sub-price to convert to the user's fiat. `None` -> no sub-price recipe.
    pub sub_price_usd: Option<String>,
    /// 24h volume (base-coin). `None` -> no volume recipe.
    pub volume: Option<String>,
    /// USD turnover to abbreviate into fiat. `None` -> no turnover recipe.
    pub turnover_usd: Option<String>,
    /// USD market cap to abbreviate into a fiat recipe; None -> no recipe.
    pub market_cap_usd: Option<String>, // default = None
}

/// Stock rank item response from API
/// 
/// Represents a single item in the stock rank list.
#[derive(Debug, Clone, PartialEq)]
pub struct StockRankItemPo {
    /// Turnover
    pub turn_over: String,
    /// Volume
    pub volume: String,
    /// Display ID - display trading pair name (e.g., "BTC-USDT")
    pub display_id: String,
    /// Icon URL
    pub icon: String,
    /// Change percentage in 24 hours
    pub change_per_day24h: String,
    /// Change percentage at UTC 0
    pub change_per_day_utc0: String,
    /// Change percentage at UTC 8
    pub change_per_day_utc8: String,
    /// Instrument ID (e.g., "BTC-USDT")
    pub inst_id: String,
    /// Instrument type (e.g., "SPOT", "SWAP")
    pub inst_type: String,
    /// Last price
    pub last_price: String,
    /// USD price computed from last_price x cup ticker rate
    pub last_usd_price: String,
    /// Commodity tag (e.g., "Circle", "Tesla") — company name for stock-based tokens
    pub commodity_tag: String,
    /// Full company or asset name used when the shorter commodity tag is absent.
    pub full_name: String,
    /// Pre-IPO tag — true when the home TradFi item is a Pre-IPO asset.
    /// Drives the "Pre-IPO" badge on the home TradFi list (HTTP first-screen and WS share
    /// this struct, so one field covers both). Missing / null / abnormal degrades to false
    /// with no list or WS impact: `default` covers the missing key, while the null guard
    /// covers an explicit `"preIpoTag": null` (a bare `bool` would otherwise reject null).
    pub pre_ipo_tag: bool,
    /// Whether the instrument has a recently published earnings event.
    /// The HTTP snapshot owns this flag; later whole-list WS frames reuse it.
    pub has_recent_earnings: bool, // default = false
    /// V2 lifecycle state for online items. Legacy payloads omit this field.
    pub state: String,
    /// V2 open method. The server may encode this as either a number or a string;
    /// normalize both forms to the string wire representation used by the client.
    pub open_method: String,
    /// V2 auction-to-trading transition timestamp (epoch milliseconds).
    pub auction_finish_ts: String,
    /// V2 remaining auction-to-trading seconds.
    pub wait_auction_finish_time: String,
    /// Rust-computed lifecycle used by the V2 UI. It is not a wire field.
    pub lifecycle_state: StockLifecycleState,
}

/// Stock rank list response from API
/// 
/// Wrapper for API response containing spot and futures rank lists, plus the
/// pre-listing (wait-online) spot/futures groups (NEW, TEPT-6664).
/// The WebSocket sends `{"arg":{...}, "data":[{"spot":[], "futures":[], ...}]}` format.
#[derive(Debug, Clone, PartialEq)]
pub struct StockRankListPo {
    /// Pre-listing (wait-online) spot items — lifecycle-bearing (NEW). Defaults to []
    /// when empty / absent / feature-flag-off (NFR-003).
    pub wait_online_spot: Vec<StockRankWaitOnlineItemPo>,
    /// Spot rank items (TRADING) — UNCHANGED (FR-007)
    pub spot: Vec<StockRankItemPo>,
    /// Pre-listing (wait-online) futures items — lifecycle-bearing (NEW). Defaults to []
    /// when empty / absent / feature-flag-off (NFR-003).
    pub wait_online_futures: Vec<StockRankWaitOnlineItemPo>,
    /// Futures rank items (TRADING) — UNCHANGED (FR-007)
    pub futures: Vec<StockRankItemPo>,
}

/// Pre-listing (wait-online) stock rank item — NEW (TEPT-6664).
/// 
/// Only PRD/contract-scoped fields are modeled (no "for completeness" fields);
/// serde `default` silently drops unknown JSON keys and back-fills missing ones,
/// so missing/empty arrays and missing item fields never panic (NFR-003).
/// 
/// Wire semantics:
/// - `open_method`: "0"=call auction, "1"=normal pricing, "2"=pre-order. Kept as
/// String (a numeric-null would otherwise be mis-serialized to 0).
/// - `state`: `COUNT_DOWN` or `STAY_TUNED` in the V2 wait-online arrays.
/// - `r#type`: "0"=concrete countdown.
/// - `trade_ready`: "0"=not ready, "1"=ready. Kept as String to avoid numeric-null drift.
/// 
/// Lifecycle/pre-order are computed in Rust (see `derive_lifecycle`) and carried in
/// the `skip_deserializing` fields; native never re-derives them (FR-012).
#[derive(Debug, Clone, PartialEq)]
pub struct StockRankWaitOnlineItemPo {
    /// Instrument ID (e.g., "OPENAI-USDT")
    pub inst_id: String,
    /// Instrument type (e.g., "SPOT", "SWAP")
    pub inst_type: String,
    /// Icon URL
    pub icon: String,
    /// Display ID - display trading pair name (e.g., "OPENAI-USDT")
    pub display_id: String,
    /// Display name — rendered verbatim; no forced uppercase (FR-011).
    pub display_name: String,
    /// Full name
    pub full_name: String,
    /// Commodity tag (company name for stock-based tokens)
    pub commodity_tag: String,
    /// Market cap (arbitrary precision, kept as String)
    pub market_cap: String,
    /// Pre-IPO tag — true when the item is a Pre-IPO asset. Missing/null/abnormal
    /// degrades to false (null guard mirrors StockRankItemPo::pre_ipo_tag).
    pub pre_ipo_tag: bool,
    /// Non-tradable flag — true when the item cannot yet be traded. Missing/null/
    /// abnormal degrades to false (same null guard as `pre_ipo_tag`).
    pub non_tradable: bool,
    /// Last price
    pub last_price: String,
    /// Change percentage in 24 hours
    pub change_per_day24h: String,
    /// Change percentage at UTC 0
    pub change_per_day_utc0: String,
    /// Change percentage at UTC 8
    pub change_per_day_utc8: String,
    /// Volume
    pub volume: String,
    /// Turnover
    pub turn_over: String,
    /// Online timestamp (epoch milliseconds, as String)
    pub online_ts: String,
    /// Remaining online-countdown seconds; empty outside countdown.
    pub wait_online_time: String,
    /// Type — "0"=concrete countdown, "1"=stay tuned.
    pub type: String,
    /// Trade ready flag — "0"=not ready, "1"=ready. String to avoid numeric-null drift.
    pub trade_ready: String,
    /// Open method — "0"=call auction, "1"=normal pricing, "2"=pre-order. String wire.
    pub open_method: String,
    /// Lifecycle state string from the wire (`COUNT_DOWN` / `STAY_TUNED`).
    pub state: String,
    /// Auction finish timestamp (epoch milliseconds, as String)
    pub auction_finish_ts: String,
    /// Remaining auction-finish countdown seconds
    pub wait_auction_finish_time: String,
    /// Rule type
    pub rule_type: String,
    /// Typed lifecycle bucket (StayTuned / Countdown / CallAuction / PreQuote / Trading / Unknown).
    pub lifecycle_state: StockLifecycleState,
    /// True when open_method == "2" AND trade_ready == "1" — row routes into trading (FR-006).
    pub pre_order_eligible: bool,
    /// Display base coin name derived from display_id (falls back to inst_id base).
    /// Display-only; inst_id remains the raw key.
    pub display_coin_name: String,
    /// USD price computed from last_price x cup ticker rate.
    pub last_usd_price: String,
}

/// Token unlock event data.
/// 
/// Describes a scheduled token unlock for a specific currency.
#[derive(Debug, Clone, PartialEq)]
pub struct TokenUnlockDataPo {
    /// Icon URL for the token
    pub icon: String,
    /// Token name / description
    pub desc: String,
    /// Unlock timestamp (13-digit milliseconds, may arrive as number or string)
    pub time_stamp: String,
    /// Currency code (e.g., "BTC")
    pub ccy: String,
    /// Display ID for UI rendering (may be lowercase/custom); may be empty/absent when the coin
    /// is unconfigured. Display-only — `ccy` stays the underlying currency key.
    pub display_id: Option<String>,
    /// Volume unlocked in this event
    pub unlocked_volume: String,
    /// Percentage of total supply unlocked
    pub unlocked_percent: String,
    /// Next scheduled unlock amount
    pub next_unlocked_amount: String,
    /// Next scheduled unlock percentage
    pub next_unlocked_percent: String,
    /// Ratio of locked to unlocked amount
    pub locked_divide_unlocked_amount: String,
    /// Ratio of locked to unlocked percentage
    pub locked_divide_unlocked_percent: String,
}

/// One L2 tab of the TradFi business line.
/// 
/// Every tab carries the ready-to-use filter for BOTH business lines, because
/// which line is on screen is a page-level choice (see the page interactor's
/// `has_spot_tab` / `has_futures_tab`) rather than a per-tab one. The host never
/// builds a filter itself.
/// 
/// The category this tab filters by is NOT exposed on its own: `key` and `title`
/// already carry its identity and label, and the filter itself lives inside the
/// two page Vos (each `*PageVo::Category` holds the group and resolves to a
/// complete filter). A third copy would only invite the host to build its own
/// filter from it.
/// 
/// # How the host arranges these
/// 
/// The list is the category dimension, in cloud order, already narrowed by the
/// category data gate. Exactly one entry means the category dimension has
/// collapsed, so the visible tab strip is the business line (Spot / Futures)
/// with that single category applied to both lists. More than one entry means
/// the categories are the strip and the business line moves into a dropdown.
/// An empty list means there is nothing to render.
#[derive(Debug, Clone, PartialEq)]
pub struct TradFiL2TabVo {
    /// Stable tab identity, namespaced as `tradfi:{group_id}` (see `from_category` below —
    /// no underscore; a native consumer parsing this prefix must match the actual format).
    /// 
    /// Deliberately NOT `FinanceCategoryTabVo`: that type is the identity of an
    /// L1 tab everywhere it appears (cloud L1 config, Discover / Nav Search /
    /// Explore L1 pages, the L1 local fallback), and its key is `category:{id}`.
    /// The TradFi business line and a Finance Category L1 page can be delivered
    /// at the same time, so reusing that key would give two tabs at DIFFERENT
    /// levels the same identity — which the host uses for selection state and
    /// analytics.
    pub key: String,
    /// Display name, from the category the server delivered.
    pub title: String,
    /// Spot list filter for this tab. No charge group is applied, matching the
    /// Category pages (no implicit quote-currency narrowing).
    pub spot_page: SpotPageVo,
    /// Futures list filter for this tab. No unit is applied, for the same reason.
    pub futures_page: FuturesPageVo,
}

/// Trade Group Data (from external trading API)
/// 
/// Corresponds to `com.okinc.tradeapi.bean.TradeGroupData`
#[derive(Debug, Clone, PartialEq)]
pub struct TradeGroupData {
    /// Group ID (e.g., "1" for mainstream, "41" for new coins)
    pub id: i32,
    /// Display name
    pub name: String,
    /// Group type
    pub group_type: String,
    /// Mark/label
    pub is_new: i32,
    /// Whether this category should appear as a Tab in Market & Navigation entries (BOSS-driven).
    pub show_in_market_nav: bool,
    /// Whether this category should appear in the local L1 fallback.
    pub show_in_l1: bool, // default = false
    /// Whether this category should appear in the Nav Search coin-mode fallback.
    pub show_in_coin_l2: bool, // default = false
    /// Whether this category should appear on the Spot business line (BOSS-driven).
    /// Opt-out, unlike [Self::show_in_market_nav]: absent means visible, so the gate only
    /// ever removes what BOSS explicitly turns off.
    pub show_in_spot: bool, // default = true
    /// Whether this category should appear on the Futures business line (BOSS-driven).
    /// Opt-out, same contract as [Self::show_in_spot].
    pub show_in_futures: bool, // default = true
}

#[derive(Debug, Clone, PartialEq)]
pub struct TradeProductItemVo {
    pub inst_id: String,
    pub inst_type: String,
    /// Base currency display symbol from TradingCore (e.g. "xTSLA").
    /// Display-only; keep `inst_id` as the navigation / favorite key.
    pub display_symbol: String,
    /// Raw price string. For option items = index price from getIndexTickers.
    pub last: String,
    /// Format: "+2.34%" / "-1.20%" / "0.00%". Empty string for option items.
    pub change_pct: String,
    pub max_leverage: String,
    pub is_watching: bool,
    /// Server-provided `alias_name` from `TokenInfoOriginData`
    /// (e.g. `"this_week"` / `"next_week"` / `"quarter"` / `"260626"`).
    /// 
    /// Required for FUTURES favorite add/remove APIs which key on
    /// `(inst_family, alias)` rather than the raw delivery inst_id.
    /// Empty for non-FUTURES instrument types.
    pub alias: String,
}

/// Trade products grouped by instrument type.
/// Note: for `option` items, `TradeProductItemVo.last` = index price from getIndexTickers;
/// `change_pct` is empty string.
#[derive(Debug, Clone, PartialEq)]
pub struct TradeProductsVo {
    pub spot: Vec<TradeProductItemVo>,
    pub swap: Vec<TradeProductItemVo>,
    pub futures: Vec<TradeProductItemVo>,
    pub margin: Vec<TradeProductItemVo>,
    pub option: Vec<TradeProductItemVo>,
}

/// A single trading radar signal card.
/// 
/// Corresponds to iOS `OKMTradeRadarCardModel`.
/// 
/// Note: The server uses `changPercent` (missing 'e') as the JSON key.
/// Fields like `vol`, `turnover`, `buySell` are only populated for certain signal types.
#[derive(Debug, Clone, PartialEq)]
pub struct TradeRadarItemPo {
    /// Instrument ID (e.g., "BTC-USDT")
    pub inst_id: String,
    /// Instrument type (e.g., "SPOT")
    pub inst_type: String,
    /// Signal type: "whale" | "priceSwing" | "unlocks" | "traders"
    pub radar_type: String,
    /// Coin icon URL
    pub icon: String,
    /// Currency code (e.g., "BTC")
    pub ccy: String,
    /// Display ID for UI rendering (e.g., "xMU-USDT"); may be empty/absent when the coin is
    /// unconfigured. Display-only — `inst_id`/`ccy` stay the query/subscribe/deeplink keys.
    pub display_id: Option<String>,
    /// Trade price (whale signals)
    pub price: String,
    /// Trade volume (whale signals)
    pub vol: String,
    /// Trade turnover in USD (whale signals)
    pub turnover: String,
    /// Price change percentage (server-selected)
    pub change_percent: String,
    /// Price swing percentage value (priceSwing signals)
    pub ptc: String,
    /// Buy or sell direction: "bought" | "sold" (whale signals)
    pub buy_sell: String,
    /// Signal description text
    pub radar_desc: String,
    /// Signal timestamp (13-digit milliseconds, server may send as number or string)
    pub time_stamp: String,
    /// Current server timestamp (13-digit milliseconds, server may send as number or string)
    pub current_time_stamp: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TradeRadarState {
    /// Current filter type
    pub radar_type: TradeRadarType,
    /// Loaded radar items (accumulated across pages)
    pub items: Vec<TradeRadarItemPo>,
    /// Whether the initial load is in progress
    pub is_loading: bool,
    /// Whether a load-more is in progress
    pub is_load_more_loading: bool,
    /// Whether there are more pages to load
    pub has_more: bool,
    /// Current page number (1-based)
    pub page_num: u32,
    /// Error message from the last failed fetch
    pub error: Option<String>,
    /// Stable coin group filter (cached for load_more)
    pub group: String,
    /// Zone filter (cached for load_more)
    pub zone: String,
}

/// State for the trading calendar monthly list page.
/// 
/// `is_loading` defaults to `true` so that all platforms show a loading state
/// (not empty) before the first `load_month` call completes. This prevents
/// a brief "no data" flash on initial render.
#[derive(Debug, Clone, PartialEq)]
pub struct TradingCalendarListState {
    /// Year of the currently loaded month
    pub year: i32,
    /// Month number (1–12) of the currently loaded month
    pub month: u32,
    /// Current filter applied to the raw event list
    pub filter: CalendarFilterType,
    /// All events fetched from the network (unfiltered)
    pub raw_events: Vec<FinancialEventPo>,
    /// Events grouped by date after applying the current filter
    pub grouped_events: Vec<CalendarGroupedEvents>,
    /// Date picker items derived from grouped events
    pub date_items: Vec<CalendarDateItem>,
    /// Index of the currently selected date in `date_items`, or -1 if none
    pub selected_date_index: i32,
    /// Whether a network request is in progress (defaults to `true`)
    pub is_loading: bool,
    /// Error message if the last request failed
    pub error: Option<String>,
}

/// State for the trading calendar four-month sliding window.
#[derive(Debug, Clone, PartialEq)]
pub struct TradingCalendarMonthState {
    /// Start year of the anchor month for the four-month window
    pub start_year: i32,
    /// Start month (1–12) of the anchor month for the four-month window
    pub start_month: u32,
    /// All events fetched from the network for the four-month range
    pub events: Vec<FinancialEventPo>,
    /// Events grouped by date
    pub grouped_events: Vec<CalendarGroupedEvents>,
    /// Whether a network request is in progress
    pub is_loading: bool,
    /// Error message if the last request failed
    pub error: Option<String>,
}

/// State for the trading calendar overview card.
#[derive(Debug, Clone, PartialEq)]
pub struct TradingCalendarOverviewState {
    /// Compact Economic events retained for backward-compatible native consumers
    pub events: Vec<FinancialEventPo>,
    /// Compact Earnings groups projected by Rust in backend order
    pub earnings_groups: Vec<EarningsPreviewDatePo>,
    /// Whether a network request is in progress
    pub is_loading: bool,
    /// Error message if the last request failed
    pub error: Option<String>,
}

/// A coin icon item within a trending category.
/// 
/// Corresponds to iOS `OKMDTrendCategoryCoinModel`.
#[derive(Debug, Clone, PartialEq)]
pub struct TrendCategoryCoinPo {
    /// Coin icon URL
    pub icon: String,
    /// Coin symbol (e.g., "BTC")
    pub coin_name: String,
}

/// A single trending category item.
/// 
/// Corresponds to iOS `OKMDTrendCategoryListModel`.
#[derive(Debug, Clone, PartialEq)]
pub struct TrendCategoryItemPo {
    /// Category ID
    pub category_id: String,
    /// Category name (e.g., "DeFi", "Layer 1", "Meme")
    pub category_name: String,
    /// Category market cap volume
    pub market_cap_volume: String,
    /// Average price change percentage
    pub change_percent: String,
    /// Additional icon count beyond displayed coins
    /// "top" mode: total coins - 3; "all" mode: total coins - 4
    pub other_icon_size: String,
    /// Top coin icons in this category
    pub coin_list: Vec<TrendCategoryCoinPo>,
    /// Category price trend chart data (max 100 data points).
    /// API returns `[["timestamp","value"],...]`; this deserializer extracts
    /// the second element (value) from each pair as `f64`.
    pub category_chart: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TrendingCategoriesListState {
    /// "top" or "all"
    pub summary_type: String,
    /// Category items
    pub categories: Vec<TrendCategoryItemPo>,
    pub is_loading: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TrendingCategoryDetailState {
    /// Category ID
    pub category_id: String,
    /// Category detail data
    pub detail: Option<CategoryDetailPo>,
    /// Top movers (mixed up/down sorted)
    pub top_movers: Vec<UpDownSpotRank>,
    /// Full crypto coin list for the category (A5 addition: not present in iOS V1)
    pub crypto_list: Vec<UpDownSpotRank>,
    /// Current period filter for top movers and crypto list
    pub movers_period: TopMoversPeriod,
    /// Zone preference (cached for reload)
    pub zone: String,
    /// Stable coin preference (cached for reload)
    pub ccy_type: String,
    pub is_loading: bool,
    pub is_movers_loading: bool,
    pub is_crypto_loading: bool,
    pub error: Option<String>,
}

/// A single Trending Topics row. Rust owns the 1-based `index` and `is_hot`.
#[derive(Debug, Clone, PartialEq)]
pub struct TrendingTopicVo {
    /// 1-based position (Rust-assigned; native renders AS-IS, no re-derivation).
    pub index: i32,
    /// Interface-provided title (one line, tail-truncated on native).
    pub title: String,
    /// Per-item deeplink passed to the existing unified routing dispatcher.
    pub deeplink: String,
    /// Hot flag; `is_hot == (trendTag == "2")` — Rust-owned. Native shows the
    /// Hot pill iff true.
    pub is_hot: bool,
}

/// State for `TrendingTopicsInteractor`.
#[derive(Debug, Clone, PartialEq)]
pub struct TrendingTopicsState {
    /// Section visibility driver.
    pub section_state: TrendingTopicsSectionState,
    /// Ordered topics (Rust-owned order/count/index/is_hot; native renders AS-IS).
    pub topics: Vec<TrendingTopicVo>,
    /// Section-level (full-list) deeplink — opened only by the header arrow.
    pub list_deeplink: String,
    /// Error message for diagnostics (native still just hides the section).
    pub error: Option<String>,
}

/// Turnover rank item (spot).
/// 
/// Represents an instrument ranked by its trading volume/turnover.
/// Primary metric: `turn_over`.
/// 
/// API: GET /priapi/v5/rubik/app/public/turn-over-rank
#[derive(Debug, Clone, PartialEq)]
pub struct TurnoverRankItemPo {
    /// Instrument ID (e.g., "BTC-USDT")
    pub inst_id: String,
    /// Instrument type (e.g., "SPOT")
    pub inst_type: String,
    /// Trading turnover - primary metric for this ranking type
    pub turn_over: String,
    /// Latest price
    pub last_price: String,
    /// 24-hour price change percentage
    pub change_per_day24h: String,
    /// Price change percentage at UTC+0
    pub change_per_day_utc0: String,
    /// Price change percentage at UTC+8
    pub change_per_day_utc8: String,
    /// Icon URL (version 2)
    pub icon_v2: String,
    /// Trading volume (version 2)
    pub volume_v2: String,
    /// Full instrument name (version 2)
    pub full_name_v2: String,
    /// Price change percentage (version 2)
    pub change_per_v2: String,
    /// Currency code (version 2, e.g., "BTC")
    pub ccy_v2: String,
    /// Display ID - display trading pair name (e.g., "xSNDK-USDT")
    pub display_id: Option<String>,
    /// Whether the instrument has a recently published earnings event.
    pub has_recent_earnings: bool, // default = false
    /// USD price computed from last_price x cup ticker rate
    pub last_usd_price: String,
    /// Display base coin name derived from display_id (falls back to inst_id base).
    /// Display-only; ccy_v2/inst_id remain raw keys for ticker/sort/dedup.
    pub display_coin_name: String,
}

/// Allocation pie chart slice.
#[derive(Debug, Clone, PartialEq)]
pub struct UnlockAllocationItemVo {
    pub label: String,
    /// Percent of total supply allocated to this label (parsed to f32)
    pub perc_of_label: f32,
    /// Percent of this label that is unlocked (parsed to f32)
    pub perc_of_unlocked: f32,
    /// Progress ratio 0.0 .. 1.0 (parsed to f32)
    pub progress: f32,
    /// Color token name: "Category01" .. "Category05", "Extended06" .. "Extended18"
    /// Android maps this to actual @ColorInt using KLINE_UNLOCK_COLOR_LIST
    pub color_token: String,
    /// 0-based index in the allocation list (used for deterministic color lookup)
    pub color_index: i32,
}

/// Event allocation item.
#[derive(Debug, Clone, PartialEq)]
pub struct UnlockEventAllocationVo {
    pub label: String,
    pub perc_of_unlocked: String,
    pub unlocked: String,
}

/// Progress donut chart data.
/// Rust calculates the percent floats; Android formats amount strings.
#[derive(Debug, Clone, PartialEq)]
pub struct UnlockProgressVo {
    pub ccy: String,
    /// Raw amount strings (Android formats via formatICUCompact)
    pub unlocked_amount: String,
    pub locked_amount: String,
    pub untracked_amount: String,
    pub max_supply: String,
    pub mcap: String,
    /// Calculated: uses API's pre-calculated percentages when available.
    /// Falls back to (value / maxSupply * 100) if maxSupply > 0.
    pub unlocked_percent: f32,
    pub locked_percent: f32,
    pub untracked_percent: f32,
    pub notes: String,
    pub data_source_info: String,
}

/// Individual chart data point within a label group.
#[derive(Debug, Clone, PartialEq)]
pub struct UnlockScheduleChartItemVo {
    /// Timestamp string (milliseconds)
    pub ts: String,
    /// Stacked unlock amount (this label + all previous labels at same timestamp)
    pub unlock_amount: f64,
    /// Stacked fiat value (this label + all previous labels at same timestamp)
    pub fiat: f64,
    /// Raw (non-stacked) unlock amount for this label at this timestamp
    pub current_amount: f64,
    /// Raw (non-stacked) fiat value for this label at this timestamp
    pub current_fiat: f64,
    /// Index within the sorted time_list
    pub chart_index: i32,
    /// Index within the sorted label_key_list (descending by color provider index)
    pub label_index: i32,
}

/// Per-label group (used for stacked area chart rendering).
/// Each label has one entry per timestamp, with values STACKED (cumulative across labels).
#[derive(Debug, Clone, PartialEq)]
pub struct UnlockScheduleLabelGroupVo {
    pub label: String,
    pub color_token: String,
    pub color_index: i32,
    pub items: Vec<UnlockScheduleChartItemVo>,
}

/// Composite schedule result -- single atomic state update for all schedule data.
#[derive(Debug, Clone, PartialEq)]
pub struct UnlockScheduleResultVo {
    /// Per-timestamp raw data (for list display when user taps chart)
    pub ts_data: Vec<UnlockScheduleTsGroupVo>,
    /// Per-label stacked cumulative data (for chart rendering)
    pub chart_data: Vec<UnlockScheduleLabelGroupVo>,
    /// Sorted list of all timestamps (strings)
    pub time_list: Vec<String>,
    /// Position of "today" within the time range: (currentTime - min) / (max - min)
    /// May be negative (before range) or > 1.0 (after range)
    pub current_time_percent: f32,
    /// Timestamp of the default selected entry (nearest to today)
    pub default_select_ts: Option<String>,
}

/// Per-timestamp group (used for list display).
/// Values are NOT cumulative -- they are raw per-label deltas at this timestamp.
#[derive(Debug, Clone, PartialEq)]
pub struct UnlockScheduleTsGroupVo {
    /// Timestamp string (milliseconds)
    pub ts: String,
    pub near_today: bool,
    pub items: Vec<UnlockScheduleTsItemVo>,
}

/// Individual item within a timestamp group.
#[derive(Debug, Clone, PartialEq)]
pub struct UnlockScheduleTsItemVo {
    pub label: String,
    /// Raw unlock amount (not cumulative)
    pub unlock_amount: String,
    /// Raw fiat value (not cumulative)
    pub fiat: String,
    pub color_token: String,
    pub color_index: i32,
    /// Position in the sorted label_key_list (descending by color provider index).
    /// Used by Android's updateScheduleList to sort via `sortedByDescending { it.labelIndex }`.
    pub label_index: i32,
}

/// Stage card data (for upcoming unlock carousel).
/// All values are raw strings -- Android handles formatting + countdown.
#[derive(Debug, Clone, PartialEq)]
pub struct UnlockStageCardVo {
    pub ccy: String,
    /// Timestamp in milliseconds (string)
    pub unlock_timestamp_ms: String,
    /// Raw unlock amount string
    pub unlocked_amount: String,
    /// Raw percent of total unlocked (e.g., "5.23")
    pub unlocked_percent: String,
    /// Raw fiat value string
    pub unlocked_value: String,
    /// Raw percent of fiat value (e.g., "2.1")
    pub unlocked_value_percent: String,
}

/// Event data (for event list page).
#[derive(Debug, Clone, PartialEq)]
pub struct UnlockTokenEventVo {
    /// Timestamp in milliseconds (string)
    pub unlock_timestamp_ms: String,
    pub emission_type: String,
    /// Raw amount string
    pub unlocked_amount: String,
    /// Raw percent string (e.g., "5.23")
    pub unlocked_percent: String,
    /// Raw fiat value string
    pub unlocked_value: String,
    /// Raw percent of fiat value
    pub unlocked_value_percent: String,
    pub allocations: Vec<UnlockEventAllocationVo>,
}

/// Event list interactor state (separate page, single-load only).
#[derive(Debug, Clone, PartialEq)]
pub struct UnlockTokenEventsState {
    pub ccy: String,
    pub event_type: UnlockEventType,
    pub events: Vec<UnlockTokenEventVo>,
    pub is_loading: bool,
    pub error: Option<String>,
}

/// Main interactor state for the unlock tokens overview page.
#[derive(Debug, Clone, PartialEq)]
pub struct UnlockTokensState {
    /// Currency code (e.g., "BTC")
    pub ccy: String,
    pub progress: Option<UnlockProgressVo>,
    pub is_loading_progress: bool,
    pub progress_error: Option<String>,
    pub stage_cards: Vec<UnlockStageCardVo>,
    /// Index of the current stage card to display. -1 = none.
    pub current_stage_index: i32,
    pub is_loading_stages: bool,
    pub stages_error: Option<String>,
    pub allocation_items: Vec<UnlockAllocationItemVo>,
    pub allocation_max_supply: String,
    pub is_loading_allocation: bool,
    pub allocation_error: Option<String>,
    pub schedule_result: Option<UnlockScheduleResultVo>,
    pub schedule_period: UnlockSchedulePeriod,
    pub is_loading_schedule: bool,
    pub schedule_error: Option<String>,
}

/// Up/Down rank item response from API
/// 
/// Represents a single item in the up/down rank list.
/// This corresponds to the Kotlin `UpDownRank` data class.
#[derive(Debug, Clone, PartialEq)]
pub struct UpDownSpotRank {
    /// Icon URL (version 2)
    pub icon_v2: String,
    /// Full name (version 2)
    pub full_name_v2: String,
    /// Volume (version 2)
    pub volume_v2: String,
    /// Turnover (version 2)
    pub turn_over_v2: String,
    /// Last price
    pub last_price: String,
    /// Change percentage
    pub change_per: String,
    /// Currency code (version 2)
    pub ccy_v2: String,
    /// Instrument ID (e.g., "BTC-USDT")
    pub inst_id: String,
    /// Instrument type (e.g., "SPOT", "SWAP")
    pub inst_type: String,
    /// USD price computed from last_price x cup ticker rate
    pub last_usd_price: String,
    /// Display ID - display trading pair name (e.g., "BTC-USDC"); backend may return empty.
    pub display_id: Option<String>,
    /// Whether the instrument has a recently published earnings event.
    pub has_recent_earnings: bool, // default = false
    /// Display base coin name derived from display_id (falls back to inst_id base).
    /// Display-only; ccy_v2/inst_id remain the raw keys for ticker/sort/dedup.
    pub display_coin_name: String,
}

/// Inclusive index window of the list items currently visible on screen.
/// 
/// Indices refer to positions in the final displayed list (post filter/sort),
/// which map 1:1 to adapter positions on the consumer side.
#[derive(Debug, Clone, PartialEq)]
pub struct VisibleWindowVo {
    /// First visible item index (inclusive)
    pub first_index: u32,
    /// Last visible item index (inclusive)
    pub last_index: u32,
}

/// Wait online rank item response from API
/// 
/// Represents a single item in the wait online rank list (pending listing).
/// This corresponds to the Kotlin `WaitOnlineRank` data class.
#[derive(Debug, Clone, PartialEq)]
pub struct WaitOnlineRankPo {
    /// Icon URL
    pub icon: String,
    /// Online timestamp
    pub online_ts: String,
    /// Notice URL (Chinese)
    pub notice_url_cn: String,
    /// Notice URL (English)
    pub notice_url_en: String,
    /// Wait online time
    pub wait_online_time: String,
    /// Type
    pub type: String,
    /// Trade ready flag - 1 means ready, 0 means not ready
    pub trade_ready: i32,
    /// Display ID - display trading pair name (e.g., "BTC-USDT")
    pub display_id: Option<String>,
    /// Rule type - pre_market means pre-market trading; normal means regular trading
    pub rule_type: String,
    /// Instrument ID (e.g., "BTC-USDT")
    pub inst_id: String,
    /// Instrument type (e.g., "SPOT", "SWAP")
    pub inst_type: String,
    /// Whether the instrument has a recently published earnings event.
    pub has_recent_earnings: bool, // default = false
    /// Display base coin name derived from display_id (falls back to inst_id base).
    /// Display-only; inst_id remains the raw key for ticker/sort/dedup.
    pub display_coin_name: String,
}

// -----------------------------------------------------------------------------
// 3. Interfaces / Interactors / Services (139)
// -----------------------------------------------------------------------------

/// Interactor for the coin info overview page.
pub struct CiOverviewInteractor;
impl CiOverviewInteractor {
    /// Create a default instance backed by the production HTTP client.
    pub fn new_default() -> Arc<CiOverviewInteractor> { todo!() }
    /// Load coin info data.
    /// 
    /// Fetches both APIs in parallel, transforms data, and updates state.
    /// 
    /// # Arguments
    /// * `ccy` - Currency symbol (e.g. "BTC")
    /// * `is_lite_mode` - Whether the app is in Lite mode (hides token unlock)
    /// * `rank_jump_disabled` - Whether rank jump navigation is disabled
    pub fn load(&self, ccy: String, is_lite_mode: bool, rank_jump_disabled: bool) { todo!() }
    /// Load CertiK audit data from /priapi/v3/growth/price/currency-detail.
    /// 
    /// LEGACY — callers should not need this. `load` now resolves the canonical
    /// Price identity itself and fetches the audit record, so the UI supplies
    /// neither `symbol` nor `full_symbol_name`. The action is retained only
    /// because removing an exported UniFFI Action breaks the native ABI; it will
    /// go away once no native path calls it. Do NOT pass values derived from a
    /// display name — only canonical Price metadata is accepted, and anything
    /// else is rejected before dispatch.
    /// 
    /// This path is held to the same request gate as the automatic one: the
    /// parameters go through `CiAuditParams::validated`, so all three identity
    /// fields must be present and `slug` must agree with the other two. A caller
    /// that cannot supply a consistent set gets `missing_price_identity` and no
    /// request is sent.
    /// 
    /// Failure is non-fatal: `audit_info` stays None. Check
    /// `audit_unavailable_reason` to tell "no answer" apart from "no audit".
    /// 
    /// # Arguments
    /// * `symbol` - Lowercase coin symbol (e.g. "btc")
    /// * `full_symbol_name` - Price-page English URL name (e.g. "bitcoin")
    /// * `slug` - Price-page slug, required (e.g. "bitcoin-btc")
    /// * `fiat_currency` - User's current fiat currency (e.g. "USD")
    /// * `market_fiat` - Current market fiat (e.g. "USD")
    pub fn load_audit(&self, symbol: String, full_symbol_name: String, slug: String, fiat_currency: String, market_fiat: String) { todo!() }
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
    pub fn state(&self) -> CiOverviewState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

/// Compare Search Manager
/// 
/// Provides APIs for compare/multi-selection navigation search operations.
/// This manager wraps `SelectionNavSearchManager` with compare-specific configuration.
/// 
/// # Features
/// 
/// - Multi-selection with checkbox support (min 2, max 10 instruments)
/// - Filter out base comparison instrument (automatically filters first selected)
/// - Search across Favorites/Spot/Futures tabs simultaneously
/// - Validation: requires at least 2 instruments for comparison, maximum 10
/// 
/// # Example
/// 
/// ```ignore
/// use retail_trading::core::nav_search::compare_search_manager::{CompareSearchManager, SelectedInstrument};
/// 
/// let manager = CompareSearchManager::new();
/// 
/// // Initialize with pre-selected instruments (auto-filters first as base)
/// manager.initialize_for_compare(
/// vec![selected_instrument1, selected_instrument2],
/// ).await?;
/// 
/// // Search favorites
/// let stream = manager.search_favorites("BTC".to_string()).await?;
/// 
/// // Toggle selection
/// manager.toggle_selection(instrument).await?;
/// 
/// // Check if can save (requires at least 2 instruments)
/// let can_save = manager.can_save().await;
/// ```
pub struct CompareSearchManager;
impl CompareSearchManager {
    /// Create a new CompareSearchManager instance
    /// 
    /// Initializes with compare-specific configuration:
    /// - Multi-selection mode
    /// - Min 2 instruments required, max 10 instruments allowed
    /// - Supports Spot + Futures types
    /// - Pre-selected instruments are cancelable
    /// - Validates on toggle
    pub fn new() -> Arc<CompareSearchManager> { todo!() }
    /// Update selected instruments in batch using QuotationFavoriteVo
    /// 
    /// Replaces current selection with the provided set.
    pub async fn batch_update_selected_instruments(&self, instruments: Vec<QuotationFavoriteVo>) { todo!() }
    /// Check if save button should be enabled
    /// 
    /// Compare requires at least 2 instruments to make sense.
    /// 
    /// # Returns
    /// `bool` - true if at least 2 instruments are selected
    pub async fn can_save(&self) -> bool { todo!() }
    /// Get selected count
    pub async fn get_selected_count(&self) -> u64 { todo!() }
    /// Get current selected instruments with full quotation data
    /// 
    /// # Returns
    /// `Vec<QuotationFavoriteVo>` - List of currently selected instruments
    pub async fn get_selected_instruments(&self) -> Vec<QuotationFavoriteVo> { todo!() }
    /// Get current selected instruments as SelectedInstrument (legacy)
    /// 
    /// # Returns
    /// `Vec<SelectedInstrument>` - List of currently selected instruments
    pub async fn get_selected_instruments_legacy(&self) -> Vec<SelectedInstrument> { todo!() }
    /// Initialize for compare scenario using full quotation data
    /// 
    /// Automatically filters out the first instrument as the base comparison instrument.
    /// 
    /// # Arguments
    /// * `selected` - List of pre-selected instruments with full quotation data (first one will be filtered out)
    /// 
    /// # Example
    /// If selected = [BTC-USDT, ETH-USDT], then BTC-USDT will be filtered from search results
    pub async fn initialize_for_compare(&self, selected: Vec<QuotationFavoriteVo>) { todo!() }
    /// Initialize for compare scenario using legacy SelectedInstrument
    /// 
    /// Converts SelectedInstrument to QuotationFavoriteVo internally.
    /// 
    /// # Arguments
    /// * `selected` - List of pre-selected instruments (first one will be filtered out)
    pub async fn initialize_for_compare_legacy(&self, selected: Vec<SelectedInstrument>) { todo!() }
    /// Initialize with QuotationFavoriteVo instruments and filter list
    /// 
    /// # Arguments
    /// * `selected` - List of pre-selected instruments with full quotation data
    /// * `filter_out` - List of instrument IDs to filter out from search results
    pub async fn initialize_selected_instruments(&self, selected: Vec<QuotationFavoriteVo>, filter_out: Vec<String>) { todo!() }
    /// Legacy method for backward compatibility with SelectedInstrument
    /// 
    /// Note: This method converts SelectedInstrument to QuotationFavoriteVo internally.
    /// Prefer using `initialize_for_compare` or the QuotationFavoriteVo version for new code.
    /// 
    /// # Arguments
    /// * `selected` - List of pre-selected instruments (as SelectedInstrument)
    /// * `filter_out` - List of instrument IDs to filter out from search results
    pub async fn initialize_selected_instruments_legacy(&self, selected: Vec<SelectedInstrument>, filter_out: Vec<String>) { todo!() }
    /// Check if an instrument is selected by its ID
    pub async fn is_selected(&self, inst_id: String) -> bool { todo!() }
    /// Check if an instrument is selected using SelectedInstrument (legacy)
    pub async fn is_selected_legacy(&self, instrument: SelectedInstrument) -> bool { todo!() }
    /// Toggle instrument selection using QuotationFavoriteVo
    /// 
    /// Adds the instrument if not selected, removes if already selected.
    /// 
    /// # Returns
    /// `bool` - true if instrument is now selected, false if deselected
    pub async fn toggle_selection(&self, instrument: QuotationFavoriteVo) -> bool { todo!() }
    /// Toggle instrument selection using SelectedInstrument (legacy)
    /// 
    /// # Returns
    /// `bool` - true if instrument is now selected, false if deselected
    pub async fn toggle_selection_legacy(&self, instrument: SelectedInstrument) -> bool { todo!() }
}

/// CountingBannerInteractor - manages banner data subscription and refresh.
pub struct CountingBannerInteractor;
impl CountingBannerInteractor {
    /// Create a new CountingBannerInteractor with default dependencies.
    pub fn new_default() -> Arc<CountingBannerInteractor> { todo!() }
    /// Cancel the active banner subscription.
    pub fn cancel_subscription(&self) { todo!() }
    /// Query the call auction state for a specific instrument from the Rust-side cache.
    /// 
    /// Returns the state string (e.g., "CALL_AUCTION", "PRE_QUOTE", "TRADING")
    /// or `None` if not found in cache.
    /// 
    /// This is used by Android V2 to check call auction state at banner click time,
    /// replacing the V1 Android-side `CallAuctionWsManager.getCachedCoinState()`.
    pub fn get_call_auction_state(&self, inst_id: String) -> Option<String> { todo!() }
    /// One-shot refresh of banner data.
    pub fn refresh_banners(&self, group: String) { todo!() }
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
    pub fn state(&self) -> CountingBannerState { todo!() }
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
    /// Subscribe to banner data with persistent WS-triggered refresh.
    /// entity_id is obtained internally from FeatureRestrictionService.
    pub fn subscribe_banners(&self, group: String) { todo!() }
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

pub struct CryptoChartInteractor;
impl CryptoChartInteractor {
    pub fn new_default() -> Arc<CryptoChartInteractor> { todo!() }
    pub fn load_candles(&self, ccy: String, period: ChartPeriod) { todo!() }
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
    pub fn state(&self) -> CryptoChartState { todo!() }
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
    pub fn subscribe_tickers(&self, ccy: String) { todo!() }
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

pub struct CryptoDetailInteractor;
impl CryptoDetailInteractor {
    pub fn new_default() -> Arc<CryptoDetailInteractor> { todo!() }
    pub fn create_chart_interactor(&self) -> Arc<CryptoChartInteractor> { todo!() }
    pub fn create_earn_interactor(&self) -> Arc<CryptoEarnInteractor> { todo!() }
    pub fn create_profile_interactor(&self) -> Arc<CryptoProfileInteractor> { todo!() }
    pub fn create_trade_list_interactor(&self) -> Arc<CryptoTradeListInteractor> { todo!() }
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
    pub fn set_coin(&self, ccy: String, symbol: String) { todo!() }
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
    pub fn state(&self) -> CryptoDetailState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

pub struct CryptoEarnInteractor;
impl CryptoEarnInteractor {
    pub fn new_default() -> Arc<CryptoEarnInteractor> { todo!() }
    pub fn load(&self, currency_id: String) { todo!() }
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
    pub fn state(&self) -> CryptoEarnState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

pub struct CryptoProfileInteractor;
impl CryptoProfileInteractor {
    pub fn new_default() -> Arc<CryptoProfileInteractor> { todo!() }
    pub fn load(&self, ccy: String) { todo!() }
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
    pub fn state(&self) -> CryptoProfileState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

pub struct CryptoTradeListInteractor;
impl CryptoTradeListInteractor {
    pub fn new_default() -> Arc<CryptoTradeListInteractor> { todo!() }
    pub fn load(&self, ccy: String, ccy_type: String) { todo!() }
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
    pub fn set_ui_event_observer(&self, observer: Arc<dyn CryptoTradeListUxEventObserver>) { todo!() }
    /// Start the interactor lifecycle.
    /// 
    /// Calls the user's `InteractorLifecycle::start()` implementation.
    /// Override that method to add custom startup behavior (e.g., initial data loads).
    pub fn start(&self) { todo!() }
    /// Get the current state (UniFFI export).
    /// 
    /// Returns the External type (UniFFI-compatible).
    pub fn state(&self) -> CryptoTradeListState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
    /// Toggle favorite for a trade product.
    /// 
    /// `alias` is the server-provided `alias_name` from `TradeProductItemVo`
    /// (e.g. `"this_week"` / `"260626"`). Required for FUTURES; empty string
    /// for non-FUTURES instrument types.
    pub fn toggle_favorite(&self, inst_id: String, inst_type: String, alias: String, to_watch: bool) { todo!() }
    /// Opt-in async UI event stream (cycle-free). Single-subscriber.
    pub fn ui_event_stream(&self) -> Arc<CryptoTradeListUxEventStream> { todo!() }
}

/// Deprecated UniFFI callback interface for observing UI events. Prefer the cycle-free `CryptoTradeListUxEventStream` returned by `interactor.ui_event_stream()`.
pub struct CryptoTradeListUxEventObserver;
impl CryptoTradeListUxEventObserver {
    pub fn on_ui_event(&self, event: CryptoTradeListUxEvent) { todo!() }
}

/// Opt-in async event stream from `interactor.ui_event_stream()`.
/// Single-subscriber, latest-wins. `next()` resolves to `None` when the
/// interactor is dropped or a new subscription replaces this one.
pub struct CryptoTradeListUxEventStream;
impl CryptoTradeListUxEventStream {
    pub async fn next(&self) -> Option<CryptoTradeListUxEvent> { todo!() }
}

/// Interactor for the contract info (trading rules) page.
pub struct CtrContractInfoInteractor;
impl CtrContractInfoInteractor {
    /// Create with default manager (production HTTP clients).
    pub fn new_default() -> Arc<CtrContractInfoInteractor> { todo!() }
    /// Load contract info for the given instrument and biz type.
    /// 
    /// # Flow
    /// 1. Set is_loading = true, emit LoadEffect
    /// 2. Call manager.query_supplementary_data(inst_input, biz_type)
    /// 3. Call ContractInfoManager::build_field_list(biz_type, inst_input, data)
    /// 4. Set field_list, is_loading = false
    /// 
    /// If a previous load is in progress, it is cancelled first.
    pub fn load(&self, inst_input: CtrInstrumentInput, biz_type: CtrBizType) { todo!() }
    /// Load contract info by resolving instrument data from Rust cache.
    /// 
    /// # Flow
    /// 1. Call `ContractInfoManager::resolve_instrument_input(inst_id, inst_type)`
    /// 2. On success: proceed with `query_supplementary_data()` + `build_field_list()`
    /// 3. On error: set error state
    pub fn load_by_id(&self, inst_id: String, inst_type: String) { todo!() }
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
    pub fn state(&self) -> CtrContractInfoState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

pub struct DiscoverCategoryPageInteractor;
impl DiscoverCategoryPageInteractor {
    pub fn category(&self) -> FinanceCategoryTabVo { todo!() }
    pub fn create_futures_list_interactor(&self) -> Arc<DiscoverFuturesListInteractor> { todo!() }
    pub fn create_spot_list_interactor(&self) -> Arc<DiscoverSpotListInteractor> { todo!() }
    /// Candidate Futures page. [FuturesPageVo::resolve_filter] maps Category
    /// to FuturesType::All and no unit, so no hidden USDT-M narrowing occurs.
    pub fn futures_page(&self) -> FuturesPageVo { todo!() }
    pub fn has_futures_tab(&self) -> bool { todo!() }
    pub fn has_spot_tab(&self) -> bool { todo!() }
    /// Candidate Spot page. `has_spot_tab` is the Rust-owned visibility
    /// contract; this page only carries the Category filter and no charge group.
    pub fn spot_page(&self) -> SpotPageVo { todo!() }
    /// Release factory-owned child interactors. The page itself has no async
    /// work or mutable state, so teardown only establishes the child lifetime
    /// boundary used by the debug leak watchdog.
    pub fn teardown(&self) { todo!() }
}

pub struct DiscoverCryptoInteractor;
impl DiscoverCryptoInteractor {
    pub fn new_default() -> Arc<DiscoverCryptoInteractor> { todo!() }
    pub fn create_list_interactor(&self) -> Arc<DiscoverCryptoListInteractor> { todo!() }
    pub fn load_groups(&self, force_refresh: bool) { todo!() }
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
    pub fn state(&self) -> DiscoverCryptoState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

pub struct DiscoverCryptoListInteractor;
impl DiscoverCryptoListInteractor {
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
    pub fn state(&self) -> DiscoverCryptoListState { todo!() }
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
    pub fn subscribe_crypto_list(&self, subscription_id: u32, force_refresh: bool, sort_by: SortBy, sort_order: SortOrderEnum, visible_window: Option<VisibleWindowVo>) { todo!() }
    /// Subscribe to the crypto list stream as a cancellable async Future.
    /// 
    /// Cancellation is driven by an external `OKCancellationToken` — when the
    /// caller cancels the token, the underlying stream subscription is dropped
    /// and this future resolves. The caller owns the token's lifetime; the
    /// interactor does not store or cancel it internally.
    /// 
    /// The category is fixed to `ALL`. To subscribe with a specific category,
    /// use the Action-style `subscribe_crypto_list` method instead.
    pub async fn subscribe_crypto_list_future(&self, subscription_id: u32, force_refresh: bool, sort_by: SortBy, sort_order: SortOrderEnum, visible_window: Option<VisibleWindowVo>, cancel_token: Arc<OKCancellationToken>) -> Result<(), OKFFIError> { todo!() }
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

pub struct DiscoverDexInteractor;
impl DiscoverDexInteractor {
    pub fn new_default() -> Arc<DiscoverDexInteractor> { todo!() }
    pub fn create_list_interactor(&self) -> Arc<DiscoverDexListInteractor> { todo!() }
    pub fn load_groups(&self, force_refresh: bool) { todo!() }
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
    pub fn state(&self) -> DiscoverDexState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

pub struct DiscoverDexListInteractor;
impl DiscoverDexListInteractor {
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
    pub fn state(&self) -> DiscoverDexListState { todo!() }
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
    pub fn subscribe_dex_list(&self, subscription_id: u32, force_refresh: bool, category_group: DexChainGroupVo, period: DexPeriod, sort_by: DexSortBy, sort_order: SortOrderEnum) { todo!() }
    /// Subscribe to the DEX list stream as a cancellable async Future.
    /// 
    /// Cancellation is driven by an external `OKCancellationToken` — when the
    /// caller cancels the token, the underlying stream subscription is dropped
    /// and this future resolves. The caller owns the token's lifetime; the
    /// interactor does not store or cancel it internally.
    /// 
    /// Use this when the consumer (e.g. Kotlin `subscribeFutureV2`) needs to
    /// switch subscription parameters mid-flight — cancel the previous token,
    /// start a new call with a fresh token.
    pub async fn subscribe_dex_list_future(&self, subscription_id: u32, force_refresh: bool, category_group: DexChainGroupVo, period: DexPeriod, sort_by: DexSortBy, sort_order: SortOrderEnum, cancel_token: Arc<OKCancellationToken>) -> Result<(), OKFFIError> { todo!() }
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

pub struct DiscoverEventContractInteractor;
impl DiscoverEventContractInteractor {
    pub fn new_default() -> Arc<DiscoverEventContractInteractor> { todo!() }
    /// Create an Explore interactor that shares this interactor's Manager.
    /// 
    /// Reusing the same `DiscoverEventContractManager` avoids a duplicate HTTP
    /// call when both Discover and Explore pages need event contract data.
    pub fn create_explore_interactor(&self) -> Arc<ExploreEventContractInteractor> { todo!() }
    pub fn create_list_interactor(&self) -> Arc<DiscoverEventContractListInteractor> { todo!() }
    pub fn load_groups(&self, force_refresh: bool) { todo!() }
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
    pub fn state(&self) -> DiscoverEventContractGroupState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

pub struct DiscoverEventContractListInteractor;
impl DiscoverEventContractListInteractor {
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
    pub fn select_date_for_currency(&self, base_currency: String, date_label_ts: i64) { todo!() }
    /// Per-card date selection scoped to a single (base_currency, method, freq)
    /// series, so each above-like card (HIT-DAILY / HIT-MONTHLY / ABOVE / BETWEEN)
    /// remembers its own date independently. Prefer this over
    /// `select_date_for_currency` (which is a currency-wide fallback). `method` is
    /// taken as the `EventContractMethod` enum (not a raw string) to avoid the
    /// as_str vs enum-name ambiguity (e.g. `PriceHit` -> "HIT").
    pub fn select_date_for_series(&self, base_currency: String, method: EventContractMethod, freq: String, date_label_ts: i64) { todo!() }
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
    pub fn state(&self) -> DiscoverEventContractListState { todo!() }
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
    /// Subscribe to the event contract list, filtered by (method, freq, ccy).
    /// 
    /// `freq=None` means accept all frequencies (legacy behavior). The new
    /// `freq` parameter is inserted between `method` and `ccy` to match the
    /// natural narrowing order: method -> freq -> ccy.
    pub fn subscribe_event_contract_list(&self, subscription_id: u32, method: Option<EventContractMethod>, freq: Option<String>, ccy: Option<String>, force_refresh: bool) { todo!() }
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

pub struct DiscoverFuturesListInteractor;
impl DiscoverFuturesListInteractor {
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
    pub fn state(&self) -> DiscoverFuturesListState { todo!() }
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
    /// Subscribe (Action form): fire-and-forget; a new call cancels the
    /// previous via `SpawnPolicy::AlwaysCancel`. For cancellable one-shot
    /// (e.g. mid-flight switch), use the `_future` variant with an external
    /// `OKCancellationToken`.
    pub fn subscribe_futures_list(&self, subscription_id: u32, force_refresh: bool, page_vo: FuturesPageVo, sort_by: SortBy, sort_order: SortOrderEnum, tickers_period: Option<TickersPeriodEnum>, visible_window: Option<VisibleWindowVo>) { todo!() }
    /// Subscribe to the futures list stream as a cancellable async Future.
    /// 
    /// Cancellation is driven by an external `OKCancellationToken` — when the
    /// caller cancels the token, the underlying stream subscription is dropped
    /// and this future resolves. The caller owns the token's lifetime; the
    /// interactor does not store or cancel it internally.
    /// 
    /// `page_vo` (e.g. `FuturesPageVo::All { ... }`) encapsulates the business
    /// filter (category_group + futures_type + unit); the interactor resolves
    /// it internally.
    pub async fn subscribe_futures_list_future(&self, subscription_id: u32, force_refresh: bool, page_vo: FuturesPageVo, sort_by: SortBy, sort_order: SortOrderEnum, tickers_period: Option<TickersPeriodEnum>, visible_window: Option<VisibleWindowVo>, cancel_token: Arc<OKCancellationToken>) -> Result<(), OKFFIError> { todo!() }
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

pub struct DiscoverFuturesPageInteractor;
impl DiscoverFuturesPageInteractor {
    pub fn new_default() -> Arc<DiscoverFuturesPageInteractor> { todo!() }
    /// Create a futures list interactor that shares the same manager instance.
    /// 
    /// Page Interactor entry for the futures list sub-interactor.
    pub fn create_list_interactor(&self) -> Arc<DiscoverFuturesListInteractor> { todo!() }
    pub fn load_data(&self) { todo!() }
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
    pub fn state(&self) -> DiscoverFuturesPageState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

pub struct DiscoverMarketsRootInteractor;
impl DiscoverMarketsRootInteractor {
    pub fn new_default() -> Arc<DiscoverMarketsRootInteractor> { todo!() }
    /// Create one immutable Category page context. Fixed and Category pages
    /// in this Discover root session share the same lazy Spot/Futures source
    /// managers, while every Page/List Fragment keeps its own Interactor state.
    pub fn create_category_page_interactor(&self, category: FinanceCategoryTabVo) -> Arc<DiscoverCategoryPageInteractor> { todo!() }
    /// Create the fixed Futures page with the same Futures manager used by
    /// all Category Futures lists in this Discover root session.
    pub fn create_futures_page_interactor(&self) -> Arc<DiscoverFuturesPageInteractor> { todo!() }
    /// Create the fixed Spot page with the same Spot manager used by all
    /// Category Spot lists in this Discover root session.
    pub fn create_spot_page_interactor(&self) -> Arc<DiscoverSpotPageInteractor> { todo!() }
    /// Create the TradFi business-line page. It shares this root's market-tabs
    /// session and the same lazy Spot/Futures managers the fixed pages use, so
    /// its L2 strip is resolved from the identical cloud config snapshot.
    pub fn create_trad_fi_page_interactor(&self) -> Arc<DiscoverTradFiPageInteractor> { todo!() }
    /// Cache-first read: the gated L1 page list when the cloud config can
    /// answer WITHOUT waiting on the network, else null (the host then falls
    /// through to `load_markets_l1_tabs`). Lets the host render instantly and
    /// skip the whole loading pass on a warm cache.
    /// 
    /// NOT side-effect free: a hit also FIXES this page's snapshot — every L2
    /// in the same root then renders from the identical config — and schedules
    /// the one detached refresh whose result serves the NEXT page visit. Call
    /// it only when the result is actually going to be rendered.
    pub fn get_cached_markets_l1_tabs(&self) -> Option<Vec<DiscoverL1Page>> { todo!() }
    /// Resolve the L1 tab list (cloud-bounded, gate-filtered; local
    /// fallback on any degradation) and publish it into the state.
    pub fn load_markets_l1_tabs(&self) { todo!() }
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
    pub fn state(&self) -> DiscoverMarketsRootState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

pub struct DiscoverOptionsChainInteractor;
impl DiscoverOptionsChainInteractor {
    pub fn new_default() -> Arc<DiscoverOptionsChainInteractor> { todo!() }
    pub fn load_options_chains(&self) { todo!() }
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
    pub fn state(&self) -> DiscoverOptionsChainState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

pub struct DiscoverOptionsQuoteGroupInteractor;
impl DiscoverOptionsQuoteGroupInteractor {
    pub fn new_default() -> Arc<DiscoverOptionsQuoteGroupInteractor> { todo!() }
    pub fn create_list_interactor(&self) -> Arc<DiscoverOptionsQuoteListInteractor> { todo!() }
    pub fn load_groups(&self, force_refresh: bool) { todo!() }
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
    pub fn state(&self) -> DiscoverOptionsQuoteGroupState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

pub struct DiscoverOptionsQuoteListInteractor;
impl DiscoverOptionsQuoteListInteractor {
    /// Get all available strike prices for a specific category group and expiration time.
    /// 
    /// This method retrieves strike prices from cache, filtered by the given category and exp_time.
    /// - If `exp_time` is `Some(date)`, returns strike prices for that specific expiration date
    /// - If `exp_time` is `None`, returns all strike prices merged from all expiration dates
    /// 
    /// # Returns
    /// A sorted list of unique strike price strings (deduplicated and sorted)
    pub async fn get_stk_price_list_by_group_and_exp_time(&self, category_group: OptionsCategoryGroupVo, exp_time: Option<String>) -> Vec<String> { todo!() }
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
    pub fn state(&self) -> DiscoverOptionsQuoteListState { todo!() }
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
    /// Action form: fire-and-forget; a new call cancels the previous via
    /// `SpawnPolicy::AlwaysCancel`. For cancellable one-shot, use the
    /// `_future` variant with an external `OKCancellationToken`.
    pub fn subscribe_options_list(&self, subscription_id: u32, force_refresh: bool, category_group: OptionsCategoryGroupVo, exp_time: Option<String>, strike_price: Option<String>, direction: OptionsDirection, sort_by: SortBy, sort_order: SortOrderEnum) { todo!() }
    /// Subscribe to filtered and sorted options list stream using Future pattern.
    /// 
    /// Cancellable one-shot variant: the caller provides an `OKCancellationToken`
    /// which can be triggered externally to abort the in-flight subscription
    /// (e.g. when the user switches tab / filter before data arrives). For the
    /// fire-and-forget form, use the `subscribe_options_list` Action which uses
    /// `SpawnPolicy::AlwaysCancel` internally.
    pub async fn subscribe_options_list_future(&self, subscription_id: u32, force_refresh: bool, category_group: OptionsCategoryGroupVo, exp_time: Option<String>, strike_price: Option<String>, direction: OptionsDirection, sort_by: SortBy, sort_order: SortOrderEnum, cancel_token: Arc<OKCancellationToken>) -> Result<(), OKFFIError> { todo!() }
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

pub struct DiscoverOptionsSimpleInteractor;
impl DiscoverOptionsSimpleInteractor {
    pub fn new_default() -> Arc<DiscoverOptionsSimpleInteractor> { todo!() }
    pub fn check_is_simple_options_and_copilot_enabled(&self) -> bool { todo!() }
    pub fn load_options_simple(&self) { todo!() }
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
    pub fn state(&self) -> DiscoverOptionsSimpleState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

pub struct DiscoverSpotListInteractor;
impl DiscoverSpotListInteractor {
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
    pub fn state(&self) -> DiscoverSpotListState { todo!() }
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
    /// Subscribe (Action form): fire-and-forget; a new call cancels the
    /// previous via `SpawnPolicy::AlwaysCancel`. For cancellable one-shot
    /// (e.g. mid-flight switch), use the `_future` variant with an external
    /// `OKCancellationToken`.
    pub fn subscribe_spot_list(&self, subscription_id: u32, force_refresh: bool, page_vo: SpotPageVo, sort_by: SortBy, sort_order: SortOrderEnum, tickers_period: Option<TickersPeriodEnum>, visible_window: Option<VisibleWindowVo>) { todo!() }
    /// Subscribe to the spot list stream as a cancellable async Future.
    /// 
    /// Cancellation is driven by an external `OKCancellationToken` — when the
    /// caller cancels the token, the underlying stream subscription is dropped
    /// and this future resolves. The caller owns the token's lifetime; the
    /// interactor does not store or cancel it internally.
    /// 
    /// `page_vo` (e.g. `SpotPageVo::Ranking { ... }`) encapsulates the business
    /// filter (category_group + charge_group + ranking flags); the interactor
    /// resolves it internally.
    pub async fn subscribe_spot_list_future(&self, subscription_id: u32, force_refresh: bool, page_vo: SpotPageVo, sort_by: SortBy, sort_order: SortOrderEnum, tickers_period: Option<TickersPeriodEnum>, visible_window: Option<VisibleWindowVo>, cancel_token: Arc<OKCancellationToken>) -> Result<(), OKFFIError> { todo!() }
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

pub struct DiscoverSpotPageInteractor;
impl DiscoverSpotPageInteractor {
    pub fn new_default() -> Arc<DiscoverSpotPageInteractor> { todo!() }
    /// Create a crypto list interactor that shares the same crypto manager instance.
    /// 
    /// This is the Page Interactor equivalent of
    /// Page Interactor entry for the crypto list sub-interactor.
    pub fn create_crypto_list_interactor(&self) -> Arc<DiscoverCryptoListInteractor> { todo!() }
    /// Create a spot list interactor that shares the same manager instance.
    /// 
    /// This is the Page Interactor equivalent of
    /// Page Interactor entry for the spot list sub-interactor.
    pub fn create_spot_list_interactor(&self) -> Arc<DiscoverSpotListInteractor> { todo!() }
    pub fn load_data(&self) { todo!() }
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
    pub fn state(&self) -> DiscoverSpotPageState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

pub struct DiscoverTradFiPageInteractor;
impl DiscoverTradFiPageInteractor {
    pub fn create_futures_list_interactor(&self) -> Arc<DiscoverFuturesListInteractor> { todo!() }
    pub fn create_spot_list_interactor(&self) -> Arc<DiscoverSpotListInteractor> { todo!() }
    /// Resolve the L2 tab layout and the page-level business-line availability,
    /// then publish both into the state.
    pub fn load_data(&self) { todo!() }
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
    pub fn state(&self) -> DiscoverTradFiPageState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

/// ExploreBoostCardInteractor — manages explore boost card data fetch + stage.
pub struct ExploreBoostCardInteractor;
impl ExploreBoostCardInteractor {
    /// Create a default instance backed by the production HTTP client.
    /// 
    /// Convention: #[uniffi::constructor] new_default() — consistent with every
    /// other interactor in this crate (CountingBannerInteractor,
    /// DiscoverSpotInteractor, etc.).
    pub fn new_default() -> Arc<ExploreBoostCardInteractor> { todo!() }
    /// Fetch boost card data using the given trusted now timestamp (milliseconds).
    /// 
    /// `trusted_now_ms` is supplied by Android's TrustedTimeHelper (which honors
    /// Settings.Global.AUTO_TIME). Rust never reads system time — it classifies
    /// stage deterministically from this value + the server timestamps.
    /// 
    /// Call sites: (a) first attach (initial load), (b) pull-to-refresh / Lua
    /// `refreshContent`, (c) each stage-boundary timer fire from the V2 Fragment.
    pub fn load_data(&self, trusted_now_ms: i64) { todo!() }
    /// Named alias for the Lua/countdown re-fetch path. Identical to load_data.
    /// 
    /// A separate name gives Android a clear call site for `refreshContent` /
    /// stage-boundary triggers, and preserves a forward-compat hook for diverging
    /// behavior without an ABI break.
    pub fn refresh(&self, trusted_now_ms: i64) { todo!() }
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
    pub fn state(&self) -> ExploreBoostCardState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

pub struct ExploreCardDexInteractor;
impl ExploreCardDexInteractor {
    /// Creates a new ExploreCardDexInteractor instance with default dependencies.
    pub fn new_default() -> Arc<ExploreCardDexInteractor> { todo!() }
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
    pub fn state(&self) -> ExploreCardDexState { todo!() }
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
    pub fn subscribe(&self) { todo!() }
    /// Subscribe to DEX data stream using Future pattern.
    /// 
    /// Returns an OKRustFuture for cancellation control and Kotlin coroutine integration.
    /// The Future completes when the stream ends or errors.
    pub fn subscribe_future(&self) -> Arc<OKRustFutureboolWithOKFFIError> { todo!() }
    pub async fn subscribe_future_v2(&self, cancel_token: Arc<OKCancellationToken>) -> Result<(), OKFFIError> { todo!() }
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

pub struct ExploreCardFuturesInteractor;
impl ExploreCardFuturesInteractor {
    /// Creates a new ExploreCardFuturesInteractor instance with default dependencies.
    pub fn new_default() -> Arc<ExploreCardFuturesInteractor> { todo!() }
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
    pub fn state(&self) -> ExploreCardFuturesState { todo!() }
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
    pub fn subscribe(&self) { todo!() }
    /// Subscribe to Futures data stream using Future pattern.
    /// 
    /// Returns an OKRustFuture for cancellation control and Kotlin coroutine integration.
    /// The Future completes when the stream ends or errors.
    pub fn subscribe_future(&self) -> Arc<OKRustFutureboolWithOKFFIError> { todo!() }
    pub async fn subscribe_future_v2(&self, cancel_token: Arc<OKCancellationToken>) -> Result<(), OKFFIError> { todo!() }
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

pub struct ExploreCardOptionsInteractor;
impl ExploreCardOptionsInteractor {
    /// Creates a new ExploreCardOptionsInteractor instance with default dependencies.
    pub fn new_default() -> Arc<ExploreCardOptionsInteractor> { todo!() }
    pub fn load_options_chains(&self) { todo!() }
    /// Load Options chains data using Future pattern.
    /// 
    /// Returns an OKRustFuture for cancellation control and Kotlin coroutine integration.
    /// The Future completes when the load finishes or errors.
    pub fn load_options_chains_future(&self) -> Arc<OKRustFutureboolWithOKFFIError> { todo!() }
    pub async fn load_options_chains_future_v2(&self, cancel_token: Arc<OKCancellationToken>) -> Result<(), OKFFIError> { todo!() }
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
    pub fn state(&self) -> ExploreCardOptionsState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

pub struct ExploreCardSpotInteractor;
impl ExploreCardSpotInteractor {
    /// Creates a new ExploreCardSpotInteractor instance with default dependencies.
    pub fn new_default() -> Arc<ExploreCardSpotInteractor> { todo!() }
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
    pub fn state(&self) -> ExploreCardSpotState { todo!() }
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
    pub fn subscribe(&self) { todo!() }
    /// Subscribe to Spot data stream using Future pattern.
    /// 
    /// Returns an OKRustFuture for cancellation control and Kotlin coroutine integration.
    /// The Future completes when the stream ends or errors.
    pub fn subscribe_future(&self) -> Arc<OKRustFutureboolWithOKFFIError> { todo!() }
    pub async fn subscribe_future_v2(&self, cancel_token: Arc<OKCancellationToken>) -> Result<(), OKFFIError> { todo!() }
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

pub struct ExploreEventContractInteractor;
impl ExploreEventContractInteractor {
    /// Creates a new ExploreEventContractInteractor with an independent Manager instance.
    /// 
    /// Use `new_with_manager` from Rust when a shared Manager is available (e.g.,
    /// from the Discover interactor) to avoid duplicate network calls and share the
    /// underlying WS subscription.
    pub fn new_default() -> Arc<ExploreEventContractInteractor> { todo!() }
    pub fn load_data(&self) { todo!() }
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
    pub fn state(&self) -> ExploreEventContractState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

pub struct ExploreRootInteractor;
impl ExploreRootInteractor {
    pub fn new_default() -> Arc<ExploreRootInteractor> { todo!() }
    pub fn category_has_futures_tab(&self) -> bool { todo!() }
    pub fn category_has_spot_tab(&self, category: FinanceCategoryTabVo) -> bool { todo!() }
    pub fn create_category_futures_card_interactor(&self, category: FinanceCategoryTabVo) -> Arc<ExploreCardFuturesInteractor> { todo!() }
    pub fn create_category_spot_card_interactor(&self, category: FinanceCategoryTabVo) -> Arc<ExploreCardSpotInteractor> { todo!() }
    pub fn create_futures_card_interactor(&self) -> Arc<ExploreCardFuturesInteractor> { todo!() }
    pub fn create_spot_card_interactor(&self) -> Arc<ExploreCardSpotInteractor> { todo!() }
    pub fn get_cached_explore_l1_tabs(&self) -> Option<Vec<ExploreL1Page>> { todo!() }
    pub fn load_explore_l1_tabs(&self) { todo!() }
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
    pub fn state(&self) -> ExploreRootState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

/// Interactor for the futures analysis calendar view.
pub struct FaCalendarInteractor;
impl FaCalendarInteractor {
    /// Create a default instance backed by the production HTTP client.
    pub fn new_default() -> Arc<FaCalendarInteractor> { todo!() }
    /// Load calendar PnL data for a given month.
    /// 
    /// # Arguments
    /// * `date_time` - Date string for the target month (e.g. "2026-03")
    /// * `valuation_unit` - Valuation currency (e.g. "USDT", "CNY")
    /// * `uid` - User ID
    /// * `timezone` - Timezone offset string (e.g. "8")
    pub fn load(&self, date_time: String, valuation_unit: String, uid: String, timezone: String) { todo!() }
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
    pub fn state(&self) -> FaCalendarState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

/// Interactor for the futures analysis funding fee page.
pub struct FaFundingFeeInteractor;
impl FaFundingFeeInteractor {
    /// Create a default instance backed by the production HTTP client.
    pub fn new_default() -> Arc<FaFundingFeeInteractor> { todo!() }
    /// Load funding fee chart data.
    /// 
    /// # Arguments
    /// * `period` - Time period for the query
    /// * `inst_id` - Optional instrument ID filter (empty string means all)
    /// * `valuation_unit` - Valuation currency (e.g. "USDT")
    /// * `uid` - User ID
    pub fn load_chart(&self, period: AssetAnalysisPeriod, inst_id: String, valuation_unit: String, uid: String) { todo!() }
    /// Load estimated funding fee list.
    /// 
    /// # Arguments
    /// * `sort_direction` - Sort direction for the list
    /// * `uid` - User ID
    pub fn load_estimated(&self, sort_direction: FaFundingSortDirection, uid: String) { todo!() }
    /// Load available swap instrument IDs for coin pair selection.
    /// 
    /// # Arguments
    /// * `uid` - User ID
    pub fn load_swap_inst_ids(&self, uid: String) { todo!() }
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
    pub fn state(&self) -> FaFundingFeeState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

/// Interactor for the futures analysis overview page.
pub struct FaOverviewInteractor;
impl FaOverviewInteractor {
    /// Create a default instance backed by the production HTTP client.
    pub fn new_default() -> Arc<FaOverviewInteractor> { todo!() }
    /// Load overview performance data.
    /// 
    /// # Arguments
    /// * `period` - Time period for the query
    /// * `valuation_unit` - Valuation currency (e.g. "USDT")
    /// * `uid` - User ID
    pub fn load(&self, period: AssetAnalysisPeriod, valuation_unit: String, uid: String) { todo!() }
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
    pub fn state(&self) -> FaOverviewState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

/// Interactor for the futures analysis ticker breakdown.
pub struct FaTickerInteractor;
impl FaTickerInteractor {
    /// Create a default instance backed by the production HTTP client.
    pub fn new_default() -> Arc<FaTickerInteractor> { todo!() }
    /// Load PnL breakdown by instrument.
    /// 
    /// # Arguments
    /// * `period` - Time period for the query
    /// * `pnl_type` - PnL type (net, long, short)
    /// * `valuation_unit` - Valuation currency (e.g. "USDT")
    /// * `uid` - User ID
    pub fn load(&self, period: AssetAnalysisPeriod, pnl_type: FaPnlType, valuation_unit: String, uid: String) { todo!() }
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
    pub fn state(&self) -> FaTickerState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

/// Callback interface for favorite items updates
/// 
/// Native platforms implement this trait to receive real-time favorite updates.
/// This is used for subscribing to favorite changes without ticker data.
pub struct FavoriteItemsUpdateCallback;
impl FavoriteItemsUpdateCallback {
    /// Called when favorite items are updated
    /// 
    /// # Arguments
    /// * `favorites` - The current list of favorite items
    pub fn on_update_favorites(&self, favorites: Vec<RetailTradingFavoriteItem>) { todo!() }
    /// Called when the favorites update stream is completed
    pub fn on_update_favorites_completed(&self) { todo!() }
    /// Called when an error occurs while updating favorites
    /// 
    /// # Arguments
    /// * `error` - The error that occurred
    pub fn on_update_favorites_error(&self, error: OKFFIError) { todo!() }
}

pub struct FavoriteSelectionCryptoListInteractor;
impl FavoriteSelectionCryptoListInteractor {
    pub fn new_default() -> Arc<FavoriteSelectionCryptoListInteractor> { todo!() }
    /// Clear search results
    pub fn clear(&self) { todo!() }
    /// Load crypto business line config (category groups)
    pub fn load_config(&self) { todo!() }
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
    pub fn state(&self) -> FavoriteSelectionCryptoListState { todo!() }
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
    /// Subscribe to crypto search results
    pub fn subscribe(&self, subscription_id: u32, keyword: Option<String>, filter: FavoriteSelectionCryptoFilterVo, sort_by: SortBy, sort_order: SortOrderEnum) { todo!() }
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

pub struct FavoriteSelectionDexListInteractor;
impl FavoriteSelectionDexListInteractor {
    pub fn new_default() -> Arc<FavoriteSelectionDexListInteractor> { todo!() }
    /// Clear search results
    pub fn clear(&self) { todo!() }
    /// Load DEX business line config (chain groups, periods, sort options)
    pub fn load_config(&self) { todo!() }
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
    pub fn state(&self) -> FavoriteSelectionDexListState { todo!() }
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
    /// Subscribe to DEX search results
    pub fn subscribe(&self, subscription_id: u32, keyword: Option<String>, filter: FavoriteSelectionDexFilterVo, sort_by: SortBy, sort_order: SortOrderEnum) { todo!() }
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

pub struct FavoriteSelectionExosDexListInteractor;
impl FavoriteSelectionExosDexListInteractor {
    pub fn new_default() -> Arc<FavoriteSelectionExosDexListInteractor> { todo!() }
    /// Clear search results.
    pub fn clear(&self) { todo!() }
    /// Load exos DEX config (biz tabs, swap chain sub-tabs, filter options).
    pub fn load_config(&self) { todo!() }
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
    pub fn state(&self) -> FavoriteSelectionExosDexListState { todo!() }
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
    /// Subscribe to a market list (Perp / Spot / Swap).
    /// 
    /// - `biz_tab`: Perp / Spot / Swap.
    /// - `keyword`: keyword mode searches; no-keyword browses the ranking.
    /// - `filter`: swap chain id + period (perp/spot ignore chain id).
    /// - `sort_by` / `sort_order`: market-list sort.
    pub fn subscribe(&self, subscription_id: u32, biz_tab: ExchangeOsBizTab, keyword: Option<String>, filter: FavoriteSelectionExosDexFilterVo, sort_by: DexSortBy, sort_order: SortOrderEnum) { todo!() }
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

pub struct FavoriteSelectionFuturesListInteractor;
impl FavoriteSelectionFuturesListInteractor {
    pub fn new_default() -> Arc<FavoriteSelectionFuturesListInteractor> { todo!() }
    /// Clear search results
    pub fn clear(&self) { todo!() }
    /// Load the futures config (cloud-aware tab page list)
    pub fn load_config(&self) { todo!() }
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
    pub fn state(&self) -> FavoriteSelectionFuturesListState { todo!() }
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
    /// Subscribe to futures search results
    pub fn subscribe(&self, subscription_id: u32, keyword: Option<String>, filter: FavoriteSelectionFuturesFilterVo, sort_by: SortBy, sort_order: SortOrderEnum) { todo!() }
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

pub struct FavoriteSelectionOptionsListInteractor;
impl FavoriteSelectionOptionsListInteractor {
    pub fn new_default() -> Arc<FavoriteSelectionOptionsListInteractor> { todo!() }
    /// Clear search results
    pub fn clear(&self) { todo!() }
    /// Load options business line config (category groups, directions)
    pub fn load_config(&self) { todo!() }
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
    pub fn state(&self) -> FavoriteSelectionOptionsListState { todo!() }
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
    /// Subscribe to options search results
    pub fn subscribe(&self, subscription_id: u32, keyword: Option<String>, filter: FavoriteSelectionOptionsFilterVo, sort_by: SortBy, sort_order: SortOrderEnum) { todo!() }
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

pub struct FavoriteSelectionRootInteractor;
impl FavoriteSelectionRootInteractor {
    pub fn new_default() -> Arc<FavoriteSelectionRootInteractor> { todo!() }
    /// Clear all selections across all business lines
    pub fn clear_selections(&self) { todo!() }
    /// Confirm and add all selected items to favorites
    pub fn confirm_add_to_favorites(&self) { todo!() }
    /// Create a crypto list interactor sharing the same manager
    pub fn create_crypto_list_interactor(&self) -> Arc<FavoriteSelectionCryptoListInteractor> { todo!() }
    /// Create a DEX list interactor sharing the same manager
    pub fn create_dex_list_interactor(&self) -> Arc<FavoriteSelectionDexListInteractor> { todo!() }
    /// Create an exos DEX list interactor sharing the same manager.
    /// 
    /// The exos DEX path (Perp / Spot / Swap) is the AB-on data source,
    /// backed by the same NavSearch ExchangeOS pipeline. The legacy
    /// `create_dex_list_interactor` (AB-off) stays untouched.
    pub fn create_exos_dex_list_interactor(&self) -> Arc<FavoriteSelectionExosDexListInteractor> { todo!() }
    /// Create a futures list interactor sharing the same manager
    pub fn create_futures_list_interactor(&self) -> Arc<FavoriteSelectionFuturesListInteractor> { todo!() }
    /// Create an options list interactor sharing the same manager
    pub fn create_options_list_interactor(&self) -> Arc<FavoriteSelectionOptionsListInteractor> { todo!() }
    /// Create a spot list interactor sharing the same manager
    pub fn create_spot_list_interactor(&self) -> Arc<FavoriteSelectionSpotListInteractor> { todo!() }
    /// Get the three-state selection status for an item (non-blocking synchronous call).
    pub fn get_selection_state(&self, biz_line: FavoriteSelectionBusinessLineVo, item_id: String) -> FavoriteSelectionStateVo { todo!() }
    /// Initialize the interactor with a group name, loading existing favorites internally
    pub fn initialize(&self, group_name: String) { todo!() }
    /// Load hit statistics for all business lines
    pub fn load_hit_statistics(&self, keyword: Option<String>, is_dex_enabled: bool) { todo!() }
    /// Refresh the selected IDs state (sync state from manager)
    pub fn refresh_selected_ids(&self) { todo!() }
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
    pub fn state(&self) -> FavoriteSelectionRootState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
    /// Toggle selection for an item
    pub fn toggle_selection(&self, biz_line: FavoriteSelectionBusinessLineVo, item_id: String) { todo!() }
}

pub struct FavoriteSelectionSpotListInteractor;
impl FavoriteSelectionSpotListInteractor {
    pub fn new_default() -> Arc<FavoriteSelectionSpotListInteractor> { todo!() }
    /// Clear search results
    pub fn clear(&self) { todo!() }
    /// Load the spot config (cloud-aware tab page list)
    pub fn load_config(&self) { todo!() }
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
    pub fn state(&self) -> FavoriteSelectionSpotListState { todo!() }
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
    /// Subscribe to spot search results
    pub fn subscribe(&self, subscription_id: u32, keyword: Option<String>, filter: FavoriteSelectionSpotFilterVo, sort_by: SortBy, sort_order: SortOrderEnum) { todo!() }
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

pub struct FavoritesAddToGroupInteractor;
impl FavoritesAddToGroupInteractor {
    pub fn new_default() -> Arc<FavoritesAddToGroupInteractor> { todo!() }
    /// Add the provided cryptos to all currently selected groups.
    pub fn add_cryptos_to_selected_groups(&self, cryptos: Vec<RetailTradingFavoriteItem>) { todo!() }
    /// Clear all group selections (synchronous state mutation).
    pub fn clear_selections(&self) { todo!() }
    /// Load available groups for the "add to group" bottom sheet.
    /// All filtering is done in Rust. Also populates `all_group_names`.
    /// 
    /// When `force_refresh` is `true`, the local cache is bypassed and the latest
    /// group data is fetched directly from the API.
    pub fn load_available_groups(&self, exclude_group_name: Option<String>, is_dex_restricted: bool, force_refresh: bool) { todo!() }
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
    pub fn set_ui_event_observer(&self, observer: Arc<dyn FavoritesAddToGroupUxEffectObserver>) { todo!() }
    /// Start the interactor lifecycle.
    /// 
    /// Calls the user's `InteractorLifecycle::start()` implementation.
    /// Override that method to add custom startup behavior (e.g., initial data loads).
    pub fn start(&self) { todo!() }
    /// Get the current state (UniFFI export).
    /// 
    /// Returns the External type (UniFFI-compatible).
    pub fn state(&self) -> FavoritesAddToGroupState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
    /// Toggle selection for a group (synchronous state mutation).
    pub fn toggle_group_selection(&self, group_name: String) { todo!() }
    /// Opt-in async UI event stream (cycle-free). Single-subscriber.
    pub fn ui_event_stream(&self) -> Arc<FavoritesAddToGroupUxEffectStream> { todo!() }
}

/// Deprecated UniFFI callback interface for observing UI events. Prefer the cycle-free `FavoritesAddToGroupUxEffectStream` returned by `interactor.ui_event_stream()`.
pub struct FavoritesAddToGroupUxEffectObserver;
impl FavoritesAddToGroupUxEffectObserver {
    pub fn on_ui_event(&self, event: FavoritesAddToGroupUxEffect) { todo!() }
}

/// Opt-in async event stream from `interactor.ui_event_stream()`.
/// Single-subscriber, latest-wins. `next()` resolves to `None` when the
/// interactor is dropped or a new subscription replaces this one.
pub struct FavoritesAddToGroupUxEffectStream;
impl FavoritesAddToGroupUxEffectStream {
    pub async fn next(&self) -> Option<FavoritesAddToGroupUxEffect> { todo!() }
}

/// Favorites CRUD Bridge — Repository migration minimal surface.
/// 
/// Provides 8 CRUD async methods + 2 subscribe methods, all supporting
/// `OKCancellationToken` for Kotlin coroutine cancellation.
/// 
/// - CRUD: WatchlistManager (handles FavoriteEventBus publishing)
/// - Subscribe: WatchlistManager stream infrastructure
pub struct FavoritesCrudBridge;
impl FavoritesCrudBridge {
    pub fn new_default() -> Arc<FavoritesCrudBridge> { todo!() }
    /// Add a favorite item to a specific group.
    pub async fn add_favorite(&self, item: RetailTradingFavoriteItem, group_name: String, cancel_token: Arc<OKCancellationToken>) -> Result<bool, FavoritesCrudError> { todo!() }
    /// Add a favorite item to the watchlist (ALL group, inserted at head).
    /// 
    /// Uses the `/setting` API which inserts at the **beginning** of the list,
    /// unlike `add_favorite` which uses the `/add-to-group` API that appends
    /// to the **end**. Use this method when the item should be immediately
    /// visible in the home favorites list (which displays the first N items).
    pub async fn add_favorite_to_watchlist(&self, item: RetailTradingFavoriteItem, cancel_token: Arc<OKCancellationToken>) -> Result<bool, FavoritesCrudError> { todo!() }
    /// Batch add favorite items to the watchlist (ALL group).
    /// 
    /// Uses the batch `/add-to-group` API with `ALL_GROUP_NAME`,
    /// adding all items in a single network request.
    pub async fn add_favorites_to_watchlist(&self, items: Vec<RetailTradingFavoriteItem>, cancel_token: Arc<OKCancellationToken>) -> Result<bool, FavoritesCrudError> { todo!() }
    /// Check if a favorite item exists in any group.
    pub async fn check_favorite(&self, item: RetailTradingFavoriteItem, cancel_token: Arc<OKCancellationToken>) -> Result<bool, FavoritesCrudError> { todo!() }
    /// Create a new group.
    pub async fn create_group(&self, group_name: String, cancel_token: Arc<OKCancellationToken>) -> Result<bool, FavoritesCrudError> { todo!() }
    /// Delete a group.
    pub async fn delete_group(&self, group_name: String, cancel_token: Arc<OKCancellationToken>) -> Result<bool, FavoritesCrudError> { todo!() }
    /// Get all groups with their favorites.
    pub async fn get_all_groups(&self, cancel_token: Arc<OKCancellationToken>) -> Result<Vec<RetailTradingGroupFavorite>, FavoritesCrudError> { todo!() }
    /// Get favorites for a specific group.
    pub async fn get_favorites_by_group(&self, group_name: String, cancel_token: Arc<OKCancellationToken>) -> Result<Vec<RetailTradingFavoriteItem>, FavoritesCrudError> { todo!() }
    /// Remove favorite items from a specific group (batch).
    pub async fn remove_favorites(&self, items: Vec<RetailTradingFavoriteItem>, group_name: String, cancel_token: Arc<OKCancellationToken>) -> Result<bool, FavoritesCrudError> { todo!() }
    /// Replace (rearrange) all favorites in a group.
    pub async fn replace_favorites(&self, group_name: String, favorites: Vec<RetailTradingFavoriteItem>, cancel_token: Arc<OKCancellationToken>) -> Result<bool, FavoritesCrudError> { todo!() }
    /// Subscribe to favorites changes for a specific group.
    /// 
    /// - ALL group → `subscribe_all_favorites_with_auto_refresh` (existing)
    /// - Other groups → `subscribe_group_favorites_with_auto_refresh` (new)
    /// 
    /// The stream is cancelled when `cancel_token` is cancelled (Kotlin
    /// coroutine cancellation triggers this via `awaitClose { token.cancel() }`).
    pub async fn subscribe_favorites_by_group(&self, group_name: String, cancel_token: Arc<OKCancellationToken>, callback: Arc<dyn FavoriteItemsUpdateCallback>) -> Result<(), FavoritesCrudError> { todo!() }
    /// Subscribe to group list changes.
    /// 
    /// Monitors GroupCreated, GroupDeleted, and Refreshed events.
    /// 
    /// The stream is cancelled when `cancel_token` is cancelled.
    pub async fn subscribe_groups(&self, cancel_token: Arc<OKCancellationToken>, callback: Arc<dyn GroupsUpdateCallback>) -> Result<(), FavoritesCrudError> { todo!() }
}

/// FavoritesCryptosEditInteractor - manages favorites group items for the edit screen
pub struct FavoritesCryptosEditInteractor;
impl FavoritesCryptosEditInteractor {
    pub fn new_default() -> Arc<FavoritesCryptosEditInteractor> { todo!() }
    /// Load all favorites groups
    pub fn load_groups(&self) { todo!() }
    /// Load items for a specific group
    pub fn load_items_for_group(&self, group_name: String) { todo!() }
    /// Rearrange items within a group. The full ordered list must be provided.
    pub fn rearrange_items(&self, group_name: String, items: Vec<RetailTradingFavoriteItem>) { todo!() }
    /// Stop-gap teardown (TEMPORARY — target state is the stream APIs,
    /// which need no teardown call).
    /// 
    /// One idempotent call at host lifecycle end that breaks the
    /// UniFFI handleMap cycle for any registered callback observers,
    /// ends active streams, and stops effects. On Kotlin call BEFORE
    /// `destroy()`. Includes `stop()`.
    pub fn release(&self) { todo!() }
    /// Remove items from a group. Pass `ALL_GROUP_NAME` ("ALL") to remove from all groups.
    pub fn remove_items(&self, items: Vec<RetailTradingFavoriteItem>, group_name: String) { todo!() }
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
    pub fn set_ui_event_observer(&self, observer: Arc<dyn FavoritesCryptosEditUxEffectObserver>) { todo!() }
    /// Start the interactor lifecycle.
    /// 
    /// Calls the user's `InteractorLifecycle::start()` implementation.
    /// Override that method to add custom startup behavior (e.g., initial data loads).
    pub fn start(&self) { todo!() }
    /// Get the current state (UniFFI export).
    /// 
    /// Returns the External type (UniFFI-compatible).
    pub fn state(&self) -> FavoritesCryptosEditState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
    /// Opt-in async UI event stream (cycle-free). Single-subscriber.
    pub fn ui_event_stream(&self) -> Arc<FavoritesCryptosEditUxEffectStream> { todo!() }
}

/// Deprecated UniFFI callback interface for observing UI events. Prefer the cycle-free `FavoritesCryptosEditUxEffectStream` returned by `interactor.ui_event_stream()`.
pub struct FavoritesCryptosEditUxEffectObserver;
impl FavoritesCryptosEditUxEffectObserver {
    pub fn on_ui_event(&self, event: FavoritesCryptosEditUxEffect) { todo!() }
}

/// Opt-in async event stream from `interactor.ui_event_stream()`.
/// Single-subscriber, latest-wins. `next()` resolves to `None` when the
/// interactor is dropped or a new subscription replaces this one.
pub struct FavoritesCryptosEditUxEffectStream;
impl FavoritesCryptosEditUxEffectStream {
    pub async fn next(&self) -> Option<FavoritesCryptosEditUxEffect> { todo!() }
}

/// FavoritesGroupsEditInteractor - manages favorites groups for the groups edit screen
pub struct FavoritesGroupsEditInteractor;
impl FavoritesGroupsEditInteractor {
    pub fn new_default() -> Arc<FavoritesGroupsEditInteractor> { todo!() }
    /// Create a new group with the given name
    pub fn create_group(&self, group_name: String) { todo!() }
    /// Delete a group by name
    pub fn delete_group(&self, group_name: String) { todo!() }
    /// Load all favorites groups, split into showing and hidden
    pub fn load_groups(&self) { todo!() }
    /// Rearrange groups order. Accepts the full ordered list of group names.
    pub fn rearrange_groups(&self, group_names: Vec<String>) { todo!() }
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
    /// Rename a group
    pub fn rename_group(&self, old_group_name: String, new_group_name: String) { todo!() }
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
    pub fn set_ui_event_observer(&self, observer: Arc<dyn FavoritesGroupsEditUxEffectObserver>) { todo!() }
    /// Start the interactor lifecycle.
    /// 
    /// Calls the user's `InteractorLifecycle::start()` implementation.
    /// Override that method to add custom startup behavior (e.g., initial data loads).
    pub fn start(&self) { todo!() }
    /// Get the current state (UniFFI export).
    /// 
    /// Returns the External type (UniFFI-compatible).
    pub fn state(&self) -> FavoritesGroupsEditState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
    /// Opt-in async UI event stream (cycle-free). Single-subscriber.
    pub fn ui_event_stream(&self) -> Arc<FavoritesGroupsEditUxEffectStream> { todo!() }
    /// Toggle group visibility (hide/show)
    pub fn update_group_visibility(&self, group_name: String, hidden: bool) { todo!() }
    /// Validate a group name (async, for real-time input validation)
    pub fn validate_group_name(&self, group_name: String) { todo!() }
}

/// Deprecated UniFFI callback interface for observing UI events. Prefer the cycle-free `FavoritesGroupsEditUxEffectStream` returned by `interactor.ui_event_stream()`.
pub struct FavoritesGroupsEditUxEffectObserver;
impl FavoritesGroupsEditUxEffectObserver {
    pub fn on_ui_event(&self, event: FavoritesGroupsEditUxEffect) { todo!() }
}

/// Opt-in async event stream from `interactor.ui_event_stream()`.
/// Single-subscriber, latest-wins. `next()` resolves to `None` when the
/// interactor is dropped or a new subscription replaces this one.
pub struct FavoritesGroupsEditUxEffectStream;
impl FavoritesGroupsEditUxEffectStream {
    pub async fn next(&self) -> Option<FavoritesGroupsEditUxEffect> { todo!() }
}

/// FormulaNavInteractor - manages formula building operations
pub struct FormulaNavInteractor;
impl FormulaNavInteractor {
    pub fn new_default() -> Arc<FormulaNavInteractor> { todo!() }
    pub fn new_with_formula(existing_formula: String) -> Arc<FormulaNavInteractor> { todo!() }
    pub fn add_constant(&self, value: f64) { todo!() }
    pub fn add_instrument(&self, inst_id: String, inst_type: String) { todo!() }
    pub fn add_operator(&self, operator: FormulaOperator) { todo!() }
    /// Check if the formula can be saved.
    /// 
    /// # Returns
    /// `true` if the formula is valid and can be saved
    pub fn can_save(&self) -> bool { todo!() }
    pub fn clear_formula(&self) { todo!() }
    /// Get the current display formula string.
    /// 
    /// # Returns
    /// The display formula (e.g., "{BTC-USDT&SPOT}+{ETH-USDT&SWAP}×2")
    pub fn get_display_formula(&self) -> String { todo!() }
    /// Get the current formula data.
    /// 
    /// # Returns
    /// FormulaData containing display formula, calc expression, and inst array
    pub fn get_formula_data(&self) -> FormulaData { todo!() }
    /// Get all instrument IDs currently in the formula.
    /// 
    /// # Returns
    /// Vector of instrument IDs in the formula
    pub fn get_formula_inst_ids(&self) -> Vec<String> { todo!() }
    /// Get the current formula state from the manager.
    /// 
    /// # Returns
    /// Complete formula state including data, validation, and capacity
    pub fn get_manager_formula_state(&self) -> FormulaSearchState { todo!() }
    /// Check if the formula is full.
    /// 
    /// # Returns
    /// `true` if the formula has reached the maximum element count (10)
    pub fn is_formula_full(&self) -> bool { todo!() }
    pub fn refresh_state(&self) { todo!() }
    /// Stop-gap teardown (TEMPORARY — target state is the stream APIs,
    /// which need no teardown call).
    /// 
    /// One idempotent call at host lifecycle end that breaks the
    /// UniFFI handleMap cycle for any registered callback observers,
    /// ends active streams, and stops effects. On Kotlin call BEFORE
    /// `destroy()`. Includes `stop()`.
    pub fn release(&self) { todo!() }
    pub fn remove_last_element(&self) { todo!() }
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
    pub fn state(&self) -> FormulaNavState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
    pub fn validate_formula(&self) { todo!() }
}

/// Formula search manager for managing formula search and building workflow
pub struct FormulaSearchManager;
impl FormulaSearchManager {
    /// Create a new FormulaSearchManager
    pub fn new() -> Arc<FormulaSearchManager> { todo!() }
    /// Create FormulaSearchManager with existing formula
    /// 
    /// # Arguments
    /// * `existing_formula` - Existing formula string to parse
    pub fn with_formula(existing_formula: String) -> Arc<FormulaSearchManager> { todo!() }
    /// Add a constant value to the formula
    /// 
    /// # Arguments
    /// * `value` - The numeric constant value
    /// 
    /// # Returns
    /// `true` if added successfully
    pub fn add_constant(&self, value: f64) -> bool { todo!() }
    /// Add an instrument to the formula
    /// 
    /// Note: The same instrument can be added multiple times in a formula.
    /// For example: `BTC + (BTC / 2)` is a valid formula.
    /// 
    /// # Arguments
    /// * `inst_id` - The instrument ID
    /// * `inst_type` - The instrument type
    /// 
    /// # Returns
    /// Result indicating success or failure reason
    pub fn add_instrument_to_formula(&self, inst_id: String, inst_type: String) -> AddInstrumentResult { todo!() }
    /// Add an operator to the formula
    /// 
    /// # Arguments
    /// * `operator` - The operator to add
    pub fn add_operator(&self, operator: FormulaOperator) { todo!() }
    /// Check if formula is valid and can be saved
    pub fn can_save(&self) -> bool { todo!() }
    /// Clear the entire formula
    pub fn clear_formula(&self) { todo!() }
    /// Get count of constants in formula
    pub fn get_constant_count(&self) -> u32 { todo!() }
    /// Get display formula string
    pub fn get_display_formula(&self) -> String { todo!() }
    /// Get current formula data
    pub fn get_formula_data(&self) -> FormulaData { todo!() }
    /// Get all instrument IDs currently in the formula
    pub fn get_formula_inst_ids(&self) -> Vec<String> { todo!() }
    /// Get current formula state
    pub fn get_formula_state(&self) -> FormulaSearchState { todo!() }
    /// Get number of remaining elements that can be added
    pub fn get_remaining_capacity(&self) -> u32 { todo!() }
    /// Get total element count (variables + constants)
    pub fn get_total_element_count(&self) -> u32 { todo!() }
    /// Get count of variables (instruments) in formula
    pub fn get_variable_count(&self) -> u32 { todo!() }
    /// Check if formula is full
    pub fn is_formula_full(&self) -> bool { todo!() }
    /// Remove the last element from the formula (backspace)
    pub fn remove_last_element(&self) { todo!() }
    /// Validate current formula
    /// 
    /// Returns any parse error from `with_formula()` first, then validates the formula.
    pub fn validate_formula(&self) -> Option<FormulaValidationError> { todo!() }
}

/// Callback interface for group list updates.
/// 
/// Native platforms (Kotlin/Swift) implement this trait to receive
/// real-time group list changes (GroupCreated/GroupDeleted/Refreshed).
pub struct GroupsUpdateCallback;
impl GroupsUpdateCallback {
    /// Called when the group list is updated
    pub fn on_update_groups(&self, groups: Vec<RetailTradingGroupFavorite>) { todo!() }
    /// Called when the group update stream is completed
    pub fn on_update_groups_completed(&self) { todo!() }
    /// Called when an error occurs while updating groups
    pub fn on_update_groups_error(&self, error: OKFFIError) { todo!() }
}

/// HomeCommodityInteractor - manages commodity ranking data subscription
pub struct HomeCommodityInteractor;
impl HomeCommodityInteractor {
    /// Creates a new HomeCommodityInteractor with default dependencies.
    pub fn new_default() -> Arc<HomeCommodityInteractor> { todo!() }
    /// Cancel commodity rank subscription (for page invisibility)
    pub fn cancel_commodity_rank(&self) { todo!() }
    /// Load commodity ranking data (both spot and futures)
    pub fn load_commodity_rank(&self) { todo!() }
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
    pub fn state(&self) -> HomeCommodityState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

pub struct HomeCustomRankInteractor;
impl HomeCustomRankInteractor {
    pub fn new_default() -> Arc<HomeCustomRankInteractor> { todo!() }
    pub fn cancel_custom_rank(&self) { todo!() }
    pub fn load_custom_rank(&self, rank_type: String, num: i32, ccy: String) { todo!() }
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
    pub fn state(&self) -> HomeCustomRankState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

/// HomeEquityInteractor - manages equity (real stock) ranking data subscription
pub struct HomeEquityInteractor;
impl HomeEquityInteractor {
    /// Creates a new HomeEquityInteractor with default dependencies.
    pub fn new_default() -> Arc<HomeEquityInteractor> { todo!() }
    /// Cancel equity rank subscription (for page invisibility)
    pub fn cancel_equity_rank(&self) { todo!() }
    /// Load equity ranking data (both spot and futures)
    pub fn load_equity_rank(&self) { todo!() }
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
    pub fn state(&self) -> HomeEquityState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

/// HomeFavoritesInteractor - manages favorites with real-time ticker updates
pub struct HomeFavoritesInteractor;
impl HomeFavoritesInteractor {
    pub fn new_default() -> Arc<HomeFavoritesInteractor> { todo!() }
    pub fn new_with_config(max_display_count: i32) -> Arc<HomeFavoritesInteractor> { todo!() }
    pub fn add_favorite(&self, item: QuotationFavoriteVo) { todo!() }
    pub fn add_favorites(&self, items: Vec<QuotationFavoriteVo>, group_name: String) { todo!() }
    /// Check if an item is in the favorites watchlist.
    /// 
    /// This is an async query that ensures the watchlist data is loaded
    /// before checking. Safe to call even on cold start.
    /// 
    /// # Arguments
    /// * `item` - The item to check
    /// 
    /// # Returns
    /// `OKRustFuture<bool>` - `true` if the item is in the watchlist, `false` otherwise.
    /// 
    /// # Platform Usage
    /// 
    /// ## Android (Kotlin)
    /// ```kotlin
    /// viewModelScope.launch {
    /// val isFavorite = interactor.isFavorite(spotVo).toSuspend()
    /// updateFavoriteIcon(isFavorite)
    /// }
    /// ```
    /// 
    /// ## iOS (Swift)
    /// ```swift
    /// Task {
    /// let isFavorite = try await interactor.isFavorite(item: spotVo)
    /// updateFavoriteIcon(isFavorite)
    /// }
    /// ```
    pub fn is_favorite(&self, item: QuotationFavoriteVo) -> Arc<OKRustFutureboolWithOKFFIError> { todo!() }
    pub fn load_data(&self, group_name: String, bump_data_version: bool) { todo!() }
    pub fn rearrange(&self, items: Vec<QuotationFavoriteVo>) { todo!() }
    /// Stop-gap teardown (TEMPORARY — target state is the stream APIs,
    /// which need no teardown call).
    /// 
    /// One idempotent call at host lifecycle end that breaks the
    /// UniFFI handleMap cycle for any registered callback observers,
    /// ends active streams, and stops effects. On Kotlin call BEFORE
    /// `destroy()`. Includes `stop()`.
    pub fn release(&self) { todo!() }
    pub fn remove_favorite(&self, item: QuotationFavoriteVo) { todo!() }
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
    pub fn set_ui_event_observer(&self, observer: Arc<dyn HomeFavoritesUxEffectObserver>) { todo!() }
    /// Start the interactor lifecycle.
    /// 
    /// Calls the user's `InteractorLifecycle::start()` implementation.
    /// Override that method to add custom startup behavior (e.g., initial data loads).
    pub fn start(&self) { todo!() }
    /// Get the current state (UniFFI export).
    /// 
    /// Returns the External type (UniFFI-compatible).
    pub fn state(&self) -> HomeFavoritesState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
    /// Opt-in async UI event stream (cycle-free). Single-subscriber.
    pub fn ui_event_stream(&self) -> Arc<HomeFavoritesUxEffectStream> { todo!() }
    pub fn update_sort(&self, sort_by: SortBy, sort_order: SortOrderEnum) { todo!() }
}

/// Deprecated UniFFI callback interface for observing UI events. Prefer the cycle-free `HomeFavoritesUxEffectStream` returned by `interactor.ui_event_stream()`.
pub struct HomeFavoritesUxEffectObserver;
impl HomeFavoritesUxEffectObserver {
    pub fn on_ui_event(&self, event: HomeFavoritesUxEffect) { todo!() }
}

/// Opt-in async event stream from `interactor.ui_event_stream()`.
/// Single-subscriber, latest-wins. `next()` resolves to `None` when the
/// interactor is dropped or a new subscription replaces this one.
pub struct HomeFavoritesUxEffectStream;
impl HomeFavoritesUxEffectStream {
    pub async fn next(&self) -> Option<HomeFavoritesUxEffect> { todo!() }
}

/// HomeGainerInteractor - manages gainer ranking data subscription
pub struct HomeGainerInteractor;
impl HomeGainerInteractor {
    /// Creates a new HomeGainerInteractor with default dependencies.
    pub fn new_default() -> Arc<HomeGainerInteractor> { todo!() }
    /// Cancel gainer rank subscription (for page invisibility)
    pub fn cancel_gainer_rank(&self) { todo!() }
    /// Load gainer ranking data for the specified instrument type
    pub fn load_gainer_rank(&self, inst_type: GainerRankingInstType) { todo!() }
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
    pub fn state(&self) -> HomeGainerState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

/// HomeHotInteractor - manages hot ranking data subscription
pub struct HomeHotInteractor;
impl HomeHotInteractor {
    /// Creates a new HomeHotInteractor with default dependencies.
    pub fn new_default() -> Arc<HomeHotInteractor> { todo!() }
    /// Cancel hot rank subscription (for page invisibility)
    pub fn cancel_hot_rank(&self) { todo!() }
    /// Load hot ranking data for the specified instrument type
    pub fn load_hot_rank(&self, inst_type: HotRankingInstType) { todo!() }
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
    pub fn state(&self) -> HomeHotState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

/// HomeIndexInteractor - manages home index coin quotes subscription
pub struct HomeIndexInteractor;
impl HomeIndexInteractor {
    /// Creates a new HomeIndexInteractor with default dependencies.
    pub fn new_default() -> Arc<HomeIndexInteractor> { todo!() }
    /// Cancel index quote subscription (for page invisibility).
    pub fn cancel_index_quotes(&self) { todo!() }
    /// Load home index coin quotes (bootstrap + subscribe). Call on page visible.
    pub fn load_index_quotes(&self) { todo!() }
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
    pub fn state(&self) -> HomeIndexState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

/// HomeNewCoinInteractor - manages new coin ranking data subscription
pub struct HomeNewCoinInteractor;
impl HomeNewCoinInteractor {
    /// Creates a new HomeNewCoinInteractor with default dependencies.
    pub fn new_default() -> Arc<HomeNewCoinInteractor> { todo!() }
    /// Cancel new coin rank subscription (for page invisibility)
    pub fn cancel_new_coin_rank(&self) { todo!() }
    /// Load new coin ranking data for the specified instrument type
    pub fn load_new_coin_rank(&self, inst_type: NewCoinRankingInstType) { todo!() }
    /// Load unified new coin rank data (includes spot + futures + wait online)
    /// Uses subscribe_new_coin_rank which returns HomeNewCoinGroupVo containing all data
    pub fn load_new_coin_rank_unified(&self) { todo!() }
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
    pub fn state(&self) -> HomeNewCoinState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

pub struct HomeRankingDexGroupInteractor;
impl HomeRankingDexGroupInteractor {
    /// Creates a new HomeRankingDexGroupInteractor with default dependencies.
    pub fn new_default() -> Arc<HomeRankingDexGroupInteractor> { todo!() }
    pub fn load_groups(&self) { todo!() }
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
    pub fn state(&self) -> HomeRankingDexGroupState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

pub struct HomeRankingDexInteractor;
impl HomeRankingDexInteractor {
    /// Creates a new HomeRankingDexInteractor with default dependencies.
    pub fn new_default() -> Arc<HomeRankingDexInteractor> { todo!() }
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
    pub fn state(&self) -> HomeRankingDexState { todo!() }
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
    pub fn subscribe(&self, chain_group: DexChainGroupVo) { todo!() }
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

/// HomeSelectedGroupInteractor - manages current favorites group selection
pub struct HomeSelectedGroupInteractor;
impl HomeSelectedGroupInteractor {
    pub fn new_default() -> Arc<HomeSelectedGroupInteractor> { todo!() }
    pub fn rearrange_groups(&self, group_name_list: Vec<String>) { todo!() }
    pub fn refresh(&self) { todo!() }
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
    pub fn set_initial_group(&self, group_name: String) { todo!() }
    pub fn set_selected_group(&self, group_name: String) { todo!() }
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
    pub fn state(&self) -> HomeSelectedGroupState { todo!() }
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
    pub fn subscribe(&self) { todo!() }
    pub fn subscribe_with_initial_group(&self, initial_group_name: Option<String>) { todo!() }
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

/// HomeStockInteractor - manages stock ranking data subscription
pub struct HomeStockInteractor;
impl HomeStockInteractor {
    /// Creates a new HomeStockInteractor with default dependencies.
    pub fn new_default() -> Arc<HomeStockInteractor> { todo!() }
    /// Cancel stock rank subscription (for page invisibility)
    pub fn cancel_stock_rank(&self) { todo!() }
    /// Load stock ranking data (both spot and futures)
    pub fn load_stock_rank(&self) { todo!() }
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
    pub fn state(&self) -> HomeStockState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

/// Interactor for the index components display.
pub struct IndexComponentsInteractor;
impl IndexComponentsInteractor {
    /// Create a new IndexComponentsInteractor with default dependencies.
    pub fn new_default() -> Arc<IndexComponentsInteractor> { todo!() }
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
    pub fn state(&self) -> IndexComponentsState { todo!() }
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
    /// Subscribe to index components for the given index.
    /// 
    /// Sets `is_loading = true`, spawns a persistent effect (cancels any previous
    /// subscription). The stream yields an HTTP snapshot first; on success state
    /// is updated and `is_loading` is cleared. If the HTTP frame returns an error
    /// the error is stored in state and the stream is terminated. Subsequent
    /// WebSocket frames update `data` in place.
    pub fn subscribe(&self, index: String) { todo!() }
    /// Subscribe by instrument. Resolves the index key (SPOT -> margin-gated
    /// instId, SWAP / FUTURES -> underlying) internally so native never derives
    /// it via the trade core, then delegates to [`subscribe`].
    pub fn subscribe_by_inst(&self, inst_id: String, inst_type: String) { todo!() }
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

/// Implemented by the native side (Kotlin/Swift) to surface a leak to the
/// developer, e.g. as a debug dialog. Invoked on a Rust worker thread — the
/// implementation must hop to its platform main thread before touching UI.
pub struct InteractorLeakReporter;
impl InteractorLeakReporter {
    /// `interactor_type` is the full Rust type path of the leaked interactor.
    pub fn on_leak(&self, interactor_type: String) { todo!() }
}

/// MainSearchInteractor - manages main search operations
pub struct MainSearchInteractor;
impl MainSearchInteractor {
    /// Default constructor. Event-contract AB / FR gates are read inside
    /// `MainSearchManager` from the crate-wide environment signals
    /// (`RetailTradingNativeApi` + e2e AB) — the former
    /// `new_with_provider(MainSearchPlatformProvider)` variant was retired
    /// with the provider trait.
    pub fn new_default() -> Arc<MainSearchInteractor> { todo!() }
    /// Constructor for native consumers that can resolve whether DEX Spot / Perp
    /// favorites are supported. Register the capability synchronously before any
    /// search or favorites subscription can start.
    pub fn new_with_dex_sub_type_support(supports_dex_sub_type: bool) -> Arc<MainSearchInteractor> { todo!() }
    pub fn clear(&self) { todo!() }
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
    pub fn search(&self, keyword: String) { todo!() }
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
    pub fn state(&self) -> MainSearchState { todo!() }
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
    pub fn subscribe_all_tab(&self, is_default_content_tab: bool) { todo!() }
    pub fn subscribe_announcement(&self) { todo!() }
    pub fn subscribe_bots(&self) { todo!() }
    pub fn subscribe_campaign(&self) { todo!() }
    pub fn subscribe_dex(&self) { todo!() }
    pub fn subscribe_earn(&self) { todo!() }
    pub fn subscribe_event_contract(&self) { todo!() }
    pub fn subscribe_futures(&self, sort_by: SortBy, sort_order: SortOrderEnum) { todo!() }
    pub fn subscribe_options(&self, exp_time: Option<String>, sort_by: SortBy, sort_order: SortOrderEnum) { todo!() }
    pub fn subscribe_p2p(&self) { todo!() }
    pub fn subscribe_spot(&self, sort_by: SortBy, sort_order: SortOrderEnum) { todo!() }
    pub fn subscribe_trader(&self) { todo!() }
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
    pub fn update_restriction_config(&self, is_social_auth: bool, is_support_copy_trading: bool, is_support_bot: bool, is_show_dex_trade: bool) { todo!() }
}

/// Interactor for the market flow card on the discover overview page.
pub struct MarketFlowInteractor;
impl MarketFlowInteractor {
    /// Create a default instance backed by the production HTTP client.
    pub fn new_default() -> Arc<MarketFlowInteractor> { todo!() }
    /// Fetch market flow data.
    /// 
    /// Should be called on initial load **and** whenever the user expands the
    /// card (to ensure fresh data in the expanded histogram view).
    /// 
    /// # Arguments
    /// * `zone`  - Price-change time convention derived from the user's
    /// `riseFallCycle` preference
    /// * `group` - Stable-coin group, e.g. `"USDT"`, read from
    /// `OKTrade.curStableCoin` on the native side
    pub fn load_flow(&self, zone: MarketFlowZone, group: String) { todo!() }
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
    pub fn state(&self) -> MarketFlowState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

/// Market number formatter.
/// 
/// Holds a BizManager reference to resolve instrument precision and currency
/// info. Returns `MarketFormatRecipe` for native to format with OKLocalization.
/// 
/// # Creation
/// 
/// ```ignore
/// // From BizManager (favorites — has all biz types)
/// let formatter = MarketFormatter::new_with_biz_manager(biz_manager);
/// 
/// // From individual Biz (discover/ranking — pass already-loaded biz)
/// let formatter = MarketFormatter::new_with_spot_biz(spot_biz);
/// 
/// // Without any Biz (fiat/change only, price uses default precision)
/// let formatter = MarketFormatter::new();
/// ```
pub struct MarketFormatter;
impl MarketFormatter {
    /// Create from the favorites BizManager global cache (no args).
    /// 
    /// Uses the BizManager populated by the latest watchlist API response.
    /// Falls back to `from_trading_core()` if the cache is empty (i.e., the
    /// watchlist API has not responded yet in this session).
    /// 
    /// This is the **recommended constructor for favorites pages** because
    /// the favorites flow may load before TradingCore initialises.
    pub fn from_favorite_biz() -> Arc<MarketFormatter> { todo!() }
    /// Create from TradingCore global singleton using already-loaded Biz.
    /// 
    /// This is the **recommended default** for most pages (Discover, Ranking, Nav Search).
    /// Synchronously reads cached Biz from CTRSTrade — does NOT trigger loading.
    pub fn from_trading_core() -> Arc<MarketFormatter> { todo!() }
    /// Create without any Biz.
    /// Fiat conversion and change ratio are fully available.
    /// Price precision inferred from server-provided value as fallback.
    pub fn new() -> Arc<MarketFormatter> { todo!() }
    /// Convert USD value to user's fiat currency, returning the raw numeric value.
    /// 
    /// Unlike `format_fiat_price` which returns a formatted string (with thousand separators
    /// and fixed fiat precision that truncates small values), this method returns the raw
    /// converted value preserving full precision. Use this when the result needs further
    /// computation (e.g., dynamic precision calculation via PrecisionUtils).
    pub fn convert_fiat_price_raw(&self, usd_value: String) -> MarketFormatRecipe { todo!() }
    /// Format a coin holding balance with precision from OkSpotBiz.
    /// 
    /// Looks up the coin's `sizeDig` from the global TradingCore coins map for
    /// decimal precision. Falls back to precision 2 if coin not found.
    /// Trailing zeros are kept (fixed precision) and rounding is HALF_DOWN to
    /// match native `FormatPnlDetailBalanceUseCase`.
    /// 
    /// Used by: spot analysis coin list (CurrentHoldings tab).
    pub fn format_balance(&self, value: String, coin_name: String) -> MarketFormatRecipe { todo!() }
    /// Format price change ratio.
    /// 
    /// Input: raw ratio string (e.g., "0.0345" = +3.45%).
    /// Output: value already multiplied by 100 (e.g., "3.45"), is_percentage=true.
    pub fn format_change(&self, ratio: String) -> MarketFormatRecipe { todo!() }
    /// Format DeFi change from 0-10000 scale.
    /// "345" -> same as format_change("0.0345")
    pub fn format_change_scaled(&self, scaled: String) -> MarketFormatRecipe { todo!() }
    /// Format every display field of a Crypto list item in one call.
    /// 
    /// Price is a USD value converted to the user's fiat. A `None` price yields
    /// no price recipe; `change` is always returned.
    pub fn format_crypto_item(&self, input: CryptoItemFormatInput) -> CryptoItemRecipes { todo!() }
    /// Format USD amount to abbreviated fiat value.
    pub fn format_fiat_amount(&self, usd_value: String) -> MarketFormatRecipe { todo!() }
    /// Format USD value to user's fiat currency.
    pub fn format_fiat_price(&self, usd_value: String) -> MarketFormatRecipe { todo!() }
    /// Format every display field of a Futures/Swap list item in one call.
    /// 
    /// Price uses `format_futures_price`, which owns the per-margin-type sign
    /// policy (linear: no sign; inverse: "$"). A `None` input field yields no
    /// recipe for that field; `change` is always returned.
    pub fn format_futures_item(&self, input: FuturesItemFormatInput) -> MarketListItemRecipes { todo!() }
    /// Format a futures/swap list price with the correct currency sign per margin type.
    /// 
    /// - Positive (USDT/USDC linear) contracts: instrument precision, NO currency sign.
    /// - Inverse (coin-margined, USD-quoted) contracts: USD precision + "$" prefix. The value
    /// is already a USD price, so it is NOT converted; the sign and precision both come from
    /// currency config (not hardcoded), reusing the same source as `InverseToUSDChargeConvertor`.
    /// 
    /// Keeping the per-margin-type sign policy in Rust (SSOT) lets iOS/Android render the recipe
    /// directly, without re-implementing the branch or hardcoding "$" on each platform.
    pub fn format_futures_price(&self, value: String, inst_family: String, inst_type: String, is_positive_contract: bool) -> MarketFormatRecipe { todo!() }
    /// Format every display field of an Options list item in one call.
    /// 
    /// `change` is `None` when native supplied no change ratio (native then
    /// renders its own "0.0%" placeholder); `turnover` defaults to "0" when
    /// absent, matching the native fallback.
    pub fn format_options_item(&self, input: OptionsItemFormatInput) -> OptionsItemRecipes { todo!() }
    /// Format a value as a percentage.
    /// 
    /// If `is_already_percentage` is true, input is already a percentage value
    /// (e.g. "5.23" for 5.23%) and is passed through as-is.
    /// If false, input is a ratio (e.g. "0.0523") and is multiplied by 100.
    /// 
    /// Returns: recipe with is_percentage=true, precision=2, rounding=up
    /// (Kotlin parity: OKTNumberF.formatICUPercent defaults to RoundingMode.UP).
    pub fn format_percentage(&self, value: String, is_already_percentage: bool) -> MarketFormatRecipe { todo!() }
    /// Format a value as a percentage with explicit rounding / sign policy.
    /// 
    /// Additive variant of `format_percentage` (existing export unchanged):
    /// - `rounding`: "down" | "up" | "half_up". `None` defaults to "up"
    /// (Kotlin parity: OKTNumberF.formatICUPercent default RoundingMode.UP).
    /// - `display_sign`: "auto" | "except_zero" | "always". `None` defaults to
    /// "auto" (Kotlin DisplaySign.AUTO — negative sign only). The value is
    /// also mirrored into `prepend_plus` for legacy recipe renderers.
    pub fn format_percentage_v2(&self, value: String, is_already_percentage: bool, rounding: Option<String>, display_sign: Option<String>) -> MarketFormatRecipe { todo!() }
    /// Format a plain numeric amount with abbreviation, no currency sign.
    pub fn format_plain_amount(&self, value: String) -> MarketFormatRecipe { todo!() }
    /// Format a PnL value with currency sign and color hint.
    /// 
    /// Returns a recipe where:
    /// - `value`: raw PnL string
    /// - `precision`: from the selected valuation currency metadata
    /// - `sign`: from TradingCore for USDT, or from settings for fiat
    /// - `sign_position`: "prefix"
    /// - `prepend_plus`: true (positive values show "+")
    /// - `rounding`: "half_down"
    /// 
    /// Rounding is HALF_DOWN to match the native spot-analysis PnL formatters
    /// (`FormatPnlDetailPnlUseCase`, the Holdings-PnL summary header, and the
    /// cumulative-PnL chart use cases) — all of which round HALF_DOWN.
    /// 
    /// Used by: spot analysis (chart PnL, coin PnL, holdings PnL summary).
    pub fn format_pnl_value(&self, value: String, valuation_unit: String) -> MarketFormatRecipe { todo!() }
    /// Format a holding's position market value (the valuation-unit sub-line of the Amount
    /// column) with the valuation-currency sign and precision.
    /// 
    /// Mirrors native `FormatPnlDetailPositionMarketValueUseCase`: currency sign
    /// prefix, valuation-currency precision, trailing zeros kept, rounded
    /// HALF_DOWN, and NO leading "+" (a market value is not a signed PnL).
    /// 
    /// Used by: spot analysis coin list (CurrentHoldings tab).
    pub fn format_position_value(&self, value: String, valuation_unit: String) -> MarketFormatRecipe { todo!() }
    /// Format instrument price.
    /// 
    /// Resolves precision and currency sign from BizManager using inst_family and inst_type.
    pub fn format_price(&self, value: String, inst_family: String, inst_type: String) -> MarketFormatRecipe { todo!() }
    /// Format a plain decimal ratio for display.
    /// 
    /// No currency sign, no abbreviation, no percentage.
    /// Used for: volume-to-market-cap ratio.
    pub fn format_ratio(&self, value: String) -> MarketFormatRecipe { todo!() }
    /// Format every display field of a Spot list item in one call.
    /// 
    /// Spot price carries no currency sign (mirrors the native V2 behavior that
    /// stripped `sign`/`sign_position` after `format_price`). A `None` input
    /// field yields no recipe for that field; `change` is always returned.
    pub fn format_spot_item(&self, input: SpotItemFormatInput) -> MarketListItemRecipes { todo!() }
    /// Format a futures range with the same linear/inverse precision policy as the row price,
    /// while omitting the currency sign shown only on price values.
    pub fn format_unsigned_futures_price(&self, value: Option<String>, inst_family: String, inst_type: String, is_positive_contract: bool) -> Option<MarketFormatRecipe> { todo!() }
    /// Format a price using Biz's instrument precision without adding its quote sign.
    /// The K-line header's 24h range is value-only even for inverse contracts.
    pub fn format_unsigned_price(&self, value: Option<String>, inst_family: String, inst_type: String) -> Option<MarketFormatRecipe> { todo!() }
    /// Format 24h volume with abbreviation.
    pub fn format_volume(&self, value: String, unit: String) -> MarketFormatRecipe { todo!() }
}

/// Interactor for the market trends card on the discover overview page.
pub struct MarketTrendsInteractor;
impl MarketTrendsInteractor {
    /// Create a default instance backed by the production HTTP client.
    pub fn new_default() -> Arc<MarketTrendsInteractor> { todo!() }
    /// Fetch historical chart data for the given metric, time window, and coin.
    /// 
    /// # Arguments
    /// * `chart_type` - Which metric to chart
    /// * `period`     - Time window for the chart
    /// * `coin`       - Coin code for dominance charts, e.g. `"BTC"`.
    /// Pass an empty string when `chart_type != Ratio`.
    pub fn load_chart(&self, chart_type: MarketChartType, period: MarketChartPeriod, coin: String) { todo!() }
    /// Fetch market cap, 24 h volume, and per-coin dominance data.
    /// 
    /// Sets `state.overview.is_loading = true` immediately, then updates
    /// `state.overview.data` (or `state.overview.error`) when the request
    /// completes.
    pub fn load_overview(&self) { todo!() }
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
    pub fn state(&self) -> MarketTrendsState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

pub struct NavSearchCategoryMixedListInteractor;
impl NavSearchCategoryMixedListInteractor {
    pub fn clear(&self) { todo!() }
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
    pub fn state(&self) -> NavSearchCategoryMixedListState { todo!() }
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
    pub fn subscribe(&self, subscription_id: u32, keyword: String, sort_by: SortBy, sort_order: SortOrderEnum) { todo!() }
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

pub struct NavSearchCategoryPageInteractor;
impl NavSearchCategoryPageInteractor {
    pub fn category(&self) -> FinanceCategoryTabVo { todo!() }
    pub fn create_futures_list_interactor(&self) -> Arc<NavSearchFuturesListInteractor> { todo!() }
    pub fn create_mixed_list_interactor(&self) -> Arc<NavSearchCategoryMixedListInteractor> { todo!() }
    pub fn create_spot_list_interactor(&self) -> Arc<NavSearchSpotListInteractor> { todo!() }
    pub fn futures_page(&self) -> FuturesPageVo { todo!() }
    pub fn has_futures_tab(&self) -> bool { todo!() }
    pub fn has_spot_tab(&self) -> bool { todo!() }
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
    pub fn search(&self, keyword: Option<String>) { todo!() }
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
    pub fn spot_page(&self) -> SpotPageVo { todo!() }
    /// Start the interactor lifecycle.
    /// 
    /// Calls the user's `InteractorLifecycle::start()` implementation.
    /// Override that method to add custom startup behavior (e.g., initial data loads).
    pub fn start(&self) { todo!() }
    /// Get the current state (UniFFI export).
    /// 
    /// Returns the External type (UniFFI-compatible).
    pub fn state(&self) -> NavSearchCategoryPageState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

pub struct NavSearchCryptoGroupInteractor;
impl NavSearchCryptoGroupInteractor {
    pub fn create_list_interactor(&self) -> Arc<NavSearchCryptoListInteractor> { todo!() }
    pub fn load_data(&self) { todo!() }
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
    pub fn state(&self) -> NavSearchCryptoGroupState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

pub struct NavSearchCryptoListInteractor;
impl NavSearchCryptoListInteractor {
    pub fn clear(&self) { todo!() }
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
    pub fn state(&self) -> NavSearchCryptoListState { todo!() }
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
    pub fn subscribe(&self, subscription_id: u32, category_group: Option<CategoryGroupVo>, sort_by: SortBy, sort_order: SortOrderEnum) { todo!() }
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

pub struct NavSearchDexGroupInteractor;
impl NavSearchDexGroupInteractor {
    pub fn create_list_interactor(&self) -> Arc<NavSearchDexListInteractor> { todo!() }
    pub fn load_data(&self, keyword: Option<String>) { todo!() }
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
    pub fn state(&self) -> NavSearchDexGroupState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

pub struct NavSearchDexListInteractor;
impl NavSearchDexListInteractor {
    pub fn clear(&self) { todo!() }
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
    pub fn state(&self) -> NavSearchDexListState { todo!() }
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
    pub fn subscribe(&self, subscription_id: u32, keyword: Option<String>, chain_group: Option<DexChainGroupVo>, period: Option<DexPeriod>, sort_by: Option<DexSortBy>, sort_order: SortOrderEnum) { todo!() }
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

pub struct NavSearchEventContractGroupInteractor;
impl NavSearchEventContractGroupInteractor {
    pub fn create_list_interactor(&self) -> Arc<NavSearchEventContractListInteractor> { todo!() }
    pub fn load_data(&self, keyword: Option<String>) { todo!() }
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
    pub fn state(&self) -> NavSearchEventContractGroupState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

pub struct NavSearchEventContractListInteractor;
impl NavSearchEventContractListInteractor {
    pub fn clear(&self) { todo!() }
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
    pub fn select_date_for_currency(&self, base_currency: String, date_label_ts: i64) { todo!() }
    /// Per-card date selection scoped to a single (base_currency, method, freq)
    /// series, so each above-like card (HIT-DAILY / HIT-MONTHLY / ABOVE / BETWEEN)
    /// remembers its own date independently. Prefer this over
    /// `select_date_for_currency` (a currency-wide fallback). `method` is taken as
    /// the `EventContractMethod` enum to avoid the as_str vs enum-name ambiguity
    /// (e.g. `PriceHit` -> "HIT").
    pub fn select_date_for_series(&self, base_currency: String, method: EventContractMethod, freq: String, date_label_ts: i64) { todo!() }
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
    pub fn state(&self) -> NavSearchEventContractListState { todo!() }
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
    /// Subscribe to keyword + (method, freq) filtered list.
    /// `freq=None` means accept all frequencies.
    pub fn subscribe(&self, subscription_id: u32, method: Option<EventContractMethod>, freq: Option<String>, keyword: Option<String>) { todo!() }
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

pub struct NavSearchExchangeOsGroupInteractor;
impl NavSearchExchangeOsGroupInteractor {
    /// Mint a market-list interactor (perp/spot/swap) sharing this Group's manager.
    pub fn create_market_list_interactor(&self) -> Arc<NavSearchExchangeOsMarketListInteractor> { todo!() }
    pub fn load_data(&self, keyword: Option<String>) { todo!() }
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
    pub fn state(&self) -> NavSearchExchangeOsGroupState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

pub struct NavSearchExchangeOsMarketListInteractor;
impl NavSearchExchangeOsMarketListInteractor {
    pub fn clear(&self) { todo!() }
    /// Fetch the next market-list page and append it to `items` (no-op unless `has_more`
    /// and no load-more is already in flight). Requires a prior `subscribe`.
    pub fn load_more(&self) { todo!() }
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
    pub fn state(&self) -> NavSearchExchangeOsMarketListState { todo!() }
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
    pub fn subscribe(&self, subscription_id: u32, tab: ExchangeOsBizTab, keyword: Option<String>, chain_group: Option<DexChainGroupVo>, filter: Option<String>, period: Option<DexPeriod>, sort_by: Option<DexSortBy>, sort_order: SortOrderEnum) { todo!() }
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

pub struct NavSearchFavoritesGroupInteractor;
impl NavSearchFavoritesGroupInteractor {
    pub fn add_favorite(&self, favorite: QuotationFavoriteVo, group_name: String) { todo!() }
    pub fn create_list_interactor(&self) -> Arc<NavSearchFavoritesListInteractor> { todo!() }
    pub fn load_data(&self, keyword: Option<String>) { todo!() }
    /// Stop-gap teardown (TEMPORARY — target state is the stream APIs,
    /// which need no teardown call).
    /// 
    /// One idempotent call at host lifecycle end that breaks the
    /// UniFFI handleMap cycle for any registered callback observers,
    /// ends active streams, and stops effects. On Kotlin call BEFORE
    /// `destroy()`. Includes `stop()`.
    pub fn release(&self) { todo!() }
    pub fn remove_favorite(&self, favorite: QuotationFavoriteVo, group_name: String) { todo!() }
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
    pub fn set_ui_event_observer(&self, observer: Arc<dyn NavSearchFavoritesGroupUxEffectObserver>) { todo!() }
    /// Start the interactor lifecycle.
    /// 
    /// Calls the user's `InteractorLifecycle::start()` implementation.
    /// Override that method to add custom startup behavior (e.g., initial data loads).
    pub fn start(&self) { todo!() }
    /// Get the current state (UniFFI export).
    /// 
    /// Returns the External type (UniFFI-compatible).
    pub fn state(&self) -> NavSearchFavoritesGroupState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
    /// Opt-in async UI event stream (cycle-free). Single-subscriber.
    pub fn ui_event_stream(&self) -> Arc<NavSearchFavoritesGroupUxEffectStream> { todo!() }
}

/// Deprecated UniFFI callback interface for observing UI events. Prefer the cycle-free `NavSearchFavoritesGroupUxEffectStream` returned by `interactor.ui_event_stream()`.
pub struct NavSearchFavoritesGroupUxEffectObserver;
impl NavSearchFavoritesGroupUxEffectObserver {
    pub fn on_ui_event(&self, event: NavSearchFavoritesGroupUxEffect) { todo!() }
}

/// Opt-in async event stream from `interactor.ui_event_stream()`.
/// Single-subscriber, latest-wins. `next()` resolves to `None` when the
/// interactor is dropped or a new subscription replaces this one.
pub struct NavSearchFavoritesGroupUxEffectStream;
impl NavSearchFavoritesGroupUxEffectStream {
    pub async fn next(&self) -> Option<NavSearchFavoritesGroupUxEffect> { todo!() }
}

pub struct NavSearchFavoritesListInteractor;
impl NavSearchFavoritesListInteractor {
    pub fn clear(&self) { todo!() }
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
    pub fn state(&self) -> NavSearchFavoritesListState { todo!() }
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
    pub fn subscribe(&self, subscription_id: u32, keyword: Option<String>, inst_type: Option<WatchlistInstType>, sort_by: SortBy, sort_order: SortOrderEnum) { todo!() }
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

pub struct NavSearchFuturesGroupInteractor;
impl NavSearchFuturesGroupInteractor {
    pub fn create_list_interactor(&self) -> Arc<NavSearchFuturesListInteractor> { todo!() }
    pub fn load_data(&self, keyword: Option<String>) { todo!() }
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
    pub fn state(&self) -> NavSearchFuturesGroupState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

pub struct NavSearchFuturesListInteractor;
impl NavSearchFuturesListInteractor {
    pub fn clear(&self) { todo!() }
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
    pub fn state(&self) -> NavSearchFuturesListState { todo!() }
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
    /// Subscribe to filtered and sorted futures search results stream.
    /// 
    /// Action form: fire-and-forget; a new call cancels the previous via
    /// `SpawnPolicy::AlwaysCancel`.
    pub fn subscribe(&self, subscription_id: u32, page_vo: FuturesPageVo, sort_by: SortBy, sort_order: SortOrderEnum, tickers_period: Option<TickersPeriodEnum>) { todo!() }
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

pub struct NavSearchFuturesPageInteractor;
impl NavSearchFuturesPageInteractor {
    /// Create a futures list interactor that shares the same manager instance.
    /// 
    /// This is the Page Interactor equivalent of
    /// `NavSearchFuturesGroupInteractor::create_list_interactor()`.
    pub fn create_list_interactor(&self) -> Arc<NavSearchFuturesListInteractor> { todo!() }
    /// `keyword`: Optional search keyword passed through to the manager and stored in state.
    pub fn load_data(&self, keyword: Option<String>) { todo!() }
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
    pub fn state(&self) -> NavSearchFuturesPageState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

/// NavSearchHistoryInteractor - manages navigation search history
/// 
/// Uses `SearchEntry::Navigation`, which maps to the dedicated storage key
/// `records_v2_nav`. This is fully isolated from global search history
/// (`records_v2_global`) and landscape search history (`records_v2_land`).
pub struct NavSearchHistoryInteractor;
impl NavSearchHistoryInteractor {
    pub fn new_default() -> Arc<NavSearchHistoryInteractor> { todo!() }
    /// Add a DEX token to history.
    pub fn add_dex(&self, item: SearchDexItem) { todo!() }
    /// Add a futures/swap contract to history.
    pub fn add_futures(&self, vo: QuotationFuturesItemVo) { todo!() }
    /// Add an options contract to history.
    pub fn add_options(&self, vo: QuotationOptionsItemVo) { todo!() }
    /// Add a SearchRecord directly.
    /// Used by Android when only inst_id / inst_type is available (e.g. from BizInstrument).
    pub fn add_record(&self, vo: SearchRecord) { todo!() }
    /// Add a spot trading pair to history.
    pub fn add_spot(&self, vo: QuotationSpotItemVo) { todo!() }
    pub fn clear_history(&self) { todo!() }
    pub fn load_data(&self) { todo!() }
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
    pub fn state(&self) -> NavSearchHistoryState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

pub struct NavSearchOptionsChainInteractor;
impl NavSearchOptionsChainInteractor {
    pub fn clear(&self) { todo!() }
    pub fn load_options_chain(&self, keyword: Option<String>) { todo!() }
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
    pub fn state(&self) -> NavSearchOptionsChainState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

pub struct NavSearchOptionsQuoteGroupInteractor;
impl NavSearchOptionsQuoteGroupInteractor {
    pub fn create_list_interactor(&self) -> Arc<NavSearchOptionsQuoteListInteractor> { todo!() }
    pub fn load_data(&self, keyword: Option<String>) { todo!() }
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
    pub fn state(&self) -> NavSearchOptionsQuoteGroupState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

pub struct NavSearchOptionsQuoteListInteractor;
impl NavSearchOptionsQuoteListInteractor {
    pub fn clear(&self) { todo!() }
    /// Get all available strike prices for a specific category group and expiration time.
    /// 
    /// Reads from cache without applying strike_price filter, so the result always
    /// contains the full set of strike prices for the given group + exp_time combination.
    pub async fn get_stk_price_list_by_group_and_exp_time(&self, category_group: OptionsCategoryGroupVo, exp_time: Option<String>) -> Vec<String> { todo!() }
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
    pub fn state(&self) -> NavSearchOptionsQuoteListState { todo!() }
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
    pub fn subscribe(&self, subscription_id: u32, keyword: Option<String>, category_group: Option<OptionsCategoryGroupVo>, exp_time: Option<String>, strike_price: Option<String>, direction: Option<OptionsDirection>, sort_by: SortBy, sort_order: SortOrderEnum) { todo!() }
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

pub struct NavSearchOptionsSimpleInteractor;
impl NavSearchOptionsSimpleInteractor {
    pub fn check_is_simple_options_and_copilot_enabled(&self) -> bool { todo!() }
    pub fn clear(&self) { todo!() }
    pub fn load_options_simple(&self, keyword: Option<String>) { todo!() }
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
    pub fn state(&self) -> NavSearchOptionsSimpleState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

pub struct NavSearchResultInteractor;
impl NavSearchResultInteractor {
    /// Clear all data and cancel all subscriptions.
    pub fn clear(&self) { todo!() }
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
    /// Main entry: search by keyword. Triggers Phase 1 (parallel load) then Phase 2 (subscriptions).
    pub fn search(&self, keyword: Option<String>) { todo!() }
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
    pub fn state(&self) -> NavSearchResultState { todo!() }
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
    /// **DEPRECATED — use `switch_derivatives_tab_v2` instead.**
    /// 
    /// Switch derivatives active tab (legacy 2-tab variant: futures / options).
    /// Retained for ABI stability; the forward implementation delegates to
    /// `switch_derivatives_tab_v2` with `is_event_active = false` so the
    /// Events sub-tab is always cleared on legacy entry. New callers MUST use
    /// `switch_derivatives_tab_v2` directly to drive the Events sub-tab.
    /// 
    /// Note: `#[deprecated]` attribute is intentionally NOT applied — the
    /// `#[interactor::actions]` macro forwards outer attrs to the generated
    /// inherent wrapper, where the wrapper-internal trait call becomes a
    /// hard error under `-D warnings`. Removing the attribute keeps ABI
    /// stable; future fix tracked at `interactor-macros/src/actions.rs`
    /// (wrap generated body in `#[allow(deprecated)]`).
    pub fn switch_derivatives_tab(&self, is_futures_active: bool, is_options_active: bool) { todo!() }
    /// Switch derivatives active tab — 3-tab variant covering Futures, Events,
    /// and Options. Each bool is independent so any one-of / all-off
    /// combination is representable.
    pub fn switch_derivatives_tab_v2(&self, is_futures_active: bool, is_event_active: bool, is_options_active: bool) { todo!() }
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
    /// Toggle "show all spot" mode. Updates state flag only (no re-subscription needed).
    pub fn toggle_show_all_spot(&self, show_all: bool) { todo!() }
    /// Update options quote filter and re-subscribe options quote list.
    pub fn update_options_quote_filter(&self, category_group: Option<OptionsCategoryGroupVo>, exp_time: Option<String>, strike_price: Option<String>, direction: Option<OptionsDirection>) { todo!() }
}

pub struct NavSearchRootInteractor;
impl NavSearchRootInteractor {
    pub fn new_default() -> Arc<NavSearchRootInteractor> { todo!() }
    pub fn new_with_metadata(metadata: String) -> Arc<NavSearchRootInteractor> { todo!() }
    /// Backward-compatible constructor carrying the EXPLICIT SearchKey scene-eligibility flag.
    /// 
    /// The native caller declares eligibility (never guessed from `metadata` or any fragment
    /// name). Standard navigation, Landscape and K-line-entry nav all participate in the
    /// multilingual SearchKey recall and pass `true`. `false` is reserved for scenes that are not
    /// yet confirmed for SearchKey (e.g. Finance-Category mixed search, Options, DEX, Event and
    /// contract-voucher entries): it makes the fixed Spot/Futures lines return an empty snapshot
    /// even for a non-empty keyword and skips the dictionary warm-up. `new_default` /
    /// `new_with_metadata` stay ENABLED, so existing callers are unaffected.
    pub fn new_with_metadata_and_search_key_scene(metadata: String, search_key_scene_enabled: bool) -> Arc<NavSearchRootInteractor> { todo!() }
    pub fn create_category_page_interactor(&self, category: FinanceCategoryTabVo) -> Arc<NavSearchCategoryPageInteractor> { todo!() }
    pub fn create_crypto_interactor(&self) -> Arc<NavSearchCryptoGroupInteractor> { todo!() }
    pub fn create_dex_interactor(&self) -> Arc<NavSearchDexGroupInteractor> { todo!() }
    pub fn create_event_contract_interactor(&self) -> Arc<NavSearchEventContractGroupInteractor> { todo!() }
    pub fn create_exchange_os_interactor(&self) -> Arc<NavSearchExchangeOsGroupInteractor> { todo!() }
    pub fn create_favorites_interactor(&self) -> Arc<NavSearchFavoritesGroupInteractor> { todo!() }
    pub fn create_futures_interactor(&self) -> Arc<NavSearchFuturesGroupInteractor> { todo!() }
    pub fn create_futures_page_interactor(&self) -> Arc<NavSearchFuturesPageInteractor> { todo!() }
    pub fn create_options_chain_interactor(&self) -> Arc<NavSearchOptionsChainInteractor> { todo!() }
    pub fn create_options_interactor(&self) -> Arc<NavSearchOptionsQuoteGroupInteractor> { todo!() }
    pub fn create_options_simple_interactor(&self) -> Arc<NavSearchOptionsSimpleInteractor> { todo!() }
    pub fn create_result_interactor(&self) -> Arc<NavSearchResultInteractor> { todo!() }
    pub fn create_spot_interactor(&self) -> Arc<NavSearchSpotGroupInteractor> { todo!() }
    pub fn create_spot_page_interactor(&self) -> Arc<NavSearchSpotPageInteractor> { todo!() }
    /// Create the TradFi business-line page. It shares this root's aggregated
    /// manager, hence the same market-tabs session, so its L2 strip resolves
    /// from the identical cloud config snapshot as the root tab bar.
    pub fn create_trad_fi_page_interactor(&self) -> Arc<NavSearchTradFiPageInteractor> { todo!() }
    pub fn load_biz_tabs(&self) { todo!() }
    pub fn load_coin_tabs(&self) { todo!() }
    /// Silently prefetch biz tabs in background without setting is_loading flag.
    pub fn prefetch_biz_tabs(&self) { todo!() }
    /// Silently prefetch coin tabs in background without setting is_loading flag.
    pub fn prefetch_coin_tabs(&self) { todo!() }
    /// Warm up the shared page-level multilingual Search Keys dictionary once on
    /// page entry so the fixed Spot and fixed Futures/SWAP lines share one snapshot
    /// and issue a single dictionary request. Independent of keyword search.
    pub fn prepare_search_keys(&self) { todo!() }
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
    pub fn state(&self) -> NavSearchRootState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

pub struct NavSearchSpotGroupInteractor;
impl NavSearchSpotGroupInteractor {
    pub fn create_list_interactor(&self) -> Arc<NavSearchSpotListInteractor> { todo!() }
    pub fn load_data(&self, keyword: Option<String>) { todo!() }
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
    pub fn state(&self) -> NavSearchSpotGroupState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

pub struct NavSearchSpotListInteractor;
impl NavSearchSpotListInteractor {
    pub fn clear(&self) { todo!() }
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
    pub fn state(&self) -> NavSearchSpotListState { todo!() }
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
    /// Subscribe to filtered and sorted spot search results stream.
    /// 
    /// Action form: fire-and-forget; a new call cancels the previous via
    /// `SpawnPolicy::AlwaysCancel`. Defensive: `SpotPageVo::Crypto` is
    /// not handled by this interactor and is ignored with an error log.
    pub fn subscribe(&self, subscription_id: u32, page_vo: SpotPageVo, sort_by: SortBy, sort_order: SortOrderEnum, tickers_period: Option<TickersPeriodEnum>) { todo!() }
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

pub struct NavSearchSpotPageInteractor;
impl NavSearchSpotPageInteractor {
    /// Create a spot list interactor that shares the same manager instance.
    /// 
    /// This is the Page Interactor equivalent of
    /// `NavSearchSpotGroupInteractor::create_list_interactor()`.
    pub fn create_list_interactor(&self) -> Arc<NavSearchSpotListInteractor> { todo!() }
    pub fn load_data(&self, keyword: Option<String>) { todo!() }
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
    pub fn state(&self) -> NavSearchSpotPageState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

pub struct NavSearchTradFiPageInteractor;
impl NavSearchTradFiPageInteractor {
    pub fn create_futures_list_interactor(&self) -> Arc<NavSearchFuturesListInteractor> { todo!() }
    /// Mixed Spot/Futures result list for the keyword search. It SHARES this
    /// page's scope and re-reads it on every subscribe, so creating the list
    /// before `load_data` finishes is safe — the first search after the scope
    /// lands already uses it.
    pub fn create_mixed_list_interactor(&self) -> Arc<NavSearchCategoryMixedListInteractor> { todo!() }
    pub fn create_spot_list_interactor(&self) -> Arc<NavSearchSpotListInteractor> { todo!() }
    /// Resolve the L2 tabs and the page-level business-line availability. Resolved
    /// ONCE per page: it comes from keyword-independent data and the tab strip must
    /// not move while the user types.
    pub fn load_data(&self) { todo!() }
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
    /// Mixed Spot/Futures hit count across every category of this line.
    pub fn search(&self, keyword: Option<String>) { todo!() }
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
    pub fn state(&self) -> NavSearchTradFiPageState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureFavoriteItemListWithFavoriteServiceError;
impl OKRustFutureFavoriteItemListWithFavoriteServiceError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<FavoriteServiceError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<Vec<RetailTradingFavoriteItem>> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), FavoriteServiceError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureGroupFavoriteListWithFavoriteServiceError;
impl OKRustFutureGroupFavoriteListWithFavoriteServiceError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<FavoriteServiceError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<Vec<RetailTradingGroupFavorite>> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), FavoriteServiceError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureSpotInstrumentListWithFavoriteServiceError;
impl OKRustFutureSpotInstrumentListWithFavoriteServiceError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<FavoriteServiceError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<Vec<QuotationSpotItemVo>> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), FavoriteServiceError> { todo!() }
}

/// Interactor for the position tier (trading rules) display.
pub struct PositionTierInteractor;
impl PositionTierInteractor {
    /// Create with default manager (production HTTP clients).
    pub fn new_default() -> Arc<PositionTierInteractor> { todo!() }
    /// Load public tier data for the given instrument.
    /// 
    /// # Parameters
    /// - `inst_type`: Instrument type (e.g. "SWAP", "FUTURES")
    /// - `inst_family`: Instrument family (e.g. "BTC-USDT")
    /// 
    /// # Flow
    /// 1. Set is_loading = true
    /// 2. Spawn async task (cancels previous)
    /// 3. Call CTPositionTiersManager::fetch_public_tier()
    /// 4. Map Vec<TierInfo> -> Vec<PositionTierItemVo>
    /// 5. Update state with results
    pub fn load(&self, inst_type: String, inst_family: String) { todo!() }
    /// Load public tier data by resolving inst_family from BizManagerCache.
    /// 
    /// # Parameters
    /// - `inst_id`: Instrument ID (e.g. "BTC-USDT-SWAP")
    /// - `inst_type`: Instrument type (e.g. "SWAP", "FUTURES")
    /// 
    /// # Flow
    /// 1. Resolve inst_family from BizManagerCache
    /// 2. Proceed with load(inst_type, inst_family)
    pub fn load_by_id(&self, inst_id: String, inst_type: String) { todo!() }
    /// Load public borrowing tiers for a Spot Margin pair.
    /// 
    /// `inst_id` and `ccy` are passed through unchanged. The request uses
    /// `instType=MARGIN` and cross margin mode to match native Android.
    pub fn load_margin(&self, inst_id: String, ccy: String) { todo!() }
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
    pub fn state(&self) -> PositionTierState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

pub struct PreMarketLaunchInteractor;
impl PreMarketLaunchInteractor {
    pub fn new_default() -> Arc<PreMarketLaunchInteractor> { todo!() }
    /// Load the full list (all filtered items, for the "View More" page).
    pub fn load_full_list(&self) { todo!() }
    /// Load the preview list (max 4 items). Resets state and fetches fresh data.
    pub fn load_preview(&self) { todo!() }
    /// Refresh the preview list (pull-to-refresh).
    pub fn refresh(&self) { todo!() }
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
    pub fn state(&self) -> PreMarketLaunchState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

pub struct RankingDetailInteractor;
impl RankingDetailInteractor {
    /// Create a new interactor with default dependencies.
    pub fn new_default() -> Arc<RankingDetailInteractor> { todo!() }
    /// Fetch ranking data for the given type and filters.
    /// 
    /// # Parameters
    /// - `rank_type`:         Which ranking list to fetch
    /// - `ccy`:               Currency filter ("USDT" | "USDC" | "Crypto" | …)
    /// - `zone`:              Price change display zone ("24h" | "utc0" | "utc8"),
    /// pass empty string to omit
    /// - `market_cap_filter`: Market cap level ("1"–"5"), pass empty string to omit
    /// - `period`:            Time period for turnover ("1D" | "5m" | "1H" | …),
    /// pass empty string to omit
    /// - `limit`:             Max items to return; pass 0 to use server default
    pub fn load_ranking(&self, rank_type: RankingDetailType, market_type: RankingDetailMarketType, ccy: String, zone: String, market_cap_filter: String, period: String, limit: u32) { todo!() }
    /// Fetch ranking overview: top items for all ranking categories.
    /// 
    /// # Parameters
    /// - `ccy`:  Currency filter ("USDT" | "USDC" | …)
    /// - `zone`: Price change display zone ("24h" | "utc0" | "utc8"),
    /// pass empty string to omit
    pub fn load_ranking_overview(&self, ccy: String, zone: String) { todo!() }
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
    pub fn state(&self) -> RankingDetailState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

/// Interactor for the spot analysis chart.
pub struct SaChartInteractor;
impl SaChartInteractor {
    /// Create a default instance backed by the production HTTP client.
    pub fn new_default() -> Arc<SaChartInteractor> { todo!() }
    /// Load cumulative PnL chart data.
    /// 
    /// # Arguments
    /// * `period` - Time period for the query
    /// * `valuation_unit` - Valuation currency (e.g. "USDT")
    /// * `uid` - User ID
    pub fn load(&self, period: AssetAnalysisPeriod, valuation_unit: String, uid: String) { todo!() }
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
    pub fn state(&self) -> SaChartState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

/// Interactor for the spot analysis coin list.
pub struct SaCoinListInteractor;
impl SaCoinListInteractor {
    /// Create a default instance backed by the production HTTP client.
    pub fn new_default() -> Arc<SaCoinListInteractor> { todo!() }
    /// Load coin PnL detail.
    /// 
    /// # Arguments
    /// * `segment` - Tab segment (CurrentHoldings or AllTime)
    /// * `valuation_unit` - Valuation currency (e.g. "USDT")
    /// * `uid` - User ID
    /// * `hide_low_value_ccy` - Whether to hide low-value coins (CurrentHoldings only)
    /// 
    /// The cost-price basis is sourced from core-trading inside the manager
    /// (Rust SSOT) — callers no longer pass it.
    pub fn load(&self, segment: SaSegmentType, valuation_unit: String, uid: String, hide_low_value_ccy: Option<bool>) { todo!() }
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
    pub fn state(&self) -> SaCoinListState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
    /// Toggle sort on the given column index.
    /// Re-sorts the current data in-place (no network call).
    pub fn toggle_sort(&self, column: u8) { todo!() }
}

/// SearchEntryRecommendInteractor - manages search entry recommendations
pub struct SearchEntryRecommendInteractor;
impl SearchEntryRecommendInteractor {
    pub fn new_default() -> Arc<SearchEntryRecommendInteractor> { todo!() }
    pub fn load_data(&self, size: Option<String>) { todo!() }
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
    pub fn state(&self) -> SearchEntryRecommendState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

/// SearchHistoryInteractor - manages search history
pub struct SearchHistoryInteractor;
impl SearchHistoryInteractor {
    pub fn new_default() -> Arc<SearchHistoryInteractor> { todo!() }
    /// Add an announcement to history.
    pub fn add_announcement(&self, item: SearchAnnouncementItem) { todo!() }
    /// Add a trading bot to history.
    pub fn add_bots(&self, item: SearchBotsItem) { todo!() }
    /// Add a campaign to history.
    pub fn add_campaign(&self, item: SearchCampaignItem) { todo!() }
    /// Add content (Feed/KOL) to history.
    pub fn add_content(&self, vo: SearchContentVo) { todo!() }
    /// Add a Planet resident (KOL) to history.
    /// 
    /// OKMarket's own search-overview KOL click path. Structurally identical to the
    /// 12 sibling `add_*` exports: it goes through `add_internal` -> `manager.add`
    /// (which applies the shared `content_kol_{kol_id}` dedup key, Global capacity,
    /// and persistence) -> `get_records` -> `filter_offline_records` -> state, so it
    /// carries its OWN UI refresh and does NOT depend on the change event bus.
    pub fn add_content_kol(&self, vo: SearchContentKolVo) { todo!() }
    /// Add a DEX token to history.
    pub fn add_dex(&self, item: SearchDexItem) { todo!() }
    /// Add an earn product to history.
    pub fn add_earn(&self, item: SearchEarnItem) { todo!() }
    /// Add an event contract to history.
    pub fn add_event(&self, vo: QuotationEventContractItemVo) { todo!() }
    /// Add a futures/swap contract to history.
    pub fn add_futures(&self, vo: QuotationFuturesItemVo) { todo!() }
    /// Add an options contract to history.
    pub fn add_options(&self, vo: QuotationOptionsItemVo) { todo!() }
    /// Add a P2P trading item to history.
    pub fn add_p2p(&self, item: SearchP2pItem) { todo!() }
    /// Add a spot trading pair to history.
    pub fn add_record(&self, vo: SearchRecord) { todo!() }
    /// Add a spot trading pair to history.
    pub fn add_spot(&self, vo: QuotationSpotItemVo) { todo!() }
    /// Add a copy trading leader to history.
    pub fn add_trader(&self, item: SearchTraderItem) { todo!() }
    pub fn clear_history(&self) { todo!() }
    pub fn load_data(&self) { todo!() }
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
    pub fn state(&self) -> SearchHistoryState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

/// SearchHotInteractor - manages hot search ranking subscription
pub struct SearchHotInteractor;
impl SearchHotInteractor {
    pub fn new_default() -> Arc<SearchHotInteractor> { todo!() }
    pub fn load_data(&self, quote_ccy: String, zone: String) { todo!() }
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
    pub fn state(&self) -> SearchHotState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

pub struct SelectionNavSearchFavoritesInteractor;
impl SelectionNavSearchFavoritesInteractor {
    pub fn new_default() -> Arc<SelectionNavSearchFavoritesInteractor> { todo!() }
    pub fn clear(&self) { todo!() }
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
    pub fn search(&self, keyword: String) { todo!() }
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
    pub fn state(&self) -> SelectionNavSearchFavoritesState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

pub struct SelectionNavSearchFuturesGroupInteractor;
impl SelectionNavSearchFuturesGroupInteractor {
    pub fn new_default() -> Arc<SelectionNavSearchFuturesGroupInteractor> { todo!() }
    /// Create a list interactor that shares the same manager.
    /// 
    /// The list interactor manages futures search results independently.
    pub fn create_list_interactor(&self) -> Arc<SelectionNavSearchFuturesListInteractor> { todo!() }
    pub fn load_groups(&self, keyword: Option<String>) { todo!() }
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
    pub fn state(&self) -> SelectionNavSearchFuturesGroupState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

pub struct SelectionNavSearchFuturesListInteractor;
impl SelectionNavSearchFuturesListInteractor {
    pub fn new_default() -> Arc<SelectionNavSearchFuturesListInteractor> { todo!() }
    pub fn clear(&self) { todo!() }
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
    pub fn search(&self, futures_type: SelectionFuturesTypeVo, category_group: Option<CategoryGroupVo>, unit: Option<FuturesUnitVo>) { todo!() }
    /// New API — page_vo carries category and (optional) unit;
    /// `futures_type` stays external because SelectionFuturesTypeVo
    /// has 6 variants (e.g., PerpetualOnly) which FuturesPageVo cannot express.
    pub fn search_v2(&self, page_vo: FuturesPageVo, futures_type: SelectionFuturesTypeVo) { todo!() }
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
    pub fn state(&self) -> SelectionNavSearchFuturesListState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

pub struct SelectionNavSearchSelectionInteractor;
impl SelectionNavSearchSelectionInteractor {
    pub fn new_default() -> Arc<SelectionNavSearchSelectionInteractor> { todo!() }
    pub fn new_with_config(config: SelectionConfigVo) -> Arc<SelectionNavSearchSelectionInteractor> { todo!() }
    pub fn batch_update(&self, instruments: Vec<QuotationFavoriteVo>) { todo!() }
    pub fn clear_selections(&self) { todo!() }
    /// Create a favorites interactor that shares the same manager.
    /// 
    /// The favorites interactor manages favorites/watchlist search results independently.
    pub fn create_favorites_interactor(&self) -> Arc<SelectionNavSearchFavoritesInteractor> { todo!() }
    /// Create a futures group interactor that shares the same manager.
    /// 
    /// The futures group interactor manages futures category groups and units.
    /// Use `create_list_interactor()` on the returned interactor to get a futures list interactor.
    pub fn create_futures_group_interactor(&self) -> Arc<SelectionNavSearchFuturesGroupInteractor> { todo!() }
    /// Create a spot group interactor that shares the same manager.
    /// 
    /// The spot group interactor manages spot category/charge groups.
    /// Use `create_list_interactor()` on the returned interactor to get a spot list interactor.
    pub fn create_spot_group_interactor(&self) -> Arc<SelectionNavSearchSpotGroupInteractor> { todo!() }
    pub fn initialize(&self, selected: Vec<QuotationFavoriteVo>, pre_selected: Vec<QuotationFavoriteVo>, filter_out: Vec<String>) { todo!() }
    pub fn refresh_selected_instruments(&self) { todo!() }
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
    pub fn state(&self) -> SelectionNavSearchSelectionState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
    pub fn toggle_selection(&self, instrument: QuotationFavoriteVo) { todo!() }
    pub fn toggle_selection_futures(&self, instrument: QuotationFuturesItemVo) { todo!() }
    pub fn toggle_selection_spot(&self, instrument: QuotationSpotItemVo) { todo!() }
    pub fn validate_selection(&self) { todo!() }
}

pub struct SelectionNavSearchSpotGroupInteractor;
impl SelectionNavSearchSpotGroupInteractor {
    pub fn new_default() -> Arc<SelectionNavSearchSpotGroupInteractor> { todo!() }
    /// Create a list interactor that shares the same manager.
    /// 
    /// The list interactor manages spot search results independently.
    pub fn create_list_interactor(&self) -> Arc<SelectionNavSearchSpotListInteractor> { todo!() }
    pub fn load_groups(&self, keyword: Option<String>) { todo!() }
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
    pub fn state(&self) -> SelectionNavSearchSpotGroupState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

pub struct SelectionNavSearchSpotListInteractor;
impl SelectionNavSearchSpotListInteractor {
    pub fn new_default() -> Arc<SelectionNavSearchSpotListInteractor> { todo!() }
    pub fn clear(&self) { todo!() }
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
    pub fn search(&self, category_group: Option<CategoryGroupVo>, charge_group: Option<ChargeGroupVo>) { todo!() }
    /// New API — accepts SpotPageVo with embedded filter info.
    pub fn search_v2(&self, page_vo: SpotPageVo) { todo!() }
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
    pub fn state(&self) -> SelectionNavSearchSpotListState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

pub struct TradeRadarInteractor;
impl TradeRadarInteractor {
    pub fn new_default() -> Arc<TradeRadarInteractor> { todo!() }
    /// Load next page (appends to existing items).
    pub fn load_more(&self) { todo!() }
    /// Load first page (resets pagination).
    /// 
    /// # Parameters
    /// - `radar_type`: Signal type filter (All, Whale, PriceSwing, Unlocks, Traders)
    /// - `zone`: "24h" | "utc0" | "utc8", pass empty to omit
    /// - `group`: Stable coin filter (e.g., "USDT")
    pub fn load_radar_list(&self, radar_type: TradeRadarType, zone: String, group: String) { todo!() }
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
    pub fn state(&self) -> TradeRadarState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

/// Interactor for the trading calendar monthly list page with date picker.
pub struct TradingCalendarListInteractor;
impl TradingCalendarListInteractor {
    /// Create a default instance backed by the production HTTP client.
    pub fn new_default() -> Arc<TradingCalendarListInteractor> { todo!() }
    /// Apply a filter to the already-loaded raw events without a network call.
    /// 
    /// Re-groups `raw_events` with the new filter and recalculates date items
    /// and the smart-selected index.
    /// 
    /// # Arguments
    /// * `filter` - New filter to apply
    pub fn apply_filter(&self, filter: CalendarFilterType) { todo!() }
    /// Load events for the given month and apply a filter.
    /// 
    /// Makes a network request, then groups and filters results.
    /// 
    /// # Arguments
    /// * `year` - Full year (e.g. 2026)
    /// * `month` - Month number 1–12
    /// * `filter` - Event type filter to apply
    pub fn load_month(&self, year: i32, month: u32, filter: CalendarFilterType) { todo!() }
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
    pub fn state(&self) -> TradingCalendarListState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

/// Interactor for the trading calendar four-month picker view.
pub struct TradingCalendarMonthInteractor;
impl TradingCalendarMonthInteractor {
    /// Create a default instance backed by the production HTTP client.
    pub fn new_default() -> Arc<TradingCalendarMonthInteractor> { todo!() }
    /// Load events for a four-month window centred around the given month.
    /// 
    /// The window spans: `start_month - 1` through `start_month + 2`.
    /// 
    /// # Arguments
    /// * `start_year` - Full year of the anchor month
    /// * `start_month` - Month number 1–12 (anchor month)
    pub fn load_four_months(&self, start_year: i32, start_month: u32) { todo!() }
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
    pub fn state(&self) -> TradingCalendarMonthState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

/// Interactor for the trading calendar overview card.
pub struct TradingCalendarOverviewInteractor;
impl TradingCalendarOverviewInteractor {
    /// Create a default instance backed by the production HTTP client.
    pub fn new_default() -> Arc<TradingCalendarOverviewInteractor> { todo!() }
    /// Load up to 3 upcoming events for the overview card.
    pub fn load_overview(&self) { todo!() }
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
    pub fn state(&self) -> TradingCalendarOverviewState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

/// OKMarket-facing facade over the [`TradingCore`] service. See the module doc
/// for the layering and the per-case contract.
pub struct TradingCoreBridge;
impl TradingCoreBridge {
    /// Production constructor. Resolves the wired [`TradingCore`] from the IoC
    /// container (the container auto-selects the mock impl under `cfg!(test)`).
    pub fn new() -> Arc<TradingCoreBridge> { todo!() }
    /// Warm up the trading core plus the given biz caches (`"SPOT"` / `"MARGIN"` /
    /// `"SWAP"` / `"FUTURES"` / `"OPTION"` / `"EVENTS"`; unknown ids are skipped).
    /// Idempotent — already-loaded bizes return immediately.
    /// 
    /// Replaces the Kotlin-facade warm-up (`getITradeManager().ensureInitialize()` +
    /// `getTrade().initTradeData()` + `getMinBiz(type).ensureInit()`), which dies with
    /// the facade: the sync getters on this bridge read already-loaded caches only, so
    /// a cold-start page awaits this once before its first read (or fire-and-forgets
    /// it and relies on `register_biz_notify` reload pushes to re-render). Best-effort:
    /// failures are logged, never surfaced — the page renders whatever the caches hold.
    pub async fn ensure_biz_loaded(&self, inst_types: Vec<String>) { todo!() }
    /// Instrument record by instId + instType (spot/swap/futures/option). Replaces
    /// Kotlin `getBiz(instType).getBizInfoForId(instId)` / `getWatchMarketInstrument(...)`.
    pub fn get_biz_info_for_id(&self, inst_id: String, inst_type: String) -> Option<InstrumentRecord> { todo!() }
    /// Legacy biz-type index (SPOT 0 / MARGIN 1 / SWAP 2 / FUTURES 3 /
    /// OPTION 4 / CEDEFI 5 / EVENTS 6 / other 0). Replaces Kotlin
    /// `MarketTradeTitleUtils.getBizType` == `TradeTitleUtils.getBizType`
    /// (OKMarket consumer: MarketNewListingBannerFragment).
    pub fn get_biz_type(&self, inst_type: String) -> i32 { todo!() }
    /// Quote-currency charge config for an inst-family (sign / precision / display
    /// name) used by local-currency price formatting.
    /// Replaces Kotlin `getBiz(instType).createChargeConverterWithIndex(instFamily)`.
    pub fn get_charge_currency_by_index(&self, inst_family: String, inst_type: String) -> Option<ChargeCurrencyRecord> { todo!() }
    /// Quote-currency charge groups (drives the USDC/USDT settle-currency banner
    /// variants). Replaces Kotlin `getTrade().getChargeGroupList()`.
    pub fn get_charge_group_records(&self) -> Vec<ChargeGroupRecord> { todo!() }
    /// Coin info record by symbol (name / url / tag / id). Replaces Kotlin
    /// `getTrade().getTradeCoinInfo(symbol)` / `coinInfoProvider().getCoinInfoBySymbol`.
    pub fn get_coin_info(&self, symbol: String) -> Option<TradeCoinInfoRecord> { todo!() }
    /// New-coin label flag by symbol. Replaces Kotlin `getTrade().getCoinIsNewLabel(symbol)`.
    pub fn get_coin_is_new_label(&self, symbol: String) -> bool { todo!() }
    /// New-coin tag flag by symbol. Replaces Kotlin `getTrade().getCoinIsNewTag(symbol)`.
    pub fn get_coin_is_new_tag(&self, symbol: String) -> bool { todo!() }
    /// Coin display name by symbol. Replaces Kotlin `getTrade().getCoinName(symbol)`.
    pub fn get_coin_name(&self, symbol: String) -> String { todo!() }
    /// Coin icon URL by symbol. Replaces Kotlin `getTrade().getCoinUrl(symbol)`
    /// (~49 call sites; hot per-row read).
    pub fn get_coin_url(&self, symbol: String) -> String { todo!() }
    /// Current fiat currency preference (iso code / symbol / display name; NO
    /// exchange rate — the symbol-only use case must not do rate math).
    /// Reads the cached settings currency preference; never hits the network.
    pub fn get_current_currency_info(&self) -> Option<CurrencyInfoRecord> { todo!() }
    /// Coin display symbol by symbol. Replaces Kotlin
    /// `coinInfoProvider().getDisplaySymbol(symbol)`.
    pub fn get_display_symbol(&self, symbol: String) -> String { todo!() }
    /// USDⓈ-margined options user-level gate.
    /// Replaces Kotlin `TradeCommonUtils.isEnableLinearOptions()`.
    pub fn get_enable_linear_options(&self) -> bool { todo!() }
    /// Futures contract groups (futures-only). Replaces Kotlin `getTrade().getFutureGroup()`.
    pub fn get_future_group_records(&self) -> Vec<FutureGroupInfoRecord> { todo!() }
    /// Suggested-instrument snapshot by instType + instId. Replaces Kotlin
    /// `getTrade().getSuggestedInstrument(instType, instId)` (nav-search seed /
    /// kline-monitor preselect reads). Returns `None` until the biz cache loads.
    pub fn get_instrument_brief(&self, inst_type: String, inst_id: String) -> Option<MarketInstrumentBrief> { todo!() }
    /// New-coin listing countdown record by instId. Replaces Kotlin
    /// `getSuggestedInstrument(...).getCountDownInfo()` (search hot-list rows).
    pub fn get_new_coin_countdown(&self, inst_id: String) -> Option<CountDownModelRecord> { todo!() }
    /// Simple display title. Replaces Kotlin
    /// `MarketTradeTitleUtils.getSimpleTitle(instId, instType)` (9 call sites:
    /// SearchHistoryFragmentV2 / NavSearchHistoryFragmentV2 /
    /// LandscapeNavSearchDefaultListAdapterV2 — all local-history rendering).
    /// SPOT/MARGIN render `"base/quote[ {n}X]"` from the loaded instrument
    /// (empty title when the instrument is not cached, matching Kotlin's null
    /// branch); FUTURES / SWAP / OPTION collapse to [`Self::get_trade_title`].
    pub fn get_simple_title(&self, inst_id: String, inst_type: String) -> CTTradeTitle { todo!() }
    /// Supported biz-type identifiers (e.g. `["SPOT", "SWAP", ...]`).
    /// Replaces Kotlin `getTrade().getSupportedBizType()`.
    pub fn get_supported_biz_types(&self) -> Vec<String> { todo!() }
    /// Swap contract groups (swap-only unit labels). Replaces Kotlin `getTrade().getSwapGroup()`.
    pub fn get_swap_group_records(&self) -> Vec<FutureGroupInfoRecord> { todo!() }
    /// Whether a coin is shown in trade context, keyed by instId + instType.
    /// Replaces Kotlin `getTrade().getTradeCoinIsShowByInstId(instId, instType)`.
    pub fn get_trade_coin_is_show_by_inst_id(&self, inst_id: String, inst_type: String) -> bool { todo!() }
    /// Whether a coin is shown in trade context, keyed by symbol.
    /// Replaces Kotlin `getTrade().getTradeCoinIsShowBySymbol(symbol)`.
    pub fn get_trade_coin_is_show_by_symbol(&self, symbol: String) -> bool { todo!() }
    /// User's stable-coin preference (defaults to `"USDT"`).
    /// Replaces Kotlin `getTrade().getTradeDataSettings().fetchStableCoin*()`.
    pub fn get_trade_setting_stable_coin(&self) -> String { todo!() }
    /// Preferred stable coin **display name** (falls back to the coin code when
    /// the trade config carries no display entry — legacy Kotlin parity).
    /// Replaces Kotlin `getTrade().getTradeDataSettings().fetchStableCoinDisplayName()`.
    pub fn get_trade_setting_stable_coin_display_name(&self) -> String { todo!() }
    /// Localized trade title with all-default options. Replaces Kotlin
    /// `TradeTitleUtils.getTitleByIdAndType(instId, instType)` /
    /// `MarketTradeTitleUtils.getTitleByIdAndType(instId, instType)`.
    /// Futures auto-route to X-Perps / Premarket from the loaded instrument
    /// (with an inst-family string fallback when the instrument is not
    /// cached). Native resolves each `DisplayString` segment
    /// (`DisplayStringExt.resolve(ctx)`) and joins them in order.
    pub fn get_trade_title(&self, inst_id: String, inst_type: String) -> CTTradeTitle { todo!() }
    /// Localized trade title with explicit options. Parameter mapping from the
    /// full Kotlin overload `getTitleByIdAndType(instId, instType, isSimple,
    /// isWatch, ruleType, swapWithSuffix, displayId, swapUseLongSuffix)`:
    /// - `simple` <- `isSimple` (swap/option compact form),
    /// - `rule_type` <- `ruleType` (see [`TradeTitleRuleType`]),
    /// - `swap_suffix` <- `swapWithSuffix` + `swapUseLongSuffix` collapsed
    /// (see [`TradeTitleSwapSuffix`]),
    /// - `isWatch` dropped — getTrade-vs-getSafeTrade cold-start crash guard
    /// is a Kotlin artifact; Rust `CTRSTrade` reads are always safe,
    /// - `displayId` dropped — resolved from the loaded instrument in Rust,
    /// - `eventFullName` dropped — EVENTS out of scope (see boundary above).
    /// 
    /// KNOWN UPSTREAM GAPS (recorded for the core-trading team; retail-trading
    /// deliberately does NOT modify core-trading or fork its title logic):
    /// 1. No XPERPS rule value yet — core-trading `FutureTitleIntent` lacks
    /// the variant (see the TODO on [`TradeTitleRuleType`]). The four
    /// OKMarket explicit-XPERPS call sites cannot migrate until it lands.
    /// 2. `GroupFilter` currently strips the X-Perp label (old Kotlin
    /// behavior); Kotlin `TradeTitleUtils.kt:269-277` now KEEPS the label
    /// (`convertXPerps(underlying, swapWithSuffix)`). No OKMarket site
    /// passes GROUP_FILTER today, so nothing regresses — but do not
    /// migrate an OKTrade GROUP_FILTER site until upstream aligns. A
    /// tripwire test pins the current behavior
    /// (`trade_title_delegation_tests::with_options_matrix` in
    /// `core/service/trading_core/real.rs`).
    pub fn get_trade_title_with_options(&self, inst_id: String, inst_type: String, simple: bool, rule_type: TradeTitleRuleType, swap_suffix: TradeTitleSwapSuffix) -> CTTradeTitle { todo!() }
    /// USD-margined settlement currency list (e.g. `["USDT", "USDC"]`).
    /// Replaces Kotlin `getTrade().tradeConfigInfoProvider().getUSDSettleCcyList()`
    /// on the settle-currency banners (search / discover / nav-search futures).
    pub fn get_usd_settle_ccy_list(&self) -> Vec<String> { todo!() }
    /// Whether the current account is in demo/simulate mode.
    /// Replaces Kotlin `getITradeManager().isSimulateLine()`
    /// (== `TradeAccountManager.isDemoMode()`).
    pub fn is_demo_trading(&self) -> bool { todo!() }
    /// MARGIN trading support gate (feature restriction + channel fallback).
    /// Replaces Kotlin `TradeUIService.getTradeConfig()?.isSupportMargin()`
    /// (`FeatureRestrictionConfig.isSupportMargin()`): FS margin sub-model
    /// present -> `!hidden`; FS-uninitialized window -> channel default
    /// `!(isTR || isLite)`. Semantics live in `TradingCore::is_support_margin`.
    pub fn is_support_margin(&self) -> bool { todo!() }
    /// Whether the instrument behind (instId, instType) belongs to the
    /// TradFi asset domain — the unified spot+futures classifier the TradFi
    /// navigation L1 preference (OKMarket A2) reads. See
    /// `TradingCore::is_tradfi_asset` for the resolution shape.
    pub fn is_tradfi_asset(&self, inst_id: String, inst_type: String) -> bool { todo!() }
    /// Whether the current account is a "trading account" (acctLv == "1").
    /// Replaces Kotlin `getTrade().getTradeDataSettings().isTradingAccount()`
    /// (via `RustTradeConfigInfoProvider`), consumed by
    /// `AssembleNavSearchResultUseCaseV2.isTradingAccount`.
    pub fn is_trading_account(&self) -> bool { todo!() }
    /// Whether xStock SPOT surfaces are visible for the current environment.
    /// 
    /// Delegates the entire decision and its minimal TradeSettingInfo readiness
    /// to core-trading through [`TradingCore`]. Runtime and policy failures are
    /// surfaced to the caller so each product boundary can choose its fallback.
    pub async fn is_xstock_spot_visible(&self) -> Result<bool, OKFFIError> { todo!() }
    /// Register a per-biz instrument-change listener (bizNotify). Returns an opaque
    /// handle; pass it to `unregister_listener` to stop. Returns 0 if the inst_type
    /// is unknown (nothing attached). Safe to call many times from many callers on a
    /// shared (singleton) bridge — each registration is independent.
    pub fn register_biz_notify(&self, inst_type: String, listener: Box<dyn TradingInstrumentChangeListener>) -> u64 { todo!() }
    /// Register the new-inst-countdown listener. Returns an opaque handle (0 if the
    /// event source is unavailable). Safe for multiple callers on a shared bridge.
    pub fn register_new_inst_countdown_listener(&self, listener: Box<dyn TradingNewInstCountdownListener>) -> u64 { todo!() }
    /// Stop and drop a previously-registered listener by its handle (from either
    /// `register_biz_notify` or `register_new_inst_countdown_listener`). A handle of
    /// 0 or an unknown handle is a no-op.
    pub fn unregister_listener(&self, handle: u64) { todo!() }
}

/// Callback for per-biz instrument-change notifications (bizNotify). Native
/// implements this to react to WS-driven instrument reloads.
pub struct TradingInstrumentChangeListener;
impl TradingInstrumentChangeListener {
    /// `data` = raw WS push JSON (only EVENTS biz carries Some(json); others None).
    pub fn on_instrument_changed(&self, data: Option<String>) { todo!() }
}

/// Callback for new-inst-countdown changes (maps to Android
/// NEW_COIN / CONTRACT_ONLINE_STATUS_UPDATE RxBus events).
pub struct TradingNewInstCountdownListener;
impl TradingNewInstCountdownListener {
    pub fn on_new_inst_countdown_changed(&self) { todo!() }
}

pub struct TrendingCategoriesListInteractor;
impl TrendingCategoriesListInteractor {
    pub fn new_default() -> Arc<TrendingCategoriesListInteractor> { todo!() }
    /// Load categories.
    /// - `summary_type`: "top" (overview, top 5) or "all" (full list)
    /// - `ccy_type`: Stable coin filter (e.g., "USDT")
    pub fn load_categories(&self, summary_type: String, ccy_type: String) { todo!() }
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
    pub fn state(&self) -> TrendingCategoriesListState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

pub struct TrendingCategoryDetailInteractor;
impl TrendingCategoryDetailInteractor {
    pub fn new_default() -> Arc<TrendingCategoryDetailInteractor> { todo!() }
    /// Synchronously initialize params before async subscription.
    /// Called from Android before subscribeInteractor to avoid race condition
    /// where async subscribe uses stale/empty params.
    pub fn init_params(&self, category_id: String, zone: String, ccy_type: String) { todo!() }
    /// Load the full crypto coin list for this category (A5 addition).
    /// Uses cached `category_id`, `zone`, `ccy_type` from `load_detail`.
    /// Added during migration to support the Android Crypto tab which fetches
    /// up to 350 items. Not present in iOS V1.
    pub fn load_crypto_list(&self) { todo!() }
    /// Load category detail info + initial top movers.
    /// - `category_id`: Category ID
    /// - `zone`: "24h" | "utc0" | "utc8", empty to omit
    /// - `ccy_type`: Stable coin (e.g., "USDT")
    pub fn load_detail(&self, category_id: String, zone: String, ccy_type: String) { todo!() }
    /// Reload top movers with a different period filter.
    /// - `period`: Time period filter
    pub fn load_top_movers(&self, period: TopMoversPeriod) { todo!() }
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
    pub fn state(&self) -> TrendingCategoryDetailState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

/// `TrendingTopicsInteractor` - manages the Global Search trending section.
pub struct TrendingTopicsInteractor;
impl TrendingTopicsInteractor {
    pub fn new_default() -> Arc<TrendingTopicsInteractor> { todo!() }
    /// One-shot load of the trending topics section.
    pub fn load(&self) { todo!() }
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
    pub fn state(&self) -> TrendingTopicsState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

pub struct UnlockTokenEventsInteractor;
impl UnlockTokenEventsInteractor {
    /// Create a new UnlockTokenEventsInteractor with default dependencies.
    /// 
    /// Each events interactor creates its own `UnlockTokensManager` instance.
    /// The events interactor does not need the color map, so it gets a fresh
    /// manager with an empty map.
    pub fn new_default() -> Arc<UnlockTokenEventsInteractor> { todo!() }
    /// Load all events for a given currency and type.
    /// 
    /// Single-shot load (pageSize=200, no real pagination).
    /// 
    /// # Arguments
    /// * `ccy` - Currency code
    /// * `event_type` - Event type filter (Upcoming/Past/Recently)
    pub fn load_events(&self, ccy: String, event_type: UnlockEventType) { todo!() }
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
    pub fn state(&self) -> UnlockTokenEventsState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

pub struct UnlockTokensInteractor;
impl UnlockTokensInteractor {
    /// Create a new UnlockTokensInteractor with default dependencies.
    pub fn new_default() -> Arc<UnlockTokensInteractor> { todo!() }
    /// Change schedule period and reload schedule chart data only.
    /// 
    /// Does NOT reload allocation, progress, or stages.
    /// Does NOT clear the color map (reuses existing colors).
    /// 
    /// # Arguments
    /// * `period` - New schedule period
    /// * `current_time_millis` - Device's current time in milliseconds
    pub fn change_schedule_period(&self, period: UnlockSchedulePeriod, current_time_millis: i64) { todo!() }
    /// Load all page data for a given currency.
    /// 
    /// Clears the color map, then spawns 3 parallel effects:
    /// - LoadProgressEffect (independent)
    /// - LoadStageCardsEffect (independent)
    /// - LoadAllocationAndScheduleEffect (allocation then schedule, sequential)
    /// 
    /// # Arguments
    /// * `ccy` - Currency code (e.g., "BTC")
    /// * `current_time_millis` - Device's current time in milliseconds
    pub fn load_page(&self, ccy: String, current_time_millis: i64) { todo!() }
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
    pub fn state(&self) -> UnlockTokensState { todo!() }
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
    /// Business teardown entry: logs the lifecycle event, delegates to the
    /// framework `release()`, and arms the debug leak watchdog (drains own
    /// adopted children; self-arms unless this instance was adopted). Call
    /// THIS (not `release()`) at host lifecycle end, BEFORE `destroy()`.
    pub fn teardown(&self) { todo!() }
}

/// Facade exposed to UniFFI native layers for WatchlistManager
/// 
/// This bridge provides a simplified interface for watchlist operations,
/// converting between native-friendly types and internal Rust types.
pub struct WatchlistManagerBridge;
impl WatchlistManagerBridge {
    pub fn new() -> Arc<WatchlistManagerBridge> { todo!() }
    /// Add a favorite to a specific group
    /// 
    /// # Arguments
    /// * `favorite` - FavoriteItem representing the favorite item
    /// * `group_name` - Optional name of the group to add the favorite to. If None, adds to watchlist
    /// 
    /// # Returns
    /// OKRustFuture<bool> - `true` if the favorite was successfully added, `false` otherwise
    pub fn add_favorite_to_group(&self, favorite: RetailTradingFavoriteItem, group_name: Option<String>) -> Arc<OKRustFutureboolWithOKFFIError> { todo!() }
    /// Add a favorite to the watchlist
    /// 
    /// # Arguments
    /// * `favorite` - FavoriteItem representing the favorite item
    /// 
    /// # Returns
    /// OKRustFuture<bool> - `true` if the favorite was successfully added, `false` otherwise
    pub fn add_favorite_to_watchlist(&self, favorite: RetailTradingFavoriteItem) -> Arc<OKRustFutureboolWithOKFFIError> { todo!() }
    /// Create a new group
    /// 
    /// # Arguments
    /// * `group_name` - The name of the group to create
    /// 
    /// # Returns
    /// OKRustFuture<bool> - `true` if the group was successfully created, `false` otherwise
    pub fn create_group(&self, group_name: String) -> Arc<OKRustFutureboolWithOKFFIError> { todo!() }
    /// Check if a favorite exists in any group
    /// 
    /// # Arguments
    /// * `favorite` - FavoriteItem representing the favorite item
    /// 
    /// # Returns
    /// OKRustFuture<bool> - `true` if the favorite exists in any group, `false` otherwise
    pub fn favorite_exists(&self, favorite: RetailTradingFavoriteItem) -> Arc<OKRustFutureboolWithOKFFIError> { todo!() }
    /// Find which group contains a specific favorite
    /// 
    /// # Arguments
    /// * `favorite` - FavoriteItem representing the favorite item
    /// 
    /// # Returns
    /// OKRustFuture<String> - JSON string representation of Option<String> (group name, or null if not found)
    pub fn find_group_for_favorite(&self, favorite: RetailTradingFavoriteItem) -> Arc<OKRustFutureStringWithOKFFIError> { todo!() }
    /// Get all groups with their favorites
    /// 
    /// # Returns
    /// OKRustFuture<Vec<GroupFavorite>> - List of all groups
    pub fn get_all_groups(&self) -> Arc<OKRustFutureGroupFavoriteListWithFavoriteServiceError> { todo!() }
    /// Get and subscribe to watchlist favorites with real-time ticker updates
    /// 
    /// This function fetches watchlist favorites for the specified group and subscribes
    /// to real-time ticker updates. Data is delivered through the callback interface.
    /// 
    /// # Arguments
    /// * `group_name` - The name of the watchlist group to fetch favorites from
    /// * `sort_by` - Field to sort by (Name, Price, Change, Volume, or None)
    /// * `sort_order` - Sort direction (Asc, Desc, or None)
    /// * `callback` - Callback interface for receiving updates
    /// 
    /// # Returns
    /// OKRustFuture<bool> - Resolves to true when stream completes successfully
    pub fn get_and_subscribe_watchlist_favorites(&self, group_name: String, sort_by: SortBy, sort_order: SortOrderEnum, callback: Arc<dyn WatchlistUpdateCallback>) -> Arc<OKRustFutureboolWithOKFFIError> { todo!() }
    /// Get favorites from a specific group
    /// 
    /// # Arguments
    /// * `group_name` - The name of the group
    /// 
    /// # Returns
    /// OKRustFuture<Vec<FavoriteItem>> - List of favorite items in the specified group
    pub fn get_favorites_by_group(&self, group_name: String) -> Arc<OKRustFutureFavoriteItemListWithFavoriteServiceError> { todo!() }
    /// Get a specific group by name
    /// 
    /// # Arguments
    /// * `group_name` - The name of the group
    /// 
    /// # Returns
    /// OKRustFuture<Vec<GroupFavorite>> - The group if found (as a list with one element), empty list otherwise
    pub fn get_group_by_name(&self, group_name: String) -> Arc<OKRustFutureGroupFavoriteListWithFavoriteServiceError> { todo!() }
    /// Get recommended favorites from the server
    /// 
    /// # Returns
    /// OKRustFuture<Vec<QuotationSpotItemVo>> - List of recommended spot instruments
    pub fn get_recommended_favorites(&self) -> Arc<OKRustFutureSpotInstrumentListWithFavoriteServiceError> { todo!() }
    /// Get the currently selected group
    /// 
    /// # Returns
    /// OKRustFuture<Vec<GroupFavorite>> - The selected group if one is set (as a list with one element), empty list otherwise
    pub fn get_selected_group(&self) -> Arc<OKRustFutureGroupFavoriteListWithFavoriteServiceError> { todo!() }
    /// Get favorites in the selected group
    /// 
    /// # Returns
    /// OKRustFuture<Vec<FavoriteItem>> - List of favorite items in the selected group
    pub fn get_selected_group_favorites(&self) -> Arc<OKRustFutureFavoriteItemListWithFavoriteServiceError> { todo!() }
    /// Check if a favorite exists in the watchlist
    /// 
    /// # Arguments
    /// * `favorite` - FavoriteItem representing the favorite item
    /// 
    /// # Returns
    /// `true` if the favorite exists, `false` otherwise
    pub fn is_favorite_in_watchlist(&self, favorite: RetailTradingFavoriteItem) -> Result<bool, FavoriteServiceError> { todo!() }
    /// Load the watchlist from the repository
    /// 
    /// Note: Since WatchlistManager::load_watchlist requires &mut self which is not supported
    /// in UniFFI, this method triggers data loading by calling get_all_groups() which will
    /// load the watchlist data if it's not already loaded.
    /// 
    /// # Returns
    /// OKRustFuture<bool> - `true` if the watchlist was successfully loaded, `false` otherwise
    pub fn load_watchlist(&self) -> Arc<OKRustFutureboolWithOKFFIError> { todo!() }
    /// Rearrange favorites within a group
    /// 
    /// # Arguments
    /// * `group_name` - The name of the group
    /// * `favorite_list` - A vector of favorite items in the desired order
    /// 
    /// # Returns
    /// OKRustFuture<bool> - `true` if the favorites were successfully rearranged, `false` otherwise
    pub fn rearrange_favorites(&self, group_name: String, favorite_list: Vec<RetailTradingFavoriteItem>) -> Arc<OKRustFutureboolWithOKFFIError> { todo!() }
    /// Rearrange the order of groups
    /// 
    /// # Arguments
    /// * `group_name_list` - A vector of group names in the desired order
    /// 
    /// # Returns
    /// OKRustFuture<bool> - `true` if the groups were successfully rearranged, `false` otherwise
    pub fn rearrange_groups(&self, group_name_list: Vec<String>) -> Arc<OKRustFutureboolWithOKFFIError> { todo!() }
    /// Reload the selected group from shared-preference using the key scoped
    /// to the current login state.
    /// 
    /// A background observer in `WatchlistManager` already calls this on
    /// login / logout / account-switch events, so native does **not** need
    /// to invoke it for those transitions. This remains exposed as an escape
    /// hatch for tests and for recovering from a missed event (e.g. if the
    /// observer was lagging during a burst of account activity).
    pub fn reload_selected_group_from_sp(&self) { todo!() }
    /// Remove a favorite from a specific group
    /// 
    /// # Arguments
    /// * `favorite` - FavoriteItem representing the favorite item
    /// * `group_name` - The name of the group to remove the favorite from
    /// 
    /// # Returns
    /// OKRustFuture<bool> - `true` if the favorite was successfully removed, `false` otherwise
    pub fn remove_favorite_from_group(&self, favorite: RetailTradingFavoriteItem, group_name: String) -> Arc<OKRustFutureboolWithOKFFIError> { todo!() }
    /// Remove a favorite from the watchlist
    /// 
    /// # Arguments
    /// * `favorite` - FavoriteItem representing the favorite item
    /// 
    /// # Returns
    /// OKRustFuture<bool> - `true` if the favorite was successfully removed, `false` otherwise
    pub fn remove_favorite_from_watchlist(&self, favorite: RetailTradingFavoriteItem) -> Arc<OKRustFutureboolWithOKFFIError> { todo!() }
    /// Set the currently selected group
    /// 
    /// # Arguments
    /// * `group_name` - The name of the group to select
    /// 
    /// # Returns
    /// `true` if the selected group was successfully set, `false` otherwise
    pub fn set_selected_group(&self, group_name: String) -> Result<bool, FavoriteServiceError> { todo!() }
}

/// Callback interface for watchlist favorites updates
/// 
/// Native platforms implement this trait to receive real-time watchlist updates.
pub struct WatchlistUpdateCallback;
impl WatchlistUpdateCallback {
    /// Called when watchlist favorites are updated with new data
    pub fn on_update_watchlist_batched(&self, favorites: Vec<QuotationFavoriteVo>) { todo!() }
    /// Called when the watchlist update stream is completed
    pub fn on_update_watchlist_completed(&self) { todo!() }
    /// Called when an error occurs while updating watchlist
    pub fn on_update_watchlist_error(&self, error: OKFFIError) { todo!() }
}

// -----------------------------------------------------------------------------
// 4. Top-Level Exported Functions (6)
// -----------------------------------------------------------------------------

/// Sectioned plain-text health report. v1 carries the interactor leak-watch adoption
/// tree; future sections (cache stats, WS subscriptions, task state, …) are appended
/// HERE without adding new exported symbols.
/// 
/// Read-only diagnostics with no side effects. In production the adoption registry is
/// still kept up to date (adoption bookkeeping is deliberately ungated) but nothing is
/// ever armed or reported, and the live inventory stays empty; the trigger itself is
/// registered on debuggable builds only anyway.
pub fn debug_health_report() -> String { todo!() }

/// Register (or replace) the process-wide leak reporter. Native sides call this
/// once at startup, ONLY in debug builds — registration is what switches the
/// whole watchdog (including the adoption registry) on. Last registration wins.
pub fn register_interactor_leak_reporter(reporter: Arc<dyn InteractorLeakReporter>) { todo!() }

/// Read the navigation account derivative-config (page-independent).
/// 
/// Constructs the capability with the real cefi transport by default, so the
/// production entry executes real HTTP (not a pure function production never
/// invokes). Returns the three raw-string fields per element; an empty list is a
/// valid success.
pub async fn retail_trading_get_nav_account_config(cancel_token: Arc<OKCancellationToken>) -> Result<Vec<NavDerivativeConfigVo>, NavAccountConfigError> { todo!() }

/// Record a Planet resident (KOL) into the GLOBAL search history.
/// 
/// Top-level UniFFI free function for the Orbit-team Planet-tab embedded page. It
/// constructs its own `SearchHistoryManager::new(SearchEntry::Global)` (reading the
/// latest disk state each call — deliberately NOT a long-lived shared singleton,
/// which would survive an account / real-vs-demo switch) and performs read ->
/// append & move-to-top -> write-back itself; it does NOT delegate to
/// `SearchHistoryInteractor`. It writes only — no navigation, no analytics — and is
/// idempotent by `kol_id` (shares CR-1's `content_kol_{kol_id}` dedup key). The
/// Global change event is published only after a successful write-back.
/// 
/// Call-and-return: no return value, no caller-side object ownership; a write
/// failure is logged and never blocks the caller.
pub fn retail_trading_record_global_search_kol(kol_id: String, kol_name: String) { todo!() }

/// Write one navigation account derivative-config entry (page-independent) and
/// return the server's resulting config list.
/// 
/// `config_value` is the raw string sent verbatim; a not-provided value is
/// `None` (never coerced to an empty string). The returned list (including
/// `show_config_times`) is surfaced to the caller — the write is not
/// fire-and-forget.
pub async fn retail_trading_set_nav_account_config(config_type: String, config_value: Option<String>, cancel_token: Arc<OKCancellationToken>) -> Result<Vec<NavDerivativeConfigVo>, NavAccountConfigError> { todo!() }

/// Register whether this client can render DEX Spot / Perp favorites.
/// 
/// Called by the native consumer whenever its gate resolves — app init, login /
/// logout, or an experiment refresh. Last write wins. Pass `false` for a guest, or
/// for a build that cannot render the DEX sub-types.
pub fn set_favorites_supports_dex_sub_type(supported: bool) { todo!() }
