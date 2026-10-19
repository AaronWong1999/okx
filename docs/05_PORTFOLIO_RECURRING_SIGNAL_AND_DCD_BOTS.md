# Deep Dive: Smart Portfolio, Recurring Buy, Signal Bot & DCD Bots (Dip Sniper, Peak Sniper, Flywheel)

## 1. Smart Portfolio (屯币宝 `smart_portfolio`)

### 1.1 Architecture & Core Data Models
- **Rust Core**: `components/biz-service/trading-bot/src/smart_portfolio/`
- **Android Native**: `com.okinc.tradingbot.portfolio.*` ([`portfolio_and_recurring_bots.java`](file:///Users/aaron/Desktop/aaron/okx/reconstructed/dex_decompiled/portfolio_and_recurring_bots.java))
- **Purpose**: Maintains a basket of $K \in [2, 10]$ spot cryptocurrencies at fixed target weight percentages $w_1, w_2, \dots, w_K$ ($\sum_{i=1}^K w_i = 100\%$, each $w_i \ge 1\%$ in $1\%$ increments). Whenever relative price movements cause weights to drift, the bot sells outperforming coins and buys underperforming coins to lock in relative volatility gains ("rebalancing bonus").

### 1.2 Rebalancing Modes & Mathematical Trigger Logic

Let $Q_i(t)$ be the held quantity of coin $i$, $P_i(t)$ be its current USDT price, $V_i(t) = Q_i(t) \cdot P_i(t)$ be its current valuation, and $V_{\text{total}}(t) = \sum_{j=1}^K V_j(t)$ be total portfolio value. The current live weight of coin $i$ is:
$$\hat{w}_i(t) = \frac{V_i(t)}{V_{\text{total}}(t)}$$

1. **Proportional Rebalancing (按比例触发 `rebalance_type = "proportional"`)**:
   - User selects threshold $\theta_{\text{ratio}} \in [1\%, 20\%]$ (default $3\%$ or $5\%$).
   - **Trigger Condition**: Checked every 5 minutes; triggers a rebalance trade group (`trade_group_list`) whenever ANY coin's absolute weight drift exceeds $\theta_{\text{ratio}}$ (or relative drift $|\hat{w}_i(t) - w_i| \ge \theta_{\text{ratio}}$):
     $$\max_{i \in \{1 \dots K\}} \left| \hat{w}_i(t) - w_i \right| \ge \theta_{\text{ratio}}$$
2. **Periodic Rebalancing (按周期触发 `rebalance_type = "periodic"`)**:
   - User selects time interval $\Delta T \in \{30\text{m}, 1\text{h}, 4\text{h}, 12\text{h}, 24\text{h}, 7\text{d}, 28\text{d}\}$.
   - **Trigger Condition**: At every interval tick $t_n = t_0 + n \cdot \Delta T$, if any coin's rebalance trade size exceeds the minimum exchange order notional ($\max_i |V_i(t_n) - w_i V_{\text{total}}(t_n)| \ge \text{minNotional}$), the bot executes a rebalance trade group.
3. **Rebalance Trade Execution**:
   - Target value for coin $i$: $V_i^* = w_i \cdot V_{\text{total}}(t)$.
   - Delta value: $\Delta V_i = V_i^* - V_i(t)$.
   - Coins with $\Delta V_i < 0$ are **sold first** ($\Delta Q_i = |\Delta V_i| / P_i(t)$) to generate USDT; then coins with $\Delta V_i > 0$ are **bought** using the proceeds.
4. **Minimum Investment (`min_invest`) Formula**:
   Let $w_{\min} = \min_{i \in \{1 \dots K\}} w_i$ be the smallest coin allocation weight and $N_{\min,i} = S_{\min,i} \cdot P_i(0)$ be the minimum order notional for coin $i$:
   $$I_{\min} = \max_{i \in \{1 \dots K\}} \left( \frac{N_{\min,i}}{w_i \cdot \theta_{\text{ratio}}} \right) \cdot 1.10$$

---

## 2. Recurring Buy (定投策略 `recurring_buy`)

### 2.1 Architecture & Core Interactors
- **Rust Core**: `components/biz-service/trading-bot/src/recurring_buy/`
- **Key UniFFI Interactors** ([`trading_bot_full_schema.rs`](file:///Users/aaron/Desktop/aaron/okx/reconstructed/rust_okcore_idl/trading_bot_full_schema.rs)):
  - `RecurringBuyPlaceOrderInteractor` (`set_trade_quote_ccy`, `set_frequency`, `set_amount`, `add_coin`, `remove_coin`, `set_coin_ratio`, `submit`)
  - `RecurringBuyDetailInteractor` (`pause`, `resume`, `stop`, `release`)
  - `RecurringBuyEditInteractor`
  - `RecurringBuyInvestMoreInteractor`
  - `RecurringBuySimpleEarnJourneyInteractor`

### 2.2 Parameter Schema & Execution Rules
| Parameter | Wire Key | Supported Values / Validation |
| :--- | :--- | :--- |
| **Selected Coins** | `alkList` | `1` to `20` spot assets (`ccy` + `ratio`), where $\sum \text{ratio} = 100\%$ |
| **Frequency** | `period` | `1H`, `4H`, `8H`, `12H`, `daily` (每日), `weekly` (每周), `biweekly` (每两周), `monthly` (每月 1–28 日) |
| **Execution Time** | `execTime` | `00:00` – `23:00` local timezone (`timeZone` UTC offset) |
| **Per-Cycle Amount** | `amt` | $\ge 2\text{ USDT}$ (scales with number of coins so each coin leg $\ge 1\text{ USDT}$) |
| **Investment Currency** | `tradeQuoteCcy` | `USDT` or `USDC` |
| **Advanced Price Range** | `minPx` / `maxPx` | Optional per-coin price ceiling/floor; skips buying a coin in cycles where $P_{\text{cur}}$ is outside range |
| **Simple Earn Auto-Subscribe** | `earnEnable` | `true` / `false` — automatically deposits purchased spot coins into OKX Simple Earn (赚币活期) |
| **Funding Source** | `acctMode` | Trading Account (`trading`) or Funding Account (`funding`) |

---

## 3. Signal Bot (信号策略 `signal_bot` & `signal_square`)

### 3.1 Architecture & Core Interactors
- **Rust Core**: `components/biz-service/trading-bot/src/signal_bot/` & `src/signal_square/`
- **Key UniFFI Interactors & Functions**:
  - `SignalBotDetailInteractor`, `SignalBotActionsInteractor` (`stop_bot`, `edit_pairs`, `edit_tp_sl`, `manual_place_order`)
  - `SignalBotAdjustMarginInteractor`
  - `SignalSquareInteractor` (Marketplace for subscribing to third-party Lead Trader signals)
  - `signal_bot_ratio_percent_display(raw_ratio)` & `signal_bot_win_rate_display(raw_win_rate)`

### 3.2 TradingView Webhook Alert Specification (`create_sign`)
When a user creates a custom signal in OKX Signal Bot, the engine provisions a dedicated Webhook URL (`https://www.okx.com/pap/v5/algo/signal/webhook`) and a secret `signalToken`, and generates the TradingView PineScript / Alert JSON template:

```json
{
  "action": "ENTER_LONG",
  "instrument": "{{ticker}}",
  "signalToken": "<USER_SIGNAL_TOKEN>",
  "timestamp": "{{timenow}}",
  "maxLag": "60",
  "investmentType": "percentage_balance",
  "amount": "100",
  "orderType": "market",
  "priceOffset": "0"
}
```
- **Supported `action` Values**:
  - `ENTER_LONG`: Open or add to Long position (if Short position exists and `reversal == true`, closes Short first then opens Long).
  - `EXIT_LONG`: Close existing Long position (full or partial `amount`).
  - `ENTER_SHORT`: Open or add to Short position.
  - `EXIT_SHORT`: Close existing Short position.
- **Risk Controls**:
  - `maxLag`: Rejects stale webhook alerts if `server_timestamp - alert_timestamp > maxLag` seconds.
  - **Bot-Level TP/SL (`edit_tp_sl`)**: Enforces per-position Take-Profit % and Stop-Loss % independent of webhook exit signals.
  - **Multiple Entry Toggle (`allowMultipleEntries`)**: Controls whether consecutive `ENTER_LONG` signals pyramid into the position or are ignored while already in a Long position.

---

## 4. DCD Structured Strategy Bots (抄底宝 Dip Sniper, 逃顶宝 Peak Sniper & 飞轮策略 Flywheel `dcd`)

### 4.1 Overview & Variants
Located in `components/biz-service/trading-bot/src/dcd/`:
1. **Dip Sniper (抄底宝 `dcd_dip`)**: Buy-low dual-currency option strategy. The user locks `USDT` at a target strike price $K < P_{\text{cur}}$ (`TargetPrice` or `PriceDrop`). At expiry:
   - If $P_{\text{settlement}} \le K$: User buys `BTC` at discount price $K$ **plus** earns option premium yield ($\text{APY}$).
   - If $P_{\text{settlement}} > K$: User receives principal `USDT` back **plus** option premium yield in `USDT`.
2. **Peak Sniper (逃顶宝 `dcd_peak`)**: Sell-high dual-currency option strategy. The user locks `BTC` at a target strike price $K > P_{\text{cur}}$. At expiry:
   - If $P_{\text{settlement}} \ge K$: User sells `BTC` at premium price $K$ **plus** earns option yield in `USDT`.
   - If $P_{\text{settlement}} < K$: User keeps `BTC` principal **plus** earns option yield in `BTC`.
3. **Flywheel Strategy (飞轮策略 `flywheel`)**: Automated cyclical **Dip Sniper $\leftrightarrow$ Peak Sniper** loop!
   - Starts with `USDT` in a Dip Sniper leg (`Buy Low` at $K_{\text{buy}}$):
     - Keeps collecting `USDT` yield every settlement cycle until exercised into `BTC` at $K_{\text{buy}}$.
   - Once exercised into `BTC`, automatically flips into a Peak Sniper leg (`Sell High` at $K_{\text{sell}} \ge \max(K_{\text{buy}}, P_{\text{cur}} \cdot (1 + \delta_{\text{up}}))$):
     - Keeps collecting `BTC` yield every cycle until exercised back into `USDT` at $K_{\text{sell}}$.
   - Supports two AI preset tiers (`AiTier::FlywheelAggressive` 高收益型 and `AiTier::FlywheelBalanced` 稳健型) and two intention modes (`AiDcdInvestMethod::TargetPrice` vs `AiDcdInvestMethod::PriceDrop`).

### 4.2 Exact Validation Order (`AiDcdInvestValidation` in `trading_bot_full_schema.rs`)
From `UNIFFI_META_TRADING_BOT_ENUM_AIDCDINVESTVALIDATION`:
1. `InvalidAmount`: Amount missing, non-numeric, or $\le 0$.
2. `InsufficientBalance`: Amount $>$ `displayAvail.originQuoteMax`.
3. `AmountTooSmall`: Amount $<$ quote's `tradeSizeMin`.
4. `AmountTooLarge`: Amount $>$ quote's `tradeSizeMax`.
5. `Valid`: Passes all guards and dispatches `PlaceDcdOrderInput`.
