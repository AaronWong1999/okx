// =============================================================================
// Reconstructed Rust UniFFI IDL & Schema for crate: `social_trading`
// Extracted from OKX Android v6.194.0 (lib/arm64-v8a/libOKCore.so)
// Total symbols decoded: 1567
//   Records: 275 | Enums: 127 | Interfaces: 154
//   Constructors: 17 | Methods: 919 | Functions: 74
// =============================================================================

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

// -----------------------------------------------------------------------------
// 1. Enums (127)
// -----------------------------------------------------------------------------

/// Display place for banners.
/// 
/// Determines where the banner should be displayed.
#[derive(Debug, Clone, PartialEq)]
pub enum BannerDisplayPlace {
    /// NMP (New Marketplace) display
    Nmp,
    /// Bot display
    Bot,
}

/// Banner layout type.
/// 
/// Determines the visual layout of the banner.
/// Backend sends as integer, not string (confirmed 2026-05-25); deserialised via `serde_repr` from the int discriminant.
#[derive(Debug, Clone, PartialEq)]
#[repr(i64)]
pub enum BannerLayoutType {
    /// Normal sized banner
    Normal = 1,
    /// Large sized banner
    Large = 2,
}

/// Banner link type.
/// 
/// Determines how the banner link should be opened.
#[derive(Debug, Clone, PartialEq)]
pub enum BannerLinkType {
    /// Internal link (opens within the app)
    Internal,
    /// External link (opens in external browser)
    External,
}

/// Result code for batch leverage setting operation.
#[derive(Debug, Clone, PartialEq)]
pub enum BatchSetLeverageResultCode {
    /// All instruments were successfully modified (result = "0").
    AllSuccess,
    /// Some instruments were modified, some failed (result = "1").
    PartialSuccess,
    /// All instruments failed to modify (result = "2").
    AllFailed,
    /// Unknown result code.
    Unknown,
}

/// Business line enum for trader follower list (UniFFI-compatible)
#[derive(Debug, Clone, PartialEq)]
pub enum BusinessLine {
    /// Contract/Futures trading
    Contract,
    /// Spot trading
    Spot,
}

/// Business line type for determining API endpoint
#[derive(Debug, Clone, PartialEq)]
pub enum CancelFollowBusinessLine {
    /// Contract/futures trading - uses /priapi/v5/ecotrade/copier/cancel-follow
    Contract,
    /// Spot trading - uses /priapi/v5/ecotrade/spot/copier/cancel-follow
    Spot,
}

/// Close position type enum
/// Corresponds to ClosePositionType enum in Kotlin
#[derive(Debug, Clone, PartialEq)]
pub enum ClosePositionType {
    /// Partial close
    /// JSON value: "1"
    Partial,
    /// Full close
    /// JSON value: "2"
    Full,
    /// Liquidated
    /// JSON value: "3"
    Liquidated,
    /// Force reduce
    /// JSON value: "4"
    ForceReduce,
    /// ADL (Auto-Deleveraging)
    /// JSON value: "5"
    Adl,
}

/// Content loading state - flat enum for readability on Native side.
/// 
/// Trade-off: +1.7KB package size for better Native code readability.
/// Native can use: `ContentLoadState.IDLE`, `ContentLoadState.LOADING`, etc.
#[derive(Debug, Clone, PartialEq)]
#[repr(i32)]
pub enum ContentLoadState {
    /// Initial state, no data loaded
    Idle = 0,
    /// Data is being loaded
    Loading = 1,
    /// Data loaded successfully
    Success = 2,
    /// Error occurred during loading
    Error = 3,
}

/// Load state for the leverage list (mirrors native ShowLoading/DismissLoading
/// + error toast collapsed into a single observable state).
#[derive(Debug, Clone, PartialEq)]
pub enum ContractLeverageContentState {
    Loading,
    Success,
    Error,
}

/// Content load state for the Copier Message page
#[derive(Debug, Clone, PartialEq)]
pub enum CopierMessageLoadState {
    /// Initial state, not loaded yet
    Idle,
    /// Loading data
    Loading,
    /// Data loaded successfully
    Success,
    /// Error occurred
    Error,
}

/// UI events for Copier Message.
#[derive(Debug, Clone, PartialEq)]
pub enum CopierMessageUiEvent {
    /// Data loaded successfully
    DataLoaded,
    /// Failed to load data
    DataLoadError {
        message: String,
    },
    /// Messages marked as read successfully
    MarkReadSuccess,
    /// Failed to mark messages as read
    MarkReadError {
        message: String,
    },
}

/// instType values for copytrading-pnl
#[derive(Debug, Clone, PartialEq)]
pub enum CopierPnlInstType {
    Swap,
    Spot,
}

/// Data type mapping to copyRelId
#[derive(Debug, Clone, PartialEq)]
pub enum CopierPnlLiveDataType {
    /// Total PnL only
    Home,
    /// Total PnL + all copied lead traders
    CopierDashboard,
    /// Specific copied lead trader
    CopyingTraderDetails {
        relation_id: String,
    },
}

/// Rise/Fall cycle mapping to timeZone
#[derive(Debug, Clone, PartialEq)]
pub enum CopierPnlRiseFallCycleType {
    Utc0,
    Utc8,
}

/// Trading mode for private WebSocket paths
#[derive(Debug, Clone, PartialEq)]
pub enum CopierPnlTradingMode {
    Live,
    Demo,
}

/// UI events for CopierPnlWs.
#[derive(Debug, Clone, PartialEq)]
pub enum CopierPnlWsUiEvent {
    /// WebSocket started successfully
    Started,
    /// WebSocket stopped
    Stopped,
    /// Snapshot data received
    SnapshotUpdated,
    /// Failed to start WebSocket
    StartError {
        message: String,
    },
}

/// One-time UI events emitted by the Copier Position History interactor.
#[derive(Debug, Clone, PartialEq)]
pub enum CopierPositionHistoryUiEvent {
    /// A history page was loaded successfully (initial reset or append).
    PageLoaded,
    /// Loading the history page failed.
    LoadError {
        message: String,
    },
}

/// One-time UI events emitted by the Copier Positions interactor.
#[derive(Debug, Clone, PartialEq)]
pub enum CopierPositionsUiEvent {
    /// Ongoing positions list loaded successfully.
    /// 
    /// `epoch` is the dispatch epoch of the load this terminal belongs to
    /// (see `ongoing_load`). Terminals can queue on the Holder's main hop;
    /// carrying the identity IN the event lets the Holder discard a terminal
    /// that predates its latest dispatch even when a newer terminal has
    /// already committed state — a later `state()` read cannot tell the two
    /// apart.
    OngoingLoaded {
        epoch: u64,
    },
    /// Failed to load ongoing positions list. Same `epoch` contract as
    /// [`CopierPositionsUiEvent::OngoingLoaded`].
    OngoingLoadError {
        message: String,
        epoch: u64,
    },
    /// A close action (`close_smart_position` / `close_custom_position`) succeeded.
    CloseSucceeded,
    /// A close action failed.
    CloseFailed {
        message: String,
    },
    /// Positions WebSocket started.
    PositionsWsStarted,
    /// Positions WebSocket stopped (with optional reason message).
    PositionsWsStopped {
        message: String,
    },
    /// Positions WebSocket pushed a snapshot — `ongoing_*_positions` updated.
    /// 
    /// Carries the diff between the previous and new snapshot so the Holder
    /// can shortcut its UI-model cache: only items in `added_ids` /
    /// `changed_ids` need re-mapping; items in `removed_ids` should be dropped
    /// from the cache. `added_ids` excludes closing items (avail_sub_pos == 0)
    /// — mirrors native `updateOrPrepend`'s `if (!shouldRemove)` guard.
    /// 
    /// All three lists are SUBSETS of `ongoing_*_positions` ids; the Holder
    /// reads the new state via [`get_state`] for the actual position payload.
    PositionsWsUpdated {
        added_ids: Vec<String>,
        changed_ids: Vec<String>,
        removed_ids: Vec<String>,
    },
    /// One or more positions appeared in this WS push that were NOT in the
    /// previous snapshot. The Holder should respond by calling
    /// [`refresh_coupon_flags`] — the WS frame doesn't carry compensation
    /// ticket data, so the new items currently have empty `compensation_*`
    /// fields and need a side-channel API call to populate them. Mirrors
    /// native `CurrentPositionViewModel.fetchOrderCouponState(newlyAdded)`.
    NewlyAddedItems {
        ids: Vec<String>,
    },
}

/// One-time events. The list flows through observable state; these cover the
/// native one-shot toast on load failure
/// (`CopyTradingSettingContractViewModel.kt:172` ToastError).
#[derive(Debug, Clone, PartialEq)]
pub enum CopyContractSettingsUiEvent {
    /// Load succeeded (list is in state).
    Loaded,
    /// Load failed. Per native parity the screen shows a FIXED localized
    /// network-error toast (`CopyTradingSettingContractViewModel.kt:172`), so
    /// `message` is intentionally empty — the consumer shows the localized
    /// network-error string. (The backend reason is logged, not surfaced.)
    LoadFailed {
        message: String,
    },
    /// `on_change_leverage` resolved: biz ready + ticker fetched. Native then shows the
    /// trade-core adjust-leverage dialog (`CopyTradingSettingContractViewModel.kt:393-400`
    /// → `showAdjustLeverageDialog`); the dialog UI stays native, this carries its inputs.
    ShowAdjustLeverageDialog {
        item: ContractLeverageItem,
        last: String,
        inst_id: String,
        pos_side: String,
        leverage: String,
        margin_mode: String,
    },
    /// Dismiss the loading indicator with no further UI. Mirrors native `DismissLoading`
    /// in both `on_change_leverage` (biz not ready / ticker unavailable, `:402,405`) and
    /// `on_bulk_edit` (biz not ready, `:330`).
    AdjustLeverageDismissed,
    /// Batch edit: NO selected instrument is under a special-lever restriction → all can take
    /// the batch lever (native `list.isEmpty() → LeverageSuccess`, `:341-346`). Native rebuilds
    /// the `leverageItems` from its synced `selectedLeverage`; this carries only `margin_mode`.
    LeverageSuccess {
        margin_mode: String,
    },
    /// Batch edit: EVERY selected instrument is under restriction → none can take the batch
    /// lever (native `list.size == selectedLeverage.size → LeverageFailed`, `:349`).
    LeverageFailed,
    /// Batch edit: SOME selected instruments are restricted (native `else →
    /// LeveragePartiallySuccess(noPassInstIds)`, `:351`). `no_pass_inst_ids` is the comma-joined
    /// set that cannot take the batch lever.
    LeveragePartiallySuccess {
        no_pass_inst_ids: String,
    },
    /// Batch-edit network failure. Native surfaces the backend message
    /// (`onFailure → ToastError(it.errorMessage)`, `:355`) — UNLIKE load (fixed string), here
    /// the backend message IS shown, so `message` carries `e.to_string()`.
    BulkEditError {
        message: String,
    },
}

/// Copy mode restriction options.
/// 
/// Determines which copy modes are available for followers.
/// Maps to Android `CopyModeRestriction` enum.
#[derive(Debug, Clone, PartialEq)]
#[repr(i32)]
pub enum CopyModeRestriction {
    /// All copy modes available (Smart sync + Custom)
    Unrestricted = 0,
    /// Only Smart Sync mode available
    SmartSyncOnly = 1,
}

/// Copy restriction mode for lead traders
/// 
/// Determines how copy traders can copy the lead trader's trades.
#[derive(Debug, Clone, PartialEq)]
pub enum CopyRestrictionMode {
    /// No restrictions - copy traders can use either Smart sync or Custom mode
    NoRestriction,
    /// Smart sync only - copy traders can only use Smart sync
    SmartOnly,
    /// Custom only - copy traders can only use Custom mode
    CustomOnly,
}

/// Copy mode options for copy trading.
/// 
/// Maps to iOS `CopyMode` / Android `CopyMode` enum.
#[derive(Debug, Clone, PartialEq)]
pub enum CopySettingsCopyMode {
    /// Fixed cost per trade
    FixedCost,
    /// Ratio-based copying
    RatioCopy,
    /// Smart sync (auto-managed)
    SmartCopy,
}

/// UI events for Copy Settings Spot.
#[derive(Debug, Clone, PartialEq)]
pub enum CopySettingsSpotUiEvent {
    DataLoaded,
    DataLoadError {
        message: String,
    },
    PreCheckPassed,
    PreCheckFailed {
        message: String,
    },
    PositionValueCheckPassed,
    PositionValueCheckWarning {
        message: String,
    },
    PositionValueCheckFailed {
        message: String,
    },
    FirstSettingsSuccess,
    FirstSettingsError {
        message: String,
        /// Backend business code, preserved end-to-end (60060 for a structured
        /// compliance requirement rejection; never collapsed to -1 for a real
        /// network error). `-1` only for non-network internal errors.
        code: i64,
        /// Structured compliance requirement (`data[0]`) when `code == 60060` and it
        /// parses; `None` otherwise (message-fallback). Option per ADR 0006 —
        /// added to a live event for backward compatibility.
        requirement: Option<CopySettingsRequirement>,
    },
    AmendSettingsSuccess,
    AmendSettingsError {
        message: String,
        /// See `FirstSettingsError::code`.
        code: i64,
        /// See `FirstSettingsError::requirement`.
        requirement: Option<CopySettingsRequirement>,
    },
    RecommendRatioLoaded,
    RecommendRatioError {
        message: String,
    },
    SmartSettingsLoaded,
    SmartSettingsError {
        message: String,
    },
    ReducibleAmountLoaded,
    ReducibleAmountError {
        message: String,
    },
    SmartAmountTransferSuccess,
    SmartAmountTransferError {
        message: String,
    },
    SmartAmendSuccess,
    SmartAmendError {
        message: String,
    },
    SmartMaxAvailableLoaded,
    SmartMaxAvailableError {
        message: String,
    },
    CancelCopyTradeSuccess,
    CancelCopyTradeError {
        message: String,
    },
    CancelCopyTradeHasPosition,
    CancelCopyTradeNoPosition,
    /// Cancel retention popup loaded. Native parity:
    /// `CopyTradingSettingHomeViewModel.handleCopyTraderRetentionFlow`.
    CancelPopupLoaded {
        responses: Vec<CopierRetentionPopupResponse>,
    },
    /// Cancel retention popup load failed — caller falls through to precheck.
    CancelPopupLoadError {
        message: String,
    },
    UsableCouponLoaded,
    UsableCouponError {
        message: String,
    },
    UsingCouponLoaded,
    UsingCouponError {
        message: String,
    },
    BindCouponSuccess,
    BindCouponError {
        message: String,
    },
}

/// UI events for Copy Settings Swap.
/// 
/// These are one-time events delivered via the interactor's `UiEventEmitter`.
/// The `#[interactor::event]` macro generates:
/// - `CopySettingsSwapUiEventObserver` trait (UniFFI callback interface)
/// - `CopySettingsSwapUiEventObserverAdapter` struct
#[derive(Debug, Clone, PartialEq)]
pub enum CopySettingsSwapUiEvent {
    /// Initial data loaded successfully
    DataLoaded,
    /// Data load failed
    DataLoadError {
        message: String,
    },
    /// Pre-check passed, can proceed with copy
    PreCheckPassed,
    /// Pre-check failed
    PreCheckFailed {
        message: String,
    },
    /// Position value check passed
    PositionValueCheckPassed,
    /// Position value check warning (can proceed but show warning)
    PositionValueCheckWarning {
        message: String,
    },
    /// Position value check failed
    PositionValueCheckFailed {
        message: String,
    },
    /// First settings submitted successfully
    FirstSettingsSuccess,
    /// First settings submission failed
    FirstSettingsError {
        message: String,
        /// Backend business code, preserved end-to-end (60060 for a structured
        /// compliance requirement rejection; never collapsed to -1 for a real
        /// network error). `-1` only for non-network internal errors.
        code: i64,
        /// Structured compliance requirement (`data[0]`) when `code == 60060` and it
        /// parses; `None` otherwise (message-fallback). Option per ADR 0006 —
        /// added to a live event for backward compatibility.
        requirement: Option<CopySettingsRequirement>,
    },
    /// Settings amended successfully
    AmendSettingsSuccess,
    /// Settings amend failed
    AmendSettingsError {
        message: String,
        /// See `FirstSettingsError::code`.
        code: i64,
        /// See `FirstSettingsError::requirement`.
        requirement: Option<CopySettingsRequirement>,
    },
    /// Recommend ratio loaded successfully
    RecommendRatioLoaded,
    /// Recommend ratio load failed
    RecommendRatioError {
        message: String,
    },
    /// Smart copy settings loaded successfully
    SmartSettingsLoaded,
    /// Smart copy settings load failed
    SmartSettingsError {
        message: String,
    },
    /// Smart reducible amount loaded successfully
    ReducibleAmountLoaded,
    /// Smart reducible amount load failed
    ReducibleAmountError {
        message: String,
    },
    /// Smart amount transfer succeeded
    SmartAmountTransferSuccess,
    /// Smart amount transfer failed
    SmartAmountTransferError {
        message: String,
    },
    /// Smart settings amended successfully
    SmartAmendSuccess,
    /// Smart settings amend failed
    SmartAmendError {
        message: String,
    },
    /// Smart max available amount loaded
    SmartMaxAvailableLoaded,
    /// Smart max available amount load failed
    SmartMaxAvailableError {
        message: String,
    },
    /// Cancel copy trade succeeded
    CancelCopyTradeSuccess,
    /// Cancel copy trade failed
    CancelCopyTradeError {
        message: String,
    },
    /// Cancel copy trade check: user has open positions (show cancel mode dialog)
    CancelCopyTradeHasPosition,
    /// Cancel copy trade check: no open positions (show simple cancel dialog)
    CancelCopyTradeNoPosition,
    /// Cancel retention popup loaded (may be empty when neither popup is
    /// visible — caller falls through to precheck). Native parity:
    /// `CopyTradingSettingHomeViewModel.handleCopyTraderRetentionFlow`.
    CancelPopupLoaded {
        responses: Vec<CopierRetentionPopupResponse>,
    },
    /// Cancel retention popup load failed — caller falls through to precheck
    /// (matches native onFailure branch in `handleCopyTraderRetentionFlow`).
    CancelPopupLoadError {
        message: String,
    },
    /// Usable coupon loaded successfully
    UsableCouponLoaded,
    /// Usable coupon load failed (non-fatal)
    UsableCouponError {
        message: String,
    },
    /// Using coupon loaded successfully
    UsingCouponLoaded,
    /// Using coupon load failed (non-fatal)
    UsingCouponError {
        message: String,
    },
    /// Coupon bound to trader successfully after first-copy
    BindCouponSuccess,
    /// Coupon binding failed (non-fatal, first-copy already succeeded)
    BindCouponError {
        message: String,
    },
}

/// Trade type for copy settings.
/// 
/// Maps to iOS `businessLine` parameter.
#[derive(Debug, Clone, PartialEq)]
#[repr(i32)]
pub enum CopySettingsTradeType {
    /// Contract/Futures trading (instType = "SWAP")
    Swap = 0,
    /// Spot trading (instType = "SPOT")
    Spot = 1,
}

/// UI events for the gatekeeper flow. Mirrors native `CopyTradeRetainedViewModel.UiEvent`
/// (minus dead events `DismissLoading`/`ErrorShortToast` which the VM never emits,
/// and `ShowLogin` which stays a Kotlin pre-guard).
#[derive(Debug, Clone, PartialEq)]
pub enum CopyTradeRetainedUiEvent {
    /// Loading started (= native `UiEvent.Loading`).
    Loading,
    /// New-copy success → open SmartCopyFlowFragment (= `StartCopyTrade`).
    StartCopyTrade {
        params: CopyFlowParams,
    },
    /// Manage success → open SocialCopyFlowManageActivity (= `ManageCopyTrade`).
    ManageCopyTrade {
        params: CopyFlowParams,
    },
    /// Custom-copy → open SocialCopyFlowSettingsActivity (= `NavigateToCustomCopySetting`).
    NavigateToCustomCopySetting {
        params: CopyFlowParams,
    },
    /// 59247 single-currency → switch-account dialog.
    ShowSwitchDialog,
    /// 59211 → KYC dialog (carries server message).
    ShowKycDialog {
        message: String,
    },
    /// 59205 → custom-copy upper-limit dialog.
    ShowUpperLimitDialog,
    /// 59266 → spot switch-account dialog.
    ShowSpotSwitchDialog,
    /// 59267 → spot account-mode-for-swap-smart-sync dialog.
    ShowSpotAccModeForSwapSmartSync,
    /// Copy-full dialog (new-copy eligibility; carries trader limit).
    ShowCopyFullDialog {
        limit_count: i32,
    },
    /// Single-position switch dialog (new-copy eligibility; carries trader limit).
    ShowSwitchPositionModeDialog {
        limit_count: i32,
    },
    /// Account-switch success toast (= native `ShowSwitchSuccessToast`).
    ShowSwitchSuccessToast,
    /// Generic error toast (else / non-OKServerException).
    ErrorToast {
        message: String,
    },
    /// Trader-settings load failed (= native SettingException branch). Kotlin
    /// fires COPYTRADINGSETTING_FULL_PAGE_VIEW tracking with this code, then toasts.
    SettingLoadError {
        code: i32,
        message: String,
    },
}

/// Instrument type for copytrading positions
#[derive(Debug, Clone, PartialEq)]
pub enum CopytradingPositionsInstType {
    Swap,
    Spot,
}

/// Trading mode for private WebSocket paths
#[derive(Debug, Clone, PartialEq)]
pub enum CopytradingPositionsTradingMode {
    Live,
    Demo,
}

/// Content load state for the Coupon Center page.
/// 
/// Maps to the native `ViewState<List<CouponCenterItemResponse>>` sealed class
/// (`ViewState.Loading` / `ViewState.Success` / `ViewState.Error`).
#[derive(Debug, Clone, PartialEq)]
pub enum CouponCenterLoadState {
    /// Initial state (matches native `ViewState.loading(true)` at VM init).
    Idle,
    /// Loading data (CouponCenterViewModel.kt:52-58).
    Loading,
    /// Data loaded successfully (CouponCenterViewModel.kt:73).
    Success,
    /// Error occurred (CouponCenterViewModel.kt:76).
    Error,
}

/// UI events for Coupon Center Interactor.
/// 
/// These mirror the side effects observable on the native VM after a successful
/// or failed load. Native VM does not emit explicit events (state-driven), so
/// consumers may prefer to ignore these and observe `content_state` transitions
/// on `CouponCenterState` directly. Kept here for symmetry with sibling
/// interactors (e.g. `CopierMessageInteractor`).
#[derive(Debug, Clone, PartialEq)]
pub enum CouponCenterUiEvent {
    /// Coupon list loaded successfully.
    ListLoaded,
    /// Coupon list failed to load.
    ListLoadError {
        message: String,
    },
}

/// Coupon item state enum
/// Corresponds to CouponItemState enum in Kotlin
/// Note: Simplified version - only serialization values are needed
#[derive(Debug, Clone, PartialEq)]
pub enum CouponItemState {
    /// Available
    /// JSON value: "0"
    Available,
    /// Guaranteed
    /// JSON value: "1"
    Guaranteed,
    /// Position opened
    /// JSON value: "2"
    PositionOpened,
    /// In progress
    /// JSON value: "3"
    InProgress,
    /// Paid
    /// JSON value: "4"
    Paid,
    /// Failed
    /// JSON value: "5"
    Failed,
    /// Expired
    /// JSON value: "6"
    Expired,
    /// Invalid
    /// JSON value: "7"
    Invalid,
    /// Empty (default)
    /// JSON value: ""
    Empty,
}

/// Content load state. Mirrors native `EcoHoldPositionViewModel.kt:23-24`
/// which initializes `_positionData` to `ViewState.loading(isFirstLoad = true)`,
/// so `Loading` is the default. There is no Idle.
#[derive(Debug, Clone, PartialEq)]
pub enum EcoHoldPositionLoadState {
    Loading,
    Success,
    Error,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EcoHoldPositionUiEvent {
    DataLoaded,
    DataLoadError {
        message: String,
    },
}

/// Content load state for the eco pnl weekly page
#[derive(Debug, Clone, PartialEq)]
pub enum EcoPnlWeeklyLoadState {
    Loading,
    Success,
    Error,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EcoPnlWeeklyUiEvent {
    DataLoaded,
    DataLoadError {
        message: String,
    },
}

/// Effect type - flat enum for minimal package size.
/// 
/// Each effect type corresponds to a one-shot event that Native should handle.
/// Payload data is embedded in `LeadTraderApplicationState` fields.
#[derive(Debug, Clone, PartialEq)]
#[repr(i32)]
pub enum EffectType {
    /// No effect
    None = 0,
    /// Pre-check passed, can proceed to application
    PreCheckPassed = 1,
    /// Pre-check failed with toast message (see effect_message)
    PreCheckNotPassedToast = 2,
    /// Pre-check failed with popup content (see effect_* fields)
    PreCheckNotPassedPopup = 3,
    /// Application submitted with auto approval
    SubmitAutoApproved = 10,
    /// Application submitted, pending manual review
    SubmitManualApproval = 11,
    /// Submit failed with error (see effect_message)
    SubmitError = 12,
    /// Affiliate ID is invalid
    AffiliateIdInvalid = 20,
    /// Affiliate ID is valid
    AffiliateIdValid = 21,
    /// Fetch instruments success - check state.instruments
    FetchInstrumentsSuccess = 30,
    /// Fetch instruments failed (see effect_message)
    FetchInstrumentsError = 31,
    /// Set instruments success
    SetInstrumentsSuccess = 32,
    /// Set instruments failed (see effect_message)
    SetInstrumentsError = 33,
    /// Audit fail popup closed successfully
    AuditFailPopupClosed = 40,
    /// Audit fail popup close failed (see effect_message)
    AuditFailPopupCloseError = 41,
    /// Popup status fetched - check effect_should_show_popup
    PopupStatusFetched = 50,
    /// Popup status fetch failed (see effect_message)
    PopupStatusFetchError = 51,
    /// Popup marked as read successfully
    PopupMarkedAsRead = 52,
    /// Popup mark as read failed (see effect_message)
    PopupMarkAsReadError = 53,
}

/// Business line for empty reminder operations (UniFFI-compatible)
/// 
/// Determines which API endpoint to use.
#[derive(Debug, Clone, PartialEq)]
pub enum EmptyReminderBusinessLineFFI {
    /// Contract/Swap trading
    Contract,
    /// Spot trading
    Spot,
}

/// Resolved follow/copy relationship state for a leader-rank row.
/// 
/// Sinks native `FollowTypeEnum.map(isCopying, isCopierFull, isSubscribedToReminder)`
/// (`SocialSquareViewModel.kt`) into Rust so the derivation is shared across platforms.
/// Filled by [`LeaderRankItem::enrich`].
#[derive(Debug, Clone, PartialEq)]
pub enum FollowState {
    /// Not copying and follower slots are still available (native `UNCOPY`).
    Uncopy,
    /// Currently copying this trader (native `COPYING`; `followType == "1"`).
    Copying,
    /// Full (no slots) and not subscribed to the empty-slot reminder (native `FULL`).
    Full,
    /// Full but subscribed to the empty-slot reminder (native `REMINDER_ON`).
    ReminderOn,
}

/// Content load state for the Follower List page
#[derive(Debug, Clone, PartialEq)]
pub enum FollowerListLoadState {
    /// Initial state, not loaded yet
    Idle,
    /// Loading data
    Loading,
    /// Data loaded successfully
    Success,
    /// Error occurred
    Error,
}

/// Sort order for follower list
#[derive(Debug, Clone, PartialEq)]
pub enum FollowerListSortType {
    /// No sort parameter (server default)
    None,
    /// Descending order (newest first) - default in native
    Desc,
    /// Ascending order (oldest first)
    Asc,
}

/// Trade type for Follower List
#[derive(Debug, Clone, PartialEq)]
pub enum FollowerListTradeType {
    /// Futures/Swap trading
    Futures,
    /// Spot trading
    Spot,
}

/// UI events for Follower List.
#[derive(Debug, Clone, PartialEq)]
pub enum FollowerListUiEvent {
    /// Data loaded successfully
    DataLoaded,
    /// Failed to load data
    DataLoadError {
        message: String,
    },
    /// Load more completed (client-side pagination)
    LoadMoreCompleted,
    /// Load more failed / no more data
    LoadMoreError {
        message: String,
    },
    /// Refresh completed
    RefreshCompleted,
    /// Refresh failed
    RefreshError {
        message: String,
    },
}

/// Sort type for followers list
#[derive(Debug, Clone, PartialEq)]
pub enum FollowerSortType {
    /// No sorting (default)
    None,
    /// Sort descending (high to low)
    Descending,
    /// Sort ascending (low to high)
    Ascending,
}

/// History position type enum
/// Corresponds to HistoryPosType enum in Kotlin
#[derive(Debug, Clone, PartialEq)]
pub enum HistoryPosType {
    /// Old position without handling fees
    /// JSON value: "1"
    OldPositionNoHandlingFees,
    /// Old position with handling fees
    /// JSON value: "2"
    OldPositionWithHandlingFees,
    /// New position after feature launch
    /// JSON value: "3"
    NewPositionPostFeatureLaunch,
    /// Invalid position type
    InvalidPosType,
}

/// Lead mode for lead traders
/// 
/// Determines whether the lead trader is public or private.
#[derive(Debug, Clone, PartialEq)]
pub enum LeadMode {
    /// Public mode - all traders can copy
    Public,
    /// Private mode - only invited traders can copy
    Private,
}

/// Trade type for lead trade crypto selection.
/// 
/// Determines which set of trading pairs to load (Futures or Spot).
#[derive(Debug, Clone, PartialEq)]
#[repr(i32)]
pub enum LeadTradeCryptoTradeType {
    /// Contract/Futures trading (instType = "SWAP", e.g., "BTC-USDT-SWAP")
    Futures = 0,
    /// Spot trading (instType = "SPOT", e.g., "BTC-USDT")
    Spot = 1,
}

/// UI events for Lead Trade Crypto.
/// 
/// These are one-time events delivered via the interactor's `UiEventEmitter`.
/// The `#[interactor::event]` macro generates:
/// - `LeadTradeCryptoUiEventObserver` trait (UniFFI callback interface)
/// - `LeadTradeCryptoUiEventObserverAdapter` struct
#[derive(Debug, Clone, PartialEq)]
pub enum LeadTradeCryptoUiEvent {
    /// Data loaded successfully
    DataLoaded,
    /// Data load failed
    DataLoadError {
        message: String,
    },
    /// Submit successful - native should close the screen
    SubmitSuccess,
    /// Submit failed
    SubmitError {
        message: String,
    },
    /// At least one coin must be selected
    SelectAtLeastOne,
}

/// Trade type for lead trade history.
/// 
/// Determines which set of historical trades to load (Futures or Spot).
#[derive(Debug, Clone, PartialEq)]
#[repr(i32)]
pub enum LeadTradeHistoryTradeType {
    /// Contract/Futures trading (instType = "SWAP")
    Futures = 0,
    /// Spot trading (instType = "SPOT")
    Spot = 1,
}

/// UI events for Lead Trade History.
/// 
/// These are one-time events delivered via the interactor's `UiEventEmitter`.
#[derive(Debug, Clone, PartialEq)]
pub enum LeadTradeHistoryUiEvent {
    /// First page loaded successfully
    DataLoaded,
    /// Load more completed (may have more pages or reached end)
    LoadMoreComplete {
        has_more: bool,
    },
    /// Error occurred
    Error {
        message: String,
    },
}

/// Business type enum for lead trader application (UniFFI export)
/// 
/// Using flat enum (#[repr(i32)]) to minimize package size (~1.7KB vs ~7KB).
/// 
/// Maps to Android `EcoApplyFrom` enum:
/// - ContractApply(0, "0") → Futures
/// - SpotApply(3, "1") → Spot
/// - StrategyApply(1, "2") → Strategy
/// - SignalProviderApply(2, "3") → SignalProvider
#[derive(Debug, Clone, PartialEq)]
#[repr(i32)]
pub enum LeadTraderApplicationBizType {
    /// Futures/Contract trading (bizType = "0")
    Futures,
    /// Spot trading (bizType = "1")
    Spot,
    /// Strategy trading (bizType = "2")
    Strategy,
    /// Signal Provider (bizType = "3")
    SignalProvider,
}

/// Business line for lead trader list.
#[derive(Debug, Clone, PartialEq)]
#[repr(i32)]
pub enum LeadTraderListBusinessLine {
    /// Contract/Futures trading
    Contract = 0,
    /// Spot trading
    Spot = 1,
}

/// UI events for Lead Trader List.
#[derive(Debug, Clone, PartialEq)]
pub enum LeadTraderListUiEvent {
    /// Data loaded successfully
    DataLoaded,
    /// Data load failed
    DataLoadError {
        message: String,
    },
    /// No more data to load
    NoMoreData,
    /// Trader selected for copy action
    TraderSelected,
    /// Selection failed (invalid index or uniqueName)
    SelectionError {
        message: String,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum LeadTraderPositionSide {
    Long,
    Short,
}

/// Trade type for lead trading settings.
/// 
/// Maps to Android `TradeTypes` enum.
#[derive(Debug, Clone, PartialEq)]
#[repr(i32)]
pub enum LeadTraderSettingsTradeType {
    /// Contract/Futures trading (instType = "SWAP")
    Futures = 0,
    /// Spot trading (instType = "SPOT")
    Spot = 1,
}

/// One-time UI events emitted by the Lead Trader Settings interactor.
#[derive(Debug, Clone, PartialEq)]
pub enum LeadTraderSettingsUiEvent {
    /// Data loaded successfully
    DataLoaded,
    /// Data load failed
    DataLoadError {
        message: String,
    },
    /// Private mode turned ON
    PrivateModeOn,
    /// Private mode turned OFF
    PrivateModeOff,
    /// Private mode update failed
    PrivateModeError {
        message: String,
    },
    /// Private mode update failed: reached max (error code 60018)
    PrivateModeErrorMaxReached {
        message: String,
    },
    /// Show private mode confirmation popup before toggling
    /// `desired_state` is "on" or "off"
    ShowPrivateModeConfirmation {
        desired_state: String,
    },
    /// Strategy protection turned ON
    StrategyProtectionOn,
    /// Strategy protection turned OFF
    StrategyProtectionOff,
    /// Strategy protection update failed
    StrategyProtectionError {
        message: String,
    },
    /// Copy mode restriction updated successfully
    CopyModeUpdated,
    /// Copy mode restriction update failed
    CopyModeError {
        message: String,
    },
    /// Min copy amount updated successfully
    MinCopyAmountUpdated,
    /// Min copy amount update failed
    MinCopyAmountError {
        message: String,
    },
    /// User has open positions, cannot exit
    ExitHasPosition,
    /// Exit application is under review
    ExitReviewing,
    /// Exit application submitted successfully
    ExitApplied,
    /// Exit failed
    ExitError {
        message: String,
    },
    /// Profit sharing ratio updated successfully
    ProfitSharingRatioUpdated,
    /// Profit sharing ratio update failed
    ProfitSharingRatioError {
        message: String,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum LeadTraderTone {
    Profit,
    Loss,
    Flat,
}

/// Business line enum for UniFFI export
#[derive(Debug, Clone, PartialEq)]
pub enum LeaderRankBusinessLine {
    /// Contract/Futures trading
    Contract,
    /// Spot trading
    Spot,
}

/// Trader-card time dimension resolved to a period.
/// 
/// Sinks native `String.timeDimMap2TimePeriod()` (`SocialSquareViewModel.kt:867`).
/// Filled by [`LeaderRankItem::enrich`].
#[derive(Debug, Clone, PartialEq)]
pub enum LeaderTimePeriod {
    /// `traderCardTimeDimension == "1"`.
    SevenDays,
    /// `traderCardTimeDimension == "2"`.
    ThirtyDays,
    /// `traderCardTimeDimension == "3"` (also the native fallback for any other value).
    NinetyDays,
    /// `traderCardTimeDimension == "4"`.
    OneYear,
}

/// Effect type for one-shot events
#[derive(Debug, Clone, PartialEq)]
pub enum ManageCopyTradersEffectType {
    /// No effect
    None,
    /// Data loaded successfully
    DataLoaded,
    /// Data load failed
    DataLoadError,
    /// Refresh completed
    RefreshCompleted,
    /// Refresh failed
    RefreshError,
    /// More data loaded
    MoreDataLoaded,
    /// More data load failed
    MoreDataLoadError,
    /// Copiers removed successfully
    CopiersRemoved,
    /// Copiers removal failed
    CopiersRemoveError,
    /// Copiers removal failed due to specific business reason (error_code 59220/59221)
    CopiersFailedToRemove,
}

/// Load state for the page
#[derive(Debug, Clone, PartialEq)]
pub enum ManageCopyTradersLoadState {
    /// Initial state, no data loaded
    Idle,
    /// Loading data
    Loading,
    /// Data loaded successfully
    Success,
    /// Error occurred
    Error,
}

/// Trade type for the manage copy traders page
#[derive(Debug, Clone, PartialEq)]
pub enum ManageCopyTradersTradeType {
    /// Futures/Swap trading (SWAP)
    Futures,
    /// Spot trading (SPOT)
    Spot,
}

/// Sort field for managed copiers
#[derive(Debug, Clone, PartialEq)]
pub enum ManagedCopierSortField {
    /// Sort by follow days
    FollowDays,
    /// Sort by PnL
    Pnl,
    /// Sort by invested amount (default)
    InvestedAmount,
    /// Sort by trade volume
    TradeVolume,
}

/// Sort order for managed copiers
#[derive(Debug, Clone, PartialEq)]
pub enum ManagedCopierSortOrder {
    /// Descending order (high to low)
    Descending,
    /// Ascending order (low to high)
    Ascending,
}

/// Marketplace banner type.
/// 
/// Determines the type of banner content.
#[derive(Debug, Clone, PartialEq)]
pub enum MarketplaceBannerType {
    /// Normal banner
    Normal,
    /// Voucher banner
    Voucher,
}

/// Content load state for the marketplace home page
#[derive(Debug, Clone, PartialEq)]
pub enum MarketplaceHomeLoadState {
    /// Initial state, not loaded yet
    Idle,
    /// Loading data
    Loading,
    /// Data loaded successfully
    Success,
    /// Error occurred
    Error,
}

/// User role type for the marketplace home page (UniFFI-compatible)
#[derive(Debug, Clone, PartialEq)]
pub enum MarketplaceHomeRoleType {
    /// Normal user
    Normal,
    /// Lead trader
    LeadTrader,
    /// Copier
    Copier,
}

/// UI events for Marketplace Home.
#[derive(Debug, Clone, PartialEq)]
pub enum MarketplaceHomeUiEvent {
    /// Data loaded successfully. Covers both initial load and pull-to-refresh — callers
    /// drive the visual distinction via their own state transition before invoking
    /// `load_data`. (A separate `RefreshCompleted` variant existed previously but the only
    /// consumer that distinguished the two was the TUI debug tool.)
    DataLoaded {
        message: String,
    },
    /// Failed to load data. Covers both initial-load and refresh failures.
    DataLoadError {
        message: String,
    },
    /// System requires app update
    ShowUpdateDialog {
        message: String,
    },
    /// Tier upgrade dialog should be shown
    ShowTierUpgradeDialog,
    /// Received coupons available for display
    ShowReceivedCoupons,
    /// Trader application approved popup
    ShowTraderApplicationApproved {
        biz_type: String,
    },
    /// Trader application status fetched
    TraderApplicationStatus {
        biz_type: String,
    },
    /// Voucher claimed successfully. `url` is the opaque navigation target supplied
    /// by the caller of `claim_voucher` — Rust does not interpret it; it is round-tripped
    /// back to the UI so the caller can navigate after a successful claim.
    VoucherClaimed {
        batch_no: String,
        url: String,
    },
    /// Voucher claim failed. The caller-supplied `url` is dropped on this branch — error
    /// handling is uniform (toast the message); the UI does not navigate on failure.
    VoucherClaimError {
        message: String,
    },
}

/// Business line type for marketplace trade info query
#[derive(Debug, Clone, PartialEq)]
pub enum MarketplaceTradeBusinessLine {
    /// Perpetual swap/futures trading
    Swap,
    /// Spot trading
    Spot,
}

/// UI events for MyCopiesCurrentTraders.
#[derive(Debug, Clone, PartialEq)]
pub enum MyCopiesCurrentTradersUiEvent {
    /// Data loaded successfully for a user-initiated `refresh`. `request_id`
    /// echoes the caller-supplied id (same contract as
    /// `TraderProfileRootUiEvent::ProfileLoaded`) so native can correlate the
    /// terminal event with the refresh command that produced it.
    DataLoaded {
        request_id: String,
    },
    /// A user-initiated `refresh` failed. Carries the same `request_id`
    /// correlation as `DataLoaded`.
    DataLoadError {
        request_id: String,
        message: String,
    },
    /// The internal WS-triggered unknown-trader fetch completed and state was
    /// merged. Carries NO request id by construction — it can never resolve a
    /// user refresh; consumers apply state and re-render only.
    InternalDataRefreshed,
    /// The internal WS-triggered unknown-trader fetch failed. Log-only
    /// downstream; never an error surface for a user refresh.
    InternalDataRefreshError {
        message: String,
    },
    /// WebSocket started successfully
    WsStarted,
    /// WebSocket stopped
    WsStopped,
    /// WebSocket failed to start
    WsStartError {
        message: String,
    },
    /// WebSocket snapshot received and items updated
    WsSnapshotUpdated {
        message: String,
    },
}

/// UI events for My Copies History List Spot.
#[derive(Debug, Clone, PartialEq)]
pub enum MyCopiesHistoryListSpotUiEvent {
    /// Data loaded successfully
    DataLoaded,
    /// Failed to load data
    DataLoadError {
        message: String,
    },
    /// No more data available
    NoMoreData,
    /// Row selected
    RowSelected,
    /// Profile selected
    ProfileSelected,
    /// Selection error (invalid index or unsupported action)
    SelectionError {
        message: String,
    },
}

/// UI events for My Copies History List Swap.
#[derive(Debug, Clone, PartialEq)]
pub enum MyCopiesHistoryListSwapUiEvent {
    /// Data loaded successfully
    DataLoaded,
    /// Failed to load data
    DataLoadError {
        message: String,
    },
    /// No more data available
    NoMoreData,
    /// Row selected
    RowSelected,
    /// Profile selected
    ProfileSelected,
    /// Selection error (invalid index or unsupported action)
    SelectionError {
        message: String,
    },
    /// Private info loaded successfully
    PrivateInfoLoaded,
    /// Failed to load private info
    PrivateInfoLoadError {
        message: String,
    },
}

/// Content load state for the My Followers page
#[derive(Debug, Clone, PartialEq)]
pub enum MyFollowersLoadState {
    /// Initial state, not loaded yet
    Idle,
    /// Loading data
    Loading,
    /// Data loaded successfully
    Success,
    /// Error occurred
    Error,
}

/// Trade type for My Followers
#[derive(Debug, Clone, PartialEq)]
pub enum MyFollowersTradeType {
    /// Futures/Swap trading
    Futures,
    /// Spot trading
    Spot,
}

/// UI events for My Followers.
#[derive(Debug, Clone, PartialEq)]
pub enum MyFollowersUiEvent {
    /// Data loaded successfully
    DataLoaded,
    /// Failed to load data
    DataLoadError {
        message: String,
    },
    /// More data loaded (pagination)
    MoreDataLoaded,
    /// Failed to load more data
    MoreDataLoadError {
        message: String,
    },
    /// Refresh completed
    RefreshCompleted,
    /// Refresh failed
    RefreshError {
        message: String,
    },
}

/// Copy mode for the lead trader relation.
#[derive(Debug, Clone, PartialEq)]
pub enum MyLeadTraderCopyMode {
    /// Fixed cost copy mode (default)
    FixedCost,
    /// Ratio copy mode
    RatioCopy,
    /// Smart copy mode
    SmartCopy,
}

/// Custom-copy ongoing-positions tab selection.
/// 
/// Moved here from `my_lead_trader_details::types` in Step 1.8 — only consumer
/// after the split is CopierPositions.
#[derive(Debug, Clone, PartialEq)]
pub enum MyLeadTraderCustomOngoingTab {
    /// Summary tab (default)
    Summary,
    /// Details tab
    Details,
}

/// One-time UI events emitted by the My Lead Trader Details interactor.
#[derive(Debug, Clone, PartialEq)]
pub enum MyLeadTraderDetailsUiEvent {
    /// Detail loaded successfully
    DetailLoaded,
    /// Detail load failed
    DetailLoadError {
        message: String,
    },
    /// Voucher loaded successfully
    VoucherLoaded,
    /// Voucher load failed
    VoucherLoadError {
        message: String,
    },
    /// PnL WebSocket started
    WsStarted,
    /// PnL WebSocket stopped
    WsStopped {
        message: String,
    },
    /// PnL WebSocket snapshot updated
    WsSnapshotUpdated,
    /// History list page loaded
    HistoryListLoaded,
    /// History list load failed
    HistoryListLoadError {
        message: String,
    },
}

/// Follow type for the lead trader relation.
#[derive(Debug, Clone, PartialEq)]
pub enum MyLeadTraderFollowType {
    /// Unfollowed
    Unfollow,
    /// Following (default)
    Following,
    /// Pending unfollow
    PendingUnfollow,
    /// Processing unfollow request
    ProcessingUnfollowRequest,
    /// Processing follow request
    ProcessingFollowRequest,
}

/// Message level for copier notifications
/// 
/// Maps to native MessageRecord.Level enum
#[derive(Debug, Clone, PartialEq)]
pub enum OKSTCopierMessageLevel {
    /// Informational message (level="1")
    Info,
    /// Warning message (level="2")
    Warn,
    /// Important message (level="3")
    Important,
    /// Any other level value
    Others,
}

/// Exported error type to native layer
/// 
/// TODO: Remove all error cases and use OKFFIError instead when we migrate all business logic to OKCore.
/// Note: Named OKSTFFIError to clearly indicate this is the FFI-exported error type
#[derive(Debug, Clone, PartialEq)]
pub enum OKSTFFIError {
    /// Network request failed - contains error details from OKNetworkError
    NetworkFailed {
        http_status: i32,
        business_code: i32,
        error_message: String,
    },
    /// Network request succeeded, but server data contains errors
    ServerResponseError {
        code: i32,
        error_message: String,
    },
    /// JSON parsing error
    JSONParseError {
        error_message: String,
    },
    /// Empty response data (valid HTTP 200 with empty array/object)
    EmptyData,
    /// Generic internal error for data mapping, validation, or other internal failures
    InternalError {
        error_message: String,
    },
}

/// Business line enum for ongoing lead trades (UniFFI-compatible)
#[derive(Debug, Clone, PartialEq)]
pub enum OngoingLeadTradesBusinessLine {
    /// Contract/Futures trading
    Contract,
    /// Spot trading
    Spot,
}

/// Instrument type filter for Ongoing Lead Trades
#[derive(Debug, Clone, PartialEq)]
pub enum OngoingLeadTradesInstFilter {
    /// Contract/Futures trading (SWAP)
    Swap,
    /// Spot trading
    Spot,
    /// All instrument types
    All,
}

/// Instrument type enum for ongoing lead trades (UniFFI-compatible)
#[derive(Debug, Clone, PartialEq)]
pub enum OngoingLeadTradesInstType {
    /// Contract/Futures trading (SWAP)
    Swap,
    /// Spot trading
    Spot,
    /// All instrument types
    All,
}

/// Content load state for Ongoing Lead Trades page
#[derive(Debug, Clone, PartialEq)]
pub enum OngoingLeadTradesLoadState {
    /// Initial state, not loaded yet
    Idle,
    /// Loading data
    Loading,
    /// Data loaded successfully
    Success,
    /// Error occurred
    Error,
}

/// Reminder banner type for the ongoing lead trades page.
/// 
/// Decision tree mirrors native `OrderListViewModel.initReminder` (lines 446-467).
/// Bridge maps each variant to its corresponding `R.string.*` resource:
/// 
/// | Variant | Native R.string |
/// |---|---|
/// | `None` | (no reminder shown) |
/// | `LeadTraderPrivateModeHidden` | `pr_social_leadtrader_reminder_desc_in_private_mode_hidden_data` |
/// | `OwnerSignalProtectHidden` | `pr_social_profile_reminder_desc_ongoing_data_hidden_lead` |
/// | `OwnerDataDelay` | `pr_social_profile_reminder_desc_ongoing_data_delay_lead` |
/// | `CopyFollowingDataDelay` | `pr_social_profile_reminder_desc_ongoing_data_delay` |
/// | `CopyTraderPrivateModeProtect` | `pr_social_copytraders_reminder_desc_trader_in_private_mode` |
/// | `CopyDataHidden` | `pr_social_profile_reminder_desc_ongoing_data_hidden_copy` |
#[derive(Debug, Clone, PartialEq)]
pub enum OngoingLeadTradesReminderType {
    /// No reminder banner should be shown.
    None,
    /// Owner viewing own Swap positions while in private-lead-trader mode (data hidden).
    LeadTraderPrivateModeHidden,
    /// Owner with signal-protect on — data hidden from copiers.
    OwnerSignalProtectHidden,
    /// Owner viewing own data — disclaimer about display delay.
    OwnerDataDelay,
    /// Viewer is copying — disclaimer about display delay.
    CopyFollowingDataDelay,
    /// Copier viewing a Swap trader in private mode with signal protect.
    CopyTraderPrivateModeProtect,
    /// Default for non-owner / non-copier case — data hidden notice.
    CopyDataHidden,
}

/// Sort mode for ongoing lead trades list.
/// 
/// Values mirror native constants in `com.okinc.unify_trade.trade.position.SortPositionUtils`:
/// `DEFAULT_SORT = 0`, `NOTIONAL_USD_SORT = 1`, `COIN_NAME_SORT = 2`.
#[derive(Debug, Clone, PartialEq)]
#[repr(i32)]
pub enum OngoingLeadTradesSortMode {
    /// Keep server-returned order
    Default = 0,
    /// Sort by notional USD descending (`sortListByNotionalUsd`, OrderLeadSortUtils.kt:35)
    NotionalUsdDesc = 1,
    /// Sort by coin symbol ascending (`sortListByCoinName`, OrderLeadSortUtils.kt:42)
    CoinName = 2,
}

/// Trading mode for the WS private path.
/// 
/// Mirrors `SubpositionsTradingMode` — `Live` → `/ws/v5/iprivate`,
/// `Demo` → `/ws/v5/iprivate?brokerId=9999`.
#[derive(Debug, Clone, PartialEq)]
pub enum OngoingLeadTradesTradingMode {
    Live,
    Demo,
}

/// UI events for Ongoing Lead Trades.
#[derive(Debug, Clone, PartialEq)]
pub enum OngoingLeadTradesUiEvent {
    /// Position summary loaded successfully
    SummaryLoaded,
    /// Failed to load position summary
    SummaryError {
        message: String,
    },
    /// Position detail loaded successfully
    DetailLoaded,
    /// Failed to load position detail
    DetailError {
        message: String,
    },
    /// Instruments loaded successfully
    InstrumentsLoaded,
    /// Failed to load instruments
    InstrumentsError {
        message: String,
    },
    /// WS started successfully (both detail + summary subscriptions open)
    PositionsWsStarted,
    /// WS stopped (either by user or by failure). `message` is empty on clean stop.
    PositionsWsStopped {
        message: String,
    },
    /// WS pushed a delta and the state was merged. Carries the diff so the bridge
    /// can patch per-item UI caches efficiently (skip remap for unchanged items).
    PositionsWsUpdated {
        /// Was the delta from the `subpositions` (detail) channel?
        /// `false` = sum-subpositions (summary) channel.
        is_detail: bool,
        added_ids: Vec<String>,
        changed_ids: Vec<String>,
        removed_ids: Vec<String>,
    },
    /// Close custom / lead-trade position succeeded.
    /// `message` is the server-provided toast text (may be empty when the
    /// server's response list had no string head — bridge should fall back
    /// to a localized generic in that case, mirroring native `EmptyString`).
    CustomCloseSucceeded {
        message: String,
    },
    /// Close custom / lead-trade position failed.
    CustomCloseFailed {
        error_code: i64,
        message: String,
    },
}

/// View mode for Ongoing Lead Trades
#[derive(Debug, Clone, PartialEq)]
pub enum OngoingLeadTradesViewMode {
    /// Viewing own positions (authenticated, no uniqueName needed)
    Self_,
    /// Viewing another trader's positions (public, requires uniqueName)
    Public,
}

/// Effect types for Partner Email Code Interactor
#[derive(Debug, Clone, PartialEq)]
pub enum PartnerEmailCodeEffectType {
    /// No effect (default)
    None,
    /// Email code sent successfully
    SendSuccess,
    /// Failed to send email code
    SendError,
    /// Not an affiliate user (error code 81001)
    NotAffiliateUser,
    /// Verification code error (error code 81004)
    VerificationCodeError,
}

/// Content load state for the partner email code action
#[derive(Debug, Clone, PartialEq)]
pub enum PartnerEmailCodeLoadState {
    /// Initial state, not sent yet
    Idle,
    /// Sending email code
    Loading,
    /// Email code sent successfully
    Success,
    /// Error occurred
    Error,
}

/// How the request center treats an empty API response array.
/// 
/// The two platforms genuinely differ here, and both behaviours must keep shipping:
/// 
/// * Android mirrors Kotlin's `List.first()` throwing `NoSuchElementException` on an
/// empty list (`EcoUserChartDataRepository.kt:61,74` -> `runOrErrorResponse` ->
/// `ApiResult.Error` -> `ViewState.error`), so an empty array is an **error**.
/// * iOS treats an empty array as a *successful empty*: `OKGTEPositionDistributionVC.m`
/// checks `[OKGTEUtils validateArray:]` (false for nil, non-array **or** `count == 0`)
/// and calls `showEmptyStatusView`, still marking the page as fully displayed.
/// 
/// The policy is opt-in and additive: the original
/// `create_eco_hold_position_interactor` export keeps `ErrorOnEmpty`, so Android's
/// exported factory and runtime behaviour are unchanged. iOS selects `EmptySuccess`
/// through `create_eco_hold_position_interactor_with_empty_policy` so the Rust branch
/// renders the same empty state as the legacy native branch.
#[derive(Debug, Clone, PartialEq)]
pub enum PositionScatterEmptyPolicy {
    /// Android parity: an empty API array becomes `Err` (mirrors Kotlin `.first()`).
    ErrorOnEmpty,
    /// iOS parity: an empty API array becomes `Ok(PositionHistoryScatter::default())`
    /// so the UI renders its own empty state.
    EmptySuccess,
}

/// Access level enum for currency preference chart
#[derive(Debug, Clone, PartialEq)]
pub enum PreferenceCoinAccessLevel {
    /// Public endpoint (viewing other traders)
    Public,
    /// Account endpoint (viewing own data)
    Account,
}

/// Business line enum for currency preference chart
#[derive(Debug, Clone, PartialEq)]
pub enum PreferenceCoinBusinessLine {
    /// Contract (futures) trading
    Contract,
    /// Spot trading
    Spot,
}

/// Instrument type for price limit check.
#[derive(Debug, Clone, PartialEq)]
pub enum PriceLimitInstType {
    /// Spot trading
    Spot,
    /// Swap (perpetual futures)
    Swap,
    /// Futures
    Futures,
    /// Margin
    Margin,
    /// All (Spot + Swap combined)
    All,
}

/// Profile chart type enum
/// Corresponds to ProfileChartTypeDto enum in Kotlin
#[derive(Debug, Clone, PartialEq)]
pub enum ProfileChartType {
    /// PNL chart type
    Pnl,
    /// PNL ratio chart type
    PnlRatio,
    /// Assets/Equity chart type
    Assets,
}

/// Trade type for profit settlement.
/// 
/// Maps to Android `TradeTypes` enum.
#[derive(Debug, Clone, PartialEq)]
#[repr(i32)]
pub enum ProfitSettlementTradeType {
    /// Contract/Futures trading (instType = "SWAP")
    Futures = 0,
    /// Spot trading (instType = "SPOT")
    Spot = 1,
}

/// UI events for Profit Settlement.
/// 
/// One-time events delivered via the interactor's `UiEventEmitter`.
/// The `#[interactor::event]` macro generates:
/// - `ProfitSettlementUiEventObserver` trait (UniFFI callback interface)
/// - `ProfitSettlementUiEventObserverAdapter` struct
#[derive(Debug, Clone, PartialEq)]
pub enum ProfitSettlementUiEvent {
    /// Profit info loaded successfully
    ProfitInfoLoaded,
    /// Profit info load failed
    ProfitInfoError {
        message: String,
    },
    /// First page of pre-profit list loaded
    PreProfitLoaded,
    /// Pre-profit load failed
    PreProfitError {
        message: String,
    },
    /// Additional page of pre-profit appended
    PreProfitPageAppended,
    /// Load-more pre-profit failed
    PreProfitPageError {
        message: String,
    },
    /// First page of history profit loaded
    HistoryProfitLoaded,
    /// History profit load failed
    HistoryProfitError {
        message: String,
    },
    /// Additional page of history profit appended
    HistoryProfitPageAppended,
    /// Load-more history profit failed
    HistoryProfitPageError {
        message: String,
    },
    /// First page of profit detail loaded
    ProfitDetailLoaded,
    /// Profit detail load failed (mirrors native `runCatching{}.onFailure`
    /// in DividendDetailsViewModel.loadProfitDetails)
    ProfitDetailError {
        message: String,
    },
    /// Additional page of profit detail appended
    ProfitDetailPageAppended,
    /// Load-more profit detail failed
    ProfitDetailPageError {
        message: String,
    },
    /// AUM dashboard info loaded
    AumInfoLoaded,
    /// AUM info load failed
    AumInfoError {
        message: String,
    },
    /// Ratio update in progress (show loading indicator)
    RatioUpdateLoading,
    /// Profit sharing ratio updated successfully
    RatioUpdated,
    /// Ratio update failed
    RatioError {
        message: String,
    },
    /// Navigate to profit sharing edit screen
    NavigateToEditProfitSharing,
    /// Failed to prepare edit profit sharing (ratio limit fetch failed).
    /// 
    /// `code` carries the server business code so the client can branch the same
    /// way the native limits-fetch `onError` did — iOS shows the profit-sharing
    /// terms dialog on 59289 (`OKTEErrorCodeTraderNotAcceptedTerms`) instead of a
    /// generic toast. `-1` means "no business code available" (non-HTTP/JSON
    /// failure), matching `OKNetworkError`'s own not-applicable convention.
    EditProfitSharingError {
        message: String,
        code: i32,
    },
}

/// Recommended copy mode enum
#[derive(Debug, Clone, PartialEq)]
pub enum RecommendedCopyModeDto {
    /// 0: Both custom and smart allowed
    Unrestricted,
    /// 1: Smart copy only
    Smart,
    /// 2: Custom copy only
    Custom,
}

/// Signal protection mode for lead traders
/// 
/// Determines whether signal protection is enabled.
#[derive(Debug, Clone, PartialEq)]
pub enum SignalProtectionMode {
    /// Signal protection is off
    Off,
    /// Signal protection is on
    On,
}

/// Content load state for smart copy trading page
#[derive(Debug, Clone, PartialEq)]
pub enum SmartCopyTradingLoadState {
    /// Initial state, not loaded yet
    Idle,
    /// Loading data
    Loading,
    /// Data loaded successfully
    Success,
    /// Error occurred
    Error,
}

/// UI events for Smart Copy Trading.
#[derive(Debug, Clone, PartialEq)]
pub enum SmartCopyTradingUiEvent {
    /// Settings loaded successfully
    SettingsLoaded,
    /// Failed to load settings
    SettingsLoadError {
        message: String,
    },
    /// Reducible amount loaded successfully
    ReducibleAmountLoaded,
    /// Failed to load reducible amount
    ReducibleAmountLoadError {
        message: String,
    },
    /// Transfer amount succeeded
    TransferAmountSuccess,
    /// Transfer amount failed
    TransferAmountError {
        message: String,
    },
    /// Transfer amount validation failed: amount is empty or zero
    TransferAmountEmptyOrZero,
    /// Transfer amount validation failed: increase amount exceeds available balance
    TransferIncreaseInsufficientBalance,
    /// Transfer amount validation failed: increase amount exceeds max increase limit
    TransferIncreaseExceedsMax {
        max_amount: String,
    },
    /// Transfer amount validation failed: decrease amount exceeds reducible limit
    TransferDecreaseExceedsReducible {
        reducible_amount: String,
    },
    /// Amend settings succeeded
    AmendSettingsSuccess,
    /// Amend settings failed
    AmendSettingsError {
        message: String,
    },
    /// Wallet available balance loaded into state (read it from `available_balance`)
    AvailableBalanceLoaded,
    /// Failed to load the wallet available balance
    AvailableBalanceLoadError {
        message: String,
    },
}

/// Transfer type for smart copy amount adjustment.
#[derive(Debug, Clone, PartialEq)]
pub enum SmartCopyTransferType {
    /// Increase investment (value: "2")
    Increase,
    /// Decrease investment (value: "3")
    Decrease,
}

/// Result of validating a transfer-amount input for LIVE (per-keystroke) UI
/// feedback, before the transfer is submitted.
/// 
/// Single source of truth shared by `transfer_amount` (submit guard) and the
/// synchronous `validate_transfer_input` (live validation). Non-`Valid` variants
/// map 1:1 to the corresponding `SmartCopyTradingUiEvent`. Rules mirror the
/// native `checkInput` logic in `IncreaseBalanceFragment` / `ReduceBalanceFragment`.
#[derive(Debug, Clone, PartialEq)]
pub enum SmartCopyTransferValidation {
    /// Input is valid; the transfer button may be enabled. `final_amount` is the
    /// post-transfer follow amount (current ± input), truncated to 2 dp — mirrors
    /// native `setAmendBalance` / `setReduceBalance` (`setScale(2, DOWN)`).
    Valid {
        final_amount: String,
    },
    /// Input is empty or zero — button disabled, no error text (native parity).
    EmptyOrZero,
    /// Increase amount exceeds the wallet available balance.
    IncreaseInsufficientBalance,
    /// Increase amount exceeds the max-increase limit (`max_amount`).
    IncreaseExceedsMax {
        max_amount: String,
    },
    /// Decrease amount exceeds the reducible amount (`reducible_amount`).
    DecreaseExceedsReducible {
        reducible_amount: String,
    },
}

/// Content load state for the safety checks
#[derive(Debug, Clone, PartialEq)]
pub enum SpotSafetyChecksLoadState {
    /// Initial state, not checked yet
    Idle,
    /// Checking price limit
    Loading,
    /// Check completed successfully
    Success,
    /// Error occurred during check
    Error,
}

/// One-time UI events emitted by the Spot Trade Safety Checks interactor.
#[derive(Debug, Clone, PartialEq)]
pub enum SpotSafetyChecksUiEvent {
    /// Price limit check passed (not triggered)
    CheckPassed,
    /// Price limit check triggered (price below limit)
    CheckTriggered {
        sell_limit: String,
    },
    /// Error during price limit check
    CheckError {
        msg: String,
    },
}

/// Channel type for subpositions WebSocket
#[derive(Debug, Clone, PartialEq)]
pub enum SubpositionsChannelType {
    /// Individual positions (subpositions channel)
    Detail,
    /// Aggregated/summarized positions by pair (sum-subpositions channel)
    Gather,
}

/// Instrument type for subpositions
#[derive(Debug, Clone, PartialEq)]
pub enum SubpositionsInstType {
    Swap,
    Spot,
}

/// Trading mode for private WebSocket paths
#[derive(Debug, Clone, PartialEq)]
pub enum SubpositionsTradingMode {
    Live,
    Demo,
}

/// Content load state for the total-asset page.
/// 
/// Mirrors native `TotalAssetViewModel.kt:19` which initializes
/// `_state` to `ViewState.loading(isFirstLoad = true)`, so `Loading` is the
/// default. There is no Idle.
#[derive(Debug, Clone, PartialEq)]
pub enum TotalAssetLoadState {
    Loading,
    Success,
    Error,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TotalAssetUiEvent {
    DataLoaded,
    DataLoadError {
        message: String,
    },
}

/// Result status for trader cancellation
#[derive(Debug, Clone, PartialEq)]
pub enum TraderCancelStatusResult {
    /// Pending approval
    Pending,
    /// Processed
    Processed,
    /// No audit info
    NoAuditInfo,
}

/// Result status for trader position query
#[derive(Debug, Clone, PartialEq)]
pub enum TraderPosResultStatus {
    /// No lead trading positions
    NoPositions,
    /// Has lead trading positions
    HasPositions,
}

/// One-time UI events emitted by the Trader Profile Root interactor.
#[derive(Debug, Clone, PartialEq)]
pub enum TraderProfileRootUiEvent {
    /// Profile loaded successfully. `request_id` echoes the initiating load/refresh
    /// command's client-supplied id (`TraderProfileRootLoadInput::request_id`) so
    /// native can correlate this terminal with the exact load cycle and drop stale
    /// terminals from superseded loads — same contract as `FollowUpdated`.
    ProfileLoaded {
        request_id: String,
    },
    /// Profile load failed (or a degraded load's paired trade-info error). Carries
    /// the same `request_id` correlation as `ProfileLoaded` (see above).
    ProfileLoadError {
        request_id: String,
        message: String,
    },
    /// Follow state updated successfully (message is server response).
    /// `request_id` echoes the input's client-supplied id so native can correlate
    /// this terminal with the exact request and drop stale/superseded terminals.
    FollowUpdated {
        request_id: String,
        message: String,
    },
    /// Follow update failed. Carries the same `request_id` correlation as
    /// `FollowUpdated` (see above).
    FollowUpdateError {
        request_id: String,
        message: String,
    },
    /// Empty reminder updated successfully. `request_id` echoes the input's
    /// client-supplied id so native can correlate this terminal with the exact
    /// request and drop stale/superseded terminals (same contract as `FollowUpdated`).
    EmptyReminderUpdated {
        request_id: String,
    },
    /// Empty reminder update failed. Carries the same `request_id` correlation as
    /// `EmptyReminderUpdated` (see above).
    EmptyReminderUpdateError {
        request_id: String,
        message: String,
    },
}

/// Business type enum for trader trade performance (UniFFI-compatible)
#[derive(Debug, Clone, PartialEq)]
pub enum TraderTradePerformanceBizType {
    /// Contract/Futures trading (SWAP)
    Contract,
    /// Spot trading
    Spot,
}

/// Business line for vanguard share data
#[derive(Debug, Clone, PartialEq)]
pub enum VanguardShareBusinessLine {
    /// Contract/Futures trading
    Contract,
    /// Spot trading
    Spot,
}

/// One rendered share card. Mirrors the legacy `OKTEShareViewType` plus the
/// private-copy card, which legacy expressed through a different container
/// class rather than an enum case.
#[derive(Debug, Clone, PartialEq)]
pub enum VanguardShareCardKind {
    CopyTraderProfile,
    LeadTraderProfileChart,
    LeadTraderProfileBillboard,
    LeadTraderTrade,
    CopyTraderTrade,
    PrivateCopyInviteLink,
    TraderHistoryPosition,
}

#[derive(Debug, Clone, PartialEq)]
pub enum VanguardShareDeeplinkIntent {
    TraderProfile,
    PrivateCopySettings,
}

#[derive(Debug, Clone, PartialEq)]
pub enum VanguardShareEmptyKind {
    None,
    Chart,
    Billboard,
}

/// IM share CTA — legacy `IMMessageShareCtaType`.
#[derive(Debug, Clone, PartialEq)]
pub enum VanguardShareImCtaKind {
    CopyNow,
    FollowNow,
}

#[derive(Debug, Clone, PartialEq)]
pub enum VanguardShareManagerType {
    LeadTraderUserProfile,
    NormalTraderUserProfile,
    LeadUserTrade,
    NormalUserTrade,
    PrivateCopyInviteLink,
    TraderHistoryPosition,
}

#[derive(Debug, Clone, PartialEq)]
pub enum VanguardShareTone {
    Profit,
    Loss,
    Flat,
}

/// Trade type for Vanguard Share
#[derive(Debug, Clone, PartialEq)]
pub enum VanguardShareTradeType {
    /// Futures/Contract trading
    Contract,
    /// Spot trading
    Spot,
}

/// Access level enum for weekly PnL chart
#[derive(Debug, Clone, PartialEq)]
pub enum WeeklyPnlAccessLevel {
    /// Public endpoint (viewing other traders)
    Public,
    /// Account endpoint (viewing own data)
    Account,
}

/// Business line enum for weekly PnL chart
#[derive(Debug, Clone, PartialEq)]
pub enum WeeklyPnlBusinessLine {
    /// Contract (futures) trading
    Contract,
    /// Spot trading
    Spot,
}

/// Access level for yield PnL chart data
#[derive(Debug, Clone, PartialEq)]
pub enum YieldPnlAccessLevel {
    /// Public endpoint (no authentication required, requires uniqueName)
    Public,
    /// Account endpoint (requires authentication)
    Account,
}

/// Business line for yield PnL chart data
#[derive(Debug, Clone, PartialEq)]
pub enum YieldPnlBusinessLine {
    /// Contract/Futures trading
    Contract,
    /// Spot trading
    Spot,
}

// -----------------------------------------------------------------------------
// 2. Records / Data Models (275)
// -----------------------------------------------------------------------------

/// Request body for amending copy instruments.
/// 
/// # API
/// POST /priapi/v5/ecotrade/copier/amend-instruments
/// POST /priapi/v5/ecotrade/spot/copier/amend-instruments
#[derive(Debug, Clone, PartialEq)]
pub struct AmendInstrumentsRequest {
    /// Trader's unique name
    pub trader_unique_name: String,
    /// Copy instrument mode: 0=custom, 1=follow trader
    pub copy_instrument_mode: String,
    /// List of instruments
    pub instruments: Option<Vec<InstrumentBody>>,
}

/// Input for amending settings.
#[derive(Debug, Clone, PartialEq)]
pub struct AmendSettingsInput {
    /// Trader's unique name
    pub trader_unique_name: String,
    /// Follow amount per trade
    pub follow_amount: Option<String>,
    /// Maximum follow amount
    pub max_follow_amount: String,
    /// Take profit ratio
    pub tp_ratio: Option<String>,
    /// Stop loss ratio
    pub sl_ratio: Option<String>,
    /// Copy mode
    pub copy_mode: String,
    /// Copy ratio (for ratio mode)
    pub copy_ratio: Option<String>,
    /// Margin mode (for Swap)
    pub margin_mode: Option<String>,
    /// Remaining margin deal mode
    pub remain_margin_deal: Option<String>,
    /// Stop loss amount
    pub stop_loss_amount: Option<String>,
    /// Copy instrument mode
    pub copy_instrument_mode: Option<String>,
    /// Initial amount (for smart copy)
    pub initial_amount: Option<String>,
    /// Replicate positions flag
    pub replication_required: Option<String>,
    /// Selected instruments
    pub instruments: Vec<UserInstrumentDto>,
}

/// Request body for amending copy trading settings.
/// 
/// Used for both Contract and Spot amend settings APIs.
/// 
/// # API
/// POST /priapi/v5/ecotrade/copier/amend-settings
/// POST /priapi/v5/ecotrade/spot/copier/amend-settings
#[derive(Debug, Clone, PartialEq)]
pub struct AmendSettingsRequest {
    /// List of instruments to copy. Same skip rule as CopierFirstSettingsRequest:
    /// Smart amend doesn't set instruments, native omits the field.
    pub instruments: Vec<UserInstrumentDto>,
    /// Follow amount per trade. Same rule as `CopierFirstSettingsRequest` — the
    /// two endpoints post the same `CopierTraderSettingsBean`, whose `followAmount`
    /// defaults to `""`: kotlinx omits the key on `""` and emits it (including the
    /// explicit `null` RATIO amend sends) otherwise.
    pub follow_amount: Option<String>,
    /// Maximum follow amount. Same skip rule as CopierFirstSettingsRequest:
    /// Smart amend may set this to "" (no upper limit / not applicable);
    /// server expects the field omitted in that case.
    pub max_follow_amount: String,
    /// Margin mode: cross, isolated
    pub mgn_mode: Option<String>,
    /// Take profit ratio
    pub tp_ratio: Option<String>,
    /// Stop loss ratio
    pub sl_ratio: Option<String>,
    /// Trader's unique name
    pub trader_unique_name: Option<String>,
    /// Stop loss amount
    pub stop_loss_amount: Option<String>,
    /// Remaining margin deal mode: 0=not set, 1=market close, 2=follow trader, 3=manual
    pub remain_mgn_deal: Option<String>,
    /// Copy mode: FIXED_COST, RATIO_COPY, SMART_COPY.
    /// Native declares the default `""` → kotlinx omits the key when empty.
    pub copy_mode: String,
    /// Copy ratio (required for ratio mode)
    pub copy_ratio: Option<String>,
    /// Private copy link ID
    pub private_copy_link_id: Option<String>,
    /// Copy instrument mode: 0=custom, 1=follow trader
    pub copy_instrument_mode: Option<String>,
    /// Initial investment amount (for smart copy)
    pub initial_amount: Option<String>,
    /// Whether to replicate position: "0" = no, "1" = yes
    pub replication_required: Option<String>,
}

/// Request body for amending smart copy settings.
/// 
/// # API
/// POST /priapi/v5/ecotrade/copytrading/relation/copier/amend-smart-settings
#[derive(Debug, Clone, PartialEq)]
pub struct AmendSmartSettingsRequest {
    /// Copy relation ID
    pub copy_rel_id: String,
    /// Stop loss amount
    pub stop_loss_amount: Option<String>,
    /// Remaining margin deal mode: 1=market close, 2=follow trader, 3=manual
    pub remain_mgn_deal: Option<String>,
    /// Instrument type (SWAP/SPOT)
    pub inst_type: String,
}

/// APR voucher information.
#[derive(Debug, Clone, PartialEq)]
pub struct AprVoucherDto {
    /// Voucher PNL ratio
    pub voucher_pnl_ratio: String,
    /// Investment upper limit
    pub invest_up_limit: String,
    /// Maximum voucher value
    pub voucher_max_value: String,
    /// List of underlying asset IDs
    pub underlying_ids: Vec<String>,
}

/// Asset filter configuration domain type.
/// 
/// Mirrors Swift `OKTEAssetConfig`.
#[derive(Debug, Clone, PartialEq)]
pub struct AssetConfig {
    /// Minimum asset value
    pub low: f32,
    /// Maximum asset value
    pub high: f32,
    /// Slider step size
    pub step_size: f32,
}

/// Asset filter configuration DTO.
/// 
/// Mirrors Swift `OKTELeadTraderRankConfigDTO.OKTEAssetConfigDTO`.
#[derive(Debug, Clone, PartialEq)]
pub struct AssetConfigDTO {
    /// Minimum asset value (as string)
    pub low: String,
    /// Maximum asset value (as string)
    pub high: String,
    /// Slider step size (as string)
    pub slider: String,
}

/// Associated affiliate info (UniFFI-compatible)
/// 
/// Contains information about the affiliate trader if the viewer is an invitee.
#[derive(Debug, Clone, PartialEq)]
pub struct AssociatedAffiliateInfo {
    /// Affiliate's nickname
    pub nick_name: String,
    /// Affiliate's portrait URL
    pub portrait: String,
    /// Affiliate's target ID
    pub target_id: String,
}

/// AUM filter configuration domain type.
/// 
/// Mirrors Swift `OKTEAumConfig`.
#[derive(Debug, Clone, PartialEq)]
pub struct AumConfig {
    /// Minimum AUM value
    pub low: f32,
    /// Maximum AUM value
    pub high: f32,
    /// Slider step size
    pub step_size: f32,
}

/// AUM filter configuration DTO.
/// 
/// Mirrors Swift `OKTELeadTraderRankConfigDTO.OKTEAumConfigDTO`.
#[derive(Debug, Clone, PartialEq)]
pub struct AumConfigDTO {
    /// Minimum AUM value (as string)
    pub low: String,
    /// Maximum AUM value (as string)
    pub high: String,
    /// Slider step size (as string)
    pub slider: String,
}

/// Basic benefits provided
#[derive(Debug, Clone, PartialEq)]
pub struct BasicBenefits {
    /// Followers limit
    pub followers_limit: String,
    /// Maximum profit sharing ratio
    pub max_pub_profit_sharing_ratio: String,
}

/// Detailed benefit information for a tier
#[derive(Debug, Clone, PartialEq)]
pub struct BenefitDetail {
    /// Tier information
    pub tier_info: Option<Tier>,
    /// List of benefit information by business type
    pub benefit_infos: Vec<BenefitInfo>,
    /// List of premium benefits
    pub premium_benefits: Vec<PremiumBenefits>,
}

/// Result wrapper for BenefitDetail list operations
#[derive(Debug, Clone, PartialEq)]
pub struct BenefitDetailResult {
    /// Success data, None if error occurred
    pub data: Option<Vec<BenefitDetail>>,
    /// Error information, None if successful
    pub error: Option<OKSTFFIError>,
}

/// Benefit information for specific business type
#[derive(Debug, Clone, PartialEq)]
pub struct BenefitInfo {
    /// Business type identifier
    pub business_type: String,
    /// Requirements for this benefit
    pub requirements: Requirements,
    /// Basic benefits provided
    pub basic_benefits: BasicBenefits,
}

/// Benefits icon information
/// Matches Kotlin BenefitsIcon with optional fields
#[derive(Debug, Clone, PartialEq)]
pub struct BenefitsIcon {
    /// App icon for day theme (matches Kotlin appIconDay)
    pub app_icon_day: Option<String>,
    /// App icon for night theme (matches Kotlin appIconNight)
    pub app_icon_night: Option<String>,
    /// Web icon (matches Kotlin webIcon)
    pub web_icon: Option<String>,
}

/// Request body for binding a coupon to a trader
/// Used in POST /priapi/v5/ecotrade/coupon/binding
#[derive(Debug, Clone, PartialEq)]
pub struct BindCouponBody {
    /// Business type: "SPOT" (spot trading) or "SWAP" (futures)
    pub biz_type: String,
    /// Unique identifier for the coupon
    pub coupon_id: String,
    /// Unique name of the trader to bind the coupon to
    pub trader_unique_name: String,
}

/// Response for cancel copy trade check (apply-cancel)
/// Indicates whether the user has open positions with the trader
/// 
/// ## API Response Example
/// ```json
/// {"hasPosition": "1"}  // "1" = true, "0" = false
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct CancelCopyPosResponse {
    /// Whether the user has open positions with this trader
    /// Server returns "1" for has positions, "0" for no positions
    /// Stored as String to preserve original format for Kotlin serialization
    pub has_position: String,
}

/// Input for canceling copy trade.
/// 
/// Maps to native `cancelCopyTrade(uniqueName, remainMgnDeal, isSpotTrade)`.
/// After cancel success, `postCancelReason` is called with the reason fields
/// for analytics (maps to native `PostCancelCopyingUseCase`).
#[derive(Debug, Clone, PartialEq)]
pub struct CancelCopyTradeInput {
    /// Trader's unique name to stop copying
    pub trader_unique_name: String,
    /// Position close type (how to handle remaining positions):
    /// - "0" = none (no open positions)
    /// - "1" = market price (close at market)
    /// - "2" = follow trader (let trader close)
    /// - "3" = manual process (user handles manually)
    pub remain_margin_deal: Option<String>,
    /// Reason type for cancellation (maps to native `ReasonForStopCopy.option.getOptionPosition()`)
    /// Empty string means no reason was selected.
    pub reason_type: Option<String>,
    /// Free-text "other" reason (maps to native `ReasonForStopCopy.reasonText`)
    pub other_reason: Option<String>,
}

/// Request body for canceling copying
/// Sent when user confirms cancellation
#[derive(Debug, Clone, PartialEq)]
pub struct CancelCopyingReasonBody {
    /// Trader's unique identifier
    pub unique_name: String,
    /// Type of reason for cancellation
    pub reason_type: String,
    /// Other reason (if reason_type is "other")
    pub other_reason: String,
    /// Business type identifier
    pub biz_type: String,
}

/// Request body for cancel-follow operation
/// Sent when user confirms cancellation of copy trading
#[derive(Debug, Clone, PartialEq)]
pub struct CancelFollowBody {
    /// Trader's unique identifier
    pub unique_name: String,
    /// Position close type - how to handle remaining positions
    /// "0" = none (no open positions)
    /// "1" = market price (close at market)
    /// "2" = follow trader (let trader close)
    /// "3" = manual process (user will handle manually)
    pub remain_mgn_deal: Option<String>,
}

/// Chart data point interface
/// Corresponds to ChartData interface in Kotlin
/// Note: In Rust, we use a struct instead of a trait for serialization
#[derive(Debug, Clone, PartialEq)]
pub struct ChartData {
    /// Timestamp in milliseconds
    /// JSON field: "statTime" (deserialized from string via LongAsStringSerializer)
    pub timestamp: i64,
    /// Value as string (BigDecimal in Kotlin)
    /// JSON field: "value" (deserialized from string via BigDecimalAsStringSerializer)
    /// Using String to preserve precision, matching BigDecimal behavior
    pub value: String,
}

/// Response for claiming/locking a voucher
/// Contains the voucher ID after successful lock
#[derive(Debug, Clone, PartialEq)]
pub struct ClaimVoucherResponse {
    /// Unique identifier for the locked voucher
    pub voucher_id: String,
}

/// UniFFI input record for [`OngoingLeadTradesInteractor::close_custom_position`].
/// 
/// Mirrors native `MarketPriceCloseBean` (`CloseOrderDto.kt:50-57`). String values
/// for `order_role_type` and `id_type` use the native `@SerialName` values
/// ("1" / "2") so the bridge can pass through what it already has without
/// re-introducing an enum on the Rust public surface.
#[derive(Debug, Clone, PartialEq)]
pub struct CloseCopyOrderInput {
    /// Trade item id. Maps to native `MarketPriceCloseBean.tradeItemId`.
    pub trade_item_id: String,
    /// Comma-separated position detail ids; `None` to omit. Maps to native
    /// `detailIds` (nullable).
    pub detail_ids: Option<String>,
    /// Order role: "1" = Leader, "2" = Copier. Mirrors native `OrderRoleType`
    /// @SerialName.
    pub order_role_type: String,
    /// Id kind: "1" = by-order-id, "2" = by-detail-order-ids. Mirrors native
    /// `IdType` @SerialName.
    pub id_type: String,
    /// Instrument type — usually SWAP (native default).
    pub inst_type: String,
    /// Limit price; `None` means market price. Maps to native `px`.
    pub px: Option<String>,
}

/// Close custom copy position request
/// POST /priapi/v5/ecotrade/close-copy-order
#[derive(Debug, Clone, PartialEq)]
pub struct CloseCustomPositionRequest {
    /// Trade item ID (order ID)
    pub trade_item_id: String,
    /// Detail IDs (position detail IDs, nullable)
    pub detail_ids: Option<String>,
    /// Type: 1=Leader, 2=Copier
    pub role_type: String,
    /// ID type: 1=ByOrderId, 2=ByDetailOrderIds
    pub id_type: String,
    /// Instrument type (default: SWAP)
    pub inst_type: String,
    /// Order price (null for market price)
    pub px: Option<String>,
}

/// Close smart copy position request
/// POST /priapi/v5/ecotrade/universals/smart/account/copier/close-position
#[derive(Debug, Clone, PartialEq)]
pub struct CloseSmartPositionRequest {
    /// Instrument ID
    pub inst_id: String,
    /// Position side
    pub pos_side: String,
    /// Margin mode
    pub mgn_mode: String,
    /// Copy relation ID
    pub copy_rel_id: String,
    /// Instrument type (SWAP, SPOT)
    pub inst_type: String,
}

/// Coin asset ratio information.
/// 
/// Mirrors ObjC `OKGTEContractSelectModel` for coinAssetRatios.
#[derive(Debug, Clone, PartialEq)]
pub struct CoinAssetRatio {
    /// Coin name
    pub name: String,
    /// Instrument ID (e.g., "BTC-USDT-SWAP")
    pub inst_id: String,
    /// Coin symbol for icon
    pub symbol: String,
    /// Asset ratio percentage
    pub asset_ratio: String,
    /// Raw coin-store icon URL — NOT from JSON; the bare
    /// `SocialTradeCore::coin_url_by_inst_id` lookup, applied in
    /// `leader_rank_request_center::enrich_coin_urls_with_core`.
    /// 
    /// `""` means **"the coin store had no icon for this instId"** — a plain lookup miss,
    /// NOT an instruction to fall back. On the Rust leg Android renders it verbatim:
    /// `SpotItemViewBinder.resolvedCoinUrl()` (Android `SocialSquareFragment.kt`) is
    /// `if (isRustLeg()) coinUrl else coinUrl.ifEmpty { SocialTradeManager.getCoinUrl(...) }`,
    /// so the `ifEmpty` native retry is reachable only from Android's own native leg, which
    /// never reads this field. Android's comment is explicit that this is deliberate — "an
    /// empty URL is a genuine Rust-side miss, and answering it with a native read is the
    /// value-level fallback this migration removes" — so an empty value surfaces as a
    /// missing icon rather than being masked.
    /// 
    /// Keep this field the bare lookup; use [`Self::resolved_coin_url`] for anything richer.
    pub coin_url: String,
    /// Display-ready icon URL from [`crate::public::coin_icon::social_resolved_coin_url`]:
    /// symbol precedence, uppercase normalisation, coin store, constructed-path fallback and
    /// CDN-domain acceleration all applied in Rust.
    /// 
    /// `""` means **"no icon"**, and unlike [`Self::coin_url`] that is rare: the
    /// constructed-path fallback still yields a usable URL on a coin-store miss. That
    /// difference in *resolution depth* — not a difference in fallback policy — is why the
    /// two cannot share one field. `coin_url` is frozen as the bare store lookup for
    /// Android, so widening it to this chain would change what Android renders and would
    /// silently paper over coin-store misses.
    /// 
    /// `#[uniffi(default = "")]` keeps existing native constructors source-compatible.
    pub resolved_coin_url: String, // default = ""
}

/// One instrument the user has selected for copy trading, supplied by the
/// native layer from `InstrumentsManager.getInstruments()`
/// (`copytrading/manager/InstrumentsManager.kt`). Passed into `load` as an
/// explicit parameter rather than read from a shared store (skill rule:
/// interactor methods take explicit context params).
/// 
/// - `inst_id`   : `UserInstruments.instId`
/// - `name`      : `UserInstruments.name` (native uses it as the display name)
/// - `selected`  : native `UserInstruments.selected == "1"`
#[derive(Debug, Clone, PartialEq)]
pub struct ContractInstrumentSelection {
    pub inst_id: String,
    pub name: String,
    pub selected: bool,
}

/// One leverage row. Mirrors native `ContractLeverageItemData`
/// (`CopyTradingSettingContractViewModel.kt:46-63`), minus the native-only
/// fuzzy-search matcher fields (`searchQuery`, `primary*`/`secondary*`) which
/// stay on the native side per the A2 decision. All lever values are raw
/// strings (no parsing/formatting in Rust — skill from_api Rule 1).
#[derive(Debug, Clone, PartialEq)]
pub struct ContractLeverageItem {
    pub name: String,
    pub inst_id: String,
    /// Copier side is single-sided (net) or cross margin.
    pub self_only_one_side: bool,
    /// Trader side is single-sided (net) or cross margin.
    pub trader_only_one_side: bool,
    pub self_net_lever: String,
    pub trader_net_lever: String,
    pub self_long_lever: String,
    pub self_short_lever: String,
    pub trader_long_lever: String,
    pub trader_short_lever: String,
    /// Margin mode this row belongs to ("cross" / "isolated").
    pub mgn_mode: String,
}

/// Contract leverage comparison data for a single instrument.
/// 
/// # API Response Format
/// ```json
/// {
/// "instId": "ETH-USDT-SWAP",
/// "mgnMode": "isolated",
/// "copierLeverDataVos": [
/// {"lever": "3", "posSide": "long"},
/// {"lever": "3", "posSide": "short"}
/// ],
/// "traderLeverDataVos": [
/// {"lever": "12.9", "posSide": "long"},
/// {"lever": "12.9", "posSide": "short"}
/// ]
/// }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct ContrastLeverageDto {
    /// Instrument ID (e.g., "ETH-USDT-SWAP").
    pub inst_id: String,
    /// Margin mode (e.g., "isolated", "cross").
    pub mgn_mode: String,
    /// Copier's leverage settings per position side.
    pub copier_lever_data: Vec<LeverageDataDto>,
    /// Trader's leverage settings per position side.
    pub trader_lever_data: Vec<LeverageDataDto>,
}

/// Leverage info for a single position side within an instrument.
#[derive(Debug, Clone, PartialEq)]
pub struct ContrastLeverageInfo {
    /// Instrument ID (e.g., "ETH-USDT-SWAP").
    pub inst_id: String,
    /// Margin mode (e.g., "isolated", "cross").
    pub mgn_mode: String,
    /// Position side ("long" or "short").
    pub pos_side: String,
    /// Copier's current leverage value.
    pub copier_lever: String,
    /// Trader's current leverage value.
    pub trader_lever: String,
}

/// Result of querying contrast leverage settings.
/// 
/// This is the domain type returned to native layers after processing
/// the API response.
#[derive(Debug, Clone, PartialEq)]
pub struct ContrastLeverageResult {
    /// List of leverage comparison data per instrument.
    pub items: Vec<ContrastLeverageDto>,
    /// Total number of instruments returned.
    pub total_count: u32,
    /// Number of instruments where copier and trader leverage differ.
    pub difference_count: u32,
    /// List of instrument IDs where leverage differs.
    pub instruments_with_difference: Vec<String>,
    /// Whether all instruments have matching leverage.
    pub all_match: bool,
}

/// Response DTO for batch set leverage API.
/// 
/// # API Response Format
/// ```json
/// {
/// "code": "0",
/// "msg": "",
/// "data": [
/// {
/// "result": "0",
/// "successfulInstId": "SOL-USDT-SWAP,BTC-USDT-SWAP,SUI-USDT-SWAP,ETH-USDT-SWAP",
/// "failedInstId": ""
/// }
/// ]
/// }
/// ```
/// 
/// # Result Codes
/// - "0": All modifications successful
/// - "1": Partial success (some instruments modified)
/// - "2": All modifications failed
#[derive(Debug, Clone, PartialEq)]
pub struct CopierBatchSetLeverageDto {
    /// Result code: "0" (all success), "1" (partial), "2" (all failed).
    pub result: String,
    /// Comma-separated list of instrument IDs that were successfully modified.
    pub successful_inst_id: String,
    /// Comma-separated list of instrument IDs that failed to modify.
    pub failed_inst_id: String,
}

/// Domain result for batch setting leverage.
/// 
/// This is the domain type returned to native layers after processing
/// the API response.
#[derive(Debug, Clone, PartialEq)]
pub struct CopierBatchSetLeverageResult {
    /// Result code indicating success level.
    pub result_code: BatchSetLeverageResultCode,
    /// Raw result code string from API ("0", "1", or "2").
    pub raw_result: String,
    /// List of instrument IDs that were successfully modified.
    pub successful_instrument_ids: Vec<String>,
    /// List of instrument IDs that failed to modify.
    pub failed_instrument_ids: Vec<String>,
    /// Number of instruments successfully modified.
    pub success_count: u32,
    /// Number of instruments that failed to modify.
    pub failure_count: u32,
}

/// Request body for submitting first copy trading settings.
/// 
/// # API
/// POST /priapi/v5/ecotrade/copier/first-settings
/// POST /priapi/v5/ecotrade/spot/copier/first-settings
#[derive(Debug, Clone, PartialEq)]
pub struct CopierFirstSettingsRequest {
    /// List of instruments to copy. Native omits the field entirely for
    /// Smart copy submissions (which never set instruments). Without
    /// `skip_serializing_if`, Rust serializes the default `[]` and the
    /// server-side strictness may differ from native's leaner shape.
    pub instruments: Vec<UserInstrumentDto>,
    /// Follow amount per trade. Native's `CopierTraderSettingsBean.followAmount`
    /// declares the default `""`, so kotlinx (`encodeDefaults = false`) omits the
    /// key when the value is `""` and emits it otherwise — including the explicit
    /// `"followAmount":null` that RATIO/Smart call sites produce by passing `null`.
    /// Mirror both halves: skip `Some("")`, serialize `None` as `null`.
    pub follow_amount: Option<String>,
    /// Maximum follow amount. Skip serialization when empty — Smart copy
    /// submissions don't use this field and native body omits it; sending an
    /// empty string triggers server 400 / 50029 "format error".
    pub max_follow_amount: String,
    /// Margin mode: cross, isolated
    pub mgn_mode: Option<String>,
    /// Take profit ratio
    pub tp_ratio: Option<String>,
    /// Stop loss ratio
    pub sl_ratio: Option<String>,
    /// Trader's unique name
    pub trader_unique_name: Option<String>,
    /// Stop loss amount
    pub stop_loss_amount: Option<String>,
    /// Remaining margin deal mode: 0=not set, 1=market close, 2=follow trader, 3=manual
    pub remain_mgn_deal: Option<String>,
    /// Copy mode: FIXED_COST, RATIO_COPY, SMART_COPY.
    /// Native declares the default `""` → kotlinx omits the key when empty.
    pub copy_mode: String,
    /// Copy ratio (required for ratio mode)
    pub copy_ratio: Option<String>,
    /// Private copy link ID. Always serialize (even when None → `null`) to
    /// match native body shape exactly — `CopierTraderSettingsBean.privateCopyLinkId`
    /// is the one field declared WITHOUT a default, so kotlinx always encodes it
    /// (`"privateCopyLinkId":null` when the user isn't in a private-invite flow,
    /// the link id otherwise). Other Option fields use `skip_if_none` because
    /// their native default IS null, so kotlinx omits them.
    pub private_copy_link_id: Option<String>,
    /// Copy instrument mode
    /// Request values: "0" = custom, "1" = follow trader (per Kotlin CopierTraderSettingsBean)
    /// Note: Response uses different values (see TraderSettingsDto)
    pub copy_instrument_mode: Option<String>,
    /// Initial investment amount (for smart copy)
    pub initial_amount: Option<String>,
    /// Whether to replicate position: "0" = no, "1" = yes
    pub replication_required: Option<String>,
}

/// Individual copier message item for UniFFI export
#[derive(Debug, Clone, PartialEq)]
pub struct CopierMessageItem {
    /// Unique identifier for the message
    pub id: String,
    /// Message level as string: "1"=info, "2"=warn, "3"=important
    pub level: String,
    /// Message title
    pub title: String,
    /// Message content body
    pub content: String,
    /// Optional URL link for the message
    pub link: String,
    /// Whether to display transfer button ("1" = true)
    pub display_transfer: bool,
}

/// State for Copier Message Interactor
#[derive(Debug, Clone, PartialEq)]
pub struct CopierMessageState {
    /// Content load state
    pub content_state: CopierMessageLoadState,
    /// All messages loaded from API
    pub messages: Vec<CopierMessageItem>,
    /// Whether the list is empty
    pub is_empty: bool,
    /// Backend error message — empty when not in Error state.
    /// Bridge surfaces this via OKServerException to match native error display
    /// (see social-trading-migration skill: Error Display Native Parity Rule).
    pub error_message: String,
}

/// Amount metrics in detail block
#[derive(Debug, Clone, PartialEq)]
pub struct CopierPnlAmountDetail {
    pub init_follow_amount: String,
    pub transfer_add_amount: String,
    pub transfer_reduce_amount: String,
    pub investment_amount: String,
    pub follow_amount_occupied: String,
}

/// Per-copier PnL detail
#[derive(Debug, Clone, PartialEq)]
pub struct CopierPnlDetail {
    pub copy_rel_id: String,
    pub copy_state: String,
    pub u_time: String,
    pub pnl_detail: CopierPnlDetailInfo,
    pub amount_detail: CopierPnlAmountDetail,
}

/// PnL metrics in detail block
#[derive(Debug, Clone, PartialEq)]
pub struct CopierPnlDetailInfo {
    pub pnl: String,
    pub net_pnl: String,
    pub realized_pnl: String,
    pub upl: String,
    pub today_pnl: String,
}

/// Live PnL payload (DTO) from copytrading-pnl WebSocket channel
#[derive(Debug, Clone, PartialEq)]
pub struct CopierPnlLiveData {
    /// Total PnL summary (optional)
    pub total_pnl: Option<CopierPnlTotalPnl>,
    /// Detailed per-copier entries (optional)
    pub details: Option<Vec<CopierPnlDetail>>,
}

/// Root view model mapped from DTO (details default to empty)
#[derive(Debug, Clone, PartialEq)]
pub struct CopierPnlLiveDataRoot {
    /// Total PnL summary (optional)
    pub total_pnl: Option<CopierPnlTotalPnl>,
    /// Detailed per-copier entries
    pub details: Vec<CopierPnlDetail>,
}

/// Total PnL summary
#[derive(Debug, Clone, PartialEq)]
pub struct CopierPnlTotalPnl {
    pub pnl: String,
    pub realized_pnl: String,
    pub upl: String,
    pub today_pnl: String,
}

/// State for CopierPnlWs Interactor
/// 
/// Contains the WebSocket connection state and latest PnL data.
#[derive(Debug, Clone, PartialEq)]
pub struct CopierPnlWsState {
    /// Instrument type (Swap or Spot)
    pub inst_type: CopierPnlInstType,
    /// Data type (Home, CopierDashboard, or specific trader)
    pub data_type: CopierPnlLiveDataType,
    /// Trading mode (Live or Demo)
    pub trading_mode: CopierPnlTradingMode,
    /// Rise/fall cycle timezone
    pub rise_fall_cycle: CopierPnlRiseFallCycleType,
    /// Whether WebSocket is currently running
    pub is_running: bool,
    /// Latest snapshot data received from WebSocket
    pub latest: Option<CopierPnlLiveDataRoot>,
}

/// Display projection of one History-tab row. Every field is independently
/// optional: a `None` field means "Rust could not (or, per D11, does not yet)
/// produce this value" and native's pre-existing per-field logic keeps running
/// for that one field only. There is no all-or-nothing gate.
#[derive(Debug, Clone, PartialEq)]
pub struct CopierPositionHistoryRowDisplay {
    /// Avg. open price. Field map: `open_avg_px` -> `social_format_price` ->
    /// `st_price`.
    pub average_open_price_display: Option<DisplayString>,
    /// Avg. close price. Same source formatter, inverted rounding policy.
    pub average_close_price_display: Option<DisplayString>,
    /// Realized PnL. `round_pnl` -> `st_amount` (`TradingNumber`, never `.pnl`
    /// — D4).
    pub pnl_display: Option<DisplayString>,
    /// "Amount"/"Close" grid value. `social_format_position` -> `st_amount`.
    pub number_of_positions_display: Option<DisplayString>,
    /// PnL %. `st_percent(st_scale_percent(raw, 2), SIGN_EXCEPT_ZERO)` (D5),
    /// with an explicit non-finite-ratio guard ahead of it.
    pub pnl_ratio_display: Option<DisplayString>,
    /// "Buy value" (spot only in practice — a non-spot row's `deal_volume` is
    /// empty, which `round_deal_volume` already treats as a guard failure (an
    /// unparseable/empty string), so this naturally stays `None` for contract
    /// rows without a business-line branch here).
    pub deal_volume_display: Option<DisplayString>,
    /// The sell-status pill text. `st_cd` with one of the 7 existing keys
    /// above, or `None` when the code is unrecognised or (spot +
    /// liquidation-class code) has no native key at all (D10).
    pub sell_status_display: Option<DisplayString>,
    /// Bought/open time. Left `None` unconditionally — see the module-level
    /// note on `project_row` for why (D11 deferral).
    pub open_time_display: Option<DisplayString>,
    /// Sold/close time. Same D11 deferral as `open_time_display`.
    pub close_time_display: Option<DisplayString>,
}

/// State for Copier Position History Interactor.
/// 
/// `copy_mode` is supplied at construction by the caller (Fragment / Holder).
/// Native VM (`HistoricalPositionViewModel`) takes `CurrentPositionType` from
/// Fragment arguments — Rust mirrors that semantic by accepting `copy_mode`
/// up-front rather than reading from a detail-load state.
#[derive(Debug, Clone, PartialEq)]
pub struct CopierPositionHistoryState {
    /// Instrument type (Swap or Spot). Set at construction.
    pub inst_type: CopierPnlInstType,
    /// Lead-trader relation id (identifies the copier-trader pair). Set at construction.
    pub copy_rel_id: String,
    /// Copy mode of the relation. Set at construction — decides which endpoint
    /// the load action calls (`copier/positions-history` vs `copier/position-history`).
    pub copy_mode: MyLeadTraderCopyMode,
    /// Load state for the history list.
    pub content_state: ContentLoadState,
    /// Backend error message captured on any fetch failure.
    /// Cleared on Loading entry (refresh start) and on SUCCESS.
    pub error_message: String,
    /// Smart-copy history items (populated when `copy_mode = SmartCopy`).
    pub items_smart: Vec<SmartCopyPositionHistoryDto>,
    /// Custom-copy history items (populated when `copy_mode = FixedCost | RatioCopy`).
    pub items_custom: Vec<CustomCopyPositionHistoryDto>,
    /// Display projection of [`Self::items_smart`], index-aligned: index `i`
    /// here corresponds to index `i` of `items_smart`.
    /// 
    /// iOS-only projection; Android reads the raw `items_smart` DTOs directly.
    /// `#[uniffi(default = [])]` keeps Kotlin/Swift memberwise constructors
    /// source-compatible for older/Android clients that never populate this
    /// field (precedent: `LeadTraderListState::cards`).
    pub items_smart_display: Vec<CopierPositionHistoryRowDisplay>, // default = []
    /// Display projection of [`Self::items_custom`], index-aligned: index `i`
    /// here corresponds to index `i` of `items_custom`.
    /// 
    /// iOS-only projection; Android reads the raw `items_custom` DTOs directly.
    /// `#[uniffi(default = [])]` keeps Kotlin/Swift memberwise constructors
    /// source-compatible for older/Android clients that never populate this
    /// field (precedent: `LeadTraderListState::cards`).
    pub items_custom_display: Vec<CopierPositionHistoryRowDisplay>, // default = []
    /// Pagination cursor for the next page (timestamp for Smart, id for Custom).
    pub next_after: Option<String>,
}

/// State for Copier Positions Interactor.
#[derive(Debug, Clone, PartialEq)]
pub struct CopierPositionsState {
    /// Instrument type (Swap or Spot). Set at construction.
    pub inst_type: CopierPnlInstType,
    /// Lead-trader relation id (identifies the copier-trader pair). Set at construction.
    pub copy_rel_id: String,
    /// Currently selected tab in the Custom mode ongoing positions view.
    pub custom_ongoing_selected_tab: MyLeadTraderCustomOngoingTab,
    /// Whether the positions WebSocket is currently running.
    pub is_positions_ws_running: bool,
    /// Used by `pause_ws` / `resume_ws` to remember whether the WS was running before pause.
    pub paused_was_running: bool,
    /// Load state for the ongoing positions list.
    pub ongoing_content_state: ContentLoadState,
    /// Smart-copy ongoing positions list (populated when `copy_mode = SmartCopy`).
    pub ongoing_smart_positions: Vec<SmartCopyPositionDto>,
    /// Custom-copy ongoing positions list (populated when `copy_mode = FixedCost | RatioCopy`).
    pub ongoing_custom_positions: Vec<CustomCopyPositionDto>,
}

/// Response data for copier retention popup
/// Contains information about first and second retention popups
#[derive(Debug, Clone, PartialEq)]
pub struct CopierRetentionPopupResponse {
    pub first_pop: Option<FirstRetentionPopup>,
    pub second_pop: Option<SecondRetentionPopup>,
}

/// Result wrapper for CopierRetentionPopupResponse list operations
#[derive(Debug, Clone, PartialEq)]
pub struct CopierRetentionPopupResult {
    /// Success data, None if error occurred
    pub data: Option<Vec<CopierRetentionPopupResponse>>,
    /// Error information, None if successful
    pub error: Option<OKSTFFIError>,
}

/// Special lever response item from the API.
/// 
/// # API Response Format
/// ```json
/// {
/// "code": "0",
/// "msg": "",
/// "data": [
/// {
/// "instId": "BTC-USDT-SWAP,ETH-USDT-SWAP"
/// }
/// ]
/// }
/// ```
/// 
/// Note: The `instId` field may be empty string if no special leverage applies,
/// or a comma-separated list of instrument IDs that have special leverage restrictions.
#[derive(Debug, Clone, PartialEq)]
pub struct CopierSpecialLeverDto {
    /// Comma-separated instrument IDs with special leverage restrictions.
    /// Empty string if no special leverage applies to any requested instrument.
    pub inst_id: String,
}

/// Result of querying special leverage settings.
/// 
/// This is the domain type returned to native layers after processing
/// the API response.
#[derive(Debug, Clone, PartialEq)]
pub struct CopierSpecialLeverResult {
    /// List of instrument IDs that have special leverage restrictions.
    pub special_lever_instrument_ids: Vec<String>,
    /// List of base currencies extracted from instrument IDs.
    /// Useful for displaying which coins have restrictions.
    pub special_lever_base_currencies: Vec<String>,
    /// Whether all requested instruments have special leverage (batch restriction).
    /// - `true`: All instruments have special leverage (no batch modification allowed)
    /// - `false`: Some or no instruments have special leverage
    pub all_have_special_lever: bool,
    /// Whether some (but not all) instruments have special leverage (partial restriction).
    /// - `true`: Some instruments have special leverage (partial batch allowed)
    /// - `false`: Either none or all instruments have special leverage
    pub partial_special_lever: bool,
}

/// Observable state. Mirrors what native exposes to the UI:
/// `_selectedLeverageForTraderAndSelf` (leverageList + marginMode + isExpand,
/// already filtered to selected instruments) plus `_riskWarningVisibility`.
/// 
/// The native per-margin-mode caches (`crossLeverageList` / `isolateLeverageList`,
/// `CopyTradingSettingContractViewModel.kt:91-92`) are NOT here — they are
/// internal interactor state (the full, pre-selection-filter mapped lists),
/// held on the interactor struct, not observed by the UI.
#[derive(Debug, Clone, PartialEq)]
pub struct CopyContractSettingsState {
    pub content_state: ContractLeverageContentState,
    /// Intentionally ALWAYS EMPTY for this feature. Native `onFailure`
    /// (`CopyTradingSettingContractViewModel.kt:174`) shows a FIXED localized
    /// network-error string, NOT the backend message — so the backend reason is
    /// logged only, never surfaced. The field is retained for state-shape symmetry
    /// with the project's Error-Display-Parity convention; the Holder must NOT read
    /// it (it would always be ""). Do not start surfacing it without changing the
    /// native path too.
    pub error_message: String,
    /// Selected + current-margin-mode leverage rows (native: state.leverageList
    /// after `updateLeverageList`'s `filter { isSelected(it.instId) }`).
    pub leverage_list: Vec<ContractLeverageItem>,
    /// Current margin mode ("cross" / "isolated").
    pub margin_mode: String,
    /// Expand/collapse flag (native `UiState.isExpand`; collapse-to-3 is applied
    /// on the native display side, kept native in Stage 1).
    pub is_expand: bool,
    /// High-risk warning: any selected row has self lever ≥ MAX_WARNING_LEVERAGE
    /// (native `checkLeverLimit`, `CopyTradingSettingContractViewModel.kt:212-221`).
    pub risk_warning: bool,
}

/// Navigation payload assembled by the gatekeeper and handed to Kotlin to
/// rebuild the native `BaseSocialCopyFlowActivity.Params`. Pure data carrier —
/// the interactor's responsibility ends at emitting this.
/// 
/// Notes on representation:
/// - `copy_mode` is the frozen bridge carrier used by the Android private-invite path.
/// - `bio_content` is the raw bio string (= `EcoBasicUserInfoResponse.sign`);
/// Kotlin wraps it into `EcoBioContent`.
/// - `custom_pre_check_code` / `copier_followee_limit` mirror native defaults
/// (the VM never sets them in this flow: 0 / "").
#[derive(Debug, Clone, PartialEq)]
pub struct CopyFlowParams {
    /// Trade type (Swap/Spot).
    pub trade_type: CopySettingsTradeType,
    /// Trader unique name.
    pub unique_name: String,
    /// Trader bio (raw `sign`; Holder builds EcoBioContent(bio, translated_bio)).
    pub bio_content: String,
    /// Translated bio (native `translatedBio`; "" = none).
    pub translated_bio: String,
    /// Trader display nickname (RAW; Holder applies mapNickName w/ en_nick_name).
    pub nick_name: String,
    /// English nickname (mapNickName fallback).
    pub en_nick_name: String,
    /// Trader avatar URL.
    pub avatar_url: String,
    /// Trader target id.
    pub target_id: String,
    /// Trader settings (manage + non-smart-mode only; else None).
    pub setting_bean: Option<TraderSettingsDto>,
    /// Android private-invitation carrier (`private_invitation:<linkId>`);
    /// empty for the released/default path.
    pub copy_mode: String,
    /// Whether the trader is a private lead trader (Swap only; Spot always false).
    pub is_private_trader: bool,
    /// Smart copy settings (fetched unless manage+non-smart).
    pub smart_copy_bean: Option<SmartCopySettingsDto>,
    /// Relation id (manage flows).
    pub relation_id: Option<String>,
    /// Should-expand passthrough.
    pub should_expand: bool,
    /// Dead field — VM never sets it; always 0. Kept for native parity.
    pub custom_pre_check_code: i32,
    /// Dead field — always "". Kept for native parity.
    pub copier_followee_limit: String,
    /// Usable/using coupon (using on manage, usable on new; None on failure).
    pub coupon_info: Option<CouponCenterItemResponse>,
    /// Raw `recommendedCopyMode` API code (from smart settings). Kotlin Holder
    /// parses it into native `CopyModeRestriction` (= native
    /// `CopyModeRestriction.mapFromRecommendedCopyMode`). String (not enum) to
    /// stay RustToKotlinConverter-safe — see module policy note above.
    pub copy_mode_restriction: String,
}

/// Copy mode information returned when querying copy mode.
/// 
/// # API
/// GET /priapi/v5/ecotrade/universals/relation/copier/copy-mode
#[derive(Debug, Clone, PartialEq)]
pub struct CopyModeDto {
    /// Copy mode type: FIXED_COST, RATIO_COPY, SMART_COPY
    pub copy_mode: Option<String>,
    /// Copy relation ID if already copying
    pub copy_rel_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CopySettingsRequirement {
    /// Backend `data[0].errorCode` — the specific requirement error code.
    pub error_code: String,
    /// Backend `data[0].action` — passed UNCHANGED as the service requirement code
    /// (`mappingKey` for questionnaire, `requirementCode` for disclaimer).
    pub action: String,
    /// Backend `data[0].type` — routing discriminator, case-sensitive
    /// (`QUESTIONNAIRE` / `DISCLAIMER`). Named `requirement_type` in Rust because
    /// `type` is a reserved word; serde keeps the wire/binding key as `type`.
    pub requirement_type: String,
}

/// Copy Settings state.
/// 
/// This state contains all the settings data for copy trading configuration pages.
/// Native side reads this state and renders the UI accordingly.
/// 
/// The `#[interactor::state]` macro adds: `Clone, Debug, PartialEq, uniffi::Record`
/// 
/// # Corresponding iOS UI
/// 
/// | Field | UI Element |
/// |-------|------------|
/// | `trader_settings` | Trader settings form data |
/// | `available_amount` | Max available amount display |
/// | `instruments` | Instrument selection list |
/// | `copy_mode` | Copy mode selector |
/// | `copy_rel_id` | Existing relation (if amending) |
#[derive(Debug, Clone, PartialEq)]
pub struct CopySettingsState {
    /// Content loading state
    pub content_state: ContentLoadState,
    /// Current trade type (Swap/Spot)
    pub trade_type: CopySettingsTradeType,
    /// Trader's unique name being copied
    pub trader_unique_name: String,
    /// Copy relation ID (if existing relation, for amend flows)
    pub copy_rel_id: Option<String>,
    /// Current or selected copy mode
    pub copy_mode: CopySettingsCopyMode,
    /// Available copy modes for this trader
    pub available_copy_modes: Vec<CopySettingsCopyMode>,
    /// Loaded trader settings
    pub trader_settings: Option<TraderSettingsDto>,
    /// Loaded instrument info
    pub instrument_info: Option<InstrumentInfoDto>,
    /// Available amount for copy trading
    pub available_amount: Option<String>,
    /// Smart copy settings (smart input)
    pub smart_settings: Option<SmartCopySettingsDto>,
    /// Smart copy reducible amount (smart input)
    pub reducible_amount: Option<SmartCopyReducibleAmountDto>,
    /// Max available amount from trading account (smart input)
    pub smart_max_available_amount: Option<String>,
    /// Follow amount per trade (for FIXED_COST mode)
    pub follow_amount: String,
    /// Maximum follow amount (total investment)
    pub max_follow_amount: String,
    /// Take profit ratio (percentage)
    pub tp_ratio: Option<String>,
    /// Stop loss ratio (percentage)
    pub sl_ratio: Option<String>,
    /// Copy ratio (for RATIO_COPY mode)
    pub copy_ratio: Option<String>,
    /// Margin mode (for Swap: "cross" or "isolated")
    pub margin_mode: Option<String>,
    /// Remaining margin deal mode
    pub remain_margin_deal: Option<String>,
    /// Stop loss amount
    pub stop_loss_amount: Option<String>,
    /// Copy instrument mode: "0" = custom, "1" = follow trader
    pub copy_instrument_mode: String,
    /// Initial amount (for smart copy)
    pub initial_amount: Option<String>,
    /// Replicate existing positions flag
    pub replication_required: Option<String>,
    /// Selected instruments for copy
    pub selected_instruments: Vec<UserInstrumentDto>,
    /// Recommended ratio value
    pub recommended_ratio: Option<String>,
    /// Minimum recommended ratio
    pub recommended_ratio_min: Option<String>,
    /// Maximum recommended ratio
    pub recommended_ratio_max: Option<String>,
    /// Whether current form is valid for submission
    pub is_form_valid: bool,
    /// Validation error message (if any)
    pub validation_error: Option<String>,
    /// Usable coupon for first-copy (loss protection)
    pub usable_coupon: Option<CouponCenterItemResponse>,
    /// Coupon currently in use for an existing copy relation
    pub using_coupon: Option<CouponCenterItemResponse>,
    /// Created relation ID (after first settings success)
    pub created_relation_id: Option<String>,
    /// Private copy link id carried by the request body, mirroring the lifetime of
    /// native's `CopyTradingSettingHomeViewModel.settingsBean.privateCopyLinkId`.
    /// 
    /// Native writes that bean field in exactly one place — the Swap first-copy
    /// submit (`VM.kt` `handleSwapCopySubmission`) — and every later request built
    /// from the same VM instance (i.e. the amend body) carries whatever the bean
    /// holds. The Spot submit branch never writes it, so it stays `None` there.
    /// This field reproduces that carry-over without widening `AmendSettingsInput`,
    /// whose UniFFI Record shape is frozen for platform constructors.
    pub private_copy_link_id: Option<String>,
}

/// Copy Trade Retained state.
/// 
/// Holds the cross-call context needed for switch-account / custom-copy
/// **re-entry** (= native `cacheParams` + `currentCopyMode` + `editTradeSetting`).
/// This is why the interactor is stateful (not a passthrough).
/// 
/// `#[interactor::state]` adds: `Clone, Debug, PartialEq, uniffi::Record`.
#[derive(Debug, Clone, PartialEq)]
pub struct CopyTradeRetainedState {
    /// Trade type (Swap/Spot) for this flow.
    pub trade_type: CopySettingsTradeType,
    /// Trader being copied.
    pub trader_unique_name: String,
    /// Copy relation id (manage/edit flows only; None for new-copy).
    pub relation_id: Option<String>,
    /// true = manage existing copy, false = new copy.
    pub edit_trade_setting: bool,
    /// Whether to expand settings on open (passthrough to navigation payload).
    pub should_expand: bool,
    /// Current copy mode driving re-entry routing.
    /// = native `currentCopyMode` (Position[SmartCopy] for the smart path,
    /// FixedCost for the custom path). switchAccount re-entry branches on this.
    pub current_copy_mode: CopySettingsCopyMode,
    /// Assembled navigation payload from the last successful check.
    /// = native `cacheParams`; reused by custom-copy / switch-account re-entry.
    pub cached_params: Option<CopyFlowParams>,
    /// Loading state.
    pub content_state: ContentLoadState,
}

/// Copy trading order coupon flag information
/// 
/// Contains compensation ticket/coupon information for a trade position.
/// Used to display voucher/bonus indicators on ongoing positions.
/// 
/// # Fields
/// - `item_id`: Position/trade item ID
/// - `compensation_ticket_flag`: Flag indicating if coupon should be shown ("0" or "1")
/// - `compensation_ticket_amount`: Amount of the compensation ticket
/// - `compensation_ticket_unit`: Unit/currency of the compensation (e.g., "USDT")
/// - `compensation_ticket_name`: Display name of the compensation ticket
/// - `compensation_ticket_state`: State of the compensation ticket
#[derive(Debug, Clone, PartialEq)]
pub struct CopyTradingOrderCouponFlag {
    /// Position/trade item ID (posId or tradeItemId)
    pub item_id: String,
    /// Flag indicating if coupon should be shown ("0" = no, "1" = yes)
    pub compensation_ticket_flag: String,
    /// Amount of the compensation ticket
    pub compensation_ticket_amount: String,
    /// Unit/currency of the compensation (e.g., "USDT")
    pub compensation_ticket_unit: String,
    /// Display name of the compensation ticket
    pub compensation_ticket_name: String,
    /// State of the compensation ticket
    pub compensation_ticket_state: String,
}

/// DTO for coupon center item response from API
/// Represents a single coupon item with all its properties.
/// 
/// Native ground truth: `trading_eco_impl/dto/NewbieZoneResponse.kt:12-31`
/// (`CouponCenterItemResponse` — 18 fields including the loss-protection radio-button
/// `isSelected` flag used by the CopySetting channel).
/// 
/// All fields use `#[serde(default)]` for forward compatibility — backend may omit any
/// field. Field names match native `@SerialName` (camelCase via `rename_all`).
/// 
/// `state` is kept as `String` (raw API code "0".."7" + empty) per the 2026-05-04
/// "no UniFFI enum in Converter-bound Records" rule — bridge mappers parse to the
/// native `CouponItemState` enum.
#[derive(Debug, Clone, PartialEq)]
pub struct CouponCenterItemResponse {
    /// Unique identifier for the coupon (native: `couponId`, NewbieZoneResponse.kt:13)
    pub coupon_id: String,
    /// Business line: "SPOT" / "SWAP" (native: `bizType`, NewbieZoneResponse.kt:21)
    pub biz_type: String,
    /// Coupon name (native: `couponName`, NewbieZoneResponse.kt:18)
    pub coupon_name: String,
    /// Coupon rule text (native: `couponRule`, NewbieZoneResponse.kt:19)
    pub coupon_rule: String,
    /// Coupon type code (native: `couponType`, NewbieZoneResponse.kt:20)
    pub coupon_type: String,
    /// Coupon description (native: `couponDescription`, NewbieZoneResponse.kt:25).
    /// NB: this is distinct from the legacy `coupon_desc` field below — backend
    /// payload uses `couponDescription`.
    pub coupon_description: String,
    /// Compensation amount value (native: `amount`, NewbieZoneResponse.kt:14)
    pub amount: String,
    /// Compensation amount unit/currency (native: `unit`, NewbieZoneResponse.kt:15)
    pub unit: String,
    /// Valid-until timestamp string (native: `expireTime`, NewbieZoneResponse.kt:16)
    pub expire_time: String,
    /// Current server time string (native: `currTime`, NewbieZoneResponse.kt:17)
    pub curr_time: String,
    /// Grant time string (native: `grantTime`, NewbieZoneResponse.kt:26)
    pub grant_time: String,
    /// Expire-time type: "1" = countdown, "2" = time range
    /// (native: `expireTimeType`, NewbieZoneResponse.kt:27)
    pub expire_time_type: String,
    /// Usable flag: "0" = unusable, "1" = usable (native: `useFlag`,
    /// NewbieZoneResponse.kt:22). Kept as raw String per native `@SerialName` mapping.
    pub use_flag: String,
    /// Reason text when coupon is unusable (native: `reason`, NewbieZoneResponse.kt:23)
    pub reason: String,
    /// Coupon state raw API code: "0".."7" mapped by native `CouponItemState`
    /// `@SerialName` (NewbieZoneResponse.kt:52-138):
    /// "0"=Available, "1"=Guaranteed, "2"=PositionOpened, "3"=InProgress,
    /// "4"=Paid, "5"=Failed, "6"=Expired, "7"=Invalid, ""=Empty
    /// (native: `state`, NewbieZoneResponse.kt:24). Kept as String per the
    /// 2026-05-04 enum-in-Record rule — bridge layer parses via explicit helper.
    pub state: String,
    /// Unique name of the trader this coupon is bound to
    /// (native: `traderUniqueName` of type `UniqueName`, NewbieZoneResponse.kt:28).
    /// `UniqueName` is a `value class` wrapping `String` on native — wire format
    /// is plain String.
    pub trader_unique_name: String,
    /// Copy-trading relation id
    /// (native: `relationId` of type `RelationId`, NewbieZoneResponse.kt:29).
    /// `RelationId` is a `value class` wrapping `String` on native.
    pub relation_id: String,
    /// Whether this coupon is currently selected as the active loss-protection
    /// coupon (native: `isSelected: var`, NewbieZoneResponse.kt:30). Local UI
    /// state only — backend never returns this; default `false`. Mutated by
    /// the Interactor's `update_coupon_item_selected` action.
    pub is_selected: bool,
    /// (Legacy) Description of the coupon. Not in native payload.
    pub coupon_desc: String,
    /// (Legacy) Discount amount or percentage. Not in native payload.
    pub discount_amount: String,
    /// (Legacy) Currency code for the discount. Not in native payload.
    pub currency: String,
    /// (Legacy) Usable boolean. Native equivalent is `use_flag` String — kept
    /// here for existing UseCase callers.
    pub usable: bool,
    /// (Legacy) Status of the coupon. Native equivalent is `state` raw code.
    pub status: String,
}

/// Result wrapper for CouponCenterItemResponse list operations
#[derive(Debug, Clone, PartialEq)]
pub struct CouponCenterItemResult {
    /// Success data, None if error occurred
    pub data: Option<Vec<CouponCenterItemResponse>>,
    /// Error information, None if successful
    pub error: Option<OKSTFFIError>,
}

/// State for Coupon Center Interactor.
/// 
/// Two observable surfaces — mirrors the two native StateFlows:
/// `_couponListViewState` (CouponCenterViewModel.kt:25) ↔ `content_state` + `items` + `error_message`
/// `_currentLossProtection` (CouponCenterViewModel.kt:30) ↔ `current_coupon`
#[derive(Debug, Clone, PartialEq)]
pub struct CouponCenterState {
    /// Content load state for the coupon list.
    pub content_state: CouponCenterLoadState,
    /// All coupon items loaded from API. Empty until `Success`.
    pub items: Vec<CouponCenterItemResponse>,
    /// Backend error message — empty when not in `Error` state.
    /// Bridge surfaces this via `OKServerException` to match native error
    /// display (see social-trading-migration skill: Error Display Native
    /// Parity Rule).
    pub error_message: String,
    /// Whether the load was triggered with `silent_load = true`. Native VM
    /// uses this to skip transitioning to `ViewState.Loading` when the page
    /// is already showing data (CouponCenterViewModel.kt:52-58); the bridge
    /// uses this flag to suppress full-screen spinner during background
    /// refreshes.
    pub silent_load: bool,
    /// Currently selected loss-protection coupon (the
    /// `_currentLossProtection` flow in native). `None` when no coupon is
    /// selected.
    pub current_coupon: Option<CouponCenterItemResponse>,
}

/// Current position data
/// Corresponds to CurrentPosition in Kotlin
#[derive(Debug, Clone, PartialEq)]
pub struct CurrentPosition {
    /// Long leverage
    pub long_lever: String,
    /// Short leverage
    pub short_lever: String,
    /// Position data list
    pub pos_data: Vec<Position>,
}

/// Current trader response from API
/// GET /priapi/v5/ecotrade/universals/relation/copier/list
#[derive(Debug, Clone, PartialEq)]
pub struct CurrentTraderResponse {
    /// Trader information
    pub trader_info: PartialTraderInfoResponse,
    /// Relation information
    pub relation_info: OngoingRelationInfoResponse,
    /// Follow PnL details
    pub follow_pnl_detail: FollowPnlDetailResponse,
    /// Follow amount information
    pub follow_amount_info: FollowAmountInfoResponse,
}

/// Curve reset information
/// Matches Kotlin CurveResetInfo
#[derive(Debug, Clone, PartialEq)]
pub struct CurveResetInfo {
    /// Curve start time (String in both Kotlin and Rust)
    pub curve_start_time: String,
    /// Initial time (String in both Kotlin and Rust)
    pub initial_time: String,
    /// Remaining numbers (String in both Kotlin and Rust)
    pub remain_nums: String,
}

/// Custom copy current position (detail or summary)
/// GET /priapi/v5/ecotrade/universals/account/copier/position-detail
/// GET /priapi/v5/ecotrade/universals/account/copier/position-summary
#[derive(Debug, Clone, PartialEq)]
pub struct CustomCopyPositionDto {
    /// Currency
    pub ccy: String,
    /// Instrument type
    pub inst_type: String,
    /// Instrument ID
    pub inst_id: String,
    /// Trade item ID
    pub trade_item_id: String,
    /// Compensation ticket amount
    pub compensation_ticket_amount: String,
    /// Compensation ticket flag
    pub compensation_ticket_flag: bool,
    /// Compensation ticket unit
    pub compensation_ticket_unit: String,
    /// Compensation ticket name
    pub compensation_ticket_name: String,
    /// Compensation ticket state
    pub compensation_ticket_state: String,
    /// Margin mode
    pub mgn_mode: String,
    /// Position side
    pub pos_side: String,
    /// Sub position
    pub sub_pos: String,
    /// Leverage
    pub lever: String,
    /// Open average price
    pub open_avg_px: String,
    /// Initial margin requirement
    pub margin: String,
    /// Unrealized PnL
    pub pnl: String,
    /// PnL ratio
    pub pnl_ratio: f32,
    /// Available sub position
    pub avail_sub_pos: String,
    /// Open time
    pub open_time: i64,
    /// Update time
    pub u_time: i64,
    /// Last price
    pub last: String,
    /// Mark price
    pub mark_px: String,
    /// Notional USD value
    pub notional_usd: String,
    /// Take profit trigger price
    pub tp_trigger_px: String,
    /// Stop loss trigger price
    pub sl_trigger_px: String,
    /// Take profit order price
    pub tp_ord_px: String,
    /// Stop loss order price
    pub sl_ord_px: String,
    /// Take profit trigger type
    pub tp_trigger_type: String,
    /// Stop loss trigger type
    pub sl_trigger_type: String,
    /// Detail IDs (only in summary)
    pub detail_ids: String,
    /// Max sellable amount
    pub max_sellable_amount: String,
    /// Deal volume
    pub deal_volume: String,
    /// Close PnL
    pub close_pnl: String,
    /// Realized PnL
    pub realized_pnl: String,
    /// Fee
    pub fee: String,
    /// Funding fee
    pub funding_fee: String,
    /// Liquidation fee
    pub liquidation_fee: String,
    /// Position type for profit-sharing categorization (wire `posType`):
    /// - "1": Old position (no fees deducted in profit sharing)
    /// - "2": Old position (fees deducted in profit sharing)
    /// - "3": New position after feature launch
    /// 
    /// Consumed by the iOS ongoing-position cell to gate the tappable
    /// PnL-breakdown affordance (shown for new positions). Added for the
    /// `copier_positions` iOS migration; forward-compatible (defaults empty).
    pub pos_type: String,
}

/// Custom copy position history
/// GET /priapi/v5/ecotrade/universals/account/copier/position-history
#[derive(Debug, Clone, PartialEq)]
pub struct CustomCopyPositionHistoryDto {
    /// Instrument ID
    pub inst_id: String,
    /// Position ID (for pagination)
    pub id: String,
    /// Margin mode
    pub mgn_mode: String,
    /// Leverage
    pub lever: String,
    /// Position side
    pub pos_side: String,
    /// Open average price
    pub open_avg_px: String,
    /// PnL ratio
    pub pnl_ratio: f32,
    /// Open time
    pub c_time: i64,
    /// Update time
    pub u_time: i64,
    /// Instrument type
    pub inst_type: String,
    /// Close average price
    pub close_avg_px: String,
    /// Close position type
    pub close_position_type: String,
    /// Realized PnL (pnl in JSON)
    pub realized_pnl: String,
    /// Fee
    pub fee: String,
    /// Funding fee
    pub funding_fee: String,
    /// Close PnL
    pub close_pnl: String,
    /// Liquidation fee
    pub liquidation_fee: String,
    /// Position type
    pub pos_type: String,
    /// Close total position (subPos)
    pub close_total_pos: String,
    /// Currency
    pub ccy: String,
    /// Margin currency
    pub margin_ccy: String,
    /// Contract value
    pub contract_val: String,
    /// Deal volume (for spot)
    pub deal_volume: String,
    /// Margin
    pub margin: String,
    /// Available sub position
    pub avail_sub_pos: String,
    /// Last price
    pub last: String,
    /// Share profit
    pub share_profit: String,
    /// Multiplier
    pub multiplier: String,
    /// Compensation ticket flag
    pub compensation_ticket_flag: bool,
    /// Compensation ticket amount
    pub compensation_ticket_amount: String,
    /// Compensation ticket unit
    pub compensation_ticket_unit: String,
    /// Compensation ticket name
    pub compensation_ticket_name: String,
    /// Compensation ticket state
    pub compensation_ticket_state: String,
    /// Liquidation status
    pub liquidation_status: String,
}

/// User basic information response
/// Contains all profile information for a social trading user
#[derive(Debug, Clone, PartialEq)]
pub struct EcoBasicUserInfoResponse {
    /// Country identifier
    pub country_id: String,
    /// English nickname
    pub en_nick_name: String,
    /// Number of fans/followers
    pub fans_num: String,
    /// Number of users this user follows
    pub followee_num: String,
    /// KYC verification level (visible only to self)
    pub kyc_level: String,
    /// Whether the user has modified global settings
    pub modified_setting: bool,
    /// User's display nickname
    pub nick_name: String,
    /// URL to user's portrait/avatar image
    pub portrait: String,
    /// User's role type in social trading (swap/futures)
    pub role_type: String,
    /// User's bio/signature
    pub sign: String,
    /// Target identifier
    pub target_id: String,
    /// Time when user joined (timestamp string)
    pub time: String,
    /// Unique name identifier for the user
    pub unique_name: String,
    /// Public visibility status
    pub public_status: String,
    /// Whether the current user follows this user (for viewing others)
    pub follow_status: bool,
    /// Strategy role type (for strategy trading)
    pub strategy_role_type: String,
    /// Signal provider role type
    pub signal_role_type: String,
    /// Spot trading role type ("0" = Normal, "1" = LeadTrader, "2" = Copier).
    pub spot_role_type: String,
    /// Whether the user is a quantitative/API trader
    pub is_quantitative_trader: bool,
    /// Translated bio content
    pub translated_bio: Option<String>,
    /// Approved bio content
    pub approved_bio: Option<String>,
    /// User's tier information (for lead traders)
    pub tier: Option<Tier>,
    /// Country name for display
    pub country_name: Option<String>,
    /// Whether to show country tag on profile
    pub show_country_tag: bool,
    /// Detailed country information
    pub country_info: Option<UserBasicInfoCountryInfo>,
    /// Whether this user is a sub-account (BE sends "0"/"1" string on
    /// /priapi/v5/ecotrade/public/basic-info and /user/basic-info).
    /// Forward-compatible: defaults to `false` if BE omits the field.
    pub is_sub_account: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EcoHoldPositionState {
    pub content_state: EcoHoldPositionLoadState,
    /// `None` until the first successful load. Holds the API response 1:1
    /// (mirrors native `HistoryScatterResp { symbol, itemScatterList }`).
    pub data: Option<PositionHistoryScatter>,
    pub error_message: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EcoPnlWeeklyState {
    pub content_state: EcoPnlWeeklyLoadState,
    pub items: Vec<WeeklyPnlChartDataItem>,
    pub error_message: String,
}

/// Empty reminder operation result (UniFFI-compatible)
/// 
/// Represents the result of a subscribe/unsubscribe operation.
/// The API doesn't return data, so we derive the result from the operation type.
#[derive(Debug, Clone, PartialEq)]
pub struct EmptyReminderResult {
    /// Whether the reminder is currently subscribed after this operation.
    /// `true` for subscribe, `false` for unsubscribe.
    pub is_subscribed: bool,
    /// Current status string for native compatibility.
    /// "1" for subscribed, "0" for unsubscribed.
    pub current_status: String,
}

/// First retention popup data
/// Shown when user attempts to cancel copying
#[derive(Debug, Clone, PartialEq)]
pub struct FirstRetentionPopup {
    /// Whether the popup should be visible
    /// Server returns "1" for visible, "2" for not visible
    /// Stored as String to preserve original format for Kotlin serialization
    pub visible: String,
    pub detail: Option<FirstRetentionPopupDetails>,
}

/// Details for first retention popup
/// Contains trader information and performance metrics
#[derive(Debug, Clone, PartialEq)]
pub struct FirstRetentionPopupDetails {
    /// Trader's nickname
    pub nick_name: String,
    /// Trader's unique identifier
    pub unique_name: String,
    /// Trader's portrait/avatar URL
    pub portrait: String,
    /// 7-day yield rate
    pub seven_yield_rate: String,
    /// 90-day yield rate
    pub ninety_yield_rate: String,
    /// Position proportion
    pub proportion: String,
    /// Trading symbol
    pub symbol: String,
    /// List of other followers
    pub followers: Vec<RetentionPopupFollower>,
}

/// Response for first settings submission.
/// 
/// The API returns `List<RelationId?>` where RelationId is a value class
/// that serializes to a simple string. This wrapper provides a consistent
/// struct interface for UniFFI while handling the raw string response.
#[derive(Debug, Clone, PartialEq)]
pub struct FirstSettingsResponseDto {
    /// The created copy relation ID (raw string from API response)
    pub relation_id: String,
}

/// Follow amount detail (used in trader detail)
#[derive(Debug, Clone, PartialEq)]
pub struct FollowAmountDetail {
    /// Maximum follow amount
    pub max_follow_amount: String,
    /// Current follow amount
    pub follow_amount: String,
    /// Investment amount
    pub investment_amount: String,
    /// Initial follow amount
    pub init_follow_amount: String,
    /// Transfer add amount
    pub transfer_add_amount: String,
    /// Transfer reduce amount
    pub transfer_reduce_amount: String,
}

/// Follow amount info
#[derive(Debug, Clone, PartialEq)]
pub struct FollowAmountInfoResponse {
    /// Amount currently occupied in smart copy
    pub follow_amount_occupied: String,
    /// Maximum follow amount (display '--' if 0)
    pub max_follow_amount: String,
}

/// Follow PnL detail
#[derive(Debug, Clone, PartialEq)]
pub struct FollowPnlDetail {
    /// Total follow PnL
    pub follow_pnl: String,
    /// Realized PnL
    pub realized_pnl: String,
    /// Unrealized PnL
    pub upl: String,
    /// Net PnL
    pub net_pnl: String,
    /// Profit sharing amount
    pub profit_sharing: String,
    /// Settled profit sharing
    pub settled_profit_sharing: String,
    /// Pre profit sharing
    pub pre_profit_sharing: String,
    /// Today's PnL
    pub today_pnl: String,
}

/// Follow PnL detail
#[derive(Debug, Clone, PartialEq)]
pub struct FollowPnlDetailResponse {
    /// Total follow PnL
    pub follow_pnl: String,
    /// Realized PnL
    pub realized_pnl: String,
    /// Unrealized PnL
    pub upl: String,
    /// Net PnL
    pub net_pnl: String,
    /// Profit sharing amount
    pub profit_sharing: String,
    /// Settled profit sharing
    pub settled_profit_sharing: String,
    /// Pre profit sharing
    pub pre_profit_sharing: String,
    /// Today's PnL
    pub today_pnl: String,
}

/// Follow trader result (UniFFI-compatible).
#[derive(Debug, Clone, PartialEq)]
pub struct FollowTraderResult {
    /// Response message.
    pub message: String,
    /// Trader unique identifier.
    pub unique_name: String,
    /// Response code from server.
    pub code: String,
}

/// Individual follower details (UniFFI-compatible)
#[derive(Debug, Clone, PartialEq)]
pub struct FollowerDetails {
    pub pnl: String,
    pub portrait: String,
    pub follow_days: String,
    pub follow_time: String,
    pub target_id: String,
    pub coin_symbol: String,
    pub aum: String,
    pub nick_name: String,
    pub unique_name: String,
    pub uid: String,
    pub invested_amount: String,
    pub profit_sharing_ratio: String,
}

/// Individual follower item for UniFFI export
#[derive(Debug, Clone, PartialEq)]
pub struct FollowerListItem {
    /// Avatar URL
    pub portrait: String,
    /// Nickname
    pub nick_name: String,
    /// Follow time (timestamp in milliseconds)
    pub follow_time: String,
    /// Profit/Loss
    pub pnl: String,
    /// Unique name/identifier
    pub unique_name: String,
    /// Avatar target ID
    pub target_id: String,
    /// Coin symbol for margin
    pub coin_symbol: String,
    /// Copy trading principal (AUM)
    pub aum: String,
    /// Days of following
    pub follow_days: String,
    /// User ID
    pub uid: String,
    /// Invested amount (USDT)
    pub invested_amount: String,
    /// Profit sharing ratio
    pub profit_sharing_ratio: String,
}

/// State for Follower List Interactor
#[derive(Debug, Clone, PartialEq)]
pub struct FollowerListState {
    /// Content load state
    pub content_state: FollowerListLoadState,
    /// Followers loaded from API (accumulated across pages)
    pub followers: Vec<FollowerListItem>,
    /// Summary information
    pub summary: FollowerListSummary,
    /// Whether more data can be loaded from server
    pub can_load_more: bool,
    /// Whether the list is empty
    pub is_empty: bool,
    /// Whether the sort header should be visible
    pub is_header_visible: bool,
    /// Current sort type
    pub sort_type: FollowerListSortType,
    /// AUM coin symbol (for sort header display)
    pub sort_coin_symbol: String,
    /// Latest error message from a failed load. Cleared on successful load.
    /// Mirrors native ViewState.error(throwable) carrying the backend message;
    /// matches LeadTradeHistoryState.error_message pattern.
    pub error_message: String,
}

/// Summary information for the trader's follower statistics
#[derive(Debug, Clone, PartialEq)]
pub struct FollowerListSummary {
    /// Current number of followers
    pub follower_num: String,
    /// Maximum allowed followers
    pub follower_limit: String,
    /// Total historical followers
    pub total_follower_num: String,
    /// Total follower PnL
    pub total_follower_pnl: String,
    /// PnL coin symbol (e.g., "USDT")
    pub pnl_coin_symbol: String,
    /// AUM (Assets Under Management)
    pub aum: String,
    /// AUM coin symbol
    pub aum_coin_symbol: String,
    /// 7-day follower number
    pub seven_days_follower_num: String,
    /// 7-day follower ratio
    pub seven_days_follower_ratio: String,
    /// 7-day follower ratio flag
    pub seven_days_follower_ratio_flag: String,
    /// 7-day follower flag
    pub seven_days_follower_flag: String,
    /// Statistics time
    pub stat_time: String,
}

/// Full relation info (used in historical traders and trader detail)
#[derive(Debug, Clone, PartialEq)]
pub struct FullRelationInfo {
    /// Profit sharing ratio
    pub profit_sharing_ratio: f32,
    /// Follow start timestamp
    pub follow_time: i64,
    /// Update timestamp
    pub u_time: i64,
    /// Copy relation ID. Rust field renamed to `copy_rel_id` (instead of `relation_id` with
    /// `#[serde(rename = "copyRelId")]`) so the UniFFI Kotlin field name (camelCase) matches
    /// the JSON key — keeps 3-way alignment with native @SerialName("copyRelId"), avoids
    /// Gson/kotlinx bridge silent corruption (incident 2026-05-18).
    pub copy_rel_id: String,
    /// Copy ratio
    pub copy_ratio: f32,
    /// Copy mode: fixed or ratio
    pub copy_mode: String,
    /// Coin symbol (currency)
    pub coin_symbol: String,
    /// Follow type: 0=NotFollowing, 1=Following, 2=ClosingPosition, 3=Unfollowing, 4=SettingUp
    pub follow_type: String,
    /// Closing mode: 1=MarketClose, 2=CloseByLeadTrader, 3=CloseByMyself.
    /// Rust field renamed to `remain_mgn_deal` to match the API JSON key (server name carries
    /// the legacy "remainMgnDeal" identifier) — 3-way alignment as per copy_rel_id above.
    pub remain_mgn_deal: String,
    /// Whether copier is affiliate invitee
    pub is_affiliate_invitee: bool,
}

/// Historical trader response
/// GET /priapi/v5/ecotrade/universals/relation/copier/history-list
#[derive(Debug, Clone, PartialEq)]
pub struct HistoricalTraderDto {
    /// Trader information
    pub trader_info: PartialTraderInfo,
    /// Full relation information
    pub relation_info: FullRelationInfo,
    /// Follow PnL details
    pub follow_pnl_detail: FollowPnlDetail,
}

/// Historical profit settlement record
/// 
/// Contains information about a past profit settlement event.
/// Used by GET /priapi/v5/ecotrade/account/history-profit (Contract)
/// Used by GET /priapi/v5/ecotrade/spot/account/history-profit (Spot)
#[derive(Debug, Clone, PartialEq)]
pub struct HistoryProfit {
    /// Profit share amount for this settlement
    pub profit_share: String,
    /// Settlement timestamp (Unix timestamp in milliseconds)
    pub settle_time: String,
    /// Number of followers settled in this period
    pub settle_num: String,
    /// Unique identifier for this record (used for pagination)
    pub id: String,
    /// Currency symbol (e.g., "USDT")
    pub symbol: String,
}

/// Instrument body for amend instruments request.
#[derive(Debug, Clone, PartialEq)]
pub struct InstrumentBody {
    /// Instrument ID (e.g., "BTC-USDT-SWAP")
    pub inst_id: String,
    /// Leverage setting
    pub lever: Option<String>,
}

/// Instrument info response for trader's tradeable instruments.
/// 
/// # API
/// GET /priapi/v5/ecotrade/copytrading/relation/copier/trader-instruments-setting
#[derive(Debug, Clone, PartialEq)]
pub struct InstrumentInfoDto {
    /// List of instruments
    pub instruments: Vec<UserInstrumentDto>,
    /// Copy instrument mode
    /// Response values: "1" = custom, "2" = follow trader
    pub copy_instrument_mode: String,
    /// Copy state: "0" = not copying, "1" = copying
    pub copy_state: String,
}

/// Instrument item for UniFFI export
#[derive(Debug, Clone, PartialEq)]
pub struct InstrumentItem {
    /// Instrument ID (e.g., "BTC-USDT-SWAP" for contract, "BTC-USDT" for spot)
    pub inst_id: String,
    /// Instrument name/symbol (e.g., "BTC")
    pub name: String,
}

/// Investment information summary for marketplace.
/// 
/// Contains total asset, PNL information, and breakdown by business line.
#[derive(Debug, Clone, PartialEq)]
pub struct InvestmentInfo {
    /// Total asset value in USDT
    pub total_asset: String,
    /// Today's profit and loss
    pub today_pnl: String,
    /// Historical profit and loss
    pub history_pnl: String,
    /// PNL ratio (percentage) - JSON is string, needs custom deserializer
    pub pnl_ratio: f32,
    /// Currency symbol (e.g., "USDT")
    pub symbol: String,
    /// Timestamp of the data (creation time) - API returns string, needs custom deserializer
    pub timestamp: i64,
    /// Investment breakdown by business line
    pub invest_list: Vec<InvestmentItem>,
}

/// Investment item for a specific business line.
/// 
/// Contains asset and PNL information for each marketplace category.
#[derive(Debug, Clone, PartialEq)]
pub struct InvestmentItem {
    /// Business line type: "STRATEGY", "SWAP", "SPOT", "OPTION"
    pub business_type: String,
    /// Total asset value for this business line
    pub total_asset: String,
    /// Today's profit and loss for this business line
    pub today_pnl: String,
    /// Historical profit and loss for this business line
    pub history_pnl: String,
}

/// Result of validating an invite code
/// 
/// Contains the trader information associated with a valid invite code.
/// This is returned when the invite code validation succeeds.
#[derive(Debug, Clone, PartialEq)]
pub struct InviteCodeLinkInfo {
    /// Unique identifier of the lead trader
    pub trader_unique_name: String,
    /// ID of the private copy link
    pub private_link_id: String,
}

/// Latest trade record
/// Corresponds to LatestRecordDto in Kotlin
/// Note: NumericString, InstId, UniqueName are value classes in Kotlin, represented as String in Rust
#[derive(Debug, Clone, PartialEq)]
pub struct LatestRecord {
    /// Value (NumericString in Kotlin)
    pub value: String,
    /// Size (NumericString in Kotlin)
    pub sz: String,
    /// Alias
    pub alias: String,
    /// Average price (NumericString in Kotlin)
    pub avg_px: String,
    /// Base name
    pub base_name: String,
    /// Create time (deserialized from string via LongAsStringSerializer)
    pub c_time: i64,
    /// Fill time (NumericString in Kotlin)
    pub fill_time: String,
    /// Instrument family
    pub inst_family: String,
    /// Instrument ID (InstId value class in Kotlin)
    pub inst_id: String,
    /// Instrument type
    pub inst_type: String,
    /// Leverage (NumericString in Kotlin)
    pub lever: String,
    /// Nick name
    pub nick_name: String,
    /// Order ID
    pub ord_id: String,
    /// Order type
    pub ord_type: String,
    /// Position side
    pub pos_side: String,
    /// Price (NumericString in Kotlin)
    pub px: String,
    /// Quote name
    pub quote_name: String,
    /// Side
    pub side: String,
    /// Update time (deserialized from string via LongAsStringSerializer)
    pub u_time: i64,
    /// Underlying
    pub uly: String,
    /// Unique name (UniqueName value class in Kotlin)
    pub unique_name: String,
    /// Trade quote currency (nullable)
    pub trade_quote_ccy: Option<String>,
}

/// A single crypto/trading pair item.
/// 
/// Represents one row in the selection list.
#[derive(Debug, Clone, PartialEq)]
pub struct LeadTradeCryptoItem {
    /// Instrument ID (e.g., "BTC-USDT-SWAP" for futures, "BTC-USDT" for spot)
    pub inst_id: String,
    /// Display name (e.g., "BTC", "ETH")
    pub name: String,
    /// Whether this item is currently selected
    pub is_selected: bool,
}

/// Lead Trade Crypto state.
/// 
/// Contains all data for the "Lead trade crypto" selection screen.
/// The `#[interactor::state]` macro adds: `Clone, Debug, PartialEq, uniffi::Record`
/// 
/// # Corresponding UI Elements
/// 
/// | Field | UI Element |
/// |-------|------------|
/// | `instruments` | List of selectable crypto items |
/// | `is_auto_include_new` | "Auto-include new crypto" toggle |
/// | `selected_count` | "Selected (X/Y)" counter |
/// | `total_count` | Total available items count |
/// | `search_query` | Search input field (client-side filtering) |
#[derive(Debug, Clone, PartialEq)]
pub struct LeadTradeCryptoState {
    /// Content loading state (for initial data load)
    pub content_state: ContentLoadState,
    /// Submit loading state (for save operation)
    pub submit_state: ContentLoadState,
    /// Current trade type (Futures/Spot)
    pub trade_type: LeadTradeCryptoTradeType,
    /// All available crypto items with selection status
    pub instruments: Vec<LeadTradeCryptoItem>,
    /// Whether "Auto-include new crypto" is enabled
    pub is_auto_include_new: bool,
    /// Current search query (native can use this for filtering)
    pub search_query: String,
    /// Number of currently selected items
    pub selected_count: i32,
    /// Total number of available items
    pub total_count: i32,
}

/// A single history trade item.
/// 
/// Represents one row in the history list.
#[derive(Debug, Clone, PartialEq)]
pub struct LeadTradeHistoryItem {
    /// Position ID (unique identifier)
    pub id: String,
    /// Instrument ID (e.g., "BTC-USDT-SWAP")
    pub inst_id: String,
    /// Instrument type (e.g., "SWAP", "SPOT")
    pub inst_type: String,
    /// Display name (e.g., "XRPUSDT Perp")
    pub display_name: String,
    /// Position side: "long" or "short"
    pub pos_side: String,
    /// Margin mode: "isolated" or "cross"
    pub mgn_mode: String,
    /// Leverage (e.g., "3x")
    pub lever: String,
    /// Entry price
    pub entry_price: String,
    /// Exit price
    pub exit_price: String,
    /// Realized PnL (from "pnl" JSON field)
    pub realized_pnl: String,
    /// Realized PnL percentage — raw server string, no parsing.
    /// Native: TraderPositionHistoryItem.pnlRatio is plain String (PositionHistoryResponse.kt:210).
    /// Bridge converts via toSafeFloat()/BigDecimal at the consumer site.
    pub pnl_ratio: String,
    /// Closed quantity (raw subPos)
    pub close_amount: String,
    /// Closed amount in USDT (subPos × closeAvgPx × contractVal)
    pub closed_usdt: String,
    /// Quote currency (e.g., "USDT", "XRP")
    pub ccy: String,
    /// Open time (timestamp in ms)
    pub open_time: i64,
    /// Close time (timestamp in ms)
    pub close_time: i64,
    /// Whether PnL is positive
    pub is_profit: bool,
    /// Trading fee
    pub fee: String,
    /// Close PnL (for yield details calculation, same JSON "pnl" field)
    pub close_pnl: String,
    /// Funding fee
    pub funding_fee: String,
    /// Liquidation status (raw string, "1" = liquidated)
    pub liquidation_status: String,
    /// Contract value (raw, for position formatting)
    pub contract_val: String,
    /// Unique name of the trader
    pub unique_name: String,
    /// Position type (for yield details: 1=OldNoFees, 2=OldWithFees, 3=NewPost)
    pub pos_type: Option<HistoryPosType>,
    /// Close position type (1=Partial, 2=Full, 3=Liquidated, 4=ForceReduce, 5=ADL)
    pub close_position_type: Option<ClosePositionType>,
    /// Liquidation fee
    pub liquidation_fee: String,
    /// Sub position quantity (raw subPos for formatPosition)
    pub sub_pos: String,
    /// Deal volume (Spot only — raw value, bridge formats via formatDealVolume)
    pub deal_volume: String,
    /// Aggregated position unique id (Spot share / detail navigation)
    pub trade_item_id: String,
}

/// Lead Trade History state.
/// 
/// Contains all data for the "My Lead Trades → History" screen.
/// 
/// # Corresponding UI Elements
/// 
/// | Field | UI Element |
/// |-------|------------|
/// | `items` | List of closed trades |
/// | `trade_type` | Tab selection (Futures/Spot) |
/// | `has_more` | Whether more pages available |
/// | `current_page` | Current page number |
#[derive(Debug, Clone, PartialEq)]
pub struct LeadTradeHistoryState {
    /// Content loading state (for initial data load)
    pub content_state: ContentLoadState,
    /// Load more state (for pagination)
    pub load_more_state: ContentLoadState,
    /// Current trade type (Futures/Spot)
    pub trade_type: LeadTradeHistoryTradeType,
    /// Optional uniqueName — when set, fetches another trader's public history;
    /// when empty, fetches the current logged-in user's own history.
    /// Mirrors native SpotLeadHistoryOrderViewModel.kt:40-46 / MyHistoryLeadTradingViewModel.kt:120-131
    pub unique_name: String,
    /// History items
    pub items: Vec<LeadTradeHistoryItem>,
    /// Current page number (1-based)
    pub current_page: i32,
    /// Page size
    pub page_size: i32,
    /// Whether there are more pages
    pub has_more: bool,
    /// Cursor for pagination (last item's timestamp)
    pub cursor_after: Option<String>,
    /// Error message if any
    pub error_message: String,
}

/// Pre-check result for lead trader application (UniFFI export).
/// 
/// This is the public type exposed to native platforms via UniFFI.
#[derive(Debug, Clone, PartialEq)]
pub struct LeadTraderApplicationPreCheck {
    /// Whether the pre-check passed
    pub is_pass: bool,
    /// Whether to show a popup on failure
    pub fail_with_popup: bool,
    /// Toast message to display
    pub toast_msg: String,
    /// Optional popup content details
    pub popup_contents: Option<LeadTraderApplicationPreCheckPopupContent>,
}

/// Popup content for pre-check failure (UniFFI export).
#[derive(Debug, Clone, PartialEq)]
pub struct LeadTraderApplicationPreCheckPopupContent {
    /// KYC level check result
    pub kyc_level: LeadTraderApplicationPreCheckSubItem,
    /// Copy relation check result
    pub copy_relation: LeadTraderApplicationPreCheckSubItem,
    /// Account mode check result
    pub account_mode: LeadTraderApplicationPreCheckSubItem,
}

/// Individual check item result (UniFFI export).
#[derive(Debug, Clone, PartialEq)]
pub struct LeadTraderApplicationPreCheckSubItem {
    /// Whether this specific check passed
    pub is_pass: bool,
    /// Business type (e.g., "SPOT", "SWAP")
    pub biz_type: Option<String>,
}

/// Lead Trader Application State - the main Record exported to Native.
/// 
/// # Design Principles
/// 
/// 1. **Flat structure**: No nested Records (EffectPayload merged as flat fields)
/// 2. **Readable enums**: Use ContentLoadState/EffectType for readability
/// 3. **Effect payload inline**: Effect data embedded as flat fields
/// 
/// # Usage (Native)
/// 
/// ```kotlin
/// val state = interactor.getState()
/// 
/// // Check loading state with readable enum
/// when (state.contentState) {
/// ContentLoadState.IDLE -> showIdle()
/// ContentLoadState.LOADING -> showLoading()
/// ContentLoadState.SUCCESS -> showContent()
/// ContentLoadState.ERROR -> showError()
/// }
/// 
/// // Handle effect
/// if (state.effectId > lastEffectId) {
/// when (state.effectType) {
/// EffectType.PRECHECK_NOT_PASSED_TOAST -> showToast(state.effectMessage)
/// EffectType.PRECHECK_NOT_PASSED_POPUP -> showPopup(
/// isKycPass = state.effectIsKycPass,
/// isCopyRelationPass = state.effectIsCopyRelationPass,
/// // ...
/// )
/// }
/// interactor.ackEffect(state.effectId)
/// }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct LeadTraderApplicationState {
    /// Main content loading state
    pub content_state: ContentLoadState,
    /// Pre-check loading state
    pub precheck_state: ContentLoadState,
    /// Submit loading state
    pub submit_state: ContentLoadState,
    /// Whether user is already applying or has succeeded
    pub is_applying_or_success: bool,
    /// Swap (Futures) application status
    pub swap_status: i32,
    /// Spot application status
    pub spot_status: i32,
    /// Strategy (Bot) application status
    pub strategy_status: i32,
    /// Signal provider application status
    pub signal_provider_status: i32,
    /// Current effect type
    pub effect_type: EffectType,
    /// Effect ID for acknowledgment
    pub effect_id: i64,
    /// Effect message (for toast/error effects)
    pub effect_message: String,
    /// Effect biz type (for submit effects)
    pub effect_biz_type: String,
    /// Effect: KYC check passed (for popup)
    pub effect_is_kyc_pass: bool,
    /// Effect: Account mode check passed (for popup)
    pub effect_is_account_mode_pass: bool,
    /// Effect: Copy relation check passed (for popup)
    pub effect_is_copy_relation_pass: bool,
    /// Effect: Copy relation biz type (for popup)
    pub effect_copy_relation_biz_type: String,
    /// Effect: Whether popup should be shown (for PopupStatusFetched)
    pub effect_should_show_popup: bool,
    /// List of instruments available for lead trading
    pub instruments: Vec<UserInstrumentInfo>,
}

/// Display projection of one `LeaderRankItem`. Native renders this and decides
/// nothing else.
#[derive(Debug, Clone, PartialEq)]
pub struct LeadTraderCardPresentation {
    /// Free text, not a number: nicknames are frequently all-digit (e.g. a
    /// user picking "123456") and must render byte-for-byte. This is a plain
    /// `String`, NOT a `DisplayString`, so it can never be routed through the
    /// flattener's numeric-localizing path (`okte_localizeIfNumeric`), which
    /// would otherwise corrupt a numeric nickname into a grouped number
    /// (e.g. "123456" -> "123,456") or strip meaningful digits (e.g. "007").
    pub nickname: String,
    pub avatar_url: String,
    pub target_id: String,
    pub is_quant_trader: bool,
    /// `None` = hide the badge (ordinary tier, unknown tier key, or blank name).
    pub tier_level: Option<String>,
    /// `None` = hide the leverage view.
    pub leverage: Option<LeadTraderLeverage>,
    pub time_period_label: DisplayString,
    pub yield_ratio: DisplayString,
    pub yield_tone: LeadTraderTone,
    pub pnl_amount: DisplayString,
    pub pnl_tone: LeadTraderTone,
    /// Chart y-values in order. Already defaulted to `["0","0"]` when empty, so
    /// native never invents points.
    pub chart_values: Vec<String>,
    pub chart_is_negative: bool,
    pub coin_bars: Vec<LeadTraderCoinBar>,
    pub follower_count: DisplayString,
    pub follower_limit: DisplayString,
    /// `has_affiliate_invitee` — native underlines the limit and makes it tappable.
    pub is_limit_interactive: bool,
    pub aum: DisplayString,
    pub days_leading: DisplayString,
}

/// One asset-distribution bar. Populated in full by the coin-bar projection.
#[derive(Debug, Clone, PartialEq)]
pub struct LeadTraderCoinBar {
    /// Display-ready icon URL. `""` means *no icon* — native must NOT fall back.
    pub icon_url: String,
    /// Coin name. `None` for the aggregate ("everything else") bar.
    pub label: Option<DisplayString>,
    /// Normalised bar width in `0.0..=1.0`. Bars are ordered widest first, with
    /// the aggregate bar last.
    pub width_ratio: f64,
    /// Stacked icons on the aggregate bar, already capped at 4.
    pub extra_icon_urls: Vec<String>,
}

/// The leverage tag. `None` on the card means the tag is hidden entirely.
#[derive(Debug, Clone, PartialEq)]
pub struct LeadTraderLeverage {
    /// Drives the red / green tag style natively.
    pub side: LeadTraderPositionSide,
    /// e.g. `["Long", "3.5x"]` — native joins with a single space. Same shape as
    /// VanguardShare's `title_segments`, so no registry template is required.
    pub segments: Vec<DisplayString>,
}

/// Filter parameters for lead trader list.
#[derive(Debug, Clone, PartialEq)]
pub struct LeadTraderListFilter {
    /// Order type filter
    pub order_type: Option<String>,
    /// Time period filter
    pub latest_num: Option<String>,
    /// Country ID filter
    pub country_id: Option<String>,
    /// Minimum assets filter
    pub assets_low: Option<String>,
    /// Maximum assets filter
    pub assets_high: Option<String>,
    /// Minimum AUM filter
    pub aum_low: Option<String>,
    /// Maximum AUM filter
    pub aum_high: Option<String>,
    /// Duration as lead trader filter
    pub trader_period: Option<String>,
    /// Trader vacancy filter
    pub full_state: Option<String>,
    /// API trader filter
    pub api_trader: Option<String>,
    /// Win ratio filter
    pub win_ratio: Option<String>,
    /// Instrument number limit (defaults to "4", matching native SocialApi @Query instNumLimit default)
    pub inst_num_limit: Option<String>,
}

/// Lead Trader List state.
#[derive(Debug, Clone, PartialEq)]
pub struct LeadTraderListState {
    /// Content loading state
    pub content_state: ContentLoadState,
    /// Current business line (Contract/Spot)
    pub business_line: LeadTraderListBusinessLine,
    /// Data version for cache invalidation
    pub data_version: String,
    /// Current page number (1-based)
    pub next_page: i32,
    /// Page size (default 20)
    pub page_size: i32,
    /// Whether there are more pages to load
    pub has_more: bool,
    /// Total pages (from API response)
    pub total_pages: i32,
    /// List of ranked traders
    pub ranks: Vec<LeaderRankItem>,
    /// Display projection of [`Self::ranks`], index-aligned.
    /// 
    /// iOS renders from this; Android ignores it (`mapRustState` reads `ranks`).
    /// Kept on the state rather than behind a method so `ranks` and `cards`
    /// always come from one atomic snapshot — the two are indexed together in
    /// `cellForRowAt`, and a length mismatch would be an out-of-range crash.
    /// 
    /// `#[uniffi(default = [])]` keeps Kotlin/Swift memberwise constructors
    /// source-compatible (precedent: `CoinAssetRatio::resolved_coin_url`).
    pub cards: Vec<LeadTraderCardPresentation>, // default = []
    /// Selected trader's unique name
    pub selected_unique_name: String,
    /// Selected trader's index in the list
    pub selected_index: i32,
    /// Last filter used for `load_data`. `refresh` and `load_more` reuse this.
    /// Bridge reads this to know the current sort/period/country selection
    /// (e.g., to highlight the active sort dropdown option).
    /// Mutated via `load_data`, `set_sort_mode`, `set_time_period`, `set_country_filter`.
    pub current_filter: LeadTraderListFilter,
    /// Last error message — set on fetch failure, cleared on success.
    /// Bridge MUST surface this via `OKServerException(message = rs.errorMessage)`
    /// to preserve native error parity (see skill Error Display Native Parity Rule).
    pub error_message: String,
}

/// Lead trader profit sharing detail for a lead mode (public or private)
/// 
/// Contains the min/max sharing ratios and limit count.
/// Used by lead trader settings use case.
#[derive(Debug, Clone, PartialEq)]
pub struct LeadTraderProfitSharingDetail {
    /// Minimum profit sharing ratio as decimal string (e.g., "0")
    pub min_profit_sharing_ratio: String,
    /// Maximum profit sharing ratio as decimal string (e.g., "0.5")
    pub max_profit_sharing_ratio: String,
    /// Limit number as string (e.g., "500")
    pub limit_num: String,
}

/// Lead trader profit sharing ratio limit domain model (UniFFI-compatible)
/// 
/// Contains lead mode and profit sharing details for both public and private modes.
/// Used by lead trader settings use case.
#[derive(Debug, Clone, PartialEq)]
pub struct LeadTraderProfitSharingRatioLimit {
    /// Lead mode: "1" = public, "2" = private
    pub lead_mode: String,
    /// Profit sharing detail for public mode
    pub public_profit_sharing_detail: LeadTraderProfitSharingDetail,
    /// Profit sharing detail for private mode
    pub private_profit_sharing_detail: LeadTraderProfitSharingDetail,
}

/// Instrument info for lead trading.
/// 
/// Represents a trading pair that the lead trader can use.
#[derive(Debug, Clone, PartialEq)]
pub struct LeadTraderSettingsInstrument {
    /// Instrument ID (e.g., "BTC-USDT-SWAP", "ETH-USDT")
    pub inst_id: String,
    /// Display name (e.g., "BTC", "ETH")
    pub name: String,
    /// Whether this instrument is selected for lead trading
    pub is_selected: bool,
}

/// Min copy amount configuration.
/// 
/// Contains the minimum follow amount settings for both custom and smart copy.
#[derive(Debug, Clone, PartialEq)]
pub struct LeadTraderSettingsMinCopyAmount {
    /// Current custom min follow amount (can be empty/null)
    pub custom_amount: String,
    /// Min allowed custom amount
    pub custom_min: String,
    /// Max allowed custom amount
    pub custom_max: String,
    /// Current smart sync min follow amount (can be empty/null)
    pub smart_amount: String,
    /// Min allowed smart amount
    pub smart_min: String,
    /// Max allowed smart amount
    pub smart_max: String,
    /// Currency symbol (e.g., "USDT")
    pub symbol: String,
}

/// Lead Trader Settings state.
/// 
/// This state contains all the settings data for the "My Lead Trades - Settings" page.
/// Native side reads this state and renders the UI accordingly.
#[derive(Debug, Clone, PartialEq)]
pub struct LeadTraderSettingsState {
    /// Content loading state
    pub content_state: ContentLoadState,
    /// Current trade type (Futures/Spot)
    pub trade_type: LeadTraderSettingsTradeType,
    /// Number of instruments for lead trading
    pub instrument_count: i32,
    /// Full list of instruments for lead trading
    pub instruments: Vec<UserInstrumentInfo>,
    /// Profit sharing ratio as string (e.g., "8" for 8%)
    pub profit_sharing_ratio: String,
    /// Copy mode restriction (Futures only)
    pub copy_mode_restriction: CopyModeRestriction,
    /// Min copy amount configuration
    pub min_copy_amount: LeadTraderSettingsMinCopyAmount,
    /// Whether private mode is enabled (Futures only)
    pub is_private_mode: bool,
    /// Whether strategy protection is enabled
    pub is_strategy_protection: bool,
    /// Whether auto-add new assets is enabled
    pub is_auto_add_new_asset: bool,
    /// Whether user is a Swap/Futures leader
    pub is_swap_leader: bool,
    /// Whether user is a Spot leader
    pub is_spot_leader: bool,
    /// Whether user has open positions (affects exit ability)
    pub has_open_positions: bool,
    /// Exit application status: 0=None, 1=Reviewing, 2=Applied
    pub exit_status: i32,
    /// Min private profit sharing ratio (for confirmation popup)
    pub private_min_ratio: f64,
    /// Max private profit sharing ratio (for confirmation popup)
    pub private_max_ratio: f64,
    /// Max public profit sharing ratio (for turning off private mode)
    pub public_max_ratio: f64,
}

/// Parameters for [`OKSTCoreLeaderRankUseCase::fetch_leader_rank_list`], passed as a
/// SINGLE UniFFI Record instead of many separate arguments.
/// 
/// Why a single record: the previous signature took 10 separate args, each lowered as
/// a `RustBuffer.ByValue` (9 String/Option<String> + 1 enum). On Android the JNA
/// by-value marshalling of that many struct-by-value args misaligned, so an argument's
/// RustBuffer `len` was read as garbage and `RustBuffer::from_vec` panicked
/// "RustBuffer length exceeds capacity" (uniffi_core rustbuffer.rs:179) BEFORE the Rust
/// body ran — the KLineCopy card showed a network-error state. Collapsing to one record
/// arg = one RustBuffer passed by value, which marshals reliably. See incident 2026-06-29.
#[derive(Debug, Clone, PartialEq)]
pub struct LeaderRankFetchParams {
    /// Page number (1-based)
    pub start: String,
    /// Page size
    pub size: String,
    /// Data version for cache invalidation (optional)
    pub data_version: Option<String>,
    /// Time period filter (e.g., "7", "30", "90") (optional)
    pub latest_num: Option<String>,
    /// Order type filter (e.g., "pnl", "yield") (optional)
    pub order_type: Option<String>,
    /// Country ID filter (optional)
    pub country_id: Option<String>,
    /// API trader filter (optional)
    pub api_trader: Option<String>,
    /// Full state filter (optional)
    pub full_state: Option<String>,
    /// Instrument number limit (optional)
    pub inst_num_limit: Option<String>,
    /// Trading type: Contract (futures) or Spot
    pub business_line: LeaderRankBusinessLine,
    /// Minimum lead trader assets filter (optional)
    pub assets_low: Option<String>, // default = None
    /// Maximum lead trader assets filter (optional)
    pub assets_high: Option<String>, // default = None
    /// Minimum AUM filter (optional)
    pub aum_low: Option<String>, // default = None
    /// Maximum AUM filter (optional)
    pub aum_high: Option<String>, // default = None
    /// Duration as lead trader filter (e.g., "30", "60", "90") (optional)
    pub trader_period: Option<String>, // default = None
    /// Minimum win ratio filter (optional)
    pub win_ratio: Option<String>, // default = None
}

/// Instrument information for traded pairs in leader rank list.
/// 
/// Mirrors ObjC `OKGTEContractSelectModel` for instruments.
/// Named `LeaderRankInstrument` to avoid conflict with `InstrumentInfo` in core_trading.
#[derive(Debug, Clone, PartialEq)]
pub struct LeaderRankInstrument {
    /// Coin name (e.g., "BTC")
    pub name: String,
    /// Instrument ID (e.g., "BTC-USDT-SWAP")
    pub inst_id: String,
}

/// Individual trader in the leader rank list.
/// 
/// Mirrors ObjC `OKGTESquareCellModel`.
#[derive(Debug, Clone, PartialEq)]
pub struct LeaderRankItem {
    /// User nickname
    pub nick_name: String,
    /// Unique user identifier
    pub unique_name: String,
    /// Avatar URL
    pub portrait: String,
    /// Custom avatar ID (targetId in API)
    pub target_id: String,
    /// Total profit/loss (PnL) - sortable
    pub pnl: String,
    /// Yield ratio (return rate) - sortable
    pub yield_ratio: String,
    /// Win ratio (winning percentage) - sortable
    pub win_ratio: String,
    /// Follower total profit/loss - sortable
    pub follow_pnl: String,
    /// Assets Under Management (AUM) - sortable
    pub aum: String,
    /// Currency symbol (e.g., "USDT")
    pub symbol: String,
    /// Current number of followers - sortable
    pub follower_num: String,
    /// Cumulative follower count (historical) - not sortable
    pub history_follower_num: String,
    /// Maximum allowed followers
    pub follower_limit: String,
    /// Follow type: "0" = not followed, "1" = followed
    pub follow_type: String,
    /// Copy relationship ID (if following)
    pub copy_rel_id: String,
    /// Empty slot reminder state: "0" = not subscribed, "1" = subscribed
    pub empty_minder_state: String,
    /// System limit reached - no more followers allowed
    pub full_status: bool,
    /// Number of days leading trades
    pub initial_day: String,
    /// API trader flag: "0" = not quant trader, "1" = quant trader
    pub api_trader: String,
    /// Leverage value
    pub leverage: String,
    /// Total number of instruments led
    pub total_lead_inst_num: String,
    /// Trader card time dimension
    pub trader_card_time_dimension: String,
    /// Channel (numeric)
    pub channel: i32,
    /// Whether copier is an affiliate invitee
    pub is_affiliate_invitee: bool,
    /// Whether trader has any affiliate invitees
    pub has_affiliate_invitee: bool,
    /// Yield rate curve data points
    pub rates: Vec<RateChartData>,
    /// Coin asset ratios (usually empty in this API)
    pub coin_asset_ratios: Vec<CoinAssetRatio>,
    /// Traded instruments (max 5)
    pub instruments: Vec<LeaderRankInstrument>,
    /// Trader tier information
    pub tier: Option<TraderTier>,
    /// Resolved follow/copy state (native `FollowTypeEnum.map`).
    pub follow_state: FollowState,
    /// Resolved trader-card time period (native `timeDimMap2TimePeriod`).
    pub time_period: LeaderTimePeriod,
    /// Quant (API) trader flag (native `apiTrader == "1"`).
    pub is_quant: bool,
    /// Subscribed to the empty-slot reminder (native `emptyMinderState == "1"`).
    pub is_subscribed: bool,
    /// PnL direction for the UI color decision: `1` = up, `-1` = down, `0` = neutral.
    /// Sinks the SIGN decision of native `getRateColor(pnl)`; the theme-aware color
    /// resource itself stays in the Kotlin/Swift layer.
    pub pnl_direction: i32,
}

/// Response from the public follow-rank API.
/// 
/// Mirrors ObjC `OKTESquareAllModel`.
/// 
/// # Endpoints
/// - GET /priapi/v5/ecotrade/public/follow-rank (Contract)
/// - GET /priapi/v5/ecotrade/public/spot/follow-rank (Spot)
#[derive(Debug, Clone, PartialEq)]
pub struct LeaderRankResponse {
    /// Data version for cache invalidation
    pub data_version: String,
    /// Total number of pages
    pub pages: i32,
    /// List of ranked traders
    pub ranks: Vec<LeaderRankItem>,
}

/// Lever interval data from the API.
/// 
/// # API Response Format (from sample log_get_ecotrade_public_lever_interval_sample_success.txt)
/// ```json
/// {
/// "minLevel": "1",
/// "maxLevel": "20",
/// "percents": "1,2,3,5,10,15,20"
/// }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct LeverIntervalDto {
    /// Minimum leverage level (e.g., "1", "0.01").
    pub min_level: String,
    /// Maximum leverage level (e.g., "20", "100").
    pub max_level: String,
    /// Comma-separated list of preset leverage percentages (e.g., "1,2,3,5,10,15,20").
    pub percents: String,
}

/// Result of fetching lever interval settings.
/// 
/// This is the domain type returned to native layers after processing
/// the API response.
#[derive(Debug, Clone, PartialEq)]
pub struct LeverIntervalResult {
    /// Minimum leverage level as string.
    pub min_level: String,
    /// Maximum leverage level as string.
    pub max_level: String,
    /// Comma-separated list of preset percentages.
    pub percents: String,
    /// Parsed array of preset percentage strings.
    pub percents_array: Vec<String>,
    /// Minimum leverage as f64 (0.0 if parsing failed).
    pub min_level_value: f64,
    /// Maximum leverage as f64 (0.0 if parsing failed).
    pub max_level_value: f64,
}

/// Leverage data for a position side (long or short).
/// 
/// # API Response Format
/// ```json
/// {
/// "lever": "12.9",
/// "posSide": "long"
/// }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct LeverageDataDto {
    /// Leverage value as string (e.g., "3", "12.9", "20").
    /// Can include decimals.
    pub lever: String,
    /// Position side: "long" or "short".
    pub pos_side: String,
}

/// Input for loading trader data.
#[derive(Debug, Clone, PartialEq)]
pub struct LoadTraderDataInput {
    /// Trader's unique name
    pub trader_unique_name: String,
    /// Copy relation ID (optional, for amend flows)
    pub copy_rel_id: Option<String>,
}

/// Request body for locking a voucher
/// Used in POST /priapi/v5/ecotrade/marketplace/lock-voucher
#[derive(Debug, Clone, PartialEq)]
pub struct LockVoucherBody {
    /// Batch number for the voucher
    pub batch_no: String,
    /// Optional underlying asset identifier
    pub underlying_id: Option<String>,
}

/// Loss voucher information.
#[derive(Debug, Clone, PartialEq)]
pub struct LossVoucherDto {
    /// Voucher amount
    pub amount: String,
    /// Currency unit
    pub unit: String,
}

/// Item representing a managed copier (for UI display)
#[derive(Debug, Clone, PartialEq)]
pub struct ManageCopyTraderItem {
    /// User ID
    pub uid: String,
    /// User avatar URL
    pub portrait: String,
    /// User nickname
    pub nick_name: String,
    /// Unique name (identifier)
    pub unique_name: String,
    /// Investment amount
    pub invested_amount: String,
    /// Profit and Loss
    pub pnl: String,
    /// Profit sharing ratio (as decimal, e.g., "0.08" for 8%)
    pub profit_sharing_ratio: String,
    /// Settled profit
    pub settled_profit: String,
    /// Coin symbol (e.g., "USDT")
    pub coin_symbol: String,
    /// Number of days following
    pub follow_days: String,
    /// Target ID
    pub target_id: String,
    /// Trade volume
    pub trade_volume: String,
    /// Whether this is private traffic
    pub is_private_traffic: bool,
    /// Whether this copier can be removed
    pub removable: bool,
    /// Whether this is an affiliate invitee
    pub is_affiliate_invitee: bool,
    /// Selected for removal
    pub is_selected: bool,
}

/// Complete state for the Manage Copy Traders page
#[derive(Debug, Clone, PartialEq)]
pub struct ManageCopyTradersState {
    /// Current load state
    pub content_state: ManageCopyTradersLoadState,
    /// List of copiers
    pub copiers: Vec<ManageCopyTraderItem>,
    /// Summary information
    pub summary: ManageCopyTradersSummary,
    /// Whether more data can be loaded
    pub can_load_more: bool,
    /// Current page number
    pub current_page: i32,
    /// Whether the list is empty
    pub is_empty: bool,
    /// Current sort field
    pub sort_field: ManagedCopierSortField,
    /// Current sort order
    pub sort_order: ManagedCopierSortOrder,
    /// Effect ID (for tracking one-shot events)
    pub effect_id: i64,
    /// Effect type
    pub effect_type: ManageCopyTradersEffectType,
    /// Effect message (error message, etc.)
    pub effect_message: String,
    /// Total AUM (e.g., "1234.56")
    pub total_aum: String,
    /// Current follower count (e.g., "5")
    pub follower_num: String,
    /// Maximum follower limit (e.g., "500")
    pub follower_limit: String,
    /// Whether this trader is an affiliate trader
    pub is_affiliate_trader: bool,
    /// Affiliate trade volume (e.g., "12345.67")
    pub affiliate_trade_volume: String,
}

/// Summary information for the page header
#[derive(Debug, Clone, PartialEq)]
pub struct ManageCopyTradersSummary {
    /// Total number of copiers
    pub total_count: i32,
    /// Current page number
    pub current_page: i32,
}

/// Managed copier domain model (UniFFI-compatible)
/// Contains information about an individual managed copier
#[derive(Debug, Clone, PartialEq)]
pub struct ManagedCopier {
    pub uid: String,
    pub portrait: String,
    pub nick_name: String,
    pub unique_name: String,
    pub invested_amount: String,
    pub pnl: String,
    pub profit_sharing_ratio: String,
    pub settled_profit: String,
    pub coin_symbol: String,
    pub follow_days: String,
    pub target_id: String,
    pub trade_volume: String,
    pub is_private_traffic: bool,
    pub removable: bool,
    pub is_affiliate_invitee: bool,
}

/// Wrapper for list of managed copiers (UniFFI-compatible)
/// Used for async function return type
#[derive(Debug, Clone, PartialEq)]
pub struct ManagedCopiersList {
    pub copiers: Vec<ManagedCopier>,
}

/// Request body for marking profit condition
#[derive(Debug, Clone, PartialEq)]
pub struct MarkProfitConditionBody {
    /// Mark condition version
    pub mark_condition_version: String,
}

/// Investment info item for the marketplace home page (UniFFI-compatible)
#[derive(Debug, Clone, PartialEq)]
pub struct MarketplaceHomeInvestmentItem {
    pub total_asset: String,
    pub today_pnl: String,
    pub history_pnl: String,
    pub pnl_ratio: f32,
    pub symbol: String,
}

/// Aggregate state for the Marketplace Home Interactor
#[derive(Debug, Clone, PartialEq)]
pub struct MarketplaceHomeState {
    /// Content load state
    pub content_state: MarketplaceHomeLoadState,
    /// Whether the server is available
    pub is_server_valid: bool,
    /// Current user's role type
    pub role_type: MarketplaceHomeRoleType,
    /// Whether the user is currently logged in
    pub is_logged_in: bool,
    /// Current user's avatar/portrait URL (from basic info API)
    pub avatar_url: String,
    /// Current user's display nickname (from basic info API).
    /// Mirrors native `userData.avatarInfo.nickName` — the bridge surfaces this
    /// into `AvatarInfo.nickName` so downstream UI / share / profile-tap navigation
    /// has the same user-identifying string the native path produces.
    pub avatar_nick_name: String,
    /// Current user's target ID (from basic info API).
    /// Mirrors native `userData.avatarInfo.targetId` — needed for profile-tap deep-links.
    pub avatar_target_id: String,
    /// Whether the current user is a private lead trader
    pub is_private_lead_trader: bool,
    /// Home banners. Full DTO preserved so the bridge can construct both
    /// `MarketplaceBannerUiModel.NormalBanner` and `.VoucherBanner` (voucher fields
    /// like `batchNo`, `expireTime`, APR info live inside `voucher_info` / `user_voucher`).
    pub banners: Vec<NmpBannerDto>,
    /// Top trade experts
    pub trade_experts: Vec<MarketplaceHomeTradeExpertItem>,
    /// Investment information
    pub investment_info: Vec<MarketplaceHomeInvestmentItem>,
    /// Marketplace trade info
    pub trade_info: Vec<MarketplaceHomeTradeInfoItem>,
    /// Tier upgrade popup info
    pub tier_upgrade: Option<MarketplaceHomeTierUpgrade>,
    /// Received coupons for dialog display
    pub received_coupons: Vec<CouponCenterItemResponse>,
    /// Trader application popup result
    pub trader_application_popup: Option<MarketplaceHomeTraderApplicationPopup>,
    /// Error message when `content_state == Error`. Mirrors native parity:
    /// the binder reads this as `OKServerException.message` so users see the backend reason
    /// instead of the localized generic "network error" fallback.
    pub error_message: String,
}

/// Tier upgrade popup data (UniFFI-compatible)
#[derive(Debug, Clone, PartialEq)]
pub struct MarketplaceHomeTierUpgrade {
    pub should_pop_up: bool,
    pub tier_key: String,
    pub tier_level: String,
    pub tier_name: String,
}

/// Trade expert item for the marketplace home page (UniFFI-compatible)
#[derive(Debug, Clone, PartialEq)]
pub struct MarketplaceHomeTradeExpertItem {
    pub unique_name: String,
    pub nick_name: String,
    pub portrait: String,
    pub target_id: String,
    pub yield_rate: String,
    pub risk_level: String,
    pub fans_num: String,
    pub follow_status: String,
}

/// Trade info item for the marketplace home page (UniFFI-compatible)
#[derive(Debug, Clone, PartialEq)]
pub struct MarketplaceHomeTradeInfoItem {
    pub symbol: String,
    pub pnl_ratio: String,
    pub total_asset: String,
    pub total_investment: String,
    pub history_pnl: String,
    pub today_pnl: String,
    pub trade_type: String,
}

/// Trader application popup result (UniFFI-compatible)
#[derive(Debug, Clone, PartialEq)]
pub struct MarketplaceHomeTraderApplicationPopup {
    pub should_show: bool,
    pub biz_type: String,
}

/// Marketplace trade info for copier dashboard.
/// 
/// Contains asset and PNL information for the user's copy trading portfolio.
/// All numeric values are returned as strings from the API.
#[derive(Debug, Clone, PartialEq)]
pub struct MarketplaceTradeInfoDto {
    /// Currency symbol (e.g., "USDT")
    pub symbol: String,
    /// PNL ratio as string (e.g., "0", "0.05")
    pub pnl_ratio: String,
    /// Creation timestamp in milliseconds as string
    /// Can be null/missing in some responses
    pub c_time: Option<String>,
    /// Total asset value as string (e.g., "0", "1000.50")
    pub total_asset: String,
    /// Total investment amount as string
    pub total_investment: String,
    /// Historical PNL as string
    pub history_pnl: String,
    /// Today's PNL as string
    pub today_pnl: String,
    /// Trade type: "SWAP" or "SPOT"
    pub trade_type: String,
}

/// Available amount for copy trading.
/// 
/// # API
/// GET /priapi/v5/ecotrade/copytrading/relation/copier/available-amount
#[derive(Debug, Clone, PartialEq)]
pub struct MaxAvailableAmountDto {
    /// Available amount for copy trading
    pub available_amount: Option<String>,
}

/// Min follow amount config domain model (UniFFI-compatible)
/// 
/// Contains settings for both regular and smart follow amounts.
/// Used by lead trader settings use case.
#[derive(Debug, Clone, PartialEq)]
pub struct MinFollowAmountConfig {
    /// Settings for minimum follow amount (regular copy)
    pub min_follow_amount: MinFollowAmountSettings,
    /// Settings for minimum smart follow amount (smart copy)
    pub min_smart_follow_amount: MinFollowAmountSettings,
}

/// Settings for a minimum follow amount (either regular or smart copy)
/// 
/// Contains current/min/max amounts and currency symbol.
/// Used by lead trader settings use case.
#[derive(Debug, Clone, PartialEq)]
pub struct MinFollowAmountSettings {
    /// Currency symbol (e.g., "USDT")
    pub symbol: String,
    /// Current setting amount as string (e.g., "10")
    pub current: String,
    /// Minimum setting amount as string (e.g., "10")
    pub min: String,
    /// Maximum setting amount as string (e.g., "10000")
    pub max: String,
}

/// State for MyCopiesCurrentTraders Interactor
#[derive(Debug, Clone, PartialEq)]
pub struct MyCopiesCurrentTradersState {
    /// Content loading state
    pub content_state: ContentLoadState,
    /// Instrument type (Swap or Spot)
    pub inst_type: CopierPnlInstType,
    /// Timezone used for the request
    pub time_zone: String,
    /// List of currently followed traders
    pub items: Vec<CurrentTraderResponse>,
    /// Whether WebSocket is currently running
    pub is_ws_running: bool,
    /// Current rise/fall cycle for WebSocket
    pub rise_fall_cycle: CopierPnlRiseFallCycleType,
    /// Latest total PnL from WebSocket (updated on each WS snapshot)
    pub ws_total_pnl: Option<CopierPnlTotalPnl>,
    /// Timestamp of last WS snapshot (ms since epoch)
    pub ws_last_update_ms: i64,
}

/// State for My Copies History List Interactor
#[derive(Debug, Clone, PartialEq)]
pub struct MyCopiesHistoryListState {
    /// Content loading state
    pub content_state: ContentLoadState,
    /// Instrument type (Swap or Spot)
    pub inst_type: CopierPnlInstType,
    /// Timezone used for the request
    pub time_zone: String,
    /// Page size for pagination
    pub page_size: i32,
    /// Next page to load (1-based)
    pub next_page: i32,
    /// Whether there are more pages to load
    pub has_more: bool,
    /// Total count from API
    pub total: i32,
    /// List of historical traders
    pub items: Vec<HistoricalTraderDto>,
    /// Selected row index (0-based, -1 if none)
    pub selected_index: i32,
    /// Selected trader unique name
    pub selected_unique_name: String,
    /// Selected copy relation ID
    pub selected_copy_rel_id: String,
    /// Selected follow type
    pub selected_follow_type: String,
    /// Private info query unique name
    pub private_info_unique_name: String,
    /// Private info range data
    pub private_info_range: Option<PublicProfitSharingRatio>,
    /// Backend error message captured on any fetch failure (regardless of terminal content_state).
    /// 
    /// - **Cleared** on Loading entry (refresh / load_next_page start) and on SUCCESS.
    /// - **Written** in the ERROR branch with `err.to_string()` (raw backend message, via
    /// `OKNetworkError`'s transparent Display impl).
    /// - Mirrors native-parity rule: Holder maps this through `OKServerException` so the user
    /// sees the actual backend message, not a localized fallback.
    /// - Edge case: when paged load_next_page fails AND `items` is non-empty, `content_state`
    /// stays `Success` (to keep the existing list visible), but `error_message` is still
    /// populated for the UI event consumer. Holder reads `error_message` only in the ERROR
    /// state branch, so the stale value is invisible during a paged failure.
    pub error_message: String,
}

/// Individual follower item for UniFFI export
#[derive(Debug, Clone, PartialEq)]
pub struct MyFollowerItem {
    /// Avatar URL
    pub portrait: String,
    /// Nickname
    pub nick_name: String,
    /// Follow time (timestamp in milliseconds)
    pub follow_time: String,
    /// Profit/Loss
    pub pnl: String,
    /// Unique name/identifier
    pub unique_name: String,
    /// Avatar target ID
    pub target_id: String,
    /// Coin symbol for margin
    pub coin_symbol: String,
    /// Copy trading principal (AUM)
    pub aum: String,
    /// Days of following
    pub follow_days: String,
    /// User ID
    pub uid: String,
    /// Profit sharing ratio
    pub profit_sharing_ratio: String,
    /// Invested amount (USDT)
    pub invested_amount: String,
}

/// State for My Followers Interactor
#[derive(Debug, Clone, PartialEq)]
pub struct MyFollowersState {
    /// Content load state
    pub content_state: MyFollowersLoadState,
    /// List of followers
    pub followers: Vec<MyFollowerItem>,
    /// Summary information
    pub summary: MyFollowersSummary,
    /// Current sort type
    pub sort_type: FollowerSortType,
    /// Whether more data can be loaded
    pub can_load_more: bool,
    /// Whether the list is empty
    pub is_empty: bool,
    /// Whether header should be visible
    pub is_header_visible: bool,
    /// Current page number
    pub current_page: i32,
}

/// Summary information for followers
#[derive(Debug, Clone, PartialEq)]
pub struct MyFollowersSummary {
    /// Current number of followers
    pub follower_num: String,
    /// Maximum allowed followers
    pub follower_limit: String,
    /// Total historical followers
    pub total_follower_num: String,
    /// Total AUM (Assets Under Management)
    pub aum: String,
    /// AUM coin symbol
    pub aum_coin_symbol: String,
}

/// Bound voucher info for lead trader details.
#[derive(Debug, Clone, PartialEq)]
pub struct MyLeadTraderBoundVoucher {
    pub id: String,
    pub name: String,
    pub amount: f64,
    pub unit: String,
    pub biz_type: String,
    pub state: String,
    pub use_flag: String,
    pub reason: Option<String>,
    pub expire_time: String,
    pub curr_time: String,
    pub expire_time_type: String,
    pub relation_id: Option<String>,
    pub rule: String,
    pub description: String,
}

/// State for My Lead Trader Details Interactor.
#[derive(Debug, Clone, PartialEq)]
pub struct MyLeadTraderDetailsState {
    pub inst_type: CopierPnlInstType,
    pub copy_rel_id: String,
    pub trading_mode: CopierPnlTradingMode,
    pub rise_fall_cycle: CopierPnlRiseFallCycleType,
    pub time_zone: String,
    pub content_state: ContentLoadState,
    pub error_message: String,
    pub detail: Option<TraderDetailDto>,
    pub voucher: Option<MyLeadTraderBoundVoucher>,
    pub show_ongoing_tab: bool,
    pub copy_mode: MyLeadTraderCopyMode,
    pub follow_type: MyLeadTraderFollowType,
    pub is_ws_running: bool,
    pub latest_live_pnl: Option<MyLeadTraderLivePnlSlice>,
    pub is_prompt_showing: bool,
    pub pending_live_update: Option<MyLeadTraderLivePnlSlice>,
    pub history_list_content_state: ContentLoadState,
    pub history_list_page: i32,
    pub history_list: Option<PaginatedHistoricalTraderList>,
    pub history_list_error: String,
}

/// Live PnL slice used by header UI.
#[derive(Debug, Clone, PartialEq)]
pub struct MyLeadTraderLivePnlSlice {
    pub copy_state: String,
    pub u_time: String,
    pub pnl: String,
    pub today_pnl: String,
    pub net_pnl: String,
    pub realized_pnl: String,
    pub upl: String,
    pub investment_amount: String,
}

/// NMP Banner data.
/// 
/// Represents a single banner item in the marketplace.
#[derive(Debug, Clone, PartialEq)]
pub struct NmpBannerDto {
    /// Banner ID
    pub id: String,
    /// Banner title
    pub title: String,
    /// Banner subtitle
    pub subtitle: String,
    /// Call to action text
    pub cta: Option<String>,
    /// Target URL/href
    pub href: String,
    /// Activity name
    pub activity_name: String,
    /// Link type (internal/external)
    pub app_link_type: BannerLinkType,
    /// Light mode icon URL
    pub icon_light: String,
    /// Dark mode icon URL
    pub icon_dark: String,
    /// Banner category
    pub category: String,
    /// Labels
    pub labels: String,
    /// Weight for sorting
    pub weight: String,
    /// ML tags
    pub ml_tags: String,
    /// Light mode Lottie animation URL
    pub icon_lottie_light: String,
    /// Dark mode Lottie animation URL
    pub icon_lottie_dark: String,
    /// Layout type (normal/large)
    pub layout_type: BannerLayoutType,
    /// Banner type (normal/voucher)
    pub banner_type: MarketplaceBannerType,
    /// Voucher information (if voucher banner)
    pub voucher_info: Option<NmpBannerVoucherInfoDto>,
    /// User's voucher information
    pub user_voucher: Option<NmpBannerUserVoucherDto>,
    /// Whether to show APR (0: no, 1: yes)
    pub is_show_apr: String,
    /// APR value (for specific banner types)
    pub apr: String,
}

/// User's voucher information for a banner.
#[derive(Debug, Clone, PartialEq)]
pub struct NmpBannerUserVoucherDto {
    /// Batch number
    pub batch_no: String,
    /// Expiration time
    pub expire_time: String,
    /// Current server time
    pub current_time: String,
}

/// Banner voucher information.
#[derive(Debug, Clone, PartialEq)]
pub struct NmpBannerVoucherInfoDto {
    /// Batch number
    pub batch_no: String,
    /// Remaining count
    pub remain_count: String,
    /// Voucher type
    pub voucher_type: String,
    /// APR voucher details
    pub apr_voucher: Option<AprVoucherDto>,
    /// Voucher PNL ratio
    pub voucher_pnl_ratio: String,
    /// Loss voucher details
    pub loss_voucher: Option<LossVoucherDto>,
    /// Voucher amount
    pub amount: String,
    /// Currency unit
    pub unit: String,
}

/// A copier notification message record
/// 
/// This is the public/UniFFI representation of a message that
/// was shown to a copier as a pop-up notification.
#[derive(Debug, Clone, PartialEq)]
pub struct OKSTCopierMessageRecord {
    /// Unique identifier for the message
    pub id: String,
    /// Message title
    pub title: String,
    /// Message content body
    pub content: String,
    /// Message level (info, warn, important, others)
    pub level: OKSTCopierMessageLevel,
    /// Optional URL link for the message
    pub link: Option<String>,
    /// Whether to display transfer button
    pub display_transfer: bool,
}

/// Instrument info domain model (UniFFI-compatible)
/// Contains information about an instrument available for copy trading
#[derive(Debug, Clone, PartialEq)]
pub struct OngoingLeadTradesInstrumentInfo {
    /// Instrument ID (e.g., "BTC-USDT-SWAP" for contract, "BTC-USDT" for spot)
    pub inst_id: String,
    /// Instrument name/symbol (e.g., "BTC")
    pub name: String,
}

/// Position summary domain model (UniFFI-compatible)
/// 
/// Contains aggregated information about a lead trader's position.
#[derive(Debug, Clone, PartialEq)]
pub struct OngoingLeadTradesPositionSummary {
    /// Trade item ID - unique identifier for this position group
    pub trade_item_id: String,
    /// Detail IDs - comma-separated IDs of individual positions
    pub detail_ids: String,
    /// Instrument ID (e.g., "BTC-USDT-SWAP", "DOGE-USDT-SWAP")
    pub inst_id: String,
    /// Instrument type (e.g., "SWAP", "SPOT")
    pub inst_type: String,
    /// Currency (e.g., "USDT")
    pub ccy: String,
    /// Margin mode - "isolated" or "cross"
    pub mgn_mode: String,
    /// Position side - "long" or "short"
    pub pos_side: String,
    /// Trade side - "buy" or "sell"
    pub side: String,
    /// Leverage multiplier (as string)
    pub lever: String,
    /// Sub-position amount
    pub sub_pos: String,
    /// Available sub-position amount
    pub avail_sub_pos: String,
    /// Maximum sellable amount
    pub max_sellable_amount: String,
    /// Deal volume
    pub deal_volume: String,
    /// Margin amount
    pub margin: String,
    /// Initial margin requirement
    pub imr: String,
    /// Notional value in USD
    pub notional_usd: String,
    /// Last price
    pub last: String,
    /// Mark price
    pub mark_px: String,
    /// Open average price
    pub open_avg_px: String,
    /// Unrealized PnL
    pub pnl: String,
    /// PnL ratio (as decimal string)
    pub pnl_ratio: String,
    /// Closed PnL
    pub close_pnl: String,
    /// Accumulated funding fee
    pub funding_fee: String,
    /// Accumulated trading fee
    pub fee: String,
    /// Liquidation fee
    pub liquidation_fee: String,
    /// Position type
    pub pos_type: String,
    /// Open time (unix timestamp in milliseconds as string)
    pub open_time: String,
    /// Update time (unix timestamp in milliseconds as string)
    pub u_time: String,
    /// Trader unique name
    pub unique_name: String,
}

/// Inputs to the reminder decision tree.
/// 
/// All fields must be resolved by the bridge BEFORE calling
/// `compute_ongoing_lead_trades_reminder`. In particular, `m_signal_protect`
/// already mirrors native's `mSignalProtect` derivation
/// (OrderListViewModel.kt:447-455 — owner+userData → userData.{swap|spot}.isSignalProtect;
/// otherwise → savedStateHandle's isSignalProtect).
#[derive(Debug, Clone, PartialEq)]
pub struct OngoingLeadTradesReminderInputs {
    /// Is the viewer the lead trader (`isOwner` from Fragment args)?
    pub is_owner: bool,
    /// Is the current tab Swap? `false` = Spot or other.
    pub is_swap: bool,
    /// Triggered from a trade-page widget — suppresses every reminder.
    pub is_from_trade_page: bool,
    /// Is the viewer copying someone (`isCopyFollowing`, default false).
    pub is_copy_following: bool,
    /// `isPrivateLeadTrader` from Fragment args (default false).
    /// Used in the copier branch — different from `user_swap_private_lead_trader`.
    pub is_private_lead_trader: bool,
    /// `isSignalProtect` from Fragment args (default false).
    /// Fallback when `is_owner` is false; passed in for completeness.
    pub is_signal_protect: bool,
    /// Resolved `mSignalProtect` per native lines 447-455.
    /// - When `is_owner && userData != null` → `userData.{swap|spot}.isSignalProtect`
    /// - Otherwise → equals `is_signal_protect`
    pub m_signal_protect: bool,
    /// For owner-Swap branch: `userData?.swap?.isPrivateLeadTrader == true`.
    /// `false` if `userData == null` or current tab is not Swap.
    pub user_swap_private_lead_trader: bool,
}

/// State for Ongoing Lead Trades Interactor
#[derive(Debug, Clone, PartialEq)]
pub struct OngoingLeadTradesState {
    /// View mode (Self_ or Public)
    pub view_mode: OngoingLeadTradesViewMode,
    /// Summary load state
    pub summary_state: OngoingLeadTradesLoadState,
    /// Detail load state
    pub detail_state: OngoingLeadTradesLoadState,
    /// Instruments load state
    pub instruments_state: OngoingLeadTradesLoadState,
    /// Position summaries
    pub summaries: Vec<PositionSummaryItem>,
    /// Position details
    pub details: Vec<PositionDetailItem>,
    /// Available instruments
    pub instruments: Vec<InstrumentItem>,
    /// Whether summaries list is empty
    pub is_summaries_empty: bool,
    /// Whether details list is empty
    pub is_details_empty: bool,
    /// Last error message — set on load failure, cleared on success.
    /// Bridge MUST surface this via `OKServerException(message = rs.errorMessage)`
    /// to preserve native error parity (see skill Error Display Native Parity Rule).
    pub error_message: String,
    /// Active tab: detail vs summary (`viewingDetail` in OrderListManager.kt:32)
    pub viewing_detail: bool,
    /// Current sort mode (`currentSortMode`, OrderListViewModel.kt:156)
    pub sort_mode: OngoingLeadTradesSortMode,
    /// Instrument id allow-list filter (`mInstIds`, OrderListViewModel.kt:136)
    pub inst_id_filter: Vec<String>,
    /// "Show only current trading pair" toggle (`showCurrentPairOnly`, OrderListViewModel.kt:153)
    pub show_current_pair_only: bool,
    /// Currently-selected trading pair instId (set by bridge via `set_show_current_pair_only`,
    /// derived from `SelectBizManager.getCurrentBizInstrument()` in native)
    pub current_pair_inst_id: String,
    /// Currently-selected trading pair instType
    pub current_pair_inst_type: String,
    /// PnL bottom sheet open id (`currentPnlId`, OrderListViewModel.kt:169).
    /// Used by WS update path to know which order's PnL sheet is open.
    pub current_pnl_id: Option<String>,
    /// Whether the WS subscription is active (set by `start_ws` / `stop_ws`).
    /// Mirrors native `OrderListViewModel.wsJobs != null` (line 161).
    pub is_ws_running: bool,
    /// Summaries after filter + sort, ready for UI consumption.
    pub displayed_summaries: Vec<PositionSummaryItem>,
    /// Details after filter + sort, ready for UI consumption.
    pub displayed_details: Vec<PositionDetailItem>,
}

/// Ongoing relation info for current traders
#[derive(Debug, Clone, PartialEq)]
pub struct OngoingRelationInfoResponse {
    /// Follow start timestamp
    pub follow_time: i64,
    /// Copy relation ID. Renamed from `relation_id` (rename "copyRelId") for 3-way alignment
    /// with native @SerialName("copyRelId"). See my_copies.rs `FullRelationInfo.copy_rel_id`.
    pub copy_rel_id: String,
    /// Copy mode: Fixed or Position
    pub copy_mode: String,
    /// Coin symbol (currency)
    pub coin_symbol: String,
    /// Follow type: 0=NotFollowing, 1=Following, 2=ClosingPosition, 3=Unfollowing, 4=SettingUp
    pub follow_type: String,
    /// Closing mode: 1=MarketClose, 2=CloseByLeadTrader, 3=CloseByMyself.
    /// Renamed from `closing_mode` (rename "remainMgnDeal") for 3-way alignment.
    pub remain_mgn_deal: String,
}

/// Request to create an algo order for take profit/stop loss
/// 
/// Used to set up automatic take profit and stop loss orders
/// for spot copy trading positions.
/// 
/// # Fields
/// - `trade_item_id`: The trade item ID
/// - `tp_trigger_px`: Take profit trigger price
/// - `sl_trigger_px`: Stop loss trigger price
/// - `tp_trigger_type`: TP trigger type (last/index/mark)
/// - `sl_trigger_type`: SL trigger type (last/index/mark)
/// - `tp_ord_px`: Take profit order price
/// - `sl_ord_px`: Stop loss order price
/// - `inst_type`: Instrument type (SWAP/SPOT)
#[derive(Debug, Clone, PartialEq)]
pub struct OrderOperationsAlgoOrderRequest {
    /// Trade item ID
    pub trade_item_id: String,
    /// Take profit trigger price
    pub tp_trigger_px: String,
    /// Stop loss trigger price
    pub sl_trigger_px: String,
    /// Take profit trigger type: "last", "index", or "mark"
    pub tp_trigger_type: Option<String>,
    /// Stop loss trigger type: "last", "index", or "mark"
    pub sl_trigger_type: Option<String>,
    /// Take profit order price
    pub tp_ord_px: String,
    /// Stop loss order price
    pub sl_ord_px: String,
    /// Instrument type: "SWAP" or "SPOT"
    pub inst_type: String,
}

/// Request to amend/edit an existing order
/// 
/// Used to modify the price of an existing copy trade order.
/// 
/// # Fields
/// - `ord_id`: The order ID to amend
/// - `trade_item_id`: The trade item ID
/// - `px`: New price (optional, pass None to keep current)
#[derive(Debug, Clone, PartialEq)]
pub struct OrderOperationsAmendOrderRequest {
    /// Order ID to amend
    pub ord_id: String,
    /// Trade item ID
    pub trade_item_id: String,
    /// New price (optional)
    pub px: Option<String>,
}

/// Request to cancel an existing order
/// 
/// Used to cancel a pending order in spot copy trading.
/// 
/// # Fields
/// - `ord_id`: The order ID to cancel
/// - `inst_id`: The instrument ID (e.g., "BTC-USDT")
#[derive(Debug, Clone, PartialEq)]
pub struct OrderOperationsCancelOrderRequest {
    /// Order ID to cancel
    pub ord_id: String,
    /// Instrument ID (e.g., "BTC-USDT")
    pub inst_id: String,
}

/// Result from querying order details
/// 
/// Contains information about a pending/executed order.
/// 
/// # Fields
/// - `px`: Order price
/// - `amount`: Order size/amount
/// - `pnl`: Profit and loss
/// - `ord_id`: Order ID
#[derive(Debug, Clone, PartialEq)]
pub struct OrderOperationsQueryOrderResult {
    /// Order price
    pub px: String,
    /// Order size/amount
    pub amount: String,
    /// Profit and loss
    pub pnl: String,
    /// Order ID
    pub ord_id: String,
}

/// Paginated list response wrapper
/// Used for APIs that return paginated data like history-list
#[derive(Debug, Clone, PartialEq)]
pub struct PaginatedHistoricalTraderList {
    /// Total count of items
    pub total: i32,
    /// List of historical traders
    pub list: Vec<HistoricalTraderDto>,
}

/// Partial trader info (used in historical traders)
#[derive(Debug, Clone, PartialEq)]
pub struct PartialTraderInfo {
    /// Trader portrait URL
    pub portrait: String,
    /// Trader nickname
    pub nick_name: String,
    /// Lead mode raw API value: "0"/"1" = public, "2" = private.
    /// Stored as String (not bool) to keep Rust DTO 3-way aligned with native @SerialName("leadMode")
    /// and avoid UniFFI Kotlin field name divergence through any Gson-based bridge. Consumers
    /// parse `lead_mode == "2"` inline (see native LeadModeSerializer.kt).
    pub lead_mode: String,
    /// Unique name identifier
    pub unique_name: String,
    /// Target ID
    pub target_id: String,
    /// Whether this trader is a sub-account (BE sends "0"/"1" string).
    /// Surfaced on the `relation/copier/history-list` endpoint as of Phase 2.
    /// Forward-compatible: defaults to `false` if BE omits the field.
    pub is_sub_account: bool,
}

/// Partial trader info
#[derive(Debug, Clone, PartialEq)]
pub struct PartialTraderInfoResponse {
    /// Trader portrait URL
    pub portrait: String,
    /// Trader nickname
    pub nick_name: String,
    /// Lead mode raw API value: "0"/"1" = public, "2" = private.
    /// Stored as String (not bool) to keep Rust DTO 3-way aligned with native @SerialName("leadMode")
    /// — the previous `is_private: bool` with `deserialize_with` BUT no `rename = "leadMode"`
    /// silently defaulted to false because rename_all=camelCase produced JSON key "isPrivate"
    /// which the server never sends (pre-existing bug from incident 2026-05-18). Consumers
    /// parse `lead_mode == "2"` inline.
    pub lead_mode: String,
    /// Unique name identifier
    pub unique_name: String,
    /// Target ID
    pub target_id: String,
    /// Whether this trader is a sub-account (BE sends "0"/"1" string).
    /// Surfaced on the `relation/copier/list` endpoint as of Phase 2 of the
    /// sub-account copy-trading initiative — native clients gate the
    /// "Personal page" entry and copy-detail avatar routing on this flag.
    /// Forward-compatible: defaults to `false` if BE omits the field.
    pub is_sub_account: bool,
}

/// Aggregate state for the Partner Email Code Interactor
#[derive(Debug, Clone, PartialEq)]
pub struct PartnerEmailCodeState {
    /// Content load state
    pub content_state: PartnerEmailCodeLoadState,
    /// Effect ID (for tracking effect acknowledgment)
    pub effect_id: i64,
    /// Effect type
    pub effect_type: PartnerEmailCodeEffectType,
    /// Effect message (error message, etc.)
    pub effect_msg: String,
}

/// Position data
/// Corresponds to Position in Kotlin
/// Note: NumericString is represented as String in Rust
/// BigDecimal fields use BigDecimalAsStringSerializer in Kotlin, represented as String in Rust
#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    /// Average price
    pub avg_px: String,
    /// Create time
    pub c_time: String,
    /// Instrument ID
    pub inst_id: String,
    /// Instrument type
    pub inst_type: String,
    /// Leverage
    pub lever: String,
    /// Alias
    pub alias: String,
    /// Position side
    pub pos_side: String,
    /// Position (NumericString in Kotlin)
    pub pos: String,
    /// Position currency
    pub pos_ccy: String,
    /// Liquidation price
    pub liq_px: String,
    /// Margin mode
    pub mgn_mode: String,
    /// Unrealized PNL ratio (NumericString in Kotlin)
    pub upl_ratio: String,
    /// Position space (NumericString in Kotlin)
    pub pos_space: String,
    /// Position ID
    pub pos_id: String,
    /// Margin (NumericString in Kotlin)
    pub margin: String,
    /// Margin ratio (BigDecimal in Kotlin, deserialized from string via BigDecimalAsStringSerializer)
    pub mgn_ratio: String,
    /// Mark price (NumericString in Kotlin)
    pub mark_px: String,
    /// Unrealized PNL (BigDecimal in Kotlin, deserialized from string via BigDecimalAsStringSerializer)
    pub upl: String,
    /// Quote currency
    pub quote_ccy: String,
    /// Margin currency
    pub margin_ccy: String,
    /// Realized PNL (BigDecimal in Kotlin, deserialized from string via BigDecimalAsStringSerializer)
    pub realized_pnl: String,
    /// PNL (BigDecimal in Kotlin, deserialized from string via BigDecimalAsStringSerializer)
    pub pnl: String,
    /// Funding fee (BigDecimal in Kotlin, deserialized from string via BigDecimalAsStringSerializer)
    pub funding_fee: String,
    /// Fee (BigDecimal in Kotlin, deserialized from string via BigDecimalAsStringSerializer)
    pub fee: String,
    /// Break-even price (NumericString in Kotlin)
    pub be_px: String,
    /// Last price (NumericString in Kotlin)
    pub last: String,
    /// Notional USD (NumericString in Kotlin)
    pub notional_usd: String,
}

/// Position detail item for UniFFI export
#[derive(Debug, Clone, PartialEq)]
pub struct PositionDetailItem {
    /// Trade item ID - unique identifier for this position
    pub trade_item_id: String,
    /// Instrument ID (e.g., "BTC-USDT-SWAP", "DOGE-USDT-SWAP")
    pub inst_id: String,
    /// Instrument type (e.g., "SWAP", "SPOT")
    pub inst_type: String,
    /// Currency (e.g., "USDT")
    pub ccy: String,
    /// Margin mode - "isolated" or "cross"
    pub mgn_mode: String,
    /// Position side - "long" or "short"
    pub pos_side: String,
    /// Trade side - "buy" or "sell"
    pub side: String,
    /// Leverage multiplier (as string)
    pub lever: String,
    /// Sub-position amount
    pub sub_pos: String,
    /// Available sub-position amount
    pub avail_sub_pos: String,
    /// Maximum sellable amount
    pub max_sellable_amount: String,
    /// Deal volume
    pub deal_volume: String,
    /// Margin amount
    pub margin: String,
    /// Initial margin requirement
    pub imr: String,
    /// Notional value in USD
    pub notional_usd: String,
    /// Last price
    pub last: String,
    /// Mark price
    pub mark_px: String,
    /// Open average price
    pub open_avg_px: String,
    /// Unrealized PnL
    pub pnl: String,
    /// PnL ratio (as decimal string)
    pub pnl_ratio: String,
    /// Closed PnL
    pub close_pnl: String,
    /// Accumulated funding fee
    pub funding_fee: String,
    /// Accumulated trading fee
    pub fee: String,
    /// Liquidation fee
    pub liquidation_fee: String,
    /// Position type
    pub pos_type: String,
    /// Open time (unix timestamp in milliseconds as string)
    pub open_time: String,
    /// Update time (unix timestamp in milliseconds as string)
    pub u_time: String,
    /// Trader unique name
    pub unique_name: String,
    /// Take-profit trigger price
    pub tp_trigger_px: String,
    /// Take-profit order price
    pub tp_ord_px: String,
    /// Take-profit trigger type (e.g., "last", "mark")
    pub tp_trigger_type: String,
    /// Stop-loss trigger price
    pub sl_trigger_px: String,
    /// Stop-loss order price
    pub sl_ord_px: String,
    /// Stop-loss trigger type (e.g., "last", "mark")
    pub sl_trigger_type: String,
    /// `pnl` after applying `get_profit_dig_with_index(inst_family)` precision + Down rounding.
    /// Empty ONLY when a [`crate::format::round_pnl`] guard fails (genuine failure — there is
    /// no config gate). Kotlin Adapter keeps the raw `pnl` in that case. F1.1 (2026-06-08).
    pub pnl_rounded: String,
    /// `pnl_ratio` after precision-4 Down rounding (truncate). Same guard semantics. F1.1.
    pub pnl_ratio_rounded: String,
}

/// `Default` is load-bearing: the iOS-parity `PositionScatterEmptyPolicy::EmptySuccess`
/// branch of the request center returns `Ok(Self::default())` for an empty API array so
/// the native UI renders its own empty state (see
/// `position_history_scatter_request_center.rs`).
#[derive(Debug, Clone, PartialEq)]
pub struct PositionHistoryScatter {
    pub symbol: String,
    pub item_scatter_list: Vec<PositionHistoryScatterItem>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PositionHistoryScatterItem {
    pub hold_time_ms: String,
    pub close_pnl: String,
}

/// Position summary item for UniFFI export
#[derive(Debug, Clone, PartialEq)]
pub struct PositionSummaryItem {
    /// Trade item ID - unique identifier for this position group
    pub trade_item_id: String,
    /// Detail IDs - comma-separated IDs of individual positions
    pub detail_ids: String,
    /// Instrument ID (e.g., "BTC-USDT-SWAP", "DOGE-USDT-SWAP")
    pub inst_id: String,
    /// Instrument type (e.g., "SWAP", "SPOT")
    pub inst_type: String,
    /// Currency (e.g., "USDT")
    pub ccy: String,
    /// Margin mode - "isolated" or "cross"
    pub mgn_mode: String,
    /// Position side - "long" or "short"
    pub pos_side: String,
    /// Trade side - "buy" or "sell"
    pub side: String,
    /// Leverage multiplier (as string)
    pub lever: String,
    /// Sub-position amount
    pub sub_pos: String,
    /// Available sub-position amount
    pub avail_sub_pos: String,
    /// Maximum sellable amount
    pub max_sellable_amount: String,
    /// Deal volume
    pub deal_volume: String,
    /// Margin amount
    pub margin: String,
    /// Initial margin requirement
    pub imr: String,
    /// Notional value in USD
    pub notional_usd: String,
    /// Last price
    pub last: String,
    /// Mark price
    pub mark_px: String,
    /// Open average price
    pub open_avg_px: String,
    /// Unrealized PnL
    pub pnl: String,
    /// PnL ratio (as decimal string)
    pub pnl_ratio: String,
    /// Closed PnL
    pub close_pnl: String,
    /// Accumulated funding fee
    pub funding_fee: String,
    /// Accumulated trading fee
    pub fee: String,
    /// Liquidation fee
    pub liquidation_fee: String,
    /// Position type
    pub pos_type: String,
    /// Open time (unix timestamp in milliseconds as string)
    pub open_time: String,
    /// Update time (unix timestamp in milliseconds as string)
    pub u_time: String,
    /// Trader unique name
    pub unique_name: String,
    /// `pnl` after applying `get_profit_dig_with_index(inst_family)` precision + Down rounding.
    /// Empty ONLY when a guard in [`crate::format::round_pnl`] fails (genuine failure —
    /// there is no config gate). Kotlin Adapter keeps the raw `pnl` in that case; the shared
    /// render pipeline rounds it idempotently. F1.1 (2026-06-08).
    pub pnl_rounded: String,
    /// `pnl_ratio` after precision-4 Down rounding (truncate). Same guard semantics as
    /// [`Self::pnl_rounded`]. F1.1.
    pub pnl_ratio_rounded: String,
}

/// Input for position value check.
#[derive(Debug, Clone, PartialEq)]
pub struct PositionValueCheckInput {
    /// Maximum follow amount
    pub max_follow_amount: String,
    /// Copy relation ID
    pub copy_rel_id: String,
}

/// Request parameters for position value check.
/// 
/// # API
/// GET /priapi/v5/ecotrade/copytrading/relation/copier/position-val-check
#[derive(Debug, Clone, PartialEq)]
pub struct PositionValueCheckRequest {
    /// Instrument type (SWAP/SPOT)
    pub inst_type: String,
    /// Maximum follow amount
    pub max_follow_amount: String,
    /// Copy relation ID
    pub copy_rel_id: String,
}

/// Pre-profit (estimated pending profit) information
/// 
/// Contains estimated profit to be settled and list of followers with their contributions.
/// Used by GET /priapi/v5/ecotrade/account/pre-profit (Contract)
/// Used by GET /priapi/v5/ecotrade/spot/account/pre-profit (Spot)
#[derive(Debug, Clone, PartialEq)]
pub struct PreProfit {
    /// Estimated pending profit amount
    pub pre_profit: String,
    /// List of followers contributing to this profit
    pub followers: Vec<PreProfitFollower>,
    /// Remaining time until settlement (in seconds or formatted string)
    pub remain_time: String,
    /// Currency symbol (e.g., "USDT")
    pub symbol: String,
}

/// Follower information in pre-profit context
/// 
/// Represents a single follower's contribution to the estimated profit.
#[derive(Debug, Clone, PartialEq)]
pub struct PreProfitFollower {
    /// Follower's display name
    pub nick_name: String,
    /// Follower's avatar URL
    pub portrait: String,
    /// Estimated profit from this follower
    pub profit: String,
    /// Target ID used for default avatar color selection
    pub target_id: String,
    /// Unique identifier for this record (used for pagination)
    pub id: String,
    /// Currency symbol (e.g., "USDT")
    pub symbol: String,
}

/// Currency preference chart data item
/// Corresponds to OKGTEBalanceChartModel in iOS
#[derive(Debug, Clone, PartialEq)]
pub struct PreferenceCoinItem {
    /// Currency code (e.g., "BTC", "ETH")
    pub currency: String,
    /// Percentage as string (e.g., "1.0", "0.5")
    pub percent: String,
}

/// Pre-provided trader basic info (RAW fields, mirrors native `basicUserInfo`
/// supplied by the entry). When present, the gatekeeper SKIPS the basic-info
/// fetch — exactly mirroring native `basicUserInfo ?: userRepo.basicInfo(...)`
/// (VM:180). `nick_name` is RAW (not mapNickName'd); the Holder applies
/// mapNickName uniformly at Params-build time (native VM:313).
#[derive(Debug, Clone, PartialEq)]
pub struct PrefetchedBasicInfo {
    /// Raw nick (pre-mapNickName).
    pub nick_name: String,
    /// English nick (mapNickName fallback when nick_name empty).
    pub en_nick_name: String,
    /// Avatar URL (= native `portrait`).
    pub avatar_url: String,
    /// Target id.
    pub target_id: String,
    /// Bio (= native `sign`; becomes EcoBioContent.bio).
    pub bio_content: String,
    /// Translated bio (= native `translatedBio`; becomes EcoBioContent.translatedBio).
    pub translated_bio: String,
}

/// Premium benefits information
#[derive(Debug, Clone, PartialEq)]
pub struct PremiumBenefits {
    /// Benefit key identifier
    pub benefit_key: String,
    /// Benefit name
    pub name: String,
    /// Benefit description
    pub desc: String,
    /// Benefit icon information
    pub icon: Option<BenefitsIcon>,
    /// Whether benefit is granted (server returns as string, convert to int)
    pub granted: i32,
}

/// Price limit check response DTO — the shared, UniFFI-exported shape.
/// 
/// Used to check if a sell price is within acceptable limits.
/// 
/// Field presence is deliberately **lenient** here. This record is what
/// `OKSTCorePriceLimitCheckUseCase` returns, and that surface takes `inst_type`
/// from the caller (`SPOT`/`SWAP`/`FUTURES`/`MARGIN`/`SPOT,SWAP`) for both
/// clients, with no `social_trading_spot_trade_safety_checks_rust_migration`
/// flag to fall back to. A non-spot instrument may legitimately omit `sellLmt`,
/// so requiring the keys on this record would turn a working call into a hard
/// failure with no way to switch it off.
/// 
/// The flagged spot sell gate does **not** use this type — see
/// [`SpotPriceLimitCheck`], which requires both keys so that a malformed row
/// fails the gate closed, matching the native arm. Keeping the two apart holds
/// the strictness inside the feature flag and inside the migration's parity
/// evidence.
#[derive(Debug, Clone, PartialEq)]
pub struct PriceLimitCheckDto {
    /// Sell limit price (minimum acceptable sell price)
    pub sell_limit: Option<String>,
    /// Whether the price triggers the limit check
    /// JSON format: "1" or "0" string
    pub trigger_limit: bool,
}

/// Profile chart data
/// Corresponds to ProfileChartDto in Kotlin
#[derive(Debug, Clone, PartialEq)]
pub struct ProfileChart {
    /// Chart type (metricType in JSON)
    pub chart_type: ProfileChartType,
    /// Data status (deserialized from string via BooleanAsStringSerializer)
    pub data_status: bool,
    /// Chart data points (rates in JSON)
    pub rates: Vec<ChartData>,
}

/// Profile position history data
/// Corresponds to ProfilePositionHistoryDto in Kotlin
/// Note: NumericString, InstId, Ccy are value classes in Kotlin, represented as String in Rust
#[derive(Debug, Clone, PartialEq)]
pub struct ProfilePositionHistory {
    /// Alias
    pub alias: String,
    /// Create time (deserialized from string via LongAsStringSerializer)
    /// Note: public endpoint uses "cTime", own-history endpoint uses "openTime"
    pub c_time: i64,
    /// Close average price (NumericString in Kotlin)
    pub close_avg_px: String,
    /// Open average price (NumericString in Kotlin)
    pub open_avg_px: String,
    /// Contract value (NumericString in Kotlin)
    /// JSON field: "contractVal" (from trader/position-history API)
    pub contract_val: String,
    /// Instrument family
    pub inst_family: String,
    /// Instrument ID (InstId value class in Kotlin)
    pub inst_id: String,
    /// Instrument type
    pub inst_type: String,
    /// Leverage (NumericString in Kotlin)
    pub lever: String,
    /// Margin mode
    pub mgn_mode: String,
    /// Close amount (NumericString in Kotlin)
    pub close_amount: String,
    /// Sub position (closed quantity for calculating closed USDT)
    /// JSON field: "subPos"
    pub sub_pos: String,
    /// Open max amount (NumericString in Kotlin)
    pub open_max_amount: String,
    /// Position ID
    /// JSON field: "posId" (copy endpoints) or "id" (trader/position-history endpoint)
    pub id: String,
    /// Position side
    pub pos_side: String,
    /// Update time (deserialized from string via LongAsStringSerializer)
    pub u_time: i64,
    /// Underlying
    pub uly: String,
    /// Realized PNL (NumericString in Kotlin)
    pub realized_pnl: String,
    /// Close PNL (NumericString in Kotlin)
    /// JSON field: "pnl"
    pub close_pnl: String,
    /// Fee (NumericString in Kotlin)
    pub fee: String,
    /// Funding fee (NumericString in Kotlin)
    pub funding_fee: String,
    /// PNL ratio — raw server string, no parsing.
    /// Native: TraderPositionHistoryItem.pnlRatio is plain String (PositionHistoryResponse.kt:210),
    /// and downstream UI uses formatPercentTruncateWithSymbol / toSafeFloat at the call site,
    /// so we keep the original precision instead of going through f32.
    pub pnl_ratio: String,
    /// Quote currency (Ccy value class in Kotlin)
    /// JSON field: "quoteCcy" (copy endpoints) or "ccy" (trader/position-history endpoint)
    pub ccy: String,
    /// Margin currency (Ccy value class in Kotlin)
    pub margin_ccy: String,
    /// Close position type (deserialized via ClosePositionTypeSerializerV2)
    /// JSON field: "closeType" (public endpoint) or "type" (own-history endpoint)
    pub close_position_type: Option<ClosePositionType>,
    /// Liquidation status
    pub liquidation_status: String,
    /// Liquidation fee (NumericString in Kotlin)
    /// JSON field: "liquidationFee"
    pub liquidation_fee: String,
    /// Position type (HistoryPosType enum in Kotlin)
    /// JSON field: "posType"
    pub pos_type: Option<HistoryPosType>,
    /// Position closing PnL (distinct from "pnl" which is total realized PnL)
    /// JSON field: "closePnl"
    /// In native Kotlin: TraderPositionHistoryItem.closePnl
    pub position_close_pnl: String,
    /// Unique name of the trader
    /// JSON field: "uniqueName"
    pub unique_name: String,
    /// Deal volume (Spot only — used to compute turnover display)
    /// JSON field: "dealVolume"
    /// Native: TraderPositionHistoryItem.dealVolume (NumericString)
    pub deal_volume: String,
    /// Aggregated position unique id (Spot UI uses this for share)
    /// JSON field: "tradeItemId"
    /// Native: TraderPositionHistoryItem.tradeItemId
    pub trade_item_id: String,
}

/// Profit curve data point
/// 
/// Represents a single data point in the profit sharing curve chart.
/// Used by GET /priapi/v5/ecotrade/account/profit-curve
#[derive(Debug, Clone, PartialEq)]
pub struct ProfitCurve {
    /// Timestamp for this data point
    pub stat_time: String,
    /// Profit share value at this time
    pub profit_share: String,
}

/// Detailed profit breakdown for a specific settlement period
/// 
/// Shows individual follower contributions to a profit settlement.
/// Used by GET /priapi/v5/ecotrade/account/profit-detail (Contract)
/// Used by GET /priapi/v5/ecotrade/spot/account/profit-detail (Spot)
#[derive(Debug, Clone, PartialEq)]
pub struct ProfitDetail {
    /// Follower's display name
    pub nick_name: String,
    /// Profit amount from this follower
    pub profit: String,
    /// Follower's avatar URL
    pub portrait: String,
    /// Target ID used for default avatar color selection
    pub target_id: String,
    /// Unique identifier for this record (used for pagination)
    pub id: String,
    /// Currency symbol (e.g., "USDT")
    pub symbol: String,
}

/// Profit information for a trader
/// 
/// Contains profit sharing ratio, latest/total profit, and follower statistics.
/// Used by GET /priapi/v5/ecotrade/account/profit-info (Contract)
/// Used by GET /priapi/v5/ecotrade/spot/account/profit-info (Spot)
#[derive(Debug, Clone, PartialEq)]
pub struct ProfitInfo {
    /// Profit sharing ratio (e.g., "0.10" means 10%)
    pub share_ratio: String,
    /// Latest profit amount
    pub latest_profit: String,
    /// Total accumulated profit
    pub total_profit: String,
    /// Total number of followers accumulated
    pub total_follower_num: String,
    /// Currency symbol (e.g., "USDT")
    pub symbol: String,
}

/// Profit Settlement state.
/// 
/// Consolidates data from three Android ViewModels into a single observable state.
/// The `#[interactor::state]` macro adds: `Clone, Debug, PartialEq, uniffi::Record`
/// 
/// # Corresponding Android UI
/// 
/// | Field group | ViewModel | Screen |
/// |---|---|---|
/// | `share_ratio`, `latest_profit`, `total_profit` | `DividendViewModel` | Dividend overview |
/// | `pre_profit_items`, `pre_profit_has_more` | `DividendViewModel` | Expected dividend list |
/// | `history_profit_items`, `history_has_more` | `DividendViewModel` | Dividend history list |
/// | `aum_level`, `aum_profit_ratio`, `last_90d_avg` | `MyLeadingHomeViewModel` | AUM dashboard |
/// | `ratio_min/max/current`, `ratio_can_confirm` | `ProfitDividendSettingViewModel` | Ratio editor |
#[derive(Debug, Clone, PartialEq)]
pub struct ProfitSettlementState {
    pub content_state: ContentLoadState,
    pub trade_type: ProfitSettlementTradeType,
    /// Profit sharing ratio string (e.g., "10" means 10%)
    pub share_ratio: String,
    /// Latest settlement profit amount
    pub latest_profit: String,
    /// Total accumulated profit
    pub total_profit: String,
    /// Total follower count
    pub total_follower_num: String,
    /// Currency symbol (e.g., "USDT")
    pub profit_symbol: String,
    pub pre_profit_items: Vec<PreProfit>,
    /// True if more pages available (size >= PAGE_SIZE)
    pub pre_profit_has_more: bool,
    /// True while loading first/more pre-profit pages
    pub pre_profit_loading: bool,
    pub history_profit_items: Vec<HistoryProfit>,
    /// True if more pages available
    pub history_has_more: bool,
    /// True while loading first/more history pages
    pub history_loading: bool,
    /// Per-settlement breakdown rows shown in the dividend details bottom sheet.
    /// Mirrors `FetchProfitDetailListUseCase.execute()` raw API response (no formatting).
    pub profit_detail_items: Vec<ProfitDetail>,
    /// True if more pages available (size >= PAGE_SIZE && cursor non-empty),
    /// matches native `PaginationUseCase.canLoadMore`.
    pub profit_detail_has_more: bool,
    /// True while loading first/more profit-detail pages.
    pub profit_detail_loading: bool,
    /// AUM tier level (e.g., "2" → displayed as "L2")
    pub aum_level: String,
    /// Profit sharing ratio from AUM info (e.g., "12" means 12%)
    pub aum_profit_ratio: String,
    /// 90-day average AUM amount
    pub last_90d_avg: String,
    /// Whether trader is in private lead mode (affects ratio edit flow)
    pub is_private_leader: bool,
    /// Number of times the profit sharing ratio can still be changed (from `ProfitSharingRatio.limit_num`)
    pub ratio_change_limit: i32,
    /// Minimum allowed profit sharing ratio (as decimal, e.g., 0.05 = 5%)
    pub ratio_min: f64,
    /// Maximum allowed profit sharing ratio (as decimal)
    pub ratio_max: f64,
    /// Currently selected ratio (as decimal)
    pub ratio_current: f64,
    /// Original ratio before user editing (as decimal, from server).
    pub ratio_original: f64,
    /// Whether confirm button is enabled (value changed from original AND within max)
    pub ratio_can_confirm: bool,
    /// Whether to show warning (value exceeds max)
    pub ratio_show_warning: bool,
}

/// Profit sharing information for a trader
/// 
/// Contains AUM level, profit sharing ratio, and trader profile information.
/// Used by GET /priapi/v5/ecotrade/trader/profit-sharing-info (Contract)
/// Used by GET /priapi/v5/ecotrade/spot/trader/profit-sharing-info (Spot)
#[derive(Debug, Clone, PartialEq)]
pub struct ProfitSharingInfo {
    /// AUM (Assets Under Management) level/tier
    pub aum_level: String,
    /// Current profit sharing ratio (e.g., "0.10" means 10%)
    pub profit_sharing_ratio: String,
    /// Average AUM over the last 30 days
    pub last30_days_aum_avg: String,
    /// Average AUM over the last 90 days
    pub last90_days_aum_avg: String,
    /// Trader's unique identifier
    pub unique_name: String,
    /// Trader's avatar URL
    pub portrait: String,
    /// Trader's display name (nickname)
    pub petname: String,
    /// Target ID used for default avatar color selection (last digit of identify_id)
    pub target_id: String,
    /// English nickname
    pub en_petname: String,
}

/// Profit sharing ratio configuration
/// 
/// Contains min/max ratio values and limit count for profit sharing.
#[derive(Debug, Clone, PartialEq)]
pub struct ProfitSharingRatio {
    /// Minimum allowed profit sharing ratio
    pub min_profit_sharing_ratio: String,
    /// Maximum allowed profit sharing ratio
    pub max_profit_sharing_ratio: String,
    /// Limit count for ratio changes
    pub limit_num: String,
}

/// Public instrument information for a trader's leading coins
/// 
/// Contains instrument details including selection status and position info.
/// 
/// # Fields
/// - `inst_id`: Instrument ID (e.g., "BTC-USDT-SWAP", "ETH-USDT")
/// - `name`: Coin name (e.g., "BTC", "ETH")
/// - `selected`: Selection status ("0" = not selected, "1" = selected)
/// - `long`: Long position info
/// - `short`: Short position info
#[derive(Debug, Clone, PartialEq)]
pub struct PublicInstrumentInfo {
    /// Instrument ID (e.g., "BTC-USDT-SWAP", "ETH-USDT")
    pub inst_id: String,
    /// Coin name (e.g., "BTC", "ETH")
    pub name: String,
    /// Selection status ("0" = not selected, "1" = selected)
    pub selected: String,
    /// Long position info
    pub long: String,
    /// Short position info
    pub short: String,
    /// Coin icon URL resolved from core-trading's coin store by the base symbol of
    /// `inst_id` (filled by `get_trader_leading_coins`, not by the server). `""` on a
    /// store miss / load failure — native parity with `OkTrade.getCoinUrl`; the platform
    /// renders a placeholder rather than re-querying the native kernel for this leg's value.
    pub coin_url: String,
}

/// Public profit sharing ratio information
/// 
/// Contains min/max profit sharing ratio and limit number for a trader.
/// 
/// # Fields
/// - `min_profit_sharing_ratio`: Minimum profit sharing ratio (e.g., "0", "0.05")
/// - `max_profit_sharing_ratio`: Maximum profit sharing ratio (e.g., "0.5", "0.3")
/// - `limit_num`: Limit number (e.g., "500", "1000")
#[derive(Debug, Clone, PartialEq)]
pub struct PublicProfitSharingRatio {
    /// Minimum profit sharing ratio (e.g., "0", "0.05")
    pub min_profit_sharing_ratio: String,
    /// Maximum profit sharing ratio (e.g., "0.5", "0.3")
    pub max_profit_sharing_ratio: String,
    /// Limit number (e.g., "500", "1000")
    pub limit_num: String,
}

/// Public trade info domain model (UniFFI-compatible)
/// 
/// Contains comprehensive information about a lead trader's public profile
/// when viewed by another user, including the viewer's relationship status.
#[derive(Debug, Clone, PartialEq)]
pub struct PublicTradeInfo {
    /// User role type: "0" = ordinary user, "1" = lead trader, "2" = copy trader
    pub role_type: String,
    /// Whether the trader is a lead trader
    pub is_lead_trader: bool,
    /// Whether the trader is a copy trader
    pub is_copy_trader: bool,
    /// Revenue unit symbol (e.g., "USDT")
    pub symbol: String,
    /// Assets under management (total margin from followers)
    pub aum: String,
    /// Profit sharing ratio (legacy field)
    pub share_ratio: String,
    /// Public copy trading profit sharing ratio
    pub public_share_ratio: String,
    /// Private copy trading profit sharing ratio
    pub private_share_ratio: String,
    /// Current profit sharing ratio based on lead mode
    pub profit_sharing_ratio: String,
    /// Current number of followers
    pub follower_num: String,
    /// Total cumulative follower count
    pub total_follower_num: String,
    /// Maximum number of followers allowed
    pub follower_limit: String,
    /// Whether trader has reached follower capacity
    pub is_full: bool,
    /// Lead mode: "1" = public, "2" = private
    pub lead_mode: String,
    /// Whether lead mode is private
    pub is_private_lead_mode: bool,
    /// Signal protection: "0" = off, "1" = on
    pub signal_protect: String,
    /// Whether signal protection is enabled
    pub is_signal_protection_enabled: bool,
    /// List of instruments configured for lead trading
    pub instruments: Vec<PublicTradeInfoInstrument>,
    /// Follow status: "0" = not following, "1" = following
    pub follower_status: String,
    /// Whether viewer is following this trader
    pub is_following: bool,
    /// Copy relation ID (follow relationship identifier)
    pub copy_rel_id: String,
    /// Follow duration timestamp
    pub follower_time: String,
    /// Viewer's copy PnL when following this trader
    pub copier_follow_pnl: String,
    /// Today's copy PnL
    pub pnl: String,
    /// Total copy trading PnL
    pub follow_pnl: String,
    /// Empty reminder subscription status: "0" = not subscribed, "1" = subscribed
    pub empty_minder_state: String,
    /// Whether viewer is subscribed to empty reminder
    pub is_empty_minder_subscribed: bool,
    /// Whether viewer is an affiliate invitee of this trader
    pub is_affiliate_invitee: bool,
    /// Associated affiliate information (if is_affiliate_invitee is true)
    pub associated_affiliate_info: Option<AssociatedAffiliateInfo>,
    /// Private traffic status: "0" = not whitelisted, "1" = viewer is whitelisted
    pub is_private_traffic: String,
    /// Whether viewer is a whitelisted private copier
    pub is_whitelisted_private_copier: bool,
}

/// Instrument in public trade info (UniFFI-compatible)
/// 
/// Represents a trading instrument configured by the lead trader.
#[derive(Debug, Clone, PartialEq)]
pub struct PublicTradeInfoInstrument {
    /// Instrument ID (e.g., "BTC-USDT-SWAP" for contract, "BTC-USDT" for spot)
    pub inst_id: String,
    /// Instrument display name (e.g., "BTC")
    pub name: String,
    /// Whether this instrument is selected for lead trading
    pub is_selected: bool,
}

/// Rank configuration domain type.
/// 
/// Mirrors Swift `OKTELeadTraderRankConfig`.
/// Values are converted from strings to floats for ease of use.
#[derive(Debug, Clone, PartialEq)]
pub struct RankConfig {
    /// Asset filter configuration
    pub asset: AssetConfig,
    /// AUM filter configuration
    pub aum: AumConfig,
    /// Full state: 0 = not selected, 1 = selected, 2 = default
    pub full_state: i32,
    /// API traders flag (not from API, set to false)
    pub api_traders: bool,
    /// Local traders flag (not from API, set to false)
    pub local_traders: bool,
    /// Trader period options
    pub trader_period: Vec<i32>,
    /// Win ratio options
    pub win_ratio: Vec<f32>,
    /// Country IDs for local trader filter
    pub country_ids: Vec<String>,
}

/// Response from the public rank-config API.
/// 
/// Mirrors Swift `OKTELeadTraderRankConfigDTO`.
/// 
/// # Endpoints
/// - GET /priapi/v5/ecotrade/public/rank-config (Contract)
/// - GET /priapi/v5/ecotrade/public/spot/rank-config (Spot)
#[derive(Debug, Clone, PartialEq)]
pub struct RankConfigDTO {
    /// Asset filter configuration
    pub asset: AssetConfigDTO,
    /// AUM filter configuration
    pub aum: AumConfigDTO,
    /// Trader period options (e.g., [7, 30, 90, 180])
    pub trader_period: Vec<i32>,
    /// Win ratio options (e.g., [0.9, 0.8, 0.7, 0.6, 0.5])
    pub win_ratio: Vec<f32>,
    /// Full state: 0 = not selected, 1 = selected
    pub full_state: i32,
    /// Country IDs for local trader filter
    pub country_ids: Vec<String>,
}

/// Rank reminder information (UniFFI-compatible)
/// 
/// Mirrors native's `RankRemind` shape (`LeaderTierDto.kt`): nested `ValueCcy { value, ccy }`
/// for both recharge_amount and cost_require, null when the server omits the field.
#[derive(Debug, Clone, PartialEq)]
pub struct RankRemind {
    /// Display flag: "1" = show, "2" = hide
    pub display: String,
    /// Recharge amount with currency, or null if not provided.
    pub recharge_amount: Option<ValueCcy>,
    /// Cost requirement with currency, or null if not provided.
    pub cost_require: Option<ValueCcy>,
}

/// Rate chart data point for yield curve.
/// 
/// Mirrors ObjC `OKGTEChartModel`.
#[derive(Debug, Clone, PartialEq)]
pub struct RateChartData {
    /// Historical rate of return (ratio)
    pub ratio: String,
    /// Data timestamp (milliseconds)
    pub stat_time: String,
    /// Profit/loss value (optional, not always present)
    pub pnl: String,
    /// Trading volume (optional, not always present)
    pub amount: String,
}

/// Recommended ratio response from the server.
/// 
/// This struct contains the min and max recommended multiplier ratios
/// for proportional copy trading.
#[derive(Debug, Clone, PartialEq)]
pub struct RecommendRatioDto {
    /// Minimum recommended ratio (multiplier)
    pub min_recommend_ratio: String,
    /// Maximum recommended ratio (multiplier)
    pub max_recommend_ratio: String,
}

/// REST response and WS push payload share the same shape.
/// 
/// Backend returns counts as JSON strings (e.g. `"2"`) and uses
/// `perpetualRelationCount` for the swap count, so we cannot rely on serde's
/// default `rename_all = "camelCase"` mapping.
#[derive(Debug, Clone, PartialEq)]
pub struct RelationCount {
    pub total_count: i64,
    pub swap_count: i64,
    pub spot_count: i64,
}

/// State exposed to native callers via `RelationCountStateCallback`.
#[derive(Debug, Clone, PartialEq)]
pub struct RelationCountState {
    pub total_count: i64,
    pub swap_count: i64,
    pub spot_count: i64,
    pub is_loading: bool,
    pub last_error: Option<String>,
}

/// Requirements for achieving benefits
#[derive(Debug, Clone, PartialEq)]
pub struct Requirements {
    /// Asset requirements
    pub assets: Option<ValueCcy>,
    /// Follower requirements
    pub followers: String,
    /// Assets under management requirements
    pub aum: Option<ValueCcy>,
}

/// Information about other followers shown in retention popup
#[derive(Debug, Clone, PartialEq)]
pub struct RetentionPopupFollower {
    /// Follower's portrait/avatar URL
    pub portrait: String,
    /// Follower's nickname
    pub nick_name: String,
    /// Follower's profit and loss
    pub pnl: String,
    /// Number of days following the trader
    pub follow_days: String,
}

/// Second retention popup data
/// Alternative retention strategy
#[derive(Debug, Clone, PartialEq)]
pub struct SecondRetentionPopup {
    /// Whether the popup should be visible
    /// Server returns "1" for visible, "0" for not visible
    /// Stored as String to preserve original format for Kotlin serialization
    pub visible: String,
    pub detail: Option<SecondRetentionPopupDetails>,
}

/// Details for second retention popup
#[derive(Debug, Clone, PartialEq)]
pub struct SecondRetentionPopupDetails {
    /// Whether the trader is superior performer
    /// Server returns "1" for true, "0" for false
    /// Stored as String to preserve original format for Kotlin serialization
    pub superior: String,
    /// 90-day yield rate
    pub ninety_yield_rate: String,
}

/// Self-user copy eligibility, fed from Kotlin `refreshSelfUserInfo()` result
/// (taken from the active trade-type side of `UserData.swap/spot`), plus the
/// single-position-mode flag from `SocialTradeManager` (trade-core).
/// 
/// Used ONLY on the new-copy branch (edit=false) to decide the copy-full /
/// switch-position-mode eligibility dialogs.
#[derive(Debug, Clone, PartialEq)]
pub struct SelfCopierEligibility {
    /// Whether the self user is a copier on this business line
    /// (= native `IBusinessLineProperties.isCopier()` → `is CopierProperties`).
    pub is_copier: bool,
    /// Whether the copier can still follow more traders
    /// (= native `CopierProperties.copierCanFollow`).
    pub copier_can_follow: bool,
    /// Max traders this copier can copy (= `CopierProperties.maxTradersCanCopy`).
    pub max_traders_can_copy: i32,
    /// DEAD INPUT — read nothing into this and pass `false`.
    /// 
    /// The interactor resolves single-position mode itself from core-trading's account
    /// config where it decides the dialog (`emit_eligibility_dialog`) and never reads this
    /// field: a platform-supplied value would be a cross-FFI snapshot of trade-core state,
    /// and on the Rust leg it could only be produced by the legacy-kernel read this
    /// migration removes. The field stays only because the record is UniFFI-ABI-frozen
    /// (platform-constructed → removing it changes the init symbol).
    pub is_single_position_mode: bool,
}

/// Profit sharing ratio limits and details for settlement
/// 
/// Contains information about profit sharing ratio limits for public and private lead trading.
/// Used by GET /priapi/v5/ecotrade/copytrading/account/profit-sharing-ratio-limit
/// 
/// Note: Named `SettlementRatioLimit` to avoid UniFFI symbol conflict with
/// `ProfitSharingRatioLimit` in `public/types/profit_sharing_ratio_limit.rs`
#[derive(Debug, Clone, PartialEq)]
pub struct SettlementRatioLimit {
    /// Whether the trader is in private lead mode
    /// Note: In JSON, this comes as "leadMode" with values "0"/"1" (public) or "2" (private)
    pub is_private_lead_trader: bool,
    /// Profit sharing ratio details for public lead trading
    pub public_profit_sharing_detail: ProfitSharingRatio,
    /// Profit sharing ratio details for private lead trading
    pub private_profit_sharing_detail: ProfitSharingRatio,
}

/// Smart copy current position
/// GET /priapi/v5/ecotrade/universals/smart/account/copier/positions
#[derive(Debug, Clone, PartialEq)]
pub struct SmartCopyPositionDto {
    /// Currency
    pub ccy: String,
    /// Instrument type (SWAP, SPOT)
    pub inst_type: String,
    /// Instrument ID
    pub inst_id: String,
    /// Margin mode
    pub mgn_mode: String,
    /// Position side
    pub pos_side: String,
    /// Notional USD value
    pub notional_usd: String,
    /// Available position
    pub avail_sub_pos: String,
    /// Current position
    pub sub_pos: String,
    /// Position ID
    pub trade_item_id: String,
    /// Leverage
    pub lever: String,
    /// Average open price
    pub open_avg_px: String,
    /// Unrealized PnL
    pub pnl: String,
    /// PnL ratio
    pub pnl_ratio: f32,
    /// Mark price
    pub mark_px: String,
    /// Last price
    pub last: String,
    /// Margin
    pub margin: String,
    /// Open time
    pub open_time: i64,
    /// Update time
    pub u_time: i64,
    /// Margin ratio
    pub mgn_ratio: f32,
    /// Deal volume (for spot)
    pub deal_volume: String,
    /// Close PnL
    pub close_pnl: String,
    /// Realized PnL
    pub realized_pnl: String,
    /// Fee
    pub fee: String,
    /// Funding fee
    pub funding_fee: String,
    /// Liquidation fee
    pub liquidation_fee: String,
    /// Liquidation price
    pub liq_px: String,
    /// Initial margin requirement
    pub imr: String,
    /// Compensation ticket amount
    pub compensation_ticket_amount: String,
    /// Compensation ticket flag
    pub compensation_ticket_flag: bool,
    /// Compensation ticket unit
    pub compensation_ticket_unit: String,
    /// Compensation ticket name
    pub compensation_ticket_name: String,
    /// Compensation ticket state
    pub compensation_ticket_state: String,
}

/// Smart copy position history
/// GET /priapi/v5/ecotrade/universals/smart/account/copier/positions-history
#[derive(Debug, Clone, PartialEq)]
pub struct SmartCopyPositionHistoryDto {
    /// Instrument ID
    pub inst_id: String,
    /// Position ID
    pub id: String,
    /// Margin mode
    pub mgn_mode: String,
    /// Leverage
    pub lever: String,
    /// Position side
    pub pos_side: String,
    /// Open average price
    pub open_avg_px: String,
    /// Close average price
    pub close_avg_px: String,
    /// Close position type: 1=Partial, 2=Full, 3=Liquidated, 4=ForceReduce, 5=ADL
    pub close_position_type: String,
    /// Create time
    pub c_time: i64,
    /// Update time
    pub u_time: i64,
    /// PnL ratio
    pub pnl_ratio: f32,
    /// Instrument type
    pub inst_type: String,
    /// Close total position
    pub close_total_pos: String,
    /// Realized PnL
    pub realized_pnl: String,
    /// Fee
    pub fee: String,
    /// Funding fee
    pub funding_fee: String,
    /// Deal volume (for spot)
    pub deal_volume: String,
    /// Close PnL
    pub close_pnl: String,
    /// Liquidation fee
    pub liquidation_fee: String,
    /// Currency
    pub ccy: String,
    /// Margin currency
    pub margin_ccy: String,
    /// Contract value
    pub contract_val: String,
    /// Compensation ticket flag
    pub compensation_ticket_flag: bool,
    /// Compensation ticket amount
    pub compensation_ticket_amount: String,
    /// Compensation ticket unit
    pub compensation_ticket_unit: String,
    /// Compensation ticket name
    pub compensation_ticket_name: String,
    /// Compensation ticket state
    pub compensation_ticket_state: String,
    /// Liquidation status
    pub liquidation_status: String,
}

/// Smart copy reducible amount response.
/// 
/// # API
/// GET /priapi/v5/ecotrade/copytrading/relation/copier/smart-reducible-amount
#[derive(Debug, Clone, PartialEq)]
pub struct SmartCopyReducibleAmountDto {
    /// Current follow amount
    pub follow_amount: Option<String>,
    /// Minimum follow amount required
    pub min_follow_amount: Option<String>,
    /// Available amount for copy trading
    pub available_amount: Option<String>,
    /// Amount that can be reduced/withdrawn
    pub reducible_amount: Option<String>,
    /// Difference between follow amount and min amount
    pub follow_and_min_amount_diff: Option<String>,
}

/// Smart copy settings response.
/// 
/// # API
/// GET /priapi/v5/ecotrade/copytrading/relation/copier/smart-settings
#[derive(Debug, Clone, PartialEq)]
pub struct SmartCopySettingsDto {
    /// Current follow amount (investment)
    pub current_follow_amount: Option<String>,
    /// Minimum follow amount
    pub min_follow_amount: Option<String>,
    /// Maximum follow amount
    pub max_follow_amount: Option<String>,
    /// Stop loss amount
    pub stop_loss_amount: Option<String>,
    /// Remaining margin deal mode: 1=market close, 2=follow trader, 3=manual
    pub remain_mgn_deal: Option<String>,
    /// Copy state: "0"=not copying, "1"=copying
    pub copy_state: Option<String>,
    /// Currency symbol (e.g., "USDT")
    pub symbol: Option<String>,
    /// Maximum amount that can be added
    pub max_increase_amount: Option<String>,
    /// Recommended copy mode
    pub recommended_copy_mode: Option<RecommendedCopyModeDto>,
}

/// UniFFI-compatible wrapper for smart copy reducible amount
#[derive(Debug, Clone, PartialEq)]
pub struct SmartCopyTradingReducibleAmount {
    /// Current follow amount
    pub follow_amount: String,
    /// Minimum follow amount required
    pub min_follow_amount: String,
    /// Available amount for copy trading
    pub available_amount: String,
    /// Amount that can be reduced/withdrawn
    pub reducible_amount: String,
    /// Difference between follow amount and min amount
    pub follow_and_min_amount_diff: String,
}

/// UniFFI-compatible wrapper for smart copy settings
#[derive(Debug, Clone, PartialEq)]
pub struct SmartCopyTradingSettings {
    /// Current follow amount (investment)
    pub current_follow_amount: String,
    /// Minimum follow amount
    pub min_follow_amount: String,
    /// Maximum follow amount
    pub max_follow_amount: String,
    /// Stop loss amount
    pub stop_loss_amount: String,
    /// Remaining margin deal mode: 1=market close, 2=follow trader, 3=manual
    pub remain_mgn_deal: String,
    /// Copy state: "0"=not copying, "1"=copying
    pub copy_state: String,
    /// Currency symbol (e.g., "USDT")
    pub symbol: String,
    /// Maximum amount that can be added
    pub max_increase_amount: String,
}

/// State for Smart Copy Trading Interactor
#[derive(Debug, Clone, PartialEq)]
pub struct SmartCopyTradingState {
    /// Content load state
    pub content_state: SmartCopyTradingLoadState,
    /// Trader unique name
    pub trader_unique_name: String,
    /// Copy relation ID
    pub copy_rel_id: String,
    /// Instrument type (SWAP/SPOT)
    pub inst_type: String,
    /// Smart copy settings (populated after load)
    pub settings: Option<SmartCopyTradingSettings>,
    /// Reducible amount (populated after fetch)
    pub reducible_amount: Option<SmartCopyTradingReducibleAmount>,
    /// Available balance for smart copy (from wallet/account, used for increase validation)
    pub available_balance: Option<String>,
    /// Pending stop loss amount for amend
    pub pending_stop_loss_amount: Option<String>,
    /// Pending remain margin deal for amend
    pub pending_remain_mgn_deal: Option<String>,
}

/// Coin auto-product support flags (native `getCoinConfig`: `getTradeCoinInfo(ccy)`'s
/// `supportAutoLend/Earn/Staking == YES("1")`).
#[derive(Debug, Clone, PartialEq)]
pub struct SocialCoinConfig {
    pub support_auto_lend: bool,
    pub support_auto_earn: bool,
    pub support_auto_staking: bool,
}

/// Home banner domain model (UniFFI-compatible)
/// 
/// Represents a banner displayed on the social trading home page.
/// Contains all display information including images for light/dark mode.
#[derive(Debug, Clone, PartialEq)]
pub struct SocialHomeBanner {
    /// Banner unique identifier (called `functionId` in native ObjC model)
    pub function_id: String,
    /// Activity name for the banner
    pub activity_name: String,
    /// Main title displayed on the banner
    pub main_title: String,
    /// Subtitle displayed on the banner
    pub sub_title: String,
    /// Banner image URL for day/light mode
    pub app_day_banner_img: String,
    /// Banner image URL for night/dark mode
    pub app_night_banner_img: String,
    /// Link type for the app (e.g., "1" for web link)
    pub app_link_type: String,
    /// App deep link URL
    pub app_link: String,
    /// Login type required for the link (e.g., "1" for login required)
    pub link_login_type: String,
}

/// A2: the instrument fields the lead-order sort/group logic consumes — native
/// `getSuggestedInstrument` feeds exactly these four reads in `OrderLeadSortUtils`
/// (sort key = tradeSymbol; grouping/predicate = instType + isPositiveContract +
/// quoteSymbol, mirroring `BusinessType.needSortCurrent`, `SortPositionUtils.kt:31-43`).
#[derive(Debug, Clone, PartialEq)]
pub struct SocialInstBrief {
    pub trade_symbol: String,
    pub inst_type: String,
    pub is_positive_contract: bool,
    pub quote_symbol: String,
}

/// Bridge `getInstrumentDisplayInfo` display fields — native
/// `getSuggestedInstrument(instType, instId)`'s `getDisplayQuoteSymbol` / `getDisplayId` /
/// `getTradeQuoteCcyList`. The `instTitle` field of the native table is
/// `tradingBotService.generateFutureGridBotTitle` (a bot service) and stays native. Like
/// [`social_inst_brief`] this uses `get_biz_info_for_id` as the rust stand-in for
/// `getSuggestedInstrument`; a cold / unknown lookup returns `None` and Kotlin falls back to
/// the intact native `getSuggestedInstrument` read (which covers the fuzzy-suggest case).
#[derive(Debug, Clone, PartialEq)]
pub struct SocialInstDisplayInfo {
    pub display_quote_symbol: String,
    pub display_id: String,
    pub trade_quote_ccy_list: Vec<String>,
}

/// Bridge `getOptionQuotePrecision` — native option quote/base precisions:
/// `quote` = SPOT `getIndexConfig("<tradeSymbol>-USDT").quoteDig`, `base` = OPTION
/// `getIndexConfig(instFamily).marginDig`, both `toSafeInt` (0 when absent). `None` = empty /
/// cold option store → Kotlin falls back to the native read (which also
/// covers the `getSuggestedInstrument ?: coinFormat`-derived-family path for a missing
/// instrument). When the instrument IS found, the 0-default matches native's `toSafeInt`.
#[derive(Debug, Clone, PartialEq)]
pub struct SocialOptionQuotePrecision {
    pub quote: i32,
    pub base: i32,
}

/// Position/amount formatted through core-trading's user-setting-unit convertor (F1.2).
/// 
/// `value` is the converted amount at per-instrument precision, NO thousand separator and NO
/// suffix — Kotlin applies `.group()` / suffix at render time, mirroring the split already
/// documented for [`round_pnl`]. `symbol` is the convertor's display symbol (may be empty —
/// Kotlin then runs native's 3-branch fallback, `PositionFormat.kt:100-108`). `unit` is the
/// resolved display unit (`"coin"` / `"u"` / `"contract"` / `"lot"` / `"unknown"`) so Kotlin
/// can pick the localized contract-unit string without re-reading settings.
#[derive(Debug, Clone, PartialEq)]
pub struct SocialPositionFormat {
    pub value: String,
    pub symbol: String,
    pub unit: String,
}

/// Price formatted through core-trading's charge convertors (F1.3).
/// 
/// `value` is the converted price rounded at per-instrument precision, NO thousand separator
/// — Kotlin applies the ICU localization (`formatICUNumber(maxPrecision(100))`,
/// `PriceFormat.kt:303-311`) plus the sign/symbol concatenation. `symbol` / `sign` are the
/// convertor's display currency and prefix (may be empty — native's are too when the charge
/// config is absent).
#[derive(Debug, Clone, PartialEq)]
pub struct SocialPriceFormat {
    pub value: String,
    pub symbol: String,
    pub sign: String,
}

/// Result of a price limit check, wrapped for UniFFI compatibility.
#[derive(Debug, Clone, PartialEq)]
pub struct SpotSafetyChecksCheckResult {
    /// Whether the price triggers the limit check
    pub trigger_limit: bool,
    /// Sell limit price (minimum acceptable sell price), empty if not available
    pub sell_limit: String,
}

/// State for Spot Trade Safety Checks Interactor.
#[derive(Debug, Clone, PartialEq)]
pub struct SpotSafetyChecksState {
    /// Content load state
    pub content_state: SpotSafetyChecksLoadState,
    /// Last check result
    pub check_result: SpotSafetyChecksCheckResult,
    /// The instrument ID that was checked
    pub inst_id: String,
    /// The side that was checked (buy/sell)
    pub side: String,
    /// The price that was checked
    pub px: String,
    /// The size that was checked
    pub sz: String,
}

/// Input for [`init_start_copy_check`] / switch-account re-entry.
#[derive(Debug, Clone, PartialEq)]
pub struct StartCopyCheckInput {
    /// Trade type (Swap/Spot). Passed per-call so the Fragment (ground truth for
    /// tradeType, mirrors native savedStateHandle) always wins — avoids the
    /// construction-order trap (skill incident 2026-05-24).
    pub trade_type: CopySettingsTradeType,
    /// Trader to copy.
    pub trader_unique_name: String,
    /// Relation id (manage flows only).
    pub relation_id: Option<String>,
    /// Should-expand passthrough.
    pub should_expand: bool,
    /// true = manage existing copy, false = new copy.
    pub edit_trade_setting: bool,
    /// When true, emit `ShowSwitchSuccessToast` first
    /// (= native `showToast`, set on switch-account re-entry).
    pub show_toast: bool,
    /// Self-user eligibility (used on new-copy branch only).
    pub eligibility: SelfCopierEligibility,
    /// Pre-provided trader basic info; when Some, skips the basic-info fetch
    /// (mirrors native `basicUserInfo ?:`). When None, the gatekeeper fetches.
    pub prefetched_basic: Option<PrefetchedBasicInfo>,
}

/// Input for submitting first settings.
#[derive(Debug, Clone, PartialEq)]
pub struct SubmitFirstSettingsInput {
    /// Trader's unique name
    pub trader_unique_name: String,
    /// Follow amount per trade
    pub follow_amount: Option<String>,
    /// Maximum follow amount
    pub max_follow_amount: String,
    /// Take profit ratio
    pub tp_ratio: Option<String>,
    /// Stop loss ratio
    pub sl_ratio: Option<String>,
    /// Copy mode
    pub copy_mode: String,
    /// Copy ratio (for ratio mode)
    pub copy_ratio: Option<String>,
    /// Margin mode (for Swap)
    pub margin_mode: Option<String>,
    /// Remaining margin deal mode
    pub remain_margin_deal: Option<String>,
    /// Stop loss amount
    pub stop_loss_amount: Option<String>,
    /// Copy instrument mode
    pub copy_instrument_mode: Option<String>,
    /// Initial amount (for smart copy)
    pub initial_amount: Option<String>,
    /// Replicate positions flag
    pub replication_required: Option<String>,
    /// Private copy link ID
    pub private_copy_link_id: Option<String>,
    /// Selected instruments
    pub instruments: Vec<UserInstrumentDto>,
    /// Coupon ID to bind after first-copy success (optional)
    pub coupon_id: Option<String>,
}

/// Individual subposition data from WebSocket
/// 
/// Contains all fields from both Lead Trader (OKGTELTCurrentOrderModel) and
/// Copy Trader (OKGTECTCurrentOrderModel) position models.
#[derive(Debug, Clone, PartialEq)]
pub struct SubpositionData {
    /// Product/instrument ID (e.g., "BTC-USDT-SWAP")
    pub inst_id: String,
    /// Position unique ID
    pub trade_item_id: String,
    /// Detail IDs for gather positions (used for market close all)
    pub detail_ids: Option<String>,
    /// Margin mode: "cross" or "isolated"
    pub mgn_mode: String,
    /// Leverage multiplier
    pub lever: String,
    /// Position side: "long", "short", or "net"
    /// - long: two-way position long
    /// - short: two-way position short
    /// - net: one-way position
    pub pos_side: String,
    /// Trade side: "buy" or "sell"
    pub side: String,
    /// Position size (number of contracts/shares)
    pub sub_pos: String,
    /// Available position size for closing
    /// When <= 0, position is considered closed
    pub avail_sub_pos: String,
    /// Average opening price
    pub open_avg_px: String,
    /// Latest price
    pub last: String,
    /// Mark price
    pub mark_px: String,
    /// Limit order price (nil if market order)
    pub px: Option<String>,
    /// Margin amount
    pub margin: String,
    /// Initial margin requirement
    pub imr: Option<String>,
    /// Position PnL (profit and loss)
    pub pnl: String,
    /// Position PnL ratio (ROI)
    pub pnl_ratio: String,
    /// Position USD notional value (used for sorting)
    pub notional_usd: String,
    /// Take profit trigger price
    pub tp_trigger_px: Option<String>,
    /// Stop loss trigger price
    pub sl_trigger_px: Option<String>,
    /// Take profit trigger type
    pub tp_trigger_type: Option<String>,
    /// Stop loss trigger type
    pub sl_trigger_type: Option<String>,
    /// Take profit order price (-1 for market order)
    pub tp_ord_px: Option<String>,
    /// Stop loss order price (-1 for market order)
    pub sl_ord_px: Option<String>,
    /// Position opening time (timestamp)
    pub open_time: String,
    /// Position update time (timestamp)
    pub u_time: String,
    /// Margin currency
    pub ccy: String,
    /// Instrument type: "SWAP", "SPOT", "FUTURES", "OPTION", "MARGIN"
    pub inst_type: String,
    /// Trader's unique name identifier
    pub unique_name: String,
    /// Position closing PnL
    pub close_pnl: String,
    /// Transaction fees
    pub fee: String,
    /// Funding fee costs
    pub funding_fee: String,
    /// Liquidation penalty fee
    pub liquidation_fee: String,
    /// Position type for historical categorization:
    /// - "1": Old position (no fees deducted in profit sharing)
    /// - "2": Old position (fees deducted in profit sharing)
    /// - "3": New position after feature launch
    pub pos_type: String,
    /// Maximum sellable amount for spot positions
    pub max_sellable_amount: String,
    /// Spot turnover/deal volume
    pub deal_volume: Option<String>,
    /// Trader nickname
    pub nick_name: Option<String>,
    /// Trader portrait/avatar (returns only suffix)
    pub portrait: Option<String>,
    /// Target ID for avatar generation when portrait is empty
    pub target_id: Option<String>,
    /// Show compensation coupon flag: "0" = not show, "1" = show
    pub compensation_ticket_flag: Option<String>,
    /// Compensation coupon amount
    pub compensation_ticket_amount: Option<String>,
    /// Compensation coupon unit
    pub compensation_ticket_unit: Option<String>,
    /// Compensation coupon display name/title
    pub compensation_ticket_name: Option<String>,
    /// Compensation coupon state
    pub compensation_ticket_state: Option<String>,
}

/// Container for subpositions snapshot data
#[derive(Debug, Clone, PartialEq)]
pub struct SubpositionsSnapshot {
    /// List of position data
    pub positions: Vec<SubpositionData>,
}

/// Result type for system status check (UniFFI-compatible).
/// 
/// Since the endpoint returns no data on success, this is a simple marker type
/// indicating the server status check completed successfully.
#[derive(Debug, Clone, PartialEq)]
pub struct SystemStatusResult {
    /// Indicates the server is valid and available.
    /// Always `true` when returned from a successful check.
    pub is_valid: bool,
}

/// Tier information for trader levels
#[derive(Debug, Clone, PartialEq)]
pub struct Tier {
    /// Tier key identifier
    pub tier_key: String,
    /// Tier level
    pub level: String,
    /// Tier name
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TotalAssetState {
    pub content_state: TotalAssetLoadState,
    /// `None` until the first successful load. Holds the unformatted API
    /// response 1:1 (mirrors native `InvestmentInfoDto`).
    pub info: Option<InvestmentInfo>,
    pub error_message: String,
}

/// Individual trader in the trade expert list.
/// 
/// Mirrors ObjC `OKGTRecommendUserModel`.
#[derive(Debug, Clone, PartialEq)]
pub struct TradeExpertItem {
    /// User's unique identifier
    pub unique_name: String,
    /// User's nickname (display name)
    pub nick_name: String,
    /// User's avatar URL
    pub portrait: String,
    /// User's default avatar map ID
    pub target_id: String,
    /// User's yield rate (return rate percentage as string)
    pub yield_rate: String,
    /// User's risk level ("low", "middle", "high")
    pub risk_level: String,
    /// Maximum drawdown/retreat
    pub max_retreat: String,
    /// Number of fans/followers
    pub fans_num: String,
    /// Follow status: "0" = not followed, "1" = followed
    pub follow_status: String,
    /// Role type: "0" = normal, "1" = lead trader
    pub role_type: String,
    /// Signal role type: "0" = normal, "1" = signal trader
    pub signal_role_type: String,
    /// Strategy role type: "0" = normal, "1" = strategy trader
    pub strategy_role_type: String,
    /// Spot role type: "0" = normal, "1" = spot trader
    pub spot_role_type: String,
    /// Long leverage value
    pub long_lever: String,
    /// Short leverage value
    pub short_lever: String,
    /// Historical rate data points (yield curve)
    pub rates: Vec<TradeExpertRateData>,
}

/// Rate data point for yield curve.
/// 
/// Used for displaying historical rate of return.
#[derive(Debug, Clone, PartialEq)]
pub struct TradeExpertRateData {
    /// Historical rate of return (ratio)
    pub ratio: String,
    /// Data timestamp (milliseconds)
    pub stat_time: String,
}

/// Response from the public trade-expert API.
/// 
/// Mirrors ObjC `OKGTRecommendSectionModel`.
/// 
/// # Endpoint
/// GET /priapi/v5/ecotrade/public/trade-expert
#[derive(Debug, Clone, PartialEq)]
pub struct TradeExpertResponse {
    /// Data version number for cache invalidation.
    /// Should be passed back in subsequent non-refresh requests.
    pub data_version: String,
    /// Number of pages
    pub pages: i32,
    /// Total number of items
    pub total: i32,
    /// List of trade expert (ranked traders)
    pub ranks: Vec<TradeExpertItem>,
}

/// Strong-typed request for trader application form submission.
/// 
/// This struct provides type-safe parameters for the lead trader application,
/// supporting `Vec<String>` fields directly without JSON serialization.
/// 
/// # Platform Support
/// - **Android**: Direct mapping from Kotlin `List<String>` to `Vec<String>`
/// - **iOS**: Direct mapping from Swift `[String]` to `Vec<String>`
/// 
/// # Example (Kotlin)
/// ```kotlin
/// val request = TraderApplyFormRequest(
/// socialAccount = "telegram_user",
/// socialAccountType = "0",
/// signalProviderImages = listOf("url1", "url2"),
/// // ...
/// )
/// useCase.submitApplication(request)
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct TraderApplyFormRequest {
    /// Social account identifier (e.g., Telegram username)
    pub social_account: String,
    /// Social account platform type:
    /// - 0: Telegram
    /// - 1: WeChat
    /// - 2: Discord
    /// - 3: X (Twitter)
    /// - 4: Instagram
    /// - 5: Facebook
    /// - 6: YouTube
    /// - 7: TikTok
    pub social_account_type: String,
    /// Contact information (phone number with area code or email)
    pub contact: String,
    /// Contact type:
    /// - 0: Phone (with area code)
    /// - 1: Email
    /// - 2: Telegram
    /// - 3: Discord
    /// - 4: KakaoTalk
    /// - 5: LINE
    /// - 6: WeChat
    /// - 7: Zalo
    pub contact_type: String,
    /// Introduction/bio text
    pub sign: Option<String>,
    /// Business types (comma-separated):
    /// - 0: Futures/Contract
    /// - 1: Spot
    /// - 2: Strategy
    /// - 3: Signal
    pub biz_types: String,
    /// Affiliate UID
    pub affiliate_uid: String,
    /// Signal provider images (URLs)
    pub signal_provider_images: Vec<String>,
    /// Contract/Futures images (URLs)
    pub images: Vec<String>,
    /// Strategy images (URLs)
    pub strategy_images: Vec<String>,
    /// Spot trading images (URLs)
    pub spot_images: Vec<String>,
    /// Generated global nickname (optional)
    pub generated_global_nickname: Option<String>,
    /// Mark condition flag (default: "1")
    pub mark_condition: String,
    /// Mark condition version (from strings resource)
    pub mark_condition_version: String,
}

/// Public domain model for trader apply form result (exposed via UniFFI)
#[derive(Debug, Clone, PartialEq)]
pub struct TraderApplyFormResult {
    pub biz_type: i32,
    pub audit_type: i32,
}

/// Wrapper for list of trader apply form results (for OKRustFuture compatibility)
#[derive(Debug, Clone, PartialEq)]
pub struct TraderApplyFormResults {
    pub results: Vec<TraderApplyFormResult>,
}

/// Trader cancel status domain model
#[derive(Debug, Clone, PartialEq)]
pub struct TraderCancelStatus {
    /// The cancellation status result
    pub result: TraderCancelStatusResult,
}

/// Trader detail response
/// GET /priapi/v5/ecotrade/universals/relation/copier/detail
#[derive(Debug, Clone, PartialEq)]
pub struct TraderDetailDto {
    /// Trader information with state
    pub trader_info: TraderStateTraderInfo,
    /// Full relation information
    pub relation_info: FullRelationInfo,
    /// Follow PnL details
    pub follow_pnl_detail: FollowPnlDetail,
    /// Follow amount details
    pub follow_amount_detail: FollowAmountDetail,
}

/// Trader follower list domain model (UniFFI-compatible)
/// Contains information about a trader's followers and their statistics
#[derive(Debug, Clone, PartialEq)]
pub struct TraderFollowerList {
    pub seven_days_follower_ratio_flag: String,
    pub seven_days_follower_num: String,
    pub seven_days_follower_ratio: String,
    pub stat_time: String,
    pub seven_days_follower_flag: String,
    pub total_follower_pnl: String,
    pub total_follower_num: String,
    pub pnl_coin_symbol: String,
    pub followers: Vec<FollowerDetails>,
    pub follower_num: String,
    pub aum_coin_symbol: String,
    pub follower_limit: String,
    pub aum: String,
}

/// Trader info domain model (UniFFI-compatible)
/// 
/// Contains lead trader settings and configuration information.
#[derive(Debug, Clone, PartialEq)]
pub struct TraderInfo {
    /// Instrument type (SWAP or SPOT), may be None
    pub inst_type: Option<String>,
    /// Copy restriction mode
    pub recommended_copy_mode: CopyRestrictionMode,
    /// Lead mode (public or private)
    pub lead_mode: LeadMode,
    /// Public profit sharing ratio as decimal string
    pub public_sharing_ratio: String,
    /// Private profit sharing ratio as decimal string
    pub private_sharing_ratio: String,
    /// Signal protection mode
    pub signal_protect_mode: SignalProtectionMode,
    /// Whether auto-add new instruments is enabled
    pub auto_add_new_instruments: bool,
}

/// Trader position result domain model
#[derive(Debug, Clone, PartialEq)]
pub struct TraderPosResult {
    /// The position query result
    pub result: TraderPosResultStatus,
}

/// Empty reminder input.
#[derive(Debug, Clone, PartialEq)]
pub struct TraderProfileEmptyReminderInput {
    pub trader_unique_name: String,
    pub business_line: EmptyReminderBusinessLineFFI,
    pub should_subscribe: bool,
    /// Client-supplied correlation id echoed back verbatim on the terminal
    /// `EmptyReminderUpdated` / `EmptyReminderUpdateError` event. Lets the native
    /// layer correlate each terminal with the exact request that produced it and
    /// drop stale terminals from superseded/timed-out reminder requests (same
    /// contract as `TraderProfileFollowInput::request_id`).
    /// 
    /// `#[uniffi(default = "")]` keeps existing Kotlin/Swift callers source-compatible:
    /// consumers that have not yet been migrated to supply a correlation id still
    /// compile (omitting it yields an empty, no-correlation id) — forward-compat per
    /// OKCore best practices.
    pub request_id: String, // default = ""
}

/// Follow/unfollow input.
#[derive(Debug, Clone, PartialEq)]
pub struct TraderProfileFollowInput {
    pub trader_unique_name: String,
    pub should_follow: bool,
    /// Client-supplied correlation id echoed back verbatim on the terminal
    /// `FollowUpdated` / `FollowUpdateError` event. Lets the native layer correlate
    /// each terminal with the exact request that produced it and drop stale
    /// terminals from superseded/cancelled follow taps (making the stale-terminal
    /// bug class unrepresentable — see the trader-profile-root migration plan §1.2).
    /// 
    /// `#[uniffi(default = "")]` keeps existing Kotlin/Swift callers source-compatible:
    /// consumers that have not yet been migrated to supply a correlation id still
    /// compile (omitting it yields an empty, no-correlation id) — forward-compat per
    /// OKCore best practices.
    pub request_id: String, // default = ""
}

/// Load input for trader profile root.
#[derive(Debug, Clone, PartialEq)]
pub struct TraderProfileRootLoadInput {
    pub trader_unique_name: String,
    pub is_self_profile: bool,
    pub time_zone: String,
    /// Client-supplied correlation id echoed back verbatim on the terminal
    /// `ProfileLoaded` / `ProfileLoadError` events. Lets the native layer correlate
    /// each load terminal with the exact load/refresh command that initiated that
    /// load cycle and drop stale terminals from superseded loads (same contract as
    /// `TraderProfileFollowInput::request_id`). A degraded load emits BOTH terminals
    /// (`ProfileLoaded` then `ProfileLoadError`) carrying the SAME id.
    /// 
    /// `#[uniffi(default = "")]` keeps existing Kotlin/Swift callers source-compatible:
    /// consumers that have not yet been migrated to supply a correlation id still
    /// compile (omitting it yields an empty, no-correlation id) — forward-compat per
    /// OKCore best practices.
    pub request_id: String, // default = ""
}

/// State for Trader Profile Root Interactor.
#[derive(Debug, Clone, PartialEq)]
pub struct TraderProfileRootState {
    pub trader_unique_name: String,
    pub is_self_profile: bool,
    pub time_zone: String,
    pub content_state: ContentLoadState,
    pub error_message: String,
    pub basic_info: Option<EcoBasicUserInfoResponse>,
    pub self_contract_trade_info: Option<UserTradeInfo>,
    pub self_spot_trade_info: Option<UserSpotTradeInfoPublic>,
    pub public_contract_trade_info: Option<PublicTradeInfo>,
    pub public_spot_trade_info: Option<PublicTradeInfo>,
    pub is_swap_lead_trader: bool,
    pub is_spot_lead_trader: bool,
    pub is_lead_trader: bool,
    pub is_following: bool,
    pub is_empty_reminder_spot_subscribed: bool,
    pub is_empty_reminder_contract_subscribed: bool,
    /// True when the user is a normal user in both swap and spot (no trade info to show).
    pub is_unable_follow: bool,
}

/// Trader settings response for copy trading setup.
/// 
/// # API
/// GET /priapi/v5/ecotrade/copier/trader-setting
/// GET /priapi/v5/ecotrade/spot/copier/trader-setting
#[derive(Debug, Clone, PartialEq)]
pub struct TraderSettingsDto {
    /// List of tradeable instruments
    pub instruments: Vec<UserInstrumentDto>,
    /// Margin mode: cross, isolated, copy_trader
    pub mgn_mode: String,
    /// Maximum take profit ratio
    pub max_tp_ratio: String,
    /// Maximum stop loss ratio
    pub max_sl_ratio: String,
    /// Maximum total follow amount per relation
    pub max_total_amount: String,
    /// Minimum total follow amount per relation
    pub min_total_amount: String,
    /// Minimum follow amount per trade
    pub min_follow_amount: String,
    /// Maximum follow amount per trade
    pub max_follow_amount: String,
    /// Current set follow amount
    pub current_follow_amount: String,
    /// Current set max amount
    pub current_max_amount: String,
    /// Current take profit ratio
    pub tp_ratio: String,
    /// Current stop loss ratio
    pub sl_ratio: String,
    /// Symbol (e.g., "USDT")
    pub symbol: String,
    /// Average daily open positions
    pub open_num_avg: Option<String>,
    /// Copy stop loss amount
    pub stop_loss_amount: Option<String>,
    /// Remaining margin deal mode: 1=market close, 2=follow trader, 3=manual
    pub remain_mgn_deal: Option<String>,
    /// Minimum ratio for ratio copy mode
    pub min_ratio: Option<String>,
    /// Maximum ratio for ratio copy mode
    pub max_ratio: Option<String>,
    /// Historical position average for ratio mode
    pub history_item_average: Option<String>,
    /// Copy mode: FIXED_COST, RATIO_COPY, SMART_COPY
    pub copy_mode: Option<String>,
    /// Copy ratio (for ratio mode)
    pub copy_ratio: String,
    /// Copy state: "0" = not copying, "1" = copying
    pub copy_state: String,
    /// Copy instrument mode
    /// Response values: "1" = custom, "2" = follow trader
    /// Note: Request body uses different values (see CopierFirstSettingsRequest)
    pub copy_instrument_mode: String,
    /// Recommended copy mode
    pub recommended_copy_mode: Option<RecommendedCopyModeDto>,
}

/// Trader info with state (used in trader detail)
#[derive(Debug, Clone, PartialEq)]
pub struct TraderStateTraderInfo {
    /// Trader portrait URL
    pub portrait: String,
    /// Trader nickname
    pub nick_name: String,
    /// Lead mode raw API value: "0"/"1" = public, "2" = private.
    /// See PartialTraderInfo.lead_mode for rationale.
    pub lead_mode: String,
    /// Unique name identifier
    pub unique_name: String,
    /// Target ID
    pub target_id: String,
    /// Trader state: 0=Leading, 1=NoLongerLeading
    pub trader_state: String,
    /// Whether this trader is a sub-account ("0"=false, non-"0"=true).
    /// Surfaced by BE on every endpoint that returns a `traderInfo` block
    /// (detail, current-list, history-list) so native clients can gate
    /// sub-account-specific UI: routing the avatar tap in TraderDetail to
    /// the business-line L2 page instead of the comprehensive profile, and
    /// hiding the "Personal page" entry on L2 — sub-accounts have no
    /// comprehensive profile.
    /// 
    /// Field is forward-compatible: defaults to `false` when BE omits it,
    /// so older deployments and the BE rollout window stay safe.
    pub is_sub_account: bool,
}

/// Trader status domain model (UniFFI-compatible)
/// Contains status information for different types of lead trader applications
#[derive(Debug, Clone, PartialEq)]
pub struct TraderStatus {
    /// Overall result status (for futures/contract lead trader)
    pub result: Option<String>,
    /// Strategy lead trader application status
    pub strategy_result: Option<String>,
    /// Signal provider lead trader application status
    pub signal_provider_result: Option<String>,
    /// Spot lead trader application status
    pub spot_result: Option<String>,
}

/// Trader tier information.
/// 
/// Mirrors Swift `TraderTier` (ObjC name: `OKTETraderTier`).
#[derive(Debug, Clone, PartialEq)]
pub struct TraderTier {
    /// Tier key: "0" = ordinary, "1" = level 1, etc.
    pub tier_key: String,
    /// Tier level display string (e.g., "Lvl 1")
    pub level: String,
    /// Tier name (e.g., "Bronze", "Silver", "Gold", "Platinum")
    pub name: String,
}

/// Individual trade performance detail (UniFFI-compatible)
#[derive(Debug, Clone, PartialEq)]
pub struct TraderTradePerformanceDetail {
    /// Title of the stat (e.g., "Days leading trades", "Win rate")
    pub title: String,
    /// Description of the stat
    pub description: String,
    /// URL for more information
    pub learn_more_url: String,
    /// Display order
    pub order: i32,
    /// Function ID for identifying the stat type (e.g., "winRatio", "profitDays")
    pub function_id: String,
    /// Type of the value (0=decimal, 1=percentage, 2=integer, 3=string, 4=splicedString)
    pub data_type: String,
    /// Raw value of the stat as string
    pub value: String,
}

/// Trader trade performance stats domain model (UniFFI-compatible)
/// Contains trade performance statistics for a trader
#[derive(Debug, Clone, PartialEq)]
pub struct TraderTradePerformanceStats {
    /// Periodic part - stats that change based on the selected time filter
    pub periodic_part: Vec<TraderTradePerformanceDetail>,
    /// Non-periodic part - stats that are not time-dependent
    pub non_periodic_part: Vec<TraderTradePerformanceDetail>,
}

/// Request body for transferring smart copy amount.
/// 
/// # API
/// POST /priapi/v5/ecotrade/copytrading/relation/copier/transfer-smart-amount
#[derive(Debug, Clone, PartialEq)]
pub struct TransferSmartAmountRequest {
    /// Copy relation ID
    pub copy_rel_id: String,
    /// Transfer type: "2"=increase, "3"=decrease
    pub transfer_type: String,
    /// Transfer amount
    pub transfer_amount: String,
    /// Instrument type (SWAP/SPOT)
    pub inst_type: String,
}

/// Request to update profit sharing ratio.
#[derive(Debug, Clone, PartialEq)]
pub struct UpdateProfitRatioRequest {
    /// New ratio as decimal string (e.g., "0.10" for 10%)
    pub ratio: String,
}

/// Request body for updating recommended copy mode.
/// 
/// This is used by lead traders to set their recommended copy mode.
/// 
/// # API
/// PUT /priapi/v5/ecotrade/copytrading/account/recommended-copy-mode
#[derive(Debug, Clone, PartialEq)]
pub struct UpdateRecommendedCopyModeRequest {
    /// Instrument type (SWAP/SPOT)
    pub inst_type: String,
    /// Recommended copy mode: "0"=Unrestricted, "1"=Smart, "2"=Custom
    pub recommended_copy_mode: String,
}

/// Upgrade popup information
#[derive(Debug, Clone, PartialEq)]
pub struct UpgradePopupInfo {
    /// Whether popup should be shown (server returns as string "0"/"1")
    pub should_pop_up: bool,
    /// Information about the upgraded tier
    pub upgraded_tier: Option<Tier>,
}

/// Result wrapper for UpgradePopupInfo list operations
#[derive(Debug, Clone, PartialEq)]
pub struct UpgradePopupInfoResult {
    /// Success data, None if error occurred
    pub data: Option<Vec<UpgradePopupInfo>>,
    /// Error information, None if successful
    pub error: Option<OKSTFFIError>,
}

/// Country information for user profile
#[derive(Debug, Clone, PartialEq)]
pub struct UserBasicInfoCountryInfo {
    /// Country identifier
    pub country_id: String,
    /// Country name for display
    pub country_name: String,
    /// Timestamp when restriction started (if applicable)
    pub restriction_start_time: i64,
    /// Whether the country is restricted
    pub is_restricted: bool,
}

/// Instrument information for copy trading.
#[derive(Debug, Clone, PartialEq)]
pub struct UserInstrumentDto {
    /// Instrument ID (e.g., "BTC-USDT-SWAP")
    pub inst_id: String,
    /// Instrument name. Native `UserInstruments.name: String? = ""` — kotlinx
    /// omits it on `""` (the declared default) and emits `"name":null` when the
    /// backend handed back an explicit null. Mirror both halves.
    pub name: Option<String>,
    /// Is selected: "0" = no, "1" = yes. Native `selected: String? = ""` — same
    /// omit-on-`""`, emit-on-null rule as `name`.
    pub selected: Option<String>,
    /// Long position info. Native `UserInstruments.long` is `String = ""` with
    /// kotlinx `encodeDefaults=false` → omitted when at default. Mirror by
    /// skipping empty so Spot bodies don't carry useless `"long":""`.
    pub long: String,
    /// Short position info. Same omit-when-empty rule as `long`.
    pub short: String,
}

/// User instrument info domain model (UniFFI-compatible)
/// 
/// Contains information about a trading instrument that the user
/// has configured for lead trading, including selection status.
#[derive(Debug, Clone, PartialEq)]
pub struct UserInstrumentInfo {
    /// Instrument ID (e.g., "BTC-USDT-SWAP" for futures, "BTC-USDT" for spot)
    pub inst_id: String,
    /// Coin name (e.g., "BTC", "ETH")
    pub name: String,
    /// Selected status ("0" = not selected, "1" = selected)
    pub selected: String,
}

/// Wrapper for a list of user instrument info (UniFFI-compatible)
/// 
/// Used as the return type for fetching user's configured lead trading instruments.
#[derive(Debug, Clone, PartialEq)]
pub struct UserInstrumentInfoList {
    /// List of user instruments
    pub instruments: Vec<UserInstrumentInfo>,
}

/// User profile overview information
/// Matches Kotlin UserProfileOverviewDto
#[derive(Debug, Clone, PartialEq)]
pub struct UserProfileOverview {
    /// Profit and loss (NumericString in Kotlin, String in Rust)
    pub pnl: String,
    /// Profit and loss ratio (NumericString in Kotlin, String in Rust)
    pub pnl_ratio: String,
    /// Equity (NumericString in Kotlin, String in Rust)
    pub equity: String,
    /// Onboard duration in days (Long in Kotlin, i64 in Rust)
    /// Server returns as string, use custom deserializer (matches LongAsStringSerializer)
    pub onboard_duration: i64,
    /// Maximum retreat (Float in Kotlin, f32 in Rust)
    /// Server returns as string, use custom deserializer
    pub max_retreat: f32,
    /// Risk reward ratio (String in both)
    pub risk_reward_ratio: String,
    /// Win rate (Float in Kotlin, f32 in Rust)
    /// Server returns as string, use custom deserializer
    pub win_rate: f32,
    /// Withdrawal amount (NumericString in Kotlin, String in Rust)
    pub withdrawal: String,
}

/// Instrument in user spot trade info (UniFFI-compatible)
#[derive(Debug, Clone, PartialEq)]
pub struct UserSpotTradeInfoInstrument {
    /// Instrument ID (e.g., "BTC-USDT")
    pub inst_id: String,
    /// Instrument display name (e.g., "BTC")
    pub name: String,
    /// Whether this instrument is selected for lead trading
    pub is_selected: bool,
}

/// User spot trade info domain model (UniFFI-compatible)
#[derive(Debug, Clone, PartialEq)]
pub struct UserSpotTradeInfoPublic {
    pub role_type: String,
    pub is_lead_trader: bool,
    pub is_copy_trader: bool,
    pub symbol: String,
    pub aum: String,
    pub share_ratio: String,
    pub public_share_ratio: String,
    pub private_share_ratio: String,
    pub follower_num: String,
    pub total_follower_num: String,
    pub follower_limit: String,
    pub lead_mode: String,
    pub is_private_lead_mode: bool,
    pub pre_profit: String,
    pub instruments: Vec<UserSpotTradeInfoInstrument>,
    pub signal_protect: String,
    pub is_signal_protection_enabled: bool,
    pub auto_add_new_inst: String,
    pub is_auto_add_new_inst_enabled: bool,
    pub recommended_copy_mode: String,
    pub rank_remind: Option<RankRemind>,
    pub grade_level: String,
    pub follow_pnl: String,
    pub pnl: String,
    pub margin: String,
    pub copier_can_follow: String,
    pub copier_followee_limit: String,
    pub unrealized_pnl: String,
    pub realized_pnl: String,
    pub settle_profit: String,
    pub wait_settle_profit: String,
    pub affiliate_trade_volume: String,
    pub is_affiliate_trader: bool,
}

/// User trade info domain model (UniFFI-compatible)
/// 
/// Contains comprehensive trading information for a user including:
/// - Role and account settings
/// - Profit sharing configuration
/// - Follower information
/// - Trading instruments
/// - PnL data
#[derive(Debug, Clone, PartialEq)]
pub struct UserTradeInfo {
    /// User role: "1" = lead trader, "2" = copier, "0" = regular user
    pub role_type: String,
    /// Revenue unit symbol (e.g., "USDT")
    pub symbol: String,
    /// Total margin from followers (AUM)
    pub aum: String,
    /// Public copy trading profit sharing ratio
    pub public_share_ratio: String,
    /// Private copy trading profit sharing ratio
    pub private_share_ratio: String,
    /// Current follower count
    pub follower_num: String,
    /// Cumulative total follower count
    pub total_follower_num: String,
    /// Maximum followers allowed
    pub follower_limit: String,
    /// List of trading instruments
    pub instruments: Vec<UserTradeInfoInstrument>,
    /// Expected profit to be shared
    pub pre_profit: String,
    /// Total copy trading PnL
    pub follow_pnl: String,
    /// Today's copy PnL
    pub pnl: String,
    /// Deposit margin
    pub margin: String,
    /// Copy verification status: "0" = cannot copy, "1" = can copy
    pub copier_can_follow: String,
    /// Maximum traders a copier can follow
    pub copier_followee_limit: String,
    /// Unrealized PnL
    pub unrealized_pnl: String,
    /// Realized PnL
    pub realized_pnl: String,
    /// Settled profit sharing
    pub settle_profit: String,
    /// Unsettled profit sharing
    pub wait_settle_profit: String,
    /// Lead mode: "1" = public, "2" = private
    pub lead_mode: String,
    /// Signal protection: "0" = off, "1" = on
    pub signal_protect: String,
    /// Recommended copy mode: "0" = no restriction, "1" = custom only, "2" = smart only
    pub recommended_copy_mode: String,
    /// Auto add new instruments: "1" = yes, "2" = no
    pub auto_add_new_inst: String,
    /// Whether user is an affiliate trader
    pub is_affiliate_trader: bool,
    /// Affiliate trade volume
    pub affiliate_trade_volume: String,
    /// Rank reminder information (optional)
    pub rank_remind: Option<RankRemind>,
    /// Trader tier grade level
    pub grade_level: String,
    /// Private traffic status: "0" = not whitelisted, "1" = whitelisted
    pub is_private_traffic: String,
}

/// Instrument in user trade info (UniFFI-compatible)
/// 
/// Represents a trading instrument/coin pair that the user has configured.
#[derive(Debug, Clone, PartialEq)]
pub struct UserTradeInfoInstrument {
    /// Instrument ID (e.g., "BTC-USDT-SWAP")
    pub inst_id: String,
    /// Instrument name (e.g., "BTC")
    pub name: String,
    /// Selection status: "0" = not selected, "1" = selected
    pub selected: String,
}

/// Value with currency information
#[derive(Debug, Clone, PartialEq)]
pub struct ValueCcy {
    /// Value amount
    pub value: String,
    /// Currency code - defaults to empty string (matches Kotlin Ccy.Empty)
    pub ccy: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VanguardShareCardPresentation {
    pub kind: VanguardShareCardKind,
    pub is_vip: bool,
    pub profile: VanguardShareProfilePresentation,
    pub primary_metric: VanguardShareMetricPresentation,
    /// Label with its `{ccy}` substitution already attached.
    pub primary_metric_label: DisplayString,
    pub show_chart: bool,
    pub chart_points: Vec<VanguardShareChartPoint>,
    pub show_follower_income: bool,
    pub follower_income_label: DisplayString,
    pub follower_income: DisplayString,
    pub follower_income_tone: VanguardShareTone,
    pub followers: Vec<VanguardShareFollowerPresentation>,
    pub show_copying_row: bool,
    pub copying_label: DisplayString,
    pub copying_trader_name: String,
    pub is_empty: bool,
    pub empty_kind: VanguardShareEmptyKind,
    pub trade: Option<VanguardShareTradePresentation>,
    pub background_tone: VanguardShareTone,
    pub im_business_name: String,
    pub im_cta_kind: VanguardShareImCtaKind,
    pub im_message_preview_cd_key: String,
    /// IM title with the KOL name already substituted.
    pub im_title: DisplayString,
    pub im_content_cd_key: String,
    pub im_tagline_cd_key: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VanguardShareChartPoint {
    pub period: f64,
    /// Yield ratio already scaled to percent units.
    pub value: f64,
}

/// UniFFI-exported vanguard share data
/// 
/// Contains all trader share information for generating share cards.
#[derive(Debug, Clone, PartialEq)]
pub struct VanguardShareData {
    /// Server timestamp in milliseconds as string
    pub server_time: String,
    /// Trader's display name
    pub nick_name: String,
    /// Trader's avatar URL
    pub portrait: String,
    /// Original portrait URL (without filters)
    pub original_portrait: String,
    /// Original avatar URL
    pub original_avatar: String,
    /// Target ID for avatar generation
    pub target_id: String,
    /// Whether user is VIP
    pub vip: bool,
    /// QR code URL for sharing
    pub qr_code_url: String,
    /// User role type
    pub role_type: String,
    /// Current number of followers/copiers
    pub current_follower_number: String,
    /// Maximum allowed followers
    pub max_follower_number: String,
    /// Trader's total PnL
    pub trader_pnl: String,
    /// Currency symbol (e.g., "USDT")
    pub symbol: String,
    /// Win ratio as decimal string
    pub trader_win_ratio: String,
    /// PnL ratio as decimal string
    pub trader_pnl_ratio: String,
    /// Number of fans
    pub fans_num: String,
    /// Total yield/PnL ratio
    pub pnl_ratio: String,
    /// Registration timestamp
    pub register_time: String,
    /// Historical follower count
    pub history_follow_num: String,
    /// 30-day historical PnL
    pub thirty_history_pnl: String,
    /// 30-day yield rate
    pub thirty_yield_rate: String,
    /// 7-day history rate
    pub seven_history_rate: String,
    /// Cumulative follow PnL
    pub follow_pnl: String,
    /// Yield curve data points
    pub yield_curve: Vec<VanguardShareYieldCurve>,
    /// Top followers list
    pub followers: Vec<VanguardShareFollower>,
}

/// UniFFI-exported follower data for share card
#[derive(Debug, Clone, PartialEq)]
pub struct VanguardShareFollower {
    /// Follower's avatar URL
    pub portrait: String,
    /// Follower's display name
    pub nick_name: String,
    /// Follower's unique identifier
    pub unique_name: String,
    /// Target ID for avatar generation
    pub target_id: String,
    /// Follow timestamp
    pub follow_time: String,
    /// Number of days following
    pub follow_days: String,
    /// Profit and loss from following
    pub pnl: String,
    /// Coin symbol (e.g., "USDT", "BTC")
    pub coin_symbol: String,
    /// User ID (may be empty for privacy)
    pub uid: String,
    /// Assets under management
    pub aum: String,
    /// Invested amount
    pub invested_amount: String,
    /// Profit sharing ratio
    pub profit_sharing_ratio: String,
}

/// Follower info for UniFFI export (share card top followers)
#[derive(Debug, Clone, PartialEq)]
pub struct VanguardShareFollowerInfo {
    /// Follower's avatar URL
    pub portrait: String,
    /// Follower's display name
    pub nick_name: String,
    /// Follower's unique identifier
    pub unique_name: String,
    /// Target ID for avatar generation
    pub target_id: String,
    /// Follow timestamp
    pub follow_time: String,
    /// Number of days following
    pub follow_days: String,
    /// Profit and loss from following
    pub pnl: String,
    /// Coin symbol (e.g., "USDT", "BTC")
    pub coin_symbol: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VanguardShareFollowerPresentation {
    pub user_name: String,
    pub portrait: String,
    pub target_id: String,
    /// Money row: `TradingNumber` so native applies locale grouping.
    pub pnl: DisplayString,
    pub tone: VanguardShareTone,
    /// 0-based row index; the host picks the rank badge asset for 0...2.
    pub rank: u32,
}

/// Flattened trader share info for UniFFI export
#[derive(Debug, Clone, PartialEq)]
pub struct VanguardShareInfo {
    /// Server timestamp in milliseconds as string
    pub server_time: String,
    /// Trader's display name
    pub nick_name: String,
    /// Trader's avatar URL
    pub portrait: String,
    /// Original portrait URL (without filters)
    pub original_portrait: String,
    /// Original avatar URL
    pub original_avatar: String,
    /// Target ID for avatar generation
    pub target_id: String,
    /// Whether user is VIP
    pub vip: bool,
    /// QR code URL for sharing
    pub qr_code_url: String,
    /// User role type
    pub role_type: String,
    /// Current number of followers/copiers
    pub current_follower_number: String,
    /// Maximum allowed followers
    pub max_follower_number: String,
    /// Trader's total PnL
    pub trader_pnl: String,
    /// Currency symbol (e.g., "USDT")
    pub symbol: String,
    /// Win ratio as decimal string
    pub trader_win_ratio: String,
    /// PnL ratio as decimal string
    pub trader_pnl_ratio: String,
    /// Number of fans
    pub fans_num: String,
    /// Total yield/PnL ratio
    pub pnl_ratio: String,
    /// Registration timestamp
    pub register_time: String,
    /// Historical follower count
    pub history_follow_num: String,
    /// 30-day historical PnL
    pub thirty_history_pnl: String,
    /// 30-day yield rate
    pub thirty_yield_rate: String,
    /// 7-day history rate
    pub seven_history_rate: String,
    /// Cumulative follow PnL
    pub follow_pnl: String,
    /// Yield curve data points
    pub yield_curve: Vec<VanguardShareYieldCurvePoint>,
    /// Top followers list
    pub followers: Vec<VanguardShareFollowerInfo>,
}

/// A percentage metric. `value` is `Percent` with the already-scaled number, or
/// `Placeholder` when the payload has none.
#[derive(Debug, Clone, PartialEq)]
pub struct VanguardShareMetricPresentation {
    pub value: DisplayString,
    pub tone: VanguardShareTone,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VanguardSharePresentation {
    /// Custom URL when the caller supplied a non-empty one, otherwise the
    /// payload's QR URL. Empty means the host shows its no-network error.
    pub effective_qr_code_url: String,
    pub server_time: String,
    pub nick_name: String,
    pub portrait: String,
    pub target_id: String,
    pub symbol: String,
    pub vip: bool,
    pub deeplink_intent: VanguardShareDeeplinkIntent,
    pub link_id: Option<String>,
    pub cards: Vec<VanguardShareCardPresentation>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VanguardSharePresentationRequest {
    pub unique_name: String,
    pub trade_type: VanguardShareTradeType,
    pub manager_type: VanguardShareManagerType,
    pub custom_url: Option<String>,
    pub link_id: Option<String>,
    pub is_self_share: bool,
    pub is_lite_mode: bool,
    pub trade_seed: Option<VanguardShareTradeSeed>,
    pub user_seed: Option<VanguardShareUserSeed>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VanguardShareProfilePresentation {
    pub user_name: String,
    pub avatar_url: String,
    pub target_id: String,
    /// "{num} followers" with the count already substituted as a param, or
    /// `Placeholder` when the card shows no subtitle.
    pub follower_subtitle: DisplayString,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VanguardShareTradePresentation {
    /// core-trading trade-title segments, carried through unchanged.
    pub title_segments: Vec<DisplayString>,
    /// Used when the instrument stores are cold and `title_segments` is empty.
    pub title_fallback: String,
    /// Empty for a net history position, which legacy renders without a tag.
    pub position: DisplayString,
    pub position_tone: VanguardShareTone,
    pub leverage: DisplayString,
    /// Spot cards show a static "Spot" tag instead of position + leverage.
    pub spot_tag: DisplayString,
    pub coin_url: String,
    pub pnl_label: DisplayString,
    pub open_price_label: DisplayString,
    pub current_price_label: DisplayString,
    pub open_price: DisplayString,
    pub current_price: DisplayString,
}

/// Raw trade inputs captured at the share entry point. Optional: profile,
/// billboard and private-invite cards carry no trade.
#[derive(Debug, Clone, PartialEq)]
pub struct VanguardShareTradeSeed {
    pub inst_id: String,
    pub inst_type: String,
    pub side: String,
    pub pos_side: String,
    /// Net-mode available position. Empty for history orders, where legacy
    /// renders no direction tag for a net position.
    pub available_position: String,
    pub raw_open_price: String,
    pub raw_current_price: String,
    pub raw_close_price: String,
    pub raw_pnl_ratio: String,
    pub raw_leverage: String,
    pub is_history: bool,
    pub is_liquidated: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VanguardShareUserSeed {
    pub nick_name: String,
    pub english_nick_name: String,
    pub portrait: String,
    pub target_id: String,
    pub prefer_chinese_name: bool,
}

/// UniFFI-exported yield curve data point for share card
#[derive(Debug, Clone, PartialEq)]
pub struct VanguardShareYieldCurve {
    /// Yield ratio as string
    pub ratio: String,
    /// Statistics timestamp in milliseconds as string
    pub stat_time: String,
}

/// Yield curve data point for UniFFI export
#[derive(Debug, Clone, PartialEq)]
pub struct VanguardShareYieldCurvePoint {
    /// Yield ratio as string
    pub ratio: String,
    /// Statistics timestamp in milliseconds as string
    pub stat_time: String,
}

/// Weekly PnL chart data item
/// Corresponds to OKTEWeeklyPnlChartModel in iOS
#[derive(Debug, Clone, PartialEq)]
pub struct WeeklyPnlChartDataItem {
    /// Week start timestamp in milliseconds as string
    pub stat_time: String,
    /// Profit and loss value as string with high precision decimals
    pub pnl: String,
    /// Yield ratio as string (decimal format, can be scientific notation like "1.0E-4")
    pub ratio: String,
}

/// UniFFI-exported yield PnL chart data item
/// 
/// Represents a single data point in the yield/PnL chart for a trader.
/// All numeric values are kept as strings to preserve precision.
#[derive(Debug, Clone, PartialEq)]
pub struct YieldPnlChartDataItem {
    /// Profit and loss value as string (high precision decimal)
    /// Examples: "0", "5439.9331118460000000", "-9850.8167831501786068"
    pub pnl: String,
    /// Yield ratio as string (decimal format)
    /// Examples: "0.0", "0.0085", "-0.0154", "3.3293"
    pub ratio: String,
    /// Statistics timestamp in milliseconds as string
    /// Example: "1763481600000"
    pub stat_time: String,
}

// -----------------------------------------------------------------------------
// 3. Interfaces / Interactors / Services (154)
// -----------------------------------------------------------------------------

/// Copier Message Notifications Interactor
/// 
/// Manages the copier message notifications page.
/// Fetches historical pop-up messages and supports marking them as read.
pub struct CopierMessageInteractor;
impl CopierMessageInteractor {
    /// Load notification messages.
    pub fn load_data(&self) { todo!() }
    /// Mark messages as read.
    pub fn mark_as_read(&self, notice_ids: Vec<String>) { todo!() }
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
    pub fn set_ui_event_observer(&self, observer: Arc<dyn CopierMessageUiEventObserver>) { todo!() }
    /// Start the interactor lifecycle.
    /// 
    /// Calls the user's `InteractorLifecycle::start()` implementation.
    /// Override that method to add custom startup behavior (e.g., initial data loads).
    pub fn start(&self) { todo!() }
    /// Get the current state (UniFFI export).
    /// 
    /// Returns the External type (UniFFI-compatible).
    pub fn state(&self) -> CopierMessageState { todo!() }
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
    pub fn ui_event_stream(&self) -> Arc<CopierMessageUiEventStream> { todo!() }
}

/// Deprecated UniFFI callback interface for observing UI events. Prefer the cycle-free `CopierMessageUiEventStream` returned by `interactor.ui_event_stream()`.
pub struct CopierMessageUiEventObserver;
impl CopierMessageUiEventObserver {
    pub fn on_ui_event(&self, event: CopierMessageUiEvent) { todo!() }
}

/// Opt-in async event stream from `interactor.ui_event_stream()`.
/// Single-subscriber, latest-wins. `next()` resolves to `None` when the
/// interactor is dropped or a new subscription replaces this one.
pub struct CopierMessageUiEventStream;
impl CopierMessageUiEventStream {
    pub async fn next(&self) -> Option<CopierMessageUiEvent> { todo!() }
}

/// CopierPnlWs Interactor
/// 
/// Manages WebSocket subscription for live PnL updates.
/// Wraps CopierPnlLiveDataSource and adapts it to the interactor macro pattern.
pub struct CopierPnlWsInteractor;
impl CopierPnlWsInteractor {
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
    /// Set rise/fall cycle and restart if running.
    pub fn set_cycle(&self, rise_fall_cycle: CopierPnlRiseFallCycleType) { todo!() }
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
    pub fn set_ui_event_observer(&self, observer: Arc<dyn CopierPnlWsUiEventObserver>) { todo!() }
    /// Start the interactor lifecycle.
    /// 
    /// Calls the user's `InteractorLifecycle::start()` implementation.
    /// Override that method to add custom startup behavior (e.g., initial data loads).
    pub fn start(&self) { todo!() }
    /// Start WebSocket subscription.
    pub fn start_ws(&self) { todo!() }
    /// Get the current state (UniFFI export).
    /// 
    /// Returns the External type (UniFFI-compatible).
    pub fn state(&self) -> CopierPnlWsState { todo!() }
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
    /// Stop WebSocket subscription.
    pub fn stop_ws(&self) { todo!() }
    /// Opt-in async UI event stream (cycle-free). Single-subscriber.
    pub fn ui_event_stream(&self) -> Arc<CopierPnlWsUiEventStream> { todo!() }
}

/// Deprecated UniFFI callback interface for observing UI events. Prefer the cycle-free `CopierPnlWsUiEventStream` returned by `interactor.ui_event_stream()`.
pub struct CopierPnlWsUiEventObserver;
impl CopierPnlWsUiEventObserver {
    pub fn on_ui_event(&self, event: CopierPnlWsUiEvent) { todo!() }
}

/// Opt-in async event stream from `interactor.ui_event_stream()`.
/// Single-subscriber, latest-wins. `next()` resolves to `None` when the
/// interactor is dropped or a new subscription replaces this one.
pub struct CopierPnlWsUiEventStream;
impl CopierPnlWsUiEventStream {
    pub async fn next(&self) -> Option<CopierPnlWsUiEvent> { todo!() }
}

pub struct CopierPositionHistoryInteractor;
impl CopierPositionHistoryInteractor {
    /// Append the next history page (called on scroll-to-bottom).
    pub fn load_next_page(&self, page_size: i32) { todo!() }
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
    /// Reset state and load the first history page (called on refresh / OnResume).
    pub fn reset_and_load_first_page(&self, page_size: i32) { todo!() }
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
    pub fn set_ui_event_observer(&self, observer: Arc<dyn CopierPositionHistoryUiEventObserver>) { todo!() }
    /// Start the interactor lifecycle.
    /// 
    /// Calls the user's `InteractorLifecycle::start()` implementation.
    /// Override that method to add custom startup behavior (e.g., initial data loads).
    pub fn start(&self) { todo!() }
    /// Get the current state (UniFFI export).
    /// 
    /// Returns the External type (UniFFI-compatible).
    pub fn state(&self) -> CopierPositionHistoryState { todo!() }
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
    pub fn ui_event_stream(&self) -> Arc<CopierPositionHistoryUiEventStream> { todo!() }
}

/// Deprecated UniFFI callback interface for observing UI events. Prefer the cycle-free `CopierPositionHistoryUiEventStream` returned by `interactor.ui_event_stream()`.
pub struct CopierPositionHistoryUiEventObserver;
impl CopierPositionHistoryUiEventObserver {
    pub fn on_ui_event(&self, event: CopierPositionHistoryUiEvent) { todo!() }
}

/// Opt-in async event stream from `interactor.ui_event_stream()`.
/// Single-subscriber, latest-wins. `next()` resolves to `None` when the
/// interactor is dropped or a new subscription replaces this one.
pub struct CopierPositionHistoryUiEventStream;
impl CopierPositionHistoryUiEventStream {
    pub async fn next(&self) -> Option<CopierPositionHistoryUiEvent> { todo!() }
}

/// Copier Positions Interactor. See module docs for the relationship with
/// `my_lead_trader_details::MyLeadTraderDetailsInteractor`.
pub struct CopierPositionsInteractor;
impl CopierPositionsInteractor {
    /// Close a custom-copy position.
    /// 
    /// Same `CloseSucceeded` / `CloseFailed` contract as `close_smart_position`,
    /// including the non-cancelling guarantee for concurrent closes. No reload
    /// on success — the WS avail=0 frame removes the position (native parity).
    pub fn close_custom_position(&self, trade_item_id: String, detail_ids: Option<String>, role_type: String, id_type: String, inst_type: String, px: Option<String>, copy_mode: MyLeadTraderCopyMode) { todo!() }
    /// Close a smart-copy position.
    /// 
    /// On success, emits `CloseSucceeded`; on failure, emits
    /// `CloseFailed { message }`. Neither outcome reloads the list — the closed
    /// position is removed via the WS avail=0 frame (native parity).
    /// 
    /// Closes are irreversible financial commands: every accepted call runs to
    /// completion and surfaces its own terminal event. Concurrent closes
    /// (several cells, or repeated taps) never cancel each other.
    pub fn close_smart_position(&self, inst_id: String, pos_side: String, mgn_mode: String, copy_mode: MyLeadTraderCopyMode) { todo!() }
    /// Load the ongoing positions list.
    /// 
    /// `copy_mode` selects the endpoint:
    /// - `SmartCopy` → `/copier/positions`
    /// - `FixedCost | RatioCopy` → `/copier/position-summary` or `/copier/position-detail`
    /// based on `state.custom_ongoing_selected_tab`
    pub fn ongoing_load(&self, copy_mode: MyLeadTraderCopyMode) { todo!() }
    /// Temporarily stop the positions WS, remembering whether it was running.
    /// 
    /// Holder calls this before any DETAIL action (e.g. history pagination) that
    /// should suppress positions-WS pushes. State after:
    /// `paused_was_running = (was_running ? true : false)` and `is_positions_ws_running = false`.
    pub fn pause_ws(&self) { todo!() }
    /// Fetch compensation-ticket (newbie reward voucher) flags for the given position
    /// trade item IDs and merge the result into `ongoing_*_positions`.
    /// 
    /// Holder invokes this in response to [`CopierPositionsUiEvent::NewlyAddedItems`].
    /// The fetch hits `/priapi/v5/ecotrade/public/query-item-flag`; on success
    /// the matching positions in state get their `compensation_ticket_*` fields
    /// populated and a [`CopierPositionsUiEvent::PositionsWsUpdated`] event
    /// fires so the Holder re-emits the list. On failure this is a silent
    /// no-op — the coupon banner is non-critical UI and a missing flag does
    /// not break the position list.
    /// 
    /// Empty input is a no-op (no API call). Mirrors native
    /// `CurrentPositionViewModel.fetchOrderCouponState` which short-circuits
    /// when `newlyAdded.isEmpty()`.
    pub fn refresh_coupon_flags(&self, item_ids: Vec<String>) { todo!() }
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
    /// Resume the positions WS if it was running before `pause_ws`.
    /// 
    /// Reads `state.paused_was_running` and starts the WS again iff true.
    /// Holder is responsible for skipping this call if `follow_type` became
    /// Unfollow during the pause window.
    pub fn resume_ws(&self) { todo!() }
    /// Select the Custom-mode ongoing positions tab.
    /// 
    /// Mutates `state.custom_ongoing_selected_tab`. Caller should usually
    /// follow with `ongoing_load(copy_mode)` to fetch the corresponding endpoint.
    pub fn set_custom_ongoing_tab(&self, tab: MyLeadTraderCustomOngoingTab) { todo!() }
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
    pub fn set_ui_event_observer(&self, observer: Arc<dyn CopierPositionsUiEventObserver>) { todo!() }
    /// Start the interactor lifecycle.
    /// 
    /// Calls the user's `InteractorLifecycle::start()` implementation.
    /// Override that method to add custom startup behavior (e.g., initial data loads).
    pub fn start(&self) { todo!() }
    /// Start the positions WebSocket if `follow_type` is not Unfollow.
    /// 
    /// If `follow_type.is_unfollow()`, any running WS is stopped and this is a
    /// no-op otherwise. When following, a fresh `PositionsWsSource` is created
    /// based on `copy_mode` + the interactor's current `custom_ongoing_selected_tab`,
    /// and `do_start_positions_ws` is spawned to actually connect.
    pub fn start_ws_if_following(&self, copy_mode: MyLeadTraderCopyMode, follow_type: MyLeadTraderFollowType, trading_mode: CopierPnlTradingMode) { todo!() }
    /// Get the current state (UniFFI export).
    /// 
    /// Returns the External type (UniFFI-compatible).
    pub fn state(&self) -> CopierPositionsState { todo!() }
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
    pub fn ui_event_stream(&self) -> Arc<CopierPositionsUiEventStream> { todo!() }
}

/// Deprecated UniFFI callback interface for observing UI events. Prefer the cycle-free `CopierPositionsUiEventStream` returned by `interactor.ui_event_stream()`.
pub struct CopierPositionsUiEventObserver;
impl CopierPositionsUiEventObserver {
    pub fn on_ui_event(&self, event: CopierPositionsUiEvent) { todo!() }
}

/// Opt-in async event stream from `interactor.ui_event_stream()`.
/// Single-subscriber, latest-wins. `next()` resolves to `None` when the
/// interactor is dropped or a new subscription replaces this one.
pub struct CopierPositionsUiEventStream;
impl CopierPositionsUiEventStream {
    pub async fn next(&self) -> Option<CopierPositionsUiEvent> { todo!() }
}

pub struct CopyContractSettingsInteractor;
impl CopyContractSettingsInteractor {
    /// Write back a batch-edit result (native `refreshLeverageList`,
    /// `CopyTradingSettingContractViewModel.kt:294-306`): set every successful
    /// instId's self net/long/short lever to `lever` in the current mode's cache,
    /// then re-filter to `instruments` + recompute risk + re-publish. The batch
    /// network (`fetchSpecialLever` + `getSuggestedInstrument`) stays native and
    /// feeds its result (`success_inst_ids` + `lever`) here.
    pub fn apply_batch_lever(&self, success_inst_ids: Vec<String>, lever: String, instruments: Vec<ContractInstrumentSelection>) { todo!() }
    /// Load the trader-vs-self contrast leverage for `margin_mode`.
    /// 
    /// `instruments` is the full user instrument selection (instId + name +
    /// selected), supplied by native from `InstrumentsManager.getInstruments()`.
    /// The query uses every instId; the published list is filtered to selected.
    pub fn load(&self, margin_mode: String, instruments: Vec<ContractInstrumentSelection>) { todo!() }
    /// Batch-edit pre-check (native `Action.OnBulkEdit` → `fetchSpecialLeverResult`,
    /// `CopyTradingSettingContractViewModel.kt:318-357`): fetch the special-lever set for the
    /// selected instIds, suspend-gate via `ensure_swap_biz_loaded`, then for each returned
    /// instId resolve `suggested_trade_symbol` (membership) to compute the no-pass set. Emits
    /// `LeverageSuccess` (none restricted) / `LeverageFailed` (all restricted) /
    /// `LeveragePartiallySuccess` (some) / `BulkEditError` (fetch failed). `selected_inst_ids`
    /// is the current selected list (native `selectedLeverage` instIds).
    pub fn on_bulk_edit(&self, margin_mode: String, selected_inst_ids: Vec<String>) { todo!() }
    /// Resolve the inputs for the adjust-leverage dialog (native `Action.OnChangeLeverage`,
    /// `CopyTradingSettingContractViewModel.kt:373-407`): suspend-gate via core-trading
    /// (`checkInstrumentSuspend` → `ensure_swap_biz_loaded`), then fetch the ticker last price
    /// (`getMarketTickerByInstId().last` → `market_last_price`). On success emits
    /// `ShowAdjustLeverageDialog` (native then shows the trade-core dialog — UI stays native);
    /// if biz isn't ready or the ticker is unavailable, emits `AdjustLeverageDismissed`.
    /// Both core-trading reads go through `trade_core`, with no config gate in this crate:
    /// native decides whether the Rust interactor runs at all.
    pub fn on_change_leverage(&self, item: ContractLeverageItem, pos_side: String, leverage: String) { todo!() }
    /// Toggle expand/collapse of the leverage list (native `Action.OnExpand`,
    /// `CopyTradingSettingContractViewModel.kt:310-318`). Pure list-state op; the
    /// collapse-to-3 display rule stays on the native side.
    pub fn on_expand(&self, is_expand: bool) { todo!() }
    /// Write back a single row after its leverage changed (native
    /// `Action.OnLeverageChanged`, `CopyTradingSettingContractViewModel.kt:359-372`):
    /// update the cached + displayed row and recompute the risk warning. The
    /// trade-core fetch that produced the new leverage stays native; only the
    /// list-state write-back is here.
    pub fn on_leverage_changed(&self, item: ContractLeverageItem) { todo!() }
    /// Re-filter the current mode's cached list to `instruments` and re-publish
    /// (native `Action.Filter` / `Action.EnterSearchMode`, :386-400 — they re-run
    /// `updateLeverageList` from the cache). Fuzzy search itself stays native (A2).
    pub fn refilter(&self, instruments: Vec<ContractInstrumentSelection>) { todo!() }
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
    /// Selected leverage rows across BOTH cached margin modes, in native order
    /// (cross then isolated). Native `checkIsSameMgnNumber`
    /// (`CopyTradingSettingContractViewModel.kt:410-447`) iterates
    /// `crossLeverageList` then `isolateLeverageList` (both filtered to selected)
    /// and applies its `getLever` formatting + all-equal comparison. The raw rows
    /// come from here; the formatting/comparison stays native (from_api Rule 1).
    pub fn selected_items_all_modes(&self, instruments: Vec<ContractInstrumentSelection>) -> Vec<ContractLeverageItem> { todo!() }
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
    pub fn set_ui_event_observer(&self, observer: Arc<dyn CopyContractSettingsUiEventObserver>) { todo!() }
    /// Start the interactor lifecycle.
    /// 
    /// Calls the user's `InteractorLifecycle::start()` implementation.
    /// Override that method to add custom startup behavior (e.g., initial data loads).
    pub fn start(&self) { todo!() }
    /// Get the current state (UniFFI export).
    /// 
    /// Returns the External type (UniFFI-compatible).
    pub fn state(&self) -> CopyContractSettingsState { todo!() }
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
    pub fn ui_event_stream(&self) -> Arc<CopyContractSettingsUiEventStream> { todo!() }
}

/// Deprecated UniFFI callback interface for observing UI events. Prefer the cycle-free `CopyContractSettingsUiEventStream` returned by `interactor.ui_event_stream()`.
pub struct CopyContractSettingsUiEventObserver;
impl CopyContractSettingsUiEventObserver {
    pub fn on_ui_event(&self, event: CopyContractSettingsUiEvent) { todo!() }
}

/// Opt-in async event stream from `interactor.ui_event_stream()`.
/// Single-subscriber, latest-wins. `next()` resolves to `None` when the
/// interactor is dropped or a new subscription replaces this one.
pub struct CopyContractSettingsUiEventStream;
impl CopyContractSettingsUiEventStream {
    pub async fn next(&self) -> Option<CopyContractSettingsUiEvent> { todo!() }
}

pub struct CopySettingsSpotInteractor;
impl CopySettingsSpotInteractor {
    pub fn amend_settings(&self, input: AmendSettingsInput) { todo!() }
    pub fn cancel_copy_trade(&self, input: CancelCopyTradeInput) { todo!() }
    pub fn check_cancel_copy_trade(&self, trader_unique_name: String) { todo!() }
    pub fn check_position_value(&self, input: PositionValueCheckInput) { todo!() }
    /// Fetch cancel retention popup pages. See Swap interactor doc for details.
    pub fn fetch_cancel_popup(&self, trader_unique_name: String) { todo!() }
    /// Fetch recommended copy ratio for a given trader + investment principal.
    /// `trader_unique_name` is passed explicitly (not read from state) so the
    /// caller can drive the request even from contexts where Rust state has
    /// not been bootstrapped via `load_trader_data` (e.g. Custom Activity flow,
    /// where `maxAvailable()` triggers via `refreshAvailableAmount` which does
    /// not populate state.trader_unique_name). Mirrors native parity — native
    /// passes `uniqueName` from `settingsBean.traderUniqueName` directly.
    pub fn fetch_recommend_ratio(&self, principal: String, trader_unique_name: String) { todo!() }
    pub fn fetch_usable_coupon(&self) { todo!() }
    pub fn fetch_using_coupon(&self) { todo!() }
    pub fn get_trade_type(&self) -> CopySettingsTradeType { todo!() }
    pub fn load_reducible_amount(&self, copy_rel_id: String, inst_type: Option<String>) { todo!() }
    pub fn load_smart_settings(&self, trader_unique_name: String, copy_rel_id: Option<String>, inst_type: Option<String>) { todo!() }
    pub fn load_trader_data(&self, input: LoadTraderDataInput) { todo!() }
    pub fn perform_pre_check(&self, copy_mode: CopySettingsCopyMode) { todo!() }
    pub fn refresh(&self) { todo!() }
    /// Narrow refresh — fetches only the available balance and updates
    /// `state.available_amount`. Mirrors native
    /// `CopyTradingSettingHomeViewModel.maxAvailable()` Custom branch which
    /// calls `copyTradeRepo.fetchAvailableBalance(...)` only — it does NOT
    /// refetch trader-setting or instruments (those come from the cached
    /// `traderSettingsBean` passed via Activity params).
    pub fn refresh_available_amount(&self) { todo!() }
    pub fn refresh_smart_max_available(&self) { todo!() }
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
    pub fn set_ui_event_observer(&self, observer: Arc<dyn CopySettingsSpotUiEventObserver>) { todo!() }
    /// Start the interactor lifecycle.
    /// 
    /// Calls the user's `InteractorLifecycle::start()` implementation.
    /// Override that method to add custom startup behavior (e.g., initial data loads).
    pub fn start(&self) { todo!() }
    /// Get the current state (UniFFI export).
    /// 
    /// Returns the External type (UniFFI-compatible).
    pub fn state(&self) -> CopySettingsState { todo!() }
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
    pub fn submit_first_settings(&self, input: SubmitFirstSettingsInput) { todo!() }
    pub fn transfer_smart_amount(&self, transfer_type: SmartCopyTransferType, transfer_amount: String, inst_type: Option<String>) { todo!() }
    /// Opt-in async UI event stream (cycle-free). Single-subscriber.
    pub fn ui_event_stream(&self) -> Arc<CopySettingsSpotUiEventStream> { todo!() }
    pub fn update_copy_mode(&self, mode: CopySettingsCopyMode) { todo!() }
    pub fn update_copy_ratio(&self, ratio: String) { todo!() }
    pub fn update_follow_amount(&self, amount: String) { todo!() }
    pub fn update_max_follow_amount(&self, amount: String) { todo!() }
    pub fn update_sl_ratio(&self, ratio: Option<String>) { todo!() }
    pub fn update_tp_ratio(&self, ratio: Option<String>) { todo!() }
    pub fn update_usable_coupon(&self, coupon: Option<CouponCenterItemResponse>) { todo!() }
}

/// Deprecated UniFFI callback interface for observing UI events. Prefer the cycle-free `CopySettingsSpotUiEventStream` returned by `interactor.ui_event_stream()`.
pub struct CopySettingsSpotUiEventObserver;
impl CopySettingsSpotUiEventObserver {
    pub fn on_ui_event(&self, event: CopySettingsSpotUiEvent) { todo!() }
}

/// Opt-in async event stream from `interactor.ui_event_stream()`.
/// Single-subscriber, latest-wins. `next()` resolves to `None` when the
/// interactor is dropped or a new subscription replaces this one.
pub struct CopySettingsSpotUiEventStream;
impl CopySettingsSpotUiEventStream {
    pub async fn next(&self) -> Option<CopySettingsSpotUiEvent> { todo!() }
}

/// Copy Settings Swap Interactor
/// 
/// Manages the copy trading settings for Contract/Futures (SWAP).
pub struct CopySettingsSwapInteractor;
impl CopySettingsSwapInteractor {
    /// Amend existing copy trading settings.
    pub fn amend_settings(&self, input: AmendSettingsInput) { todo!() }
    /// Cancel copy trade.
    pub fn cancel_copy_trade(&self, input: CancelCopyTradeInput) { todo!() }
    /// Check if user has open positions before canceling copy trade.
    pub fn check_cancel_copy_trade(&self, trader_unique_name: String) { todo!() }
    /// Check position value before amending settings.
    pub fn check_position_value(&self, input: PositionValueCheckInput) { todo!() }
    /// Fetch cancel retention popup pages. Native parity:
    /// `CopyTradingSettingHomeViewModel.handleCopyTraderRetentionFlow` →
    /// `getCancelCopyingPopupUseCase.invoke(...)`. Emits `CancelPopupLoaded`
    /// with the popup payload on success (caller decides whether to show
    /// first/second popup or fall through to precheck), or
    /// `CancelPopupLoadError` on failure (caller mirrors native onFailure by
    /// falling through to precheck).
    pub fn fetch_cancel_popup(&self, trader_unique_name: String) { todo!() }
    /// Fetch recommended ratio. Native parity: pass `trader_unique_name`
    /// explicitly instead of reading from Rust state, so Custom Activity flow
    /// (which doesn't bootstrap state via `load_trader_data`) still produces
    /// a request with `?uniqueName=...`.
    pub fn fetch_recommend_ratio(&self, principal: String, trader_unique_name: String) { todo!() }
    /// Fetch usable coupon for first-copy.
    pub fn fetch_usable_coupon(&self) { todo!() }
    /// Fetch coupon currently being used.
    pub fn fetch_using_coupon(&self) { todo!() }
    /// Get trade type (always Swap for this interactor).
    pub fn get_trade_type(&self) -> CopySettingsTradeType { todo!() }
    /// Load smart copy reducible amount.
    pub fn load_reducible_amount(&self, copy_rel_id: String, inst_type: Option<String>) { todo!() }
    /// Load smart copy settings.
    pub fn load_smart_settings(&self, trader_unique_name: String, copy_rel_id: Option<String>, inst_type: Option<String>) { todo!() }
    /// Load trader data for copy settings.
    pub fn load_trader_data(&self, input: LoadTraderDataInput) { todo!() }
    /// Perform copy permission pre-check.
    pub fn perform_pre_check(&self, copy_mode: CopySettingsCopyMode) { todo!() }
    /// Refresh data (re-load with current trader).
    pub fn refresh(&self) { todo!() }
    /// Narrow refresh — fetches only the available balance and updates
    /// `state.available_amount`. Mirrors native
    /// `CopyTradingSettingHomeViewModel.maxAvailable()` Custom branch which
    /// calls `copyTradeRepo.fetchAvailableBalance(...)` only — it does NOT
    /// refetch trader-setting or instruments (those come from the cached
    /// `traderSettingsBean` passed via Activity params).
    pub fn refresh_available_amount(&self) { todo!() }
    /// Refresh smart max available amount.
    pub fn refresh_smart_max_available(&self) { todo!() }
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
    pub fn set_ui_event_observer(&self, observer: Arc<dyn CopySettingsSwapUiEventObserver>) { todo!() }
    /// Start the interactor lifecycle.
    /// 
    /// Calls the user's `InteractorLifecycle::start()` implementation.
    /// Override that method to add custom startup behavior (e.g., initial data loads).
    pub fn start(&self) { todo!() }
    /// Start custom settings flow from quick copy.
    pub fn start_custom_copy(&self) { todo!() }
    /// Start quick copy flow.
    pub fn start_quick_copy(&self, input: LoadTraderDataInput) { todo!() }
    /// Get the current state (UniFFI export).
    /// 
    /// Returns the External type (UniFFI-compatible).
    pub fn state(&self) -> CopySettingsState { todo!() }
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
    /// Submit first copy trading settings (create relation).
    pub fn submit_first_settings(&self, input: SubmitFirstSettingsInput) { todo!() }
    /// Transfer smart copy amount.
    pub fn transfer_smart_amount(&self, transfer_type: SmartCopyTransferType, transfer_amount: String, inst_type: Option<String>) { todo!() }
    /// Opt-in async UI event stream (cycle-free). Single-subscriber.
    pub fn ui_event_stream(&self) -> Arc<CopySettingsSwapUiEventStream> { todo!() }
    /// Update copy mode in state.
    pub fn update_copy_mode(&self, mode: CopySettingsCopyMode) { todo!() }
    /// Update follow amount in state.
    pub fn update_follow_amount(&self, amount: String) { todo!() }
    /// Update initial amount in state (smart copy).
    pub fn update_initial_amount(&self, amount: Option<String>) { todo!() }
    /// Update margin mode in state (Swap only).
    pub fn update_margin_mode(&self, mode: String) { todo!() }
    /// Update max follow amount in state.
    pub fn update_max_follow_amount(&self, amount: String) { todo!() }
    /// Update remain margin deal in state (smart copy).
    pub fn update_remain_margin_deal(&self, deal: Option<String>) { todo!() }
    /// Update replication required in state (smart copy).
    pub fn update_replication_required(&self, is_required: bool) { todo!() }
    /// Update stop loss ratio in state.
    pub fn update_sl_ratio(&self, ratio: Option<String>) { todo!() }
    /// Update stop loss amount in state (smart copy).
    pub fn update_stop_loss_amount(&self, amount: Option<String>) { todo!() }
    /// Update take profit ratio in state.
    pub fn update_tp_ratio(&self, ratio: Option<String>) { todo!() }
    /// Update the usable coupon in state.
    pub fn update_usable_coupon(&self, coupon: Option<CouponCenterItemResponse>) { todo!() }
}

/// Deprecated UniFFI callback interface for observing UI events. Prefer the cycle-free `CopySettingsSwapUiEventStream` returned by `interactor.ui_event_stream()`.
pub struct CopySettingsSwapUiEventObserver;
impl CopySettingsSwapUiEventObserver {
    pub fn on_ui_event(&self, event: CopySettingsSwapUiEvent) { todo!() }
}

/// Opt-in async event stream from `interactor.ui_event_stream()`.
/// Single-subscriber, latest-wins. `next()` resolves to `None` when the
/// interactor is dropped or a new subscription replaces this one.
pub struct CopySettingsSwapUiEventStream;
impl CopySettingsSwapUiEventStream {
    pub async fn next(&self) -> Option<CopySettingsSwapUiEvent> { todo!() }
}

/// Copy Trade Retained Interactor (start-copy gatekeeper).
pub struct CopyTradeRetainedInteractor;
impl CopyTradeRetainedInteractor {
    /// Start the copy pre-check flow (= native `initStartCopyTradeCheck`).
    pub fn init_start_copy_check(&self, input: StartCopyCheckInput) { todo!() }
    /// Custom-copy path (= native `onCustomCopyClicked`): preCheck(Fixed) +
    /// fetch trader settings + navigate, reusing the cached params.
    pub fn on_custom_copy_clicked(&self) { todo!() }
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
    pub fn set_ui_event_observer(&self, observer: Arc<dyn CopyTradeRetainedUiEventObserver>) { todo!() }
    /// Start the interactor lifecycle.
    /// 
    /// Calls the user's `InteractorLifecycle::start()` implementation.
    /// Override that method to add custom startup behavior (e.g., initial data loads).
    pub fn start(&self) { todo!() }
    /// Get the current state (UniFFI export).
    /// 
    /// Returns the External type (UniFFI-compatible).
    pub fn state(&self) -> CopyTradeRetainedState { todo!() }
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
    /// Re-enter after Kotlin completed `AccountSwitchingUseCase`
    /// (= native `switchAccountLevelWhileStartCopying` tail). Routes by the
    /// persisted `current_copy_mode`: Smart(Position) → re-run the check
    /// (show_toast); else → custom-copy.
    pub fn switch_account_reenter(&self, input: StartCopyCheckInput) { todo!() }
    /// Opt-in async UI event stream (cycle-free). Single-subscriber.
    pub fn ui_event_stream(&self) -> Arc<CopyTradeRetainedUiEventStream> { todo!() }
}

/// Deprecated UniFFI callback interface for observing UI events. Prefer the cycle-free `CopyTradeRetainedUiEventStream` returned by `interactor.ui_event_stream()`.
pub struct CopyTradeRetainedUiEventObserver;
impl CopyTradeRetainedUiEventObserver {
    pub fn on_ui_event(&self, event: CopyTradeRetainedUiEvent) { todo!() }
}

/// Opt-in async event stream from `interactor.ui_event_stream()`.
/// Single-subscriber, latest-wins. `next()` resolves to `None` when the
/// interactor is dropped or a new subscription replaces this one.
pub struct CopyTradeRetainedUiEventStream;
impl CopyTradeRetainedUiEventStream {
    pub async fn next(&self) -> Option<CopyTradeRetainedUiEvent> { todo!() }
}

/// Coupon Center Interactor — coordinates coupon list load + selected-item
/// management for the loss-protection radio-button screen.
pub struct CouponCenterInteractor;
impl CouponCenterInteractor {
    /// Fetch the coupon center list from the backend.
    /// 
    /// Mirrors native `getCouponCenterList(silentLoad: Boolean)`
    /// (`CouponCenterViewModel.kt:49-78`).
    /// 
    /// * `biz_type` — optional business type filter (e.g. "SWAP" / "SPOT").
    /// * `channel` — channel value, must match a native `CouponCenterSource`
    /// value ("0" Community or "1" CopySetting). On CopySetting channel the
    /// Interactor runs the radio-button matching loop against the current
    /// `current_coupon` (line-by-line port of the native VM line 60-72).
    /// * `silent_load` — when true, do NOT transition the load state to
    /// `Loading` (native VM line 52-58 preserves prior data on background
    /// refresh).
    pub fn load_coupon_center_list(&self, biz_type: Option<String>, channel: String, silent_load: bool) { todo!() }
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
    pub fn set_ui_event_observer(&self, observer: Arc<dyn CouponCenterUiEventObserver>) { todo!() }
    /// Start the interactor lifecycle.
    /// 
    /// Calls the user's `InteractorLifecycle::start()` implementation.
    /// Override that method to add custom startup behavior (e.g., initial data loads).
    pub fn start(&self) { todo!() }
    /// Get the current state (UniFFI export).
    /// 
    /// Returns the External type (UniFFI-compatible).
    pub fn state(&self) -> CouponCenterState { todo!() }
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
    pub fn ui_event_stream(&self) -> Arc<CouponCenterUiEventStream> { todo!() }
    /// Toggle the selected flag for one coupon and rebroadcast the list.
    /// 
    /// Mirrors native `updateCouponItemSelected(couponItem)`
    /// (`CouponCenterViewModel.kt:80-91`) — radio-button semantics: setting
    /// one item's `isSelected` deselects every other item, and the
    /// `current_coupon` flow becomes `Some(item)` if the toggle is on, or
    /// `None` if off.
    pub fn update_coupon_item_selected(&self, coupon_id: String, selected: bool) { todo!() }
    /// Set the current loss-protection coupon directly.
    /// 
    /// Mirrors native `updateCurrentCoupon(item: CouponCenterItemResponse?)`
    /// (`CouponCenterViewModel.kt:93-95`). Used at VM init from
    /// `savedStateHandle[CURRENT_COUPON]` (native line 41-46).
    pub fn update_current_coupon(&self, item: Option<CouponCenterItemResponse>) { todo!() }
}

/// Deprecated UniFFI callback interface for observing UI events. Prefer the cycle-free `CouponCenterUiEventStream` returned by `interactor.ui_event_stream()`.
pub struct CouponCenterUiEventObserver;
impl CouponCenterUiEventObserver {
    pub fn on_ui_event(&self, event: CouponCenterUiEvent) { todo!() }
}

/// Opt-in async event stream from `interactor.ui_event_stream()`.
/// Single-subscriber, latest-wins. `next()` resolves to `None` when the
/// interactor is dropped or a new subscription replaces this one.
pub struct CouponCenterUiEventStream;
impl CouponCenterUiEventStream {
    pub async fn next(&self) -> Option<CouponCenterUiEvent> { todo!() }
}

/// EcoBatchLever Interactor — stateless Facade owning both request centers
/// (range + batch-set). No state, no observers, no lifecycle.
pub struct EcoBatchLeverInteractor;
impl EcoBatchLeverInteractor {
    /// Batch-set leverage for the given instruments.
    /// Mirrors native `EcoBatchLeverViewModel.batchLeverage(lever)`.
    /// 
    /// * `inst_ids` - selected instrument IDs
    /// * `mgn_mode` - margin mode ("isolated" / "cross")
    /// * `lever` - target leverage value (e.g. "4.00")
    /// 
    /// Maps the DTO via [`CopierBatchSetLeverageResult::from_dto`]; a `None`
    /// response resolves to [`CopierBatchSetLeverageResult::empty`] (matching
    /// `copier_special_lever_use_case.batch_set_leverage`).
    pub fn batch_set_leverage(&self, inst_ids: Vec<String>, mgn_mode: String, lever: String) -> Arc<OKRustFutureCopierBatchSetLeverageResultWithOKSTFFIError> { todo!() }
    /// Fetch the allowed leverage range (min/max + preset percentages).
    /// Mirrors native `EcoBatchLeverViewModel.getBatchLeverRange()`.
    /// 
    /// Maps the DTO via [`LeverIntervalResult::from_dto`]; a `None` response
    /// resolves to [`LeverIntervalResult::empty`] (matching
    /// `copier_special_lever_use_case.fetch_lever_interval`).
    pub fn fetch_lever_range(&self) -> Arc<OKRustFutureLeverIntervalResultWithOKSTFFIError> { todo!() }
}

pub struct EcoHoldPositionInteractor;
impl EcoHoldPositionInteractor {
    /// Load position-history-scatter for the given period code
    /// (`"7D"` or `"30D"`). Mirrors native `fetchPositionHistory()`.
    pub fn load(&self, period_code: String) { todo!() }
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
    pub fn set_ui_event_observer(&self, observer: Arc<dyn EcoHoldPositionUiEventObserver>) { todo!() }
    /// Start the interactor lifecycle.
    /// 
    /// Calls the user's `InteractorLifecycle::start()` implementation.
    /// Override that method to add custom startup behavior (e.g., initial data loads).
    pub fn start(&self) { todo!() }
    /// Get the current state (UniFFI export).
    /// 
    /// Returns the External type (UniFFI-compatible).
    pub fn state(&self) -> EcoHoldPositionState { todo!() }
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
    pub fn ui_event_stream(&self) -> Arc<EcoHoldPositionUiEventStream> { todo!() }
}

/// Deprecated UniFFI callback interface for observing UI events. Prefer the cycle-free `EcoHoldPositionUiEventStream` returned by `interactor.ui_event_stream()`.
pub struct EcoHoldPositionUiEventObserver;
impl EcoHoldPositionUiEventObserver {
    pub fn on_ui_event(&self, event: EcoHoldPositionUiEvent) { todo!() }
}

/// Opt-in async event stream from `interactor.ui_event_stream()`.
/// Single-subscriber, latest-wins. `next()` resolves to `None` when the
/// interactor is dropped or a new subscription replaces this one.
pub struct EcoHoldPositionUiEventStream;
impl EcoHoldPositionUiEventStream {
    pub async fn next(&self) -> Option<EcoHoldPositionUiEvent> { todo!() }
}

/// Eco Pnl Weekly Interactor
pub struct EcoPnlWeeklyInteractor;
impl EcoPnlWeeklyInteractor {
    /// Load weekly PnL chart data.
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
    pub fn set_ui_event_observer(&self, observer: Arc<dyn EcoPnlWeeklyUiEventObserver>) { todo!() }
    /// Start the interactor lifecycle.
    /// 
    /// Calls the user's `InteractorLifecycle::start()` implementation.
    /// Override that method to add custom startup behavior (e.g., initial data loads).
    pub fn start(&self) { todo!() }
    /// Get the current state (UniFFI export).
    /// 
    /// Returns the External type (UniFFI-compatible).
    pub fn state(&self) -> EcoPnlWeeklyState { todo!() }
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
    pub fn ui_event_stream(&self) -> Arc<EcoPnlWeeklyUiEventStream> { todo!() }
}

/// Deprecated UniFFI callback interface for observing UI events. Prefer the cycle-free `EcoPnlWeeklyUiEventStream` returned by `interactor.ui_event_stream()`.
pub struct EcoPnlWeeklyUiEventObserver;
impl EcoPnlWeeklyUiEventObserver {
    pub fn on_ui_event(&self, event: EcoPnlWeeklyUiEvent) { todo!() }
}

/// Opt-in async event stream from `interactor.ui_event_stream()`.
/// Single-subscriber, latest-wins. `next()` resolves to `None` when the
/// interactor is dropped or a new subscription replaces this one.
pub struct EcoPnlWeeklyUiEventStream;
impl EcoPnlWeeklyUiEventStream {
    pub async fn next(&self) -> Option<EcoPnlWeeklyUiEvent> { todo!() }
}

/// Follower List Interactor
/// 
/// Manages a trader follower list page. Routes to account or public endpoint
/// based on whether `initial_unique_name` is non-empty.
pub struct FollowerListInteractor;
impl FollowerListInteractor {
    /// Change sort order (triggers refresh).
    pub fn change_sort(&self, sort_type: FollowerListSortType) { todo!() }
    /// Get current trade type (UniFFI export).
    pub fn get_trade_type(&self) -> FollowerListTradeType { todo!() }
    /// Load initial data (first page).
    pub fn load_data(&self) { todo!() }
    /// Load more data (next page from server).
    pub fn load_more(&self) { todo!() }
    /// Refresh data (pull-to-refresh).
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
    pub fn set_ui_event_observer(&self, observer: Arc<dyn FollowerListUiEventObserver>) { todo!() }
    /// Start the interactor lifecycle.
    /// 
    /// Calls the user's `InteractorLifecycle::start()` implementation.
    /// Override that method to add custom startup behavior (e.g., initial data loads).
    pub fn start(&self) { todo!() }
    /// Get the current state (UniFFI export).
    /// 
    /// Returns the External type (UniFFI-compatible).
    pub fn state(&self) -> FollowerListState { todo!() }
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
    pub fn ui_event_stream(&self) -> Arc<FollowerListUiEventStream> { todo!() }
}

/// Deprecated UniFFI callback interface for observing UI events. Prefer the cycle-free `FollowerListUiEventStream` returned by `interactor.ui_event_stream()`.
pub struct FollowerListUiEventObserver;
impl FollowerListUiEventObserver {
    pub fn on_ui_event(&self, event: FollowerListUiEvent) { todo!() }
}

/// Opt-in async event stream from `interactor.ui_event_stream()`.
/// Single-subscriber, latest-wins. `next()` resolves to `None` when the
/// interactor is dropped or a new subscription replaces this one.
pub struct FollowerListUiEventStream;
impl FollowerListUiEventStream {
    pub async fn next(&self) -> Option<FollowerListUiEvent> { todo!() }
}

/// Lead Trade Crypto Interactor
/// 
/// Manages the trading pair selection screen for lead traders.
/// 
/// # Auto-Generated by `#[interactor::interactor]`:
/// 
/// - `state()` - Get current state
/// - `set_state_observer(observer)` - Set state change observer
/// - `remove_state_observer()` - Remove state observer
/// - `set_ui_event_observer(observer)` - Set UI event observer
/// - `remove_ui_event_observer()` - Remove UI event observer
/// - `ui_event_handle()` - Get weak handle for emitting events
/// - `effects()` - Get effects manager
/// - `start()` / `stop()` - Lifecycle management
/// - `set_debug(enabled)` - Enable/disable debug logging
pub struct LeadTradeCryptoInteractor;
impl LeadTradeCryptoInteractor {
    /// Coin icon URL for `inst_id`, resolved from core-trading's coin store. Replaces the
    /// Kotlin-side `SocialTradeManager.getCoinUrl` read so the Rust leg never touches the
    /// legacy native kernel. `""` on a store miss — native parity with `OkTrade.getCoinUrl`
    /// (the platform renders a placeholder).
    /// 
    /// Safe to read synchronously right after `DataLoaded`: both `init_with_data` and
    /// `load_data` await the store load before they publish state, precisely because the
    /// platform resolves every icon off that one emission and never re-renders. This mirrors
    /// retail-trading, whose Vo mappers take an already-loaded `BizManager` rather than
    /// reading the global store mid-map.
    /// 
    /// Not adopted from OKMarket: its bridge derives a CDN-convention URL when the store
    /// misses (`MarketTradingCoreBridge.getCoinUrl` -> `MarketConfig.getConfigIconUrl`).
    /// That is right for OKMarket because its own V1 native had that fallback; social's
    /// native (`OkTrade.getCoinUrl`) returns `""`, so adding one here would change behavior
    /// rather than replicate it.
    pub fn coin_url_by_inst_id(&self, inst_id: String) -> String { todo!() }
    /// Initialize state from external data (skip API load).
    /// Use when instrument data is already available from parent page.
    pub fn init_with_data(&self, instruments: Vec<LeadTradeCryptoItem>) { todo!() }
    /// Load all available trading pairs from API.
    pub fn load_data(&self) { todo!() }
    /// Refresh data (alias for load_data).
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
    /// DEPRECATED. Use `ui_event_stream()`; drop the returned stream
    /// wrapper to unsubscribe.
    pub fn remove_ui_event_observer(&self) { todo!() }
    /// Set the "Auto-include new crypto" toggle.
    pub fn set_auto_include_new(&self, is_auto_include: bool) { todo!() }
    /// Enable or disable debug logging.
    /// 
    /// In release builds this is a no-op, but the symbol must exist
    /// so that UniFFI bindings generated in debug mode continue to
    /// link successfully against the release library.
    pub fn set_debug(&self, _enabled: bool) { todo!() }
    /// Set selection state for a single item.
    pub fn set_item_selected(&self, inst_id: String, is_selected: bool) { todo!() }
    /// Set search query for client-side filtering.
    pub fn set_search_query(&self, query: String) { todo!() }
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
    pub fn set_ui_event_observer(&self, observer: Arc<dyn LeadTradeCryptoUiEventObserver>) { todo!() }
    /// Start the interactor lifecycle.
    /// 
    /// Calls the user's `InteractorLifecycle::start()` implementation.
    /// Override that method to add custom startup behavior (e.g., initial data loads).
    pub fn start(&self) { todo!() }
    /// Get the current state (UniFFI export).
    /// 
    /// Returns the External type (UniFFI-compatible).
    pub fn state(&self) -> LeadTradeCryptoState { todo!() }
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
    /// Submit the selected trading pairs.
    pub fn submit(&self) { todo!() }
    /// Toggle selection for a single item.
    pub fn toggle_item(&self, inst_id: String) { todo!() }
    /// Toggle select all / deselect all.
    pub fn toggle_select_all(&self, select_all: bool) { todo!() }
    /// Get current trade type (UniFFI export).
    pub fn trade_type(&self) -> LeadTradeCryptoTradeType { todo!() }
    /// Opt-in async UI event stream (cycle-free). Single-subscriber.
    pub fn ui_event_stream(&self) -> Arc<LeadTradeCryptoUiEventStream> { todo!() }
}

/// Deprecated UniFFI callback interface for observing UI events. Prefer the cycle-free `LeadTradeCryptoUiEventStream` returned by `interactor.ui_event_stream()`.
pub struct LeadTradeCryptoUiEventObserver;
impl LeadTradeCryptoUiEventObserver {
    pub fn on_ui_event(&self, event: LeadTradeCryptoUiEvent) { todo!() }
}

/// Opt-in async event stream from `interactor.ui_event_stream()`.
/// Single-subscriber, latest-wins. `next()` resolves to `None` when the
/// interactor is dropped or a new subscription replaces this one.
pub struct LeadTradeCryptoUiEventStream;
impl LeadTradeCryptoUiEventStream {
    pub async fn next(&self) -> Option<LeadTradeCryptoUiEvent> { todo!() }
}

/// Lead Trade History Interactor
/// 
/// Manages the history list for lead traders showing their own closed trades.
/// 
/// Note: Uses `/priapi/v5/ecotrade/trader/position-history` API which returns
/// the current logged-in trader's own history (no uniqueName needed).
pub struct LeadTradeHistoryInteractor;
impl LeadTradeHistoryInteractor {
    /// Load first page of history (resets pagination).
    pub fn load_first_page(&self) { todo!() }
    /// Load more items (next page).
    pub fn load_more(&self) { todo!() }
    /// Refresh data (alias for load_first_page).
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
    /// Change trade type and reload.
    pub fn set_trade_type(&self, trade_type: LeadTradeHistoryTradeType) { todo!() }
    /// DEPRECATED: callback-based observer. Use `ui_event_stream()` —
    /// the callback observer pins the foreign object in UniFFI's
    /// static handleMap and produces a cross-language retain cycle
    /// (Android GC + iOS ARC).
    pub fn set_ui_event_observer(&self, observer: Arc<dyn LeadTradeHistoryUiEventObserver>) { todo!() }
    /// Start the interactor lifecycle.
    /// 
    /// Calls the user's `InteractorLifecycle::start()` implementation.
    /// Override that method to add custom startup behavior (e.g., initial data loads).
    pub fn start(&self) { todo!() }
    /// Get the current state (UniFFI export).
    /// 
    /// Returns the External type (UniFFI-compatible).
    pub fn state(&self) -> LeadTradeHistoryState { todo!() }
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
    /// Get current trade type (UniFFI export).
    pub fn trade_type(&self) -> LeadTradeHistoryTradeType { todo!() }
    /// Opt-in async UI event stream (cycle-free). Single-subscriber.
    pub fn ui_event_stream(&self) -> Arc<LeadTradeHistoryUiEventStream> { todo!() }
}

/// Deprecated UniFFI callback interface for observing UI events. Prefer the cycle-free `LeadTradeHistoryUiEventStream` returned by `interactor.ui_event_stream()`.
pub struct LeadTradeHistoryUiEventObserver;
impl LeadTradeHistoryUiEventObserver {
    pub fn on_ui_event(&self, event: LeadTradeHistoryUiEvent) { todo!() }
}

/// Opt-in async event stream from `interactor.ui_event_stream()`.
/// Single-subscriber, latest-wins. `next()` resolves to `None` when the
/// interactor is dropped or a new subscription replaces this one.
pub struct LeadTradeHistoryUiEventStream;
impl LeadTradeHistoryUiEventStream {
    pub async fn next(&self) -> Option<LeadTradeHistoryUiEvent> { todo!() }
}

/// Lead Trader Application Interactor
/// 
/// Manages the complete application flow with state management.
/// All business logic is delegated to UseCases; Native only observes state changes.
/// 
/// # Architecture
/// 
/// Interactor injects UseCases (Account pattern):
/// - `LoadStatusUseCase`: Load trader status and check if applying
/// - `PreCheckUseCase`: Pre-check eligibility and decide result
/// - `VerifyAffiliateUseCase`: Verify affiliate ID
/// - `SubmitApplicationUseCase`: Submit application and decide approval type
/// 
/// # Usage (Native side)
/// 
/// ```kotlin
/// // Create interactor
/// val interactor = createLeadTraderApplicationInteractor(LeadTraderApplicationBizType.FUTURES)
/// 
/// // Get state (poll or use callback)
/// val state = interactor.getState()
/// 
/// // Handle effects
/// if (state.effectId > lastEffectId) {
/// when (state.effectType) {
/// EffectType.PRECHECK_PASSED -> navigateToForm()
/// EffectType.PRECHECK_NOT_PASSED_TOAST -> showToast(state.effectPayload.message)
/// // ...
/// }
/// interactor.ackEffect(state.effectId)
/// }
/// 
/// // Trigger actions
/// interactor.loadData()
/// interactor.performPrecheck()
/// interactor.submitApplication(request)
/// ```
pub struct LeadTraderApplicationInteractor;
impl LeadTraderApplicationInteractor {
    /// Acknowledge an effect has been handled.
    /// 
    /// Native should call this after handling an effect to clear it.
    pub fn ack_effect(&self, effect_id: i64) { todo!() }
    /// Clear all callbacks.
    /// 
    /// Call this when the native side is being destroyed to prevent callbacks
    /// to deallocated objects.
    pub fn clear_callbacks(&self) { todo!() }
    /// Close audit fail popup.
    /// 
    /// This is called when user dismisses the audit failure popup.
    /// It resets the trader application failure status.
    /// 
    /// Effects emitted:
    /// - `AuditFailPopupClosed`: Popup closed successfully
    /// - `AuditFailPopupCloseError`: Close failed (see effect_message)
    pub fn close_audit_fail_popup(&self) { todo!() }
    /// Fetch instruments available for lead trading.
    /// 
    /// Fetches the list of instruments that the user can select for lead trading.
    /// All instruments are marked as selected by default.
    /// 
    /// Effects emitted:
    /// - `FetchInstrumentsSuccess`: Instruments fetched, stored in `state.instruments`
    /// - `FetchInstrumentsError`: Fetch failed (see `effect_message`)
    pub fn fetch_instruments(&self) { todo!() }
    /// Fetch popup status (whether to show first-time popup).
    /// 
    /// This checks if the user should see the first-time lead trader popup.
    /// 
    /// # Arguments
    /// 
    /// * `biz_type` - The business type string (e.g., "0" for futures, "1" for spot)
    /// 
    /// Effects emitted:
    /// - `PopupStatusFetched`: Status fetched (see effect_should_show_popup)
    /// - `PopupStatusFetchError`: Fetch failed (see effect_message)
    pub fn fetch_popup_status(&self, biz_type: String) { todo!() }
    /// Fetches popup status for the given business type.
    /// 
    /// Returns `OKRustFuture<bool>` for easy integration with `OkRustFutureWrapper`.
    /// Use this for Repository-level calls where you don't need state management.
    /// 
    /// # Parameters
    /// - `biz_type`: Business type string (e.g., "0" for futures, "1" for spot)
    /// 
    /// # Returns
    /// - `true` if popup should be shown
    /// - `false` if popup should not be shown
    pub fn fetch_popup_status_async(&self, biz_type: String) -> Arc<OKRustFutureboolWithOKSTFFIError> { todo!() }
    /// Fetches pre-check details as an async future.
    /// 
    /// This is a compatibility method for Repository-level usage where
    /// you don't need state management / callbacks. It calls the same
    /// internal PreCheckUseCase and RequestCenter as `perform_precheck()`.
    /// 
    /// Returns the same `LeadTraderApplicationPreCheck` type as the old UseCase.
    pub fn fetch_pre_check_details_async(&self, biz_type: LeadTraderApplicationBizType) -> Arc<OKRustFutureLeadTraderApplicationPreCheckWithOKSTFFIError> { todo!() }
    /// Get current state snapshot.
    /// 
    /// Note: Prefer using `set_state_callback()` instead of polling this method.
    pub fn get_state(&self) -> LeadTraderApplicationState { todo!() }
    /// Load initial data and check application status.
    /// 
    /// This combines multiple API calls into a single action:
    /// 1. Fetch trader status
    /// 2. Check if already applying or succeeded
    /// 
    /// State changes:
    /// - `content_state`: Loading -> Success/Error
    /// - `is_applying_or_success`: Updated based on status
    /// - `swap_status`, `spot_status`, etc.: Updated from API
    pub fn load_data(&self) { todo!() }
    /// Mark popup as read.
    /// 
    /// This marks the first-time lead trader popup as read so it won't show again.
    /// 
    /// # Arguments
    /// 
    /// * `biz_type` - The business type string (e.g., "0" for futures, "1" for spot)
    /// 
    /// Effects emitted:
    /// - `PopupMarkedAsRead`: Marked successfully
    /// - `PopupMarkAsReadError`: Mark failed (see effect_message)
    pub fn mark_popup_as_read(&self, biz_type: String) { todo!() }
    /// Marks popup as read for the given business type.
    /// 
    /// Returns `OKRustFuture<bool>` for easy integration with `OkRustFutureWrapper`.
    /// Use this for Repository-level calls where you don't need state management.
    /// 
    /// # Parameters
    /// - `biz_type`: Business type string (e.g., "0" for futures, "1" for spot)
    /// 
    /// # Returns
    /// - `true` on success
    pub fn mark_popup_as_read_async(&self, biz_type: String) -> Arc<OKRustFutureboolWithOKSTFFIError> { todo!() }
    /// Perform pre-check for eligibility.
    /// 
    /// This checks if the user meets all requirements to apply.
    /// 
    /// State changes:
    /// - `precheck_state`: Loading -> Success/Error
    /// 
    /// Effects emitted:
    /// - `PreCheckPassed`: User can proceed
    /// - `PreCheckNotPassedToast`: Show toast with message
    /// - `PreCheckNotPassedPopup`: Show popup with details
    pub fn perform_precheck(&self) { todo!() }
    /// Set effect callback.
    /// 
    /// When an effect is emitted, the callback's `on_effect` method will be called.
    /// This eliminates the need for polling effects.
    /// 
    /// # Parameters
    /// 
    /// * `callback` - The callback to receive effect notifications
    pub fn set_effect_callback(&self, callback: Arc<dyn LeadTraderEffectCallback>) { todo!() }
    /// Set instruments for lead trading.
    /// 
    /// Updates the list of instruments that the lead trader can use.
    /// 
    /// # Parameters
    /// - `instruments`: List of instrument IDs to enable (e.g., `["BTC-USDT-SWAP", "ETH-USDT-SWAP"]`)
    /// - `auto_add_new_inst`: Whether to auto-add newly listed instruments
    /// 
    /// # Effects
    /// 
    /// - `SetInstrumentsSuccess`: Instruments updated successfully
    /// - `SetInstrumentsError`: Update failed (see `effect_message`)
    pub fn set_instruments(&self, instruments: Vec<String>, auto_add_new_inst: bool) { todo!() }
    /// Set state change callback.
    /// 
    /// When state changes, the callback's `on_state_changed` method will be called.
    /// This eliminates the need for polling `get_state()`.
    /// 
    /// # Parameters
    /// 
    /// * `callback` - The callback to receive state updates
    pub fn set_state_callback(&self, callback: Arc<dyn LeadTraderStateCallback>) { todo!() }
    /// Submit application.
    /// 
    /// Uses the existing `TraderApplyFormRequest` type to avoid creating duplicate types.
    /// 
    /// State changes:
    /// - `submit_state`: Loading -> Success/Error
    /// 
    /// Effects emitted:
    /// - `SubmitAutoApproved`: Auto-approved (show success)
    /// - `SubmitManualApproval`: Pending review (show applying page)
    /// - `SubmitError`: Submit failed
    pub fn submit_application(&self, request: TraderApplyFormRequest) { todo!() }
    /// Submits the lead trader application as an async future.
    /// 
    /// Compatibility method matching the old UseCase's `submitApplication()`.
    /// Returns `TraderApplyFormResults` on success.
    pub fn submit_application_async(&self, form_request: TraderApplyFormRequest) -> Arc<OKRustFutureTraderApplyFormResultsWithOKSTFFIError> { todo!() }
    /// Verify affiliate ID.
    /// 
    /// Effects emitted:
    /// - `AffiliateIdValid`: ID is valid
    /// - `AffiliateIdInvalid`: ID is invalid
    pub fn verify_affiliate_id(&self, affiliate_id: String) { todo!() }
    /// Verifies an affiliate UID as an async future.
    /// 
    /// Compatibility method matching the old UseCase's `verifyAffiliateUid()`.
    /// Returns `true` if valid, `false` if invalid or empty response.
    pub fn verify_affiliate_uid_async(&self, affiliate_uid: String) -> Arc<OKRustFutureboolWithOKSTFFIError> { todo!() }
}

/// Callback trait for effect (one-shot event) notifications.
/// 
/// Native side implements this trait to receive effects from Rust.
/// Effects are one-shot events that should trigger navigation or toasts.
/// 
/// # Example (Kotlin)
/// 
/// ```kotlin
/// class MyCallback : LeadTraderEffectCallback {
/// override fun onEffect(state: LeadTraderApplicationState) {
/// when (state.effectType) {
/// EffectType.PRECHECK_PASSED -> navigateToForm()
/// // ...
/// }
/// }
/// }
/// 
/// interactor.setEffectCallback(MyCallback())
/// ```
pub struct LeadTraderEffectCallback;
impl LeadTraderEffectCallback {
    /// Called when an effect is emitted.
    /// 
    /// The state contains effect details in `effect_*` fields.
    /// Native should handle the effect and then call `ack_effect()`.
    pub fn on_effect(&self, state: LeadTraderApplicationState) { todo!() }
}

/// Lead Trader List Interactor
pub struct LeadTraderListInteractor;
impl LeadTraderListInteractor {
    /// Get current business line.
    pub fn get_business_line(&self) -> LeadTraderListBusinessLine { todo!() }
    /// Load lead trader list with filter.
    pub fn load_data(&self, filter: LeadTraderListFilter) { todo!() }
    /// Load more data (next page).
    pub fn load_more(&self) { todo!() }
    /// Refresh data (alias for load_data with last filter).
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
    /// DEPRECATED. Use `ui_event_stream()`; drop the returned stream
    /// wrapper to unsubscribe.
    pub fn remove_ui_event_observer(&self) { todo!() }
    /// Select trader by index in the current list.
    pub fn select_trader_by_index(&self, index: i32) { todo!() }
    /// Select trader by unique name.
    pub fn select_trader_by_unique_name(&self, unique_name: String) { todo!() }
    /// Update the country filter and trigger a refetch. Mirrors native
    /// `SocialSquareViewModel.setLocalFirst()` (line 367) which toggles
    /// country-based filtering. Bridge resolves the user's country via
    /// `FilterDataController.getCountryId()` and passes it here.
    /// Pass `None` to clear the filter.
    pub fn set_country_filter(&self, country_id: Option<String>) { todo!() }
    /// Enable or disable debug logging.
    /// 
    /// In release builds this is a no-op, but the symbol must exist
    /// so that UniFFI bindings generated in debug mode continue to
    /// link successfully against the release library.
    pub fn set_debug(&self, _enabled: bool) { todo!() }
    /// Update the sort key and trigger a refetch. Mirrors native
    /// `SocialSquareViewModel.setSort(key)` (line 349).
    /// `sort_code` is the server `type` param value (e.g., the `SquareSort.value` string).
    pub fn set_sort_mode(&self, sort_code: String) { todo!() }
    /// Set a state observer (UniFFI export).
    /// 
    /// DEPRECATED: callback-based observer. Use `state_stream()` —
    /// the callback observer pins the foreign object in UniFFI's
    /// static handleMap and produces a cross-language retain cycle
    /// (Android GC + iOS ARC).
    pub fn set_state_observer(&self, observer: Arc<dyn InteractorStateChangeObserver>) { todo!() }
    /// Update the time-period filter and trigger a refetch. Mirrors native
    /// `SocialSquareViewModel.setTimePeriod(period)` (line 359).
    /// `period_key` is the server `latestNum` param value (e.g., "7", "30", "90").
    pub fn set_time_period(&self, period_key: String) { todo!() }
    /// DEPRECATED: callback-based observer. Use `ui_event_stream()` —
    /// the callback observer pins the foreign object in UniFFI's
    /// static handleMap and produces a cross-language retain cycle
    /// (Android GC + iOS ARC).
    pub fn set_ui_event_observer(&self, observer: Arc<dyn LeadTraderListUiEventObserver>) { todo!() }
    /// Start the interactor lifecycle.
    /// 
    /// Calls the user's `InteractorLifecycle::start()` implementation.
    /// Override that method to add custom startup behavior (e.g., initial data loads).
    pub fn start(&self) { todo!() }
    /// Get the current state (UniFFI export).
    /// 
    /// Returns the External type (UniFFI-compatible).
    pub fn state(&self) -> LeadTraderListState { todo!() }
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
    pub fn ui_event_stream(&self) -> Arc<LeadTraderListUiEventStream> { todo!() }
    /// Patch a single rank item in place by `unique_name`. Mirrors native
    /// `SocialSquareViewModel.updateMarketBoardData(item)` (line 665).
    /// Used after follow/unfollow to update the row's `follow_type` +
    /// `follower_num` without refetching. No-op if the item isn't in the list.
    pub fn update_rank_item(&self, unique_name: String, follow_type: String, follower_num: String) { todo!() }
}

/// Deprecated UniFFI callback interface for observing UI events. Prefer the cycle-free `LeadTraderListUiEventStream` returned by `interactor.ui_event_stream()`.
pub struct LeadTraderListUiEventObserver;
impl LeadTraderListUiEventObserver {
    pub fn on_ui_event(&self, event: LeadTraderListUiEvent) { todo!() }
}

/// Opt-in async event stream from `interactor.ui_event_stream()`.
/// Single-subscriber, latest-wins. `next()` resolves to `None` when the
/// interactor is dropped or a new subscription replaces this one.
pub struct LeadTraderListUiEventStream;
impl LeadTraderListUiEventStream {
    pub async fn next(&self) -> Option<LeadTraderListUiEvent> { todo!() }
}

/// Lead Trader Settings Interactor
pub struct LeadTraderSettingsInteractor;
impl LeadTraderSettingsInteractor {
    /// Confirm and apply private mode change.
    pub fn confirm_private_mode(&self, is_on: bool, share_ratio: Option<f64>) { todo!() }
    /// Exit leading trade.
    pub fn exit_leading_trade(&self, reason: String, advise: String) { todo!() }
    /// Load all settings data.
    pub fn load_data(&self) { todo!() }
    /// Refresh min copy amount (lightweight).
    pub fn refresh_min_copy_amount(&self) { todo!() }
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
    pub fn set_ui_event_observer(&self, observer: Arc<dyn LeadTraderSettingsUiEventObserver>) { todo!() }
    /// Start the interactor lifecycle.
    /// 
    /// Calls the user's `InteractorLifecycle::start()` implementation.
    /// Override that method to add custom startup behavior (e.g., initial data loads).
    pub fn start(&self) { todo!() }
    /// Get the current state (UniFFI export).
    /// 
    /// Returns the External type (UniFFI-compatible).
    pub fn state(&self) -> LeadTraderSettingsState { todo!() }
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
    /// Sync profit sharing ratio without API call.
    pub fn sync_profit_sharing_ratio(&self, ratio: String) { todo!() }
    /// Toggle private mode (shows confirmation first).
    pub fn toggle_private_mode(&self, is_on: bool) { todo!() }
    /// Toggle strategy protection.
    pub fn toggle_strategy_protection(&self, is_on: bool) { todo!() }
    /// Opt-in async UI event stream (cycle-free). Single-subscriber.
    pub fn ui_event_stream(&self) -> Arc<LeadTraderSettingsUiEventStream> { todo!() }
    /// Update copy mode restriction.
    pub fn update_copy_mode(&self, mode: CopyModeRestriction) { todo!() }
    /// Update minimum copy amount settings.
    pub fn update_min_copy_amount(&self, custom_amount: Option<String>, smart_amount: Option<String>) { todo!() }
    /// Update profit sharing ratio.
    pub fn update_profit_sharing_ratio(&self, ratio: String, lead_mode: String) { todo!() }
}

/// Deprecated UniFFI callback interface for observing UI events. Prefer the cycle-free `LeadTraderSettingsUiEventStream` returned by `interactor.ui_event_stream()`.
pub struct LeadTraderSettingsUiEventObserver;
impl LeadTraderSettingsUiEventObserver {
    pub fn on_ui_event(&self, event: LeadTraderSettingsUiEvent) { todo!() }
}

/// Opt-in async event stream from `interactor.ui_event_stream()`.
/// Single-subscriber, latest-wins. `next()` resolves to `None` when the
/// interactor is dropped or a new subscription replaces this one.
pub struct LeadTraderSettingsUiEventStream;
impl LeadTraderSettingsUiEventStream {
    pub async fn next(&self) -> Option<LeadTraderSettingsUiEvent> { todo!() }
}

/// Callback trait for state change notifications.
/// 
/// Native side implements this trait to receive state updates from Rust.
/// This eliminates the need for polling.
/// 
/// # Example (Kotlin)
/// 
/// ```kotlin
/// class MyCallback : LeadTraderStateCallback {
/// override fun onStateChanged(state: LeadTraderApplicationState) {
/// // Update UI with new state
/// }
/// }
/// 
/// interactor.setStateCallback(MyCallback())
/// ```
pub struct LeadTraderStateCallback;
impl LeadTraderStateCallback {
    /// Called when state changes.
    pub fn on_state_changed(&self, state: LeadTraderApplicationState) { todo!() }
}

/// Callback trait for effect (one-shot event) notifications.
/// 
/// Native side implements this trait to receive effects from Rust.
pub struct ManageCopyTradersEffectCallback;
impl ManageCopyTradersEffectCallback {
    /// Called when an effect is emitted.
    pub fn on_effect(&self, state: ManageCopyTradersState) { todo!() }
}

/// Manage Copy Traders Interactor
/// 
/// Manages the "Manage Copy Traders" page for lead traders.
/// Uses the `managed-copiers-list` API endpoint.
pub struct ManageCopyTradersInteractor;
impl ManageCopyTradersInteractor {
    /// Acknowledge an effect (clear effect state).
    /// 
    /// Call this after handling an effect to clear the effect from state.
    /// 
    /// # Parameters
    /// 
    /// * `effect_id` - The effect ID to acknowledge
    pub fn ack_effect(&self, effect_id: i64) { todo!() }
    /// Change sort order.
    /// 
    /// Changes the sort field/order and reloads data from first page.
    /// 
    /// # Parameters
    /// 
    /// * `sort_field` - The field to sort by
    /// * `sort_order` - The sort order (ascending/descending)
    /// 
    /// # Effects
    /// 
    /// - `DataLoaded` on success
    /// - `DataLoadError` on failure
    pub fn change_sort(&self, sort_field: ManagedCopierSortField, sort_order: ManagedCopierSortOrder) { todo!() }
    /// Clear all callbacks to avoid memory leaks.
    pub fn clear_callbacks(&self) { todo!() }
    /// Get current state.
    pub fn get_state(&self) -> ManageCopyTradersState { todo!() }
    /// Get current trade type.
    pub fn get_trade_type(&self) -> ManageCopyTradersTradeType { todo!() }
    /// Load initial data.
    /// 
    /// Fetches the first page of managed copiers with default sort,
    /// and concurrently fetches aggregated trade info (AUM, follower count).
    /// 
    /// # Effects
    /// 
    /// - `DataLoaded` on success
    /// - `DataLoadError` on failure
    pub fn load_data(&self) { todo!() }
    /// Load more data (pagination).
    /// 
    /// Loads the next page of managed copiers and appends to the list.
    /// 
    /// # Effects
    /// 
    /// - `MoreDataLoaded` on success
    /// - `MoreDataLoadError` on failure
    pub fn load_more(&self) { todo!() }
    /// Refresh data (pull-to-refresh).
    /// 
    /// Reloads the first page of managed copiers and aggregated trade info.
    /// 
    /// # Effects
    /// 
    /// - `RefreshCompleted` on success
    /// - `RefreshError` on failure
    pub fn refresh(&self) { todo!() }
    /// Remove copiers by their unique names.
    /// 
    /// # Parameters
    /// 
    /// * `unique_names` - List of unique names to remove
    /// 
    /// # Effects
    /// 
    /// - `CopiersRemoved` on success
    /// - `CopiersRemoveError` on failure
    pub fn remove_copiers(&self, unique_names: Vec<String>) { todo!() }
    /// Set effect callback.
    pub fn set_effect_callback(&self, callback: Arc<dyn ManageCopyTradersEffectCallback>) { todo!() }
    /// Set filters for the copier list query.
    /// 
    /// Maps to the native filter logic:
    /// - For private leads: `privateRelationType` = relation_type, `trafficType` = traffic_type
    /// - For public leads (Swap): `affiliateInviteeType` = relation_type, `trafficType` = traffic_type
    /// - For public leads (Spot): `affiliateInviteeType` = relation_type, `trafficType` = null
    /// 
    /// # Parameters
    /// 
    /// * `traffic_type` - "0"=all, "1"=public/platform, "2"=private/invite. None to clear.
    /// * `affiliate_invitee_type` - Relation type for public leads. None to clear.
    /// * `private_relation_type` - Relation type for private leads. None to clear.
    pub fn set_filters(&self, traffic_type: Option<String>, affiliate_invitee_type: Option<String>, private_relation_type: Option<String>) { todo!() }
    /// Set search query for filtering copiers by name.
    pub fn set_search_query(&self, query: Option<String>) { todo!() }
    /// Set state change callback.
    pub fn set_state_callback(&self, callback: Arc<dyn ManageCopyTradersStateCallback>) { todo!() }
}

/// Callback trait for state change notifications.
/// 
/// Native side implements this trait to receive state updates from Rust.
pub struct ManageCopyTradersStateCallback;
impl ManageCopyTradersStateCallback {
    /// Called when state changes.
    pub fn on_state_changed(&self, state: ManageCopyTradersState) { todo!() }
}

/// Marketplace Home Interactor
pub struct MarketplaceHomeInteractor;
impl MarketplaceHomeInteractor {
    /// Check if the current user should see a tier upgrade dialog.
    pub fn check_tier_upgrade(&self) { todo!() }
    /// Check if a trader application approved popup should be shown.
    pub fn check_trader_application_popup(&self) { todo!() }
    /// Check trader application status.
    pub fn check_trader_application_status(&self) { todo!() }
    /// Claim a voucher by batch number.
    /// 
    /// `url` is an opaque navigation target the caller wants to receive back when the claim
    /// resolves. It is not interpreted on the Rust side — only round-tripped through the
    /// resulting `VoucherClaimed { url }` / `VoucherClaimError { url }` UI event.
    pub fn claim_voucher(&self, batch_no: String, url: String) { todo!() }
    /// Fetch received coupons.
    pub fn fetch_received_coupons(&self) { todo!() }
    /// Load (or reload) all dashboard data. Drives both the initial load and the
    /// pull-to-refresh path — the caller is responsible for any visual transition
    /// (e.g. setting a Loading view state) before invoking this.
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
    pub fn set_ui_event_observer(&self, observer: Arc<dyn MarketplaceHomeUiEventObserver>) { todo!() }
    /// Start the interactor lifecycle.
    /// 
    /// Calls the user's `InteractorLifecycle::start()` implementation.
    /// Override that method to add custom startup behavior (e.g., initial data loads).
    pub fn start(&self) { todo!() }
    /// Get the current state (UniFFI export).
    /// 
    /// Returns the External type (UniFFI-compatible).
    pub fn state(&self) -> MarketplaceHomeState { todo!() }
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
    pub fn ui_event_stream(&self) -> Arc<MarketplaceHomeUiEventStream> { todo!() }
}

/// Deprecated UniFFI callback interface for observing UI events. Prefer the cycle-free `MarketplaceHomeUiEventStream` returned by `interactor.ui_event_stream()`.
pub struct MarketplaceHomeUiEventObserver;
impl MarketplaceHomeUiEventObserver {
    pub fn on_ui_event(&self, event: MarketplaceHomeUiEvent) { todo!() }
}

/// Opt-in async event stream from `interactor.ui_event_stream()`.
/// Single-subscriber, latest-wins. `next()` resolves to `None` when the
/// interactor is dropped or a new subscription replaces this one.
pub struct MarketplaceHomeUiEventStream;
impl MarketplaceHomeUiEventStream {
    pub async fn next(&self) -> Option<MarketplaceHomeUiEvent> { todo!() }
}

/// MyCopiesCurrentTraders Interactor
pub struct MyCopiesCurrentTradersInteractor;
impl MyCopiesCurrentTradersInteractor {
    /// Get current instrument type.
    pub fn get_inst_type(&self) -> CopierPnlInstType { todo!() }
    /// Refresh the list of currently followed traders (original, backward-
    /// compatible signature). Delegates to `refresh_with_request_id` with an
    /// EMPTY request id, so its terminal `DataLoaded`/`DataLoadError` events
    /// echo `request_id: ""` and cannot be correlated with this exact command.
    /// Correlation-aware consumers (the iOS My Copies migration) must use
    /// `refresh_with_request_id` instead.
    pub fn refresh(&self, rise_fall_cycle: CopierPnlRiseFallCycleType) { todo!() }
    /// Refresh the list of currently followed traders with a caller-supplied
    /// correlation token.
    /// 
    /// `request_id` is an opaque caller-supplied token (native generates a
    /// fresh UUID per user/parent refresh); the terminal `DataLoaded` /
    /// `DataLoadError` event echoes it so the caller can correlate the
    /// completion with this exact command. A user refresh supersedes any
    /// pending internal (WS-triggered) fetch.
    pub fn refresh_with_request_id(&self, request_id: String, rise_fall_cycle: CopierPnlRiseFallCycleType) { todo!() }
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
    /// Set rise/fall cycle and restart WebSocket if running.
    pub fn set_cycle(&self, rise_fall_cycle: CopierPnlRiseFallCycleType) { todo!() }
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
    pub fn set_ui_event_observer(&self, observer: Arc<dyn MyCopiesCurrentTradersUiEventObserver>) { todo!() }
    /// Start the interactor lifecycle.
    /// 
    /// Calls the user's `InteractorLifecycle::start()` implementation.
    /// Override that method to add custom startup behavior (e.g., initial data loads).
    pub fn start(&self) { todo!() }
    /// Start WebSocket subscription manually.
    pub fn start_ws(&self) { todo!() }
    /// Get the current state (UniFFI export).
    /// 
    /// Returns the External type (UniFFI-compatible).
    pub fn state(&self) -> MyCopiesCurrentTradersState { todo!() }
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
    /// Stop WebSocket subscription.
    pub fn stop_ws(&self) { todo!() }
    /// Opt-in async UI event stream (cycle-free). Single-subscriber.
    pub fn ui_event_stream(&self) -> Arc<MyCopiesCurrentTradersUiEventStream> { todo!() }
}

/// Deprecated UniFFI callback interface for observing UI events. Prefer the cycle-free `MyCopiesCurrentTradersUiEventStream` returned by `interactor.ui_event_stream()`.
pub struct MyCopiesCurrentTradersUiEventObserver;
impl MyCopiesCurrentTradersUiEventObserver {
    pub fn on_ui_event(&self, event: MyCopiesCurrentTradersUiEvent) { todo!() }
}

/// Opt-in async event stream from `interactor.ui_event_stream()`.
/// Single-subscriber, latest-wins. `next()` resolves to `None` when the
/// interactor is dropped or a new subscription replaces this one.
pub struct MyCopiesCurrentTradersUiEventStream;
impl MyCopiesCurrentTradersUiEventStream {
    pub async fn next(&self) -> Option<MyCopiesCurrentTradersUiEvent> { todo!() }
}

/// My Copies History List Spot Interactor
pub struct MyCopiesHistoryListSpotInteractor;
impl MyCopiesHistoryListSpotInteractor {
    /// Get current instrument type (Spot).
    pub fn get_inst_type(&self) -> CopierPnlInstType { todo!() }
    /// Load the next page.
    pub fn load_next_page(&self, page_size: i32) { todo!() }
    /// Load private info (not supported on spot).
    pub fn load_private_info(&self, index: i32) { todo!() }
    /// Refresh the history list (first page).
    pub fn refresh(&self, page_size: i32, rise_fall_cycle: CopierPnlRiseFallCycleType) { todo!() }
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
    /// Select a profile (for navigation to user profile).
    pub fn select_profile(&self, index: i32) { todo!() }
    /// Select a row (for navigation to details).
    pub fn select_row(&self, index: i32) { todo!() }
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
    pub fn set_ui_event_observer(&self, observer: Arc<dyn MyCopiesHistoryListSpotUiEventObserver>) { todo!() }
    /// Start the interactor lifecycle.
    /// 
    /// Calls the user's `InteractorLifecycle::start()` implementation.
    /// Override that method to add custom startup behavior (e.g., initial data loads).
    pub fn start(&self) { todo!() }
    /// Get the current state (UniFFI export).
    /// 
    /// Returns the External type (UniFFI-compatible).
    pub fn state(&self) -> MyCopiesHistoryListState { todo!() }
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
    pub fn ui_event_stream(&self) -> Arc<MyCopiesHistoryListSpotUiEventStream> { todo!() }
}

/// Deprecated UniFFI callback interface for observing UI events. Prefer the cycle-free `MyCopiesHistoryListSpotUiEventStream` returned by `interactor.ui_event_stream()`.
pub struct MyCopiesHistoryListSpotUiEventObserver;
impl MyCopiesHistoryListSpotUiEventObserver {
    pub fn on_ui_event(&self, event: MyCopiesHistoryListSpotUiEvent) { todo!() }
}

/// Opt-in async event stream from `interactor.ui_event_stream()`.
/// Single-subscriber, latest-wins. `next()` resolves to `None` when the
/// interactor is dropped or a new subscription replaces this one.
pub struct MyCopiesHistoryListSpotUiEventStream;
impl MyCopiesHistoryListSpotUiEventStream {
    pub async fn next(&self) -> Option<MyCopiesHistoryListSpotUiEvent> { todo!() }
}

/// My Copies History List Swap Interactor
pub struct MyCopiesHistoryListSwapInteractor;
impl MyCopiesHistoryListSwapInteractor {
    /// Get current instrument type (Swap).
    pub fn get_inst_type(&self) -> CopierPnlInstType { todo!() }
    /// Load the next page.
    pub fn load_next_page(&self, page_size: i32) { todo!() }
    /// Load private info for a private trader (Swap only).
    pub fn load_private_info(&self, index: i32) { todo!() }
    /// Refresh the history list (first page).
    pub fn refresh(&self, page_size: i32, rise_fall_cycle: CopierPnlRiseFallCycleType) { todo!() }
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
    /// Select a profile (for navigation to user profile).
    pub fn select_profile(&self, index: i32) { todo!() }
    /// Select a row (for navigation to details).
    pub fn select_row(&self, index: i32) { todo!() }
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
    pub fn set_ui_event_observer(&self, observer: Arc<dyn MyCopiesHistoryListSwapUiEventObserver>) { todo!() }
    /// Start the interactor lifecycle.
    /// 
    /// Calls the user's `InteractorLifecycle::start()` implementation.
    /// Override that method to add custom startup behavior (e.g., initial data loads).
    pub fn start(&self) { todo!() }
    /// Get the current state (UniFFI export).
    /// 
    /// Returns the External type (UniFFI-compatible).
    pub fn state(&self) -> MyCopiesHistoryListState { todo!() }
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
    pub fn ui_event_stream(&self) -> Arc<MyCopiesHistoryListSwapUiEventStream> { todo!() }
}

/// Deprecated UniFFI callback interface for observing UI events. Prefer the cycle-free `MyCopiesHistoryListSwapUiEventStream` returned by `interactor.ui_event_stream()`.
pub struct MyCopiesHistoryListSwapUiEventObserver;
impl MyCopiesHistoryListSwapUiEventObserver {
    pub fn on_ui_event(&self, event: MyCopiesHistoryListSwapUiEvent) { todo!() }
}

/// Opt-in async event stream from `interactor.ui_event_stream()`.
/// Single-subscriber, latest-wins. `next()` resolves to `None` when the
/// interactor is dropped or a new subscription replaces this one.
pub struct MyCopiesHistoryListSwapUiEventStream;
impl MyCopiesHistoryListSwapUiEventStream {
    pub async fn next(&self) -> Option<MyCopiesHistoryListSwapUiEvent> { todo!() }
}

/// My Followers Interactor
/// 
/// Manages the followers list page for lead traders.
pub struct MyFollowersInteractor;
impl MyFollowersInteractor {
    /// Change sort order (triggers reload).
    pub fn change_sort(&self, sort_type: FollowerSortType) { todo!() }
    /// Get current trade type (UniFFI export).
    pub fn get_trade_type(&self) -> MyFollowersTradeType { todo!() }
    /// Load initial data (first page with default sort).
    pub fn load_data(&self) { todo!() }
    /// Load more data (pagination).
    pub fn load_more(&self) { todo!() }
    /// Refresh data (pull-to-refresh).
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
    pub fn set_ui_event_observer(&self, observer: Arc<dyn MyFollowersUiEventObserver>) { todo!() }
    /// Start the interactor lifecycle.
    /// 
    /// Calls the user's `InteractorLifecycle::start()` implementation.
    /// Override that method to add custom startup behavior (e.g., initial data loads).
    pub fn start(&self) { todo!() }
    /// Get the current state (UniFFI export).
    /// 
    /// Returns the External type (UniFFI-compatible).
    pub fn state(&self) -> MyFollowersState { todo!() }
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
    pub fn ui_event_stream(&self) -> Arc<MyFollowersUiEventStream> { todo!() }
}

/// Deprecated UniFFI callback interface for observing UI events. Prefer the cycle-free `MyFollowersUiEventStream` returned by `interactor.ui_event_stream()`.
pub struct MyFollowersUiEventObserver;
impl MyFollowersUiEventObserver {
    pub fn on_ui_event(&self, event: MyFollowersUiEvent) { todo!() }
}

/// Opt-in async event stream from `interactor.ui_event_stream()`.
/// Single-subscriber, latest-wins. `next()` resolves to `None` when the
/// interactor is dropped or a new subscription replaces this one.
pub struct MyFollowersUiEventStream;
impl MyFollowersUiEventStream {
    pub async fn next(&self) -> Option<MyFollowersUiEvent> { todo!() }
}

/// My Lead Trader Details Interactor
pub struct MyLeadTraderDetailsInteractor;
impl MyLeadTraderDetailsInteractor {
    /// Load next history list page.
    pub fn history_list_load_next_page(&self, page_size: i32) { todo!() }
    /// Reset and load first history list page.
    pub fn history_list_reset_and_load_first_page(&self, page_size: i32) { todo!() }
    /// Refresh detail and start WS if allowed.
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
    /// DEPRECATED. Use `ui_event_stream()`; drop the returned stream
    /// wrapper to unsubscribe.
    pub fn remove_ui_event_observer(&self) { todo!() }
    /// Set the cycle for PnL WS.
    pub fn set_cycle(&self, rise_fall_cycle: CopierPnlRiseFallCycleType) { todo!() }
    /// Enable or disable debug logging.
    /// 
    /// In release builds this is a no-op, but the symbol must exist
    /// so that UniFFI bindings generated in debug mode continue to
    /// link successfully against the release library.
    pub fn set_debug(&self, _enabled: bool) { todo!() }
    /// Set whether the prompt is showing.
    pub fn set_prompt_showing(&self, is_showing: bool) { todo!() }
    /// Set a state observer (UniFFI export).
    /// 
    /// DEPRECATED: callback-based observer. Use `state_stream()` —
    /// the callback observer pins the foreign object in UniFFI's
    /// static handleMap and produces a cross-language retain cycle
    /// (Android GC + iOS ARC).
    pub fn set_state_observer(&self, observer: Arc<dyn InteractorStateChangeObserver>) { todo!() }
    /// Set the trading mode (Live/Demo).
    pub fn set_trading_mode(&self, trading_mode: CopierPnlTradingMode) { todo!() }
    /// DEPRECATED: callback-based observer. Use `ui_event_stream()` —
    /// the callback observer pins the foreign object in UniFFI's
    /// static handleMap and produces a cross-language retain cycle
    /// (Android GC + iOS ARC).
    pub fn set_ui_event_observer(&self, observer: Arc<dyn MyLeadTraderDetailsUiEventObserver>) { todo!() }
    /// Start the interactor lifecycle.
    /// 
    /// Calls the user's `InteractorLifecycle::start()` implementation.
    /// Override that method to add custom startup behavior (e.g., initial data loads).
    pub fn start(&self) { todo!() }
    /// Get the current state (UniFFI export).
    /// 
    /// Returns the External type (UniFFI-compatible).
    pub fn state(&self) -> MyLeadTraderDetailsState { todo!() }
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
    pub fn ui_event_stream(&self) -> Arc<MyLeadTraderDetailsUiEventStream> { todo!() }
}

/// Deprecated UniFFI callback interface for observing UI events. Prefer the cycle-free `MyLeadTraderDetailsUiEventStream` returned by `interactor.ui_event_stream()`.
pub struct MyLeadTraderDetailsUiEventObserver;
impl MyLeadTraderDetailsUiEventObserver {
    pub fn on_ui_event(&self, event: MyLeadTraderDetailsUiEvent) { todo!() }
}

/// Opt-in async event stream from `interactor.ui_event_stream()`.
/// Single-subscriber, latest-wins. `next()` resolves to `None` when the
/// interactor is dropped or a new subscription replaces this one.
pub struct MyLeadTraderDetailsUiEventStream;
impl MyLeadTraderDetailsUiEventStream {
    pub async fn next(&self) -> Option<MyLeadTraderDetailsUiEvent> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureBenefitDetailVecWithOKSTFFIError;
impl OKRustFutureBenefitDetailVecWithOKSTFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKSTFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<Vec<BenefitDetail>> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKSTFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureCancelCopyPosResponseWithOKSTFFIError;
impl OKRustFutureCancelCopyPosResponseWithOKSTFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKSTFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<CancelCopyPosResponse> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKSTFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureCopierBatchSetLeverageResultWithOKSTFFIError;
impl OKRustFutureCopierBatchSetLeverageResultWithOKSTFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKSTFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<CopierBatchSetLeverageResult> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKSTFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureCopierRetentionPopupResponseVecWithOKSTFFIError;
impl OKRustFutureCopierRetentionPopupResponseVecWithOKSTFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKSTFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<Vec<CopierRetentionPopupResponse>> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKSTFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureCopyTradingOrderCouponFlagVecWithOKSTFFIError;
impl OKRustFutureCopyTradingOrderCouponFlagVecWithOKSTFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKSTFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<Vec<CopyTradingOrderCouponFlag>> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKSTFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureCurrentPositionVecWithOKSTFFIError;
impl OKRustFutureCurrentPositionVecWithOKSTFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKSTFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<Vec<CurrentPosition>> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKSTFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureCurveResetInfoVecWithOKSTFFIError;
impl OKRustFutureCurveResetInfoVecWithOKSTFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKSTFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<Vec<CurveResetInfo>> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKSTFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureHistoryProfitVecWithOKSTFFIError;
impl OKRustFutureHistoryProfitVecWithOKSTFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKSTFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<Vec<HistoryProfit>> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKSTFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureInviteCodeLinkInfoWithOKSTFFIError;
impl OKRustFutureInviteCodeLinkInfoWithOKSTFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKSTFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<InviteCodeLinkInfo> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKSTFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureLatestRecordVecWithOKSTFFIError;
impl OKRustFutureLatestRecordVecWithOKSTFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKSTFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<Vec<LatestRecord>> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKSTFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureLeadTraderApplicationPreCheckWithOKSTFFIError;
impl OKRustFutureLeadTraderApplicationPreCheckWithOKSTFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKSTFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<LeadTraderApplicationPreCheck> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKSTFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureLeaderRankResponseWithOKSTFFIError;
impl OKRustFutureLeaderRankResponseWithOKSTFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKSTFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<LeaderRankResponse> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKSTFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureLeverIntervalResultWithOKSTFFIError;
impl OKRustFutureLeverIntervalResultWithOKSTFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKSTFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<LeverIntervalResult> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKSTFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureOngoingLeadTradesInstrumentInfoVecWithOKSTFFIError;
impl OKRustFutureOngoingLeadTradesInstrumentInfoVecWithOKSTFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKSTFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<Vec<OngoingLeadTradesInstrumentInfo>> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKSTFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureOngoingLeadTradesPositionSummaryVecWithOKSTFFIError;
impl OKRustFutureOngoingLeadTradesPositionSummaryVecWithOKSTFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKSTFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<Vec<OngoingLeadTradesPositionSummary>> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKSTFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureOptionalVanguardShareInfoWithOKSTFFIError;
impl OKRustFutureOptionalVanguardShareInfoWithOKSTFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKSTFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<Option<VanguardShareInfo>> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKSTFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureOptionalVanguardSharePresentationWithOKSTFFIError;
impl OKRustFutureOptionalVanguardSharePresentationWithOKSTFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKSTFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<Option<VanguardSharePresentation>> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKSTFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFuturePreProfitVecWithOKSTFFIError;
impl OKRustFuturePreProfitVecWithOKSTFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKSTFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<Vec<PreProfit>> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKSTFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFuturePreferenceCoinItemVecWithOKSTFFIError;
impl OKRustFuturePreferenceCoinItemVecWithOKSTFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKSTFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<Vec<PreferenceCoinItem>> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKSTFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureProfileChartVecWithOKSTFFIError;
impl OKRustFutureProfileChartVecWithOKSTFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKSTFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<Vec<ProfileChart>> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKSTFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureProfilePositionHistoryVecWithOKSTFFIError;
impl OKRustFutureProfilePositionHistoryVecWithOKSTFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKSTFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<Vec<ProfilePositionHistory>> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKSTFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureProfitCurveVecWithOKSTFFIError;
impl OKRustFutureProfitCurveVecWithOKSTFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKSTFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<Vec<ProfitCurve>> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKSTFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureProfitDetailVecWithOKSTFFIError;
impl OKRustFutureProfitDetailVecWithOKSTFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKSTFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<Vec<ProfitDetail>> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKSTFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureProfitInfoVecWithOKSTFFIError;
impl OKRustFutureProfitInfoVecWithOKSTFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKSTFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<Vec<ProfitInfo>> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKSTFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureProfitSharingInfoVecWithOKSTFFIError;
impl OKRustFutureProfitSharingInfoVecWithOKSTFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKSTFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<Vec<ProfitSharingInfo>> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKSTFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFuturePublicInstrumentInfoVecWithOKSTFFIError;
impl OKRustFuturePublicInstrumentInfoVecWithOKSTFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKSTFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<Vec<PublicInstrumentInfo>> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKSTFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFuturePublicProfitSharingRatioVecWithOKSTFFIError;
impl OKRustFuturePublicProfitSharingRatioVecWithOKSTFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKSTFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<Vec<PublicProfitSharingRatio>> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKSTFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureQueryOrderResultVecWithOKSTFFIError;
impl OKRustFutureQueryOrderResultVecWithOKSTFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKSTFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<Vec<OrderOperationsQueryOrderResult>> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKSTFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureRankConfigWithOKSTFFIError;
impl OKRustFutureRankConfigWithOKSTFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKSTFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<RankConfig> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKSTFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureSettlementRatioLimitVecWithOKSTFFIError;
impl OKRustFutureSettlementRatioLimitVecWithOKSTFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKSTFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<Vec<SettlementRatioLimit>> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKSTFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureSystemStatusResultWithOKSTFFIError;
impl OKRustFutureSystemStatusResultWithOKSTFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKSTFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<SystemStatusResult> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKSTFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureTraderApplyFormResultsWithOKSTFFIError;
impl OKRustFutureTraderApplyFormResultsWithOKSTFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKSTFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<TraderApplyFormResults> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKSTFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureTraderFollowerListWithOKSTFFIError;
impl OKRustFutureTraderFollowerListWithOKSTFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKSTFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<TraderFollowerList> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKSTFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureTraderTradePerformanceStatsWithOKSTFFIError;
impl OKRustFutureTraderTradePerformanceStatsWithOKSTFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKSTFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<TraderTradePerformanceStats> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKSTFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureUpgradePopupInfoVecWithOKSTFFIError;
impl OKRustFutureUpgradePopupInfoVecWithOKSTFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKSTFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<Vec<UpgradePopupInfo>> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKSTFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureUserProfileOverviewVecWithOKSTFFIError;
impl OKRustFutureUserProfileOverviewVecWithOKSTFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKSTFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<Vec<UserProfileOverview>> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKSTFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureYieldPnlChartDataItemVecWithOKSTFFIError;
impl OKRustFutureYieldPnlChartDataItemVecWithOKSTFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKSTFFIError> { todo!() }
    /// Get current task status.
    /// 
    /// # Returns
    /// - `NotReady` if the task hasn't produced a result yet
    /// - `Success` if the task completed successfully
    /// - `Failed` if the task completed with an error (includes cancellation)
    pub fn get_result_status(&self) -> OKRustFutureStatus { todo!() }
    /// Get successful result (if any)
    pub fn get_success_result(&self) -> Option<Vec<YieldPnlChartDataItem>> { todo!() }
    /// Check whether the task has finished (success/failure/cancelled).
    pub fn is_finished(&self) -> bool { todo!() }
    /// Start async task with uniffi callback interface
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKSTFFIError> { todo!() }
}

/// OKRustFuture wrapper generated for specific type (non-generic) with uniffi export
pub struct OKRustFutureboolWithOKSTFFIError;
impl OKRustFutureboolWithOKSTFFIError {
    /// Cancel running task
    pub fn cancel(&self) { todo!() }
    /// Get error information (if any)
    pub fn get_error(&self) -> Option<OKSTFFIError> { todo!() }
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
    pub fn start_with_callback(&self, callback: Box<dyn OKRustFutureCallback>) -> Result<(), OKSTFFIError> { todo!() }
}

/// Use case for amending copy trading settings.
/// 
/// Provides UniFFI-exported methods for:
/// - Amend settings (Contract/Spot)
/// - Amend instruments (Contract/Spot)
/// - Position value check
/// - Update recommended copy mode
pub struct OKSTCoreAmendSettingsUseCase;
impl OKSTCoreAmendSettingsUseCase {
    /// Creates a new OKSTCoreAmendSettingsUseCase instance.
    pub fn new() -> Arc<OKSTCoreAmendSettingsUseCase> { todo!() }
    /// Amend copy instruments (Contract).
    /// 
    /// # Parameters
    /// - `request`: The amend instruments request
    /// 
    /// # Returns
    /// Returns Ok(true) if successful
    pub fn amend_instruments_contract(&self, request: AmendInstrumentsRequest) -> Arc<OKRustFutureboolWithOKSTFFIError> { todo!() }
    /// Amend copy instruments (Spot).
    /// 
    /// # Parameters
    /// - `request`: The amend instruments request
    /// 
    /// # Returns
    /// Returns Ok(true) if successful
    pub fn amend_instruments_spot(&self, request: AmendInstrumentsRequest) -> Arc<OKRustFutureboolWithOKSTFFIError> { todo!() }
    /// Amend copy trading settings (Contract).
    /// 
    /// # Parameters
    /// - `request`: The amend settings request
    /// 
    /// # Returns
    /// Returns Ok(true) if successful
    pub fn amend_settings_contract(&self, request: AmendSettingsRequest) -> Arc<OKRustFutureboolWithOKSTFFIError> { todo!() }
    /// Amend copy trading settings (Spot).
    /// 
    /// # Parameters
    /// - `request`: The amend settings request
    /// 
    /// # Returns
    /// Returns Ok(true) if successful
    pub fn amend_settings_spot(&self, request: AmendSettingsRequest) -> Arc<OKRustFutureboolWithOKSTFFIError> { todo!() }
    /// Check position value before amending settings.
    /// 
    /// # Parameters
    /// - `inst_type`: Instrument type (SWAP/SPOT)
    /// - `max_follow_amount`: Maximum follow amount
    /// - `copy_rel_id`: Copy relation ID
    /// 
    /// # Returns
    /// Returns Ok(true) if check passes
    pub fn position_value_check(&self, inst_type: String, max_follow_amount: String, copy_rel_id: String) -> Arc<OKRustFutureboolWithOKSTFFIError> { todo!() }
    /// Update recommended copy mode (for lead traders).
    /// 
    /// # Parameters
    /// - `inst_type`: Instrument type (SWAP/SPOT)
    /// - `recommended_copy_mode`: Recommended mode ("0"=Unrestricted, "1"=Smart, "2"=Custom)
    /// 
    /// # Returns
    /// Returns Ok(true) if successful
    pub fn update_recommended_copy_mode(&self, inst_type: String, recommended_copy_mode: String) -> Arc<OKRustFutureboolWithOKSTFFIError> { todo!() }
}

/// Use case for copytrader retention operations.
/// 
/// Handles the retention flow when copiers attempt to stop following a trader:
/// 1. Display retention popup with trader performance data
/// 2. Collect and submit cancellation reasons
pub struct OKSTCoreCopytraderRetentionUseCase;
impl OKSTCoreCopytraderRetentionUseCase {
    /// Creates a new use case instance with default dependencies.
    /// 
    /// Uses the global `DependencyContainer` to wire up the request center.
    pub fn new() -> Arc<OKSTCoreCopytraderRetentionUseCase> { todo!() }
    /// Checks if user has open positions before canceling copy trading.
    /// 
    /// This pre-check API should be called before canceling copy trading to
    /// determine if the user has open positions with the trader.
    /// 
    /// # Arguments
    /// 
    /// * `business_line` - Business line type (Contract or Spot) to determine endpoint
    /// * `unique_name` - Trader's unique identifier
    /// 
    /// # Returns
    /// 
    /// A cancellable `OKRustFuture` resolving to:
    /// - `Ok(CancelCopyPosResponse)` - Position check result
    /// - `has_position: true` - User has open positions, need to handle them
    /// - `has_position: false` - No positions, safe to cancel directly
    /// - `Err(NetworkFailed)` - Network request failed
    /// - `Err(ServerResponseError)` - Server returned a business error
    pub fn get_cancel_copy_trade_check(&self, business_line: CancelFollowBusinessLine, unique_name: String) -> Arc<OKRustFutureCancelCopyPosResponseWithOKSTFFIError> { todo!() }
    /// Fetches retention popup data when user attempts to cancel copying.
    /// 
    /// The popup displays trader performance metrics and other followers' results
    /// to encourage the user to continue copying.
    /// 
    /// # Arguments
    /// 
    /// * `biz_type` - Business type: `"futures"` or `"spot"`
    /// * `unique_name` - Trader's unique identifier
    /// * `pop_detail` - Popup detail level/type
    /// 
    /// # Returns
    /// 
    /// A cancellable `OKRustFuture` resolving to:
    /// - `Ok(Vec<CopierRetentionPopupResponse>)` - Retention popup data
    /// - `Err(NetworkFailed)` - Network request failed
    /// - `Err(JSONParseError)` - Response parsing failed
    /// - `Err(ServerResponseError)` - Server returned a business error
    pub fn get_cancel_copying_popup(&self, biz_type: String, unique_name: String, pop_detail: String) -> Arc<OKRustFutureCopierRetentionPopupResponseVecWithOKSTFFIError> { todo!() }
    /// Submits the reason for canceling copy trading.
    /// 
    /// Called after the user confirms cancellation and selects a reason.
    /// 
    /// # Arguments
    /// 
    /// * `body` - Cancellation details including trader ID, reason type, and optional text
    /// 
    /// # Returns
    /// 
    /// A cancellable `OKRustFuture` resolving to:
    /// - `Ok(true)` - Reason submitted successfully
    /// - `Err(NetworkFailed)` - Network request failed
    /// - `Err(ServerResponseError)` - Server returned a business error
    pub fn post_cancel_copying_reason(&self, body: CancelCopyingReasonBody) -> Arc<OKRustFutureboolWithOKSTFFIError> { todo!() }
    /// Cancels the copy trading relationship with a trader.
    /// 
    /// This operation stops copying the specified trader. If there are open positions,
    /// the `remain_mgn_deal` parameter specifies how to handle them.
    /// 
    /// # Arguments
    /// 
    /// * `business_line` - Business line type (Contract or Spot) to determine endpoint
    /// * `unique_name` - Trader's unique identifier
    /// * `remain_mgn_deal` - Optional position close type:
    /// - `None` or `"0"` - No open positions / default
    /// - `"1"` - Close at market price
    /// - `"2"` - Let trader close (follow trader)
    /// - `"3"` - Manual process (user will handle)
    /// 
    /// # Returns
    /// 
    /// A cancellable `OKRustFuture` resolving to:
    /// - `Ok(true)` - Cancel follow successful
    /// - `Err(NetworkFailed)` - Network request failed
    /// - `Err(ServerResponseError)` - Server returned a business error
    pub fn post_cancel_follow(&self, business_line: CancelFollowBusinessLine, unique_name: String, remain_mgn_deal: Option<String>) -> Arc<OKRustFutureboolWithOKSTFFIError> { todo!() }
}

/// Use case for eco leader tier operations.
/// 
/// Manages trader tier functionality:
/// - Fetch benefit details for different trader tiers
/// - Get upgrade popup information when tier changes
/// - Mark tier upgrade popups as read
pub struct OKSTCoreEcoLeaderTierUseCase;
impl OKSTCoreEcoLeaderTierUseCase {
    /// Creates a new use case instance with default dependencies.
    /// 
    /// Uses the global `DependencyContainer` to wire up the request center.
    pub fn new() -> Arc<OKSTCoreEcoLeaderTierUseCase> { todo!() }
    /// Fetches benefit details for trader tiers.
    /// 
    /// Returns information about benefits available at different trader tiers,
    /// including requirements, follower limits, and premium benefits.
    /// 
    /// # Returns
    /// 
    /// A cancellable `OKRustFuture` resolving to:
    /// - `Ok(Vec<BenefitDetail>)` - Benefit details for each tier
    /// - `Err(NetworkFailed)` - Network request failed
    /// - `Err(JSONParseError)` - Response parsing failed
    /// - `Err(ServerResponseError)` - Server returned a business error
    pub fn get_benefit_detail(&self) -> Arc<OKRustFutureBenefitDetailVecWithOKSTFFIError> { todo!() }
    /// Fetches upgrade popup information for tier changes.
    /// 
    /// Returns data needed to display tier upgrade notifications to the user.
    /// 
    /// # Returns
    /// 
    /// A cancellable `OKRustFuture` resolving to:
    /// - `Ok(Vec<UpgradePopupInfo>)` - Upgrade popup data
    /// - `Err(NetworkFailed)` - Network request failed
    /// - `Err(JSONParseError)` - Response parsing failed
    /// - `Err(ServerResponseError)` - Server returned a business error
    pub fn get_upgrade_popup_info(&self) -> Arc<OKRustFutureUpgradePopupInfoVecWithOKSTFFIError> { todo!() }
    /// Marks profit condition.
    /// 
    /// Call this to mark the profit condition with the specified version.
    /// 
    /// # Arguments
    /// 
    /// * `mark_condition_version` - The version of the mark condition
    /// 
    /// # Returns
    /// 
    /// A cancellable `OKRustFuture` resolving to:
    /// - `Ok(())` - Profit condition marked successfully
    /// - `Err(NetworkFailed)` - Network request failed
    /// - `Err(ServerResponseError)` - Server returned a business error
    pub fn mark_profit_condition(&self, mark_condition_version: String) -> Arc<OKRustFutureboolWithOKSTFFIError> { todo!() }
    /// Marks the trader tier popup as read.
    /// 
    /// Call this after displaying the tier upgrade popup to the user
    /// to prevent it from showing again.
    /// 
    /// # Returns
    /// 
    /// A cancellable `OKRustFuture` resolving to:
    /// - `Ok(true)` - Popup marked as read successfully
    /// - `Err(NetworkFailed)` - Network request failed
    /// - `Err(ServerResponseError)` - Server returned a business error
    pub fn read_trader_tier_popup(&self) -> Arc<OKRustFutureboolWithOKSTFFIError> { todo!() }
}

/// Use case for the copier side of Invite Copy Traders.
/// 
/// Exposes the two copier-facing operations as cancellable `OKRustFuture`s.
pub struct OKSTCoreInviteCopyTradersUseCase;
impl OKSTCoreInviteCopyTradersUseCase {
    /// Creates a new use case instance with default dependencies.
    /// 
    /// Uses the global `DependencyContainer` to wire up the request center.
    pub fn new() -> Arc<OKSTCoreInviteCopyTradersUseCase> { todo!() }
    /// Gets link info for an invite code.
    /// 
    /// Retrieves the trader information associated with an invite code. Used to
    /// validate an invite code and get the trader details before proceeding with
    /// the copy trading flow.
    /// 
    /// # Parameters
    /// 
    /// * `invite_code` - The invite code to validate
    /// 
    /// # Returns
    /// 
    /// A cancellable `OKRustFuture` resolving to:
    /// - `Ok(InviteCodeLinkInfo)` - The trader information for the invite code
    /// - `Err(ServerResponseError)` - Server returned a business error (invalid code, etc.)
    /// - `Err(NetworkFailed)` - Network request failed
    /// - `Err(JSONParseError)` - Response parsing failed
    pub fn getLinkInfo(&self, invite_code: String) -> Arc<OKRustFutureInviteCodeLinkInfoWithOKSTFFIError> { todo!() }
    /// Verifies a private copy trading invite link.
    /// 
    /// Validates whether the current user can use a specific private copy trading
    /// invite link. Checks invite type restrictions (direct invitees only, all
    /// invitees, etc.) and link validity (not revoked, not full).
    /// 
    /// # Parameters
    /// 
    /// * `trader_unique_name` - Unique identifier of the lead trader
    /// * `private_copy_link_id` - Optional ID of the private copy link
    /// * `invite_code` - Optional invite code for verification
    /// 
    /// # Returns
    /// 
    /// A cancellable `OKRustFuture` resolving to:
    /// - `Ok(true)` - Verification passed, user can use the invite link
    /// - `Err(ServerResponseError)` - Verification failed with specific error:
    /// - 60014: Link revoked
    /// - 60015: Link full (copier limit reached)
    /// - 60016: Only direct invitees allowed
    /// - 60017: Only invitees (direct or indirect) allowed
    /// - 59215: Copy relation already exists
    /// - 60057: Only direct or indirect invitees of this lead trader can use this code
    /// - `Err(NetworkFailed)` - Network request failed
    pub fn verify(&self, trader_unique_name: String, private_copy_link_id: Option<String>, invite_code: Option<String>) -> Arc<OKRustFutureboolWithOKSTFFIError> { todo!() }
}

/// Use case for fetching leader rank list and rank configuration.
/// 
/// Returns typed responses mirroring the ObjC/Swift DTOs:
/// - `OKTESquareAllModel` → `LeaderRankResponse`
/// - `OKGTESquareCellModel` → `LeaderRankItem`
/// - `OKTELeadTraderRankConfig` → `RankConfig`
/// 
/// # Endpoints
/// - GET /priapi/v5/ecotrade/public/follow-rank (Contract)
/// - GET /priapi/v5/ecotrade/public/spot/follow-rank (Spot)
/// - GET /priapi/v5/ecotrade/public/rank-config (Contract)
/// - GET /priapi/v5/ecotrade/public/spot/rank-config (Spot)
pub struct OKSTCoreLeaderRankUseCase;
impl OKSTCoreLeaderRankUseCase {
    /// Creates a new use case instance with default dependencies.
    /// 
    /// Uses the global `DependencyContainer` to wire up the request center.
    pub fn new() -> Arc<OKSTCoreLeaderRankUseCase> { todo!() }
    /// Fetches leader rank list and returns typed response.
    /// 
    /// Returns typed `LeaderRankResponse` mirroring the ObjC/Swift DTOs.
    /// 
    /// # Arguments
    /// 
    /// * `start` - Page number (1-based)
    /// * `size` - Page size
    /// * `data_version` - Data version for cache invalidation (optional)
    /// * `latest_num` - Time period filter (e.g., "7", "30", "90") (optional)
    /// * `order_type` - Order type filter (e.g., "pnl", "yield") (optional)
    /// * `country_id` - Country ID filter (optional)
    /// * `api_trader` - API trader filter (optional)
    /// * `full_state` - Full state filter (optional)
    /// * `inst_num_limit` - Instrument number limit (optional)
    /// * `business_line` - Trading type: `Contract` (futures) or `Spot`
    /// * `assets_low` - Minimum lead trader assets filter (optional)
    /// * `assets_high` - Maximum lead trader assets filter (optional)
    /// * `aum_low` - Minimum AUM filter (optional)
    /// * `aum_high` - Maximum AUM filter (optional)
    /// * `trader_period` - Duration as lead trader filter, e.g. "30", "60", "90" (optional)
    /// * `win_ratio` - Minimum win ratio filter (optional)
    /// 
    /// # Returns
    /// 
    /// A cancellable `OKRustFuture` resolving to:
    /// - `Ok(LeaderRankResponse)` - Typed response with data_version, pages, and ranks
    /// - `Err(NetworkFailed)` - Network request failed
    /// - `Err(JSONParseError)` - Response parsing failed
    /// - `Err(ServerResponseError)` - Server returned a business error
    pub fn fetch_leader_rank_list(&self, params: LeaderRankFetchParams) -> Arc<OKRustFutureLeaderRankResponseWithOKSTFFIError> { todo!() }
    /// Fetches rank configuration for filter sliders and options.
    /// 
    /// Always returns a valid `RankConfig` - returns default config on any error.
    /// This matches the native Swift behavior where errors are swallowed and
    /// a default configuration is returned.
    /// 
    /// # Arguments
    /// 
    /// * `business_line` - Trading type: `Contract` (futures) or `Spot`
    /// 
    /// # Returns
    /// 
    /// A cancellable `OKRustFuture` resolving to:
    /// - `Ok(RankConfig)` - Filter configuration (always succeeds, returns default on error)
    /// 
    /// # Example (Swift)
    /// 
    /// ```swift
    /// let useCase = OKSTCoreLeaderRankUseCase()
    /// let future = useCase.fetchRankConfig(businessLine: .contract)
    /// let config = try await OkRustFutureWrapper(future).exec()
    /// ```
    pub fn fetch_rank_config(&self, business_line: LeaderRankBusinessLine) -> Arc<OKRustFutureRankConfigWithOKSTFFIError> { todo!() }
}

/// Use case for ongoing lead trades operations.
/// 
/// Provides functionality for:
/// - **Position summaries**: Fetch position summaries with instrument type and access level filters
/// - **Instrument list**: Fetch available instruments for copy trading by business line
pub struct OKSTCoreOngoingLeadTradesUseCase;
impl OKSTCoreOngoingLeadTradesUseCase {
    /// Creates a new use case instance with default dependencies.
    /// 
    /// Uses the global `DependencyContainer` to wire up the request center.
    pub fn new() -> Arc<OKSTCoreOngoingLeadTradesUseCase> { todo!() }
    /// Fetches all available instruments for copy trading.
    /// 
    /// Returns the list of instruments available for copy trading based on the business line.
    /// 
    /// # Arguments
    /// 
    /// * `business_line` - Trading type: `Contract` (futures) or `Spot`
    /// 
    /// # Returns
    /// 
    /// A cancellable `OKRustFuture` resolving to:
    /// - `Ok(Vec<OngoingLeadTradesInstrumentInfo>)` - List of available instruments
    /// - `Err(NetworkFailed)` - Network request failed
    /// - `Err(JSONParseError)` - Response parsing failed
    /// - `Err(ServerResponseError)` - Server returned a business error
    pub fn fetch_instrument_list(&self, business_line: OngoingLeadTradesBusinessLine) -> Arc<OKRustFutureOngoingLeadTradesInstrumentInfoVecWithOKSTFFIError> { todo!() }
    /// Fetches position summaries for another trader (public endpoint).
    /// 
    /// Returns the list of aggregated positions for a specific lead trader.
    /// 
    /// # Arguments
    /// 
    /// * `inst_type` - Instrument type: `Swap` (futures), `Spot`, or `All`
    /// * `unique_name` - The unique identifier of the trader
    /// 
    /// # Returns
    /// 
    /// A cancellable `OKRustFuture` resolving to:
    /// - `Ok(Vec<OngoingLeadTradesPositionSummary>)` - List of position summaries
    /// - `Err(NetworkFailed)` - Network request failed
    /// - `Err(JSONParseError)` - Response parsing failed
    /// - `Err(ServerResponseError)` - Server returned a business error
    pub fn fetch_position_summary_public(&self, inst_type: OngoingLeadTradesInstType, unique_name: String) -> Arc<OKRustFutureOngoingLeadTradesPositionSummaryVecWithOKSTFFIError> { todo!() }
    /// Fetches position summaries for the current user's lead trades.
    /// 
    /// Returns the list of aggregated positions for the authenticated lead trader.
    /// 
    /// # Arguments
    /// 
    /// * `inst_type` - Instrument type: `Swap` (futures), `Spot`, or `All`
    /// 
    /// # Returns
    /// 
    /// A cancellable `OKRustFuture` resolving to:
    /// - `Ok(Vec<OngoingLeadTradesPositionSummary>)` - List of position summaries
    /// - `Err(NetworkFailed)` - Network request failed
    /// - `Err(JSONParseError)` - Response parsing failed
    /// - `Err(ServerResponseError)` - Server returned a business error
    pub fn fetch_position_summary_self(&self, inst_type: OngoingLeadTradesInstType) -> Arc<OKRustFutureOngoingLeadTradesPositionSummaryVecWithOKSTFFIError> { todo!() }
}

/// Profit Settlement Use Case
/// 
/// Provides business logic for profit settlement operations including:
/// - Profit info (contract/spot)
/// - Pre-profit (contract/spot)
/// - History profit (contract/spot)
/// - Profit detail (contract/spot)
/// - Profit sharing info (contract/spot)
/// - Update profit sharing ratio
pub struct OKSTCoreProfitSettlementUseCase;
impl OKSTCoreProfitSettlementUseCase {
    /// Create a new instance of OKSTCoreProfitSettlementUseCase
    /// 
    /// Uses the default DependencyContainer to create the request center.
    pub fn new() -> Arc<OKSTCoreProfitSettlementUseCase> { todo!() }
    /// Fetch contract history profit records
    /// 
    /// Gets historical profit settlement records.
    /// 
    /// # Parameters
    /// - `size`: Number of items to fetch
    /// - `settle_time`: Optional settlement time cursor for pagination
    /// 
    /// # Returns
    /// A cancellable future that resolves to a list of HistoryProfit or an error.
    pub fn get_history_profit(&self, size: i32, settle_time: Option<String>) -> Arc<OKRustFutureHistoryProfitVecWithOKSTFFIError> { todo!() }
    /// Fetch contract pre-profit (estimated pending profit)
    /// 
    /// Gets estimated profit to be settled and list of followers with their contributions.
    /// 
    /// # Parameters
    /// - `size`: Number of items to fetch
    /// - `id`: Optional pagination cursor (ID of last item from previous page)
    /// 
    /// # Returns
    /// A cancellable future that resolves to a list of PreProfit or an error.
    pub fn get_pre_profit(&self, size: i32, id: Option<String>) -> Arc<OKRustFuturePreProfitVecWithOKSTFFIError> { todo!() }
    /// Fetch profit curve data
    /// 
    /// Gets profit curve data points for charting.
    /// 
    /// # Parameters
    /// - `unique_name`: Trader's unique identifier
    /// 
    /// # Returns
    /// A cancellable future that resolves to a list of ProfitCurve or an error.
    pub fn get_profit_curve(&self, unique_name: String) -> Arc<OKRustFutureProfitCurveVecWithOKSTFFIError> { todo!() }
    /// Fetch contract profit detail for a specific settlement period
    /// 
    /// Gets individual follower contributions to a profit settlement.
    /// 
    /// # Parameters
    /// - `settle_time`: Settlement time to query details for
    /// - `size`: Number of items to fetch
    /// - `before`: Optional pagination cursor (before this ID)
    /// - `after`: Optional pagination cursor (after this ID)
    /// 
    /// # Returns
    /// A cancellable future that resolves to a list of ProfitDetail or an error.
    pub fn get_profit_detail(&self, settle_time: String, size: i32, before: Option<String>, after: Option<String>) -> Arc<OKRustFutureProfitDetailVecWithOKSTFFIError> { todo!() }
    /// Fetch contract profit info
    /// 
    /// Gets profit sharing ratio, latest/total profit, and follower statistics
    /// for contract trading.
    /// 
    /// # Returns
    /// A cancellable future that resolves to a list of ProfitInfo or an error.
    pub fn get_profit_info(&self) -> Arc<OKRustFutureProfitInfoVecWithOKSTFFIError> { todo!() }
    /// Fetch contract profit sharing info
    /// 
    /// Gets AUM level, profit sharing ratio, and trader profile information.
    /// 
    /// # Returns
    /// A cancellable future that resolves to a list of ProfitSharingInfo or an error.
    pub fn get_profit_sharing_info(&self) -> Arc<OKRustFutureProfitSharingInfoVecWithOKSTFFIError> { todo!() }
    /// Fetch profit sharing ratio limit
    /// 
    /// Gets the min/max profit sharing ratio limits for an instrument type.
    /// 
    /// # Parameters
    /// - `inst_type`: Instrument type ("SWAP" for contract, "SPOT" for spot)
    /// 
    /// # Returns
    /// A cancellable future that resolves to a list of SettlementRatioLimit or an error.
    pub fn get_profit_sharing_ratio_limit(&self, inst_type: String) -> Arc<OKRustFutureSettlementRatioLimitVecWithOKSTFFIError> { todo!() }
    /// Fetch spot history profit records
    /// 
    /// Gets historical profit settlement records.
    /// 
    /// # Parameters
    /// - `size`: Number of items to fetch
    /// - `settle_time`: Optional settlement time cursor for pagination
    /// 
    /// # Returns
    /// A cancellable future that resolves to a list of HistoryProfit or an error.
    pub fn get_spot_history_profit(&self, size: i32, settle_time: Option<String>) -> Arc<OKRustFutureHistoryProfitVecWithOKSTFFIError> { todo!() }
    /// Fetch spot pre-profit (estimated pending profit)
    /// 
    /// Gets estimated profit to be settled and list of followers with their contributions.
    /// 
    /// # Parameters
    /// - `size`: Number of items to fetch
    /// - `id`: Optional pagination cursor (ID of last item from previous page)
    /// 
    /// # Returns
    /// A cancellable future that resolves to a list of PreProfit or an error.
    pub fn get_spot_pre_profit(&self, size: i32, id: Option<String>) -> Arc<OKRustFuturePreProfitVecWithOKSTFFIError> { todo!() }
    /// Fetch spot profit detail for a specific settlement period
    /// 
    /// Gets individual follower contributions to a profit settlement.
    /// 
    /// # Parameters
    /// - `settle_time`: Settlement time to query details for
    /// - `size`: Number of items to fetch
    /// - `before`: Optional pagination cursor (before this ID)
    /// - `after`: Optional pagination cursor (after this ID)
    /// 
    /// # Returns
    /// A cancellable future that resolves to a list of ProfitDetail or an error.
    pub fn get_spot_profit_detail(&self, settle_time: String, size: i32, before: Option<String>, after: Option<String>) -> Arc<OKRustFutureProfitDetailVecWithOKSTFFIError> { todo!() }
    /// Fetch spot profit info
    /// 
    /// Gets profit sharing ratio, latest/total profit, and follower statistics
    /// for spot trading.
    /// 
    /// # Returns
    /// A cancellable future that resolves to a list of ProfitInfo or an error.
    pub fn get_spot_profit_info(&self) -> Arc<OKRustFutureProfitInfoVecWithOKSTFFIError> { todo!() }
    /// Fetch spot profit sharing info
    /// 
    /// Gets AUM level, profit sharing ratio, and trader profile information.
    /// 
    /// # Returns
    /// A cancellable future that resolves to a list of ProfitSharingInfo or an error.
    pub fn get_spot_profit_sharing_info(&self) -> Arc<OKRustFutureProfitSharingInfoVecWithOKSTFFIError> { todo!() }
    /// Update profit sharing ratio
    /// 
    /// Updates the profit sharing ratio for the trader.
    /// 
    /// # Parameters
    /// - `inst_type`: Instrument type ("SWAP" for contract, "SPOT" for spot)
    /// - `profit_sharing_ratio`: New profit sharing ratio as a FRACTION string
    /// (e.g. "0.15" means 15%) — the same fractional representation the load
    /// paths return and both native callers already compute.
    /// - `lead_mode`: Lead mode — "1" for public, "2" for private
    /// (matches `LeadMode { public = 1, private = 2 }` on both platforms and
    /// what `do_update_ratio` in the interactor sends).
    /// 
    /// # Returns
    /// A cancellable future that resolves to true on success or an error.
    pub fn update_profit_sharing_ratio(&self, inst_type: String, profit_sharing_ratio: String, lead_mode: String) -> Arc<OKRustFutureboolWithOKSTFFIError> { todo!() }
}

/// Use case for social trading public info operations.
/// 
/// Provides methods to fetch publicly available information:
/// - User portraits for displaying avatars
/// - Trader's leading coins/instruments
/// - Profit sharing ratios for traders
/// - Copy trading order coupon flags
pub struct OKSTCoreSocialTradingUseCase;
impl OKSTCoreSocialTradingUseCase {
    /// Creates a new use case instance with default dependencies.
    /// 
    /// Uses the global `DependencyContainer` to wire up the request center.
    pub fn new() -> Arc<OKSTCoreSocialTradingUseCase> { todo!() }
    /// Fetches coupon/compensation ticket flags for copy trading order items.
    /// 
    /// Gets the compensation ticket information for ongoing positions.
    /// Used to display voucher/bonus indicators on trade positions.
    /// 
    /// # Arguments
    /// 
    /// * `item_id_list` - Comma-separated list of item IDs (posId or tradeItemId)
    /// 
    /// # Returns
    /// 
    /// A cancellable `OKRustFuture` resolving to:
    /// - `Ok(Vec<CopyTradingOrderCouponFlag>)` - Coupon flag info for each item
    /// - `Err(NetworkFailed)` - Network request failed
    /// - `Err(JSONParseError)` - Response parsing failed
    /// - `Err(ServerResponseError)` - Server returned a business error
    /// 
    /// # Example (Swift)
    /// ```swift
    /// let useCase = OkstCoreSocialTradingUseCase()
    /// let future = useCase.getItemCouponFlags("12345,67890")
    /// let result = try await OkRustFutureWrapper(future).exec()
    /// ```
    pub fn get_item_coupon_flags(&self, item_id_list: String) -> Arc<OKRustFutureCopyTradingOrderCouponFlagVecWithOKSTFFIError> { todo!() }
    /// Fetches public profit sharing ratio for a trader.
    /// 
    /// Gets the profit sharing ratio configuration for a specific trader.
    /// 
    /// # Arguments
    /// 
    /// * `unique_name` - The unique identifier of the trader
    /// 
    /// # Returns
    /// 
    /// A cancellable `OKRustFuture` resolving to:
    /// - `Ok(Vec<PublicProfitSharingRatio>)` - Profit sharing ratio info (usually single item)
    /// - `Err(NetworkFailed)` - Network request failed
    /// - `Err(JSONParseError)` - Response parsing failed
    /// - `Err(ServerResponseError)` - Server returned a business error
    /// 
    /// # Example (Kotlin)
    /// ```kotlin
    /// val useCase = OkstCoreSocialTradingUseCase()
    /// val future = useCase.getPublicProfitSharingRatio("UNIQUE123")
    /// val result = OkRustFutureWrapper(...).exec()
    /// ```
    pub fn get_public_profit_sharing_ratio(&self, unique_name: String) -> Arc<OKRustFuturePublicProfitSharingRatioVecWithOKSTFFIError> { todo!() }
    /// Fetches trader's leading coins/instruments.
    /// 
    /// Gets the instruments/coins that a trader is leading (copy trading).
    /// 
    /// # Arguments
    /// 
    /// * `unique_name` - The unique identifier of the trader
    /// * `biz_type` - Business type: "SWAP" for contract, "SPOT" for spot trading
    /// 
    /// # Returns
    /// 
    /// A cancellable `OKRustFuture` resolving to:
    /// - `Ok(Vec<PublicInstrumentInfo>)` - List of instrument info
    /// - `Err(NetworkFailed)` - Network request failed
    /// - `Err(JSONParseError)` - Response parsing failed
    /// - `Err(ServerResponseError)` - Server returned a business error
    /// 
    /// # Example (Kotlin)
    /// ```kotlin
    /// val useCase = OkstCoreSocialTradingUseCase()
    /// val future = useCase.getTraderLeadingCoins("UNIQUE123", "SWAP")
    /// val result = OkRustFutureWrapper(...).exec()
    /// ```
    pub fn get_trader_leading_coins(&self, unique_name: String, biz_type: String) -> Arc<OKRustFuturePublicInstrumentInfoVecWithOKSTFFIError> { todo!() }
}

/// Use case for checking social trading server status.
/// 
/// This is a simple health check that verifies the server is available
/// before proceeding with other operations.
pub struct OKSTCoreSystemStatusUseCase;
impl OKSTCoreSystemStatusUseCase {
    /// Creates a new use case instance with default dependencies.
    /// 
    /// Uses the global `DependencyContainer` to wire up the request center.
    pub fn new() -> Arc<OKSTCoreSystemStatusUseCase> { todo!() }
    /// Checks if the social trading server is valid and available.
    /// 
    /// This should be called before making other social trading API requests
    /// to ensure the server is reachable.
    /// 
    /// # Returns
    /// 
    /// A cancellable `OKRustFuture` resolving to:
    /// - `Ok(SystemStatusResult)` - Server is valid and available
    /// - `Err(NetworkFailed)` - Network request failed (server unavailable)
    /// - `Err(ServerResponseError)` - Server returned a business error
    /// 
    /// # Example (Swift)
    /// 
    /// ```swift
    /// let useCase = OKSTCoreSystemStatusUseCase()
    /// let future = useCase.checkServerStatus()
    /// let wrapper = OkRustFutureWrapper(
    /// start: future.startWithCallback,
    /// cancel: future.cancel,
    /// getError: future.getError,
    /// getResult: { future.getSuccessResult() }
    /// )
    /// let result = try await wrapper.exec()
    /// // result.isValid is true if server is available
    /// ```
    pub fn check_server_status(&self) -> Arc<OKRustFutureSystemStatusResultWithOKSTFFIError> { todo!() }
}

/// Use case for fetching trader follower lists.
/// 
/// Retrieves follower information for traders with support for:
/// - **Business lines**: Contract (futures) or Spot trading
/// - **Access levels**: Public (no auth) or Account (authenticated)
pub struct OKSTCoreTraderFollowerListUseCase;
impl OKSTCoreTraderFollowerListUseCase {
    /// Creates a new use case instance with default dependencies.
    /// 
    /// Uses the global `DependencyContainer` to wire up the request center.
    pub fn new() -> Arc<OKSTCoreTraderFollowerListUseCase> { todo!() }
    /// Fetches account follower list with pagination (authenticated user's own followers).
    /// 
    /// Returns follower statistics and list for the authenticated user's own followers
    /// with support for pagination and sorting.
    /// 
    /// # Arguments
    /// 
    /// * `page` - Page number (e.g., `"1"`)
    /// * `size` - Page size (e.g., `"20"`)
    /// * `sort` - Optional sort order: `"asc"` or `"desc"` (defaults to no sorting if None)
    /// * `business_line` - Trading type: `Contract` (futures) or `Spot`
    /// 
    /// # Returns
    /// 
    /// A cancellable `OKRustFuture` resolving to:
    /// - `Ok(TraderFollowerList)` - Follower statistics and list
    /// - `Err(NetworkFailed)` - Network request failed
    /// - `Err(JSONParseError)` - Response parsing failed
    /// - `Err(ServerResponseError)` - Server returned a business error
    pub fn fetch_my_followers_paginated(&self, page: String, size: String, sort: Option<String>, business_line: BusinessLine) -> Arc<OKRustFutureTraderFollowerListWithOKSTFFIError> { todo!() }
    /// Fetches public trader follower list (no authentication required).
    /// 
    /// Returns follower statistics and list from public API endpoints.
    /// 
    /// # Arguments
    /// 
    /// * `unique_name` - Trader's unique identifier
    /// * `size` - Page size (e.g., `"100"`)
    /// * `business_line` - Trading type: `Contract` (futures) or `Spot`
    /// 
    /// # Returns
    /// 
    /// A cancellable `OKRustFuture` resolving to:
    /// - `Ok(TraderFollowerList)` - Follower statistics and list
    /// - `Err(NetworkFailed)` - Network request failed
    /// - `Err(JSONParseError)` - Response parsing failed
    /// - `Err(ServerResponseError)` - Server returned a business error
    pub fn fetch_public_follower_list(&self, unique_name: String, size: String, business_line: BusinessLine) -> Arc<OKRustFutureTraderFollowerListWithOKSTFFIError> { todo!() }
}

/// Use case for fetching trader trade performance statistics.
/// 
/// Retrieves trade performance data for traders with support for:
/// - **Business types**: Contract (futures/SWAP) or Spot trading
/// - **Access levels**: Public (no auth) or Account (authenticated)
pub struct OKSTCoreTraderTradePerformanceUseCase;
impl OKSTCoreTraderTradePerformanceUseCase {
    /// Creates a new use case instance with default dependencies.
    /// 
    /// Uses the global `DependencyContainer` to wire up the request center.
    pub fn new() -> Arc<OKSTCoreTraderTradePerformanceUseCase> { todo!() }
    /// Fetches account trader trade performance statistics (authentication required).
    /// 
    /// Returns trade performance data from authenticated API endpoints.
    /// Used when viewing own profile.
    /// 
    /// # Arguments
    /// 
    /// * `latest_num` - Filter period value (7, 30, 90, or 0 for 1 year)
    /// * `biz_type` - Trading type: `Contract` (futures/SWAP) or `Spot`
    /// 
    /// # Returns
    /// 
    /// A cancellable `OKRustFuture` resolving to:
    /// - `Ok(TraderTradePerformanceStats)` - Trade performance statistics
    /// - `Err(NetworkFailed)` - Network request failed
    /// - `Err(JSONParseError)` - Response parsing failed
    /// - `Err(ServerResponseError)` - Server returned a business error
    pub fn fetch_account_trade_performance(&self, latest_num: String, biz_type: TraderTradePerformanceBizType) -> Arc<OKRustFutureTraderTradePerformanceStatsWithOKSTFFIError> { todo!() }
    /// Fetches public trader trade performance statistics (no authentication required).
    /// 
    /// Returns trade performance data from public API endpoints.
    /// 
    /// # Arguments
    /// 
    /// * `unique_name` - Trader's unique identifier
    /// * `latest_num` - Filter period value (7, 30, 90, or 0 for 1 year)
    /// * `biz_type` - Trading type: `Contract` (futures/SWAP) or `Spot`
    /// 
    /// # Returns
    /// 
    /// A cancellable `OKRustFuture` resolving to:
    /// - `Ok(TraderTradePerformanceStats)` - Trade performance statistics
    /// - `Err(NetworkFailed)` - Network request failed
    /// - `Err(JSONParseError)` - Response parsing failed
    /// - `Err(ServerResponseError)` - Server returned a business error
    pub fn fetch_public_trade_performance(&self, unique_name: String, latest_num: String, biz_type: TraderTradePerformanceBizType) -> Arc<OKRustFutureTraderTradePerformanceStatsWithOKSTFFIError> { todo!() }
}

/// Use case for user profile operations
/// 
/// This use case encapsulates the business logic for retrieving user profile overview
/// information. It follows Clean Architecture principles by depending on abstractions
/// (UserProfileRequestCenterTrait) rather than concrete implementations.
pub struct OKSTCoreUserProfileUseCase;
impl OKSTCoreUserProfileUseCase {
    /// Create a new UserProfileUseCase with default dependencies
    /// 
    /// This constructor uses the global DependencyContainer to wire up dependencies
    /// and delegates to the canonical `new_with_request` constructor.
    /// 
    /// # Returns
    /// * `Arc<Self>` - Shared reference to the use case instance
    pub fn new() -> Arc<OKSTCoreUserProfileUseCase> { todo!() }
    /// Get profile chart data
    /// 
    /// # Arguments
    /// * `unique_name` - Unique name of the user
    /// * `chart_type` - Chart type (PNL, PNL_RATIO, EQUITY)
    /// * `time_span` - Optional time span in days
    /// * `preview` - Preview mode flag
    /// 
    /// # Returns
    /// * `OKRustFuture` - Cancellable future that resolves to Result<Vec<ProfileChart>, OKSTFFIError>
    /// - Ok(charts) with list of profile charts
    /// - Err(NetworkFailed) if network request fails
    /// - Err(JSONParseError) if response parsing fails
    /// - Err(ServerResponseError) if server returns business error
    pub fn get_charts_data(&self, unique_name: String, chart_type: ProfileChartType, time_span: Option<i32>, preview: bool) -> Arc<OKRustFutureProfileChartVecWithOKSTFFIError> { todo!() }
    /// Get current future position
    /// 
    /// # Arguments
    /// * `unique_name` - Unique name of the user
    /// 
    /// # Returns
    /// * `OKRustFuture` - Cancellable future that resolves to Result<Vec<CurrentPosition>, OKSTFFIError>
    /// - Ok(positions) with list of current positions
    /// - Err(NetworkFailed) if network request fails
    /// - Err(JSONParseError) if response parsing fails
    /// - Err(ServerResponseError) if server returns business error
    pub fn get_current_future_position(&self, unique_name: String) -> Arc<OKRustFutureCurrentPositionVecWithOKSTFFIError> { todo!() }
    /// Get curve reset information
    /// 
    /// # Returns
    /// * `OKRustFuture` - Cancellable future that resolves to Result<Vec<CurveResetInfo>, OKSTFFIError>
    /// - Ok(infos) with list of curve reset info
    /// - Err(NetworkFailed) if network request fails
    /// - Err(JSONParseError) if response parsing fails
    /// - Err(ServerResponseError) if server returns business error
    pub fn get_curve_reset_info(&self) -> Arc<OKRustFutureCurveResetInfoVecWithOKSTFFIError> { todo!() }
    /// Get history future position
    /// 
    /// # Arguments
    /// * `unique_name` - Unique name of the user
    /// * `limit` - Optional limit for pagination
    /// * `after` - Optional pagination cursor (after)
    /// * `before` - Optional pagination cursor (before)
    /// * `inst_type` - Optional instrument type
    /// * `inst_id` - Optional instrument ID
    /// * `sort_type` - Optional sort type ("1" for recently closed, "4" for highest profit ratio, "5" for lowest profit ratio)
    /// 
    /// # Returns
    /// * `OKRustFuture` - Cancellable future that resolves to Result<Vec<ProfilePositionHistory>, OKSTFFIError>
    /// - Ok(history) with list of position history
    /// - Err(NetworkFailed) if network request fails
    /// - Err(JSONParseError) if response parsing fails
    /// - Err(ServerResponseError) if server returns business error
    pub fn get_history_future_position(&self, unique_name: String, limit: Option<String>, after: Option<String>, before: Option<String>, inst_type: Option<String>, inst_id: Option<String>, sort_type: Option<String>) -> Arc<OKRustFutureProfilePositionHistoryVecWithOKSTFFIError> { todo!() }
    /// Get latest trade records
    /// 
    /// # Arguments
    /// * `unique_name` - Unique name of the user
    /// * `inst_id` - Optional instrument ID
    /// * `inst_type` - Optional instrument type
    /// * `after` - Optional pagination cursor (after)
    /// * `before` - Optional pagination cursor (before)
    /// * `start_modify` - Optional start modify time
    /// * `end_modify` - Optional end modify time
    /// * `limit` - Optional limit for pagination
    /// * `page` - Optional page number
    /// 
    /// # Returns
    /// * `OKRustFuture` - Cancellable future that resolves to Result<Vec<LatestRecord>, OKSTFFIError>
    /// - Ok(records) with list of latest trade records
    /// - Err(NetworkFailed) if network request fails
    /// - Err(JSONParseError) if response parsing fails
    /// - Err(ServerResponseError) if server returns business error
    pub fn get_latest_records(&self, unique_name: String, inst_id: Option<String>, inst_type: Option<String>, after: Option<String>, before: Option<String>, start_modify: Option<String>, end_modify: Option<String>, limit: Option<i32>, page: Option<i32>) -> Arc<OKRustFutureLatestRecordVecWithOKSTFFIError> { todo!() }
    /// Get user profile overview information
    /// 
    /// # Arguments
    /// * `unique_name` - Unique name of the user
    /// 
    /// # Returns
    /// * `OKRustFuture` - Cancellable future that resolves to Result<Vec<UserProfileOverview>, OKSTFFIError>
    /// - Ok(overviews) with list of user profile overviews
    /// - Err(NetworkFailed) if network request fails
    /// - Err(JSONParseError) if response parsing fails
    /// - Err(ServerResponseError) if server returns business error
    pub fn get_user_profile_overview(&self, unique_name: String) -> Arc<OKRustFutureUserProfileOverviewVecWithOKSTFFIError> { todo!() }
}

/// Use case for fetching yield PnL chart data.
/// 
/// Retrieves yield/PnL chart data for traders with support for:
/// - **Business lines**: Contract (futures) or Spot trading
/// - **Access levels**: Public (no auth) or Account (authenticated)
/// - **Time periods**: 7 days, 30 days, 90 days, or all data
pub struct OKSTCoreYieldPnlChartDataUseCase;
impl OKSTCoreYieldPnlChartDataUseCase {
    /// Creates a new use case instance with default dependencies.
    /// 
    /// Uses the global `DependencyContainer` to wire up the request center.
    pub fn new() -> Arc<OKSTCoreYieldPnlChartDataUseCase> { todo!() }
    /// Fetches account yield PnL chart data (authentication required).
    /// 
    /// Returns yield/PnL chart data from authenticated API endpoints.
    /// Used for viewing the logged-in user's own data.
    /// 
    /// # Arguments
    /// 
    /// * `latest_num` - Number of days to fetch ("7", "30", "90", or "0" for all)
    /// * `business_line` - Trading type: `Contract` (futures) or `Spot`
    /// 
    /// # Returns
    /// 
    /// A cancellable `OKRustFuture` resolving to:
    /// - `Ok(Vec<YieldPnlChartDataItem>)` - List of chart data items
    /// - `Err(NetworkFailed)` - Network request failed
    /// - `Err(JSONParseError)` - Response parsing failed
    /// - `Err(ServerResponseError)` - Server returned a business error
    pub fn fetch_account_chart_data(&self, latest_num: String, business_line: YieldPnlBusinessLine) -> Arc<OKRustFutureYieldPnlChartDataItemVecWithOKSTFFIError> { todo!() }
    /// Fetches public yield PnL chart data (no authentication required).
    /// 
    /// Returns yield/PnL chart data from public API endpoints.
    /// Requires the trader's unique name.
    /// 
    /// # Arguments
    /// 
    /// * `unique_name` - Trader's unique identifier
    /// * `latest_num` - Number of days to fetch ("7", "30", "90", or "0" for all)
    /// * `business_line` - Trading type: `Contract` (futures) or `Spot`
    /// 
    /// # Returns
    /// 
    /// A cancellable `OKRustFuture` resolving to:
    /// - `Ok(Vec<YieldPnlChartDataItem>)` - List of chart data items
    /// - `Err(NetworkFailed)` - Network request failed
    /// - `Err(JSONParseError)` - Response parsing failed
    /// - `Err(ServerResponseError)` - Server returned a business error
    pub fn fetch_public_chart_data(&self, unique_name: String, latest_num: String, business_line: YieldPnlBusinessLine) -> Arc<OKRustFutureYieldPnlChartDataItemVecWithOKSTFFIError> { todo!() }
}

/// Use case for order operations
/// 
/// Provides methods for managing copy trading orders including
/// amending, creating algo orders, querying, and canceling.
/// 
/// # Example (Swift)
/// ```swift
/// let useCase = OkstCoreOrderOperationsUseCase()
/// 
/// // Amend order
/// let amendRequest = OrderOperationsAmendOrderRequest(
/// ordId: "ORDER123",
/// tradeItemId: "TRADE456",
/// px: "50000"
/// )
/// let future = useCase.amendOrder(request: amendRequest)
/// try await OkRustFutureWrapper(future).execute()
/// 
/// // Query order
/// let queryFuture = useCase.queryOrder(tradeItemId: "TRADE123")
/// let orders = try await OkRustFutureWrapper(queryFuture).execute()
/// ```
pub struct OkstCoreOrderOperationsUseCase;
impl OkstCoreOrderOperationsUseCase {
    /// Creates a new use case instance with default dependencies
    /// 
    /// Uses the global `DependencyContainer` to wire up the request center.
    pub fn new() -> Arc<OkstCoreOrderOperationsUseCase> { todo!() }
    /// Amend/edit an existing order
    /// 
    /// Modifies the price of an existing copy trade order.
    /// 
    /// # Arguments
    /// * `request` - The amend order request containing ordId, tradeItemId, and optional px
    /// 
    /// # Returns
    /// * `Ok(true)` - Order amended successfully
    /// * `Err(NetworkFailed)` - Network request failed
    /// * `Err(ServerResponseError)` - Server returned a business error
    pub fn amend_order(&self, request: OrderOperationsAmendOrderRequest) -> Arc<OKRustFutureboolWithOKSTFFIError> { todo!() }
    /// Cancel an existing order
    /// 
    /// Cancels a pending order in spot copy trading.
    /// 
    /// # Arguments
    /// * `request` - The cancel order request containing ordId and instId
    /// 
    /// # Returns
    /// * `Ok(true)` - Order cancelled successfully
    /// * `Err(NetworkFailed)` - Network request failed
    /// * `Err(ServerResponseError)` - Server returned a business error
    pub fn cancel_order(&self, request: OrderOperationsCancelOrderRequest) -> Arc<OKRustFutureboolWithOKSTFFIError> { todo!() }
    /// Create an algo order (Take Profit / Stop Loss)
    /// 
    /// Sets up automatic take profit and stop loss orders for spot copy trading.
    /// 
    /// # Arguments
    /// * `request` - The algo order request with TP/SL settings
    /// 
    /// # Returns
    /// * `Ok(true)` - Algo order created successfully
    /// * `Err(NetworkFailed)` - Network request failed
    /// * `Err(ServerResponseError)` - Server returned a business error
    pub fn create_algo_order(&self, request: OrderOperationsAlgoOrderRequest) -> Arc<OKRustFutureboolWithOKSTFFIError> { todo!() }
    /// Query order details
    /// 
    /// Retrieves information about pending/executed orders for a trade item.
    /// 
    /// # Arguments
    /// * `trade_item_id` - The trade item ID to query
    /// 
    /// # Returns
    /// * `Ok(Vec<OrderOperationsQueryOrderResult>)` - List of order details
    /// * `Err(NetworkFailed)` - Network request failed
    /// * `Err(ServerResponseError)` - Server returned a business error
    pub fn query_order(&self, trade_item_id: String) -> Arc<OKRustFutureQueryOrderResultVecWithOKSTFFIError> { todo!() }
}

/// Use case for currency preference chart data operations
/// 
/// This use case encapsulates the business logic for retrieving currency preference
/// chart data. It follows Clean Architecture principles by depending on
/// abstractions (PreferenceCoinRequestCenterTrait) rather than concrete implementations.
pub struct OkstCorePreferenceCoinUseCase;
impl OkstCorePreferenceCoinUseCase {
    /// Create a new OkstCorePreferenceCoinUseCase with default dependencies
    /// 
    /// This constructor uses the global DependencyContainer to wire up dependencies
    /// and delegates to the canonical `new_with_request` constructor.
    /// 
    /// # Returns
    /// * `Arc<Self>` - Shared reference to the use case instance
    pub fn new() -> Arc<OkstCorePreferenceCoinUseCase> { todo!() }
    /// Fetch account currency preference chart data (for viewing own data)
    /// 
    /// # Arguments
    /// * `business_line` - Business line (contract or spot)
    /// 
    /// # Returns
    /// * `OKRustFuture` - Cancellable future that resolves to Result<Vec<PreferenceCoinItem>, OKSTFFIError>
    /// - Ok(data) with list of currency preference chart data items
    /// - Err(NetworkFailed) if network request fails
    /// - Err(JSONParseError) if response parsing fails
    /// - Err(ServerResponseError) if server returns business error
    pub fn fetch_account_chart_data(&self, business_line: PreferenceCoinBusinessLine) -> Arc<OKRustFuturePreferenceCoinItemVecWithOKSTFFIError> { todo!() }
    /// Fetch public currency preference chart data (for viewing other traders)
    /// 
    /// # Arguments
    /// * `unique_name` - Unique name of the trader
    /// * `business_line` - Business line (contract or spot)
    /// 
    /// # Returns
    /// * `OKRustFuture` - Cancellable future that resolves to Result<Vec<PreferenceCoinItem>, OKSTFFIError>
    /// - Ok(data) with list of currency preference chart data items
    /// - Err(NetworkFailed) if network request fails
    /// - Err(JSONParseError) if response parsing fails
    /// - Err(ServerResponseError) if server returns business error
    pub fn fetch_public_chart_data(&self, unique_name: String, business_line: PreferenceCoinBusinessLine) -> Arc<OKRustFuturePreferenceCoinItemVecWithOKSTFFIError> { todo!() }
}

/// Ongoing Lead Trades Interactor
/// 
/// Manages position summaries, details, and instruments for lead traders.
pub struct OngoingLeadTradesInteractor;
impl OngoingLeadTradesInteractor {
    /// Close a custom / lead-trade position. Mirrors native
    /// `ClosePositionUseCase.Input.CustomPosition` →
    /// `POST /priapi/v5/ecotrade/close-copy-order`. Emits
    /// [`OngoingLeadTradesUiEvent::CustomCloseSucceeded`] /
    /// [`OngoingLeadTradesUiEvent::CustomCloseFailed`] on completion.
    /// The interactor does NOT optimistically mutate the displayed list —
    /// the WS push (`subpositions` / `sum-subpositions`) drives the
    /// post-close refresh, matching native behavior.
    pub fn close_custom_position(&self, input: CloseCopyOrderInput) { todo!() }
    /// Get the trader's unique name (None for self mode).
    pub fn get_unique_name(&self) -> Option<String> { todo!() }
    /// Get the current view mode.
    pub fn get_view_mode(&self) -> OngoingLeadTradesViewMode { todo!() }
    /// Load available instruments for copy trading (self mode only).
    pub fn load_instruments(&self, business_line: OngoingLeadTradesBusinessLine) { todo!() }
    /// Load position detail.
    pub fn load_position_detail(&self, inst_filter: OngoingLeadTradesInstFilter) { todo!() }
    /// Load position summary.
    pub fn load_position_summary(&self, inst_filter: OngoingLeadTradesInstFilter) { todo!() }
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
    /// Track which order's PnL bottom sheet is currently open.
    /// Mirrors native `OrderListViewModel.currentPnlId` (line 169).
    /// Used by the WS update path to know which order should trigger live updates.
    /// Pass `None` when the sheet is closed.
    pub fn set_current_pnl_id(&self, pnl_id: Option<String>) { todo!() }
    /// Enable or disable debug logging.
    /// 
    /// In release builds this is a no-op, but the symbol must exist
    /// so that UniFFI bindings generated in debug mode continue to
    /// link successfully against the release library.
    pub fn set_debug(&self, _enabled: bool) { todo!() }
    /// Set the instrument-id allow-list filter. Mirrors native
    /// `OrderListViewModel.Action.OnFiltersChanged.instIds`. Empty vec = no filter.
    /// Recomputes both displayed lists.
    pub fn set_inst_id_filter(&self, inst_ids: Vec<String>) { todo!() }
    /// Toggle "show only current trading pair". Mirrors native
    /// `OrderListViewModel.Action.OnSortChanged.showCurrentPairOnly`.
    /// `current_inst_id` / `current_inst_type` must come from the bridge
    /// (read from `SelectBizManager.getCurrentBizInstrument()` natively).
    /// Recomputes both displayed lists.
    pub fn set_show_current_pair_only(&self, enabled: bool, current_inst_id: String, current_inst_type: String) { todo!() }
    /// Change sort mode. Mirrors native `OrderListViewModel.Action.OnSortChanged.sortMode`
    /// (OrderLeadSortUtils.kt). Recomputes both displayed lists.
    pub fn set_sort_mode(&self, sort_mode: OngoingLeadTradesSortMode) { todo!() }
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
    pub fn set_ui_event_observer(&self, observer: Arc<dyn OngoingLeadTradesUiEventObserver>) { todo!() }
    /// Switch active tab (detail vs summary). Mirrors native
    /// `OrderListViewModel.Action.OnTabChanged.isDetail`.
    pub fn set_view_tab(&self, is_detail: bool) { todo!() }
    /// Start the interactor lifecycle.
    /// 
    /// Calls the user's `InteractorLifecycle::start()` implementation.
    /// Override that method to add custom startup behavior (e.g., initial data loads).
    pub fn start(&self) { todo!() }
    /// Subscribe to live position updates via WebSocket (subpositions + sum-subpositions
    /// channels). Mirrors native `OrderListViewModel.startWs()` → `manager.webSocketFlow()`.
    /// Bridge SHOULD only call this in self/owner mode (native gates on `isOwner` at
    /// OrderListViewModel.kt:339). Calling twice with different `inst_filter` restarts.
    /// `OngoingLeadTradesInstFilter::All` is mapped to Swap (server requires a concrete
    /// instType).
    pub fn start_ws(&self, inst_filter: OngoingLeadTradesInstFilter, trading_mode: OngoingLeadTradesTradingMode) { todo!() }
    /// Get the current state (UniFFI export).
    /// 
    /// Returns the External type (UniFFI-compatible).
    pub fn state(&self) -> OngoingLeadTradesState { todo!() }
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
    /// Tear down the WS subscriptions. Mirrors native
    /// `OrderListViewModel.onPause()` → `wsJobs?.forEach { it.cancel() }`.
    pub fn stop_ws(&self) { todo!() }
    /// Opt-in async UI event stream (cycle-free). Single-subscriber.
    pub fn ui_event_stream(&self) -> Arc<OngoingLeadTradesUiEventStream> { todo!() }
}

/// Deprecated UniFFI callback interface for observing UI events. Prefer the cycle-free `OngoingLeadTradesUiEventStream` returned by `interactor.ui_event_stream()`.
pub struct OngoingLeadTradesUiEventObserver;
impl OngoingLeadTradesUiEventObserver {
    pub fn on_ui_event(&self, event: OngoingLeadTradesUiEvent) { todo!() }
}

/// Opt-in async event stream from `interactor.ui_event_stream()`.
/// Single-subscriber, latest-wins. `next()` resolves to `None` when the
/// interactor is dropped or a new subscription replaces this one.
pub struct OngoingLeadTradesUiEventStream;
impl OngoingLeadTradesUiEventStream {
    pub async fn next(&self) -> Option<OngoingLeadTradesUiEvent> { todo!() }
}

/// Callback trait for effect (one-shot event) notifications.
/// 
/// Native side implements this trait to receive effects from Rust.
pub struct PartnerEmailCodeEffectCallback;
impl PartnerEmailCodeEffectCallback {
    /// Called when an effect is emitted.
    pub fn on_effect(&self, state: PartnerEmailCodeState) { todo!() }
}

/// Partner Email Code Interactor
/// 
/// Manages sending email verification codes to affiliate partners.
pub struct PartnerEmailCodeInteractor;
impl PartnerEmailCodeInteractor {
    /// Acknowledge an effect (clear effect state).
    /// 
    /// # Parameters
    /// 
    /// * `effect_id` - The effect ID to acknowledge
    pub fn ack_effect(&self, effect_id: i64) { todo!() }
    /// Clear all callbacks to avoid memory leaks.
    pub fn clear_callbacks(&self) { todo!() }
    /// Get current state.
    pub fn get_state(&self) -> PartnerEmailCodeState { todo!() }
    /// Send email verification code to the affiliate partner.
    /// 
    /// # Parameters
    /// 
    /// * `partner_uid` - The affiliate partner's UID
    /// 
    /// # Effects
    /// 
    /// - `SendSuccess` on success
    /// - `SendError` on general failure
    /// - `NotAffiliateUser` if error code is 81001
    /// - `VerificationCodeError` if error code is 81004
    pub fn send_email_code(&self, partner_uid: String) { todo!() }
    /// Set effect callback.
    pub fn set_effect_callback(&self, callback: Arc<dyn PartnerEmailCodeEffectCallback>) { todo!() }
    /// Set state change callback.
    pub fn set_state_callback(&self, callback: Arc<dyn PartnerEmailCodeStateCallback>) { todo!() }
}

/// Callback trait for state change notifications.
/// 
/// Native side implements this trait to receive state updates from Rust.
pub struct PartnerEmailCodeStateCallback;
impl PartnerEmailCodeStateCallback {
    /// Called when state changes.
    pub fn on_state_changed(&self, state: PartnerEmailCodeState) { todo!() }
}

/// Profit Settlement Interactor.
/// 
/// Manages dividend overview, paginated pre-profit and history profit lists,
/// AUM dashboard info, and profit sharing ratio editing.
/// 
/// # Auto-Generated by `#[interactor::interactor]`:
/// 
/// - `state()` - Get current state
/// - `set_state_observer(observer)` - Set state change observer
/// - `remove_state_observer()` - Remove state observer
/// - `set_ui_event_observer(observer)` - Set UI event observer
/// - `remove_ui_event_observer()` - Remove UI event observer
/// - `start()` / `stop()` - Lifecycle management
pub struct ProfitSettlementInteractor;
impl ProfitSettlementInteractor {
    /// Load AUM dashboard info and profit sharing ratio limits.
    pub fn load_aum_info(&self) { todo!() }
    /// Load first page of dividend history.
    pub fn load_history_profit(&self) { todo!() }
    /// Load next page of dividend history and append.
    pub fn load_more_history_profit(&self) { todo!() }
    /// Load next page of pre-profit list and append.
    pub fn load_more_pre_profit(&self) { todo!() }
    /// Load next page of profit detail and append.
    /// 
    /// Mirrors `DividendDetailsViewModel.loadMoreProfitDetails` (file: VM.kt:61):
    /// does NOT reset; uses cursor from previous page's last `id`.
    pub fn load_more_profit_detail(&self, settle_time: String) { todo!() }
    /// Load first page of pre-profit list.
    pub fn load_pre_profit(&self) { todo!() }
    /// Load first page of profit detail (breakdown for a settlement).
    /// 
    /// Mirrors `DividendDetailsViewModel.loadProfitDetails` (file: VM.kt:37):
    /// resets cursor + items, then fetches first page with `settle_time` as input.
    pub fn load_profit_detail(&self, settle_time: String) { todo!() }
    /// Load profit info (share_ratio, latest_profit, total_profit, follower count).
    pub fn load_profit_info(&self) { todo!() }
    /// Called when user moves the ratio slider/input.
    pub fn on_ratio_changed(&self, ratio: f64) { todo!() }
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
    /// Request navigation to profit sharing edit screen.
    pub fn request_edit_profit_sharing(&self) { todo!() }
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
    pub fn set_ui_event_observer(&self, observer: Arc<dyn ProfitSettlementUiEventObserver>) { todo!() }
    /// Start the interactor lifecycle.
    /// 
    /// Calls the user's `InteractorLifecycle::start()` implementation.
    /// Override that method to add custom startup behavior (e.g., initial data loads).
    pub fn start(&self) { todo!() }
    /// Get the current state (UniFFI export).
    /// 
    /// Returns the External type (UniFFI-compatible).
    pub fn state(&self) -> ProfitSettlementState { todo!() }
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
    /// Get current trade type (UniFFI export).
    pub fn trade_type(&self) -> ProfitSettlementTradeType { todo!() }
    /// Opt-in async UI event stream (cycle-free). Single-subscriber.
    pub fn ui_event_stream(&self) -> Arc<ProfitSettlementUiEventStream> { todo!() }
    /// Submit updated profit sharing ratio.
    pub fn update_profit_sharing_ratio(&self) { todo!() }
}

/// Deprecated UniFFI callback interface for observing UI events. Prefer the cycle-free `ProfitSettlementUiEventStream` returned by `interactor.ui_event_stream()`.
pub struct ProfitSettlementUiEventObserver;
impl ProfitSettlementUiEventObserver {
    pub fn on_ui_event(&self, event: ProfitSettlementUiEvent) { todo!() }
}

/// Opt-in async event stream from `interactor.ui_event_stream()`.
/// Single-subscriber, latest-wins. `next()` resolves to `None` when the
/// interactor is dropped or a new subscription replaces this one.
pub struct ProfitSettlementUiEventStream;
impl ProfitSettlementUiEventStream {
    pub async fn next(&self) -> Option<ProfitSettlementUiEvent> { todo!() }
}

pub struct RelationCountInteractor;
impl RelationCountInteractor {
    /// Default constructor wired through the social-trading DI container.
    pub fn new() -> Arc<RelationCountInteractor> { todo!() }
    /// Clear the foreign callback. Safe to call multiple times.
    pub fn clear_state_callback(&self) { todo!() }
    /// Current state snapshot (mainly for tests and explicit reads).
    pub fn get_state(&self) -> RelationCountState { todo!() }
    /// Re-issue the REST seed without touching the WS subscription.
    /// Used by pull-to-refresh and view-will-appear re-entry on iOS.
    pub fn refresh_rest(&self) { todo!() }
    /// Register the foreign callback. The current state is delivered immediately.
    pub fn set_state_callback(&self, callback: Arc<dyn RelationCountStateCallback>) { todo!() }
    /// REST seed + subscribe to the WS channel. Re-entrant calls re-issue the
    /// REST seed and leave the WS subscription untouched.
    /// WS is currently gated by [`WS_LIVE_UPDATES_ENABLED`].
    pub fn start(&self) { todo!() }
    /// Cancel pending REST/WS work and clear the latest snapshot.
    /// Called on logout, role change, or VC deinit.
    pub fn stop(&self) { todo!() }
}

/// Foreign callback for state changes. Implemented by the native side.
pub struct RelationCountStateCallback;
impl RelationCountStateCallback {
    pub fn on_state_changed(&self, state: RelationCountState) { todo!() }
}

/// Smart Copy Trading Interactor
pub struct SmartCopyTradingInteractor;
impl SmartCopyTradingInteractor {
    /// Amend smart copy settings.
    pub fn amend_settings(&self) { todo!() }
    /// Fetch reducible amount.
    pub fn fetch_reducible_amount(&self) { todo!() }
    /// Load smart copy settings.
    pub fn load_data(&self) { todo!() }
    /// Fetch the wallet trading-account available balance into state, then emit
    /// `AvailableBalanceLoaded` / `AvailableBalanceLoadError`.
    /// 
    /// The Rust-owned counterpart of [`Self::update_available_balance`]: the platform used
    /// to run this query itself (resolving the currency id through the legacy native trade
    /// kernel) and inject the result. Doing it here keeps the whole increase-validation
    /// input inside Rust — no native-kernel read on this leg.
    pub fn refresh_available_balance(&self) { todo!() }
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
    pub fn set_ui_event_observer(&self, observer: Arc<dyn SmartCopyTradingUiEventObserver>) { todo!() }
    /// Start the interactor lifecycle.
    /// 
    /// Calls the user's `InteractorLifecycle::start()` implementation.
    /// Override that method to add custom startup behavior (e.g., initial data loads).
    pub fn start(&self) { todo!() }
    /// Get the current state (UniFFI export).
    /// 
    /// Returns the External type (UniFFI-compatible).
    pub fn state(&self) -> SmartCopyTradingState { todo!() }
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
    /// Transfer smart copy amount (increase or decrease investment).
    pub fn transfer_amount(&self, amount: String, is_increase: bool) { todo!() }
    /// Opt-in async UI event stream (cycle-free). Single-subscriber.
    pub fn ui_event_stream(&self) -> Arc<SmartCopyTradingUiEventStream> { todo!() }
    /// Update the available balance (from wallet/account query).
    pub fn update_available_balance(&self, balance: Option<String>) { todo!() }
    /// Update pending remain margin deal (local state only, not submitted yet).
    pub fn update_pending_remain_mgn_deal(&self, value: Option<String>) { todo!() }
    /// Update pending stop loss amount (local state only, not submitted yet).
    pub fn update_pending_stop_loss(&self, amount: Option<String>) { todo!() }
    /// Inject smart copy settings from an upstream source that already holds
    /// them (e.g. the modify-amount screen receives them via navigation args),
    /// so increase validation has `current_follow_amount` / `max_increase_amount`
    /// without a redundant `load_data` fetch (which would require the trader
    /// unique name that this screen does not carry).
    pub fn update_settings(&self, settings: SmartCopyTradingSettings) { todo!() }
    /// Synchronously validate a transfer-amount input for LIVE (per-keystroke) UI
    /// feedback, WITHOUT performing the transfer. Returns the validation result;
    /// the caller renders the error text / button-enable state / computed final
    /// amount (locale formatting stays platform-side).
    /// 
    /// Shares its rules with `transfer_amount`'s submit guard via
    /// [`validate_transfer_input_inner`], so live feedback and the submit check
    /// can never diverge. Reads `settings` / `reducible_amount` / `available_balance`
    /// from the current state (populated by `load_data` / `fetch_reducible_amount`
    /// / `update_available_balance`).
    pub fn validate_transfer_input(&self, amount: String, is_increase: bool) -> SmartCopyTransferValidation { todo!() }
}

/// Deprecated UniFFI callback interface for observing UI events. Prefer the cycle-free `SmartCopyTradingUiEventStream` returned by `interactor.ui_event_stream()`.
pub struct SmartCopyTradingUiEventObserver;
impl SmartCopyTradingUiEventObserver {
    pub fn on_ui_event(&self, event: SmartCopyTradingUiEvent) { todo!() }
}

/// Opt-in async event stream from `interactor.ui_event_stream()`.
/// Single-subscriber, latest-wins. `next()` resolves to `None` when the
/// interactor is dropped or a new subscription replaces this one.
pub struct SmartCopyTradingUiEventStream;
impl SmartCopyTradingUiEventStream {
    pub async fn next(&self) -> Option<SmartCopyTradingUiEvent> { todo!() }
}

/// One instId's last-price stream.
pub struct SocialInstTickerSession;
impl SocialInstTickerSession {
    /// Fire-and-forget start. The data source's callback delivers BOTH the HTTP
    /// snapshot and every WS refresh (`execute_http_request` fires `on_refreshed` on
    /// HTTP success, `inst_ticker_data_source.rs:415-421`) — so this deliberately does
    /// NOT push `start()`'s returned snapshot itself (it would be a duplicate).
    /// Errors fall back silently (the Kotlin side shows native's placeholder until
    /// data arrives) with a once-log for diagnosis.
    pub fn start(&self) { todo!() }
    /// Stop the WS subscription. Call from the VM's `onCleared` (mirrors the native
    /// `unBindSourceObserver` cancellable).
    pub fn stop(&self) { todo!() }
}

/// Kotlin-implemented observer receiving each `last` price (snapshot + every WS refresh).
/// Empty string = ticker had no `last` (native renders its placeholder in that case).
pub struct SocialTickerObserver;
impl SocialTickerObserver {
    pub fn on_last_price(&self, last: String) { todo!() }
}

/// Deprecated UniFFI callback interface for observing UI events. Prefer the cycle-free `SpotSafetyChecksUiEventStream` returned by `interactor.ui_event_stream()`.
pub struct SpotSafetyChecksUiEventObserver;
impl SpotSafetyChecksUiEventObserver {
    pub fn on_ui_event(&self, event: SpotSafetyChecksUiEvent) { todo!() }
}

/// Opt-in async event stream from `interactor.ui_event_stream()`.
/// Single-subscriber, latest-wins. `next()` resolves to `None` when the
/// interactor is dropped or a new subscription replaces this one.
pub struct SpotSafetyChecksUiEventStream;
impl SpotSafetyChecksUiEventStream {
    pub async fn next(&self) -> Option<SpotSafetyChecksUiEvent> { todo!() }
}

/// Spot Trade Safety Checks Interactor
/// 
/// Manages price limit safety checks for spot copy trading.
pub struct SpotTradeSafetyChecksInteractor;
impl SpotTradeSafetyChecksInteractor {
    /// Check price limit for a spot trade.
    /// 
    /// # Parameters
    /// * `inst_id` - Instrument ID (e.g., "BTC-USDT")
    /// * `side` - Trade side (e.g., "buy" or "sell")
    /// * `px` - The price to check
    /// * `sz` - The size/quantity
    pub fn check_price_limit(&self, inst_id: String, side: String, px: String, sz: String) { todo!() }
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
    pub fn set_ui_event_observer(&self, observer: Arc<dyn SpotSafetyChecksUiEventObserver>) { todo!() }
    /// Start the interactor lifecycle.
    /// 
    /// Calls the user's `InteractorLifecycle::start()` implementation.
    /// Override that method to add custom startup behavior (e.g., initial data loads).
    pub fn start(&self) { todo!() }
    /// Get the current state (UniFFI export).
    /// 
    /// Returns the External type (UniFFI-compatible).
    pub fn state(&self) -> SpotSafetyChecksState { todo!() }
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
    pub fn ui_event_stream(&self) -> Arc<SpotSafetyChecksUiEventStream> { todo!() }
}

/// The one piece of CDN configuration Rust cannot know: the accelerated domain, which
/// the platform discovers at runtime from `/dns-query/info`.
/// 
/// Everything else about the rewrite — which hosts qualify, how the swap is performed —
/// is Rust's ([`HARDCODED_CDN_HOST_PATTERNS`], [`accelerate`]), so the native side keeps
/// no icon logic of its own. iOS backs this with
/// `OKFResourceAcceleratorManager.xcdnDomainFromRequest`, read per call because the
/// value arrives asynchronously after launch.
pub struct StaticCdnDomainProvider;
impl StaticCdnDomainProvider {
    /// Accelerated CDN domain, scheme included (e.g. `https://static.okx.com`).
    /// Empty until the platform's CDN configuration is ready.
    pub fn current_domain(&self) -> String { todo!() }
}

pub struct TotalAssetInteractor;
impl TotalAssetInteractor {
    /// Coin sign (e.g. "₿") for `symbol`. `""` on a store miss — native parity with
    /// `OkTrade.getCoinSign`. The coins store is warmed during `fetch_total_asset`
    /// (`ensure_coins_loaded`), mirroring the native `ensureInitializedSuspend()` the
    /// holder previously ran before formatting.
    pub fn coin_sign(&self, symbol: String) -> String { todo!() }
    /// Load total-asset / investment-info. Mirrors native
    /// `TotalAssetViewModel.fetchTotalAsset()`.
    pub fn fetch_total_asset(&self) { todo!() }
    /// Whether `iso_code` is a valuation (fiat display) currency — native parity with the
    /// holder's `getCurrencyListData().map{isoCode}.contains(ccy)`. Reads the platform
    /// `settings` service, not the legacy trade kernel.
    pub fn is_fiat_currency(&self, iso_code: String) -> bool { todo!() }
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
    pub fn set_ui_event_observer(&self, observer: Arc<dyn TotalAssetUiEventObserver>) { todo!() }
    /// Start the interactor lifecycle.
    /// 
    /// Calls the user's `InteractorLifecycle::start()` implementation.
    /// Override that method to add custom startup behavior (e.g., initial data loads).
    pub fn start(&self) { todo!() }
    /// Get the current state (UniFFI export).
    /// 
    /// Returns the External type (UniFFI-compatible).
    pub fn state(&self) -> TotalAssetState { todo!() }
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
    pub fn ui_event_stream(&self) -> Arc<TotalAssetUiEventStream> { todo!() }
}

/// Deprecated UniFFI callback interface for observing UI events. Prefer the cycle-free `TotalAssetUiEventStream` returned by `interactor.ui_event_stream()`.
pub struct TotalAssetUiEventObserver;
impl TotalAssetUiEventObserver {
    pub fn on_ui_event(&self, event: TotalAssetUiEvent) { todo!() }
}

/// Opt-in async event stream from `interactor.ui_event_stream()`.
/// Single-subscriber, latest-wins. `next()` resolves to `None` when the
/// interactor is dropped or a new subscription replaces this one.
pub struct TotalAssetUiEventStream;
impl TotalAssetUiEventStream {
    pub async fn next(&self) -> Option<TotalAssetUiEvent> { todo!() }
}

/// Trader Profile Root Interactor
pub struct TraderProfileRootInteractor;
impl TraderProfileRootInteractor {
    /// Load profile data with input.
    pub fn load(&self, input: TraderProfileRootLoadInput) { todo!() }
    /// Refresh using last input. `request_id` correlates the resulting load
    /// terminals (it replaces the stale id inside the reused last input).
    pub fn refresh(&self, request_id: String) { todo!() }
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
    /// Subscribe or unsubscribe empty reminder.
    pub fn set_empty_reminder(&self, input: TraderProfileEmptyReminderInput) { todo!() }
    /// Follow or unfollow trader.
    pub fn set_following(&self, input: TraderProfileFollowInput) { todo!() }
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
    pub fn set_ui_event_observer(&self, observer: Arc<dyn TraderProfileRootUiEventObserver>) { todo!() }
    /// Start the interactor lifecycle.
    /// 
    /// Calls the user's `InteractorLifecycle::start()` implementation.
    /// Override that method to add custom startup behavior (e.g., initial data loads).
    pub fn start(&self) { todo!() }
    /// Get the current state (UniFFI export).
    /// 
    /// Returns the External type (UniFFI-compatible).
    pub fn state(&self) -> TraderProfileRootState { todo!() }
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
    pub fn ui_event_stream(&self) -> Arc<TraderProfileRootUiEventStream> { todo!() }
}

/// Deprecated UniFFI callback interface for observing UI events. Prefer the cycle-free `TraderProfileRootUiEventStream` returned by `interactor.ui_event_stream()`.
pub struct TraderProfileRootUiEventObserver;
impl TraderProfileRootUiEventObserver {
    pub fn on_ui_event(&self, event: TraderProfileRootUiEvent) { todo!() }
}

/// Opt-in async event stream from `interactor.ui_event_stream()`.
/// Single-subscriber, latest-wins. `next()` resolves to `None` when the
/// interactor is dropped or a new subscription replaces this one.
pub struct TraderProfileRootUiEventStream;
impl TraderProfileRootUiEventStream {
    pub async fn next(&self) -> Option<TraderProfileRootUiEvent> { todo!() }
}

/// Stateless Facade aggregating the three Stats-tab use_cases.
/// 
/// Pure passthrough: every method forwards to the corresponding use_case and returns
/// its `OKRustFuture` unchanged. No mapping, no state, no branching.
pub struct TraderStatsInteractor;
impl TraderStatsInteractor {
    /// Create the Facade. Wires the three use_cases from the global `DependencyContainer`.
    pub fn new() -> Arc<TraderStatsInteractor> { todo!() }
    /// Account (own profile). Delegates to
    /// [`OkstCorePreferenceCoinUseCase::fetch_account_chart_data`].
    pub fn fetch_account_preference_coin(&self, business_line: PreferenceCoinBusinessLine) -> Arc<OKRustFuturePreferenceCoinItemVecWithOKSTFFIError> { todo!() }
    /// Account (own profile). Delegates to
    /// [`OKSTCoreTraderTradePerformanceUseCase::fetch_account_trade_performance`].
    pub fn fetch_account_trade_performance(&self, latest_num: String, biz_type: TraderTradePerformanceBizType) -> Arc<OKRustFutureTraderTradePerformanceStatsWithOKSTFFIError> { todo!() }
    /// Account (own profile). Delegates to
    /// [`OKSTCoreYieldPnlChartDataUseCase::fetch_account_chart_data`].
    pub fn fetch_account_yield_pnl(&self, latest_num: String, business_line: YieldPnlBusinessLine) -> Arc<OKRustFutureYieldPnlChartDataItemVecWithOKSTFFIError> { todo!() }
    /// Public (viewing another trader). Delegates to
    /// [`OkstCorePreferenceCoinUseCase::fetch_public_chart_data`].
    pub fn fetch_public_preference_coin(&self, unique_name: String, business_line: PreferenceCoinBusinessLine) -> Arc<OKRustFuturePreferenceCoinItemVecWithOKSTFFIError> { todo!() }
    /// Public (viewing another trader). Delegates to
    /// [`OKSTCoreTraderTradePerformanceUseCase::fetch_public_trade_performance`].
    pub fn fetch_public_trade_performance(&self, unique_name: String, latest_num: String, biz_type: TraderTradePerformanceBizType) -> Arc<OKRustFutureTraderTradePerformanceStatsWithOKSTFFIError> { todo!() }
    /// Public (viewing another trader). Delegates to
    /// [`OKSTCoreYieldPnlChartDataUseCase::fetch_public_chart_data`].
    pub fn fetch_public_yield_pnl(&self, unique_name: String, latest_num: String, business_line: YieldPnlBusinessLine) -> Arc<OKRustFutureYieldPnlChartDataItemVecWithOKSTFFIError> { todo!() }
}

/// Vanguard Share Interactor (stateless Facade)
/// 
/// Fetches trader profile, yield curve, and top followers for share card
/// rendering. Holds only its request center; no mutable state.
pub struct VanguardShareInteractor;
impl VanguardShareInteractor {
    /// Fetch vanguard share data for a trader.
    /// 
    /// Returns trader share data for generating share cards.
    /// 
    /// # Arguments
    /// 
    /// * `unique_name` - Trader's unique identifier
    /// * `trade_type` - Trading type: `Contract` (futures) or `Spot`
    /// 
    /// # Returns
    /// 
    /// A cancellable `OKRustFuture` resolving to:
    /// - `Ok(Some(VanguardShareInfo))` - Share data found
    /// - `Ok(None)` - No share data available
    /// - `Err(NetworkFailed)` - Network request failed
    /// - `Err(JSONParseError)` - Response parsing failed
    /// - `Err(ServerResponseError)` - Server returned a business error
    pub fn fetch_vanguard_share(&self, unique_name: String, trade_type: VanguardShareTradeType) -> Arc<OKRustFutureOptionalVanguardShareInfoWithOKSTFFIError> { todo!() }
    /// Fetches the same Vanguard payload and projects all platform-neutral card
    /// business decisions before crossing the native boundary.
    pub fn fetch_vanguard_share_presentation(&self, request: VanguardSharePresentationRequest) -> Arc<OKRustFutureOptionalVanguardSharePresentationWithOKSTFFIError> { todo!() }
}

// -----------------------------------------------------------------------------
// 4. Top-Level Exported Functions (74)
// -----------------------------------------------------------------------------

/// Android private-invitation entry point.
/// 
/// This intentionally stays a top-level function instead of extending
/// [`CopyTradeRetainedActions`]. Released native records and the generated iOS
/// `CopyTradeRetainedInteractorProtocol` therefore keep their existing shape.
pub fn android_copy_trade_retained_init_start_copy_check_with_private_link(interactor: Arc<CopyTradeRetainedInteractor>, input: StartCopyCheckInput, private_copy_link_id: Option<String>) { todo!() }

/// Android private-invitation re-entry after native account switching.
/// Kept outside the interactor protocol for iOS source compatibility.
pub fn android_copy_trade_retained_switch_account_reenter_with_private_link(interactor: Arc<CopyTradeRetainedInteractor>, input: StartCopyCheckInput, private_copy_link_id: Option<String>) { todo!() }

/// Compute which reminder banner (if any) to show.
/// 
/// Bridge usage:
/// ```ignore
/// val reminder = computeOngoingLeadTradesReminder(inputs)
/// val resId = when (reminder) {
/// OngoingLeadTradesReminderType.NONE -> null
/// OngoingLeadTradesReminderType.LEAD_TRADER_PRIVATE_MODE_HIDDEN ->
/// R.string.pr_social_leadtrader_reminder_desc_in_private_mode_hidden_data
/// // ...
/// }
/// ```
pub fn compute_ongoing_lead_trades_reminder(inputs: OngoingLeadTradesReminderInputs) -> OngoingLeadTradesReminderType { todo!() }

/// Factory function to create the Interactor.
pub fn create_copier_message_interactor() -> Arc<CopierMessageInteractor> { todo!() }

/// Factory function to create the Interactor.
pub fn create_copier_pnl_ws_interactor(inst_type: CopierPnlInstType, data_type: CopierPnlLiveDataType, trading_mode: CopierPnlTradingMode, rise_fall_cycle: CopierPnlRiseFallCycleType) -> Arc<CopierPnlWsInteractor> { todo!() }

/// Create the Copier Position History interactor with the default
/// `MyCopiesRequestCenter`.
pub fn create_copier_position_history_interactor(inst_type: CopierPnlInstType, copy_rel_id: String, copy_mode: MyLeadTraderCopyMode) -> Arc<CopierPositionHistoryInteractor> { todo!() }

/// Create the Copier Positions interactor with the default `MyCopiesRequestCenter`.
/// 
/// `inst_type` and `copy_rel_id` define the copier-trader pair this interactor
/// operates on; they're written into `CopierPositionsState` at construction and
/// stay immutable for the lifetime of the interactor.
pub fn create_copier_positions_interactor(inst_type: CopierPnlInstType, copy_rel_id: String) -> Arc<CopierPositionsInteractor> { todo!() }

pub fn create_copy_contract_settings_interactor(trader_unique_name: String) -> Arc<CopyContractSettingsInteractor> { todo!() }

pub fn create_copy_settings_spot_interactor() -> Arc<CopySettingsSpotInteractor> { todo!() }

/// Factory function to create the Swap Interactor.
pub fn create_copy_settings_swap_interactor() -> Arc<CopySettingsSwapInteractor> { todo!() }

/// Factory function (UniFFI export).
pub fn create_copy_trade_retained_interactor() -> Arc<CopyTradeRetainedInteractor> { todo!() }

/// Factory function exported to native via UniFFI.
/// 
/// Returns an `Arc<CouponCenterInteractor>` initialized with the default
/// `EcoCouponRequestCenter` (production HTTP client).
pub fn create_coupon_center_interactor() -> Arc<CouponCenterInteractor> { todo!() }

pub fn create_eco_batch_lever_interactor() -> Arc<EcoBatchLeverInteractor> { todo!() }

/// Android's factory. Keeps `PositionScatterEmptyPolicy::ErrorOnEmpty` — an empty API
/// array surfaces as `DataLoadError`, mirroring Kotlin `List.first()`. Signature and
/// behaviour are unchanged; do not alter either.
pub fn create_eco_hold_position_interactor(unique_name: Option<String>, is_spot: bool) -> Arc<EcoHoldPositionInteractor> { todo!() }

/// Additive factory that lets the caller choose how an empty API array is treated.
/// 
/// iOS passes `PositionScatterEmptyPolicy::EmptySuccess` so the Rust branch renders the
/// same empty state the legacy native branch does (`[OKGTEUtils validateArray:]` is NO
/// for an empty array → `showEmptyStatusView`), instead of an error/retry view. Nothing
/// about the Android factory above changes.
pub fn create_eco_hold_position_interactor_with_empty_policy(unique_name: Option<String>, is_spot: bool, empty_policy: PositionScatterEmptyPolicy) -> Arc<EcoHoldPositionInteractor> { todo!() }

pub fn create_eco_pnl_weekly_interactor(unique_name: Option<String>, is_spot: bool) -> Arc<EcoPnlWeeklyInteractor> { todo!() }

/// Factory function to create the Interactor in **account mode** (own followers).
pub fn create_follower_list_interactor(trade_type: FollowerListTradeType) -> Arc<FollowerListInteractor> { todo!() }

/// Factory function to create the Interactor.
/// 
/// # Arguments
/// 
/// * `trade_type` - The trade type (Futures or Spot)
/// * `is_auto_include_new` - Initial value for "Auto-include new crypto" toggle
/// 
/// # Returns
/// 
/// Arc-wrapped Interactor instance.
pub fn create_lead_trade_crypto_interactor(trade_type: LeadTradeCryptoTradeType, is_auto_include_new: bool) -> Arc<LeadTradeCryptoInteractor> { todo!() }

/// Factory function to create the Interactor.
/// 
/// # Arguments
/// 
/// * `trade_type` - The trade type (Futures or Spot)
/// 
/// # Returns
/// 
/// Arc-wrapped Interactor instance.
/// 
/// # Note
/// 
/// This interactor uses the trader's own position history API which automatically
/// identifies the current logged-in user, so no uniqueName parameter is needed.
pub fn create_lead_trade_history_interactor(trade_type: LeadTradeHistoryTradeType) -> Arc<LeadTradeHistoryInteractor> { todo!() }

/// Factory function — creates an interactor that loads another trader's public history.
/// 
/// When `unique_name` is non-empty, `do_load_page` calls the public endpoint
/// (`/priapi/v5/ecotrade/public/position-history`) instead of the owner endpoint.
/// Native: SpotLeadHistoryOrderViewModel.kt:43 / MyHistoryLeadTradingViewModel.kt:127-129
pub fn create_lead_trade_history_interactor_for_public(trade_type: LeadTradeHistoryTradeType, unique_name: String) -> Arc<LeadTradeHistoryInteractor> { todo!() }

/// Factory function to create the Interactor.
/// 
/// # Arguments
/// 
/// * `biz_type` - The business type (Futures, Spot, etc.)
/// 
/// # Returns
/// 
/// Arc-wrapped Interactor instance.
pub fn create_lead_trader_application_interactor(biz_type: LeadTraderApplicationBizType) -> Arc<LeadTraderApplicationInteractor> { todo!() }

/// Factory function to create the Interactor.
pub fn create_lead_trader_list_interactor(business_line: LeadTraderListBusinessLine) -> Arc<LeadTraderListInteractor> { todo!() }

/// Factory function to create the Interactor.
pub fn create_lead_trader_settings_interactor(trade_type: LeadTraderSettingsTradeType) -> Arc<LeadTraderSettingsInteractor> { todo!() }

/// Factory function to create the Interactor.
/// 
/// # Parameters
/// 
/// * `trade_type` - The trade type (Futures or Spot)
/// 
/// # Returns
/// 
/// Arc-wrapped Interactor instance.
pub fn create_manage_copy_traders_interactor(trade_type: ManageCopyTradersTradeType) -> Arc<ManageCopyTradersInteractor> { todo!() }

/// Factory function to create the Interactor.
pub fn create_marketplace_home_interactor() -> Arc<MarketplaceHomeInteractor> { todo!() }

/// Factory function to create the Interactor.
pub fn create_my_copies_current_traders_interactor(inst_type: CopierPnlInstType) -> Arc<MyCopiesCurrentTradersInteractor> { todo!() }

/// Factory function to create the Spot Interactor.
pub fn create_my_copies_history_list_spot_interactor() -> Arc<MyCopiesHistoryListSpotInteractor> { todo!() }

/// Factory function to create the Swap Interactor.
pub fn create_my_copies_history_list_swap_interactor() -> Arc<MyCopiesHistoryListSwapInteractor> { todo!() }

/// Factory function to create the Interactor.
pub fn create_my_followers_interactor(trade_type: MyFollowersTradeType) -> Arc<MyFollowersInteractor> { todo!() }

/// Factory function to create the interactor.
pub fn create_my_lead_trader_details_interactor(inst_type: CopierPnlInstType, copy_rel_id: String, trading_mode: CopierPnlTradingMode, rise_fall_cycle: CopierPnlRiseFallCycleType) -> Arc<MyLeadTraderDetailsInteractor> { todo!() }

/// Factory function to create the Interactor.
pub fn create_ongoing_lead_trades_interactor(unique_name: Option<String>) -> Arc<OngoingLeadTradesInteractor> { todo!() }

/// Factory function to create the Interactor.
/// 
/// # Returns
/// 
/// Arc-wrapped Interactor instance.
pub fn create_partner_email_code_interactor() -> Arc<PartnerEmailCodeInteractor> { todo!() }

/// Factory function — called from Native/UniFFI.
pub fn create_profit_settlement_interactor(trade_type: ProfitSettlementTradeType) -> Arc<ProfitSettlementInteractor> { todo!() }

/// Factory function to create the Interactor in **public mode** (viewing
/// another trader's profile follower list).
/// 
/// The `unique_name` parameter is the target trader's identifier; non-empty
/// values route all loads to the public endpoint.
pub fn create_public_follower_list_interactor(trade_type: FollowerListTradeType, unique_name: String) -> Arc<FollowerListInteractor> { todo!() }

/// Factory function to create the Smart Copy Trading Interactor.
/// 
/// # Parameters
/// 
/// * `unique_name` - Composite key: "traderUniqueName__copyRelId__instType"
/// (e.g., "trader123__rel456__SWAP"). Parsed into trader_unique_name, copy_rel_id, inst_type.
pub fn create_smart_copy_trading_interactor(unique_name: String) -> Arc<SmartCopyTradingInteractor> { todo!() }

pub fn create_social_inst_ticker_session(inst_id: String, observer: Arc<dyn SocialTickerObserver>) -> Arc<SocialInstTickerSession> { todo!() }

/// Factory function to create the Interactor.
pub fn create_spot_trade_safety_checks_interactor() -> Arc<SpotTradeSafetyChecksInteractor> { todo!() }

pub fn create_total_asset_interactor() -> Arc<TotalAssetInteractor> { todo!() }

/// Factory function to create the interactor.
pub fn create_trader_profile_root_interactor(trader_unique_name: String, is_self_profile: bool) -> Arc<TraderProfileRootInteractor> { todo!() }

/// Factory function to create the Interactor.
/// 
/// Builds the default request center (`cefi_client`). No arguments are needed
/// because the interactor is stateless — `unique_name` / `trade_type` are
/// passed per call to `fetch_vanguard_share`.
/// 
/// # Returns
/// 
/// Arc-wrapped Interactor instance.
pub fn create_vanguard_share_interactor() -> Arc<VanguardShareInteractor> { todo!() }

/// Native bootstrap: install the platform's accelerated-domain source. Safe to call
/// more than once (last writer wins). With no provider registered every URL passes
/// through un-accelerated, which keeps unit tests free of a fake.
pub fn register_static_cdn_domain_provider(provider: Arc<dyn StaticCdnDomainProvider>) { todo!() }

/// Bridge `formatCryptoAmount` value step — native `createCoinInfoFormat(ccy, amount)
/// .group()` = `OKTFormatter.setNumber(amount, getTradeCoinInfo(ccy).sizeDig).group()`:
/// round `amount` to the coin's `sizeDig` precision (OKTFormatter default DOWN, trailing
/// zeros dropped) then locale-group. Rust owns the round + precision and returns the
/// UNGROUPED value; Kotlin applies `.formatLocalized()` (native `.group()`), exactly the
/// [`social_usd_to_local`] split. `None` = empty amount-or-ccy / store cold
/// (self-warms) / unknown coin / blank sizeDig / unparseable amount → Kotlin falls back to
/// the intact native chain (whose own paths cover the DEFAULT_COIN_DIGIT and raw-localize
/// cases).
pub fn social_coin_amount(symbol: String, amount: String) -> Option<String> { todo!() }

/// Bridge `getCoinConfig` — native `getTradeCoinInfo(ccy)`'s auto-product support flags.
/// `None` = empty / store cold (self-warms) / unknown coin → Kotlin falls back to
/// its native read (which returns `Nil` for an unknown coin).
pub fn social_coin_config(ccy: String) -> Option<SocialCoinConfig> { todo!() }

/// Bridge `getCoinId` — native `getTradeCoinInfo(coin).id`. Reads the SAME coins store as
/// [`social_coin_size_dig`]. `None` = empty / store cold (self-warms) / unknown
/// coin → Kotlin falls back to its native lookup (whose own default is `-1`). The raw id
/// string is returned; Kotlin applies its `toSafeInt().toString()` normalization.
pub fn social_coin_id(symbol: String) -> Option<String> { todo!() }

/// Bridge `getCoinSign` (native `okTrade.getCoinSign(symbol)` =
/// `getTradeCoinInfo(symbol).getSign()`) = the coins store's `sign` for `symbol`. Reads the
/// SAME store as [`social_coin_size_dig`]. `None` = empty /
/// store cold
/// (self-warms) / unknown coin / blank sign → Kotlin falls back to the native read (whose own
/// default is the empty string).
pub fn social_coin_sign(symbol: String) -> Option<String> { todo!() }

/// F1.4: raw coin quantity precision (`sizeDig`) from core-trading's coins store —
/// the Rust source for native `getTradeCoinInfo(symbol)?.sizeDig` (`OkTrade.kt:162`,
/// consumed by `SocialTradeManager.fmtPriceAmtBySymbol` and
/// `SocialFormatExt.pnlToFixedFormat`). Formatting (formatS / ICU / sign) stays in Kotlin;
/// this returns ONLY the precision string so Kotlin keeps its own default when `None`.
/// 
/// Self-warming: the coins store is loaded by core-trading's own flows; when it isn't
/// loaded yet this returns `None` (Kotlin falls back to its native lookup + default) and
/// fires a fire-and-forget `ensure_coins_loaded` so subsequent calls succeed —
/// single-flight inside core-trading, safe to trigger per miss.
pub fn social_coin_size_dig(symbol: String) -> Option<String> { todo!() }

/// Bridge `getCoinUrl` / `getCCYUrl` — native `bizTrade.getCoinUrl(symbol)` = the coins
/// store's `url` for `symbol`. `None` = empty / store cold
/// (self-warms) / unknown
/// coin → Kotlin falls back to its native read (whose own default is the empty string).
pub fn social_coin_url(symbol: String) -> Option<String> { todo!() }

/// Bridge native `CommonFormat.getSymbol`'s instrument-MISSING branch —
/// `createChargeConverterWithIndex(convertInstFamily(id)).toQuoteFormatted("0")
/// .getSymbol()` — as a family-derived charge-currency symbol. Separate from
/// [`social_price_display_symbol`] on purpose: that export mirrors the
/// instrument-present read and must keep bailing on a rust-store miss, while THIS
/// one is only correct at call sites whose native fallback is the family chain.
/// 
/// Coverage: spot/margin (by-index) + POSITIVE-index derivatives via `ccy_name`
/// (the field the in-crate `ok_swap_biz.rs:305` precedent uses). Negative-index
/// (coin-margined) selection mixes the user fiat preference and the EEA
/// display-name remap → `None` keeps the native branch.
pub fn social_family_price_symbol(inst_type: String, inst_id: String) -> Option<String> { todo!() }

/// W2: fiat precision by ISO code — Rust source for native
/// `getTradeDataSettings().getCurrencyByIsoCode(iso)?.precision` (pnlDollarFormat,
/// `CommonNumberFormat.kt`). Reads the settings currency cache (same store F1.5's
/// usd_to_local reads). `None` → native fallback.
pub fn social_fiat_precision(iso_code: String) -> Option<u8> { todo!() }

/// Rust port of native `formatPosition` (`PositionFormat.kt:63-110`): convert a position /
/// closed amount from its origin unit (zhang for contracts, base coin for spot) into the
/// user's preferred display unit and format it at per-instrument precision.
/// 
/// Delegates the WHOLE native atomic op to core-trading's
/// [`OriginQuantityToUserSettingUnitConvertor`]:
/// - unit selection (TradFi → `tradfi_trade_unit`, linear → `linear_trade_unit`, inverse →
/// `inverse_futures_unit`) mirrors `OkSwapBiz.kt:405-425` / `OkFutureBiz.kt:408-423`;
/// - unit→convertor dispatch (Lot / Contract / linear-U / inverse-U / Coin) mirrors
/// `OkFutureBiz.kt:516-570`;
/// - rounding is the convertor default `RoundingMode::Down` (V1 `toTarget().down()` display
/// branch — same direction as native `toTargetFormatted`).
/// 
/// Returns `None` on ANY guard failure so the Kotlin Holder falls back to the intact native
/// `formatPosition` chain (no behavioural divergence, no partial output). Unlike
/// [`round_pnl`], there is NO `convert_inst_family` fallback here: the convertor needs the
/// full rich `Instrument` (contract type, ctVal, lot size), which only the loaded biz store
/// can provide — warm it via [`social_warm_trade_core_biz`].
pub fn social_format_position(inst_type: String, inst_id: String, quantity: String, price: String, contract_val: String) -> Option<SocialPositionFormat> { todo!() }

/// Rust port of native price formatting (`PriceFormat.kt internalFormatPrice:276-318`):
/// per-instrument quote-precision rounding of a price, trailing zeros kept.
/// 
/// - converter dispatch mirrors native `createChargeConverterWithIndex`
/// (`OkFutureBiz.kt:389-397`): linear swap/futures → charge-currency convertor; inverse →
/// USD→fiat convertor (user's fiat); spot/margin/option → their biz price convertor;
/// - `inst_family` falls back to `convert_inst_family` exactly like native
/// (`PriceFormat.kt:270-271`) — so this works even before the biz store warms;
/// - `rounding`: `"up"` / `"down"` per the call site's trade-side policy
/// (`buyUpSellDown`/`buyDownSellUp`, `PriceFormat.kt:320-334`); `"none"`
/// (TradeSide.Nothing → "Do nothing") keeps OKTFormatter's default, which is Down;
/// - zero price bails (`zero_price`) so the Kotlin fallback renders native's `"--"`
/// placeholder (`PriceFormat.kt:162/228/247`).
pub fn social_format_price(inst_type: String, inst_id: String, price: String, rounding: String) -> Option<SocialPriceFormat> { todo!() }

/// Bridge `getFundingRatePrecision` — native `getBiz(SWAP).getBizInfoForId(instId)
/// .getInstFamily()` → `getIndexConfig(instFamily).fundingFeeDig.toSafeInt()`. `None`
/// (empty / cold store / no family / blank dig) → Kotlin falls back to the
/// native read (whose own default is `6`). Only a real `fundingFeeDig` yields `Some`.
pub fn social_funding_rate_precision(inst_id: String) -> Option<i32> { todo!() }

/// A2: instrument brief for sort/group call sites. `None` = store cold /
/// unknown instrument — Kotlin falls back to the intact native `getSuggestedInstrument`
/// read (which native itself null-guards at every consumer).
pub fn social_inst_brief(inst_type: String, inst_id: String) -> Option<SocialInstBrief> { todo!() }

pub fn social_inst_display_info(inst_type: String, inst_id: String) -> Option<SocialInstDisplayInfo> { todo!() }

/// Bridge `fetchInverseFuturesUnit` — inverse-futures display-unit preference ("0"/"1"/"2").
/// `None` when the account-config store is cold → Kotlin falls back to the native read (whose
/// own cold default is `TRADE_UNIT_COIN`).
pub fn social_inverse_futures_unit() -> Option<String> { todo!() }

/// Bridge `getInvestmentCcyWithInstType` NON-spot branch — native
/// `getSuggestedInstrument(instType, instId).getMarginSymbol()`. The SPOT branch is the
/// multi-quote storage path (`OKTTradeReceiveStorage`) and stays native. `None` = empty /
/// cold / unknown / blank margin → Kotlin falls back to the native read (whose own
/// default is the empty string).
pub fn social_investment_ccy(inst_type: String, inst_id: String) -> Option<String> { todo!() }

/// W1: portfolio-margin ("assembly") account check — Rust replacement for native
/// `SocialTradeManager.isAssemblyAccount()` → `TradeDataSettings.isAssemblyAccount()`
/// (`getAccountLevel().eq("4")`, TradeDataSettings.kt:146). Routed directly through the
/// [`SocialTradeCore`] trait after native has selected the Rust path.
/// 
/// `None` when the account-config store is not loaded → Kotlin falls back to the native read —
/// NEVER default this bool: a cold-store default would silently change
/// portfolio-margin users' close-dialog behavior. Guest sessions yield `Some(false)`,
/// equal to native's guest result.
pub fn social_is_assembly_account() -> Option<bool> { todo!() }

/// W4: demo-trading (simulate line) check — Rust replacement for native
/// `ITradeManager.isSimulateLine()` (`TradeCoreManager.kt:441` =
/// `TradeAccountManager.isDemoMode()`); rust equivalent
/// `CTTradeAccountManager::is_demo_mode()` verified identical (token_type ==
/// DemoTrading). Reference: retail-trading `trading_core/real.rs:403`.
pub fn social_is_demo_trading() -> Option<bool> { todo!() }

/// W3: fiat-currency membership check — Rust source for native
/// `getCurrencyListData().map { it.isoCode }.contains(ccy)`
/// (`TotalAssetInteractorHolder.kt:349-353`).
/// 
/// Cold-store discriminator (review C1): a plain `is_some()` would collapse "list not
/// loaded" into "not a fiat" — a silent wrong bool. USD is always present in a loaded
/// currency list, so its absence = store cold → `None` → native fallback.
pub fn social_is_fiat_ccy(ccy: String) -> Option<bool> { todo!() }

/// Bridge native `TradeDataSettings.fetchPositionMode() == SINGLE_POSITION_MODE`
/// (single/net vs long-short position mode; `"net_mode"` / `"long_short_mode"`,
/// `TradeConfigInfoProvider.kt:441` + `TradeConst.kt:116-117`). `None` when the
/// account-config store is cold → Kotlin falls back to the native read (never default
/// the bool — see the trait doc for the cold-store rule).
pub fn social_is_single_position_mode() -> Option<bool> { todo!() }

/// W5 (read): market-close-all confirmation preference — native
/// `TradeDataSettings.fetchMtkCloseAllConfirm()`. `None` when the account-config store is cold
/// → Kotlin falls back to the native read (never default the bool).
pub fn social_mkt_close_all_confirm() -> Option<bool> { todo!() }

/// Bridge the option-expiry lookup in native `CommonFormat.getCoinTitle`'s OPTION
/// branch — `(getBiz(OPTION) as? OkOptionBiz)?.getOptionExpTime(instId)`
/// (`ok_option_biz.rs:242` is the verified Rust equivalent, same store). Returns the
/// RAW millis string exactly as the store carries it; Kotlin keeps its own
/// `toSafeLong()` + date-format + instId-derived fallback chain. `None` = blank instId /
/// cold store / unknown instrument / blank value → native fallback.
pub fn social_option_exp_time(inst_id: String) -> Option<String> { todo!() }

/// Bridge `formatLiquidationPrice` precision — native option charge-currency `priceDig`:
/// `index` = OPTION `getSuggestedInstrument(instId).getInstFamily()` (else
/// `convertInstFamily(instId)`), `priceDig` = `getBiz(OPTION).getChargeCurrencyByReferId(
/// index, USD).priceDig.ifEmpty { DEFAULT_LEGAL_DIGIT }`. Rust returns only the PRECISION;
/// Kotlin keeps native's `price.formatDownToMax(dig)` render (a localization function that
/// survives the OKTradeCore sunset). `None` (empty / cold charge store) → Kotlin
/// falls back to the whole native path.
pub fn social_option_liq_price_dig(inst_id: String) -> Option<i32> { todo!() }

pub fn social_option_quote_precision(inst_id: String) -> Option<SocialOptionQuotePrecision> { todo!() }

/// Bridge `shouldShowTradeConfirmation` (read): order-remind confirmation preference —
/// native `getTradeDataSettings().getOrderRemindConfirm()` (= `confirm.eq(1)`). `None` when
/// the account-config store is cold → Kotlin falls back to the native read. Native's
/// cold-launch value is FALSE (hide), so the `None`-on-cold is load-bearing — never default
/// the bool here (see [`SocialTradeCore::order_remind_confirm`] for why Some(true) would be
/// a regression). Same cold-store discipline as W5.
pub fn social_order_remind_confirm() -> Option<bool> { todo!() }

/// A2: price display symbol — native `getBiz(instType).getPriceDisplaySymbol(instrument)`
/// (`EcoSpotTpslLimitOrderViewModel` init). Rust counterpart exists per-biz
/// (`OkBizTrait::get_price_display_symbol`, e.g. `ok_swap_biz.rs:305`).
/// 
/// `None` = store cold / biz returned no symbol — fall back to native. When
/// the rust instrument lookup misses we deliberately do NOT substitute anything (the
/// native store may still have the instrument, whose display symbol — e.g. spot's
/// remapped display quote — can differ from any family-derived value); callers whose
/// native branch IS the family chain use [`social_family_price_symbol`] instead.
pub fn social_price_display_symbol(inst_type: String, inst_id: String) -> Option<String> { todo!() }

/// Display-ready coin icon URL. `""` means *no icon* — do not fall back.
/// 
/// `symbol` may be empty, in which case the base currency is taken from `inst_id`.
pub fn social_resolved_coin_url(symbol: String, inst_id: String) -> String { todo!() }

/// F1.6: Kotlin-callable view of [`round_pnl`] (the F1.1 helper was Rust-internal only) —
/// the value step of native `internalFormatPNL` (`PNLFormat.kt:211-245`:
/// `convertWithNumber(raw).setKeepZero(true).down()`): per-instrument profit precision,
/// Down, trailing zeros kept. Kotlin keeps the rest of the native chain (color from the
/// rounded value, ICU min/max precision, sign, optional symbol). `None` → native fallback.
/// The rounded value carries EXACTLY the profit precision's decimal count (zeros kept), so
/// Kotlin derives the ICU min-precision from the returned string — no separate field.
pub fn social_round_pnl(inst_type: String, inst_id: String, raw: String) -> Option<String> { todo!() }

/// W5 (write): persist the market-close-all confirmation preference — native
/// `setMtkCloseAllConfirm(value)`. `true` = written (HTTP + cache); `false` = update failed →
/// Kotlin falls back to the native write. Cancellation also resolves to `false`, preserving that
/// fallback contract. Async (network).
pub async fn social_set_mkt_close_all_confirm(value: bool, cancel_token: Arc<OKCancellationToken>) -> bool { todo!() }

/// Bridge `skipTradeConfirmation` (write): persist the order-remind confirmation preference
/// — native `setOrderRemindConfirm(!skip)`. `true` = written (HTTP + cache); `false` = update
/// failed → Kotlin falls back to the native write. Cancellation also resolves to `false`,
/// preserving that fallback contract. Async (network).
pub async fn social_set_order_remind_confirm(value: bool, cancel_token: Arc<OKCancellationToken>) -> bool { todo!() }

/// Bridge `supportedTradeQuoteCcyList` helper — native
/// `biz.getBizInfoForId(instId).getTradeQuoteCcyList().takeIf { it != null &&
/// isSupportTradeReceive() }.orEmpty()`: the quote-ccy list when it is non-null AND the
/// instrument supports trade-receive, else the EMPTY list. `None` (cold / unknown) → Kotlin
/// falls back to the native helper; `Some(list)` (possibly empty) is the
/// resolved list, which the caller feeds into the native `OKTTradeReceiveStorage` selection.
pub fn social_trade_quote_ccy_list(inst_type: String, inst_id: String) -> Option<Vec<String>> { todo!() }

/// F1.5: USD → user's local fiat conversion + precision rounding — the Rust equivalent of
/// native `createUSDToLocalConvertor().toQuoteFormatted(v)` (`OKTUSDChargeConvertor.kt`:
/// quote = user's currency pref (iso/sign/precision), rate = local rate with `"1"`
/// fallback), consumed by `CommonNumberFormat.getLocalCurrencyValueFromUSDT` (precision
/// override 6) and `formatLocalCurrency` (currency-pref precision).
/// 
/// Semantics mirrored from the native chain (`toQuoteFormatted(...).unGroup()`):
/// rounding = OKTFormatter default Down (`OKTFormatter.kt:29`), trailing zeros REMOVED
/// (`keepZero` defaults false, `OKTFormatter.kt:56`; our funnels never call
/// `setKeepZero(true)` — note this differs from the price path's Keep). Blank amount
/// mirrors native `toSafeBigDecimal` → 0 (formats as "0", NOT a fallback). No grouping —
/// native `.unGroup()`s and Kotlin re-localizes.
/// 
/// `value` may carry a `-` sign; Kotlin applies its `toSafeNoSignString` / sign policy
/// exactly as it does on the native result. `symbol` = fiat ISO code (e.g. "EUR"),
/// `sign` = fiat display sign (e.g. "€").
/// 
/// The settings service is a native-injected platform service (registered at app init) —
/// no store warm-up needed, unlike the instrument/coins paths.
pub fn social_usd_to_local(amount: String, precision_override: Option<u8>) -> Option<SocialPriceFormat> { todo!() }

/// Link-compat stub — **DEPRECATED**, do NOT build on it. It reads no configuration; the real
/// switch this used to expose (`rust_trade_core` && `social_use_core_trading_rust`) is gone, and
/// nothing in Rust gates on core-trading any more.
/// 
/// Why it still exists: the two-state refactor deleted this export, but iOS still calls it from
/// `CopyContractSettingsCoreTradingCapabilityService.isEnabled()`
/// (`CopyContractSettingsFeatureFlagProvider.swift`, the only call site in the iOS repo). Android
/// dropped its call in the same change and moved the trade-core state into
/// `RustAbTestManager.isRustEnabled()`; iOS has no equivalent — there is no `rust_trade_core` and
/// no `isRustTradeEnable` anywhere in the iOS OKSocialTrading repo. Deleting the export therefore
/// breaks the OKEx link step, not just the Rust build (MMR 90900, ios:e2e-build:
/// "Undefined symbol: OKCore.socialUseCoreTradingRustEnabled() -> Swift.Bool").
/// 
/// Returns a constant `false` — a deliberate temporary lock, chosen to freeze current production
/// behaviour: the iOS business key `social_trading_copy_contract_settings_rust_migration` is not in
/// gray release, so the iOS Rust leg is not running today and `false` keeps it exactly there.
/// 
/// ⚠️ CONSEQUENCE — read before touching the iOS rollout: while this stub stands, the iOS Rust leg
/// for Copy Contract Settings is pinned OFF from the Rust side. Turning the iOS business key on
/// will have NO effect. Whoever starts that gray release must delete this stub together with the
/// iOS call site FIRST; there is no way to override it from the platform or from remote config.
/// 
/// Scheduled removal: together with the iOS call site — iOS is dropping
/// `CopyContractSettingsCoreTradingCapabilityService` entirely, after which the iOS branch is
/// decided by its business key alone and both sides of this stub go away (TODO: iOS ticket).
pub fn social_use_core_trading_rust_enabled() -> bool { todo!() }

pub fn social_warm_trade_core_biz(inst_type: String) { todo!() }
