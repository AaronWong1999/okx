# OKX Strategy Bot 100% UI Specification & Pixel-Accurate Reproduction Guide

## 1. OKX Design System Tokens (`OKDesign` v6.194.0)

Extracted from `resources.arsc` (`color`, `dimen`, `style` tables) and the UniFFI presentation records (`SpotGridOverviewTagPresentation`, `TwapDetailHeaderPresentationV1`, `SpotGridConfirmationRowStyle`) in [`trading_bot_full_schema.rs`](file:///Users/aaron/Desktop/aaron/okx/reconstructed/rust_okcore_idl/trading_bot_full_schema.rs):

### 1.1 Color Palette (Dark Theme Default — OKX Pro Mode)
| Token Name | Hex Value | Usage in Strategy Bot UI |
| :--- | :--- | :--- |
| `--okx-bg-primary` | `#000000` / `#0B0E11` | Main screen background, K-line canvas background |
| `--okx-bg-elevated` | `#121212` / `#161A1E` | Cards, Bot Marketplace items, Bottom Sheet container |
| `--okx-bg-input` | `#1E2026` / `#1F2229` | Parameter input boxes (`min_px`, `max_px`, `grid_num`, `sz`), segmented control track |
| `--okx-bg-input-focus` | `#262932` | Focused input background with `1px solid #FFFFFF` border |
| `--okx-text-primary` | `#FFFFFF` / `#F0F2F5` | Primary labels, prices, input values, active tab titles |
| `--okx-text-secondary` | `#8E929B` / `#909399` | Field labels, secondary metrics (`Per grid profit`, `Available`) |
| `--okx-text-tertiary` | `#5C6068` | Placeholders, disabled text, muted voucher tags (`muted: true`) |
| `--okx-up-green` | `#00B865` / `#25C26E` | Long (`做多`), Buy grid lines, Positive PnL / APY, Primary Create Long CTA |
| `--okx-up-green-bg` | `rgba(0, 184, 101, 0.14)` | Long tag chip background, positive badge background |
| `--okx-down-red` | `#F5455C` / `#F6465D` | Short (`做空`), Sell grid lines, Negative PnL, Liquidation warning, Stop-loss line |
| `--okx-down-red-bg` | `rgba(245, 69, 92, 0.14)` | Short tag chip background, error banner background |
| `--okx-accent-lime` | `#BCFF2F` | OKX brand highlight, AI Strategy badge, selected primary pill accent |
| `--okx-warning-amber` | `#FFB800` / `#F7931A` | Voucher / Coupon chip (`SpotGridOverviewTagTone::Voucher`), Margin risk warning |
| `--okx-neutral-blue` | `#3B82F6` | Neutral (`中性`) direction chip, DCA Average Cost line on K-line |
| `--okx-border-subtle` | `#22252C` | Hairline dividers (`1px`), card outlines, table row borders |

### 1.2 Typography & Numeric Formatting Rules
- **Font Family**: `HarmonyOS Sans` / `Inter` / `-apple-system, BlinkMacSystemFont, "SF Pro Display"` with `font-variant-numeric: tabular-nums` mandatory on all prices, PnL, and orderbook numbers.
- **Number Formatting Invariants (Verified from `trading_bot_full_schema.rs` docstrings)**:
  1. **Ratio Percentages (`signal_bot_ratio_percent_display`)**: Scales $\times 100$, adds thousands grouping, and **trims trailing zeros** (`0.05` renders `"5%"`, never `"5.00%"`).
  2. **Win Rate / APY / PnL Ratio (`signal_bot_win_rate_display`, `pnl_ratio`)**: Fixed `2` decimal places with explicit `+` sign on positive values (`"+48.62%"`).
  3. **Empty / Uncomputable Values**: Always render `"--"` (`BotDisplayText::Placeholder`), never `"0"` or empty string `""`.
  4. **Instrument Fee Precision (`feeDig` / `CGD-021`)**: Row PnL color comparison uses the instrument's `feeDig` (default fallback `8` decimals if delisted).

---

## 2. Screen-by-Screen Layout & Component Hierarchy

From the 199 decoded XML layouts in [`reconstructed/resources/layouts/`](file:///Users/aaron/Desktop/aaron/okx/reconstructed/resources/layouts/) (indexed in [`bot_layouts_index.json`](file:///Users/aaron/Desktop/aaron/okx/reconstructed/resources/bot_layouts_index.json)):

### 2.1 Top-Level Navigation & UnifyTrade Bot Mode
1. **Bottom Navigation Bar (5 Tabs)**:
   - `首页 (Home)` | `行情 (Markets)` | `交易 (Trade / Strategy Bots)` | `探索 (Discover)` | `资产 (Assets)`
2. **Trade Mode Switcher Header**:
   - Mode selector pill: `策略 (Strategy Bots)` active alongside `现货 (Spot)`, `合约 (Futures)`, `闪兑 (Convert)`.
   - **Instrument & Bot Selector Bar**:
     - Left: Instrument pair button (`BTC/USDT 永续` or `BTC/USDT`) with 24h change badge (`+2.84%`) -> opens Instrument Search Drawer.
     - Right: **Current Bot Type Selector Pill** (e.g. `现货网格 ▾`, `合约马丁格尔 ▾`, `屯币套利 ▾`) -> opens the **14-Bot Categorized Strategy Drawer / Bot Marketplace**.
     - Icons: K-line toggle icon, AI Assistant (`OKA AI`) icon, Strategy Guide (`?`) icon.

### 2.2 Interactive Bot K-Line Overlay (`com.okinc.kline.features.botkline.*`)
When creating or inspecting a bot, the K-line chart renders real-time visual strategy overlays:
- **Spot / Futures Grid Overlay**:
  - Upper Limit horizontal dashed line (`MAX $105,000` in `#F5455C`).
  - Lower Limit horizontal dashed line (`MIN $86,000` in `#00B865`).
  - Intermediate horizontal grid lines (`P_1 .. P_{N-1}`): Green (`#00B865`) below current price $P_{\text{cur}}$ (Buy grids) and Red (`#F5455C`) above $P_{\text{cur}}$ (Sell grids).
  - Optional Take-Profit (`TP`) and Stop-Loss (`SL`) boundary lines.
- **Spot / Futures DCA Martingale Overlay**:
  - Initial Order line $P_0$ (`首单`).
  - Safety Order ladder lines $P_1, P_2, \dots, P_M$ (`加仓单 #1 .. #M`, spaced by $\delta_1 \cdot m_p^{k-1}$, with line thickness proportional to order volume $A_k = A_1 \cdot m_v^{k-1}$).
  - Dynamic Cycle Take-Profit line $P_{\text{TP},k}$ (`止盈价` in `#00B865`) and Average Holding Cost line $\bar{P}_k$ (`持仓均价` in `#3B82F6`).
  - Liquidation Price line $P_{\text{liq}}$ (`强平价` in `#FFB800` / `#F5455C`).

### 2.3 Order Confirmation Modal (`SpotGridConfirmationRowStyle`)
Before any bot order is submitted, the app presents a bottom-sheet Confirmation Dialog structured according to `SpotGridConfirmationRowStyle`:
- `Header`: Bot Type + Instrument + Direction/Leverage tags (`BTC-USDT-SWAP` | `合约网格` | `全仓/逐仓` | `做多 10x`).
- `SectionHeader`: Parameter section header.
- `Value`: Key-value row (`价格区间: 86,000 - 105,000 USDT`, `网格数量: 50 格 (等差)`, `单格利润率: 0.28% - 0.35%`, `投入保证金: 1,000 USDT`, `预估强平价: 78,412.5 USDT`).
- `AutoProfit` / `Reminder` / `HtmlReminder`: Risk disclosure banner (e.g., initial base coin purchase notice or liquidation distance warning).
- `Confirm CTA`: Full-width primary button (`确认创建`).

### 2.4 Bottom Dock: Running Bots (`当前策略`) & Unified Bot Detail V2
At the bottom of the Strategy Bot screen:
1. **Tabs**: `当前策略 (Running Bots)` | `历史策略 (Bot History)` | `策略广场 (Bot Marketplace)`.
2. **Running Bot Card**:
   - Header: Instrument + Bot Type tag + Direction/Leverage tag + Running duration (`运行时间 3天 14小时 22分`) + Share/Details buttons.
   - Primary Metrics Grid (3 columns):
     - `总收益 (USDT) / 总收益率`: `+142.85 (+14.28%)` in green/red.
     - `网格利润 / 套利利润 / 资金费收益` + `浮动盈亏`.
     - `价格区间 / 持仓均价` + `预估强平价` + `年化收益率 (APY)`.
   - Action Bar Buttons:
     - `修改参数 (Edit)` | `追加投入 / 调整保证金 (Margin/Invest)` | `提取利润 (Withdraw)` | `停止 (Stop)` | `详情 (Details)`.
3. **Bot Detail V2 Modal / Full Screen**:
   - Sub-tabs: `概览 (Overview)` | `当前持仓 (Positions)` | `当前挂单 (Open Orders — Buy/Sell split book)` | `套利记录 / 循环记录 (Cycle & Trade History)`.
