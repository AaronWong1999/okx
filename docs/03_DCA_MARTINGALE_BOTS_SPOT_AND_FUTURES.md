# Deep Dive: DCA / Martingale Strategy Bots (Spot DCA & Futures DCA)

## 1. Overview & Architecture

OKX's **Martingale / DCA (Dollar-Cost Averaging)** strategy engine (`spot_dca` 现货马丁格尔 and `contract_dca` 合约马丁格尔) executes cyclical dip-buying (or rally-shorting in Futures DCA) using a geometric ladder of **1 Initial Order (首单)** and **$M$ Safety Orders (加仓单)**, closing the entire position as soon as the cycle's **Take-Profit Target (`tp_pct`)** is reached, and then starting the next cycle automatically.

- **Rust Core**: `components/biz-service/trading-bot/src/dca/{common,spot,contract}/`
- **Android Native**: `com.okinc.tradingbot.dcabot.{spot,contract}.*` ([`dca_martingale_bots.java`](file:///Users/aaron/Desktop/aaron/okx/reconstructed/dex_decompiled/dca_martingale_bots.java))
- **UniFFI Schema**: `DcaOrderRequest` (39 fields), `ContractDcaPlaceOrderState`, `ContractDcaBotInfo` (37 fields), `AdditionalFundRequest`, `ContractDcaAdjustMarginInteractor`, `ContractDcaAutoMarginSettingInteractor`, `SpotDcaTimelineInteractor` ([`trading_bot_full_schema.rs`](file:///Users/aaron/Desktop/aaron/okx/reconstructed/rust_okcore_idl/trading_bot_full_schema.rs))

---

## 2. Complete Parameter Schema (`DcaOrderRequest`)

Extracted from `UNIFFI_META_TRADING_BOT_RECORD_DCAORDERREQUEST` in `libOKCore.so`:

| Field Name | Wire JSON Key | Type | Range / Enum Values | Description |
| :--- | :--- | :--- | :--- | :--- |
| `inst_id` | `instId` | `Option<String>` | e.g. `"BTC-USDT"`, `"BTC-USDT-SWAP"` | Trading instrument ID |
| `direction` | `direction` | `Option<String>` | `"long"` (做多) \| `"short"` (做空, Contract DCA only) | Cycle trade direction |
| `lever` | `lever` | `Option<String>` | `"1"` .. `"100"` | Leverage multiplier (Contract DCA only) |
| `px_steps` | `pxSteps` | `Option<String>` | `0.1%` – `50%` (wire decimal `"0.015"` = `1.5%`) | **跌多少加仓 / 涨多少加仓**: Price deviation % to trigger Safety Order #1 |
| `tp_pct` | `tpPct` | `Option<String>` | `0.1%` – `1000%` (wire decimal `"0.02"` = `2.0%`) | **单次止盈目标**: Take-profit target % per cycle |
| `tp_base` | `tpBase` | `Option<String>` | `"avg_px"` (平均持仓成本) \| `"init_px"` (首单价格) | Reference price basis for computing cycle take-profit price |
| `init_ord_amt` | `initOrdAmt` | `Option<String>` | $\ge \text{minOrdAmt}$ | **首单金额 / 首单保证金**: Initial order quote amount (or margin) |
| `safety_ord_amt` | `safetyOrdAmt` | `Option<String>` | $\ge \text{minOrdAmt}$ | **加仓单金额 / 加仓单保证金**: Base Safety Order #1 amount |
| `max_safety_ords` | `maxSafetyOrds` | `Option<String>` | `1` – `100` (typically `1` – `20`) | **最大加仓次数**: Maximum number of safety orders per cycle ($M$) |
| `px_steps_mult` | `pxStepsMult` | `Option<String>` | `0.1` – `10.0` (default `"1.0"`) | **加仓价差倍数**: Geometric multiplier $m_p$ applied to price steps |
| `vol_mult` | `volMult` | `Option<String>` | `0.1` – `10.0` (default `"1.5"` or `"2.0"`) | **加仓金额倍数**: Geometric multiplier $m_v$ applied to safety order sizes |
| `start_type` | `triggerStrategy` | `Option<String>` | `"instant"` (立即触发) \| `"price"` (价格触发) \| `"rsi"` (技术指标 RSI) | Cycle start trigger condition |
| `trigger_px` | `triggerPx` | `Option<String>` | $> 0$ | Trigger price when `start_type == "price"` |
| `rsi_timeframe` | `timeframe` | `Option<String>` | `"3m"`, `"5m"`, `"15m"`, `"30m"`, `"1H"`, `"4H"`, `"1D"` | RSI candle bar interval |
| `rsi_threshold` | `thold` | `Option<String>` | `"10"` .. `"90"` (default `"30"` Long, `"70"` Short) | RSI trigger threshold (`<` for Long, `>` for Short) |
| `sl_pct` | `slPct` | `Option<String>` | `0.1%` – `99%` | Stop-loss percentage from initial order price |
| `sl_px` | `slTriggerPx` | `Option<String>` | $> 0$ | Absolute stop-loss trigger price |
| `reserve_funds` | `reserveFunds` | `Option<bool>` | `true` (预先冻结全部资金) \| `false` (仅冻结首单+第1笔加仓) | Whether to lock full cycle funds upfront or dynamically allocate |
| `cycle_limit` | `stopType` | `Option<String>` | `"infinite"` (无限循环) \| `"single"` (本轮止盈后停止) | Cycle continuation mode |
| `margin_reload_target_mr` | `marginReloadTargetMr` | `Option<String>` | Optional decimal | Auto-transfer margin target ratio (Contract DCA only) |

---

## 3. Exact Mathematical Formulas (`OKT*DCAMinInvestMath` & `dcaPreviewServer`)

### 3.1 Safety Order Price Step Ladder ($P_0, P_1, \dots, P_M$)

Let $P_0$ be the Initial Order execution price (`last_px` or `trigger_px`), $\delta_1 = \text{px\_steps}$ (e.g. $0.015 = 1.5\%$), and $m_p = \text{px\_steps\_mult}$ (e.g. $1.2$):

- **Individual Step Percentage $\delta_k$ for Safety Order $k \in \{1, \dots, M\}$**:
  $$\delta_k = \delta_1 \cdot m_p^{k-1}$$
- **Cumulative Price Deviation $D_k$ from Initial Order $P_0$**:
  $$D_k = \sum_{i=1}^{k} \delta_i = \begin{cases} k \cdot \delta_1 & \text{if } m_p = 1 \\ \delta_1 \cdot \frac{m_p^k - 1}{m_p - 1} & \text{if } m_p \neq 1 \end{cases}$$
  - **Constraint**: For Long bots (Spot DCA & Contract DCA Long), cumulative drop must satisfy $D_M < 0.99$ ($99\%$). If $D_M \ge 0.99$, validation fails with `MaxPriceDropExceeds99Percent`.
- **Trigger Price $P_k$ of Safety Order $k$**:
  - **Long (`spot_dca` or `contract_dca` Long)**:
    $$P_k = \text{round}_{\text{tickSz}}\left(P_0 \cdot (1 - D_k)\right)$$
  - **Short (`contract_dca` Short)**:
    $$P_k = \text{round}_{\text{tickSz}}\left(P_0 \cdot (1 + D_k)\right)$$

### 3.2 Safety Order Volume & Required Capital Ladder ($A_0, A_1, \dots, A_M$)

Let $A_0 = \text{init\_ord\_amt}$, $A_1 = \text{safety\_ord\_amt}$, and $m_v = \text{vol\_mult}$:

- **Quote Amount / Margin $A_k$ of Safety Order $k \in \{1, \dots, M\}$**:
  $$A_k = A_1 \cdot m_v^{k-1}$$
- **Cumulative Capital Deployed after $k$ Safety Orders ($C_k$)**:
  $$C_k = A_0 + \sum_{i=1}^{k} A_i = A_0 + \begin{cases} k \cdot A_1 & \text{if } m_v = 1 \\ A_1 \cdot \frac{m_v^k - 1}{m_v - 1} & \text{if } m_v \neq 1 \end{cases}$$
- **Required Minimum Investment ($I_{\min}$, `AiMinInvestmentSource::NativeLocalMath`)**:
  - When `reserve_funds == true` (Default — Lock Full Cycle Funds):
    $$I_{\min} = C_M = A_0 + \sum_{i=1}^{M} A_1 \cdot m_v^{i-1}$$
  - When `reserve_funds == false` (Dynamic Reserve — Lock Initial + First Safety Order):
    $$I_{\min} = A_0 + A_1$$

### 3.3 Average Holding Cost ($\bar{P}_k$) & Cycle Take-Profit Target Price ($P_{\text{TP},k}$)

After the Initial Order ($0$) and $k$ Safety Orders ($1 \dots k$) have filled:
- **Total Base Quantity Accumulated ($Q_k$)**:
  $$Q_k = \sum_{i=0}^{k} \frac{A_i \cdot L}{P_i}$$
  (where $L = 1$ for Spot DCA, or leverage $L$ for Contract DCA).
- **Volume-Weighted Average Entry Price ($\bar{P}_k$)**:
  $$\bar{P}_k = \frac{\sum_{i=0}^{k} A_i \cdot L}{Q_k}$$
- **Cycle Take-Profit Price ($P_{\text{TP},k}$)** for target $\tau = \text{tp\_pct}$:
  - When `tp_base == "avg_px"` (Default — Based on Average Holding Cost):
    $$P_{\text{TP},k}^{\text{long}} = \bar{P}_k \cdot (1 + \tau), \qquad P_{\text{TP},k}^{\text{short}} = \bar{P}_k \cdot (1 - \tau)$$
  - When `tp_base == "init_px"` (Based on Initial Order Price $P_0$):
    $$P_{\text{TP},k}^{\text{long}} = P_0 \cdot (1 + \tau), \qquad P_{\text{TP},k}^{\text{short}} = P_0 \cdot (1 - \tau)$$
- **Required Bounce Percentage from Safety Order $k$ to Take-Profit ($\text{Bounce}_k$)**:
  $$\text{Bounce}_k^{\text{long}} = \frac{P_{\text{TP},k}^{\text{long}} - P_k}{P_k} \times 100\%$$

### 3.4 Contract DCA Worst-Case Liquidation Price (`contract_dca_liquidate_price`)

When all $M$ Safety Orders fill in a Contract DCA cycle with leverage $L$, total margin $M_{\text{total}} = C_M + M_{\text{extra}}$, full base size $Q_M$, and average entry price $\bar{P}_M$:
- **Long Contract DCA Liquidation Price**:
  $$P_{\text{liq,long}} = \frac{\bar{P}_M - \frac{M_{\text{total}}}{Q_M}}{1 - (\text{MMR} + f_{\text{liq}})}$$
- **Short Contract DCA Liquidation Price**:
  $$P_{\text{liq,short}} = \frac{\bar{P}_M + \frac{M_{\text{total}}}{Q_M}}{1 + (\text{MMR} + f_{\text{liq}})}$$
- **Risk Control Warning**: If $P_{\text{liq,long}} \ge P_M$ (meaning the bot would be liquidated *before* the $M$-th safety order can even trigger!), the UI highlights the Liquidation Price in warning red/orange and blocks or warns the user to reduce leverage $L$, reduce $D_M$, or add extra margin.

---

## 4. Running DCA Bot Lifecycle & Interactive Controls

From [`trading_bot_full_schema.rs`](file:///Users/aaron/Desktop/aaron/okx/reconstructed/rust_okcore_idl/trading_bot_full_schema.rs):
1. **Visual Safety Order Timeline (`SpotDcaTimelineInteractor`)**:
   - Renders the live visual progress bar of `[Initial Order Filled] -> [SO #1 Filled] -> [SO #2 Open @ $88,420] -> ... -> [TP Order @ $91,150]`.
2. **Manual Safety Order (`SpotDcaManualAddOrderInteractor` / `ContractDcaManualAddOrderInteractor`)**:
   - Allows the trader to immediately inject a market buy/sell into the current cycle to lower $\bar{P}_k$ without waiting for the next scheduled $P_{k+1}$ trigger.
3. **Additional Funds (`AdditionalFundRequest`) & Margin Adjustment (`ContractDcaAdjustMarginInteractor`)**:
   - Dynamically increases `safety_ord_amt` / `max_safety_ords` or adds isolated margin to push $P_{\text{liq}}$ further away.
4. **Auto-Margin Reload (`ContractDcaAutoMarginSettingInteractor`)**:
   - Automatically transfers USDT from the Trading Account when the margin ratio approaches the danger threshold, restoring it to `margin_reload_target_mr`.
5. **Terminate at End of Cycle (`stop_type = "single"`) vs Immediate Stop**:
   - Traders can either close immediately at market, keep the position, or toggle **"Stop after current cycle completes" (本轮止盈后停止)**.
