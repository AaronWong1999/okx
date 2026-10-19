// =============================================================================
// Reconstructed Rust UniFFI IDL & Schema for crate: `risk`
// Extracted from OKX Android v6.194.0 (lib/arm64-v8a/libOKCore.so)
// Total symbols decoded: 131
//   Records: 15 | Enums: 16 | Interfaces: 7
//   Constructors: 0 | Methods: 9 | Functions: 83
// =============================================================================

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

// -----------------------------------------------------------------------------
// 1. Enums (16)
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub enum AppFilterExportError {
    Message,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BiometricMergeExportError {
    Message,
}

/// Error returned by the call-state foreign traits (`CallStateListener`,
/// `CallMonitorController`).
/// 
/// Wraps `UnexpectedUniFFICallbackError` so a Kotlin/Swift implementation that
/// throws an undeclared exception surfaces as a `CallMonitorError` the Rust
/// caller can log-and-drop, rather than an `UnexpectedUniFFICallbackError` that
/// UniFFI turns into a panic — and, under this crate's `panic = "abort"`, a
/// whole-process abort. Mirrors the sibling `IosDetectionError`
/// (`platform/ios_detection_provider.rs`). The single variant carries the
/// underlying reason string for diagnostics.
#[derive(Debug, Clone, PartialEq)]
pub enum CallMonitorError {
    /// The foreign (Kotlin/Swift) side raised an error or panic; `reason` is
    /// the reason string surfaced by UniFFI.
    /// 
    /// The field is named `reason` (not `message`) on purpose: the generated
    /// Kotlin binding renders a `uniffi::Error` variant as an `Exception`
    /// subclass whose fields become constructor `val`s AND drive an
    /// `override val message` getter. A field literally named `message` then
    /// self-collides with `kotlin.Throwable.message` ("'message' hides member
    /// of supertype 'Throwable'"), which fails Kotlin compilation. Swift has no
    /// such member so the iOS binding compiles either way. Keep this field
    /// named `reason` (mirrors the RiskError bindings that use `errorMessage`).
    CallbackFailed {
        reason: String,
    },
}

/// Error returned by the foreign signal collectors.
/// 
/// The conversion is required by UniFFI: without it, an exception escaping a
/// Kotlin/Swift callback with a plain return type is surfaced as a panic.
#[derive(Debug, Clone, PartialEq)]
pub enum CollectorError {
    CallbackFailed {
        reason: String,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum DataStorageExportError {
    Message,
}

/// ata_merge export for current test only; UniFFI requires Result's E to implement uniffi::Error, not raw String.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum DatamergeExportError {
    Message,
}

/// Read errors are exposed as a throwing binding so native wrappers can fail closed.
#[derive(Debug, Clone, PartialEq)]
pub enum DeviceIntegrityReadError {
    Unavailable,
}

/// Result of the full GCC invoke + config decryption flow.
#[derive(Debug, Clone, PartialEq)]
pub enum GccInvokeResult {
    Success,
    GccByteFailure,
    /// Transport-level failure (timeout, connection error, HTTP error code).
    /// Retry was already exhausted by the time this is returned.
    NetworkFailure,
    /// Server returned a successful response (code 0) but with an empty or
    /// absent `et` field. Not a retry candidate — retrying will not change
    /// the server response.
    ServerError,
    DecryptFailure,
}

#[derive(Debug, Clone, PartialEq)]
pub enum KeyExchangeExportError {
    Message,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ModulesCollectionExportError {
    Message,
}

/// Network upload policy — determined by Rust experiment flag, passed to Kotlin.
#[derive(Debug, Clone, PartialEq)]
pub enum NetworkUploadPolicy {
    WifiOnly,
    WifiOrMobile,
}

/// Status of a profile operation.
/// 
/// Maps to `ProfileStatus` in Kotlin.
#[derive(Debug, Clone, PartialEq)]
pub enum ProfileStatus {
    Success,
    Failure,
    Timeout,
    AliTokenFetchFailed,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RiskError {
    NetworkError {
        http_status: i32,
        business_code: i32,
        error_message: String,
        origin_response: Option<Response>,
    },
    Cancelled,
    InternalError {
        error_message: String,
    },
}

/// Source of the risk profiling request.
/// 
/// Maps to `OKRiskLibrarySource` in Kotlin.
#[derive(Debug, Clone, PartialEq)]
pub enum RiskLibrarySource {
    AppLaunch,
    AccountSignup,
    AccountSignin,
    UserLogin,
    PaymentInput,
    AppEnterForeground,
    BscScreen,
    AppInstall,
    StateRefresh,
    KycLiveness,
    KycIdc,
    ExtBoost,
    WalletSend,
    WalletReceive,
    WalletManage,
    Unknown,
    /// MUST remain the LAST variant: UniFFI assigns wire tags by declaration
    /// position, and shipped Kotlin/Swift bindings depend on existing variant
    /// positions staying stable — a middle insertion renumbers them.
    /// Maps to the same dispatch category as AppLaunch (index=0, wire type `env`)
    /// on the Kotlin/native side (OKRiskLibrary `OKRiskPayloadManager.getPayloadEvent`).
    Mfa,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SecTokenExportError {
    Message,
}

/// Typed failure classification carried across the FFI boundary so Rust can apply
/// the tiered backoff (transient 1 day / permanent 7 days). Only meaningful when
/// the raw fetch produced no usable segment; ignored when any segment is present.
#[derive(Debug, Clone, PartialEq)]
pub enum TeeAttestFailure {
    /// No classification supplied (treated as Permanent — conservative).
    Unknown,
    /// Temporary outage (RKP/keymaster/generic exception) — short 1-day backoff.
    Transient,
    /// Structural (device lacks attestation / no extension) — long 7-day backoff.
    Permanent,
}

// -----------------------------------------------------------------------------
// 2. Records / Data Models (15)
// -----------------------------------------------------------------------------

/// Result type for AES operations
#[derive(Debug, Clone, PartialEq)]
pub struct AesEncryptResult {
    pub ciphertext: Vec<u8>,
    pub iv: Vec<u8>,
    pub tag: Vec<u8>,
}

/// Independent local states: -1 = unknown, 0 = no, 1 = yes.
/// A process result is not a substitute for server-side risk decisions.
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceIntegrityStatus {
    pub rooted_or_jailbroken: i32,
    pub developer_mode: i32,
    pub emulator: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModuleInfo {
    pub index: u32,
    pub path: String,
    pub md5: String,
    pub size: u64,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModuleScanResult {
    pub modules: Vec<ModuleInfo>,
    /// None when self_apk_collection disabled (field omitted from report).
    /// Some([]) when enabled but no entries pass the blacklist filter.
    pub self_apk_files: Option<Vec<SelfApkFileInfo>>,
    /// None when anonymous_modules disabled (field omitted from report).
    /// Some("N") with the segment count. ("-1" is only reachable in unwind
    /// (debug/test) builds if counting panics; release uses panic="abort".)
    pub anonymous_count: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModulesCollectionConfig {
    pub enabled: bool,
    pub anonymous_modules: bool,
    pub interval: u64,
    /// Maximum number of flagged modules to collect per scan. 0 = no limit.
    pub max_cnt: u64,
    pub rules: ModulesRules,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModulesRules {
    /// Deny rules evaluated BEFORE the whitelist (R1). Additive + serde(default):
    /// absent/empty behaves exactly as before, so config and client roll back
    /// independently. GCC config COPIES roothide markers here while keeping them
    /// in path_blacklist (copy-not-move keeps lagging clients protected).
    pub path_deny_first: Vec<String>,
    pub path_whitelist: Vec<String>,
    pub path_blacklist: Vec<String>,
    pub symbols_pattern: Vec<String>,
    pub memory_features: Vec<String>,
}

/// Result of a profile operation.
/// 
/// Maps to `ProfileStatusDTO` in Kotlin.
#[derive(Debug, Clone, PartialEq)]
pub struct ProfileStatusDTO {
    /// An ID refreshed every time requesting profile (transaction ID)
    pub risk_id: Option<String>,
    /// A unified token (for KYC, this is ali faceguard id / eDeviceToken)
    pub risk_token: Option<String>,
    /// Status code of the profile operation
    pub code: ProfileStatus,
    /// Ali-specific code (optional)
    pub ali_code: Option<i32>,
}

/// Single app entry that needs to be uploaded to OSS.
#[derive(Debug, Clone, PartialEq)]
pub struct RsdAppSample {
    pub path: String,
    pub md5: String,
    pub index: i32,
}

/// Full result of RSD flow execution.
#[derive(Debug, Clone, PartialEq)]
pub struct RsdFlowResult {
    /// Whether the RSD HTTP request succeeded.
    pub success: bool,
    /// Response et value (encrypted config). Always returned if HTTP succeeded,
    /// regardless of OSS processing result (matches Kotlin try-catch behavior).
    pub et: String,
    /// Parsed OSS config (when all 3 conditions met and parse succeeded), else None.
    pub oss_config: Option<RsdOssConfig>,
    /// Network upload policy (determined by experiment flag), else None.
    pub network_policy: Option<NetworkUploadPolicy>,
    /// Fix A (WMS-239): true when the 24h-gate timestamp save failed during this flow.
    /// Consumed by Kotlin OKAppCollector to emit the `save_scan_timestamp_failed` metric.
    pub gate_ts_save_failed: bool,
}

/// Parsed RSD OSS config — returned to Kotlin for file upload.
#[derive(Debug, Clone, PartialEq)]
pub struct RsdOssConfig {
    pub need_upload_files: Vec<i32>,
    pub oss_config: RsdOssUploadConfig,
    pub app_list: Vec<RsdAppSample>,
}

/// OSS credential and upload target config.
#[derive(Debug, Clone, PartialEq)]
pub struct RsdOssUploadConfig {
    pub access_key_id: String,
    pub access_key_secret: String,
    pub security_token: String,
    pub bucket: String,
    pub endpoint: String,
    pub expiration: i64,
    pub path_prefix: String,
}

/// Encrypted parts of the 3-part `x-sec-token`.
/// 
/// `part2` is the encrypted sec_data (identical producer as [`bridge_produce_token`]);
/// `part3` is the request-bound signature. `part1` (token_info) is assembled on the
/// Kotlin side, so it is intentionally not carried here.
#[derive(Debug, Clone, PartialEq)]
pub struct SecTokenParts {
    pub part2: String,
    pub part3: String,
}

/// Separate GCC config for self-APK entry collection (key: "self_apk_collection").
/// Decoupled from modules_collection so it can be toggled independently.
#[derive(Debug, Clone, PartialEq)]
pub struct SelfApkCollectionConfig {
    pub enabled: bool,
    /// Maximum number of matching APK entries to collect per scan. 0 = no limit.
    pub max_cnt: u64,
    /// Regex patterns matched against APK entry names (e.g. "^classes\\d*\\.dex$").
    /// Only entries whose names match AT LEAST ONE pattern are reported.
    /// Empty list → nothing reported.
    pub path_blacklist: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SelfApkFileInfo {
    pub index: u32,
    pub path: String,
    pub size: u64,
    pub crc32: u32,
    pub ts: u64,
}

/// Raw AndroidKeyStore attestation values (batch-2). Each field is empty for a
/// missing slot; the Rust side owns the CSV assembly, cache, backoff, wait-cap,
/// single-flight and stale-if-error (this is the raw fetch only).
/// 
/// Field names avoid `message` (which would collide with `kotlin.Throwable.message`
/// in the generated binding); these are plain data fields on a `uniffi::Record`.
/// 
/// The 5th field `failure_kind` is a typed failure classification read by Rust
/// ONLY on the all-empty (no usable segment) path to pick the backoff tier;
/// success rows leave it `Unknown`.
#[derive(Debug, Clone, PartialEq)]
pub struct TeeAttestRaw {
    /// Full-length lowercase hex digest of the boot-image verification public key.
    pub boot_key_hex: String,
    /// Security level: "0"=Software / "1"=TEE / "2"=StrongBox (as a string slot).
    pub security_level: String,
    /// Device locked: "1"=locked / "0"=unlocked.
    pub device_locked: String,
    /// Verified boot state: "0"=Verified / "1"=SelfSigned / "2"=Unverified / "3"=Failed.
    pub verified_boot_state: String,
    /// Failure classification when the fetch produced no usable segment (empty
    /// otherwise-populated slots). Kotlin sets this; Rust reads it only on the
    /// all-empty path. Success rows leave it `Unknown`.
    pub failure_kind: TeeAttestFailure,
}

// -----------------------------------------------------------------------------
// 3. Interfaces / Interactors / Services (7)
// -----------------------------------------------------------------------------

/// Platform monitor lifecycle, implemented by the Android/iOS side. `start()`
/// registers the system audio/telephony monitor; `stop()` unregisters it. The
/// registry drives these on the 0<->1 subscriber transitions.
/// 
/// Returns `Result` for the same reason as `CallStateListener`: a foreign impl
/// throwing (e.g. Android `PhoneStateListener` registration without
/// `READ_PHONE_STATE` → `SecurityException`) must not abort the host process.
/// The add/remove-listener call sites downgrade `Err` to a warn log.
pub struct CallMonitorController;
impl CallMonitorController {
    pub fn start(&self) -> Result<(), CallMonitorError> { todo!() }
    pub fn stop(&self) -> Result<(), CallMonitorError> { todo!() }
}

/// Consumer-implemented push callback. Rust fans out call-state changes to
/// every registered listener (after dedup).
/// 
/// Returns `Result` so a foreign impl throwing an undeclared exception surfaces
/// as `CallMonitorError` (not a `panic = "abort"` process abort); the fan-out
/// downgrades `Err` to a warn log and keeps notifying the other listeners.
pub struct CallStateListener;
impl CallStateListener {
    pub fn on_call_state_changed(&self, state: i32) -> Result<(), CallMonitorError> { todo!() }
}

/// Foreign trait implemented by OKRisk: collect the Android Camera2 P0 static
/// capability snapshot without CAMERA permission (API 29+), normalize it to the
/// four-state (value / empty / `NA` / `ERR`) model, derive the two white-balance
/// positions, and serialize the fixed 7/3/8/9 compact wire form. `collect`
/// returns the FINAL raw wire string — one of three legal forms:
/// * a single-line compact JSON `{"camera_list":[...]}` (API 29+ success,
/// including the empty `{"camera_list":[]}`),
/// * the literal `NA` (API 28 and below — the permission-free scheme is not
/// supported on that platform), or
/// * the literal `ERR` (camera-id list read / metadata / serialization / self
/// -check failure, or the field-size budget was exceeded).
/// 
/// The Kotlin side owns all collection, normalization, derivation and compact
/// serialization; Rust only schedules `(8,58)` and carries the string. The
/// returned value MUST NOT carry the report magic — the central Rust `Str`
/// encoding path (`run_detection`) appends the configured magic exactly once, so
/// appending it here would double it. Kotlin converts expected platform failures
/// into the `NA` / `ERR` sentinels itself; the `Result` additionally protects the
/// generated callback boundary if an unexpected exception escapes that
/// implementation (mapped to the field's `ERR` default by the caller).
pub struct Camera2CompactCollector;
impl Camera2CompactCollector {
    pub fn collect(&self) -> Result<String, CollectorError> { todo!() }
}

/// **Deprecated, retired (round-7 finding #24/#27).** This is the ORIGINAL
/// v1 collector trait that used to back hook sn=24 (`ClassLoaderChainCollectorImpl`
/// on the Kotlin side, since deleted). It has no relationship to sn=24's
/// current, real dispatch — see `ClassLoaderChainV2Collector` below and
/// `hook_detection/android.rs::collect_cl_loaders_v2` for that. Nothing in
/// this crate ever reads a collector registered here.
/// 
/// This trait and its registration function are kept ONLY so a stale OKRisk
/// binary — built against an older OKCore that still calls
/// `riskRegisterClassloaderCollector` — doesn't fail to link against a newer
/// OKCore artifact (partial rollback, stale dependency cache, a hotfix
/// branched before the retirement). This follows the same
/// deprecate-don't-remove policy already applied to the sn=24 wire field
/// itself; see `docs/best-practice-in-component-development.md` §1.
pub struct ClassLoaderChainCollector;
impl ClassLoaderChainCollector {
    pub fn collect(&self) -> Result<Vec<String>, CollectorError> { todo!() }
}

/// Foreign trait implemented by OKRisk: assemble the cl_loaders_v2 field (Java
/// parent-chain P, plus Stage-B ART strong-root S when supported) into the
/// final `#`-joined string. Rust does not walk anything — Kotlin owns the whole
/// business shape (P/S merge, caps, format). Empty string signals collection
/// failure or no-collector, exactly mirroring `TeeAttestCollector`.
pub struct ClassLoaderChainV2Collector;
impl ClassLoaderChainV2Collector {
    pub fn collect(&self) -> Result<String, CollectorError> { todo!() }
}

/// Foreign Risk Manager Interface (implemented by Kotlin/Swift).
/// 
/// This is the UniFFI bridge - Kotlin/Swift implement this trait.
/// The Android/iOS RiskManager implementations wrap this.
pub struct ForeignRiskManager;
impl ForeignRiskManager {
    /// Initializes the risk module.
    pub fn initialize(&self) { todo!() }
    /// Opens the risk profile.
    pub async fn profile(&self, source: RiskLibrarySource, risk_id: Option<String>) -> ProfileStatusDTO { todo!() }
}

/// Foreign trait implemented by OKRisk: perform a hardware AndroidKeyStore
/// key-attestation and parse the cert extension into the 4 raw values (empty per
/// missing slot). Kotlin converts expected platform failures into a classified
/// all-empty row; the `Result` protects the generated callback boundary if an
/// unexpected exception escapes Kotlin.
pub struct TeeAttestCollector;
impl TeeAttestCollector {
    pub fn collect(&self) -> Result<TeeAttestRaw, CollectorError> { todo!() }
}

// -----------------------------------------------------------------------------
// 4. Top-Level Exported Functions (83)
// -----------------------------------------------------------------------------

pub fn aes_whitebox_encrypt_gcm_uniffi(tables_bytes: Vec<u8>, plaintext: Vec<u8>, timestamp: String) -> Result<AesEncryptResult, AesError> { todo!() }

pub fn aes_whitebox_encrypt_gcm_with_iv_uniffi(tables_bytes: Vec<u8>, plaintext: Vec<u8>, timestamp: String, iv: Vec<u8>) -> Result<AesEncryptResult, AesError> { todo!() }

/// Build {"app_list": [{"index", "path", "md5"}, ...]} from items JSON array. For getPathsAndMd5JNI.
pub fn app_filter_build_paths_and_md5_json(items_json: String) -> Result<String, AppFilterExportError> { todo!() }

/// Filter app list by app_list_collection rules. For processAppDataJNI filter step.
pub fn app_filter_filter_malicious_apps(app_list_json: String, collection_config_json: String, last_scan_timestamp: String, full_scan: bool) -> Result<String, AppFilterExportError> { todo!() }

/// Complete collect + filter + encrypt flow matching C++ collectFilterAndProcessJNI.
/// Uses AppIteratorBridge (JNI) to stream app collection, filters in chunks, encrypts result.
/// Optionally merges serialized ModuleScanResult JSON into the payload before encryption.
/// 
/// @param client_id Device identifier
/// @param timestamp Current processing timestamp (millis)
/// @param type_ Data type
/// @param version Version string
/// @param sdk_version SDK version
/// @param ts Raw data bytes for encryption
/// @param chunk_size Apps per chunk before filtering (default 32)
/// @param module_json Serialized ModuleScanResult JSON — merged alongside app_list before encryption.
/// Pass empty string when module collection is disabled or not yet run.
pub fn bridge_collect_filter_and_process(client_id: String, timestamp: String, type_: i32, version: String, sdk_version: String, ts: Vec<u8>, chunk_size: i32, module_json: String) -> Option<String> { todo!() }

/// Complete decryptConfig flow: read file + decrypt + save to cache
/// Matches C++ decryptConfigJNI
/// 
/// @param files_dir Android app's internal files directory path (e.g. "/data/data/com.app/files")
/// @param client_id Device identifier
/// @param encrypted_config Encrypted config from backend (empty means read from file)
/// @param source_type Config source type ("gcc" or "rsd")
/// Matches C++ decryptConfigJNI: decrypt config, cache in memory, save gcc to disk.
pub fn bridge_decrypt_config(files_dir: String, client_id: String, encrypted_config: String, source_type: String) -> bool { todo!() }

pub fn bridge_execute_app_collection_rsd(transaction_id: String) -> RsdFlowResult { todo!() }

/// Full biometric / Lua report + RSD orchestration (Phase 2).
/// 
/// Replaces the Kotlin dispatchBiometric → processBiometricData → rsdInvoke chain.
/// Internally: merge biometric input → encrypt → POST /priapi/v1/dis/rsd → parse OSS config.
/// 
/// # Parameters
/// - `sensor_data`: Serialized biometric sensor payload.
/// - `touch_data`: Optional touch event data.
/// - `keyboard_data`: Optional keyboard event data.
/// - `transaction_id`: Unique transaction identifier for the RSD request.
/// - `scene`: Risk scene identifier forwarded to the RSD endpoint.
/// - `type_`: `1` = standard biometric (sensor merge applied), `3` = Lua report (raw JSON, no merge).
/// 
/// # Returns
/// `RsdFlowResult` with `success = true` and populated `et`/`oss_config`/`network_policy`
/// on HTTP success; `success = false` with empty fields if platform state is missing or
/// the request fails.
pub fn bridge_execute_biometric_rsd(sensor_data: String, touch_data: Option<String>, keyboard_data: Option<String>, transaction_id: String, scene: String, type_: i32) -> RsdFlowResult { todo!() }

/// Executes the full GCC invoke + config decryption flow.
/// 
/// Steps:
/// 1. Read config version from disk (bridge_get_config_value)
/// 2. Generate GCC byte payload (bridge_gcc_byte) — caches ChaCha20 key for step 4
/// 3. POST /priapi/v1/dis/gcc via OKXHttpClient
/// → NativeHttpEngine (Android): OkHttp, visible in Charles, Android TLS trust store
/// 4. Decrypt config (bridge_decrypt_config) — uses ChaCha20 key from step 2
/// 
/// Steps 2 and 4 MUST run in the same process in this order (ChaCha20 key cached in memory).
/// 
/// risk_session_id / risk_session_status are the tmx session values from OKRiskProvider —
/// passed directly to avoid a global write/read race under concurrent invocations.
/// This function blocks the calling thread; invoke from Dispatchers.IO.
pub fn bridge_execute_gcc_invoke_with_decryption(files_dir: String, client_id: String, timestamp: String, session_id: String, sv: String, pv: i32, rid: String, ts: Vec<u8>, risk_session_id: String, risk_session_status: i32, max_retry: u32) -> GccInvokeResult { todo!() }

/// Encrypt a pre-collected module JSON payload and POST it to /priapi/v1/dis/rsd.
/// 
/// This is the correct entry point for periodic module collection.  It does NOT
/// run app scanning (no collect_and_filter call) — that separates module collection
/// from the app-launch app-scan that lives in bridge_execute_app_collection_rsd.
/// 
/// Call hierarchy:
/// Android: OKRiskManager.scheduleModulesCollection → here (replaces NativeBridge.collectFilterAndProcess)
/// iOS:     OKModulesCollector.collect              → here (replaces bridgeCollectFilterAndProcess)
pub fn bridge_execute_modules_rsd(module_json: String, transaction_id: String) -> RsdFlowResult { todo!() }

/// Executes the full RSD flow in Rust.
/// 
/// Steps:
/// 1. Build request body (CommonBody with pm=1, matches Kotlin)
/// 2. POST /priapi/v1/dis/rsd via OKXHttpClient with exponential backoff retry
/// 3. On success, if scene=="APP_START" && isFileUploadEnabled():
/// a. Decrypt RSD config via bridge_get_config_value_rsd
/// b. Parse OSS config via parse_rsd_oss_config
/// c. Determine network policy via isFileUploadEnforced experiment flag
/// 4. Return RsdFlowResult { success, et, oss_config?, network_policy? }
/// 
/// OSS processing errors do NOT affect the et return value (matches Kotlin try-catch).
/// 
/// risk_session_id / session_status are the tmx session values from OKRiskProvider —
/// passed directly to avoid a global write/read race under concurrent invocations.
/// This function blocks the calling thread; invoke from Dispatchers.IO.
pub fn bridge_execute_rsd_flow(timestamp: String, session_id: String, encrypted_payload: String, pv: i32, rid: String, sv: String, scene: String, max_retry: u32, risk_session_id: String, session_status: i32) -> RsdFlowResult { todo!() }

/// Complete gccByte flow: decode raw_data + build JSON + encrypt + Base64
/// Matches C++ gccByteNative
pub fn bridge_gcc_byte(client_id: String, timestamp: String, type_: i32, version: String, sdk_version: String, ts: Vec<u8>) -> Result<String, KeyExchangeExportError> { todo!() }

/// Complete getConfigValue flow: read file + decrypt + query key path
/// Matches C++ getConfigValueJNI
/// 
/// @param files_dir Android app's internal files directory path
/// @param client_id Device identifier
/// @param key_path JSON key path to query (e.g. "app_list_collection")
pub fn bridge_get_config_value(files_dir: String, client_id: String, key_path: String) -> Option<String> { todo!() }

/// Complete getConfigValueRsd flow: decrypt + extract needUploadFiles + query Rust cache for path/md5.
/// Matches C++ getConfigValueRsdJNI: decrypts config, reads needUploadFiles indices,
/// fetches path+md5 from Rust-side cache (populated during filterMaliciousApps), builds result.
pub fn bridge_get_config_value_rsd(encrypted_config: String, _paths_and_md5_json: Option<String>) -> Option<String> { todo!() }

/// Complete processAppData flow: read file + decrypt config + filter + encrypt
/// Matches C++ processAppDataJNI
pub fn bridge_process_app_data(files_dir: String, client_id: String, app_data: String, timestamp: String, type_: i32, version: String, sdk_version: String, ts: Vec<u8>, last_scan_timestamp: String, full_scan: bool) -> Option<String> { todo!() }

/// Complete processBiometricData flow: merge + decode + encrypt + Base64
/// Matches C++ processBiometricDataJNI
pub fn bridge_process_biometric_data(sensor_data: String, client_id: String, timestamp: String, type_: i32, version: String, sdk_version: String, ts: Vec<u8>) -> Result<String, KeyExchangeExportError> { todo!() }

/// Complete produceToken flow: decode raw_data + generate sec token
/// Matches C++ produceSecJNI (part2 only)
pub fn bridge_produce_token(other_data: String, timestamp: String, ts: Vec<u8>, ali_token_threshold: i32, ali_device_token: String, session_id: String) -> Result<String, SecTokenExportError> { todo!() }

/// Complete signed produceToken flow: `part2` via the existing `generate_sec_token`
/// path + `part3` via `generate_request_sign`.
/// 
/// Additive sibling of [`bridge_produce_token`] (which stays 2-part and byte-for-byte
/// unchanged). `part3` failures degrade to an empty string (`unwrap_or_default`), and the
/// mandatory 2-part fallback is enforced by the Kotlin assembler. Returns `None` only when
/// raw_data decode or `part2` generation fails — Kotlin then degrades to the plain path.
pub fn bridge_produce_token_signed(other_data: String, timestamp: String, ts: Vec<u8>, ali_token_threshold: i32, ali_device_token: String, session_id: String, method: String, url: String, query_params: String, body_params: String) -> Option<SecTokenParts> { todo!() }

/// Complete uploadDataMessage flow: plaintext + encrypt + Base64
/// Matches C++ uploadDataMessageNative
pub fn bridge_upload_data_message(user_id: String, client_id: String, channel: String, device_info: String, dex_id: i32, cg: Vec<u8>, index: i32, timestamp: String, ts: Vec<u8>) -> Result<String, DatamergeExportError> { todo!() }

/// Complete uploadDataMessageMetrics flow: plaintext + encrypt + Base64
/// Matches C++ uploadDataMessageMetricsNative
pub fn bridge_upload_data_message_metrics(user_id: String, client_id: String, channel: String, device_info: String, dex_id: i32, cg: Vec<u8>, index: i32, timestamp: String, ts: Vec<u8>, health_data: Vec<u8>, health_array_cnt: i32) -> Result<String, DatamergeExportError> { todo!() }

/// Stage 5: Orchestration for collectFilterAndProcess / processAppData — filter + raw_data decode + encrypt.
/// Caller (Kotlin) is responsible for timestamp read/write (e.g. readAppDataTimestamp / saveAppDataTimestamp).
pub fn collect_filter_and_encrypt_app_data(client_id: String, timestamp: String, type_: i32, version: String, sdk_version: String, message_id: String, app_data_json: String, collection_config_json: String, last_scan_timestamp: String, full_scan: bool, ts_bytes: Vec<u8>) -> Result<String, KeyExchangeExportError> { todo!() }

/// Competitor memScannedDylibs: memory-scanned dylib enumeration (VM scan + Mach-O LC_ID_DYLIB, bypasses dyld hook).
/// Returns "|"-joined install_name list (iOS-only; other platforms return empty string).
pub fn collect_mem_scanned_dylib_list() -> String { todo!() }

pub fn collect_modules(self_apk_path: String) -> ModuleScanResult { todo!() }

/// Build RSD result JSON from decrypted config + optional app_list. For getConfigValueRsd dual-path.
pub fn configuration_build_rsd_result_from_json(decrypted_json: String, app_list_json: Option<String>) -> Option<String> { todo!() }

/// Decrypt Base64-encoded config with cached ChaCha20 key. For getConfigValueRsd dual-path.
pub fn configuration_decrypt_config(encrypted_b64: String) -> Option<String> { todo!() }

/// Query JSON by dot-separated key path (e.g. "version", "ossConfig.bucket"). For getConfigValueJNI after decrypt.
pub fn configuration_query_json_key_path(json_str: String, key_path: String) -> Option<String> { todo!() }

/// Query shallPull from cached RSD config. Call set_cached_rsd_config when RSD is decrypted. For checkShallPullJNI migration.
pub fn configuration_query_shall_pull() -> bool { todo!() }

/// Build encrypted data message v2: plaintext generation + AES-GCM encryption + Base64 encoding.
/// Matches C++ uploadDataMessageNative complete flow. Returns Base64-encoded encrypted string.
pub fn data_merge_get_encrypted_message_v2(user_id: String, client_id: String, channel: String, payload: String, dex_id: i32, cg_bytes: Vec<u8>, index: i32, random_b0: u8, timestamp: String, raw_data: Vec<u8>) -> Result<String, DatamergeExportError> { todo!() }

/// Build encrypted data message v3 (with health): plaintext + AES-GCM + Base64.
/// Matches C++ uploadDataMessageMetricsNative complete flow.
pub fn data_merge_get_encrypted_message_v3(user_id: String, client_id: String, channel: String, payload: String, dex_id: i32, cg_bytes: Vec<u8>, index: i32, random_b0: u8, timestamp: String, raw_data: Vec<u8>, health_data: Vec<u8>, health_array_cnt: i32) -> Result<String, DatamergeExportError> { todo!() }

/// Build health message byte v2 (body2 only). Params: cg_bytes + index (decode config), random_b0.
pub fn data_merge_get_health_message_byte_v2(cg_bytes: Vec<u8>, index: i32, random_b0: u8) -> Result<Vec<u8>, DatamergeExportError> { todo!() }

/// Build data message byte v2 (plain, no AES). Params match JNI uploadDataMessageNative.
/// Returns raw bytes; caller may encrypt and encode.
pub fn data_merge_get_message_byte_v2(user_id: String, client_id: String, channel: String, payload: String, dex_id: i32, cg_bytes: Vec<u8>, index: i32, random_b0: u8) -> Result<Vec<u8>, DatamergeExportError> { todo!() }

/// Build data message byte v3 (with health). Params match JNI uploadDataMessageMetricsNative.
pub fn data_merge_get_message_byte_v3(user_id: String, client_id: String, channel: String, payload: String, dex_id: i32, cg_bytes: Vec<u8>, index: i32, random_b0: u8, health_data: Vec<u8>, health_array_cnt: i32) -> Result<Vec<u8>, DatamergeExportError> { todo!() }

/// Decrypt stored config from JSON file content. Caller reads file and passes content.
pub fn data_storage_decrypt_from_storage(stored_json: String, client_id: String) -> Result<String, DataStorageExportError> { todo!() }

/// Encrypt plaintext and return JSON string to save to file. Caller writes to storage.
pub fn data_storage_encrypt_for_storage(plaintext: String, client_id: String) -> Result<String, DataStorageExportError> { todo!() }

/// Generate sec token part2: build JSON from internal caches, zlib compress, AES whitebox GCM, Base64.
/// Directly reads cached detection results from internal storage - no need to pass them as parameters.
pub fn generate_sec_token(other_data_json: String, timestamp: String, raw_data: Vec<u8>, ali_token_threshold: i32, ali_device_token: String, session_id: String, message_id: String) -> Result<String, SecTokenExportError> { todo!() }

/// Get cached device info JSON (for SecToken generation).
/// Returns JSON object string with keys: m2, m1, d1, m3, d0, d3, etc., or "{}" if empty.
/// Matches C++ get_device_info_cache().
pub fn get_cached_device_info() -> String { todo!() }

/// Get cached emulator detection result (for SecToken generation).
/// Returns raw value where bit N = case N detected emulator.
/// Matches C++ EmulatorDetection::get_cached_is_emulator().
pub fn get_cached_emulator() -> i32 { todo!() }

/// Get cached GPS spoofing apps JSON (for SecToken generation).
/// Returns JSON array string like ["com.app1", "com.app2"] or "[]" if empty.
/// Matches C++ AppFilter::getCachedGpsSpoofingApps().
pub fn get_cached_gps_spoofing_apps() -> String { todo!() }

/// Get cached root detection result (for SecToken generation).
/// Returns raw value where bit N = case N detected root.
/// Matches C++ RootDetection::get_cached_is_root().
pub fn get_cached_root() -> i32 { todo!() }

/// Read process-local states without detection, I/O, retries or logging.
pub fn get_device_integrity_status() -> Result<DeviceIntegrityStatus, DeviceIntegrityReadError> { todo!() }

pub fn get_modules_by_indices(indices: Vec<i32>) -> String { todo!() }

pub fn get_modules_collection_config() -> ModulesCollectionConfig { todo!() }

pub fn get_self_apk_collection_config() -> SelfApkCollectionConfig { todo!() }

pub fn get_upload_files_by_indices(indices: Vec<i32>) -> String { todo!() }

/// Stage 5: Set charging power override (mirrors C++ nativeHandleChargingPower / write_charging_power).
/// DeviceinfoDataDetection.detect_udg_power() returns this value when set.
pub fn handle_charging_power(value: f32) { todo!() }

/// FFI-friendly wrapper for `fg_v2_field_allowed`. Kotlin/Swift use this to consult
/// the gate without re-parsing the spec on their side. detector_id is passed as i32
/// (instead of u8) to keep bindings simple; values outside [0, 255] always return false.
pub fn is_fg_v2_field_allowed(detector_id: i32, sn: i32) -> bool { todo!() }

/// Biometric/app encryption: package JSON, gzip compress, AES whitebox, Base64. Params match get_encryption_byte.
pub fn key_exchange_get_encryption_byte(type_: i32, client_id: String, version: String, sdk_version: String, message_id: String, data_str: String, timestamp: String, raw_data: Vec<u8>) -> Result<String, KeyExchangeExportError> { todo!() }

/// GCC key exchange: build JSON (key hex from cache), AES whitebox encrypt, Base64. Params match gccByteFromJNI.
pub fn key_exchange_get_gcc_byte(client_id: String, timestamp: String, type_: i32, version: String, sdk_version: String, message_id: String, raw_data: Vec<u8>) -> Result<String, KeyExchangeExportError> { todo!() }

/// Merge biometric sensor JSON to compact format (C++ mergeBiometricData). For processBiometricData dual-path.
pub fn merge_biometric_data(sensor_data_json: String) -> Result<String, BiometricMergeExportError> { todo!() }

/// Clear cached message_id (for new session or testing).
pub fn message_id_clear_cache() { todo!() }

/// Get or generate cached message_id (24-char hex). Matches C++ MessageIdManager.
pub fn message_id_get_or_generate() -> String { todo!() }

/// Called from Kotlin via UniFFI to report screenshot detection status.
/// Kotlin calls: `uniffi.risk.notifyScreenshotStatus(status)`
/// 
/// Status codes:
/// -  1: Screenshot detected this cycle
/// - -1: Observer active, no screenshot
/// - -2: Detection unavailable (API < 24 or API >= 33)
/// - -3: Permission denied (READ_EXTERNAL_STORAGE not granted)
pub fn notify_screenshot_status(status: i32) { todo!() }

/// Decode ts byte array to raw_data (XOR inverse + gzip + CRC16). For GCC/encryption whitebox tables.
pub fn raw_data_decode(ts: Vec<u8>) -> Result<Vec<u8>, KeyExchangeExportError> { todo!() }

/// Record already-collected platform conclusions in the process-local store.
/// This is an internal SDK bridge: it validates tri-states, never invokes detection,
/// never persists data, and each dimension retains its first valid result.
pub fn record_collected_device_integrity_status(rooted_or_jailbroken: i32, developer_mode: i32, emulator: i32) { todo!() }

/// Push subscribe: register a listener, returns a unique token. The 0->1
/// transition starts the platform monitor via the registered controller.
pub fn risk_add_call_state_listener(listener: Arc<dyn CallStateListener>) -> u64 { todo!() }

/// Get file stat info via raw syscall (fstatat). For UdRisk.stat() Lua bridge.
/// Returns JSON string with file metadata or error details.
pub fn risk_file_stat(path: String) -> String { todo!() }

/// Pull: read the current device call state (0 none / 1 ringing / 2 carrier /
/// 3 VoIP / -1 unknown). Synchronous; switch-independent; zero permission.
/// 
/// Side effect: asynchronously reports {call_state, timestamp} to the backend
/// over the RSD channel (type=4, fire-and-forget, rate-limited). This report is
/// hooked at the export layer — SDK-internal reads of the internal
/// `get_current_call_state()` deliberately do NOT trigger a report.
pub fn risk_get_current_call_state() -> i32 { todo!() }

/// Initializes the risk module.
/// 
/// Automatically uses the correct platform implementation.
/// 
/// Spawns the initialize work on the global Tokio runtime and returns immediately,
/// so this function is safe to call from the main thread.
/// 
/// # Example
/// ```rust,ignore
/// risk_manager::initialize();
/// ```
pub fn risk_init() { todo!() }

/// Platform monitor -> Rust: report a call-state change. Rust dedups a repeat of
/// the last state, then fans out to every subscriber.
pub fn risk_notify_call_state_changed(state: i32) { todo!() }

/// Rising-edge hook (non-call -> call): increments the per-run call count.
/// Called by the Kotlin/Swift `CallStateMonitor` state machine.
pub fn risk_on_call_started() { todo!() }

/// Opens the risk profile.
/// 
/// Automatically uses the correct platform implementation.
/// 
/// # Example
/// ```rust,ignore
/// let result = risk_manager::profile(RiskLibrarySource::UserLogin, None).await;
/// ```
pub async fn risk_profile(cancel_token: Arc<OKCancellationToken>, source: RiskLibrarySource, risk_id: Option<String>) -> Result<Option<ProfileStatusDTO>, RiskError> { todo!() }

/// Register the process-global platform monitor controller (once, at init).
pub fn risk_register_call_monitor_controller(controller: Arc<dyn CallMonitorController>) { todo!() }

/// Register the Camera2 compact-capability collector (once, at OKRisk init). A
/// duplicate registration is ignored with a warn — mirrors the sibling
/// `risk_register_tee_attest_collector` register-once discipline (the already
/// registered instance is never replaced, so a second init cannot make the
/// snapshot drift).
pub fn risk_register_camera2_compact_collector(collector: Arc<dyn Camera2CompactCollector>) { todo!() }

/// **Deprecated, retired.** Registering a collector here has no effect on
/// anything — no code path in this crate ever consults it, including sn=24's
/// real dispatch (which goes through `ClassLoaderChainV2Collector` instead) —
/// this function exists only to keep the UniFFI symbol linkable for a stale
/// caller. New code must not call this.
pub fn risk_register_classloader_collector(collector: Arc<dyn ClassLoaderChainCollector>) { todo!() }

/// Register the cl_loaders_v2 collector (once, at OKRisk init). A duplicate
/// registration is ignored with a warn — mirrors `risk_register_tee_attest_collector`.
pub fn risk_register_classloader_v2_collector(collector: Arc<dyn ClassLoaderChainV2Collector>) { todo!() }

/// Registers the foreign (Kotlin/Swift) implementation.
/// 
/// Called once during app startup.
pub fn risk_register_manager(manager: Arc<dyn ForeignRiskManager>) { todo!() }

/// Register the TEE-attestation collector (once, at OKRisk init).
pub fn risk_register_tee_attest_collector(collector: Arc<dyn TeeAttestCollector>) { todo!() }

/// Push unsubscribe by token. The 1->0 transition stops the platform monitor.
pub fn risk_remove_call_state_listener(token: u64) { todo!() }

/// Set a risk-specific ABTest flag value, bypassing the global ABTest factory.
/// 
/// Called by iOS/Android native at startup with values read directly from Skylab.
/// Needed on iOS because OKLauncher's `OKCoreABTestBridge` occupies the global
/// ABTest factory (experiment crate `OnceLock`) before OKRiskLibrary initialises,
/// and that factory may not return values for risk-specific flags.
/// 
/// `RsdExperimentFlags` checks these overrides before falling back to the global factory.
pub fn risk_set_abtest_value(key: String, value: String) { todo!() }

/// Close a ZIP handle and free all resources.
pub fn risk_zip_close(handle: i64) { todo!() }

/// Atomic scan: check if a ZIP/APK contains any of the target filenames.
/// Returns JSON array of matched names, or error JSON.
pub fn risk_zip_contains(zip_path: String, targets_json: String) -> String { todo!() }

/// Get next entry from an open ZIP handle. Returns JSON `{"name","size","crc","is_dir"}`
/// or empty string when exhausted.
pub fn risk_zip_enum(handle: i64) -> String { todo!() }

/// Open a ZIP/APK file for entry enumeration. Returns handle >= 0, or -1 on error.
/// 
/// **Callers must call `risk_zip_close` on the returned handle when done.** Forgetting
/// to close leaks an OS file descriptor. The handle table is capped at 16 concurrent
/// opens; archives exceeding 100 000 entries are also refused (-1).
pub fn risk_zip_open(path: String) -> i64 { todo!() }

/// Register platform params for Rust RSD flow (called once at app start).
/// `base_url` is accepted for API compatibility but not stored — OKXHttpClient handles base URL.
pub fn rsd_register_platform_params(files_dir: String, base_url: String, device_id: String, tmx_session_id: String, tmx_session_status: i32) { todo!() }

/// Update crypto params (called after DID request returns with fresh pv/rid/ts).
pub fn rsd_update_crypto_params(pv: i32, rid: String, ts: Vec<u8>, sv: String, max_retry: u32) { todo!() }

/// Set cached device info JSON.
/// Called when device info is collected to store for SecToken generation.
/// Matches C++ set_device_info_cache().
pub fn set_cached_device_info(device_info_json: String) { todo!() }

/// Set cached emulator detection result.
/// Called after emulator detection runs to store result for SecToken generation.
/// Matches C++ EmulatorDetection::set_cached_is_emulator().
pub fn set_cached_emulator(value: i32) { todo!() }

/// Set cached GPS spoofing apps JSON.
/// Called after app filtering to store risk apps for SecToken generation.
/// Matches C++ AppFilter::setCachedGpsSpoofingApps().
pub fn set_cached_gps_spoofing_apps(apps_json: String) { todo!() }

/// Set cached root detection result.
/// Called after root detection runs to store result for SecToken generation.
/// Matches C++ RootDetection::set_cached_is_root().
pub fn set_cached_root(value: i32) { todo!() }

/// Set cached decrypted RSD config so checkShallPull can read it. Call after decrypting RSD.
pub fn set_cached_rsd_config(decrypted_rsd_json: String) { todo!() }

/// Called from Kotlin via UniFFI to update the allow-list spec.
/// Kotlin calls: `uniffi.risk.setFgV2AllowList(spec)`
pub fn set_fg_v2_allow_list(spec: String) { todo!() }

/// Set that target app info API has been successfully called. For markTargetAppInfoApiSuccessJNI migration.
pub fn set_target_app_info_api_success() { todo!() }

/// Start the long-running sensitive-file-access monitor daemon.
/// Idempotent — safe to call more than once; only the first call spawns a thread.
pub fn start_sensitive_file_monitor() { todo!() }
