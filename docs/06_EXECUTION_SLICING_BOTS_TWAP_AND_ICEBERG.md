# Deep Dive: Execution Slicing Strategy Bots (TWAP & Smart Iceberg)

## 1. Overview: Institutional Order-Slicing Algorithms

OKX's **TWAP (时间加权平均价格委托 `twap`)** and **Smart Iceberg (冰山委托 / 智能冰山 `smart_iceberg`)** are algorithmic execution bots designed to split large orders across `SPOT`, `MARGIN`, `SWAP`, and `FUTURES` into smaller child orders to minimize market impact and hide whale order size from the public order book.

| Dimension | 1. TWAP (时间加权委托 `twap`) | 2. Smart Iceberg (冰山委托 / 智能冰山 `smart_iceberg`) |
| :--- | :--- | :--- |
| **Rust Core Module** | `components/biz-service/trading-bot/src/twap/` | `components/biz-service/trading-bot/src/smart_iceberg/` |
| **Android Package** | `com.okinc.tradingbot.twap.*` | `com.okinc.tradingbot.iceberg.*` |
| **Primary Slicing Axis** | **Time-Driven**: Places a randomized or uniform slice every $\Delta t$ seconds (`time_interval`) over total duration $T$ | **Orderbook-Driven**: Places a visible Maker limit slice (`single_ord_sz`) at a configurable offset from BBO and replenishes immediately as slices fill |
| **Child Order Type** | IOC / Limit at BBO $\pm$ Sweep Variance (`px_var` or `px_spread`) | Post-Only / Limit Maker at BBO $\mp$ Price Variance (`px_var` or `px_spread`) |
| **Price Protection** | Hard Price Limit (`px_limit`): pauses slicing when market crosses limit | Hard Price Limit (`px_limit`): pauses iceberg replenishment when market crosses limit |
| **Live Amendment** | Yes (`TwapModifyOrderInteractor`) | Yes (`SmartIcebergDetailInteractor`) |

---

## 2. TWAP (`twap`) — Complete Parameter Schema, Math & State Machine

### 2.1 Core Data Models (`TwapBotInfo`, `TwapPlaceOrderField`, `TwapDetailHeaderPresentationV1`)

Extracted from [`trading_bot_full_schema.rs`](file:///Users/aaron/Desktop/aaron/okx/reconstructed/rust_okcore_idl/trading_bot_full_schema.rs):

| Field (`TwapPlaceOrderField`) | Wire Key | Allowed Range / Values | Mathematical & Validation Rule |
| :--- | :--- | :--- | :--- |
| **Side & Position Side** | `side`, `posSide` | `buy` / `sell`, `long` / `short` / `net` | Supports Open Long, Open Short, Close Long, Close Short, or Spot Buy/Sell |
| **Variance Mode** | `pxVarType` | `"percent"` (按比例 `%`) \| `"spread"` (按价距 `USDT`) | Controls how aggressive each child limit order sweeps into the book |
| **Price Variance (`px_var`)** | `pxVar` / `pxSpread` | `0.01%` – `1.00%` (or $> 0$ tick multiples) | Child Buy limit price = $P_{\text{ask1}} \cdot (1 + \text{pxVar})$; Child Sell limit price = $P_{\text{bid1}} \cdot (1 - \text{pxVar})$ |
| **Price Limit (`px_limit`)** | `pxLimit` | $> 0$ | Buy TWAP only executes when $P_{\text{last}} \le P_{\text{limit}}$; Sell TWAP only executes when $P_{\text{last}} \ge P_{\text{limit}}$ |
| **Time Interval (`time_interval`)** | `timeInterval` | `5s`, `10s`, `30s`, `60s` (`1m`), `300s` (`5m`), etc. | Interval $\Delta t$ between consecutive child order slices |
| **Single Order Amount (`sz_limit`)** | `szLimit` | $\ge S_{\min}$ and $\le Q_{\text{total}}$ | Target base/contract quantity per child slice (randomized within $\pm 20\%$ to prevent pattern detection) |
| **Total Amount (`sz`)** | `sz` | $\ge \max(S_{\min}, \text{szLimit})$ | Total order quantity to execute |
| **Estimated Duration ($T_{\text{est}}$)** | Computed locally | `1m` – `7d` (`604,800s`) | $T_{\text{est}} = \left\lceil \frac{\text{sz}}{\text{szLimit}} \right\rceil \times \Delta t$ |

### 2.2 TWAP Child Order Execution Algorithm
At each timer tick $t_k = t_0 + k \cdot \Delta t$:
1. **Check Price Limit Gate**:
   - If `side == "buy"` and $P_{\text{ask1}} > P_{\text{limit}}$, pause tick (status remains `Running`, sub-status `PausedByPriceLimit`).
   - If `side == "sell"` and $P_{\text{bid1}} < P_{\text{limit}}$, pause tick.
2. **Compute Randomized Slice Size $q_k$**:
   $$q_k = \min\left(Q_{\text{remaining}},\; \text{round}_{\text{lotSz}}\left(\text{szLimit} \cdot U(0.8, 1.2)\right)\right)$$
   If $Q_{\text{remaining}} - q_k < S_{\min}$, merge remainder into $q_k = Q_{\text{remaining}}$.
3. **Compute Aggressive Limit Price $P_{\text{child},k}$**:
   - **Buy**:
     $$P_{\text{child},k} = \min\left(P_{\text{limit}},\; \begin{cases} P_{\text{ask1}} \cdot (1 + \text{pxVar}) & \text{if percent mode} \\ P_{\text{ask1}} + \text{pxSpread} & \text{if spread mode} \end{cases}\right)$$
   - **Sell**:
     $$P_{\text{child},k} = \max\left(P_{\text{limit}},\; \begin{cases} P_{\text{bid1}} \cdot (1 - \text{pxVar}) & \text{if percent mode} \\ P_{\text{bid1}} - \text{pxSpread} & \text{if spread mode} \end{cases}\right)$$
4. **Submit IOC / Limit Order**: Any unfilled portion of $q_k$ is canceled before the next tick $t_{k+1}$ and rolled back into $Q_{\text{remaining}}$.

---

## 3. Smart Iceberg (`smart_iceberg`) — Complete Parameter Schema & Replenishment Logic

### 3.1 Architecture (`components/biz-service/trading-bot/src/smart_iceberg/create_journey/`)
The Smart Iceberg Rust module is structured into five clean sub-modules:
- `actions.rs`: User input mutations (`SetVarianceType`, `SetPriceVariance`, `SetPriceLimit`, `SetSingleOrderSize`, `SetTotalSize`, `SubmitOrder`)
- `authority.rs`: Account mode, margin availability, and instrument trading rule checks
- `presentation.rs`: Formatted display strings, hints, and currency suffixes
- `subscriptions.rs`: Real-time ticker (`last_px`, `bid1`, `ask1`) and balance stream bindings
- `validation.rs`: Pure synchronous field validators enforcing:
  1. $\text{sz} \ge 2 \times \text{single\_ord\_sz}$ (Total amount must be at least 2 slices)
  2. $\text{single\_ord\_sz} \ge S_{\min}$ (Each visible slice must meet exchange minimum order size)
  3. $\text{sz} / \text{single\_ord\_sz} \le 500$ (Maximum active slice count guard)

### 3.2 Iceberg Maker Order Placement & Auto-Relisting Formula
Unlike TWAP (which crosses the spread on a timer), Smart Iceberg rests **passive Maker limit orders** inside the order book:
- **Buy Iceberg Resting Price**:
  $$P_{\text{iceberg,buy}} = \min\left(P_{\text{limit}},\; \begin{cases} P_{\text{bid1}} \cdot (1 - \text{pxVar}) & \text{if percent mode} \\ P_{\text{bid1}} - \text{pxSpread} & \text{if spread mode} \end{cases}\right)$$
- **Sell Iceberg Resting Price**:
  $$P_{\text{iceberg,sell}} = \max\left(P_{\text{limit}},\; \begin{cases} P_{\text{ask1}} \cdot (1 + \text{pxVar}) & \text{if percent mode} \\ P_{\text{ask1}} + \text{pxSpread} & \text{if spread mode} \end{cases}\right)$$
- **Dynamic Order Replenishment & Re-Anchoring**:
  - Only $q_k \approx \text{single\_ord\_sz} \cdot U(0.85, 1.15)$ is visible on the book at any time.
  - As soon as the visible slice fills, the next slice is immediately placed.
  - If the market BBO moves away from the resting order by more than $2 \times \text{pxVar}$, the bot automatically cancels and re-anchors the resting limit order to the new BBO $\mp \text{pxVar}$.
