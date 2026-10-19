// =============================================================================
// Reconstructed Rust UniFFI IDL & Schema for crate: `buy_sell`
// Extracted from OKX Android v6.194.0 (lib/arm64-v8a/libOKCore.so)
// Total symbols decoded: 637
//   Records: 160 | Enums: 64 | Interfaces: 26
//   Constructors: 8 | Methods: 261 | Functions: 117
// =============================================================================

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

// -----------------------------------------------------------------------------
// 1. Enums (64)
// -----------------------------------------------------------------------------

/// State variant for quick convert
/// 
/// Using flat enum to minimize package size (best practice).
#[derive(Debug, Clone, PartialEq)]
pub enum BSCQuickConvertStateVariant {
    Submitting,
    Success,
    Failed,
}

/// Which asset class the master header is showing.
/// 
/// Crypto keeps the three tabs; Stock offers Buy and Sell only, because Stock
/// Convert does not exist (PRD §4). Which tabs a class offers is a decision, so it
/// is published as [`BscMasterUiState::visible_tabs`] rather than re-derived by
/// each platform.
#[derive(Debug, Clone, PartialEq)]
pub enum BscAssetType {
    Crypto,
    Stock,
}

/// Channel type classification for validation routing.
/// Replaces the old `PaymentMethodInfo { is_balance, is_native }` boolean pair.
#[derive(Debug, Clone, PartialEq)]
pub enum BscChannelType {
    Balance,
    Native,
    ThirdParty,
}

/// Which BSC tab is live. The chip serves all three; only Buy offers recurring.
#[derive(Debug, Clone, PartialEq)]
pub enum BscMasterTab {
    Buy,
    Sell,
    Convert,
}

/// The order types the BSC header can offer.
#[derive(Debug, Clone, PartialEq)]
pub enum BscOrderType {
    OneTime,
    Recurring,
    Trigger,
    TradingBot,
}

/// What the chip reads. Native maps each variant to a string resource; the
/// recurring case carries the backend's own label.
#[derive(Debug, Clone, PartialEq)]
pub enum BscOrderTypeChipLabel {
    InstantOrder,
    Trigger,
    TradingBot,
    /// Carries the frequency key, not a label — the app owns the wording.
    Recurring {
        frequency: String,
    },
    /// Nothing to show — the chip is hidden.
    None,
}

/// Typed, pattern-matchable builder errors. NOT panics, NOT a generic
/// degraded state: the Lua shell maps these onto `entryError` without
/// rendering any preview content.
#[derive(Debug, Clone, PartialEq)]
pub enum BscPreviewError {
    /// Native-category × sell is a structural BE fact, not a client
    /// contract — fail closed instead of silently building a buy-shaped
    /// preview (iOS would echo the side into submit payloads; Android is a
    /// silent dead button with dirty side effects).
    UnsupportedChannelSide {
        category: String,
        side: String,
    },
    /// Absent or unmapped `categoryCode` maps to `ChannelType::Unknown`
    /// and is rejected — never silently routed into the balance plan.
    UnknownChannelCategory {
        category_code: Option<i32>,
    },
    /// The channel is taxonomically known but is not a preview rail
    /// (e.g. cryptoBalance=31) or carries no plan mapping.
    NoPlanForChannel {
        deposit_platform_code: i64,
    },
    /// Skeleton marker: the rail's plan exists in the manifest but its row
    /// assembly is not implemented yet.
    PlanPending {
        rail: String,
    },
    /// BuildReq failed to parse / violated a schema invariant.
    InvalidRequest {
        error_message: String,
    },
}

/// Direction of the header's 24h change, so native picks the trend colour without
/// parsing the ratio it was handed.
#[derive(Debug, Clone, PartialEq)]
pub enum BscPriceTrend {
    Flat,
    Up,
    Down,
}

/// BSC UV entry point (iOS `BuySellUVEntry`).
#[derive(Debug, Clone, PartialEq)]
pub enum BscUvEntry {
    Buy,
    Sell,
    Convert,
    ExchangeTopup,
    BscL1,
    BscL2,
    BscL2Dex,
}

/// Which call site's rule to apply. See the module docs for the table.
#[derive(Debug, Clone, PartialEq)]
pub enum BscUvFrozenPredicate {
    /// iOS (both call sites) and Android's buy/sell button click.
    DexSideMatch,
    /// Android's landing-page entrance only.
    AndroidLandingEntrance,
}

/// How the verdict was reached — diagnostic for the A/B observation.
#[derive(Debug, Clone, PartialEq)]
pub enum BscUvFrozenSource {
    /// A non-blank `actionCode` decided it.
    ActionCode,
    /// `actionCode` was blank and the freeze list decided it (iOS only).
    FreezeList,
    /// Nothing blocked.
    NotFrozen,
}

/// What the pair check concluded.
#[derive(Debug, Clone, PartialEq)]
pub enum BscUvPairOutcome {
    /// The side is available, or the check does not apply on these parameters.
    Passed,
    /// The use-check ran and refuses this side.
    Blocked,
    /// The use-check could not be read. Native decides what that means — the
    /// two platforms and even the two pair checks disagree.
    UseCheckUnavailable,
    /// The use-check returned 200 but carried no value for this side. Native
    /// applies its own default (Android fail-open, iOS fail-closed).
    UseCheckFieldMissing,
    /// The convert from-list could not be read. Reported separately from
    /// `Blocked` even though Android maps both to `Failed`: conflating "the
    /// currency is unsupported" with "the request died" is what makes a
    /// transport blip look like a product decision in the logs.
    SupportTokenUnavailable,
    /// The default-currency read failed under
    /// [`BscUvUseCheckFlavour::InputPagePreload`], where that read is
    /// load-bearing: iOS rethrows and aborts the whole preload
    /// (`preloadForInputPage`'s `catch` block), and — critically — never issues
    /// the use-check request after it. So this outcome means NO use-check ran,
    /// and native must map it to its abort path. Never produced under
    /// [`BscUvUseCheckFlavour::Ladder`], where the same failure silently falls
    /// back to the supplied quote (`getOrNull()`), exactly as Android does.
    DefaultCurrencyUnavailable,
}

/// Which feature axis an entry's country check reads.
#[derive(Debug, Clone, PartialEq)]
pub enum BscUvRestrictionAxis {
    FiatPayment,
    Convert,
    /// `exchangeTopup` only — unreachable today, see the module docs.
    SmartAccount,
}

/// Destination the check is guarding (iOS `TargetPage`).
#[derive(Debug, Clone, PartialEq)]
pub enum BscUvTargetPage {
    /// `.bsc(tradeType)` — carries `trade_type`.
    Bsc,
    /// `.bscPair(param)` — carries `trade_type`.
    BscPair,
    CryptoList,
    InputPage,
    Convert,
    None,
}

/// Destination tab the ladder is heading for (Android `TargetTab`).
#[derive(Debug, Clone, PartialEq)]
pub enum BscUvTargetTab {
    Buy,
    Sell,
    Convert,
}

/// Trade side (iOS `OKPaymentTradeType`).
#[derive(Debug, Clone, PartialEq)]
pub enum BscUvTradeType {
    Buy,
    Sell,
    Convert,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BuySellCryptoAssetListStatus {
    Idle,
    Loading,
    Loaded,
    Error,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BuySellError {
    NetworkFailed {
        http_status: i32,
        business_code: i32,
        error_code: i32,
        error_message: String,
        raw_response_body: Option<String>,
    },
    ParseFailed {
        error_message: String,
    },
    InternalError {
        error_message: String,
    },
}

/// The validation flow for the input page. Each flow shares the core amount
/// validation and adds its own checks.
#[derive(Debug, Clone, PartialEq)]
pub enum BuySellInputFlow {
    /// One-time buy/sell (default). The only flow with the large-quote CTA.
    OneTime,
    /// Trigger order — adds trigger-price validation; channel
    /// selection restricted to trigger-supported channels.
    Trigger,
    /// Recurring buy — same input surface as one-time, but channel selection
    /// is restricted to recurring-supported channels and the large-quote CTA
    /// is suppressed (native gates it to ONE_TIME).
    Recurring,
}

/// Canonical input mode for the buy/sell input page.
/// 
/// ⚠️ Trap: the backend `standard` value is the **inverse** of the
/// legacy Android `InputType.id` (`FIAT_INPUT=0`, `CRYPTO_INPUT=1`). This enum
/// owns the single canonical mapping via [`BuySellInputMode::standard`]
/// (`Fiat => 1`, `Crypto => 0`), matching `BscConstants.FIAT_INPUT_MODE=1 /
/// CRYPTO_INPUT_MODE=0`. Never send the legacy `InputType.id` as `standard`.
#[derive(Debug, Clone, PartialEq)]
pub enum BuySellInputMode {
    /// User enters the fiat (quote) amount. Backend `standard` = 1.
    Fiat,
    /// User enters the crypto (base) amount. Backend `standard` = 0.
    Crypto,
}

/// Content-state variant for the input page.
#[derive(Debug, Clone, PartialEq)]
pub enum BuySellInputStatus {
    /// Initial/idle state.
    Idle,
    /// INITIAL trade-suggestion request in flight — no data yet. This is the
    /// only status that should show the full-page shimmer.
    LoadingTradeSuggestion,
    /// A re-fetch (currency/channel change, refresh) is in flight while `data`
    /// still holds the previous payload — render the data, no shimmer.
    Refreshing,
    /// Trade suggestion loaded.
    LoadedTradeSuggestion,
    /// An error occurred.
    Error,
}

/// Canonical page trade type. The interactor derives this once from the wire
/// request so native consumers never parse trade-type strings.
#[derive(Debug, Clone, PartialEq)]
pub enum BuySellInputTradeType {
    Buy,
    Sell,
    Convert,
}

/// Content state variant (flat enum).
/// 
/// Using flat enum + Record pattern to minimize package size.
#[derive(Debug, Clone, PartialEq)]
pub enum BuySellOrderDetailContentStateVariant {
    /// Initial idle state
    Idle,
    /// Loading order details
    Loading,
    /// Successfully loaded order details
    Success,
    /// Error occurred while loading
    Error,
}

/// Order status codes from backend.
#[derive(Debug, Clone, PartialEq)]
pub enum BuySellOrderStatus {
    /// Order pending (status = 0)
    Pending,
    /// Order processing (status = 1)
    Processing,
    /// Order failed (status = 2)
    Failed,
    /// Order completed successfully (status = 3)
    Completed,
    /// Order refunded (status = 4)
    Refunded,
    /// Order expired (status = 9)
    Expired,
    /// Order cancelled (status = 10)
    Cancelled,
    /// Unknown status
    Unknown {
        code: i32,
    },
}

/// Order type (one-time or recurring).
#[derive(Debug, Clone, PartialEq)]
pub enum BuySellOrderType {
    OneTime,
    Recurring,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BuySellPreviewQuoteStatus {
    Idle,
    Loading,
    Loaded,
    Error,
}

/// Quote request lifecycle (flat pattern).
#[derive(Debug, Clone, PartialEq)]
pub enum BuySellQuoteStatus {
    /// No quote requested (e.g. amount is zero).
    Idle,
    /// Quote request in flight.
    Loading,
    /// Quote loaded.
    Loaded,
    /// Quote request failed.
    Error,
}

/// Input token classification (CeFi vs DeFi).
#[derive(Debug, Clone, PartialEq)]
pub enum BuySellTokenType {
    Cefi,
    Defi,
}

/// Trade type for buy/sell orders.
#[derive(Debug, Clone, PartialEq)]
pub enum BuySellTradeType {
    Buy,
    Sell,
    Convert,
}

/// Channel category — typed at the FFI boundary. Matches OKCRCore's Kotlin
/// `ChannelCategoryCode` (ThirdParty=2, Balance=3, Native=4).
#[derive(Debug, Clone, PartialEq)]
pub enum ChannelCategoryCode {
    ThirdParty,
    Balance,
    Native,
}

/// Content state variant enum (flat pattern for small package size).
#[derive(Debug, Clone, PartialEq)]
pub enum ContentStatus {
    /// Initial/idle state
    Idle,
    /// Loading from currencies
    LoadingFromList,
    /// From currencies loaded
    LoadedFromList,
    /// Loading to currencies
    LoadingToList,
    /// To currencies loaded
    LoadedToList,
    /// Loading quote
    LoadingQuote,
    /// Quote loaded
    LoadedQuote,
    /// Error occurred
    Error,
}

/// Action of user on convert bottomsheet (enumerable: dismiss, reviewdetails, done)
#[derive(Debug, Clone, PartialEq)]
pub enum ConvertBottomSheetAction {
    Dismiss,
    ReviewDetails,
    Done,
}

/// Execution status variant enum (flat pattern).
#[derive(Debug, Clone, PartialEq)]
pub enum ConvertExecutionStatus {
    /// Idle state (before execution)
    Idle,
    /// Submitting trade to server
    Submitting,
    /// Trade executed successfully
    Success,
    /// Trade execution failed
    Failed,
}

/// Simple or exchange mode (enumerable: simple, exchange)
#[derive(Debug, Clone, PartialEq)]
pub enum ConvertModeType {
    Simple,
    Exchange,
}

/// Status of the convert order (enumerable: success, failure)
#[derive(Debug, Clone, PartialEq)]
pub enum ConvertOrderStatus {
    Success,
    Failure,
}

/// Convert idle cash result (enumerable: success, fail, processing)
#[derive(Debug, Clone, PartialEq)]
pub enum ConvertResult {
    Success,
    Fail,
    Processing,
}

/// Indicates which unit the user is currently inputting.
#[derive(Debug, Clone, PartialEq)]
pub enum InputAmountType {
    Fiat,
    Crypto,
}

/// Which surface owns the page's error/banner area, in priority order
/// (native runs the amount UI first, then the DEX status observer overrides
/// the shared views; migration replaces everything).
#[derive(Debug, Clone, PartialEq)]
pub enum InputErrorType {
    /// No error surface — the area is empty.
    None,
    /// VelaFi/ARS migration gate — the banner replaces the buy controls
    /// (highest priority; suppresses the error view AND the CTA slot).
    Migration,
    /// Offshore virtual-account setup (ChannelVirtualAccountNone / Pending /
    /// Failed): the setup banner replaces the input surfaces — no error view,
    /// no CTA slot, no input mark (native BuyFragmentV3 velafiBanner branches).
    /// The display policy picks copy
    /// + CTA from `validation_result` (None/Failed → Enable; Pending → none).
    VirtualAccountSetup,
    /// Trigger price on the wrong side of the market; the trigger
    /// box is marked and the error view shows the deviation message.
    TriggerExceedsLimit,
    /// A failing DEX trade status owns the error view (excludes the neutral
    /// -banner codes and the Activate-Wallet CTA swap).
    DexStatus,
    /// An amount-validation error owns the error view.
    AmountError,
    /// The selected stock cannot be bought. The page still renders and the tab
    /// stays selectable; the error view owns the message and the CTA is dead.
    /// 
    /// Split by side rather than one variant plus the trade type, so the copy
    /// follows the support flag and no consumer re-derives which side is closed.
    StockBuyUnsupported,
    /// The selected stock cannot be sold; see [`Self::StockBuyUnsupported`].
    StockSellUnsupported,
}

/// Input type for amount entry.
#[derive(Debug, Clone, PartialEq)]
pub enum InputType {
    /// User enters crypto amount (e.g., BTC amount)
    CryptoInput,
    /// User enters fiat amount (e.g., USD amount)
    FiatInput,
}

/// Validation outcomes for fiat/crypto input amounts.
/// 
/// These variants mirror the Kotlin `InputValidationResult` enum defined
/// in `InputBaseViewModel` so we can gradually move the validation
/// decision tree into Rust without changing the external behavior.
#[derive(Debug, Clone, PartialEq)]
pub enum InputValidationResult {
    /// Input equals zero.
    Zero,
    /// User has not completed login or KYC steps yet.
    PreLoginPreKyc,
    /// Large quote CTA shown, user can proceed via chat.
    LargeQuoteContinueToChat,
    /// Large quote CTA shown, channel limit exceeded and chat is required.
    LargeQuoteOverChannelLimit,
    /// DEX: Remaining 24h balance limit is below the transaction minimum.
    Dex24HourTradingLimitReachedBalance,
    /// DEX: Remaining 24h native limit is below the transaction minimum.
    Dex24HourTradingLimitReachedNative,
    /// DEX: Quote amount unavailable (pricing error).
    DexQuoteAmountUnavailable,
    /// Selected xStock is paused for a corporate action. This is a hard block
    /// and outranks amount, account, and channel validation.
    XstockCaTradePaused,
    /// Balance < min < max scenario – user input outside bounds.
    InputBalanceMinMax,
    BalanceInputMinMax,
    BalanceMinInputMax,
    BalanceMinMaxInput,
    /// min <= balance <= max family – ordering differs by case.
    InputMinBalanceMax,
    MinInputBalanceMax,
    MinBalanceInputMax,
    MinBalanceMaxInput,
    /// min < max < balance family – ordering differs by case.
    InputMinMaxBalance,
    MinInputMaxBalance,
    /// CeDeFi scenarios – dex max combined with classic min/max ordering.
    MinMaxInputBalance,
    MinMaxDexMaxInputBalance,
    MinMaxBalanceInput,
    MinMaxDexMaxBalanceInput,
    /// Native channel limits.
    MaxChannelLimitLessThanBothMin,
    MaxChannelLimit,
    MinChannelLimit,
    MaxTradeLimit,
    MinTradeLimit,
    NativeChannelValid,
    /// Third-party channel limits.
    InputMinMax,
    MinMaxInput,
    ThirdPartyValid,
    /// Offshore (VelaFi/TransFi) virtual account not created yet — the setup
    /// banner with the Enable CTA replaces the input surfaces (native
    /// CHANNEL_VIRTUAL_ACCOUNT_NONE; gated BEFORE amount validation).
    ChannelVirtualAccountNone,
    /// Virtual account creation in progress — setup-pending banner, page
    /// auto-refreshes (native CHANNEL_VIRTUAL_ACCOUNT_PENDING).
    ChannelVirtualAccountPending,
    /// Virtual account creation failed — same banner + Enable CTA surface as
    /// `ChannelVirtualAccountNone` (native CHANNEL_VIRTUAL_ACCOUNT_FAILED).
    ChannelVirtualAccountFailed,
    /// Sell virtual account exists but is inactive and must be enabled.
    ChannelVirtualAccountInactive,
    /// Sell virtual account is ready but no payout bank account is linked.
    ChannelNoBankAccount,
    /// Native ACH channel whose account got disconnected — relink banner with
    /// the Connect CTA replaces the amount validation and hides the preview
    /// (native ACH_DISCONNECTED; gated BEFORE amount validation).
    AchDisconnected,
}

/// Payment-method category — typed at the FFI boundary. Matches the
/// §14-corrected codes (Cash=0, Stablecoin=1, Crypto=2).
#[derive(Debug, Clone, PartialEq)]
pub enum PaymentMethodCategory {
    Cash,
    Stablecoin,
    Crypto,
}

/// Preview-button variant — the label and tap routing stay native (the
/// fragment's exhaustive `when`); this decides WHICH variant renders
/// (native setupPreviewButton).
#[derive(Debug, Clone, PartialEq)]
pub enum PreviewCtaType {
    /// Pre-login: launch the login flow.
    Login,
    /// Pre-KYC: launch sub-KYC onboarding.
    Verify,
    /// Offshore stablecoin flow (no channels): open the deposit crypto list.
    DepositStablecoin,
    /// DEX wallet not activated (status 106005): route to cedefi activation.
    ActivateWallet,
    /// Unbound add-account default channel (native isUnboundDefaultChannel):
    /// "Add account", always enabled — tap routing stays native (Zen launches
    /// the add-account flow; other codes stay sheet-only).
    AddAccount,
    /// Stock whose risk disclosure has not been accepted: "Unlock stocks".
    /// Native reports the unmet requirement through
    /// `set_xstock_requirement_pending`; this takes the slot Preview would
    /// otherwise hold, and is enabled without a valid amount because meeting
    /// the requirement is a precondition for trading at all.
    UnlockStocks,
    /// Normal path: frozen check → precheck → order preview. Recurring copy
    /// is a localization choice — native picks it from `input_flow`.
    Preview,
}

/// What occupies the CTA slot (mutually exclusive by construction — native
/// validInputsAndSetupErrorViewIfNeeded).
#[derive(Debug, Clone, PartialEq)]
pub enum PrimaryActionType {
    /// Quick-amount chips row (zero input + fiat mode + presets, CeFi only).
    QuickAmountChips,
    /// Quick-Sell percentage chips (25/50/75/Max) for a zero Sell input.
    SellPercentageChips,
    /// Trigger percentage chips (trigger box focused, no trigger price yet).
    TriggerPercentageChips,
    /// The preview button (variant in `preview_cta_type`).
    PreviewButton,
    /// Nothing — an error/banner claimed the slot, or migration is blocking.
    Hidden,
}

/// Content state variant for pro buy/sell order history list.
/// 
/// Using flat enum to minimize package size (best practice).
#[derive(Debug, Clone, PartialEq)]
pub enum ProBuySellOrderHistoryListContentStateVariant {
    Idle,
    Loading,
    Loaded,
    Empty,
    Error,
}

/// Quick amount selection types.
#[derive(Debug, Clone, PartialEq)]
pub enum QuickAmountType {
    /// 25% of available balance
    Percent25,
    /// 50% of available balance
    Percent50,
    /// 75% of available balance
    Percent75,
    /// 100% of available balance (ALL)
    All,
}

/// Content state variant for recurring buy plan detail
/// 
/// Using flat enum to minimize package size (best practice).
#[derive(Debug, Clone, PartialEq)]
pub enum RecurringBuyPlanDetailContentStateVariant {
    Idle,
    Loading,
    Loaded,
    Error,
}

/// Error case enum for banner logic
#[derive(Debug, Clone, PartialEq)]
pub enum RecurringBuyPlanDetailErrorCase {
    /// Low balance warning (no failures yet)
    LowBalanceWarning,
    /// Low balance with 1-2 failures (warning before auto-pause)
    LowBalanceFailWarning,
    /// Low balance with 3+ failures (auto-paused)
    LowBalanceAutoPaused,
    /// Payment method issue with 1-2 failures (warning before auto-pause)
    PaymentMethodIssueFailWarning,
    /// Payment method issue with 3+ failures (auto-paused)
    PaymentMethodIssueAutoPaused,
    /// No OTC quote with 1-2 failures (warning before auto-pause)
    NoOtcQuoteFailWarning,
    /// No OTC quote with 3+ failures (auto-paused)
    NoOtcQuoteAutoPaused,
}

/// Status change type
#[derive(Debug, Clone, PartialEq)]
pub enum RecurringBuyPlanDetailStatusChangeType {
    /// Plan was resumed (from paused to active)
    Resumed,
    /// Plan was paused (from active to paused)
    Paused,
    /// Plan was canceled
    Canceled,
    /// Plan was updated (amount or frequency changed, but status unchanged)
    PlanUpdated,
    /// Plan update failed (API error)
    PlanUpdateError,
}

/// Content state variant for recurring buy plan list
/// 
/// Using flat enum to minimize package size (best practice).
#[derive(Debug, Clone, PartialEq)]
pub enum RecurringBuyPlanListContentStateVariant {
    Idle,
    Loading,
    Loaded,
    Empty,
    Error,
}

/// Content state variant for recurring buy plan order list
/// 
/// Using flat enum to minimize package size (best practice).
#[derive(Debug, Clone, PartialEq)]
pub enum RecurringBuyPlanOrderListContentStateVariant {
    Idle,
    Loading,
    Loaded,
    Empty,
    Error,
}

/// Order status enum
#[derive(Debug, Clone, PartialEq)]
pub enum RecurringBuyPlanOrderStatus {
    Pending,
    Success,
    Failed,
    Canceled,
    Unknown,
}

/// Status tag type for UI display
#[derive(Debug, Clone, PartialEq)]
pub enum RecurringBuyPlanOrderStatusTag {
    None,
    Pending,
    Failed,
}

/// Rounding mode for decimal formatting
#[derive(Debug, Clone, PartialEq)]
#[repr(i32)]
pub enum RoundingMode {
    /// Round half up (0.5 rounds to 1)
    HalfUp = 0,
    /// Round down (truncate)
    Down = 1,
    /// Round up (ceiling)
    Up = 2,
}

/// How the currently-selected channel was chosen (selection-engine source tag).
#[derive(Debug, Clone, PartialEq)]
pub enum SelectionSource {
    /// Matched the user's explicit selection.
    User,
    /// Backend-flagged best channel. Intentionally never emitted by the
    /// selection engine today (the backend "best" flag folds into `First`
    /// ordering — see selection/engine.rs); kept for wire stability, so
    /// consumers must not branch on it.
    Best,
    /// First available channel (fallback).
    First,
    /// No channel could be selected.
    None,
}

/// Quick-Sell percentage presets (native SellInputViewModel quick-sell chips).
/// Each derives the input amount from the selected asset's available CRYPTO
/// balance; `Max` is 100% of that balance. Sell-only — Buy has no
/// quick-percentage affordance (it uses absolute fiat presets + Buy Max), so a
/// Buy dispatch of the quick-percentage intent is an inert no-op.
#[derive(Debug, Clone, PartialEq)]
pub enum SellQuickPercent {
    /// 25% of the available crypto balance.
    Percent25,
    /// 50% of the available crypto balance.
    Percent50,
    /// 75% of the available crypto balance.
    Percent75,
    /// 100% of the available crypto balance (Max).
    Max,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TradeType {
    Buy,
    Sell,
    Convert,
}

/// Trend enum for price changes
#[derive(Debug, Clone, PartialEq)]
pub enum Trend {
    Up,
    Down,
    Flat,
}

/// Result of a cancel trigger order action.
#[derive(Debug, Clone, PartialEq)]
pub enum TriggerOrderDetailCancelResult {
    /// No cancel action in progress.
    Idle,
    /// Cancel request is in flight.
    Loading,
    /// Cancel completed successfully.
    Success,
    /// Cancel failed.
    Error {
        message: String,
    },
}

/// Content state variant for trigger order detail.
#[derive(Debug, Clone, PartialEq)]
pub enum TriggerOrderDetailContentStateVariant {
    Idle,
    Loading,
    Loaded,
    Error,
}

/// Content state variant for trigger order list.
/// 
/// Using flat enum to minimise package size (best practice).
#[derive(Debug, Clone, PartialEq)]
pub enum TriggerOrderListContentStateVariant {
    Idle,
    Loading,
    Loaded,
    Empty,
    Error,
}

/// Execution status of a trigger order.
#[derive(Debug, Clone, PartialEq)]
pub enum TriggerOrderStatus {
    /// Order is set and waiting for the trigger price to be met.
    Pending,
    /// Trigger price has been met; order is executing.
    Processing,
    /// Order has been fully executed.
    Completed,
    /// Order was cancelled by the user.
    Cancelled,
    /// Order failed to execute.
    Failed,
    /// Order has expired.
    Expired,
    /// Unknown status value from API.
    Unknown,
}

/// Result of the trigger-price check (independent of the amount validation,
/// mirroring native's `hasTriggerError` signal).
#[derive(Debug, Clone, PartialEq)]
pub enum TriggerPriceValidation {
    /// Trigger price is valid (or not yet entered).
    Valid,
    /// Trigger price is on the wrong side / within the min deviation.
    ExceedsLimit,
}

/// Input validation state enum (flat pattern).
/// Maps to Android's InputValidationStateV3 with 10+ states.
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationStatus {
    /// Input is zero
    Zero,
    /// Input is zero and balance below minimum
    ZeroAndBalanceBelowMin,
    /// User not logged in or not KYC verified
    PreLoginPreKyc,
    /// Token balance < min AND input < min
    TokenBalanceBelowMinInputBelowMin,
    /// Token balance < min AND input >= min
    TokenBalanceBelowMinInputAboveMin,
    /// Token balance >= min AND input < min
    TokenBalanceAboveMinInputBelowMin,
    /// Token balance >= min AND input >= min (valid)
    TokenBalanceAboveMinInputAboveMin,
    /// Input exceeds maximum balance
    InputAboveMaxBalance,
    /// Input exceeds maximum limit
    InputAboveMaxLimit,
    /// Input exceeds both max balance and max limit
    InputAboveMaxBalanceAndMaxLimit,
    /// Valid state: input >= min AND input <= max balance
    MinInputMaxBalance,
}

/// Verification states mirrored from `OKPaymentService.VerificationStatus`.
/// 
/// Each variant encodes a priority so that higher-risk states (pre-login/kyc)
/// can be short-circuited before running expensive validation logic.
#[derive(Debug, Clone, PartialEq)]
pub enum VerificationStatus {
    /// User has not logged in yet.
    PreLogin,
    /// User is logged in but has not completed KYC.
    PreKyc,
    /// User is fully verified (normal state).
    Valid,
}

// -----------------------------------------------------------------------------
// 2. Records / Data Models (160)
// -----------------------------------------------------------------------------

/// Content state for quick convert
/// 
/// Using flat enum + Record pattern to minimize package size (best practice).
/// - `data` is only valid when `variant == Success`
#[derive(Debug, Clone, PartialEq)]
pub struct BSCQuickConvertContentState {
    pub variant: BSCQuickConvertStateVariant,
    /// Convert result data when variant is Success (None otherwise)
    pub data: Option<BSCQuickConvertData>,
}

/// Data payload for successful quick convert.
#[derive(Debug, Clone, PartialEq)]
pub struct BSCQuickConvertData {
    pub base_currency: String,
    pub base_amount: String,
}

/// State for the Interactor.
#[derive(Debug, Clone, PartialEq)]
pub struct BSCQuickConvertState {
    pub content: BSCQuickConvertContentState,
}

/// One selectable recurring frequency, mapped from the backend response.
#[derive(Debug, Clone, PartialEq)]
pub struct BscFrequencyOption {
    /// Backend `code`, the identity used when the Lua picker reports back.
    pub code: String,
    /// Backend `frequency` key (DAILY / WEEKLY / …). The app maps this to its own
    /// label; a key it has no mapping for is dropped natively, so this crate keeps
    /// every row the backend returns.
    pub frequency: String,
    /// Backend-localized name. NOT what the UI displays — it is carried onward as
    /// the order payload's frequency field (native `RecurringBuyOrderType.frequency`).
    pub localized_name: String,
    /// Epoch millis, as the backend sends it.
    pub next_purchase_time: Option<i64>,
    pub selected: bool,
}

/// What the entry point asked for, applied once a channel can support it.
#[derive(Debug, Clone, PartialEq)]
pub struct BscMasterEntry {
    pub order_type: BscOrderType,
    /// Frequency requested by a deeplink; empty when none.
    pub deeplink_frequency: String,
    /// Frequency requested by entry params; empty when none.
    pub param_frequency: String,
}

/// How the order-type domain is set up for this app build: what is switched on,
/// decided natively and passed in, so this crate reads no experiment or service
/// state and stays pure. Not a selection — see `select_order_type` for that.
#[derive(Debug, Clone, PartialEq)]
pub struct BscMasterParameters {
    /// Trigger order is enabled for the app (AB flag AND the service check).
    pub trigger_enabled: bool,
    /// The order-type selector is enabled; when false the chip shows the
    /// recurring frequency label regardless of the selected type.
    pub order_type_selector_enabled: bool,
    /// Stock trading offers order types beyond instant in this market — onshore
    /// EEA and UAE do, offshore does not in this release (PRD §4, §11.1). Decided
    /// natively like the two switches above, so no region or experiment lookup
    /// leaks into this crate.
    pub stock_order_types_enabled: bool,
}

/// Host/session values shared by the BSC child pages but not rendered by the
/// order-type header. Keeping them together prevents the interactor's domain
/// state from becoming a flat bag of platform fields.
#[derive(Debug, Clone, PartialEq)]
pub struct BscMasterSessionState {
    pub is_otc_agent_chat_active: bool,
    pub payment_method_for_tracking: Option<String>,
    pub trading_bot_inst_id: Option<String>,
    /// Consumable entry effect; cleared after the Buy page handles it.
    pub show_add_account_toast: bool,
}

/// Observable state for the BSC order-type header.
#[derive(Debug, Clone, PartialEq)]
pub struct BscMasterSnapshot {
    /// What native pushes in: live tab, asset type, channel and DEX selection.
    pub master_state: BscMasterState,
    /// What the app has switched on, supplied by native.
    pub parameters: BscMasterParameters,
    /// Requested order type, applied once the tab and channel can honour it.
    pub pending_order_type: BscOrderType,
    /// Entry/user frequency preference. Deeplink wins over the parameter; a
    /// recurring entry with neither uses the native-parity weekly default.
    pub preferred_frequency: Option<String>,
    /// The order type actually in force.
    pub selected_order_type: BscOrderType,
    /// The order type Crypto was last on, parked when the class is left and restored when it
    /// comes back. None until it has been left once. Kept per class because the two offer
    /// different sets, so one class's pick is not an answer for the other.
    pub crypto_order_type: Option<BscOrderType>, // default = None
    /// The order type Stock was last on. See [`Self::crypto_order_type`].
    pub stock_order_type: Option<BscOrderType>, // default = None
    /// Frequencies from the backend, with the selected one marked.
    pub frequency_options: Vec<BscFrequencyOption>,
    /// User-confirmed start date, epoch ms. 0 means no preference.
    pub start_date_ms: i64,
    /// Non-render host/session values shared by the input pages.
    pub session_state: BscMasterSessionState,
    /// The stock the header is on, as native selected it. None until the page's trade
    /// suggestion resolves a default pair or the user picks one — native shows its own
    /// placeholder meanwhile.
    pub selected_stock: Option<BscStockAsset>,
    /// Latest ticker price for the selected stock, raw — native formats it.
    pub ticker_price: String,
    /// Latest 24h change ratio, raw. The sign becomes the header's trend.
    pub ticker_change_ratio: String,
    /// Pay with (Buy) or Receive (Sell) on the live page — the other half of the
    /// PRD §7 pair rule, reported up by the page that owns it.
    pub counterpart_currency: String,
    /// Whether the selected stock and [`Self::counterpart_currency`] form an
    /// orderbook pair available in this region. A native lookup, pushed in.
    pub orderbook_pair_available: bool,
    /// What native paints. Recomputed after every mutation.
    pub ui_state: BscMasterUiState,
}

/// Everything the host pushes in when its context changes, in one record.
/// 
/// One call, one mutation, one render-plan recompute — the four values arrive
/// together and are read together, so pushing them separately would publish
/// intermediate frames where the tab has moved but the channel has not.
#[derive(Debug, Clone, PartialEq)]
pub struct BscMasterState {
    pub tab: BscMasterTab,
    pub asset_type: BscAssetType,
    /// `None` before the live tab has resolved a channel.
    pub channel: Option<BscSelectedChannel>,
    pub dex_selected: bool,
}

/// Everything native paints for this domain, recomputed on every state change.
#[derive(Debug, Clone, PartialEq)]
pub struct BscMasterUiState {
    /// Absorbs the whole visibility rule: a supported multi-type tab with a
    /// resolved channel and no DEX token.
    pub chip_visible: bool,
    pub chip_label: BscOrderTypeChipLabel,
    /// The live tab. Carried so the pager renders from state rather than being
    /// told to move — a swipe reports in, and every render paints the result.
    pub tab: BscMasterTab,
    pub selected_order_type: BscOrderType,
    /// Order type accepted by the preview/order payload. Trading Bot and any
    /// non-order selection book as OneTime.
    pub order_payload_type: BscOrderType,
    /// Chooser rows, enabled ones first (native sorted by enabled descending).
    pub items: Vec<BscOrderTypeItem>,
    pub frequency_options: Vec<BscFrequencyOption>,
    /// Picker rows. A cadence is preselected only while Recurring is live.
    pub frequency_picker_options: Vec<BscFrequencyOption>,
    /// The committed cadence, if any, for preview/order handoff.
    pub selected_frequency: Option<BscFrequencyOption>,
    /// Full recurring eligibility, including tab, channel and DEX constraints.
    pub recurring_supported: bool,
    /// User-confirmed recurring start date; zero means no preference.
    pub start_date_ms: i64,
    /// The live asset class.
    pub asset_type: BscAssetType,
    /// Which tabs this asset class offers, in render order: three for Crypto, Buy
    /// and Sell only for Stock (PRD §4). The pager builds from this rather than
    /// hard-coding three pages.
    pub visible_tabs: Vec<BscMasterTab>,
    /// Whether the order-type selector may be offered at all for the live asset
    /// class. Offshore Stock is instant-only, so it reads false there (PRD §11.1).
    /// [`Self::chip_visible`] is still the render answer — this is the class-level
    /// gate that feeds it.
    pub order_type_selector_visible: bool,
    /// Whether the input page draws the Crypto/Stock switch itself. Stock carries the
    /// switch above the pager, so its pages leave the in-card tabs hidden.
    pub in_page_asset_tabs_visible: bool,
    /// Whether the base currency is picked on the input page. Crypto pairs the token with
    /// the payment row; Stock picks it in the header, so the page shows no token row.
    pub base_currency_selector_visible: bool,
    /// The header strip; `None` whenever it should not be shown — Crypto, or Stock
    /// before a stock resolves.
    pub stock_header: Option<BscStockHeader>,
    /// Whether the selected stock and the page's counterpart currency form an
    /// orderbook pair in this region (PRD §7).
    pub kline_entry_visible: bool,
    /// Instrument the K-line entry opens, e.g. `xNFLX-USDT`; empty when hidden.
    pub kline_inst_id: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BscNativeB2cQuoteRequest {
    pub base_currency: String,
    pub base_amount: String,
    pub quote_currency: String,
    pub quote_amount: String,
    pub trade_type: String,
    pub payment_method: String,
    pub sub_payment_method: Option<String>,
    pub standard: i32,
    pub quote_side: bool,
    pub amount: String,
    pub deposit_name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BscNativeBuyCryptoListRequest {
    pub lite_app: String,
    pub crypto_type: Option<String>,
    pub fund_flow_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BscNativeCacheEventRequest {
    pub event: String,
    pub params_json: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BscNativeCampaignRequest {
    pub campaign_code: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BscNativeConvertCurrencyPairsRequest {
    pub from_ccy: String,
    pub to_ccy: String,
    pub settle_source_type: Option<i32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BscNativeConvertFromListRequest {
    pub local_currency: String,
    pub to: Option<String>,
    pub settle_source_type: Option<i32>,
    pub filter_funding_balance: Option<bool>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BscNativeConvertToListRequest {
    pub from: Option<String>,
    pub local_currency: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BscNativeCurrencyBalanceRequest {
    pub fund_flow_type: Option<String>,
    pub currency: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BscNativeCurrencyPairSuggestionRequest {
    pub valuation_unit: String,
    pub symbol: Option<String>,
    pub context_type: Option<i32>,
    pub settle_source_type: Option<i32>,
    pub filter_funding_balance: Option<bool>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BscNativeDefaultCurrencyRequest {
    pub trade_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BscNativeDexDetailsRequest {
    pub tokens: Vec<BscNativeDexDetailsTokenRequest>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BscNativeDexDetailsTokenRequest {
    pub chain_id: String,
    pub token_contract_address: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BscNativeDexQuoteRequest {
    pub platform_code: String,
    pub trade_type: String,
    pub to_currency: String,
    pub from_amount: String,
    pub from_currency: String,
    pub token_address: String,
    pub token_decimal: Option<i32>,
    pub chain_index: i32,
    pub slippage_tolerance: Option<String>,
    pub is_order_confirm: bool,
    pub reference_dex_quote_id: Option<String>,
    pub sub_order_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BscNativeDexSearchTokensRequest {
    pub key: String,
    pub period_type: Option<String>,
    pub filter_chain_tag: Option<String>,
    pub filter_cex_listing: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BscNativeDexTokenBaseInfoRequest {
    pub chain_index: String,
    pub base_token_address: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BscNativeDexTradeStatusRequest {
    pub chain_index: String,
    pub token_address: String,
    pub trade_type: String,
    /// Opaque session-key blob. `None` is sent as an empty string today --
    /// the wire keeps iOS's existing `signExtInfo:""` behavior until the
    /// backend confirms that omitting the key is equivalent.
    pub sign_ext_info: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BscNativeExchangeQuoteRequest {
    pub base_ccy: String,
    pub quote_ccy: String,
    pub rfq_sz: String,
    pub rfq_sz_ccy: String,
    pub side: String,
    pub settle_source_type: i32,
    pub estimate_balance_usage: Option<bool>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BscNativeIndicativeQuoteRequest {
    pub inst_id: String,
    pub side: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BscNativeLifecyclePlanEventRequest {
    pub plan_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BscNativePaginationRequest {
    pub page_num: Option<i32>,
    pub page_size: Option<i32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BscNativePaymentAccountDeleteRequest {
    pub category_code: Option<i32>,
    pub deposit_platform_code: String,
    pub payment_method: String,
    pub payment_account_id: Option<String>,
    pub user_bank_id: Option<String>,
    pub card_number: Option<String>,
    pub deposit_to_withdraw: Option<String>,
}

/// The account/query endpoint serves two query modes; callers must provide
/// exactly one complete pair — `channel_code`+`country_code` (channel
/// availability by country) or `deposit_platform_code`+`payment_account_id`
/// (saved-account lookup). Mixed or partial input is rejected at the bridge.
#[derive(Debug, Clone, PartialEq)]
pub struct BscNativePaymentAccountQueryRequest {
    pub channel_code: Option<String>,
    pub country_code: Option<String>,
    pub deposit_platform_code: Option<String>,
    pub payment_account_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BscNativePaymentBannersRequest {
    pub flow_types: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BscNativePaymentMethodsRequest {
    pub fund_flow_type: Option<String>,
    pub base_amount: Option<String>,
    pub base_currency: Option<String>,
    pub quote_amount: Option<String>,
    pub quote_currency: Option<String>,
    pub trade_type: String,
    pub standard: Option<i32>,
    pub category_code: Option<i32>,
    pub request_currency: Option<String>,
    pub multiple_currency: Option<bool>,
    pub chain_index: Option<String>,
    pub token_address: Option<String>,
    pub include_funding_balance: Option<bool>,
    /// Scene filter for the currency/payment allow-list (e.g. "SIMPLE_BOT").
    pub client_scene: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BscNativePreheatForScreenRequest {
    pub screen_id: String,
    pub params_json: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BscNativeQuotedPriceRequest {
    pub base_currency: String,
    pub quote_currency: String,
    pub side: String,
    pub amount: Option<String>,
    pub standard: i32,
    pub deposit_name: Option<String>,
    pub payment_method: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BscNativeRecurringCacheStatusRequest {
    pub node_id: String,
    pub params_json: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BscNativeRecurringEditedPaymentMethod {
    pub payment_account_id: Option<String>,
    pub deposit_platform_code: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BscNativeRecurringFrequencyRequest {
    pub plan_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BscNativeRecurringPlanDetailRequest {
    pub plan_id: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BscNativeRecurringPlanListRequest {
    pub base_currency: Option<String>,
    pub quote_currency: Option<String>,
    pub status: Option<String>,
    pub page_num: Option<i32>,
    pub page_size: Option<i32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BscNativeRecurringPlanOrderListRequest {
    pub plan_id: String,
    pub base_currency: Option<String>,
    pub quote_currency: Option<String>,
    pub page_size: Option<i32>,
    pub page_num: Option<i32>,
    pub status: Option<i32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BscNativeRecurringPlanUpdateRequest {
    pub plan_id: String,
    pub status: Option<String>,
    pub edited_amount: Option<String>,
    pub edited_frequency: Option<String>,
    pub edited_payment_method: Option<BscNativeRecurringEditedPaymentMethod>,
    pub execution_time: Option<i64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BscNativeSellCryptoListRequest {
    pub quote_currency: String,
    pub crypto_type: Option<String>,
    pub include_funding_balance: Option<bool>,
    pub fund_flow_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BscNativeTradeSuggestionRequest {
    pub trade_type: String,
    pub base_currency: String,
    pub quote_currency: String,
    pub base_amount: Option<String>,
    pub quote_amount: Option<String>,
    pub multiple_currency: bool,
    pub standard: Option<i32>,
    pub chain_index: Option<String>,
    pub token_address: Option<String>,
    pub fund_flow_type: Option<String>,
    pub include_funding_balance: Option<bool>,
    pub deposit_platform_code: Option<String>,
    /// Optional legacy trade-suggestion currency class. L1 Crypto sends `0`;
    /// L2 omits it to preserve the existing mixed Crypto/xStock resolution.
    pub currency_type: Option<i32>, // default = None
    /// Which asset class this load targets (OSBSC-1381, FR-05a per PRD rev 18).
    /// Native hands the page its context on the request record itself — the six
    /// refetch paths clone `state.request`, so the discriminator travels with
    /// every one of them, and the input interactor never reads `bsc_master`
    /// state. Read in `fetch` to route to the network-only Stock source or the
    /// Crypto direct fetch. Client-side only: `#[serde(skip)]` keeps it off the
    /// wire (the endpoint is chosen by URL, not a body field). `Option<_>` +
    /// `#[uniffi(default = None)]` keeps the generated Swift/Kotlin constructor
    /// additive-only (a bare enum default panics the vendored Swift bindgen —
    /// knowledge-base.md); `None` is treated as Crypto so existing callers are
    /// unchanged.
    pub asset_type: Option<BscAssetType>, // default = None
}

#[derive(Debug, Clone, PartialEq)]
pub struct BscNativeUiFlowRequest {
    pub request_currency: String,
    pub base_currency: Option<String>,
    pub trade_type: String,
}

/// One row of the order-type chooser.
#[derive(Debug, Clone, PartialEq)]
pub struct BscOrderTypeItem {
    pub order_type: BscOrderType,
    pub enabled: bool,
}

/// The channel selected on the live tab. A `None` at the FFI boundary means
/// nothing is selected yet, which is what keeps the chip hidden before the
/// first load.
#[derive(Debug, Clone, PartialEq)]
pub struct BscSelectedChannel {
    /// Pair used by the host when opening the trading-bot entry.
    pub base_currency: String,
    pub quote_currency: String,
    /// The channel's `supportedOrderTypes`, straight from the response.
    pub supported_order_types: Vec<String>,
}

/// The stock the master header is on, as native selected it.
/// 
/// Support is carried per side because Buy and Sell share the selection: a one-sided stock is
/// still selectable and only its unsupported side is blocked (PRD §6.1–6.2).
#[derive(Debug, Clone, PartialEq)]
pub struct BscStockAsset {
    pub currency: String,
    pub display_name: String,
    pub icon: String,
    pub night_icon: String,
    /// Corporate-action pause; the row keeps the existing Paused treatment.
    pub trade_paused: bool,
    pub buy_supported: bool,
    pub sell_supported: bool,
}

/// The header strip above the Buy/Sell segment: token, live price, 24h change and
/// the K-line entry beside them.
/// 
/// Prices cross raw — native formats and localizes them — but the trend crosses as
/// a decision so both platforms colour the change identically.
#[derive(Debug, Clone, PartialEq)]
pub struct BscStockHeader {
    pub currency: String,
    pub display_name: String,
    pub icon: String,
    pub night_icon: String,
    /// Latest ticker price, raw. Empty until the first tick arrives.
    pub price: String,
    /// The currency `price` is quoted in.
    pub price_currency: String,
    /// Raw 24h change ratio, as the ticker reports it. Empty until the first tick.
    pub change_ratio: String,
    pub trend: BscPriceTrend,
    /// Corporate-action pause on the selected stock.
    pub trade_paused: bool,
}

/// Outcome of the BSC UV login check, plus the raw current-user readings.
/// 
/// Deliberately carries NO sub-account verdict. Reporting one from this
/// snapshot alone would be wrong in 3 of 8 cells — the rule also needs the
/// entry, the target tab and the app mode, which are not user state. Use
/// [`sub_account_verdict`] for that.
#[derive(Debug, Clone, PartialEq)]
pub struct BscUvAuthVerdict {
    /// A user is logged in.
    pub is_logged_in: bool,
    /// The current user exists AND is a sub-account. Absent user => `false`,
    /// matching both platforms' `?? false` / `?: false`.
    pub is_sub_user: bool,
    /// Verdict for the ladder's `.login` check.
    pub login_check_passes: bool,
}

/// Outcome of the country-restriction check.
#[derive(Debug, Clone, PartialEq)]
pub struct BscUvCountryVerdict {
    /// `true` = restricted, i.e. the ladder check FAILS.
    pub is_restricted: bool,
    /// Which axis was consulted — diagnostic for the A/B observation, and the
    /// field a wiring bug would show up in first.
    pub axis: BscUvRestrictionAxis,
}

/// Inputs to the frozen check.
/// 
/// `needPayback` is absent on purpose — see the module docs.
#[derive(Debug, Clone, PartialEq)]
pub struct BscUvFrozenParams {
    /// Which call site's rule to apply.
    pub predicate: BscUvFrozenPredicate,
    pub entry: BscUvEntry,
    pub target_page: BscUvTargetPage,
    /// Side for `Bsc` / `BscPair`; `None` elsewhere, and `None` is a real state
    /// (iOS's `OKPaymentTradeType?` is optional at these call sites).
    pub trade_type: Option<BscUvTradeType>,
    /// `actionCode` from the deposit user-info payload. Blank and absent are
    /// equivalent — iOS tests `nonEmptyValue`, Android `isNotNullOrEmptyOrBlank`.
    pub action_code: Option<String>,
    /// `userFreezeList` from the same payload. Only read when
    /// `consult_freeze_list` is set.
    pub user_freeze_list: Vec<String>,
    /// iOS passes `true` (its legacy fallback is live); Android passes `false`.
    pub consult_freeze_list: bool,
}

/// Outcome of the frozen check.
#[derive(Debug, Clone, PartialEq)]
pub struct BscUvFrozenVerdict {
    /// `true` = frozen, i.e. the ladder check FAILS.
    pub is_frozen: bool,
    /// The `actionCode` that decided a block, when one did.
    pub blocking_action_code: Option<String>,
    /// The `userFreezeList` key that decided a block, when one did.
    pub matched_freeze_item: Option<String>,
    pub source: BscUvFrozenSource,
}

/// The three model-layer readings the rule consumes, as an already-fetched
/// payload the caller supplies.
/// 
/// This is the observation seam. The three fields are *accessor* output, not
/// rule output: `OKKYCTypeInfo`/`KycInfosBean` on the two platforms and
/// [`KycTypeInfo`] in OKCore already agree 1:1 on how to read them, and each
/// side's compliance model owns that reading. What differs — and what this
/// crate takes over — is the pass/fail rule applied on top, i.e. iOS's
/// `validateKycLevel` + `validateSubKyc` and Android's `checkKycLevel`.
/// 
/// Native can therefore evaluate the shared rule against the payload it has
/// *already* fetched, with no second read and no I/O of any kind. That matters
/// for the A/B control arm specifically: [`kyc_verdict_from_cache`] would kick
/// off a background refresh the native path does not, so it cannot be used for
/// shadow observation.
#[derive(Debug, Clone, PartialEq)]
pub struct BscUvKycSnapshot {
    /// `personal.level`, else `company.level` — iOS
    /// `info.personal?.level ?? info.company?.level`, Android
    /// `KycInfosBean.getLevel()`. `None` when neither block carries one, which
    /// scores the same as 0.
    pub level: Option<i32>,
    /// `getSubKyc()` — `personal.kycInfo[level == 1].info.subKyc`. `None` for
    /// company-only accounts and for personal accounts with no level-1 entry;
    /// the rule then degrades to the plain level test.
    pub sub_kyc_code: Option<String>,
    /// `getTagStr()`. Not a rule input — echoed back on the verdict so the
    /// caller can keep its own cache-eligibility gate.
    pub tag_str: Option<String>,
}

/// Outcome of the BSC UV KYC check.
/// 
/// Carries the inputs as well as the verdict so the native side can keep its
/// own orchestration (`tag_str`) and its own outcome mapping, and so the A/B
/// observation can attribute a divergence to a field rather than just to the
/// boolean.
#[derive(Debug, Clone, PartialEq)]
pub struct BscUvKycVerdict {
    /// Resolved KYC level: `personal.level`, else `company.level`, else 0.
    pub level: i32,
    /// `level >= target_level`.
    pub level_satisfied: bool,
    /// `personal.kycInfo[level == 1].info.subKyc`. `None` for company-only
    /// accounts, and for personal accounts with no level-1 entry.
    pub sub_kyc_code: Option<String>,
    /// The full rule — this is the verdict the ladder consumes.
    pub is_compliant: bool,
    /// `personal.tagStr`, else `company.tagStr`. iOS's cache-eligibility input
    /// (`unknown` = "0", `review` = "4" both mean "do not trust the cache").
    /// Returned so native can apply that gate without a second read.
    pub tag_str: Option<String>,
    /// A level-1 `kycInfo` entry existed but no `subKyc` could be read from it.
    /// 
    /// Diagnostic, not a verdict input. Android parses `kycInfo[].info` as a
    /// JSON *string* (`JsonParser().parse(it.info)`) while iOS and this crate
    /// read it as a nested *object* (`model.info.subKyc`). iOS and OKCore agree,
    /// so the object reading is used — this flag makes the disagreement visible
    /// during the A/B observation instead of silently degrading every bscL1
    /// verdict to the plain level test.
    pub sub_kyc_unreadable: bool,
}

/// Inputs the pair check branches on. All of them come from the destination
/// parameters the caller already holds — none is fetched.
#[derive(Debug, Clone, PartialEq)]
pub struct BscUvPairParams {
    /// The entry being verified. `Convert` takes the support-token branch;
    /// `BscL2` / `BscL2Dex` carry the deeplink short-circuit.
    pub entry: BscUvEntry,
    /// Destination tab. Not consulted by `pair_check` itself — carried so the
    /// caller can pass one params record to both pair doors.
    pub target_tab: BscUvTargetTab,
    /// Trade side. `None` and `Convert` both take the no-read pass, matching
    /// Android's empty task list for any non buy/sell type.
    pub trade_type: Option<BscUvTradeType>,
    pub base_currency: String,
    pub quote_currency: String,
    pub top_up_currency: String,
    /// Optional `fund_flow_type` header override for the use-check; defaults to
    /// `funding`, which is what both native callers send today.
    pub fund_flow_type: Option<String>,
    /// Convert from-list query params, used only on the support-token branch.
    /// 
    /// **Caller-supplied on purpose, and every caller must fill all four.** All
    /// three implementations hit the same endpoint
    /// (`/v2/asset/quick/exchange/convertExistCurrencies` — iOS
    /// `OKCHttpServer.m:192`, Android `BscUvApi.kt:108`, here
    /// `convert_repository.rs:23`) with the same response shape and the same
    /// exact-`ccy` match, so the RULE is shared. What is not shared is the
    /// values, and they change the question being asked:
    /// 
    /// | param | iOS | Android |
    /// |---|---|---|
    /// | `local_currency` | uppercase display ISO | `FuturesSettingsService…isoCode` |
    /// | `convert_to` | omitted at this call site | no such param exists |
    /// | `settle_source_type` | `1`, from `accountSettleSourceType()` | hardcoded `0` |
    /// | `filter_funding_balance` | `!isSegregatedAccount` | always `false` |
    /// 
    /// `filterFundingBalance = true` makes the backend return ONLY currencies
    /// the user holds a non-zero balance of (`OKCHttpServer.h:37-39`), and
    /// `SegregatedAccountProvider.swift:43-50` lets iOS reach that value in some
    /// app modes. So iOS is asking "do I hold a convertible balance of X?" where
    /// Android asks "is X convertible at all?" — strictly narrower, server-side.
    /// Defaulting either shape here would silently change who passes.
    /// 
    /// Wire nit worth knowing when comparing arms: `settle_source_type` is
    /// always sent (`"0"` when `None`, matching Android's explicit `0`), but
    /// `filter_funding_balance` is OMITTED when `None`, where Android sends
    /// `false` explicitly. Byte-identical only if the backend treats absent as
    /// false — unverified. Pass `Some(false)` to be sure.
    pub local_currency: String,
    pub convert_to: Option<String>,
    pub settle_source_type: Option<i32>,
    pub filter_funding_balance: Option<bool>,
}

/// Outcome of the pair check.
#[derive(Debug, Clone, PartialEq)]
pub struct BscUvPairVerdict {
    pub outcome: BscUvPairOutcome,
    /// `defaultCurrency[side]`, falling back to the supplied quote. Equal to
    /// the supplied quote on every short-circuit path, where no default
    /// currency is read. This is the one `PreloadResult` field the success path
    /// actually consumes, so it is returned rather than left native.
    pub resolved_quote_currency: String,
    /// No gate ran: the parameters alone decided the verdict. A
    /// default-currency read may still have happened to fill
    /// `resolved_quote_currency` — that read informs the destination page, it
    /// does not gate anything.
    pub short_circuited: bool,
    /// Raw `canBuy` / `canSell` as they appeared in the payload. `None` means
    /// the field was absent, NOT `false` — see the module docs.
    pub can_buy: Option<bool>,
    pub can_sell: Option<bool>,
    /// Display-only failure reason, read from either platform's spelling.
    pub can_not_buy_reason: Option<String>,
    pub can_not_sell_reason: Option<String>,
    /// Which currency the convert support-token check looked for. Set only on
    /// that branch — the quote under `pair_check`, the base under
    /// `optional_pair_check`. Diagnostic; the verdict is on `outcome`.
    pub support_token_currency: Option<String>,
}

/// The non-user inputs the sub-account rule needs.
#[derive(Debug, Clone, PartialEq)]
pub struct BscUvSubAccountParams {
    /// The entry being verified. Pass `BscL2Dex` only if the platform really
    /// has a distinct DEX landing page — Android does not, and must pass
    /// `BscL2` there. See the module docs for why that reproduces both
    /// platforms' current behaviour from one rule.
    pub entry: BscUvEntry,
    /// Destination tab. Android reads `preloadParameter.targetTab`; iOS maps
    /// `TargetPage.convert` to `Convert`. `None` when the destination carries
    /// no tab, which never admits.
    pub target_tab: Option<BscUvTargetTab>,
    /// The app is in Pro mode. Android `OKAppService.isPro()`, iOS
    /// `appService?.currentAppMode() == .pro`. Caller-supplied — OKCore has no
    /// app-mode concept.
    pub is_pro_mode: bool,
}

/// Outcome of the BSC UV sub-account check.
#[derive(Debug, Clone, PartialEq)]
pub struct BscUvSubAccountVerdict {
    /// The current user exists AND is a sub-account.
    pub is_sub_user: bool,
    /// The full rule — this is the verdict the ladder consumes.
    pub check_passes: bool,
    /// A sub-account was admitted BECAUSE of pro mode, i.e. the plain
    /// `!is_sub_user` reading would have denied.
    /// 
    /// iOS reaches this outcome today through `shouldDeferUVCheck`, which also
    /// records `failedPreVerifyType` so the page can nudge. Once the verdict
    /// admits directly, that deferral branch stops being reached — so iOS must
    /// read this flag if it still wants the nudge. Android has no equivalent
    /// side effect and can ignore it.
    pub admitted_by_pro_mode: bool,
}

/// Parameters for the input page's indicative B2C quote (the convert-channel
/// preview hop). The amounts come from the preview order, not from state, so
/// the caller supplies them.
#[derive(Debug, Clone, PartialEq)]
pub struct BuySellB2cQuoteParams {
    pub base_currency: String,
    pub base_amount: String,
    pub quote_currency: String,
    pub quote_amount: String,
    pub payment_method: String,
    pub sub_payment_method: Option<String>,
    /// 1 = base currency, 2 = quote currency.
    pub standard: i32,
    pub amount: String,
    pub deposit_name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BuySellBankAccount {
    pub id: i64,
    pub currency: String,
    pub account_type: String,
    pub account_number: String,
    pub bank_name: String,
    pub display_name: String,
    pub create_time: String,
}

/// Banking-hours window (native `BankingHours`).
#[derive(Debug, Clone, PartialEq)]
pub struct BuySellBankingHours {
    pub within_banking_hours: bool,
    pub banking_hours_start_time_utc: String,
    pub banking_hours_end_time_utc: String,
    pub next_available_banking_hours_start_time_utc: String,
    pub next_available_banking_hours_end_time_utc: String,
}

/// Suggested base cryptocurrency for the input page.
#[derive(Debug, Clone, PartialEq)]
pub struct BuySellBaseCurrency {
    /// Ticker symbol (e.g. "BTC").
    pub symbol: String,
    /// Currency code as used by the backend.
    pub currency: String,
    /// Display ticker override for xStocks.
    pub display_currency: String,
    /// Stable currency identifier.
    pub currency_id: String,
    /// Human-readable display name.
    pub display_name: String,
    /// Full currency name.
    pub currency_name: String,
    /// Light-mode icon URL.
    pub icon: String,
    /// Dark-mode icon URL.
    pub night_icon: String,
    /// USD conversion rate (raw value from backend).
    pub usd_rate: f64,
    /// Available balance as a raw string (format in ViewModel).
    pub available_balance: String,
    /// Trade-only assets cannot route users to deposit as remediation.
    pub trade_only: bool,
    /// Decimal precision scale.
    pub scale: i32,
    /// Whether this base currency is an on-chain (DEX) token.
    pub is_dex: bool,
    /// DEX token metadata when `is_dex`; `None` for CeFi currencies.
    pub dex_info: Option<BuySellDexInfo>,
    /// Asset classification (native `currencyType`): 0 crypto, 1 fiat,
    /// 2 xStock, 3 dex — the xStock disclaimer gate keys on 2.
    pub currency_type: i32,
    /// Corporate-action trade status: 0 paused, 1 tradable.
    pub trade_status: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BuySellCampaignEligibilityParams {
    pub base_currency: String,
    pub quote_currency: String,
    pub quote_amount: String,
}

/// Channel-scope limit breakdown (native `LimitInfo`). The pay-with sheet's ⓘ
/// sheet renders one row per limit, and each row is gated on its `unlimited_*`
/// flag being false — so both halves have to survive the projection.
#[derive(Debug, Clone, PartialEq)]
pub struct BuySellChannelLimitInfo {
    pub daily_limit: String,
    pub weekly_limit: String,
    pub monthly_limit: String,
    pub lifetime_limit: String,
    pub daily_limit_remaining: String,
    pub weekly_limit_remaining: String,
    pub monthly_limit_remaining: String,
    pub lifetime_limit_remaining: String,
    pub single_transaction_min: String,
    pub single_transaction_max: String,
    pub limited_by_trade_frequency: bool,
    pub frequency_check_window: String,
    pub trade_frequency_limit: String,
    pub daily_freq_limit: String,
    pub unlimited_daily_limit: bool,
    pub unlimited_weekly_limit: bool,
    pub unlimited_monthly_limit: bool,
    pub unlimited_daily_limit_remaining: bool,
    pub unlimited_weekly_limit_remaining: bool,
    pub unlimited_monthly_limit_remaining: bool,
    pub unlimited_lifetime_limit_remaining: bool,
    pub unlimited_single_transaction_max: bool,
    pub unlimited_single_transaction_min: bool,
    pub unlimited_daily_freq_limit: bool,
    /// Which limit currently binds (e.g. "PER_TX_MAX").
    pub constrained_by: String,
    /// Currency the limits are denominated in.
    pub currency: String,
    /// EDD limit-increase state; `None` when the backend omits the block.
    pub edd_limit_level: Option<BuySellEddLimitLevel>,
}

/// Per-channel limit strings (raw backend values). Parsed into the validator's
/// `LimitValue` / `RangeState` by the validation mapper, which applies the
/// "empty = unbounded, never 0.0" rule. Empty string = no bound.
#[derive(Debug, Clone, PartialEq)]
pub struct BuySellChannelLimits {
    /// Channel-level min (from `trade_limit.minChannelLimit`).
    pub min_channel_limit: String,
    /// Channel-level max (from `trade_limit.maxChannelLimit`; dynamic remaining).
    pub max_channel_limit: String,
    /// Per-user min trade limit (from `trade_limit.minTradeLimit`).
    pub min_trade_limit: String,
    /// Per-user max trade limit (from `trade_limit.maxTradeLimit`).
    pub max_trade_limit: String,
    /// DEX daily cap (from `trade_limit.maxDexDailyLimit`).
    pub max_dex_daily_limit: String,
    /// DEX daily remaining (from `trade_limit.remainDexDailyLimit`).
    pub remain_dex_daily_limit: String,
    /// Per-payment-method bounds. The selected method overrides the channel
    /// trade range when its `payment_method` matches.
    pub payment_method_limits: Vec<BuySellPaymentMethodLimit>,
}

/// Server-localized channel descriptions and limits (native Localization).
/// Carried for the preview handoff; the input page renders none of these.
#[derive(Debug, Clone, PartialEq)]
pub struct BuySellChannelLocalization {
    pub processing_time_for_display: Option<String>,
    pub fee_rate_for_display: Option<String>,
    pub limit_for_display: Option<String>,
    pub range_for_display: Option<String>,
    pub general_description_for_display: Option<String>,
    pub daily_limit_for_display: Option<String>,
    pub daily_limit_remaining_for_display: Option<String>,
    pub frequency_limit_for_display: Option<String>,
    pub single_transaction_limit_for_display: Option<String>,
    pub account_protection_for_display: String,
    pub info_box_for_display: String,
    pub price_info_box_for_display: String,
    pub available_to_trade_for_display: String,
    pub available_to_withdraw_for_display: String,
    pub crypto_withdraw_hold_days_info_box_for_display: String,
    pub fiat_withdraw_hold_days_info_box_for_display: String,
    pub disclaimer_for_display: Option<String>,
    pub disclaimer_header_for_display: Option<String>,
    pub disclaimer_body_for_display: Option<String>,
    pub google_wallet_bottom_sheet_title: Option<String>,
    pub google_wallet_bottom_sheet_desc_first_par: Option<String>,
    pub google_wallet_bottom_sheet_desc_second_par: Option<String>,
    pub payback_processing_time_for_display: Option<String>,
    pub payback_disclaimer_for_display: Option<String>,
}

/// Convert/swap order detail information.
#[derive(Debug, Clone, PartialEq)]
pub struct BuySellConvertOrderDetail {
    pub base_amount: String,
    pub base_currency: String,
    pub base_currency_name: String,
    pub convert_base_amount: String,
    pub convert_base_currency: String,
    /// Brand display ticker for the convert target (e.g. "xAAPL"); empty when
    /// not an xStock / BE omits it. Consumers fall back to `convert_base_currency`.
    pub convert_base_display_currency: String,
    pub convert_base_currency_name: String,
    pub convert_base_currency_url: String,
    pub order_status: i32,
    pub exchange_rate: String,
    pub pre_exchange_rate: String,
    pub fail_reason: String,
}

/// Advisory payment cooldown copied from the trade suggestion. Native payment
/// selection owns presentation, precedence, and the Learn-more sheet.
#[derive(Debug, Clone, PartialEq)]
pub struct BuySellCoolDownInfo {
    pub remaining_time: Option<String>,
    pub title: Option<String>,
    pub message: Option<String>,
    pub main_title: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BuySellCryptoAsset {
    pub currency: String,
    pub currency_type: i32,
    pub trade_supported_currencies: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BuySellCryptoAssetListState {
    pub status: BuySellCryptoAssetListStatus,
    pub assets: Vec<BuySellCryptoAsset>,
    pub error_message: String,
}

/// Parameters identifying the payment method to delete. Which of the optional
/// identifiers the backend needs depends on the channel type, so all are sent.
#[derive(Debug, Clone, PartialEq)]
pub struct BuySellDeletePaymentMethodParams {
    pub deposit_platform_code: String,
    pub payment_method: String,
    pub payment_account_id: Option<String>,
    pub category_code: Option<ChannelCategoryCode>,
    pub user_bank_id: Option<String>,
    pub card_number: Option<String>,
}

/// On-chain (DEX) token metadata, present when the base currency is a DEX token.
/// Carries what the DEX quote request needs (token address, chain, decimals).
#[derive(Debug, Clone, PartialEq)]
pub struct BuySellDexInfo {
    /// Token contract address.
    pub token_address: String,
    /// Chain index (e.g. "1" = Ethereum). Kept as the wire string (native
    /// carries it as String end-to-end, converting only at the quote call).
    pub chain_index: String,
    /// Token decimals; `None` when the backend omits it.
    pub token_decimal: Option<i32>,
    /// Chain logo used by native preview/navigation surfaces.
    pub chain_logo: String,
}

/// DEX order detail information.
#[derive(Debug, Clone, PartialEq)]
pub struct BuySellDexOrderDetail {
    pub network: String,
    pub network_icon_url: String,
    pub refund_amount: String,
    pub refund_currency: String,
    pub dex_crypto_icon_day_url: String,
    pub dex_crypto_icon_night_url: String,
    pub fiat_order_status: Option<i32>,
    pub disclaimer_desc: String,
    pub disclaimer_learn_more_url: String,
    pub token_address: String,
    /// Service fee rate (raw decimal, e.g., "0.0085") - formatted in Kotlin
    pub service_fee_rate: String,
    /// Pre-formatted processor fee with crypto symbol (e.g., "12.45 USDC")
    pub formatted_processor_fee: String,
    /// Pre-formatted network fee with crypto symbol (e.g., "0.001 ETH")
    pub formatted_network_fee: String,
    /// Pre-formatted service fee with crypto symbol (e.g., "0.5 USDC")
    pub formatted_service_fee: String,
    /// Pre-formatted total fee with crypto symbol (e.g., "0.01 SOL")
    pub formatted_total_fee: String,
}

/// Earn-on-buy option attached to a channel (native EarnOption).
#[derive(Debug, Clone, PartialEq)]
pub struct BuySellEarnOption {
    pub currency_id: Option<String>,
    pub product_type_id: Option<String>,
    pub product_type_name: String,
    pub saving_type_id: Option<String>,
    pub saving_type_name: String,
    pub eligible_amount_max: f64,
    pub eligible_amount_min: f64,
    pub earn_max_unlimited: bool,
    pub toggle: bool,
    pub auto_earn_enabled: bool,
    pub earn_rate: String,
    pub localized_text: BuySellLocalizedText,
    pub earn_agreement_context: String,
    pub reward_type: Option<i32>,
}

/// EDD limit-increase state under `limitInfo` (native `EddLimitLevel`).
#[derive(Debug, Clone, PartialEq)]
pub struct BuySellEddLimitLevel {
    pub max_limit_level: i32,
    pub allow_increase_limit: bool,
    pub current_limit_level: i32,
    pub status: String,
}

/// FX rate conversion information.
#[derive(Debug, Clone, PartialEq)]
pub struct BuySellFxRateConversion {
    /// Pre-formatted exchange rate (e.g., "1 USD ≈ 1.23 EUR")
    pub formatted_fx_rate: String,
}

/// Content-state record carrying all possible data (flat pattern for small
/// package size).
#[derive(Debug, Clone, PartialEq)]
pub struct BuySellInputContentState {
    /// Current status.
    pub status: BuySellInputStatus,
    /// Loaded payload (valid when status == LoadedTradeSuggestion).
    pub data: Option<BuySellInputData>,
    /// Error message (valid when status == Error).
    pub error_message: String,
}

/// The loaded trade-suggestion payload for the input page.
#[derive(Debug, Clone, PartialEq)]
pub struct BuySellInputData {
    /// Suggested base currency.
    pub base_currency: BuySellBaseCurrency,
    /// Suggested quote currency.
    pub quote_currency: BuySellQuoteCurrency,
    /// Ranked payment channels.
    pub payment_channels: Vec<BuySellPaymentChannel>,
    /// Whether multi-currency quoting is enabled this session.
    pub multiple_currency_enabled: bool,
    /// Whether stablecoin is enabled.
    pub stable_coin_enabled: bool,
    /// Whether native (card) payment is available (absent → false).
    pub native_payment_available: bool,
    /// Whether the user must complete delta-KYC migration before buying
    /// (AR cohort; absent → false).
    pub need_migration: bool,
}

/// Input-page values that are not painted directly but must survive until a
/// native preview or payment-sheet handoff. Keeping them together prevents the
/// ViewModel from becoming a second source of truth beside the interactor.
#[derive(Debug, Clone, PartialEq)]
pub struct BuySellInputSelectionDetails {
    pub selected_sub_payment_method: Option<BuySellSelectedSubPaymentMethod>,
    pub dex_chain_logo: String,
}

/// Observable state for the BSC buy/sell input page.
#[derive(Debug, Clone, PartialEq)]
pub struct BuySellInputState {
    /// Content state (Idle / Loading / Loaded / Error) — holds the mapped payload.
    pub content: BuySellInputContentState,
    /// Last request used to load, retained so `refresh` / currency changes can re-fetch.
    /// Carries the `asset_type` discriminator (OSBSC-1381) that routes the load to
    /// the Crypto or Stock source, so every refetch that clones this request keeps
    /// the class.
    pub request: Option<BscNativeTradeSuggestionRequest>,
    /// Whether this interactor instance has accepted its initial load request.
    pub has_requested_initial_load: bool,
    /// Canonical typed trade type derived from [Self::request].
    pub trade_type: BuySellInputTradeType,
    /// Canonical input mode. Buy defaults to Fiat.
    pub input_mode: BuySellInputMode,
    /// Validation flow (one-time vs trigger order).
    pub input_flow: BuySellInputFlow,
    /// Trigger price (Trigger flow only); raw string.
    pub trigger_price: String,
    /// Live market (fair) price for the Trigger flow — fed by the native
    /// subscription via `set_trigger_market_price`, already converted to the
    /// page fiat. Empty until the feed delivers. Raw — native formats display.
    pub trigger_market_price: String,
    /// Whether the fair-price feed marked the token a stablecoin (drives the
    /// native display precision rule).
    pub trigger_market_price_is_stablecoin: bool,
    /// Raw `(trigger - market) / market` ratio (native
    /// emitTriggerPriceDiffPercentage); empty when either price is missing.
    pub trigger_price_diff_ratio: String,
    /// Trigger-price validation result (Trigger flow only); `None` otherwise.
    pub trigger_validation: Option<TriggerPriceValidation>,
    /// Resolved channel selection (engine output: key + source). The concrete
    /// channel is resolved on demand from `content.data`.
    pub selection: SelectionState,
    /// The user's explicit channel key, persisted so it survives reloads.
    pub user_selected_key: Option<String>,
    /// Raw active-unit input amount (per `input_mode`).
    pub input_amount: String,
    /// Derived counterpart amount for the "≈" display: crypto in Fiat
    /// mode, fiat in Crypto mode. Raw — native applies scale/rounding.
    pub converted_amount: String,
    /// Whether [Self::converted_amount] came from a Sell percentage fill and so owns
    /// the crypto side instead of being derived from [Self::input_amount].
    /// 
    /// The displayed fiat is rounded, so dividing it back sells less than the balance.
    /// Android calls this `isChipEvent`; iOS gets there by write ordering. Cleared by
    /// anything that makes the derived value right again — typing, toggling, a reload.
    pub is_chip_fill: bool,
    /// Fraction digits the amount box renders, reported by native.
    /// 
    /// Neither payload scale is this number — both carry the token's precision (8 for
    /// USDT) where the box shows 2, because the stablecoin rule lives in native's
    /// `AmountScaleRules`. Filling past it wrote 0.54658588 into a box holding 0.54,
    /// and the host committed its own rounding back as if the user had typed it.
    pub input_display_scale: Option<i32>,
    /// Native-provided validation context (verification, large-quote, flags).
    pub validation_config: ValidationConfig,
    /// Latest validation outcome; `None` until a validation has run.
    pub validation_result: Option<InputValidationResult>,
    /// Formatted error-message placeholders for the latest validation result
    /// (localized to strings by native). `None` until a validation has run.
    pub error_message_item: Option<ErrorMessageItem>,
    /// Indicative quote state (event-driven; no TTL).
    pub quote: BuySellQuoteState,
    /// Quote-race xStock CA pause from getQuote business code 20522. A real
    /// successful quote clears this source; selection tradeStatus remains
    /// independent in the loaded currency model.
    pub xstock_quote_trade_paused: bool,
    /// Whether the user still has an unmet xStock trading requirement. Native
    /// answers this — the record lives in another module, so the interactor
    /// cannot ask — and the render plan turns it into the "Unlock stocks" CTA.
    pub xstock_requirement_pending: bool,
    /// Whether an unlock launch is in flight. Set from the tap and cleared when
    /// the confirming query settles, so the CTA reads as disabled for the
    /// duration instead of inviting a second tap.
    pub xstock_launch_in_flight: bool,
    /// Which sides the selected stock trades, from the currency list. Both true
    /// for crypto and for a two-sided stock; the render plan pairs these with
    /// [`Self::trade_type`] to decide whether this page's side is offered.
    pub stock_buy_supported: bool,
    pub stock_sell_supported: bool,
    /// Optional CeFi crypto-asset list used by the input page's asset tabs.
    pub crypto_asset_list: BuySellCryptoAssetListState,
    /// Convert preview quote; distinct from the input's indicative quote.
    pub preview_quote: BuySellPreviewQuoteState,
    /// DEX trade-status code from the status-check API ("0" = tradable).
    /// Local codes: "-9999" = passkey-incompatible device, "unknown" =
    /// status-check failure (native DexTradeStatus PASSKEY_INCOMPATIBLE /
    /// UNKNOWN). Reset to "0" on a CeFi pick.
    pub dex_trade_status_code: String,
    /// BE-localized message for a failing status ("" for local codes — native
    /// localizes those; "errorMessage is used for displaying error directly").
    pub dex_trade_status_message: String,
    /// Trigger mode: whether the trigger-price box owns the keypad. Trigger
    /// mode starts trigger-price-focused (native setupTriggerInputFocus); it
    /// drives the CTA-slot dispatch (percentage chips vs preview button).
    /// Fed by `set_trigger_input_focused`.
    pub trigger_input_focused: bool,
    /// The user's method-level pick inside the selected channel (the wire
    /// `payment` id); `None` = channel default when no account was picked.
    pub selected_payment_key: Option<String>,
    /// The user's explicit account pick inside the selected payment method.
    /// Multiple account rows (for example Bank Transfer and QRIS) can share
    /// one method key, so the method index alone cannot restore the sheet row.
    pub selected_payment_account_id: Option<String>,
    /// Values needed by native handoff surfaces but not by the input renderer.
    /// The interactor owns them so selection never has a parallel ViewModel cache.
    pub selection_details: BuySellInputSelectionDetails,
    /// DERIVED: index of the resolved selected channel in
    /// `content.data.payment_channels` (-1 = none). Selection key match, else
    /// the FIRST channel (native getMatchedOrFirstChannel) — consumers must
    /// not re-derive this.
    pub selected_channel_index: i32,
    /// DERIVED: index of the resolved payment method inside the resolved
    /// channel (account ID match, then `selected_payment_key`, else 0; -1 when
    /// the channel is unresolved or has no methods) — consumers must not re-derive this.
    pub selected_method_index: i32,
    /// DERIVED: whether `validation_result` passes `bsc_input_is_valid`.
    pub input_is_valid: bool,
    /// DERIVED: fiat amounts display as the selected channel's
    /// PEGGED currency (stablecoin balance whose peg matches the valuation
    /// currency and appears in the cash balances) — consumers must not
    /// re-derive the rule.
    pub show_pegged_currency: bool,
    /// DERIVED (D4 render plan): which surface owns the error/banner area.
    pub error_state: InputErrorState,
    /// DERIVED (D4 render plan): what occupies the CTA slot.
    pub primary_action: PrimaryActionState,
}

/// Highlight styles for the server-localized copy (native
/// `LocalizationHighlightStyle`). Every field is optional on the wire.
#[derive(Debug, Clone, PartialEq)]
pub struct BuySellLocalizationHighlightStyle {
    pub processing_time_for_display: String,
    pub fee_rate_for_display: String,
    pub limit_for_display: String,
    pub single_transaction_limit_for_display: String,
}

/// Server-localized copy block (native LocalizedText).
#[derive(Debug, Clone, PartialEq)]
pub struct BuySellLocalizedText {
    pub show_learn_more: bool,
    pub title: String,
    pub content: String,
    pub preview_title: String,
    pub preview_body: String,
    /// Server-provided label for the read-only APY row (rewardType == 1).
    pub hold_to_earn_text: Option<String>,
}

/// Action button for order detail page.
#[derive(Debug, Clone, PartialEq)]
pub struct BuySellOrderDetailAction {
    /// Deep link to navigate to when the action button is clicked
    pub link: String,
    /// Display text for the action button
    pub display: String,
}

/// Content state (Record with variant field).
/// 
/// Using flat enum + Record pattern to minimize package size.
/// - `data` is only valid when `variant == Success`
/// - `error_message` is only valid when `variant == Error`
#[derive(Debug, Clone, PartialEq)]
pub struct BuySellOrderDetailContentState {
    /// The current state variant
    pub variant: BuySellOrderDetailContentStateVariant,
    /// Order data when variant is Success (None otherwise)
    pub data: Option<BuySellOrderDetailData>,
    /// Error message when variant is Error (empty otherwise)
    pub error_message: String,
}

/// Core order detail information displayed in UI.
#[derive(Debug, Clone, PartialEq)]
pub struct BuySellOrderDetailData {
    pub order_no: String,
    pub trade_type: BuySellTradeType,
    pub status: BuySellOrderStatus,
    pub order_type: BuySellOrderType,
    pub base_amount: String,
    pub base_currency_symbol: String,
    /// Brand display ticker for the base currency (e.g. "xAAPL"); empty when the
    /// backend omits it. Raw passthrough — the Lua UI applies the display-symbol
    /// AB flag and falls back to `base_currency_symbol`. (BSC xStocks OSBSC-858.)
    pub base_display_currency: String,
    pub base_currency_icon_url: String,
    pub quote_amount: String,
    pub quote_currency_symbol: String,
    pub quote_symbol: String,
    pub quote_currency_icon_url: String,
    pub price: String,
    pub channel_fee: String,
    pub create_date: i64,
    pub deposit_name: String,
    pub deposit_platform_code: String,
    pub category_code: i32,
    pub channel_icon_url: String,
    pub channel_night_icon_url: String,
    pub payment_method: String,
    pub payment_method_url: String,
    pub payment_method_night_url: String,
    pub last4: String,
    pub account_type: String,
    pub business_error_code: String,
    pub show_customer: bool,
    pub customer_url: String,
    pub is_convert: bool,
    pub frequency: String,
    pub dex_order_detail: Option<BuySellDexOrderDetail>,
    pub convert_order_detail: Option<BuySellConvertOrderDetail>,
    pub fx_rate_conversion: Option<BuySellFxRateConversion>,
    pub sub_orders: Vec<BuySellSubOrderDetail>,
    /// Action buttons from backend (deeplink + display text)
    pub actions: Vec<BuySellOrderDetailAction>,
    /// Pre-formatted base amount with crypto symbol (e.g., "1.5 BTC")
    pub formatted_base_amount: String,
    /// Pre-formatted quote amount with fiat symbol (e.g., "$1,234.56")
    pub formatted_quote_amount: String,
    /// Pre-formatted price with fiat symbol (e.g., "$50,000.00")
    /// Note: For DEX orders, Kotlin handles scientific notation formatting
    pub formatted_price: String,
    /// Pre-formatted channel fee with fiat symbol (e.g., "$2.50")
    pub formatted_channel_fee: String,
    /// Pre-formatted create date string (e.g., "Jan 1, 2024")
    /// Formatted using native localization
    pub formatted_create_date: String,
}

/// Dependencies for Order Detail Interactor.
#[derive(Debug, Clone, PartialEq)]
pub struct BuySellOrderDetailDependencies {
    /// Localization provider for locale-aware date formatting.
    pub localization: Arc<dyn BuySellOrderDetailLocalization>,
}

/// State for the Interactor.
#[derive(Debug, Clone, PartialEq)]
pub struct BuySellOrderDetailState {
    pub content: BuySellOrderDetailContentState,
    pub order_id: String,
    pub trade_type: Option<BuySellTradeType>,
}

/// A payment method's backing account (balance / linked bank / card).
#[derive(Debug, Clone, PartialEq)]
pub struct BuySellPaymentAccount {
    /// Stable account id (precheck / ACH relink).
    pub payment_account_id: String,
    /// Backend `id` — a DIFFERENT value from `payment_account_id`: the balance
    /// rail sends `id: "balance"` with an empty `paymentAccountId`.
    pub id: String,
    /// Display description.
    pub description: String,
    /// Account status (e.g. "active", "disconnected").
    pub status: String,
    /// Card brand, when the account is a card.
    pub brand: String,
    /// Masked account number, when present.
    pub account_number: String,
    /// Last 4 digits (card / bank accounts).
    pub last4: String,
    /// Available balance (raw string).
    pub available_balance: String,
    /// Balance account type (trading / funding), when present.
    pub balance_account_type: Option<String>,
    /// Account-level advisory payment cooldown.
    pub cool_down_info: Option<BuySellCoolDownInfo>,
    /// Whether credit card payments are allowed for this account; `None` when the
    /// backend omitted the field, which natives read as allowed (the response's
    /// own default). Apple Pay channels set this to `false` to surface the
    /// "Debit Card Only" subtitle on the payment-method selection sheet; Google
    /// Pay uses the same field. Kept optional so `default()` cannot imply
    /// "credit cards refused" for the synthetic accounts built in validation.
    pub allow_credit_card: Option<bool>, // default = None
    /// Masked card / bank number rendered next to the bank name.
    pub card_number: String,
    /// Issuing bank name.
    pub bank_name: String,
    /// Bank account type (e.g. "checking"), the ACH row's subtitle.
    pub bank_account_type: String,
    /// Account type (e.g. "Omnibus").
    pub account_type: String,
    pub recipient_name: String,
    pub routing_number: String,
    pub iban: String,
    pub bsb_number: String,
    pub bank_code: String,
    pub pix_key: String,
    pub is_account_label: bool,
    pub redirect_url: String,
    pub country: String,
    pub exp_month: i32,
    pub exp_year: i32,
    pub unavailable_balance: String,
    pub total_balance: String,
    pub user_id: String,
}

/// A payment channel recommendation. Carries the display subset plus the limit
/// fields the input validator consumes; extend forward-compatibly as needed.
#[derive(Debug, Clone, PartialEq)]
pub struct BuySellPaymentChannel {
    /// Deposit platform code (channel identity, e.g. "1000" balance, "1003" card).
    pub deposit_platform_code: String,
    /// Backend `depositPlatformId` — a DIFFERENT number from
    /// `deposit_platform_code` (e.g. Apple Pay is code "1031" / id 1021). `-1`
    /// when the backend omits it; consumers must not derive one from the other.
    pub deposit_platform_id: i32,
    /// Support currency for this channel (part of the stable channel key).
    pub support_currency: String,
    /// Display scale for the support currency (balance formatting); `None` →
    /// native falls back to the page fiat scale (mirrors `supportCurrencyScale`).
    pub support_currency_scale: Option<i32>,
    /// Backend deposit name.
    pub deposit_name: String,
    /// Display name shown to the user.
    pub display_name: String,
    /// Secondary display text.
    pub display_text: String,
    /// Backend `description` — the pay-with cell's subtitle. Distinct from
    /// `display_text`; named apart from `description` so the generated Swift
    /// struct does not shadow `CustomStringConvertible`.
    pub channel_description: String,
    /// Light-mode "display" channel icon URL (used when `change_display`).
    pub display_icon_url: String,
    /// Dark-mode "display" channel icon URL.
    pub display_icon_url_night: String,
    /// Light-mode channel icon URL (the balance/native icon; native picks this
    /// over `display_icon_url` unless `change_display`).
    pub icon_url: String,
    /// Dark-mode channel icon URL.
    pub icon_url_night: String,
    /// Whether native should prefer `display_icon_url` over `icon_url`.
    pub change_display: bool,
    /// Whether this channel requires the QP quote-and-confirm flow.
    pub is_convert: bool,
    /// Channel category (Balance / Native / ThirdParty).
    pub category_code: ChannelCategoryCode,
    /// Payment-method category (Cash / Stablecoin / Crypto).
    pub payment_method_category: PaymentMethodCategory,
    /// Indicative price for this channel (fiat per 1 crypto unit; raw string).
    pub price: String,
    /// Available amount for this channel (raw string).
    pub available_amount: String,
    /// Available balance from the channel's FIRST payment method's account —
    /// native's balance-display source (`paymentMethods[0].account.availableBalance`).
    /// The channel-level `available_amount` is 0/empty in the multi-currency payload.
    pub available_balance: String,
    /// Base amount for this channel (raw string).
    pub base_amount: String,
    /// Channel-level fiat min (from `minLimitAmount`; empty = unbounded).
    pub min_limit_amount: String,
    /// Channel-level fiat max (from `maxLimitAmount`; empty = unbounded).
    pub max_limit_amount: String,
    /// Trade/channel/DEX limits used by validation.
    pub limits: BuySellChannelLimits,
    /// Fixed-price marker from `supportCurrencyInfo` ("fixed" = pegged
    /// stablecoin channel); empty when the channel is not fixed-price. Carried
    /// verbatim: the Preview shell's pegged-fiat gate is
    /// `pricingType == "fixed"` AND `userPreferredCurrency == peggedCurrency`
    /// (iOS `SupportCurrencyInfo.hasPeggedFiatCurrency`, mirrored in
    /// bsc::preview::channel_plans::balance), so a host that cannot read this
    /// field renders a fixed-price channel in stablecoin style.
    pub pricing_type: String,
    /// Pegged fiat currency for this channel (from `supportCurrencyInfo`); empty
    /// when none. Drives the pegged-currency display.
    pub pegged_currency: String,
    /// Symbol to render for the pegged currency (e.g. "$").
    pub pegged_currency_symbol: String,
    /// Scale to use when the pegged currency is shown as fiat.
    pub pegged_currency_scale: i32,
    /// Whether the pegged currency is tradeable (DEX-sell fallback).
    pub pegged_currency_tradeable: bool,
    /// Order types this channel supports (e.g. "oneTimeBuy", "recurringBuy",
    /// "triggerOrder"); drives the order-type chip / trigger / recurring gates.
    pub supported_order_types: Vec<String>,
    /// Whether the backend marks this as the best/default channel.
    pub best_channel: bool,
    /// Fee rate (raw string).
    pub fee_rate: String,
    /// Quote limit from the trading-agent info, empty when absent.
    pub quote_limit: String,
    /// Per-channel max trigger-price deviation (e.g. "0.02"); empty → the
    /// default deviation applies (native validateReadiness).
    pub minimum_trigger_price_percent: String,
    /// Max pending trigger orders (limit-reached sheet body); empty when the
    /// backend omits the cap.
    pub max_pending_trigger_orders: String,
    /// Whether the user must file a tax ID before creating this channel's
    /// virtual account (native `ChannelBean.needTaxId` =
    /// `userState.hasTaxId == false`; LATAM PSPs — VelaFi and future rails).
    /// Feeds the Enable CTA's account-creation payload (`hasTaxId`).
    pub need_tax_id: bool,
    /// Whether this specific channel requires AR delta-KYC migration.
    pub need_migration: bool,
    /// Payment methods (display + preview/precheck handoff subset).
    pub payment_methods: Vec<BuySellPaymentMethodInfo>,
    /// Earn-on-buy option (preview toggle + earn badge); `None` when absent.
    pub earn_option: Option<BuySellEarnOption>,
    /// Server-localized channel copy (preview display); `None` when absent.
    pub localization: Option<BuySellChannelLocalization>,
    /// Terms & policies content (preview display); `None` when absent.
    pub term_content: Option<BuySellTermContent>,
    /// Google Pay supported countries.
    pub supported_countries: Vec<String>,
    /// Channel-level advisory payment cooldown.
    pub cool_down_info: Option<BuySellCoolDownInfo>,
    /// Whether this channel requires a quote before the order can be placed
    /// (native `needQuote`).
    pub need_quote: bool,
    /// Whether the channel's account must be created first (native
    /// `needCreateAccount`; the Zen Wallet unbound-default gate reads
    /// `== false`, so absent must stay absent).
    pub need_create_account: Option<bool>,
    /// Whether the channel offers stablecoin balances (native
    /// `supportStablecoin`); `None` when the backend omits the key.
    pub support_stablecoin: Option<bool>,
    /// `userState.hasTaxId` verbatim, for the legacy `ChannelUserState`.
    /// `need_tax_id` above is the derived gate; this is the raw tri-state.
    pub has_tax_id: Option<bool>,
    /// Channel limits shown by the pay-with sheet's ⓘ limit breakdown
    /// (native `limitInfo`); `None` when the backend omits the block.
    pub limit_info: Option<BuySellChannelLimitInfo>,
    /// Channel UI flow control (native `uiFlowControl`); `None` when absent.
    pub ui_flow_control: Option<BuySellUiFlowControl>,
    /// `informationBox.limit` — the sheet's information-box copy; empty when
    /// the backend omits the block.
    pub information_box_limit: String,
    /// Per-field highlight styles for the localized copy (native
    /// `localizationHighlightStyle`); `None` when absent.
    pub localization_highlight_style: Option<BuySellLocalizationHighlightStyle>,
    /// Whether the backend sent a `supportCurrencyInfo` block at all. An
    /// all-empty block is NOT the same as no block: the unpegged stablecoin
    /// rails send one with `peggedCurrency: ""` and `peggedCurrencyScale: 0`,
    /// and native reads the scale off it.
    pub has_support_currency_info: bool,
    /// Remaining `FiatChannelPaymentMethodListResponse` scalars. The legacy
    /// model declares every one of these, so the projection carries them all
    /// rather than growing a field at a time each release.
    pub allow_manual_addition: bool,
    pub deposit_to_withdraw: bool,
    pub exclusive: bool,
    pub pay_on_ok: bool,
    pub is_real_time_price: bool,
    pub use_real_time_price: bool,
    pub is_using_institution_verification: bool,
    pub kyc_type: Option<i32>,
    pub time_unit: Option<i32>,
    pub min_fee: String,
    pub max_purchase_time: String,
    pub min_purchase_time: String,
    pub max_wait_time_for_order_status_fetch: String,
    pub faq_url: String,
    pub h5_path: String,
    pub checkout_account_url: String,
    pub customer_url: String,
    pub search_url: String,
    pub tutorial_url: String,
    pub target: String,
    pub tags: Vec<String>,
    pub pre_check: Vec<String>,
    pub receipt_account: Vec<String>,
    pub supported_transaction_type: Vec<String>,
    pub withdraw_hold_crypto_amount: String,
    pub withdraw_hold_days: String,
    /// Banking-hours window for the rails that have one; `None` when absent.
    pub banking_hours: Option<BuySellBankingHours>,
    /// Whether this channel may be edited after creation. The backend's optional
    /// `recurringPlanEditable` is normalized fail-open by the trade-suggestion
    /// mapper: absent/null/true means editable; only explicit false locks it.
    /// Appended with a UniFFI default to preserve existing native constructors.
    pub recurring_plan_editable: bool, // default = true
}

/// One payment method under a channel — the subset the input page displays and
/// the order-preview / precheck handoff consumes.
#[derive(Debug, Clone, PartialEq)]
pub struct BuySellPaymentMethodInfo {
    /// Payment identifier (e.g. card scheme key).
    pub payment: String,
    /// Display description (e.g. "Visa •••• 4242").
    pub description: String,
    /// Payment method name.
    pub payment_method: String,
    /// Human-readable method name (e.g. "Balance", "ApplePay").
    pub display_name: String,
    /// Stable payment-method id (native/card channels).
    pub payment_method_id: String,
    /// Light-mode icon URL.
    pub icon_url: String,
    /// Dark-mode icon URL.
    pub icon_url_night: String,
    /// Backing account; `None` when the method has no linked account.
    pub account: Option<BuySellPaymentAccount>,
    /// All accounts returned for this payment method. Native uses these IDs to
    /// resolve and restore the selected method in the payment-method sheet.
    pub accounts: Vec<BuySellPaymentAccount>,
    /// Nested payment options shown by supported rails.
    pub sub_payment_methods: Vec<BuySellSubPaymentMethod>,
    /// Linked payout bank accounts used by Sell virtual-account readiness.
    pub bank_accounts: Vec<BuySellBankAccount>,
    /// Payment-method-level advisory payment cooldown.
    pub cool_down_info: Option<BuySellCoolDownInfo>,
    /// Currency this method settles in (native `paymentCurrency`).
    pub payment_currency: String,
    /// Method type discriminator (native `paymentMethodType`).
    pub payment_method_type: String,
    /// Whether the backend marks this method as the best option.
    pub best_option: bool,
    /// Whether the method can be reused for withdrawal (native
    /// `depositToWithdraw`).
    pub deposit_to_withdraw: bool,
    /// Recurring plans already running on this method — drives the sheet's
    /// recurring badge.
    pub num_of_recurring_plans: i32,
    /// Whether the sheet offers a transfer action for this method.
    pub show_transfer: bool,
    /// External-wallet deep link, empty when the method has none.
    pub deep_link: String,
    /// External-wallet store URL, empty when the method has none.
    pub store_url: String,
    /// Card styling colours the legacy cell renders with.
    pub background_color: String,
    pub main_color: String,
}

/// Trade bounds for one payment method inside a channel.
#[derive(Debug, Clone, PartialEq)]
pub struct BuySellPaymentMethodLimit {
    pub payment_method: String,
    pub min_trade_limit: String,
    pub max_trade_limit: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BuySellPreviewConvertDetail {
    pub convert_base_amount: String,
    pub convert_base_currency: String,
    pub convert_price: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BuySellPreviewQuote {
    pub base_amount: String,
    pub base_currency_symbol: String,
    pub quote_amount: String,
    pub quote_currency_symbol: String,
    pub is_convert: bool,
    pub convert_detail: Option<BuySellPreviewConvertDetail>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BuySellPreviewQuoteState {
    pub status: BuySellPreviewQuoteStatus,
    pub quote: Option<BuySellPreviewQuote>,
    pub error_message: String,
}

/// Indicative per-channel quote from the input page (`QuotedPriceUseCase`).
/// Selected via `best_option`, else the first entry. Drives the "≈"
/// display and the two-amount conversion. Event-driven only — no TTL/countdown
/// (native `refreshTTLSec` is dead).
#[derive(Debug, Clone, PartialEq)]
pub struct BuySellQuote {
    /// Indicative price (fiat per 1 crypto unit; raw string).
    pub price: String,
    /// The channel (deposit name) this quote is for.
    pub deposit_name: String,
    /// Payment method.
    pub payment: String,
    /// Whether the backend flagged this as the best option.
    pub best_option: bool,
    /// DEX quote id (native referenceDexQuoteId — the preview handoff
    /// re-references it); empty for CeFi quotes.
    pub dex_quote_id: String,
}

/// Suggested quote currency (fiat or stable) for the input page.
#[derive(Debug, Clone, PartialEq)]
pub struct BuySellQuoteCurrency {
    /// Ticker symbol (e.g. "USD").
    pub symbol: String,
    /// Currency code as used by the backend.
    pub currency: String,
    /// Stable currency identifier.
    pub currency_id: String,
    /// Human-readable display name.
    pub display_name: String,
    /// Full currency name.
    pub currency_name: String,
    /// Light-mode icon URL.
    pub icon: String,
    /// Dark-mode icon URL.
    pub night_icon: String,
    /// USD conversion rate (raw value from backend).
    pub usd_rate: f64,
    /// Decimal precision scale.
    pub scale: i32,
    /// Quick-amount presets (e.g. [50, 100, 200]).
    pub quick_amount_list: Vec<i32>,
    /// Trigger-price percentage presets (e.g. ["0.05", "0.1", "0.15"]); the
    /// mapper injects the native default when the response omits them.
    pub quick_percentage_list: Vec<String>,
    /// Whether multiple quote currencies are available.
    pub multiple_quote_currency: bool,
}

/// Observable quote state for the input page.
#[derive(Debug, Clone, PartialEq)]
pub struct BuySellQuoteState {
    pub status: BuySellQuoteStatus,
    pub quote: Option<BuySellQuote>,
    pub error_message: String,
}

/// Native payment-sheet metadata retained with the selected payment option.
/// The identifiers participate in handoff payloads; display fields let native
/// reopen the sheet without keeping a second mutable selection cache.
#[derive(Debug, Clone, PartialEq)]
pub struct BuySellSelectedSubPaymentMethod {
    pub id: String,
    pub description: String,
    pub icon_url: String,
    pub icon_url_night: String,
}

/// Sub-order detail information.
#[derive(Debug, Clone, PartialEq)]
pub struct BuySellSubOrderDetail {
    pub status: i32,
    pub trade_type: String,
    pub display_title: String,
    pub clickable: bool,
    pub order_no: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BuySellSubPaymentMethod {
    pub sub_payment_method: String,
    pub description: String,
    pub icon_url: String,
    pub icon_url_night: String,
}

/// Terms & policies content (native TermContentChannel).
#[derive(Debug, Clone, PartialEq)]
pub struct BuySellTermContent {
    pub mode: i32,
    pub content: Option<BuySellTermContentData>,
}

/// Terms content payload.
#[derive(Debug, Clone, PartialEq)]
pub struct BuySellTermContentData {
    pub policy_url: String,
    pub service_url: String,
    pub dex_learn_more_url: String,
    pub psp_url: String,
    pub tc_desc: Option<String>,
    pub tc_key: Option<String>,
}

/// Channel-scope UI flow control (native `FlowControl`).
#[derive(Debug, Clone, PartialEq)]
pub struct BuySellUiFlowControl {
    pub allow_manual_addition_info_page: bool,
    pub show_information_box: bool,
    pub show_toc: bool,
    pub show_warning_box: bool,
    pub skip_two_factor_auth: bool,
    pub skip_wire_education: bool,
    pub show_withdraw_hold_screen: bool,
    pub show_withdraw_accounts_for_deposit: bool,
    pub skip_management_single_channel_selection_screen: Option<bool>,
}

/// Request for fetching channel payment methods.
/// 
/// # Fields
/// - `fund_flow_type`: Optional fund flow type header (defaults to "funding" if None)
/// - `base_amount`: Optional base cryptocurrency amount
/// - `base_currency`: Optional base cryptocurrency code
/// - `quote_amount`: Optional quote currency amount
/// - `quote_currency`: Optional quote currency code
/// - `trade_type`: Trade type (required, defaults to Buy)
/// - `standard`: Optional standard indicator
/// - `category_code`: Optional channel category code
/// - `request_currency`: Optional request currency
/// - `multiple_currency`: Optional flag to enable multi-currency suggestions
/// - `chain_index`: Optional chain index for DEX
/// - `token_address`: Optional token address for DEX
#[derive(Debug, Clone, PartialEq)]
pub struct ChannelPaymentMethodsRequest {
    /// Fund flow type header (optional, defaults to "funding")
    pub fund_flow_type: Option<String>,
    /// Base amount (optional)
    pub base_amount: Option<String>,
    /// Base currency code (optional)
    pub base_currency: Option<String>,
    /// Quote amount (optional)
    pub quote_amount: Option<String>,
    /// Quote currency code (optional)
    pub quote_currency: Option<String>,
    /// Trade type (required)
    pub trade_type: TradeType,
    /// Standard (optional)
    pub standard: Option<i32>,
    /// Category code (optional)
    pub category_code: Option<i32>,
    /// Request currency (optional)
    pub request_currency: Option<String>,
    /// Multiple currency flag (optional)
    pub multiple_currency: Option<bool>,
    /// Chain index for DEX (optional)
    pub chain_index: Option<String>,
    /// Token address for DEX (optional)
    pub token_address: Option<String>,
    /// Include funding balance (optional). Ignored in the trading-bot scene:
    /// bots do not support auto transfer, so the dedicated endpoint always
    /// receives `includeFundingBalance=false`.
    pub include_funding_balance: Option<bool>,
    /// Caller scene (optional). `grid_bot` / `simple_bot` route the request to
    /// the dedicated simple-bot payment-method endpoint AND are sent verbatim
    /// as the `clientScene` query parameter so the server applies the per-bot
    /// allow-list (grid_bot allows crypto holdings; simple_bot/DCA does not).
    /// `simple_bot` is retained for backward compatibility (the server contract
    /// predates the grid/DCA split). Null/empty/unknown stay on the default
    /// channel endpoint and send no scene.
    pub client_scene: Option<String>,
}

/// Currency information for convert operations.
/// Contains raw data - formatting should be done in ViewModel layer.
#[derive(Debug, Clone, PartialEq)]
pub struct ConvertCurrency {
    /// Currency code (e.g., "BTC", "ETH", "USDT")
    pub ccy: String,
    /// Full currency name (e.g., "Bitcoin", "Ethereum")
    pub ccy_full_name: String,
    /// Currency icon URL
    pub icon_url: String,
    /// Available balance as string (raw value, format in ViewModel)
    pub available_balance: String,
    /// Local currency price as string (raw value, format in ViewModel)
    pub local_price: String,
    /// Is this a fiat currency?
    pub is_fiat: bool,
    /// Minimum conversion amount as string
    pub min_amount: String,
    /// Maximum conversion amount as string
    pub max_amount: String,
    /// Decimal precision scale
    pub scale: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConvertExecutionPageState {
    pub execution_state: ConvertExecutionState,
}

/// Execution state record for convert execution interactor.
#[derive(Debug, Clone, PartialEq)]
pub struct ConvertExecutionState {
    /// Current execution status
    pub status: ConvertExecutionStatus,
    /// Quote to be executed (set before execution)
    pub quote: Option<ConvertQuote>,
    /// Trade result (valid when status == Success)
    pub trade_result: Option<ConvertTradeResult>,
    /// Error message (valid when status == Failed)
    pub error_message: String,
}

/// Content state record containing all possible data.
/// Uses flat enum + record pattern (~80% smaller package size than sealed class).
#[derive(Debug, Clone, PartialEq)]
pub struct ConvertInputContentState {
    /// Current status
    pub status: ContentStatus,
    /// From currencies (valid when status == LoadedFromList)
    pub from_currencies: Vec<ConvertCurrency>,
    /// To currencies (valid when status == LoadedToList)
    pub to_currencies: Vec<ConvertCurrency>,
    /// Current quote (valid when status == LoadedQuote)
    pub quote: Option<ConvertQuote>,
    /// Error message (valid when status == Error)
    pub error_message: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConvertInputState {
    pub content: ConvertInputContentState,
    pub selected_from_currency: Option<ConvertCurrency>,
    pub selected_to_currency: Option<ConvertCurrency>,
    pub from_amount: String,
    pub to_amount: String,
    pub input_type: InputType,
    pub validation_state: InputValidationState,
    pub local_currency: String,
}

/// Exchange quote for currency conversion.
/// Contains raw data - formatting should be done in ViewModel layer.
#[derive(Debug, Clone, PartialEq)]
pub struct ConvertQuote {
    /// Quote ID for trade execution (binding quote)
    pub quote_id: String,
    /// Base currency (target - what you're receiving)
    pub base_ccy: String,
    /// Quote currency (source - what you're paying with)
    pub quote_ccy: String,
    /// Ask price as string (raw value, format in ViewModel)
    pub ask_price: String,
    /// Bid price as string (raw value, format in ViewModel)
    pub bid_price: String,
    /// Transaction side ("buy" or "sell")
    pub side: String,
    /// Requested amount as string
    pub amount: String,
    /// Amount currency (which currency the amount is in)
    pub amount_ccy: String,
}

/// Trade execution result.
/// Contains raw data - formatting should be done in ViewModel layer.
#[derive(Debug, Clone, PartialEq)]
pub struct ConvertTradeResult {
    /// Trade ID for tracking
    pub trade_id: String,
    /// Trade state (e.g., "success", "failed", "processing")
    pub state: String,
    /// Filled base amount as string (raw value, format in ViewModel)
    pub filled_base_amount: String,
    /// Filled quote amount as string (raw value, format in ViewModel)
    pub filled_quote_amount: String,
}

/// DEX 24-hour trading limits (remaining + max daily cap).
#[derive(Debug, Clone, PartialEq)]
pub struct DexTradeLimitInfo {
    /// How much of the daily DEX limit the user has left.
    pub remaining_daily_limit: LimitValue,
    /// The maximum daily DEX trading limit.
    pub max_daily_limit: LimitValue,
}

/// Formatting metadata needed to build localized error strings on the native side.
#[derive(Debug, Clone, PartialEq)]
pub struct ErrorMessageFormatInfo {
    /// Which unit the user is actively typing in (fiat or crypto).
    pub current_input_type: InputAmountType,
    /// The selected fiat (quote) currency SYMBOL for symbol-first formatting.
    pub selected_fiat_currency: String,
    /// The selected fiat (quote) currency CODE (e.g. "USDC") — crypto-channel
    /// limits render code-suffixed (native getMinLimit selectedCurrency.currency).
    pub selected_fiat_currency_code: String,
    /// The selected crypto (base) currency code, e.g. "BTC".
    pub selected_crypto_currency: String,
    /// The pegged currency for the channel (e.g. "USD" for a USDT channel). Empty if none.
    pub pegged_currency: String,
    /// Decimal places for fiat display (e.g. 2 for USD).
    pub fiat_precision: u32,
    /// Decimal places for crypto display (e.g. 8 for BTC).
    pub crypto_precision: u32,
    /// Whether the channel's payment category is crypto (payment_method_category == 2).
    pub is_crypto_channel: bool,
    /// The quoted price (fiat per 1 crypto unit) as a decimal string for precision-safe conversion.
    /// Computed via `decimal::Decimal` division to avoid f64 precision loss.
    pub quoted_price: String,
    /// The unit that trade limits (min/max) are denominated in.
    /// Buy: Fiat (channel limits are in fiat). Sell: Crypto (channel limits are in crypto).
    pub limit_denomination: InputAmountType,
}

/// Carries placeholder values required to format localized validation strings.
#[derive(Debug, Clone, PartialEq)]
pub struct ErrorMessageItem {
    /// Placeholder `currency`.
    pub currency: String,
    /// Placeholder `totalDailyLimit`.
    pub total_daily_limit: String,
    /// Placeholder `remainderDailyLimit`.
    pub remaining_daily_limit: String,
    /// Placeholder `minLimit`.
    pub min_limit: String,
    /// Placeholder `maxLimit`.
    pub max_limit: String,
    /// Placeholder `minOtcLimit`.
    pub min_otc_limit: String,
    /// Placeholder `maxOtcLimit`.
    pub max_otc_limit: String,
    /// Placeholder `availableBalance`.
    pub available_balance: String,
    /// Placeholder `paymentLabel`.
    pub payment_label: String,
    /// Placeholder `buyMaxFiatAmount`.
    pub buy_max_fiat_amount: String,
    /// Placeholder `buyMaxCryptoAmount`.
    pub buy_max_crypto_amount: String,
    /// Placeholder `peggedCurrency`.
    pub pegged_currency: String,
    /// Placeholder `selectedCrypto`.
    pub selected_crypto: String,
    /// Placeholder `selectedFiat`.
    pub selected_fiat: String,
    /// Whether the current payment method category is stablecoin.
    pub is_stablecoin: bool,
    /// Whether the user is in segregated-account flow.
    pub is_segregated_account: bool,
    /// Whether the selected base asset blocks deposit/withdraw remediation.
    pub is_trade_only: bool,
    /// Whether the current setup only allows balance payments.
    pub is_balance_only: bool,
    /// Whether alternative payment channels exist.
    pub has_alternative_channels: bool,
    /// Whether pegged-currency labels should be shown.
    pub should_show_pegged_currency: bool,
    /// Whether a deposit-stablecoin CTA should be shown.
    pub should_show_deposit_stablecoin_cta: bool,
    /// Token classification for differentiation (CeFi vs DeFi/Dex).
    pub token_type: BuySellTokenType,
    /// Whether the channel's payment category is crypto (payment_method_category == 2).
    /// Used by native side to determine fiat vs crypto formatting for limit amounts.
    pub is_crypto_channel: bool,
}

/// Numeric amounts that drive validation. Used by both buy and sell flows.
#[derive(Debug, Clone, PartialEq)]
pub struct InputAmounts {
    /// The amount in the user's active input unit.
    /// Fiat input → fiat value (e.g. 100 USD); crypto input → crypto value (e.g. 0.005 BTC).
    pub input_amount: f64,
    /// The fiat (quote-currency) equivalent of the input, used for limit comparisons.
    /// Always in fiat regardless of `current_input_type`.
    pub input_quote: f64,
    /// Available balance of the selected payment method (fiat value).
    pub balance: f64,
}

/// The error/banner surface decision (D4). Message text, CTA label and CTA
/// routing stay native (display policy / fragment); this record decides only
/// WHICH surface renders.
#[derive(Debug, Clone, PartialEq)]
pub struct InputErrorState {
    /// Owner of the error view / migration banner.
    pub error_type: InputErrorType,
    /// Mark the active amount box as error (native parity: set for amount
    /// errors even when a trigger error owns the error view; never under
    /// migration or a DEX-status error).
    pub mark_input_error: bool,
    /// Mark the trigger-price box as error (`TriggerExceedsLimit`).
    pub mark_trigger_input_error: bool,
    /// TEE / session-expire DEX statuses render the NEUTRAL banner alongside
    /// whatever `kind` decides — a separate surface, not an error-view variant
    /// (native buyHasDexStatusError).
    pub show_dex_neutral_banner: bool,
}

/// Input validation state with associated data.
#[derive(Debug, Clone, PartialEq)]
pub struct InputValidationState {
    /// Validation status
    pub status: ValidationStatus,
    /// Minimum limit
    pub min_limit: String,
    /// Maximum limit
    pub max_limit: String,
    /// Maximum balance
    pub max_balance: String,
    /// From currency code (for error messages)
    pub from_ccy: String,
}

/// Snapshot of large-quote configuration and limits.
#[derive(Debug, Clone, PartialEq)]
pub struct LargeQuoteInfo {
    /// Whether the feature flag / remote config enables large quotes.
    pub is_enabled: bool,
    /// Quote-amount threshold (fiat) that triggers the CTA.
    pub quote_limit: LimitValue,
    /// Absolute max allowed for the current channel/trade.
    pub channel_limit: LimitValue,
}

/// A numeric limit value that may or may not be active.
#[derive(Debug, Clone, PartialEq)]
pub struct LimitValue {
    /// The limit amount (fiat currency).
    pub value: f64,
    /// Whether this limit is active. When `false`, `value` should be ignored.
    pub enabled: bool,
}

/// Native-channel-specific limits (from `channel.tradeLimit`).
/// Only present when `channel_type == Native`.
#[derive(Debug, Clone, PartialEq)]
pub struct NativeChannelLimits {
    /// Minimum per-trade limit for the native channel.
    pub min_trade_limit: LimitValue,
    /// Maximum per-trade limit for the native channel.
    pub max_trade_limit: LimitValue,
    /// Minimum channel-level limit (broader than trade limit).
    pub min_channel_limit: LimitValue,
    /// Maximum channel-level limit (broader than trade limit).
    pub max_channel_limit: LimitValue,
}

/// The CTA-slot decision (D4).
#[derive(Debug, Clone, PartialEq)]
pub struct PrimaryActionState {
    /// What occupies the primary action area.
    pub action_type: PrimaryActionType,
    /// Preview-button variant (meaningful when `kind == PreviewButton`, but
    /// always computed so a consumer may read it unconditionally).
    pub preview_cta_type: PreviewCtaType,
    /// Whether the preview button is tappable. Login / Verify /
    /// DepositStablecoin / ActivateWallet are always tappable; Preview needs a
    /// channel + a valid amount, and quote loading only gates DEX tokens
    /// (native onDexQuoteLoading — the CeFi indicative quote must NOT disable
    /// the button, else every keystroke flashes disabled).
    pub preview_enabled: bool,
}

/// Content state for pro buy/sell order history list.
/// 
/// Using flat enum + Record pattern to minimize package size (best practice).
#[derive(Debug, Clone, PartialEq)]
pub struct ProBuySellOrderHistoryListContentState {
    pub variant: ProBuySellOrderHistoryListContentStateVariant,
    /// Items when variant is Loaded.
    pub items: Vec<ProBuySellOrderHistoryListItem>,
    /// Error message when variant is Error.
    pub error_message: String,
}

/// A single order history list item.
/// 
/// Field set is aligned with what the existing Kotlin `FiatCryptoOrderViewModel`
/// maps from `FiatOrderItemBean` to `FiatThirdPartyOrder`.
#[derive(Debug, Clone, PartialEq)]
pub struct ProBuySellOrderHistoryListItem {
    pub base_amount: String,
    pub base_currency_symbol: String,
    /// Brand display ticker for the base currency (e.g. "xAAPL"); empty when the
    /// backend omits it. Raw passthrough — the Lua UI applies the display-symbol
    /// AB flag and falls back to `base_currency_symbol`. (BSC xStocks OSBSC-858.)
    pub base_display_currency: String,
    pub create_date: i64,
    pub expire_seconds: i32,
    pub order_no: String,
    pub price: String,
    pub quote_amount: String,
    pub quote_currency_symbol: String,
    pub status: i32,
    /// API field "type" (e.g. "buy"/"sell") for native side mapping if needed.
    pub order_type: String,
    pub deposit_name: String,
    pub icon: String,
    pub icon_night: String,
    pub url: String,
    pub chain_url: String,
    pub is_convert: bool,
    pub pay_card_number: String,
    pub payment_method: String,
    pub convert_order_detail: Option<ProBuySellOrderHistoryListItemConvertDetail>,
    /// Display symbol for volume (already resolved for convert success/failure).
    pub display_volume_symbol: String,
    /// Display amount for volume (already resolved for convert success/failure).
    pub display_volume_amount: String,
    pub payment_method_url: String,
    pub payment_method_night_url: String,
    pub deposit_platform_code: String,
}

/// Convert order detail for a fiat order item (minimal fields used by Kotlin today).
#[derive(Debug, Clone, PartialEq)]
pub struct ProBuySellOrderHistoryListItemConvertDetail {
    pub convert_base_currency: String,
    /// Brand display ticker for the convert target (e.g. "xAAPL"); empty when
    /// not an xStock / BE omits it. Consumers fall back to `convert_base_currency`.
    pub convert_base_display_currency: String,
    pub order_status: i32,
    pub convert_base_amount: String,
}

/// State for the Pro Buy/Sell Order History List interactor.
#[derive(Debug, Clone, PartialEq)]
pub struct ProBuySellOrderHistoryListState {
    pub content: ProBuySellOrderHistoryListContentState,
    pub trade_type: Option<String>,
    pub order_status: Option<String>,
    pub past_days: Option<i32>,
    pub page_size: i32,
    pub next_page_index: i32,
    pub can_load_more: bool,
    pub is_loading_next_page: bool,
}

/// Parameters for a single order in quick convert.
#[derive(Debug, Clone, PartialEq)]
pub struct QuickConvertOrderItem {
    pub base_currency: String,
    pub base_amount: String,
    pub quote_currency: String,
    pub quote_amount: String,
}

/// Whether a trade range check is active, and if so, its min/max bounds.
#[derive(Debug, Clone, PartialEq)]
pub struct RangeState {
    /// Whether this range check should be evaluated at all.
    pub enabled: bool,
    /// The min/max bounds. Only meaningful when `enabled` is `true`.
    pub range: TradeLimitRange,
}

/// Actions section model
#[derive(Debug, Clone, PartialEq)]
pub struct RecurringBuyPlanDetailActions {
    pub is_action_block_enabled: bool,
    pub is_cancel_action_enabled: bool,
    pub is_pause_action_enabled: bool,
    pub is_resume_action_enabled: bool,
}

/// Banner model for error/warning display
#[derive(Debug, Clone, PartialEq)]
pub struct RecurringBuyPlanDetailBanner {
    pub is_visible: bool,
    pub error_type: String,
    pub title: String,
    pub message: String,
    pub show_deposit_button: bool,
    pub deposit_button_label: String,
    pub should_navigate_to_buy_crypto: bool,
    pub crypto_symbol_to_buy: String,
}

/// Content state for recurring buy plan detail
/// 
/// Using flat enum + Record pattern to minimize package size (best practice).
/// This avoids generating sealed classes in Kotlin, reducing package size from ~7 KB to ~1.69 KB.
/// 
/// Note: `item` is only valid when `variant` is `Loaded`. When `variant` is not `Loaded`,
/// `item` contains default/empty values and should be ignored.
#[derive(Debug, Clone, PartialEq)]
pub struct RecurringBuyPlanDetailContentState {
    pub variant: RecurringBuyPlanDetailContentStateVariant,
    /// Item data when variant is Loaded. Contains default values when variant is not Loaded.
    /// Check `variant` field to determine if item is valid.
    pub item: RecurringBuyPlanDetailItem,
    /// Error message when variant is Error
    pub error_message: String,
}

/// Frequency section model
#[derive(Debug, Clone, PartialEq)]
pub struct RecurringBuyPlanDetailFrequency {
    pub selected_option: RecurringBuyPlanDetailFrequencyOption,
    pub available_options: Vec<RecurringBuyPlanDetailFrequencyOption>,
}

/// Frequency option model with raw data
/// All formatting is handled in ViewModel layer
#[derive(Debug, Clone, PartialEq)]
pub struct RecurringBuyPlanDetailFrequencyOption {
    pub id: String,
    pub frequency: String,
    pub next_purchase_timestamp: i64,
    pub is_selected: bool,
}

/// Main detail item containing all sections
#[derive(Debug, Clone, PartialEq)]
pub struct RecurringBuyPlanDetailItem {
    pub status: String,
    pub banner: RecurringBuyPlanDetailBanner,
    pub summary: RecurringBuyPlanDetailSummary,
    pub pnl: RecurringBuyPlanDetailPnl,
    pub actions: RecurringBuyPlanDetailActions,
    pub orders: RecurringBuyPlanDetailOrders,
    pub frequency: RecurringBuyPlanDetailFrequency,
    pub payment_method: RecurringBuyPlanDetailPaymentMethod,
    /// Whether this recurring plan may be edited after creation (OSBSC-1690).
    /// Non-null exported capability: the mapper normalizes the optional backend
    /// wire key `recurringPlanEditable` fail-open (absent/null/true -> `true`
    /// editable, only explicit `false` -> locked). Card recurring plans, which
    /// must start immediately, are locked via an explicit backend `false`.
    /// Exported as `recurring_plan_editable` so UniFFI generates the
    /// `recurringPlanEditable` binding field that Android OKBuySell and the Lua
    /// navigation key consume. Appended at END and `#[uniffi(default = true)]` so
    /// native memberwise initializers stay source-compatible and default to the
    /// fail-open (editable) value.
    pub recurring_plan_editable: bool, // default = true
}

/// Orders section model
#[derive(Debug, Clone, PartialEq)]
pub struct RecurringBuyPlanDetailOrders {
    pub should_show_view_all_button: bool,
    pub placed_orders: Vec<RecurringBuyPlanOrderListItem>,
}

/// Payment method section model
#[derive(Debug, Clone, PartialEq)]
pub struct RecurringBuyPlanDetailPaymentMethod {
    pub name: String,
    pub icon_url: String,
    pub icon_night_url: String,
    /// Base currency code (e.g., "BTC")
    pub base_currency: String,
    /// Brand display ticker for the base currency (e.g. "xAAPL"). Empty when
    /// the backend omits it; renderers fall back to `base_currency`.
    pub base_display_currency: String,
    /// Quote currency code (e.g., "USD")
    pub quote_currency: String,
    /// Brand display ticker for the quote currency. Empty when the backend
    /// omits it; renderers fall back to `quote_currency`.
    pub quote_display_currency: String,
    /// Payment account ID for the current payment method
    pub payment_account_id: String,
    /// Payment method category: CASH (0), STABLECOIN (1), CRYPTO (2)
    pub payment_method_category: i32,
    /// Deposit platform code for the payment method
    pub deposit_platform_code: i32,
    /// Category code: THIRD_PARTY (2), BALANCE (3), NATIVE (4)
    pub category_code: i32,
}

/// PnL section model with raw amount data
/// All formatting is handled in ViewModel layer using quote_currency info
#[derive(Debug, Clone, PartialEq)]
pub struct RecurringBuyPlanDetailPnl {
    pub average_price: RecurringBuyPlanDetailPnlBlock,
    pub current_price: RecurringBuyPlanDetailPnlBlock,
    pub total_payment_amount: RecurringBuyPlanDetailPnlBlock,
    pub performance: RecurringBuyPlanDetailPnlBlock,
    /// Quote currency code (e.g., "BRL", "USD") for formatting amounts
    pub quote_currency: String,
    /// True if currency should be displayed after amount (crypto), false for before (fiat)
    pub display_quote_currency_after_amount: bool,
}

/// PnL block model
/// Contains raw amount data - UI layer formats with currency and decides icon, color, subtitle
#[derive(Debug, Clone, PartialEq)]
pub struct RecurringBuyPlanDetailPnlBlock {
    pub title: String,
    pub trend: Trend,
    pub has_value: bool,
    pub has_average_price: bool,
}

/// State for the Recurring Buy Plan Detail interactor.
#[derive(Debug, Clone, PartialEq)]
pub struct RecurringBuyPlanDetailState {
    pub content: RecurringBuyPlanDetailContentState,
    pub plan_id: String,
    pub is_from_result_page: bool,
    pub previous_status: Option<String>,
    pub is_updating_plan: bool,
    pub status_change_type: Option<RecurringBuyPlanDetailStatusChangeType>,
}

/// Summary section model with raw data
/// All formatting is handled in ViewModel layer
#[derive(Debug, Clone, PartialEq)]
pub struct RecurringBuyPlanDetailSummary {
    pub frequency: String,
    pub quote_amount: String,
    pub quote_currency: String,
    /// Brand display ticker for the quote currency. Empty when the backend
    /// omits it; renderers fall back to `quote_currency`.
    pub quote_display_currency: String,
    pub display_quote_currency_after_amount: bool,
    /// Base currency code (e.g., "USDC", "BTC") for display in summary amount
    pub base_currency: String,
    /// Brand display ticker for the base currency (e.g. "xAAPL"). Empty when
    /// the backend omits it; renderers fall back to `base_currency`.
    pub base_display_currency: String,
    pub edit_amount_min_limit: String,
    pub edit_amount_max_limit: String,
    pub edit_amount_current_amount: String,
    pub edit_amount_quote_currency: String,
    pub edit_amount_quote_symbol: String,
    pub edit_amount_display_quote_currency_after_amount: bool,
    pub show_paused_tag: bool,
    pub next_order_timestamp: i64,
}

/// Content state for recurring buy plan list
/// 
/// Using flat enum + Record pattern to minimize package size (best practice).
/// This avoids generating sealed classes in Kotlin, reducing package size from ~7 KB to ~1.69 KB.
#[derive(Debug, Clone, PartialEq)]
pub struct RecurringBuyPlanListContentState {
    pub variant: RecurringBuyPlanListContentStateVariant,
    /// Items when variant is Loaded
    pub items: Vec<RecurringBuyPlanListItem>,
    /// Error message when variant is Error
    pub error_message: String,
}

/// List item for recurring buy plan
#[derive(Debug, Clone, PartialEq)]
pub struct RecurringBuyPlanListItem {
    pub plan_id: String,
    /// Raw frequency value (should be formatted in ViewModel layer)
    pub frequency: String,
    pub base_currency: String,
    /// Brand display ticker for the base currency (e.g. "xAAPL"). Empty when
    /// the backend omits it; renderers fall back to `base_currency`.
    pub base_display_currency: String,
    pub base_currency_icon_url: String,
    /// Raw amount (should be formatted in ViewModel layer)
    pub quote_amount: String,
    /// Quote currency (USD, BTC, etc.)
    pub quote_currency: String,
    /// Brand display ticker for the quote currency. Empty when the backend
    /// omits it; renderers fall back to `quote_currency`.
    pub quote_display_currency: String,
    /// True if currency should be displayed after amount (crypto), false for before (fiat)
    pub display_quote_currency_after_amount: bool,
    /// Raw timestamp in milliseconds (should be formatted in ViewModel layer). 0 if no date available.
    pub next_order_timestamp: i64,
    pub show_action_required_tag: bool,
    pub show_paused_tag: bool,
}

/// State for the Recurring Buy Plan List interactor.
#[derive(Debug, Clone, PartialEq)]
pub struct RecurringBuyPlanListState {
    pub content: RecurringBuyPlanListContentState,
    pub base_currency: Option<String>,
    pub quote_currency: Option<String>,
    pub status: Option<String>,
    pub next_page: i32,
    pub can_load_more: bool,
    pub is_loading_next_page: bool,
}

/// Content state for recurring buy plan order list
/// 
/// Using flat enum + Record pattern to minimize package size (best practice).
/// This avoids generating sealed classes in Kotlin, reducing package size from ~7 KB to ~1.69 KB.
#[derive(Debug, Clone, PartialEq)]
pub struct RecurringBuyPlanOrderListContentState {
    pub variant: RecurringBuyPlanOrderListContentStateVariant,
    /// Items when variant is Loaded
    pub items: Vec<RecurringBuyPlanOrderListItem>,
    /// Error message when variant is Error
    pub error_message: String,
}

/// List item for recurring buy plan order with raw data
/// 
/// All formatting (amounts, dates) is handled in the ViewModel layer.
#[derive(Debug, Clone, PartialEq)]
pub struct RecurringBuyPlanOrderListItem {
    pub order_no: String,
    pub create_date: i64,
    pub status: RecurringBuyPlanOrderStatus,
    pub status_tag: RecurringBuyPlanOrderStatusTag,
    pub has_base_amount: bool,
    pub base_currency_icon_url: String,
    pub base_amount: String,
    pub base_currency_symbol: String,
    /// Brand display ticker for the base currency (e.g. "xAAPL"). Empty when
    /// the backend omits it; renderers fall back to `base_currency_symbol`.
    pub base_display_currency: String,
    pub quote_amount: String,
    pub quote_currency_symbol: String,
    /// Brand display ticker for the quote currency. Empty when the backend
    /// omits it; renderers fall back to `quote_currency_symbol`.
    pub quote_display_currency: String,
    pub display_quote_currency_after_amount: bool,
}

/// State for the Recurring Buy Plan Order List interactor.
#[derive(Debug, Clone, PartialEq)]
pub struct RecurringBuyPlanOrderListState {
    pub content: RecurringBuyPlanOrderListContentState,
    pub plan_id: String,
    pub next_page: i32,
    pub can_load_more: bool,
    pub is_loading_next_page: bool,
}

/// The resolved channel selection: a stable key (order-independent across
/// refreshes) plus the source that produced it.
#[derive(Debug, Clone, PartialEq)]
pub struct SelectionState {
    /// Stable channel key (see `selection::keys::channel_key`); `None` when nothing selected.
    pub selected_channel_key: Option<String>,
    /// Source tag for the selection.
    pub source: SelectionSource,
}

/// Parameters for submitting quick convert order.
#[derive(Debug, Clone, PartialEq)]
pub struct SubmitOrderQuickConvertRequest {
    pub orders: Vec<QuickConvertOrderItem>,
}

/// A single numeric bound (min or max) in a trade limit range.
#[derive(Debug, Clone, PartialEq)]
pub struct TradeLimitInfo {
    /// The limit value in fiat currency.
    pub value: f64,
    /// Whether this bound is inclusive (`<=` / `>=`) vs exclusive (`<` / `>`).
    pub inclusive: bool,
    /// Whether this bound is active. When `false`, the bound should be ignored.
    pub enabled: bool,
}

/// Trade limit range composed of optional min/max bounds.
#[derive(Debug, Clone, PartialEq)]
pub struct TradeLimitRange {
    pub min: TradeLimitInfo,
    pub max: TradeLimitInfo,
}

/// Two range checks applied during validation. Used by both buy and sell flows.
#[derive(Debug, Clone, PartialEq)]
pub struct TradeRanges {
    /// Per-user trade limit (from `channel.tradeLimit`).
    /// Tighter than `channel_limit`; accounts for KYC level, daily volume, etc.
    pub trade_limit: RangeState,
    /// Channel-level fiat range (from `channel.minLimitAmount` / `maxLimitAmount`).
    /// The broadest allowed range for the channel.
    pub channel_limit: RangeState,
}

/// Content state for trigger order detail screen.
/// 
/// Using flat Record + Enum pattern to minimise package size.
#[derive(Debug, Clone, PartialEq)]
pub struct TriggerOrderDetailContentState {
    pub variant: TriggerOrderDetailContentStateVariant,
    /// Populated when variant is Loaded.
    pub item: Option<TriggerOrderDetailItem>,
    /// Error message when variant is Error.
    pub error_message: String,
}

/// A trigger order detail with full order information.
#[derive(Debug, Clone, PartialEq)]
pub struct TriggerOrderDetailItem {
    /// Unique plan ID.
    pub order_id: String,
    /// Order category: "CEFI" or "CEDEFI".
    pub order_category: String,
    /// Current order status.
    pub status: TriggerOrderStatus,
    /// Source currency (e.g. "USD").
    pub from_currency: String,
    /// Target currency (e.g. "BTC").
    pub to_currency: String,
    /// Token symbol (e.g. "BTC").
    pub token_symbol: String,
    /// Trade direction: "BUY" or "SELL".
    pub direction: String,
    /// Order type: "LIMIT" or "TPSL".
    pub order_type: String,
    /// Trigger price as a string.
    pub trigger_price: String,
    /// Trigger condition: "ABOVE" or "BELOW".
    pub trigger_condition: String,
    /// Order amount.
    pub amount: String,
    /// Estimated execution amount (empty if unavailable).
    pub estimated_amount: String,
    /// Current market price (empty if unavailable).
    pub current_price: String,
    /// Actual execution price (empty if not yet filled).
    pub executed_price: String,
    /// Actual filled amount (empty if not yet filled).
    pub executed_amount: String,
    /// Downstream order ID (empty if not yet filled).
    pub executed_order_id: String,
    /// Failure reason (empty if not failed).
    pub fail_reason: String,
    /// Payment method display name (e.g. "balance").
    pub payment_method: String,
    /// Base currency icon URL.
    pub base_currency_url: String,
    /// Quote currency sign (e.g. "$"), empty for stablecoins.
    pub quote_symbol: String,
    /// Quote currency icon URL.
    pub quote_currency_url: String,
    /// Category code.
    pub category_code: i64,
    /// Quote currency symbol (e.g. "USD").
    pub quote_currency_symbol: String,
    /// Total fee.
    pub total_fee: String,
    /// Order expiry timestamp in epoch milliseconds (None if no expiry).
    pub expiry_time: Option<i64>,
    /// Creation timestamp string.
    pub created_time: String,
    /// Last updated timestamp string.
    pub updated_time: String,
}

/// State for the Interactor.
#[derive(Debug, Clone, PartialEq)]
pub struct TriggerOrderDetailState {
    pub content: TriggerOrderDetailContentState,
    pub cancel_result: TriggerOrderDetailCancelResult,
}

/// Content state for trigger order list.
/// 
/// Using flat enum + Record pattern to minimise package size.
#[derive(Debug, Clone, PartialEq)]
pub struct TriggerOrderListContentState {
    pub variant: TriggerOrderListContentStateVariant,
    /// Items when variant is Loaded.
    pub items: Vec<TriggerOrderListItem>,
    /// Error message when variant is Error.
    pub error_message: String,
}

/// A single trigger order item in the list.
#[derive(Debug, Clone, PartialEq)]
pub struct TriggerOrderListItem {
    /// Unique order identifier.
    pub order_id: String,
    /// Order category: "CEFI" or "CEDEFI".
    pub order_category: String,
    /// Base cryptocurrency code (e.g. "BTC").
    pub base_currency: String,
    /// URL for the base currency icon.
    pub base_currency_url: String,
    /// Fiat spend amount as a plain numeric string (e.g. "500.00").
    pub formatted_amount: String,
    /// Fiat currency code (e.g. "USD").
    pub from_currency: String,
    /// Brand display ticker for the from currency (e.g. "xAAPL"). Empty when
    /// the backend omits it; renderers fall back to `from_currency`.
    pub from_display_currency: String,
    /// Target cryptocurrency code (e.g. "BTC").
    pub to_currency: String,
    /// Brand display ticker for the to currency. Empty when the backend omits
    /// it; renderers fall back to `to_currency`.
    pub to_display_currency: String,
    /// Raw trade direction from API (e.g. "BUY", "SELL").
    pub direction: String,
    /// Raw trigger condition from API (e.g. "ABOVE", "BELOW").
    pub trigger_condition: String,
    /// Raw trigger price (e.g. "80000").
    pub trigger_price: String,
    /// Current market price (empty if unavailable).
    pub current_price: String,
    /// Estimated execution amount.
    pub estimated_amount: String,
    /// Actual filled crypto amount (empty if not yet filled).
    pub executed_amount: String,
    /// Order type: "LIMIT" or "TPSL".
    pub order_type: String,
    /// Current execution status.
    pub status: TriggerOrderStatus,
    /// Payment method display name (e.g. "balance").
    pub deposit_name: String,
    /// Quote currency sign (e.g. "$"), empty for stablecoins.
    pub quote_symbol: String,
    /// Quote currency icon URL.
    pub quote_currency_url: String,
    /// Category code.
    pub category_code: i64,
    /// Quote currency symbol (e.g. "USD").
    pub quote_currency_symbol: String,
    /// Total fee.
    pub total_fee: String,
    /// Number of active trigger order plans.
    pub trigger_plan_size: i64,
}

/// State for the Interactor.
#[derive(Debug, Clone, PartialEq)]
pub struct TriggerOrderListState {
    pub content: TriggerOrderListContentState,
    pub order_category: Option<String>,
    pub status: Option<String>,
}

/// Native-provided validation context that is NOT derivable from the trade
/// suggestion — strictly native/account facts. Everything data-derived (cash
/// balance currencies, deposit-stablecoin CTA, large-quote limits) is computed
/// by the validation mapper from the loaded suggestion instead.
/// Set at init / on auth change via `set_validation_config`, then merged into
/// `BuyInputValidationData` on every validation run.
#[derive(Debug, Clone, PartialEq)]
pub struct ValidationConfig {
    /// Login / KYC status; `None` when unknown (treated as verified downstream).
    pub verification_status: Option<VerificationStatus>,
    /// Whether the account is segregated (institutional).
    pub is_segregated_account: bool,
    /// Whether the user is a Pro user — combined with the loaded stablecoin
    /// flags to derive the deposit-stablecoin CTA (native
    /// `shouldShowDepositStableCoinCTA` = pro && offshore stablecoin flow).
    pub is_pro_user: bool,
    /// User's valuation (preferred) currency code — for the pegged rule.
    pub valuation_currency: String,
    /// Native AB decision for the xStock corporate-action trade-pause rule.
    pub is_xstock_trade_pause_enabled: bool,
}

/// UI-related boolean flags consumed by validation to decide which error/CTA to show.
#[derive(Debug, Clone, PartialEq)]
pub struct ValidationUiFlags {
    /// Whether the selected channel's payment category is stablecoin (e.g. USDT balance).
    pub is_stablecoin: bool,
    /// Whether the user's account is a segregated (institutional) account.
    pub is_segregated_account: bool,
    /// Whether the selected base asset blocks deposit/withdraw remediation.
    pub is_trade_only: bool,
    /// Whether more than one payment channel is available (enables "try another method" hints).
    pub has_alternative_channels: bool,
    /// Whether ALL available channels are balance-type (no third-party/native options).
    pub is_balance_only: bool,
    /// Whether the channel's pegged currency differs from the quote currency
    /// (e.g. USDT pegged to USD while quote is USDT → show "≈ USD" label).
    pub should_show_pegged_currency: bool,
    /// Whether to show a CTA prompting the user to deposit stablecoins.
    pub should_show_deposit_stablecoin_cta: bool,
    /// Whether the DEX quoted-price API failed, triggering `DexQuoteAmountUnavailable`.
    pub should_show_dex_quote_error: bool,
}

// -----------------------------------------------------------------------------
// 3. Interfaces / Interactors / Services (26)
// -----------------------------------------------------------------------------

/// Interactor struct.
pub struct BSCQuickConvertInteractor;
impl BSCQuickConvertInteractor {
    /// Registers a native observer for state changes.
    pub fn observe_state(&self, observer: Arc<dyn BSCQuickConvertStateObserver>) { todo!() }
    pub fn on_appear(&self) { todo!() }
    pub fn on_bottom_sheet_dismissed(&self, status: ConvertOrderStatus, action: ConvertBottomSheetAction) { todo!() }
    pub fn on_disappear(&self) { todo!() }
    pub fn show_init_content(&self, has_network: bool, is_simple: bool) { todo!() }
    pub fn submit_order(&self, has_network: bool, request: SubmitOrderQuickConvertRequest, is_simple: bool) { todo!() }
}

/// Native observer for state changes.
pub struct BSCQuickConvertStateObserver;
impl BSCQuickConvertStateObserver {
    pub fn on_state_changed(&self, state: BSCQuickConvertState) { todo!() }
}

pub struct BscBuySellUseCaseBridge;
impl BscBuySellUseCaseBridge {
    pub fn instance() -> Arc<BscBuySellUseCaseBridge> { todo!() }
    pub async fn get_b2c_quote(&self, cancel_token: Arc<OKCancellationToken>, request: BscNativeB2cQuoteRequest) -> Result<String, BuySellError> { todo!() }
    pub async fn get_buy_crypto_list(&self, cancel_token: Arc<OKCancellationToken>, request: BscNativeBuyCryptoListRequest) -> Result<String, BuySellError> { todo!() }
    pub async fn get_buy_crypto_list_with_cache(&self, cancel_token: Arc<OKCancellationToken>, request: BscNativeBuyCryptoListRequest) -> Result<String, BuySellError> { todo!() }
    pub async fn get_buy_crypto_list_with_cache_ttl_1h(&self, cancel_token: Arc<OKCancellationToken>, request: BscNativeBuyCryptoListRequest) -> Result<String, BuySellError> { todo!() }
    pub async fn get_buy_crypto_list_with_cache_ttl_5s(&self, cancel_token: Arc<OKCancellationToken>, request: BscNativeBuyCryptoListRequest) -> Result<String, BuySellError> { todo!() }
    pub async fn get_campaign(&self, cancel_token: Arc<OKCancellationToken>, request: BscNativeCampaignRequest) -> Result<String, BuySellError> { todo!() }
    pub async fn get_campaign_with_cache(&self, cancel_token: Arc<OKCancellationToken>, request: BscNativeCampaignRequest) -> Result<String, BuySellError> { todo!() }
    pub async fn get_currency_balance(&self, cancel_token: Arc<OKCancellationToken>, request: BscNativeCurrencyBalanceRequest) -> Result<String, BuySellError> { todo!() }
    pub async fn get_default_currency(&self, cancel_token: Arc<OKCancellationToken>, request: BscNativeDefaultCurrencyRequest) -> Result<String, BuySellError> { todo!() }
    pub async fn get_default_currency_with_cache(&self, cancel_token: Arc<OKCancellationToken>, request: BscNativeDefaultCurrencyRequest) -> Result<String, BuySellError> { todo!() }
    pub async fn get_fiat_order_list(&self, cancel_token: Arc<OKCancellationToken>, params_json: String) -> Result<String, BuySellError> { todo!() }
    pub async fn get_order_detail(&self, cancel_token: Arc<OKCancellationToken>, params_json: String) -> Result<String, BuySellError> { todo!() }
    pub async fn get_order_status(&self, cancel_token: Arc<OKCancellationToken>, params_json: String) -> Result<String, BuySellError> { todo!() }
    pub async fn get_quoted_price(&self, cancel_token: Arc<OKCancellationToken>, request: BscNativeQuotedPriceRequest) -> Result<String, BuySellError> { todo!() }
    pub async fn get_sell_crypto_list(&self, cancel_token: Arc<OKCancellationToken>, request: BscNativeSellCryptoListRequest) -> Result<String, BuySellError> { todo!() }
    pub async fn get_sell_crypto_list_with_cache(&self, cancel_token: Arc<OKCancellationToken>, request: BscNativeSellCryptoListRequest) -> Result<String, BuySellError> { todo!() }
    pub async fn get_tpsl_suggestion(&self, cancel_token: Arc<OKCancellationToken>, params_json: String) -> Result<String, BuySellError> { todo!() }
    pub async fn get_trade_suggestion(&self, cancel_token: Arc<OKCancellationToken>, request: BscNativeTradeSuggestionRequest) -> Result<String, BuySellError> { todo!() }
    pub async fn get_trade_suggestion_with_cache(&self, cancel_token: Arc<OKCancellationToken>, request: BscNativeTradeSuggestionRequest) -> Result<String, BuySellError> { todo!() }
    pub async fn get_trigger_order_list(&self, cancel_token: Arc<OKCancellationToken>, params_json: String) -> Result<String, BuySellError> { todo!() }
    pub async fn get_ui_flow(&self, cancel_token: Arc<OKCancellationToken>, request: BscNativeUiFlowRequest) -> Result<String, BuySellError> { todo!() }
    pub async fn get_user_info(&self, cancel_token: Arc<OKCancellationToken>) -> Result<String, BuySellError> { todo!() }
    /// Synchronous 1h cache peek — returns the cached `baseCurrencies` JSON when
    /// a within-1h entry exists, else `None`. No network, no async round-trip, so
    /// native can pre-populate the To Buy sheet before it presents on a cache hit
    /// (eliminating the cache-hit blank/settle gap). [OSBSC-833]
    pub fn peek_buy_crypto_list_with_cache_ttl_1h(&self, request: BscNativeBuyCryptoListRequest) -> Option<String> { todo!() }
    pub async fn submit_buy_sell_order(&self, cancel_token: Arc<OKCancellationToken>, params_json: String) -> Result<String, BuySellError> { todo!() }
    pub async fn submit_otc_agent_order(&self, cancel_token: Arc<OKCancellationToken>, params_json: String) -> Result<String, BuySellError> { todo!() }
}

pub struct BscConvertUseCaseBridge;
impl BscConvertUseCaseBridge {
    pub fn instance() -> Arc<BscConvertUseCaseBridge> { todo!() }
    pub async fn exchange_trade(&self, cancel_token: Arc<OKCancellationToken>, params_json: String) -> Result<String, BuySellError> { todo!() }
    pub async fn get_convert_currency_pairs(&self, cancel_token: Arc<OKCancellationToken>, request: BscNativeConvertCurrencyPairsRequest) -> Result<String, BuySellError> { todo!() }
    pub async fn get_convert_from_list(&self, cancel_token: Arc<OKCancellationToken>, request: BscNativeConvertFromListRequest) -> Result<String, BuySellError> { todo!() }
    pub async fn get_convert_from_list_no_cache(&self, cancel_token: Arc<OKCancellationToken>, request: BscNativeConvertFromListRequest) -> Result<String, BuySellError> { todo!() }
    pub async fn get_convert_to_list(&self, cancel_token: Arc<OKCancellationToken>, request: BscNativeConvertToListRequest) -> Result<String, BuySellError> { todo!() }
    pub async fn get_convert_to_list_no_cache(&self, cancel_token: Arc<OKCancellationToken>, request: BscNativeConvertToListRequest) -> Result<String, BuySellError> { todo!() }
    pub async fn get_currency_pair_suggestion(&self, cancel_token: Arc<OKCancellationToken>, request: BscNativeCurrencyPairSuggestionRequest) -> Result<String, BuySellError> { todo!() }
    pub async fn get_currency_pair_suggestion_no_cache(&self, cancel_token: Arc<OKCancellationToken>, request: BscNativeCurrencyPairSuggestionRequest) -> Result<String, BuySellError> { todo!() }
    pub async fn get_exchange_quote(&self, cancel_token: Arc<OKCancellationToken>, request: BscNativeExchangeQuoteRequest) -> Result<String, BuySellError> { todo!() }
    pub async fn get_indicative_quote(&self, cancel_token: Arc<OKCancellationToken>, request: BscNativeIndicativeQuoteRequest) -> Result<String, BuySellError> { todo!() }
    pub async fn get_quick_change_currencies(&self, cancel_token: Arc<OKCancellationToken>, params_json: String) -> Result<String, BuySellError> { todo!() }
    pub async fn get_quick_change_order_detail(&self, cancel_token: Arc<OKCancellationToken>, params_json: String) -> Result<String, BuySellError> { todo!() }
}

pub struct BscDexUseCaseBridge;
impl BscDexUseCaseBridge {
    pub fn instance() -> Arc<BscDexUseCaseBridge> { todo!() }
    pub async fn get_dex_details(&self, cancel_token: Arc<OKCancellationToken>, request: BscNativeDexDetailsRequest) -> Result<String, BuySellError> { todo!() }
    pub async fn get_dex_quote(&self, cancel_token: Arc<OKCancellationToken>, request: BscNativeDexQuoteRequest) -> Result<String, BuySellError> { todo!() }
    pub async fn get_dex_trade_status(&self, cancel_token: Arc<OKCancellationToken>, request: BscNativeDexTradeStatusRequest) -> Result<String, BuySellError> { todo!() }
    pub async fn get_token_base_info(&self, cancel_token: Arc<OKCancellationToken>, request: BscNativeDexTokenBaseInfoRequest) -> Result<String, BuySellError> { todo!() }
    pub async fn search_tokens(&self, cancel_token: Arc<OKCancellationToken>, request: BscNativeDexSearchTokensRequest) -> Result<String, BuySellError> { todo!() }
}

/// Observable state manager for the BSC order-type header.
pub struct BscMasterInteractor;
impl BscMasterInteractor {
    /// Everything the host knows, in one push: live tab, asset class, the live
    /// tab's channel and whether a DEX token is selected.
    /// 
    /// Preferred over the four granular setters below, which remain for callers
    /// that genuinely change one thing (and for the existing native call sites).
    /// The four values are read together by the plan, so pushing them together
    /// avoids publishing a frame where the tab has moved but the channel has not.
    pub fn apply_master_state(&self, context: BscMasterState) { todo!() }
    /// Mark the Add Account entry effect as handled.
    pub fn dismiss_add_account_toast(&self) { todo!() }
    /// Seed from the entry point. The requested type applies as soon as a
    /// channel can support it, not immediately.
    pub fn initialize(&self, entry: BscMasterEntry) { todo!() }
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
    /// The user picked a frequency; selecting one commits the recurring type.
    pub fn select_frequency(&self, code: String) { todo!() }
    /// The user picked an order type in the chooser. Recurring is ignored here —
    /// it commits on [`select_frequency`], matching the two-step native flow.
    pub fn select_order_type(&self, order_type: BscOrderType) { todo!() }
    /// The stock the header is on, selected by native — from the page's trade suggestion
    /// on first load, or from the stock picker afterwards. Everything downstream —
    /// channels, quote, validation, CTA — follows from the page reloading for that stock,
    /// so this only moves the header's own state.
    pub fn select_stock(&self, stock: Option<BscStockAsset>) { todo!() }
    /// Asset class switched. Stock drops Convert and, offshore, every order type
    /// beyond instant (PRD §4).
    pub fn set_asset_type(&self, asset_type: BscAssetType) { todo!() }
    /// A channel was selected on the live tab. What it supports is read off its
    /// own list here. Also the trigger for loading frequencies, which are only
    /// meaningful once recurring is supported.
    pub fn set_channel(&self, channel: Option<BscSelectedChannel>) { todo!() }
    /// Pay with (Buy) or Receive (Sell) on the live page, reported up because the
    /// K-line pair rule needs both halves (PRD §7).
    pub fn set_counterpart_currency(&self, currency: String) { todo!() }
    /// Enable or disable debug logging.
    /// 
    /// In release builds this is a no-op, but the symbol must exist
    /// so that UniFFI bindings generated in debug mode continue to
    /// link successfully against the release library.
    pub fn set_debug(&self, _enabled: bool) { todo!() }
    /// A DEX token was picked or cleared.
    pub fn set_dex_selected(&self, dex_selected: bool) { todo!() }
    /// How the domain is set up: trigger availability and whether the selector is
    /// on. Sets up what may be offered, not what is selected — that is
    /// [`select_order_type`].
    pub fn set_order_type(&self, parameters: BscMasterParameters) { todo!() }
    /// Whether the selected stock and the counterpart currency form an orderbook
    /// pair available in this region. Native looks this up; the plan decides what
    /// to do with it.
    pub fn set_orderbook_pair_available(&self, available: bool) { todo!() }
    /// Record whether the active OTC chat should survive host transitions.
    pub fn set_otc_agent_chat_active(&self, active: bool) { todo!() }
    /// Record the selected payment method for host-level analytics.
    pub fn set_payment_method_for_tracking(&self, payment_method: Option<String>) { todo!() }
    /// Seed the one-shot Add Account toast from the host entry.
    pub fn set_show_add_account_toast(&self, show: bool) { todo!() }
    /// Start date confirmed on the Lua picker; 0 clears it.
    pub fn set_start_date(&self, start_date_ms: i64) { todo!() }
    /// Set a state observer (UniFFI export).
    /// 
    /// DEPRECATED: callback-based observer. Use `state_stream()` —
    /// the callback observer pins the foreign object in UniFFI's
    /// static handleMap and produces a cross-language retain cycle
    /// (Android GC + iOS ARC).
    pub fn set_state_observer(&self, observer: Arc<dyn InteractorStateChangeObserver>) { todo!() }
    /// Tab switched. Re-runs eligibility, which may drop an unsupported type.
    pub fn set_tab(&self, tab: BscMasterTab) { todo!() }
    /// Latest ticker for the selected stock. Raw price and 24h ratio; the header's
    /// trend is derived from the ratio's sign.
    pub fn set_ticker(&self, price: String, change_ratio: String) { todo!() }
    /// Start the interactor lifecycle.
    /// 
    /// Calls the user's `InteractorLifecycle::start()` implementation.
    /// Override that method to add custom startup behavior (e.g., initial data loads).
    pub fn start(&self) { todo!() }
    /// Get the current state (UniFFI export).
    /// 
    /// Returns the External type (UniFFI-compatible).
    pub fn state(&self) -> BscMasterSnapshot { todo!() }
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

pub struct BscPaymentUseCaseBridge;
impl BscPaymentUseCaseBridge {
    pub fn instance() -> Arc<BscPaymentUseCaseBridge> { todo!() }
    pub async fn delete_account(&self, cancel_token: Arc<OKCancellationToken>, request: BscNativePaymentAccountDeleteRequest) -> Result<String, BuySellError> { todo!() }
    pub async fn get_account_query(&self, cancel_token: Arc<OKCancellationToken>, request: BscNativePaymentAccountQueryRequest) -> Result<String, BuySellError> { todo!() }
    pub async fn get_banners(&self, cancel_token: Arc<OKCancellationToken>, request: BscNativePaymentBannersRequest) -> Result<String, BuySellError> { todo!() }
    pub async fn get_banners_with_cache(&self, cancel_token: Arc<OKCancellationToken>, request: BscNativePaymentBannersRequest) -> Result<String, BuySellError> { todo!() }
    pub async fn get_channel_payment_methods(&self, cancel_token: Arc<OKCancellationToken>, request: BscNativePaymentMethodsRequest) -> Result<String, BuySellError> { todo!() }
    pub async fn get_channel_payment_methods_with_cache(&self, cancel_token: Arc<OKCancellationToken>, request: BscNativePaymentMethodsRequest) -> Result<String, BuySellError> { todo!() }
}

pub struct BscRecurringBuyUseCaseBridge;
impl BscRecurringBuyUseCaseBridge {
    pub fn instance() -> Arc<BscRecurringBuyUseCaseBridge> { todo!() }
    pub async fn cache_invalidate(&self, cancel_token: Arc<OKCancellationToken>, request: BscNativeCacheEventRequest) -> Result<String, BuySellError> { todo!() }
    pub async fn debug_recurring_cache_status(&self, cancel_token: Arc<OKCancellationToken>, request: BscNativeRecurringCacheStatusRequest) -> Result<String, BuySellError> { todo!() }
    pub async fn get_recurring_frequency(&self, cancel_token: Arc<OKCancellationToken>, request: BscNativeRecurringFrequencyRequest) -> Result<String, BuySellError> { todo!() }
    pub async fn get_recurring_plan_detail(&self, cancel_token: Arc<OKCancellationToken>, request: BscNativeRecurringPlanDetailRequest) -> Result<String, BuySellError> { todo!() }
    pub async fn get_recurring_plan_detail_with_cache(&self, cancel_token: Arc<OKCancellationToken>, request: BscNativeRecurringPlanDetailRequest) -> Result<String, BuySellError> { todo!() }
    pub async fn get_recurring_plan_list(&self, cancel_token: Arc<OKCancellationToken>, request: BscNativeRecurringPlanListRequest) -> Result<String, BuySellError> { todo!() }
    pub async fn get_recurring_plan_list_with_cache(&self, cancel_token: Arc<OKCancellationToken>, request: BscNativeRecurringPlanListRequest) -> Result<String, BuySellError> { todo!() }
    pub async fn get_recurring_plan_order_list(&self, cancel_token: Arc<OKCancellationToken>, request: BscNativeRecurringPlanOrderListRequest) -> Result<String, BuySellError> { todo!() }
    pub async fn get_recurring_plan_order_list_with_cache(&self, cancel_token: Arc<OKCancellationToken>, request: BscNativeRecurringPlanOrderListRequest) -> Result<String, BuySellError> { todo!() }
    pub async fn preheat_for_screen(&self, cancel_token: Arc<OKCancellationToken>, request: BscNativePreheatForScreenRequest) -> Result<String, BuySellError> { todo!() }
    pub async fn update_recurring_plan(&self, cancel_token: Arc<OKCancellationToken>, request: BscNativeRecurringPlanUpdateRequest) -> Result<String, BuySellError> { todo!() }
}

/// Analytics wrapper for Buy-Sell component
/// Can be called from outside OKCore via UniFFI
pub struct BuySellAnalytics;
impl BuySellAnalytics {
    /// Create a new BuySellAnalytics instance using global analytics
    pub fn new() -> Arc<BuySellAnalytics> { todo!() }
    pub fn get_analytics(&self) -> Option<Arc<dyn AnalyticsInterface>> { todo!() }
    /// Track BSC quick convert bottomsheet view event.
    /// 
    /// # Arguments
    /// * `result` - convert idle cash result (success, fail, processing)
    /// * `mode_type` - simple or exchange
    /// * `total_base_amount` - detail (optional, not sent when None)
    /// * `total_quote_amount` - detail (optional, not sent when None)
    /// * `total_base_currency` - detail (optional, not sent when None)
    pub fn track_bsc_quick_convert_bottomsheet_view(&self, result: ConvertResult, mode_type: ConvertModeType, total_base_amount: Option<String>, total_quote_amount: Option<String>, total_base_currency: Option<String>) { todo!() }
    /// Track BSC Quick Convert order confirm bottom pay click event (simple trade).
    /// 
    /// # Arguments
    /// * `amount` - amount of the order
    /// * `currency` - currency of the order
    pub fn track_bsc_quick_convert_order_confirm_btm_pay_click(&self, amount: String, currency: String) { todo!() }
    pub fn track_event_internal(&self, event_name: String, params: Vec<TrackEventParam>) { todo!() }
    pub fn track_order_detail_page_view(&self, trade_type: String) { todo!() }
    /// Track portfolio convert bottomsheet click event.
    /// 
    /// # Arguments
    /// * `status` - status of the convert order (success, failure)
    /// * `action` - action of user (dismiss, reviewdetails, done)
    pub fn track_portfolio_convert_bottomsheet_click(&self, status: ConvertOrderStatus, action: ConvertBottomSheetAction) { todo!() }
    /// Track recurring buy plan detail cancel button click event
    pub fn track_recurring_buy_plan_detail_cancel_button_click(&self) { todo!() }
    /// Track recurring buy plan detail order item click event
    pub fn track_recurring_buy_plan_detail_order_item_click(&self) { todo!() }
    /// Track recurring buy plan detail page view event
    pub fn track_recurring_buy_plan_detail_page_view(&self) { todo!() }
    /// Track recurring buy plan detail pause button click event
    pub fn track_recurring_buy_plan_detail_pause_button_click(&self) { todo!() }
    /// Track recurring buy plan detail resume button click event
    pub fn track_recurring_buy_plan_detail_resume_button_click(&self) { todo!() }
    /// Track recurring buy plan detail view all orders click event
    pub fn track_recurring_buy_plan_detail_view_all_orders_click(&self) { todo!() }
    /// Track recurring buy plan list item click event
    pub fn track_recurring_buy_plan_list_item_click(&self) { todo!() }
    /// Track recurring buy plan list page view event
    pub fn track_recurring_buy_plan_list_page_view(&self) { todo!() }
    /// Track recurring buy plan list setup button click event
    pub fn track_recurring_buy_plan_list_setup_button_click(&self) { todo!() }
    /// Track recurring buy plan order list item click event
    pub fn track_recurring_buy_plan_order_list_item_click(&self) { todo!() }
    /// Track recurring buy plan order list page view event
    pub fn track_recurring_buy_plan_order_list_page_view(&self) { todo!() }
}

/// Feature restriction utility for Buy-Sell component
/// Provides convenient methods to check feature restrictions
/// 
/// This utility wraps FeatureRestrictionService and provides buy-sell specific convenience methods.
/// Each method internally uses a specific FeatureId and directly calls the service methods.
pub struct BuySellFeatureRestrictionUtil;
impl BuySellFeatureRestrictionUtil {
    /// Create a new BuySellFeatureRestrictionUtil instance.
    /// 
    /// This constructor initializes the utility with the shared FeatureRestrictionService instance.
    /// The utility provides convenient methods to check feature restrictions specific to the Buy-Sell component.
    /// 
    /// # Returns
    /// Returns a new `BuySellFeatureRestrictionUtil` instance that can be used to check
    /// various feature restrictions such as fiat payment restrictions, multi-account support,
    /// and new onshore IA visibility.
    /// 
    /// # Usage
    /// ```kotlin
    /// val util = BuySellFeatureRestrictionUtil()
    /// if (util.isFiatRestricted()) {
    /// // Handle restricted case
    /// }
    /// ```
    pub fn new() -> Arc<BuySellFeatureRestrictionUtil> { todo!() }
    /// Check if convert is restricted
    /// Returns true if convert feature is limited
    pub fn is_convert_restricted(&self) -> bool { todo!() }
    /// Check if fiat payment is restricted
    /// Returns true if fiat payment feature is limited
    pub fn is_fiat_restricted(&self) -> bool { todo!() }
    /// Check if new onshore IA feature is hidden
    /// Returns true if new onshore IA feature is hidden
    pub fn is_new_onshore_ia_hidden(&self) -> bool { todo!() }
    /// Check if multi-account (segregated account) is supported
    /// Returns true if asset multi-account feature is NOT limited (i.e., supported)
    pub fn is_segregated_account(&self) -> bool { todo!() }
    /// Check if smart account (exchange top-up) is restricted
    /// Returns true if smart account feature is limited
    pub fn is_smart_account_restricted(&self) -> bool { todo!() }
}

/// Builder for BuySellGrafana events (matching Kotlin Grafana builder pattern)
/// This builder allows chaining method calls to set properties before tracking
pub struct BuySellGrafanaBuilder;
impl BuySellGrafanaBuilder {
    /// Create a new builder instance
    pub fn new() -> Arc<BuySellGrafanaBuilder> { todo!() }
    /// Track the event
    pub fn track(&self) { todo!() }
    /// Set the click action (matching Kotlin withClick())
    /// Returns a new builder instance with the click set
    pub fn with_click(&self, click: String) -> Arc<BuySellGrafanaBuilder> { todo!() }
    /// Set the current page (matching Kotlin withCurrentPage())
    /// Returns a new builder instance with the current page set
    pub fn with_current_page(&self, current_page: String) -> Arc<BuySellGrafanaBuilder> { todo!() }
    /// Set the error code (matching Kotlin withErrorCode())
    /// Returns a new builder instance with the error code set
    pub fn with_error_code(&self, error_code: i32) -> Arc<BuySellGrafanaBuilder> { todo!() }
    /// Set the log message (matching Kotlin withLog())
    /// Returns a new builder instance with the log set
    pub fn with_log(&self, log: String) -> Arc<BuySellGrafanaBuilder> { todo!() }
    /// Set the source (matching Kotlin withSource())
    /// Returns a new builder instance with the source set
    pub fn with_source(&self, source: String) -> Arc<BuySellGrafanaBuilder> { todo!() }
    /// Set the token name (matching Kotlin withToken())
    /// Returns a new builder instance with the token set
    pub fn with_token(&self, token: String) -> Arc<BuySellGrafanaBuilder> { todo!() }
}

/// Observable state manager for the BSC buy/sell input page (procedural macro
/// pattern). No injected dependencies — effects call the native-input-page use
/// cases directly.
pub struct BuySellInputInteractor;
impl BuySellInputInteractor {
    /// Fill the max buyable amount for the selected channel.
    pub fn buy_max(&self) { todo!() }
    /// Clear a terminal preview quote after native handles it.
    pub fn clear_preview_quote(&self) { todo!() }
    /// Delete a payment method, then reload the suggestion. The reload runs on
    /// failure too: a failing response can still have removed the method
    /// server-side, so the page must not keep rendering it. The error reaches
    /// the caller, which owns the status notification.
    /// 
    /// Deliberately NOT cancellable, unlike the reads above. Abandoning a
    /// delete mid-flight leaves the caller unable to say whether the method is
    /// gone, and the backend may have removed it anyway — the same reason the
    /// suggestion reloads on failure. The refresh is the point of putting this
    /// on the interactor rather than calling the export directly.
    pub async fn delete_payment_method(&self, params: BuySellDeletePaymentMethodParams) -> Result<(), BuySellError> { todo!() }
    /// Indicative B2C quote for the convert-channel preview hop, as JSON.
    pub async fn fetch_b2c_quote(&self, cancel_token: Arc<OKCancellationToken>, params: BuySellB2cQuoteParams) -> Result<String, BuySellError> { todo!() }
    /// CeFi buy list, cache-first. Returns the `baseCurrencies` array as JSON;
    /// the page reads no other section of the response.
    pub async fn fetch_buy_crypto_list(&self, cancel_token: Arc<OKCancellationToken>, lite_app: String) -> Result<String, BuySellError> { todo!() }
    /// Whether an order qualifies for the loaded buy campaign.
    pub fn is_campaign_reward_eligible(&self, params: BuySellCampaignEligibilityParams) -> bool { todo!() }
    /// Load or clear the buy campaign. Native supplies the rollout gate; OKCore
    /// owns the request, response parsing, cache, and mission rules.
    pub fn load_campaign(&self, enabled: bool) { todo!() }
    /// Load the cache-first CeFi crypto-asset list into observable state.
    pub fn load_crypto_asset_list(&self, lite_app: String) { todo!() }
    /// Load the trade suggestion (cache-first) for the given request.
    pub fn load_trade_suggestion(&self, request: BscNativeTradeSuggestionRequest) { todo!() }
    /// Re-fetch the last request, forcing a network read.
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
    /// Request the typed convert preview quote used by the QP sheet.
    pub fn request_preview_quote(&self, params: BuySellB2cQuoteParams) { todo!() }
    /// Request an indicative quote for the current amount/channel.
    pub fn request_quote(&self) { todo!() }
    /// Clear the entered amount, leaving every other selection alone.
    /// 
    /// The page outlives an order: native returns to a loaded input whose typed
    /// amount already bought something. Only the host knows an order completed,
    /// so it calls this; the interactor does not infer it from a refresh, which
    /// also runs when the user merely came back.
    pub fn reset_input(&self) { todo!() }
    /// Change the base (crypto) currency and re-fetch the suggestion. The picked
    /// token's display fields (icon/scale) update the state IMMEDIATELY — the
    /// selected crypto is pick-time state, not response-derived (native parity).
    pub fn select_base_currency(&self, base_currency: String, icon: String, night_icon: String, scale: i32, currency_type: i32, trade_status: i32) { todo!() }
    /// Select a DEX token from the crypto sheet: pick-time display state, then
    /// the native-parity parallel loads (trade suggestion with chain/address +
    /// DEX trade-status check + token base info). `sign_ext_info` is the CeDeFi
    /// wallet session context read at pick time; `is_passkey_compatible` is the
    /// device capability (native isPasskeyCompatible()).
    pub fn select_dex_token(&self, base_currency: String, icon: String, night_icon: String, scale: i32, chain_index: String, token_address: String, sign_ext_info: String, is_passkey_compatible: bool) { todo!() }
    /// DEX selection used by Android handoff flows. It preserves the picked
    /// chain logo together with the token selection for preview navigation.
    pub fn select_dex_token_with_chain_logo(&self, base_currency: String, icon: String, night_icon: String, scale: i32, chain_index: String, token_address: String, sign_ext_info: String, is_passkey_compatible: bool, chain_logo: String) { todo!() }
    /// Select a payment channel, method, and concrete account row. The account
    /// id is retained independently because several accounts can share one
    /// method key.
    pub fn select_payment_account(&self, deposit_platform_code: String, support_currency: String, payment: String, payment_account_id: String) { todo!() }
    /// Select a payment channel locally (no re-fetch). Matched by platform code
    /// AND support currency — multi-currency payloads carry several channels with
    /// the same code differing only by currency (e.g. USD vs USDG balance).
    pub fn select_payment_channel(&self, deposit_platform_code: String, support_currency: String) { todo!() }
    /// Channel selection that atomically retains the native sub-method handoff.
    pub fn select_payment_channel_with_sub_method(&self, deposit_platform_code: String, support_currency: String, sub_payment_method: Option<BuySellSelectedSubPaymentMethod>) { todo!() }
    /// Select a payment channel AND a specific method inside it (native
    /// method-level pick: the sheet result / Lua TPP restore). `payment` is
    /// the wire method id; empty selects the channel default (first method).
    pub fn select_payment_method(&self, deposit_platform_code: String, support_currency: String, payment: String) { todo!() }
    /// Method selection that atomically retains the native sub-method handoff.
    pub fn select_payment_method_with_sub_method(&self, deposit_platform_code: String, support_currency: String, payment: String, sub_payment_method: Option<BuySellSelectedSubPaymentMethod>) { todo!() }
    /// Complete native payment selection. Carries the selected channel, method,
    /// optional concrete account, and optional sub-method in one atomic state update.
    pub fn select_payment_selection(&self, deposit_platform_code: String, support_currency: String, payment: String, payment_account_id: Option<String>, sub_payment_method: Option<BuySellSelectedSubPaymentMethod>) { todo!() }
    /// Set a backend quick-amount preset (absolute fiat).
    pub fn select_quick_amount(&self, quick_amount: i32) { todo!() }
    /// Change the quote currency and re-fetch the suggestion (cache-first).
    pub fn select_quote_currency(&self, quote_currency: String) { todo!() }
    /// Fill a Quick-Sell percentage (25/50/75/Max) of the selected asset's
    /// available crypto balance (native `SellInputViewModel` quick-sell chips,
    /// FR-11). Sell-only: a Buy dispatch is an inert no-op. In Crypto mode the
    /// fill is the proportional crypto balance; in Fiat mode it is that balance
    /// converted at the current valid price, and a missing/non-positive price
    /// leaves the input untouched (no fabricated converted amount). The fill
    /// enters the normal validation/quote pipeline like any other input.
    pub fn sell_quick_percent(&self, percent: SellQuickPercent) { todo!() }
    /// Enable or disable debug logging.
    /// 
    /// In release builds this is a no-op, but the symbol must exist
    /// so that UniFFI bindings generated in debug mode continue to
    /// link successfully against the release library.
    pub fn set_debug(&self, _enabled: bool) { todo!() }
    /// Supply native-only wallet/device context used by DEX status checks.
    /// This must be set before the initial load and is deliberately kept out
    /// of observable state because it has no rendering meaning.
    pub fn set_dex_runtime_context(&self, sign_ext_info: String, is_passkey_compatible: bool) { todo!() }
    /// Set the raw input amount and re-run validation + re-quote.
    pub fn set_input_amount(&self, amount: String) { todo!() }
    /// How long to wait after the last [`Self::set_input_amount`] before quoting.
    /// 
    /// Zero (the default) quotes immediately, which is what a surface that debounces
    /// its own keystrokes wants. A surface that forwards every keystroke sets this
    /// instead, so the burst collapses into one request here rather than in the field.
    pub fn set_input_debounce_millis(&self, millis: u64) { todo!() }
    /// Report the amount box's rendered precision, so a percentage fill produces a
    /// value the box can hold. See [`BuySellInputState::input_display_scale`].
    pub fn set_input_display_scale(&self, scale: i32) { todo!() }
    /// Switch the validation flow (one-time vs trigger order).
    pub fn set_input_flow(&self, flow: BuySellInputFlow) { todo!() }
    /// Set a state observer (UniFFI export).
    /// 
    /// DEPRECATED: callback-based observer. Use `state_stream()` —
    /// the callback observer pins the foreign object in UniFFI's
    /// static handleMap and produces a cross-language retain cycle
    /// (Android GC + iOS ARC).
    pub fn set_state_observer(&self, observer: Arc<dyn InteractorStateChangeObserver>) { todo!() }
    /// Report which sides the selected stock trades.
    /// 
    /// Both flags, not one "is my side supported" bool: deriving that needs the
    /// trade type, which this interactor already holds. Crypto sends
    /// `(true, true)`, the default, so a caller that never calls this is
    /// unaffected.
    pub fn set_stock_side_support(&self, buy_supported: bool, sell_supported: bool) { todo!() }
    /// Configure the page trade type before the first load. This makes typed
    /// trade context available to effects (such as the asset list) that can
    /// start before the trade-suggestion request completes.
    pub fn set_trade_type(&self, trade_type: BuySellInputTradeType) { todo!() }
    /// Keypad focus switch between the trigger-price and amount boxes (Trigger
    /// flow). Presentation context fed into the interactor because the CTA
    /// slot machine dispatches on it (D4 render plan).
    pub fn set_trigger_input_focused(&self, focused: bool) { todo!() }
    /// Feed the live market (fair) price from the native subscription (Trigger
    /// flow). `is_converter_needed` prices are USD-quoted and converted here
    /// with the loaded quote currency's usdRate (native fetchCefiTokensLivePrice).
    pub fn set_trigger_market_price(&self, price: String, is_converter_needed: bool, is_stablecoin: bool) { todo!() }
    /// Set the trigger price (Trigger flow) and re-validate.
    pub fn set_trigger_price(&self, trigger_price: String) { todo!() }
    /// Set the native-provided validation config (init / auth change).
    pub fn set_validation_config(&self, config: ValidationConfig) { todo!() }
    /// Report that an unlock launch is in flight, which disables that CTA.
    pub fn set_xstock_launch_in_flight(&self, in_flight: bool) { todo!() }
    /// Report whether an xStock trading requirement is still unmet, which the
    /// render plan answers with the "Unlock stocks" CTA.
    pub fn set_xstock_requirement_pending(&self, pending: bool) { todo!() }
    /// Start the interactor lifecycle.
    /// 
    /// Calls the user's `InteractorLifecycle::start()` implementation.
    /// Override that method to add custom startup behavior (e.g., initial data loads).
    pub fn start(&self) { todo!() }
    /// Get the current state (UniFFI export).
    /// 
    /// Returns the External type (UniFFI-compatible).
    pub fn state(&self) -> BuySellInputState { todo!() }
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
    /// Toggle the input mode between Fiat and Crypto.
    pub fn switch_input_mode(&self) { todo!() }
}

/// Interactor struct.
pub struct BuySellOrderDetailInteractor;
impl BuySellOrderDetailInteractor {
    /// Registers a native observer for state changes.
    pub fn observe_state(&self, observer: Arc<dyn BuySellOrderDetailStateObserver>) { todo!() }
    /// Called when the view appears.
    /// This will automatically fetch order details if not already loaded.
    pub fn on_appear(&self) { todo!() }
    /// Called when the view disappears.
    pub fn on_disappear(&self) { todo!() }
    /// Refresh the current order detail.
    pub fn refresh(&self) { todo!() }
    /// Sets the parameters for the interactor.
    /// 
    /// This must be called before `on_appear()` to configure the order details to fetch.
    /// 
    /// # Parameters
    /// - `order_id`: The order ID to fetch details for
    /// - `trade_type`: Buy or Sell trade type
    pub fn set_parameters(&self, order_id: String, trade_type: BuySellTradeType) { todo!() }
}

/// Localization trait for BuySellOrderDetail.
/// Implemented by native platforms (iOS/Android) to provide locale-aware formatting.
pub struct BuySellOrderDetailLocalization;
impl BuySellOrderDetailLocalization {
    /// Formats timestamp to standard date string.
    /// 
    /// # Arguments
    /// * `timestamp` - Unix timestamp in milliseconds
    /// 
    /// # Returns
    /// Formatted date string (e.g., "Jan 1, 2024", etc.)
    pub fn format_date(&self, timestamp: i64) -> String { todo!() }
}

/// Native observer for state changes.
pub struct BuySellOrderDetailStateObserver;
impl BuySellOrderDetailStateObserver {
    pub fn on_state_changed(&self, state: BuySellOrderDetailState) { todo!() }
}

pub struct ConvertExecutionPageInteractor;
impl ConvertExecutionPageInteractor {
    pub fn set_state_observer(&self, observer: Arc<dyn InteractorStateChangeObserver>) { todo!() }
    pub fn start(&self) { todo!() }
    pub fn state(&self) -> ConvertExecutionPageState { todo!() }
    pub fn stop(&self) { todo!() }
}

pub struct ConvertInputInteractor;
impl ConvertInputInteractor {
    pub fn set_state_observer(&self, observer: Arc<dyn InteractorStateChangeObserver>) { todo!() }
    pub fn start(&self) { todo!() }
    pub fn state(&self) -> ConvertInputState { todo!() }
    pub fn stop(&self) { todo!() }
}

/// Interface implemented on the platform side (Android/iOS) to supply localized strings.
pub struct InputValidationLocalization;
impl InputValidationLocalization {
    /// Returns a localized message for Buy validation states.
    pub fn get_buy_localized_error_message(&self, validation_state: InputValidationResult, error_message_item: ErrorMessageItem) -> String { todo!() }
    /// Returns a localized message for Sell validation states.
    pub fn get_sell_localized_error_message(&self, validation_state: InputValidationResult, error_message_item: ErrorMessageItem) -> String { todo!() }
}

/// Interactor for managing the pro buy/sell order history list.
pub struct ProBuySellOrderHistoryListInteractor;
impl ProBuySellOrderHistoryListInteractor {
    /// Load the initial page of the list.
    pub fn load_initial_page(&self) { todo!() }
    /// Load the next page of the list (pagination).
    pub fn load_next_page(&self) { todo!() }
    /// Called when the view appears.
    pub fn on_appear(&self) { todo!() }
    /// Called when the view disappears.
    pub fn on_disappear(&self) { todo!() }
    /// Handle order item click.
    pub fn on_order_item_click(&self, order_no: String) { todo!() }
    /// Refresh the list (reload first page).
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
    /// Set filter parameters for the list.
    pub fn set_filters(&self, trade_type: Option<String>, order_status: Option<String>, past_days: Option<i32>) { todo!() }
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
    pub fn state(&self) -> ProBuySellOrderHistoryListState { todo!() }
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

/// Interactor for managing the recurring buy plan detail.
pub struct RecurringBuyPlanDetailInteractor;
impl RecurringBuyPlanDetailInteractor {
    /// Load plan details.
    pub fn load_plan_details(&self, show_loading: bool) { todo!() }
    /// Called when the view appears.
    pub fn on_appear(&self) { todo!() }
    /// Handle cancel button click.
    pub fn on_cancel_button_click(&self) { todo!() }
    /// Called when the view disappears.
    pub fn on_disappear(&self) { todo!() }
    /// Handle order item click.
    pub fn on_order_item_click(&self, order_no: String) { todo!() }
    /// Handle pause button click.
    pub fn on_pause_button_click(&self) { todo!() }
    /// Handle resume button click.
    pub fn on_resume_button_click(&self) { todo!() }
    /// Handle view all orders click.
    pub fn on_view_all_orders_click(&self) { todo!() }
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
    /// Set parameters (plan_id and is_from_result_page).
    pub fn set_parameters(&self, plan_id: String, is_from_result_page: bool) { todo!() }
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
    pub fn state(&self) -> RecurringBuyPlanDetailState { todo!() }
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
    /// Update plan (status, amount, frequency, payment method).
    pub fn update_plan(&self, status: Option<String>, edited_amount: Option<String>, edited_frequency: Option<String>, edited_payment_account_id: Option<String>, edited_deposit_platform_code: Option<String>) { todo!() }
}

/// Interactor for managing the recurring buy plan list.
pub struct RecurringBuyPlanListInteractor;
impl RecurringBuyPlanListInteractor {
    /// Load the initial page of the list.
    pub fn load_initial_page(&self) { todo!() }
    /// Load the next page of the list (pagination).
    pub fn load_next_page(&self) { todo!() }
    /// Called when the view appears (lifecycle start).
    pub fn on_appear(&self) { todo!() }
    /// Called when the view disappears (lifecycle stop).
    pub fn on_disappear(&self) { todo!() }
    /// Handle recurring list item click.
    pub fn on_recurring_list_item_click(&self, plan_id: String) { todo!() }
    /// Handle setup button click.
    pub fn on_setup_button_click(&self) { todo!() }
    /// Refresh the current list (reload from page 1).
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
    /// Set filter parameters for the list.
    pub fn set_parameters(&self, base_currency: Option<String>, quote_currency: Option<String>, status: Option<String>) { todo!() }
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
    pub fn state(&self) -> RecurringBuyPlanListState { todo!() }
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

/// Interactor for managing the recurring buy plan order list.
pub struct RecurringBuyPlanOrderListInteractor;
impl RecurringBuyPlanOrderListInteractor {
    /// Load the initial page of the list.
    pub fn load_initial_page(&self) { todo!() }
    /// Load the next page of the list (pagination).
    pub fn load_next_page(&self) { todo!() }
    /// Called when the view appears (lifecycle start).
    pub fn on_appear(&self) { todo!() }
    /// Called when the view disappears (lifecycle stop).
    pub fn on_disappear(&self) { todo!() }
    /// Handle order item click.
    pub fn on_order_item_click(&self, order_no: String) { todo!() }
    /// Refresh the current list (reload from page 1).
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
    /// Set the plan ID for filtering orders.
    pub fn set_plan_id(&self, plan_id: String) { todo!() }
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
    pub fn state(&self) -> RecurringBuyPlanOrderListState { todo!() }
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

/// Interactor struct.
pub struct TriggerOrderDetailInteractor;
impl TriggerOrderDetailInteractor {
    /// Initiates cancellation of the trigger order identified by `order_id`.
    /// 
    /// Transitions `cancel_result`: `Idle` → `Loading` → `Success` / `Error`.
    pub fn cancel_trigger_order(&self, order_id: String) { todo!() }
    /// Loads the full detail of the trigger order identified by `order_id`.
    /// 
    /// Transitions `content`: `Idle` → `Loading` → `Loaded` / `Error`.
    pub fn load_detail(&self, order_id: String, trade_type: String) { todo!() }
    /// Registers a native observer for state changes.
    pub fn observe_state(&self, observer: Arc<dyn TriggerOrderDetailStateObserver>) { todo!() }
}

/// Native observer for state changes.
pub struct TriggerOrderDetailStateObserver;
impl TriggerOrderDetailStateObserver {
    pub fn on_state_changed(&self, state: TriggerOrderDetailState) { todo!() }
}

/// Interactor struct.
pub struct TriggerOrderListInteractor;
impl TriggerOrderListInteractor {
    /// Loads the first page of trigger orders.
    /// 
    /// Transitions state: `Idle` → `Loading` → `Loaded` / `Empty` / `Error`.
    pub fn load_initial_page(&self) { todo!() }
    /// Registers a native observer for state changes.
    pub fn observe_state(&self, observer: Arc<dyn TriggerOrderListStateObserver>) { todo!() }
    /// Notifies the interactor that the screen became visible.
    /// 
    /// Triggers `load_initial_page` if content is still `Idle`.
    pub fn on_appear(&self) { todo!() }
    /// Notifies the interactor that the screen is no longer visible.
    /// 
    /// Resets content state to `Idle` so the next `on_appear` triggers a fresh load.
    pub fn on_disappear(&self) { todo!() }
    /// Called when the user taps a trigger order row.
    /// 
    /// Navigation to the detail screen is handled by the native layer.
    pub fn on_trigger_order_item_click(&self, order_id: String) { todo!() }
    /// Refreshes the trigger order list from page 1.
    /// 
    /// Transitions state: current → `Loading` → `Loaded` / `Empty` / `Error`.
    pub fn refresh(&self) { todo!() }
    /// Sets the filter parameters used for subsequent loads.
    /// 
    /// Must be called before `load_initial_page`.
    pub fn set_parameters(&self, order_category: Option<String>, status: Option<String>) { todo!() }
}

/// Native observer for state changes.
pub struct TriggerOrderListStateObserver;
impl TriggerOrderListStateObserver {
    pub fn on_state_changed(&self, state: TriggerOrderListState) { todo!() }
}

// -----------------------------------------------------------------------------
// 4. Top-Level Exported Functions (117)
// -----------------------------------------------------------------------------

/// Compares two state instances for equality.
pub fn b_s_c_quick_convert_state_is_equal(first: BSCQuickConvertState, second: BSCQuickConvertState) -> bool { todo!() }

/// FR-4 — update / cancel TPSL on a live position.
/// 
/// # Returns
/// JSON array string of `SimplePerpAdjustTpslResult`. A partial failure (per-order
/// `sCode != "0"`) is reported inside the array; the call returns `Ok`.
/// 
/// # Errors
/// See [`bsc_get_open_quote_for_simple_perp`] for the shared `BuySellError` contract.
pub async fn bsc_adjust_tpsl_for_simple_perp(cancel_token: Arc<OKCancellationToken>, params_json: String) -> Result<String, BuySellError> { todo!() }

/// Builds the byte-exact balance-rail submit payload from a
/// `BalanceSubmitInput` JSON.
/// 
/// SYNC and PURE — JSON in / JSON out, no network, no clock, no cancellation
/// token. The native executors forward the returned `SubmitOrderRequest` JSON
/// verbatim to `bsc_submit_buy_sell_order`; the shell may merge display-only
/// fields onto it before forwarding (the submit request DTO tolerates unknown
/// keys — no `deny_unknown_fields`). Only the balance rail is live here.
/// 
/// # Arguments
/// * `input_json` - JSON string of `BalanceSubmitInput`
/// 
/// # Returns
/// JSON string of `SubmitOrderRequest`, or a typed [`BscPreviewError`].
pub fn bsc_build_balance_submit_payload(input_json: String) -> Result<String, BscPreviewError> { todo!() }

/// Builds a native-rail submit payload.
pub fn bsc_build_native_submit_payload(input_json: String) -> Result<String, BscPreviewError> { todo!() }

/// Builds the render-ready preview ViewState from a BuildReq JSON.
/// 
/// # Arguments
/// * `build_req_json` - JSON string of `BuildReq`
/// 
/// # Returns
/// JSON string of `PreviewViewState`, or a typed [`BscPreviewError`].
pub fn bsc_build_preview_view_data(build_req_json: String) -> Result<String, BscPreviewError> { todo!() }

/// Builds the byte-exact 3P paymentRequest submit payload from a
/// `ThirdPartyPaymentRequestInput` JSON.
/// 
/// SYNC and PURE. Covers 3P BUY (all rails 32/33/34/35) and non-VelaFi/non-DLocal
/// SELL (transFi/yellowCard, when the input carries the OTP `codeType`/`msgCode`).
/// Fails closed with [`BscPreviewError::InvalidRequest`] on a parse error OR any
/// rail outside the {32,33,34,35} allowlist (D-5). The native executor forwards the
/// returned `PaymentRequestOrder` JSON verbatim to the paymentRequest transport.
/// 
/// # Arguments
/// * `input_json` - JSON string of `ThirdPartyPaymentRequestInput`
/// 
/// # Returns
/// JSON string of `PaymentRequestOrder`, or a typed [`BscPreviewError`].
pub fn bsc_build_third_party_payment_request_payload(input_json: String) -> Result<String, BscPreviewError> { todo!() }

/// Builds the byte-exact VelaFi (32) SELL submit payload from a
/// `VelaFiSellSubmitInput` JSON.
/// 
/// SYNC and PURE. VelaFi sell uses the modern `/fiat/order/submit` endpoint
/// (the same `SubmitOrderRequest` DTO balance uses) plus `userBankId`+`sessionId`.
/// The native executor re-invokes this per MFA attempt with a fresh `sessionId`,
/// then forwards the returned `SubmitOrderRequest` JSON to `bsc_submit_buy_sell_order`.
/// 
/// # Arguments
/// * `input_json` - JSON string of `VelaFiSellSubmitInput`
/// 
/// # Returns
/// JSON string of `SubmitOrderRequest`, or a typed [`BscPreviewError`].
pub fn bsc_build_vela_fi_sell_submit_payload(input_json: String) -> Result<String, BscPreviewError> { todo!() }

/// Debug a recurring-buy cache node/key without fetching data.
pub async fn bsc_debug_recurring_cache_status(cancel_token: Arc<OKCancellationToken>, node_id: String, params_json: String) -> Result<String, OKFFIError> { todo!() }

/// Delete a payment method.
/// 
/// # Arguments
/// * `params_json` - JSON string: `{"platformCode":"1003","paymentAccountId":"abc123","paymentMethod":"card"}`
/// - `platformCode` (required): maps to `depositPlatformCode`
/// - `paymentAccountId` (optional): payment account ID
/// - `paymentMethod` (optional): payment method type string (defaults to empty)
/// - `categoryCode` (optional): channel category code
/// - `userBankId` (optional): bank account ID for bank-transfer methods
/// - `cardNumber` (optional): masked card number for card methods
/// 
/// # Returns
/// JSON string of `DeletePaymentMethodResponse`
pub async fn bsc_delete_payment_method(_cancel_token: Arc<OKCancellationToken>, params_json: String) -> Result<String, BuySellError> { todo!() }

/// Execute convert trade.
pub async fn bsc_exchange_trade(cancel_token: Arc<OKCancellationToken>, params_json: String) -> Result<String, OKFFIError> { todo!() }

/// FR-5 — fetch the order-page config (settlement currencies + margin range).
/// 
/// # Errors
/// See [`bsc_get_open_quote_for_simple_perp`] for the shared `BuySellError` contract.
pub async fn bsc_fetch_pay_with_list_for_simple_perp(cancel_token: Arc<OKCancellationToken>, params_json: String) -> Result<String, BuySellError> { todo!() }

/// Get B2C quote for buy/sell operations.
/// 
/// # Arguments
/// * `params_json` - JSON string of `B2CQuoteRequest`
/// 
/// # Returns
/// JSON string of `B2CQuoteResponse`
pub async fn bsc_get_b2c_quote(cancel_token: Arc<OKCancellationToken>, params_json: String) -> Result<String, OKFFIError> { todo!() }

/// Fetch the buy crypto list and return as JSON string.
/// 
/// # Arguments
/// * `lite_app` - "1" for lite mode, "0" for pro mode
/// * `crypto_type` - Optional filter: "cefi" (default), "all" (cefi + dex)
/// 
/// # Returns
/// JSON string of `baseCurrencies` array, or `OKFFIError` on failure.
pub async fn bsc_get_buy_crypto_list(cancel_token: Arc<OKCancellationToken>, lite_app: String, crypto_type: Option<String>) -> Result<String, OKFFIError> { todo!() }

/// Get campaign eligibility/state for reward UI.
/// 
/// # Arguments
/// * `params_json` - JSON string of `CampaignRequest`
/// 
/// # Returns
/// JSON string of the campaign payload.
pub async fn bsc_get_campaign(cancel_token: Arc<OKCancellationToken>, params_json: String) -> Result<String, OKFFIError> { todo!() }

/// Get channel payment methods for buy/sell operations.
/// 
/// # Arguments
/// * `params_json` - JSON string of `ChannelPaymentMethodsRequest`
/// 
/// # Returns
/// JSON string of `Vec<ChannelResponse>`
pub async fn bsc_get_channel_payment_methods(cancel_token: Arc<OKCancellationToken>, params_json: String) -> Result<String, OKFFIError> { todo!() }

/// FR-2 — request a (locked) close-position quote.
/// 
/// # Errors
/// See [`bsc_get_open_quote_for_simple_perp`] for the shared `BuySellError` contract.
pub async fn bsc_get_close_quote_for_simple_perp(cancel_token: Arc<OKCancellationToken>, params_json: String) -> Result<String, BuySellError> { todo!() }

/// Get convert currency pairs (check if pair is valid).
pub async fn bsc_get_convert_currency_pairs(cancel_token: Arc<OKCancellationToken>, params_json: String) -> Result<String, OKFFIError> { todo!() }

/// Get convert-from currency list.
/// 
/// Typed relay: the response is deserialized into `Vec<ConvertCurrencyResponse>`
/// and re-serialized, which drops unmodelled fields, injects defaults that do
/// not match native ones (absent `isSupport` becomes `true` here, `false` in the
/// Android bean), and fails the whole call on a payload Gson tolerates (a
/// numeric `ccyId`, a duplicated `isXStock`/`xStock` key). That shape is what
/// the Lua/MLN bridge consumes, so it stays.
/// 
/// **UV / native cut-over consumers must use the lossless twin**
/// [`crate::public::native_bridge::convert_bridge::bsc_convert_get_from_list_no_cache`]
/// instead — same endpoint and params, raw `serde_json::Value` relay, typed
/// `BuySellError`, and no cache (which is what Android native's
/// `FIRST_CACHE` + `expire=0` amounts to).
pub async fn bsc_get_convert_from_list(cancel_token: Arc<OKCancellationToken>, params_json: String) -> Result<String, OKFFIError> { todo!() }

/// Get convert-to currency list.
pub async fn bsc_get_convert_to_list(cancel_token: Arc<OKCancellationToken>, params_json: String) -> Result<String, OKFFIError> { todo!() }

/// Get available currency balance.
/// 
/// # Arguments
/// * `params_json` - JSON: `{"fundFlowType":"funding","currency":"USD"}`
/// 
/// # Returns
/// JSON string of `AvailableCurrencyBalanceResponse`
pub async fn bsc_get_currency_balance(cancel_token: Arc<OKCancellationToken>, params_json: String) -> Result<String, OKFFIError> { todo!() }

/// Get currency pair suggestion.
pub async fn bsc_get_currency_pair_suggestion(cancel_token: Arc<OKCancellationToken>, params_json: String) -> Result<String, OKFFIError> { todo!() }

/// Get default currency for buy/sell.
/// 
/// # Arguments
/// * `params_json` - JSON: `{"tradeType":"Buy"}` or `{}`
/// 
/// # Returns
/// JSON string of `DefaultCurrencyResponse`
pub async fn bsc_get_default_currency(cancel_token: Arc<OKCancellationToken>, params_json: String) -> Result<String, OKFFIError> { todo!() }

/// Cached twin of [`bsc_get_default_currency`] for callers on the UV critical
/// path (native default-currency reads use a 1-day cache; the plain door is
/// network-only and regresses that).
/// 
/// # Arguments
/// * `params_json` - JSON: `{"tradeType":"Buy"}` or `{}` (snake_case `trade_type`
/// also accepted by the param normalizer)
/// 
/// # Returns
/// Raw wire JSON (lossless — unknown server fields survive, unlike the typed door).
/// 
/// # Cache contract
/// FirstCache via the recurring-buy node framework: default TTL 86400s, remote
/// config override clamped to [300, 86400], keyed by trade type. Entries are
/// swept only when native calls `bsc_nav_cache_invalidate` on
/// login/logout/account-switch/region-change — Rust never observes login state
/// itself.
pub async fn bsc_get_default_currency_with_cache(cancel_token: Arc<OKCancellationToken>, params_json: String) -> Result<String, BuySellError> { todo!() }

/// Wallet-safe deposit user-information use case.
/// 
/// Accepts an optional `tradeType` (JSON `paramsJson`), forwards it unchanged to
/// the existing deposit user-information backend request, and returns the backend
/// payload LOSSLESSLY so the Android Unified Verification flow can read
/// `actionCode`, `needPayback`, and every other backend field. It never reduces
/// the response through the legacy `UserInfoResponse` DTO, and references no
/// Payment-capability symbol on any path (PRD FR-006 / NFR-001), so the generated
/// `uniffi.buy_sell.bscGetDepositUserInfo` binding resolves in Wallet builds that
/// exclude `uniffi.payment`.
/// 
/// Distinct from the parameterless `bsc_get_user_info`, which stays unchanged.
/// 
/// # Arguments
/// * `cancel_token` - Cancellation token bridging the native scope into Rust async.
/// * `params_json` - JSON string; may be `{}` or `{"tradeType":"<value>"}`.
/// 
/// # Returns
/// JSON string: the lossless backend deposit user-information payload.
/// 
/// # Errors
/// Returns `OKFFIError::Error` when the params JSON is invalid (before any backend
/// call), the request/transport fails, the token is cancelled, or the success
/// payload fails to serialize.
pub async fn bsc_get_deposit_user_info(cancel_token: Arc<OKCancellationToken>, params_json: String) -> Result<String, OKFFIError> { todo!() }

/// Cached twin of [`bsc_get_deposit_user_info`], for callers whose own cache
/// sits at the HTTP layer and would be lost by moving to the uncached door —
/// i.e. Android, which sends Retrofit `FIRST_CACHE` with a 60s expiry.
/// 
/// **This is a plain TTL cache and deliberately not an equivalent of iOS's
/// 1-day `BSCUserInfoCache`.** That one is safe at a day only because it also
/// (a) refuses to store a response carrying `actionCode` / `userFreezeList` /
/// `needPayback`, (b) invalidates the entry as soon as a fetch comes back
/// carrying them, and (c) revalidates in the background on every hit. The HTTP
/// cache knows none of those fields, so a long TTL here would keep serving a
/// "not frozen" answer for a user who has since been frozen. A caller that
/// wants a long TTL must keep that eligibility/invalidation logic on its own
/// side and call the *uncached* door underneath it, exactly as iOS does today.
/// 
/// The HTTP cache key includes the uid, so entries cannot be served across an
/// account switch — unlike the node cache's L1 layer, which keys on params only.
/// 
/// # Arguments
/// * `cancel_token` - Cancellation token bridging the native scope into Rust async.
/// * `params_json` - JSON: `{"tradeType":"buy","ttlSecs":60}`. Both fields are
/// optional; an absent or non-positive `ttlSecs` behaves exactly like
/// [`bsc_get_deposit_user_info`], and larger values are clamped to
/// `USER_INFO_MAX_CACHE_TTL_SECS`.
/// 
/// # Returns
/// JSON string: the lossless backend deposit user-information payload.
/// 
/// # Errors
/// - [`BuySellError::ParseFailed`](crate::types::errors::BuySellError::ParseFailed) —
/// params or response JSON failure.
/// - [`BuySellError::NetworkFailed`](crate::types::errors::BuySellError::NetworkFailed) —
/// transport / server error.
/// - [`BuySellError::InternalError`](crate::types::errors::BuySellError::InternalError) —
/// cancellation mid-request.
pub async fn bsc_get_deposit_user_info_with_cache(cancel_token: Arc<OKCancellationToken>, params_json: String) -> Result<String, BuySellError> { todo!() }

/// Fetch DEX token price details and return as JSON string.
/// 
/// # Arguments
/// * `tokens_json` - JSON string of `[{chainId, tokenContractAddress}, ...]`
/// 
/// # Returns
/// JSON string of `[{chainId, tokenContractAddress, price, priceChange}, ...]`
pub async fn bsc_get_dex_details(cancel_token: Arc<OKCancellationToken>, tokens_json: String) -> Result<String, OKFFIError> { todo!() }

/// Get DEX quote with slippage tolerance.
/// 
/// # Arguments
/// * `cancel_token` - Cancellation token
/// * `params_json` - JSON string with DEX quote parameters
/// 
/// # Returns
/// JSON string of `DexQuoteResponse`
pub async fn bsc_get_dex_quote(cancel_token: Arc<OKCancellationToken>, params_json: String) -> Result<String, OKFFIError> { todo!() }

/// Get DEX token base information by chain and token address.
pub async fn bsc_get_dex_token_base_info(cancel_token: Arc<OKCancellationToken>, params_json: String) -> Result<String, OKFFIError> { todo!() }

/// Check DEX trade status.
pub async fn bsc_get_dex_trade_status(cancel_token: Arc<OKCancellationToken>, params_json: String) -> Result<String, OKFFIError> { todo!() }

/// Get exchange quote for convert.
pub async fn bsc_get_exchange_quote(cancel_token: Arc<OKCancellationToken>, params_json: String) -> Result<String, OKFFIError> { todo!() }

/// Get paginated fiat order list.
/// 
/// # Arguments
/// * `params_json` - JSON string of `FiatOrderListRequest`
/// 
/// # Returns
/// JSON string of `FiatOrderListResponse`
pub async fn bsc_get_fiat_order_list(cancel_token: Arc<OKCancellationToken>, params_json: String) -> Result<String, OKFFIError> { todo!() }

/// Get indicative quote for convert.
pub async fn bsc_get_indicative_quote(cancel_token: Arc<OKCancellationToken>, params_json: String) -> Result<String, OKFFIError> { todo!() }

/// FR-1 — request an open-position quote (indicative or locked).
/// 
/// # Arguments
/// * `cancel_token` - Cancellation token bridging the native coroutine/Task scope.
/// * `params_json` - JSON string of `SimplePerpGetQuoteRequest`.
/// 
/// # Returns
/// JSON string of `SimplePerpGetQuoteResponse`.
/// 
/// # Errors
/// - [`BuySellError::NetworkFailed`] — BE business error; `business_code` carries
/// the OKX code for native branching.
/// - [`BuySellError::ParseFailed`] — request/response JSON (de)serialization failure.
/// - [`BuySellError::InternalError`] — cancellation mid-request.
pub async fn bsc_get_open_quote_for_simple_perp(cancel_token: Arc<OKCancellationToken>, params_json: String) -> Result<String, BuySellError> { todo!() }

/// Get order detail.
/// 
/// # Arguments
/// * `params_json` - JSON string of `OrderDetailRequest`
/// 
/// # Returns
/// JSON string of `OrderDetailResponse`
pub async fn bsc_get_order_detail(cancel_token: Arc<OKCancellationToken>, params_json: String) -> Result<String, OKFFIError> { todo!() }

/// Get order status (for polling).
/// 
/// # Arguments
/// * `params_json` - JSON string of `OrderStatusRequest`
/// 
/// # Returns
/// JSON string of `OrderStatusResponse`
pub async fn bsc_get_order_status(cancel_token: Arc<OKCancellationToken>, params_json: String) -> Result<String, OKFFIError> { todo!() }

/// Get quick change currencies.
pub async fn bsc_get_quick_change_currencies(cancel_token: Arc<OKCancellationToken>, params_json: String) -> Result<String, OKFFIError> { todo!() }

/// Get quick change order detail.
pub async fn bsc_get_quick_change_order_detail(cancel_token: Arc<OKCancellationToken>, params_json: String) -> Result<String, OKFFIError> { todo!() }

/// Get quoted prices for buy/sell operations.
/// 
/// # Arguments
/// * `params_json` - JSON string of `QuotedPriceRequest`
/// 
/// # Returns
/// JSON string of `Vec<QuotedPriceResponse>`
pub async fn bsc_get_quoted_price(cancel_token: Arc<OKCancellationToken>, params_json: String) -> Result<String, OKFFIError> { todo!() }

/// Get recurring buy frequency options.
pub async fn bsc_get_recurring_frequency(cancel_token: Arc<OKCancellationToken>, params_json: String) -> Result<String, OKFFIError> { todo!() }

/// Get recurring buy plan detail.
pub async fn bsc_get_recurring_plan_detail(cancel_token: Arc<OKCancellationToken>, params_json: String) -> Result<String, OKFFIError> { todo!() }

/// Get recurring buy plan list.
pub async fn bsc_get_recurring_plan_list(cancel_token: Arc<OKCancellationToken>, params_json: String) -> Result<String, OKFFIError> { todo!() }

/// Get recurring buy plan order list.
pub async fn bsc_get_recurring_plan_order_list(cancel_token: Arc<OKCancellationToken>, params_json: String) -> Result<String, OKFFIError> { todo!() }

/// Resolves the VelaFi SELL authentication mode/session through OKCore.
pub async fn bsc_get_sell_authentication(cancel_token: Arc<OKCancellationToken>, params_json: String) -> Result<String, OKFFIError> { todo!() }

/// Fetch the sell crypto list and return as JSON string.
/// 
/// # Arguments
/// * `quote_currency` - Fiat currency code (e.g. "USD")
/// * `crypto_type` - Optional filter: "cefi" (default), "all" (cefi + dex)
/// 
/// # Returns
/// JSON string of `CurrencyTokenResponse` array, or `OKFFIError` on failure.
pub async fn bsc_get_sell_crypto_list(cancel_token: Arc<OKCancellationToken>, quote_currency: String, crypto_type: Option<String>) -> Result<String, OKFFIError> { todo!() }

/// Get simple-bot (grid / DCA) channel payment methods.
/// 
/// Hits the dedicated simple-bot endpoint. The request shape is identical to
/// `bsc_get_channel_payment_methods` (`ChannelPaymentMethodsRequest`); the
/// difference is the endpoint, the forced `includeFundingBalance=false`, and
/// the server-side per-bot `clientScene` allow-list. `client_scene`
/// (`grid_bot` / `simple_bot`) should be set by the caller.
/// 
/// # Arguments
/// * `params_json` - JSON string of `ChannelPaymentMethodsRequest`
/// 
/// # Returns
/// JSON string of `Vec<ChannelResponse>`
pub async fn bsc_get_simple_bot_payment_methods(cancel_token: Arc<OKCancellationToken>, params_json: String) -> Result<String, OKFFIError> { todo!() }

/// Get simple-bot (grid / DCA) channel payment methods — typed-request variant.
/// 
/// Behaviourally identical to [`bsc_get_simple_bot_payment_methods`] (same
/// endpoint, forced `includeFundingBalance=false`, same per-bot `client_scene`
/// allow-list) but accepts a typed [`ChannelPaymentMethodsRequest`] instead of a
/// JSON string, so callers that already build the request natively skip the JSON
/// encode/decode round-trip.
/// 
/// The response is still returned as a JSON string of `Vec<ChannelResponse>`:
/// that response tree is large and deeply nested, so it stays on this crate's
/// thin-FFI (JSON-out) convention rather than being exported as a Record tree.
/// 
/// # Arguments
/// * `cancel_token` - cancellation token bridging the native Task/coroutine scope
/// * `request` - typed `ChannelPaymentMethodsRequest`
/// 
/// # Returns
/// JSON string of `Vec<ChannelResponse>`
pub async fn bsc_get_simple_bot_payment_methods_typed(cancel_token: Arc<OKCancellationToken>, request: ChannelPaymentMethodsRequest) -> Result<String, OKFFIError> { todo!() }

/// FR-6 — fetch order-detail entries (Activity-L2 events) for a taker `ordId`.
/// 
/// # Arguments
/// * `cancel_token` - Cancellation token bridging the native coroutine/Task scope.
/// * `params_json` - JSON string of `SimplePerpGetOrderDetailRequest`
/// (`{"ordId":"<taker order id>"}`).
/// 
/// # Returns
/// JSON array string of `SimplePerpOrderDetailItem`. An empty array is a
/// valid success (the order is outside the 3-month BE history window).
/// 
/// # Errors
/// See [`bsc_get_open_quote_for_simple_perp`] for the shared `BuySellError` contract.
pub async fn bsc_get_simple_perp_order_detail(cancel_token: Arc<OKCancellationToken>, params_json: String) -> Result<String, BuySellError> { todo!() }

/// Get TPSL suggestion (presets, thresholds, payment channels).
/// 
/// # Arguments
/// * `cancel_token` - Cancellation token bridging native coroutine/Task scope
/// * `params_json` - JSON string of `TpslSuggestionRequest`
/// 
/// # Returns
/// JSON string containing baseCurrency, quoteCurrency, tpConfig, slConfig, paymentChannels
/// 
/// # Errors
/// Returns `OKFFIError` if JSON parsing, network request, or response serialization fails.
pub async fn bsc_get_tpsl_suggestion(cancel_token: Arc<OKCancellationToken>, params_json: String) -> Result<String, OKFFIError> { todo!() }

/// Get the trade-discovery currency list (xStocks universe).
/// 
/// This is the ONLY code path reaching `/priapi/v3/b2c/fiat/trade/currencies`
/// (OSBSC-1386 FR-001). Returns [`OKFFIError`] via the crate's canonical
/// flat-error mapping (`map_err`): the PRD (§7, §18) selects the flat error for
/// this endpoint because consumers do not branch on a business code, so a
/// backend business failure and a transport/parse failure both surface as
/// [`OKFFIError::Error`] carrying the `Display` message. A successful response
/// carrying an empty `currencies` list stays a distinguishable `Ok` (FR-012).
/// 
/// Every exit path — invalid params JSON, request/transport failure, response
/// serialization failure, and cancellation via `cancel_token` — emits a
/// `phase=complete status=failed` completion trace before returning (FR-14).
/// The trace is emitted at the `with_cancellation` boundary so the cancellation
/// path (which short-circuits before the body completes) is also covered.
/// 
/// # Arguments
/// * `params_json` - JSON string of `TradeCurrenciesRequest`
/// 
/// # Returns
/// JSON string of `TradeCurrenciesResponse`
/// 
/// # Errors
/// Returns `OKFFIError::Error` when the params JSON is invalid, the request
/// fails (network / non-zero business code), the response cannot be serialized,
/// or the operation is cancelled via `cancel_token`.
pub async fn bsc_get_trade_currencies(cancel_token: Arc<OKCancellationToken>, params_json: String) -> Result<String, OKFFIError> { todo!() }

/// Cache-first read of the trade-currency list that bounds how stale an answer
/// this caller will accept (OSBSC-1492).
/// 
/// The list is cached for a day, which is right for a per-entity catalogue but
/// wrong for the Sell surfaces: Sell shows what the user can dispose of right
/// now, and the crypto list beside it is already read at 5 seconds there while
/// Buy reads at 5 minutes. This is that same distinction for xStocks.
/// 
/// `read_ttl_secs` narrows the READ window ONLY. It does not change what gets
/// written, so asking for 5 seconds here does not shorten the entry that
/// [`bsc_has_trade_currencies`] and [`bsc_trade_currency_trade_types`] read —
/// those judge freshness by the stored ttl, and a short write would make the
/// Stock chip vanish and the Convert fallback stop firing five seconds after
/// every Sell fetch. Pass `0` for the configured default; a value above it is
/// clamped down to it, because no caller may see data older than the flag allows.
/// 
/// Prefer this over [`bsc_refresh_trade_currencies`] for "this surface needs
/// fresh data": refresh goes to the network unconditionally, so a user bouncing
/// in and out of Sell pays for every entry. A 5s window de-duplicates the burst.
/// 
/// # Arguments
/// * `params_json` - JSON string of `TradeCurrenciesRequest`
/// * `read_ttl_secs` - maximum acceptable age in seconds; `0` means the default
pub async fn bsc_get_trade_currencies_with_read_ttl(cancel_token: Arc<OKCancellationToken>, params_json: String, read_ttl_secs: i64) -> Result<String, OKFFIError> { todo!() }

/// Get trade suggestion (currencies + payment channels).
/// 
/// # Arguments
/// * `params_json` - JSON string of `TradeSuggestionRequest`
/// 
/// # Returns
/// JSON string of `TradeSuggestionResponse`
pub async fn bsc_get_trade_suggestion(cancel_token: Arc<OKCancellationToken>, params_json: String) -> Result<String, OKFFIError> { todo!() }

/// Get the authenticated user's positive tradable balances for a currency type.
/// 
/// The endpoint is intentionally uncached: this is current portfolio state and
/// callers use it to decide which assets appear in a Sell selector.
pub async fn bsc_get_trade_tradable_balances(cancel_token: Arc<OKCancellationToken>, params_json: String) -> Result<String, OKFFIError> { todo!() }

/// Get trigger order list (TP/SL + limit orders).
/// 
/// Fetches the user's pending trigger orders from
/// `/priapi/v3/b2c/trigger-order/list`. Used by the Activity tab's trigger
/// order list page and by TPSL preview limit-checks.
/// 
/// # Arguments
/// * `cancel_token` - Cancellation token bridging native coroutine/Task scope
/// into the Rust async; cancelling on the native side aborts the in-flight
/// request and returns `OKFFIError::Error` with a cancellation message.
/// * `params_json` - JSON string of `TriggerOrderListRequest`
/// 
/// # Returns
/// JSON string of `TriggerOrderListResponse`
/// 
/// # Errors
/// Returns `OKFFIError::Error` when:
/// * `params_json` cannot be deserialized into `TriggerOrderListRequest`
/// * the `/priapi/v3/b2c/trigger-order/list` request fails (network / non-zero business code)
/// * the response cannot be serialized to JSON
/// * the operation is cancelled via `cancel_token`
pub async fn bsc_get_trigger_order_list(cancel_token: Arc<OKCancellationToken>, params_json: String) -> Result<String, OKFFIError> { todo!() }

/// Get input-page UI flow mode.
/// 
/// # Arguments
/// * `params_json` - JSON string of `UiFlowRequest`
/// 
/// # Returns
/// JSON string of `UiFlowResponse`
pub async fn bsc_get_ui_flow(cancel_token: Arc<OKCancellationToken>, params_json: String) -> Result<String, OKFFIError> { todo!() }

/// Get user info for deposit/withdraw operations.
/// 
/// # Returns
/// JSON string of `UserInfoResponse`
pub async fn bsc_get_user_info(cancel_token: Arc<OKCancellationToken>) -> Result<String, OKFFIError> { todo!() }

/// Whether the cached trade-currency list holds at least one currency
/// (OSBSC-1482).
/// 
/// The tab-visibility question, answered in one place so no surface re-derives
/// it. Cache-only: it never triggers a fetch, and a cold cache answers `false`
/// — which is the fail-closed reading the L1 surfaces want, since they
/// re-evaluate when the async read lands.
/// 
/// Paused currencies count. `tradeStatus = 0` means temporarily unavailable, not
/// absent, so a universe of only paused entries still shows the surface.
/// 
/// # Arguments
/// * `params_json` - JSON string of `TradeCurrenciesRequest`
pub fn bsc_has_trade_currencies(params_json: String) -> bool { todo!() }

/// Whether a validation result enables the preview/CTA. Exported for
/// native so the button-enable rule stays in Rust (native reads
/// `state.validationResult` and calls this instead of re-deriving the valid set).
pub fn bsc_input_is_valid(result: InputValidationResult) -> bool { todo!() }

pub async fn bsc_native_cache_invalidate(cancel_token: Arc<OKCancellationToken>, request: BscNativeCacheEventRequest) -> Result<String, BuySellError> { todo!() }

pub async fn bsc_native_debug_recurring_cache_status(cancel_token: Arc<OKCancellationToken>, request: BscNativeRecurringCacheStatusRequest) -> Result<String, BuySellError> { todo!() }

pub async fn bsc_native_on_account_changed(cancel_token: Arc<OKCancellationToken>) -> Result<String, BuySellError> { todo!() }

pub async fn bsc_native_on_account_switch(cancel_token: Arc<OKCancellationToken>) -> Result<String, BuySellError> { todo!() }

pub async fn bsc_native_on_deposit_add_funds(cancel_token: Arc<OKCancellationToken>) -> Result<String, BuySellError> { todo!() }

pub async fn bsc_native_on_env_changed(cancel_token: Arc<OKCancellationToken>) -> Result<String, BuySellError> { todo!() }

pub async fn bsc_native_on_login(cancel_token: Arc<OKCancellationToken>) -> Result<String, BuySellError> { todo!() }

pub async fn bsc_native_on_logout(cancel_token: Arc<OKCancellationToken>) -> Result<String, BuySellError> { todo!() }

pub async fn bsc_native_on_memory_pressure(cancel_token: Arc<OKCancellationToken>) -> Result<String, BuySellError> { todo!() }

pub async fn bsc_native_on_payment_method_updated(cancel_token: Arc<OKCancellationToken>, request: BscNativeLifecyclePlanEventRequest) -> Result<String, BuySellError> { todo!() }

pub async fn bsc_native_on_recurring_plan_update_success(cancel_token: Arc<OKCancellationToken>, request: BscNativeLifecyclePlanEventRequest) -> Result<String, BuySellError> { todo!() }

pub async fn bsc_native_on_recurring_submit_success(cancel_token: Arc<OKCancellationToken>, request: BscNativeLifecyclePlanEventRequest) -> Result<String, BuySellError> { todo!() }

pub async fn bsc_native_on_region_changed(cancel_token: Arc<OKCancellationToken>) -> Result<String, BuySellError> { todo!() }

pub async fn bsc_native_on_site_changed(cancel_token: Arc<OKCancellationToken>) -> Result<String, BuySellError> { todo!() }

pub async fn bsc_native_preheat_for_screen(cancel_token: Arc<OKCancellationToken>, request: BscNativePreheatForScreenRequest) -> Result<String, BuySellError> { todo!() }

/// Invalidate recurring buy API cache after mutation/account lifecycle events.
pub async fn bsc_nav_cache_invalidate(cancel_token: Arc<OKCancellationToken>, event: String, params_json: String) -> Result<String, OKFFIError> { todo!() }

/// Read the cached trade-currency list without touching the network
/// (OSBSC-1482).
/// 
/// Synchronous and cache-only, so a container can decide whether a Stock surface
/// exists while it is laying itself out, instead of awaiting an FFI round trip
/// and flickering the tab bar in afterwards.
/// 
/// `None` covers cold, expired and different-identity entries alike. It means
/// "no answer cached", never "no currencies" — a caller that needs a populated
/// answer calls [`bsc_get_trade_currencies`] and peeks again once it completes.
/// 
/// # Arguments
/// * `params_json` - JSON string of `TradeCurrenciesRequest`
pub fn bsc_peek_trade_currencies(params_json: String) -> Option<String> { todo!() }

/// General pre-check before order submission.
/// 
/// # Arguments
/// * `cancel_token` - Cancellation token bridging the native scope into Rust async.
/// * `params_json` - JSON string of `PreCheckRequest`
/// 
/// # Returns
/// JSON string of the lossless pre-check result.
/// 
/// # Wire contract
/// The repository always sends the `fund_flow_type: funding` header — identical
/// to the native `runPrecheckForBuySell` UV contract, which also fixes FUNDING
/// via its default provider. A non-FUNDING caller needs a new parameter here
/// (audit note A5, door-param-swallow-audit-20260716).
/// 
/// The 12 query keys match the native Retrofit contract verbatim, but the
/// *omission* semantics differ: every `None` is dropped from the query string,
/// whereas Android's `@Query` defaults put the parameter on the wire with an
/// empty / `false` / `1` value, and its UV layer hardcodes `needCheckLimit=true`
/// (the API default of `false` never reaches the server). A caller replacing the
/// native read must therefore send all 12 fields explicitly — in particular
/// `need_check_limit: Some(true)` — rather than relying on `None`; whether the
/// server treats an omitted key the same as an empty one is unverified.
/// 
/// # Errors
/// - [`BuySellError::NetworkFailed`] — server returned an error. `business_code`
/// carries the OKX code; notable: `61005` = max trigger-order limit reached.
/// - [`BuySellError::ParseFailed`] — request/response JSON (de)serialization failure.
/// - [`BuySellError::InternalError`] — cancellation mid-request.
pub async fn bsc_pre_check(cancel_token: Arc<OKCancellationToken>, params_json: String) -> Result<String, BuySellError> { todo!() }

/// Preheat recurring buy API cache for a Lua/native screen intent.
pub async fn bsc_preheat_for_screen(cancel_token: Arc<OKCancellationToken>, screen_id: String, params_json: String) -> Result<String, OKFFIError> { todo!() }

/// Force a network read of the trade-currency list, bypassing the cache
/// (OSBSC-1482).
/// 
/// [`bsc_get_trade_currencies`] became cache-first in OSBSC-1481, which is the
/// right default but takes away a caller's ability to guarantee freshness. This
/// is that ability, made explicit at the call site: reach for it on a
/// pull-to-refresh or when returning from a flow that could have changed the
/// user's eligible universe — not by default, or the cache is pointless.
/// 
/// A failure leaves any existing cached entry untouched, so a transient network
/// error cannot empty a surface that was working a moment ago.
/// 
/// # Arguments
/// * `params_json` - JSON string of `TradeCurrenciesRequest`
/// 
/// # Returns
/// JSON string of `TradeCurrenciesResponse`
pub async fn bsc_refresh_trade_currencies(cancel_token: Arc<OKCancellationToken>, params_json: String) -> Result<String, OKFFIError> { todo!() }

/// Register OKBuySell MCP action card capabilities into the action-cards registry.
/// 
/// Idempotent under repeated calls (Once guard) so eager and lazy registration
/// paths can coexist during staged rollout without duplicating tool entries.
pub fn bsc_register_mcp_capabilities() { todo!() }

/// Search CeDeFi tokens for the token picker.
pub async fn bsc_search_dex_tokens(cancel_token: Arc<OKCancellationToken>, params_json: String) -> Result<String, OKFFIError> { todo!() }

/// Submit a buy/sell order.
/// 
/// # Arguments
/// * `params_json` - JSON string of `SubmitOrderRequest`
/// 
/// # Returns
/// JSON string of `SubmitOrderResponse`
/// 
/// Returns [`BuySellError`] (not [`OKFFIError`]) so the platform submit-error
/// consumers (`jsonCallWithResult` typed catch on iOS, `toBridgeError` on
/// Android) actually receive the typed variant — the legacy [`OKFFIError`]
/// path flattens it to a `Request to '<url>' failed:` display string before
/// it crosses the FFI, which those catches can never match.
pub async fn bsc_submit_buy_sell_order(cancel_token: Arc<OKCancellationToken>, params_json: String) -> Result<String, BuySellError> { todo!() }

/// FR-3 — commit a locked quote, optionally attaching TPSL.
/// 
/// # Errors
/// See [`bsc_get_open_quote_for_simple_perp`] for the shared `BuySellError` contract.
pub async fn bsc_submit_order_for_simple_perp(cancel_token: Arc<OKCancellationToken>, params_json: String) -> Result<String, BuySellError> { todo!() }

/// Submit order for OTC agent.
/// 
/// # Arguments
/// * `cancel_token` - Cancellation token
/// * `params_json` - JSON string of `SubmitOtcAgentOrderRequest`
/// 
/// # Returns
/// JSON string of `SubmitOtcAgentOrderResponse`
/// 
/// # Errors
/// - [`BuySellError::NetworkFailed`] — server returned an error. `error_message`
/// is the BE-supplied message verbatim (no URL prefix); `business_code` carries
/// the OKX business code (e.g. `102103` trade-limit) for downstream UI branching.
/// - [`BuySellError::ParseFailed`] — request/response JSON (de)serialization
/// failure.
/// - [`BuySellError::InternalError`] — wrapper-level failure including
/// cancellation (via `From<OKCancellationError>`).
/// 
/// Returns [`BuySellError`] (not [`OKFFIError`]) so the native (Lua / Kotlin)
/// callers can branch on `business_code` without having to parse the
/// `Request to '<url>' failed:` string the legacy [`OKFFIError`] path
/// surfaces from `NetworkRepositoryError`'s thiserror Display.
pub async fn bsc_submit_otc_agent_order(cancel_token: Arc<OKCancellationToken>, params_json: String) -> Result<String, BuySellError> { todo!() }

/// Submit a BUY-only 3P order via the legacy paymentRequest endpoint.
/// 
/// # Arguments
/// * `params_json` - JSON string of `PaymentRequestOrder` (the byte-exact payload
/// built by `bsc_build_third_party_payment_request_payload`, forwarded verbatim).
/// The executor MAY additionally inject a top-level `__riskHeaders` object
/// (a `{string: string}` map of native risk-attestation headers) into this JSON;
/// it is a CONTROL key stripped from the wire body (only `PaymentRequestOrder`
/// fields are serialized to the request) and attached VERBATIM as request
/// headers. Absent `__riskHeaders` (the default) keeps the request byte-identical
/// and lets the native network layer own risk-header injection — see the
/// `submit_third_party_payment_request` doc + the SUBMIT_SPEC risk-header
/// amendment for the per-platform reality.
/// 
/// # Returns
/// JSON string of `PaymentFormResponse` {orderNo, url, formRequest, formHtml, ...}.
/// 
/// # Errors
/// Returns [`BuySellError`] (not [`OKFFIError`]) so the native submit-error
/// consumers receive the typed variant with `raw_response_body` preserved for the
/// dynamic error templates — same rationale as [`bsc_submit_buy_sell_order`].
pub async fn bsc_submit_third_party_payment_request(cancel_token: Arc<OKCancellationToken>, params_json: String) -> Result<String, BuySellError> { todo!() }

/// Which sides a currency may be traded on, per the cached list (OSBSC-1482).
/// 
/// Replaces the per-instance maps both platforms build as a side effect of their
/// own fetch, which answer "not supported" on any instance that has not fetched.
/// 
/// `None` means UNKNOWN — cold cache, or a currency absent from the list. It is
/// deliberately distinct from `Some(vec![])`, which means "known, and no side is
/// supported". The Convert fallback depends on that difference: it must not read
/// a cold cache as evidence that a token is untradable and redirect on it.
/// 
/// Matching is case-insensitive. Values are the wire's own — `buy` and `sell`.
/// 
/// # Arguments
/// * `params_json` - JSON string of `TradeCurrenciesRequest`
/// * `currency` - canonical or display currency symbol, any casing
pub fn bsc_trade_currency_trade_types(params_json: String, currency: String) -> Option<Vec<String>> { todo!() }

/// Update recurring buy plan.
pub async fn bsc_update_recurring_plan(cancel_token: Arc<OKCancellationToken>, params_json: String) -> Result<String, OKFFIError> { todo!() }

/// Buy/sell use-check gate (`GET /priapi/v3/b2c/deposit/channel/use-check`).
/// 
/// Real-time availability check consulted by the UV chain before entering the
/// buy/sell input page. Never cached — it is a real-time gate (explicitly
/// excluded from input-page preheat).
/// 
/// # Arguments
/// * `params_json` - JSON: `{"baseCurrency":"BTC","quoteCurrency":"USD"}`,
/// optionally with `"fundFlowType":"trading"`.
/// 
/// # Returns
/// Raw wire JSON (lossless). Natives read `canBuy` / `canSell` plus a failure
/// reason whose key differs per platform — iOS decodes `canNotBuyReason` /
/// `canNotSellReason` (`GetUseCheckResponse`), Android decodes
/// `canBuyFailReason` / `canSellFailReason` (`UseCheckResponse`), so at most one
/// spelling matches any given backend payload and the other platform silently
/// renders no reason. Relaying raw keeps both spellings alive; a future typed
/// DTO here would have to model both or it would break one platform. Natives
/// also keep their own defaults for absent fields (see the module header).
/// 
/// # Errors
/// [`BuySellError::ParseFailed`] on invalid params (before any network call);
/// [`BuySellError::NetworkFailed`] on transport/server errors (the raw
/// response body survives for dual-field error-code recovery).
pub async fn bsc_use_check(cancel_token: Arc<OKCancellationToken>, params_json: String) -> Result<String, BuySellError> { todo!() }

/// BSC UV login check against the current user state.
/// 
/// # Returns
/// The login verdict plus the raw state it was derived from, so the A/B
/// observation can attribute a divergence to a field rather than to a boolean.
/// `is_sub_user` is included as a reading but NOT as a verdict — see
/// [`bsc_uv_sub_account_verdict`], which is the door the ladder's
/// `.subAccount` check must use.
/// 
/// Synchronous and non-blocking — reads the in-memory user store that both
/// shells populate via `register_native_user_storage`. Infallible: an absent
/// user is a valid state (not logged in, not a sub-account), not an error.
pub fn bsc_uv_auth_verdict() -> BscUvAuthVerdict { todo!() }

/// BSC UV country-restriction check.
/// 
/// # Arguments
/// * `entry` — the UV entry point, which selects the feature axis.
/// 
/// # Returns
/// `is_restricted` (the ladder check FAILS when true) and the axis consulted.
/// 
/// Synchronous and infallible: `FeatureRestrictionService` answers from state
/// the native side has already loaded.
pub fn bsc_uv_country_verdict(entry: BscUvEntry) -> BscUvCountryVerdict { todo!() }

/// BSC UV frozen-account check.
/// 
/// # Arguments
/// * `params` — call-site predicate, entry / target page / trade side, and the
/// `actionCode` + `userFreezeList` read from the deposit user-info payload.
/// 
/// # Returns
/// `is_frozen` (the ladder check FAILS when true) plus what decided it, so the
/// A/B observation can attribute a divergence to a field.
/// 
/// Pure and infallible: a caller that could not fetch the payload should keep
/// its existing fail-open behaviour rather than calling this with a guess.
pub fn bsc_uv_frozen_verdict(params: BscUvFrozenParams) -> BscUvFrozenVerdict { todo!() }

/// BSC UV input-page preload — iOS's third pair check.
/// 
/// A third door rather than a flavour flag on [`bsc_uv_pair_verdict`], because
/// this is a different CHECK, not a variant: iOS reaches it from the
/// `.inputPage` DESTINATION rather than from the ladder, it has no
/// short-circuits at all, and two of its steps differ from Android's in ways
/// that change the verdict (an empty default currency is taken as-is; a
/// `Convert` side issues the request and is then blocked). Both differences are
/// documented on `BscUvUseCheckFlavour::InputPagePreload` and pinned by tests, with
/// the Android behaviour as the discriminator.
/// 
/// # Arguments
/// * `cancel_token` — cooperative cancellation for the in-flight requests.
/// * `params` — the same record the other two pair doors take. `entry` and
/// `target_tab` are ignored: the destination is the only context here.
/// 
/// # Returns
/// Same shape as the other two. `resolved_quote_currency` is the value iOS
/// writes back onto `BuySellInputPageParameters`; `outcome` maps onto
/// `SupportCurrency.isSupport`, and the reason onto its `reason`.
/// 
/// # Errors
/// Only cancellation. iOS's own path THROWS and aborts the whole preload when
/// either read fails — that mapping stays native, so read
/// `DefaultCurrencyUnavailable` (the default-currency read died; no use-check
/// was issued) and `UseCheckUnavailable` (the use-check died) and rethrow on
/// both.
pub async fn bsc_uv_input_page_preload_verdict(cancel_token: Arc<OKCancellationToken>, params: BscUvPairParams) -> Result<BscUvPairVerdict, BuySellError> { todo!() }

/// BSC UV KYC check against the compliance crate's cached payload.
/// 
/// # Arguments
/// * `target_level` — the KYC level the entry requires (2 on both platforms).
/// 
/// # Returns
/// The verdict, or `None` when nothing is cached — in which case the caller
/// falls through to [`bsc_uv_kyc_verdict_from_network`], which is what both
/// platforms do today.
/// 
/// Synchronous and non-blocking: it reads the in-memory cache only. The
/// compliance repository additionally spawns a background refresh on every
/// cache read; that refresh cannot change the value returned here.
pub fn bsc_uv_kyc_verdict_from_cache(target_level: i32) -> Option<BscUvKycVerdict> { todo!() }

/// BSC UV KYC check against a fresh network read.
/// 
/// # Arguments
/// * `cancel_token` — cooperative cancellation for the in-flight request.
/// * `target_level` — the KYC level the entry requires (2 on both platforms).
/// 
/// # Errors
/// - [`BuySellError::NetworkFailed`] — the KYC request failed. `http_status`
/// and `business_code` are 0: the compliance crate surfaces only a message,
/// and inventing codes here would be worse than reporting none.
/// - [`BuySellError::InternalError`] — the caller cancelled.
/// 
/// Does not retry. iOS does not retry either, and Android's retry is the path
/// whose divergent rule this crate deliberately does not reproduce, so retry
/// policy stays with the caller.
pub async fn bsc_uv_kyc_verdict_from_network(cancel_token: Arc<OKCancellationToken>, target_level: i32) -> Result<BscUvKycVerdict, BuySellError> { todo!() }

/// BSC UV KYC check against a payload the caller has already fetched.
/// 
/// Pure: no network, no cache read, no background refresh. Use this when the
/// native side holds the KYC payload and wants the shared verdict for it —
/// notably on the A/B control arm, where the two fetching doors below would
/// generate traffic the native path does not and so would not be a no-op.
/// 
/// # Arguments
/// * `snapshot` — the three model-layer readings, straight off
/// `OKKYCTypeInfo` / `KycInfosBean`. See [`BscUvKycSnapshot`].
/// * `target_level` — the KYC level the entry requires (2 on both platforms).
pub fn bsc_uv_kyc_verdict_from_snapshot(snapshot: BscUvKycSnapshot, target_level: i32) -> BscUvKycVerdict { todo!() }

/// BSC UV `optional_pair_check`.
/// 
/// A separate door rather than a flag on [`bsc_uv_pair_verdict`], because the
/// two checks branch on different axes from the same parameters: this one on
/// the target tab, that one on the entry and the trade side. On the BUY tab
/// this check gates nothing — it resolves the quote and passes.
/// 
/// # Arguments
/// * `cancel_token` — cooperative cancellation for the in-flight requests.
/// * `params` — the same record [`bsc_uv_pair_verdict`] takes, so a caller can
/// build it once and run both checks.
/// 
/// # Errors
/// Same contract as [`bsc_uv_pair_verdict`]: only cancellation. Read failures
/// are folded into the verdict so native keeps its own mapping — and the two
/// checks map them differently, which is exactly why Rust must not.
pub async fn bsc_uv_optional_pair_verdict(cancel_token: Arc<OKCancellationToken>, params: BscUvPairParams) -> Result<BscUvPairVerdict, BuySellError> { todo!() }

/// BSC UV `pair_check`.
/// 
/// # Arguments
/// * `cancel_token` — cooperative cancellation for the in-flight requests.
/// * `params` — the destination parameters. Nothing here is fetched; see
/// [`BscUvPairParams`].
/// 
/// # Returns
/// The verdict plus the resolved quote currency. A failed use-check is NOT an
/// error — it comes back as `UseCheckUnavailable`, because the two platforms
/// (and Android's two pair checks) map that outcome differently and the mapping
/// stays native.
/// 
/// # Errors
/// [`BuySellError::InternalError`] — the caller cancelled. Nothing else: every
/// read failure is folded into the verdict.
pub async fn bsc_uv_pair_verdict(cancel_token: Arc<OKCancellationToken>, params: BscUvPairParams) -> Result<BscUvPairVerdict, BuySellError> { todo!() }

/// BSC UV sub-account check against the current user state.
/// 
/// # Arguments
/// * `params` — entry, target tab and app mode. All three are needed: the rule
/// is NOT `!isSubUser`, which is right in only 5 of its 8 cells. Android
/// must pass `BscL2` (never `BscL2Dex`) because it has no DEX landing page;
/// see the module docs in [`crate::core::usecase::uv_auth`].
/// 
/// # Returns
/// The verdict plus `admitted_by_pro_mode`, which iOS needs if it wants to keep
/// setting `failedPreVerifyType` once its `shouldDeferUVCheck` branch stops
/// being reached for the pro-mode cells.
/// 
/// Synchronous, non-blocking, infallible — same store, same reasons as
/// [`bsc_uv_auth_verdict`].
pub fn bsc_uv_sub_account_verdict(params: BscUvSubAccountParams) -> BscUvSubAccountVerdict { todo!() }

/// True iff the Crypto/Stock sub-tab's Stock segment should render the CN/UAE
/// "TradFi" label instead of "Stock".
/// 
/// Fail-OPEN — defaults to `true` (TradFi). It returns `false` (Stock) ONLY
/// when the experiment resolves to the explicit `"treatment"` arm. Every other
/// path yields `true`:
/// - `"control"` arm (CN / UAE) → `true`
/// - any other / unknown / empty / invalid variant → `true`
/// - experiment absent, or a deserialization / read error → `true`
/// (the `unwrap_or(true)` below)
/// 
/// Rationale: the experiment is `if (CN || UAE) then control else treatment`,
/// i.e. treatment is the deliberate opt-out to "Stock". TradFi is the intended
/// default copy for this surface, so an unconfigured or unreadable experiment
/// must not silently fall back to "Stock" — only a deliberate `treatment`
/// assignment does.
pub fn bsc_xstocks_tradfi_label_enabled() -> bool { todo!() }

/// Compares two state instances for equality.
pub fn buy_sell_order_detail_state_is_equal(first: BuySellOrderDetailState, second: BuySellOrderDetailState) -> bool { todo!() }

/// UniFFI entry point.
pub fn create_bsc_master_interactor() -> Arc<BscMasterInteractor> { todo!() }

/// Factory function to create the BSC Quick Convert interactor.
pub fn create_bsc_quick_convert_interactor() -> Arc<BSCQuickConvertInteractor> { todo!() }

/// Factory function to create `BuySellInputInteractor`.
pub fn create_buy_sell_input_interactor() -> Arc<BuySellInputInteractor> { todo!() }

/// Factory function to create the Order Detail Interactor.
/// 
/// # Parameters
/// - `dependencies`: Dependencies for the interactor (reserved for future use)
/// 
/// Note: Call `set_parameters()` after creation to set order_id and trade_type.
pub fn create_buy_sell_order_detail_interactor(dependencies: BuySellOrderDetailDependencies) -> Arc<BuySellOrderDetailInteractor> { todo!() }

/// Factory function to create ConvertExecutionPageInteractor.
pub fn create_convert_execution_interactor() -> Arc<ConvertExecutionPageInteractor> { todo!() }

/// Factory function to create ConvertInputInteractor.
pub fn create_convert_input_interactor() -> Arc<ConvertInputInteractor> { todo!() }

/// Creates a new Pro Buy/Sell Order History List Interactor.
/// 
/// No dependencies are required — date formatting is handled on the native side.
pub fn create_pro_buy_sell_order_history_list_interactor() -> Arc<ProBuySellOrderHistoryListInteractor> { todo!() }

/// Factory function to create the recurring buy plan detail interactor.
/// 
/// # Example
/// ```ignore
/// let interactor = create_recurring_buy_plan_detail_interactor();
/// ```
pub fn create_recurring_buy_plan_detail_interactor() -> Arc<RecurringBuyPlanDetailInteractor> { todo!() }

/// Factory function to create the recurring buy plan list interactor.
/// 
/// # Example
/// ```ignore
/// let interactor = create_recurring_buy_plan_list_interactor();
/// ```
pub fn create_recurring_buy_plan_list_interactor() -> Arc<RecurringBuyPlanListInteractor> { todo!() }

/// Factory function to create the recurring buy plan order list interactor.
/// 
/// # Example
/// ```ignore
/// let interactor = create_recurring_buy_plan_order_list_interactor();
/// ```
pub fn create_recurring_buy_plan_order_list_interactor() -> Arc<RecurringBuyPlanOrderListInteractor> { todo!() }

/// Factory function to create the trigger order detail interactor.
pub fn create_trigger_order_detail_interactor() -> Arc<TriggerOrderDetailInteractor> { todo!() }

/// Factory function to create the trigger order list interactor.
pub fn create_trigger_order_list_interactor() -> Arc<TriggerOrderListInteractor> { todo!() }

/// Returns `{card_type: card_hash}` for tools the FE/native should advertise.
/// Excludes tools marked `for_ai_fallback_only` (still in `get_available_tools`).
pub fn get_all_supported_actions() -> HashMap<String, String> { todo!() }

/// Returns the full tool manifest as JSON: `{"tools":[...]}`.
/// 
/// Native callers parse this on the Kotlin/Swift side and forward it to the
/// Lua AgentToolsService for whitelist filtering. Never panics; on serialization
/// failure returns an empty manifest so downstream parsing always succeeds.
pub fn get_available_tools() -> String { todo!() }

/// Returns `{card_type: card_hash}` for stable tools the FE/native should advertise.
/// Excludes tools marked `for_ai_fallback_only` and unstable tools.
pub fn get_stable_supported_actions() -> HashMap<String, String> { todo!() }

/// True iff the Simple Perps trading flow can be entered directly —
/// i.e. tapping the entry should route to the perps pair list (not the
/// qualification survey), and the token detail page should surface the
/// Simple Perps section.
/// 
/// = AB `simple_xperps == "treatment"` AND `Future.Hidden == false`
/// AND `Future.Limit == false`.
/// 
/// Always implies [`simple_perps_is_visible`]; never the other way
/// around. (A `visible && !tradable` user sees the entry but is gated
/// behind the survey.)
pub fn simple_perps_is_tradable() -> bool { todo!() }

/// True iff the Simple Perps entry surfaces (BSC home tile, token detail
/// banner, etc.) should be visible to the current user.
/// 
/// = AB `simple_xperps == "treatment"` AND `Future.Hidden == false`.
/// 
/// Callers gate the *visibility* of the entry on this. Whether tapping
/// the entry should open the trading flow or the qualification survey
/// is a separate question — call [`simple_perps_is_tradable`] to decide.
pub fn simple_perps_is_visible() -> bool { todo!() }

/// Compares two state instances for equality.
pub fn trigger_order_detail_state_is_equal(first: TriggerOrderDetailState, second: TriggerOrderDetailState) -> bool { todo!() }

/// Compares two state instances for equality.
pub fn trigger_order_list_state_is_equal(first: TriggerOrderListState, second: TriggerOrderListState) -> bool { todo!() }
