# Deep Dive: Grid Strategy Bots (Spot Grid, Futures Grid & Infinity Grid)

## 1. Overview & Variant Matrix

OKX implements three Grid Bot engines inside `components/biz-service/trading-bot/src/grid/` and `com.okinc.tradingbot.grid.*`:

| Feature | 1. Spot Grid (现货网格 `grid`) | 2. Futures Grid (合约网格 `contract_grid`) | 3. Infinity Grid (天地网格 `moon_grid`) |
| :--- | :--- | :--- | :--- |
| **Underlying Market** | `SPOT` (`BTC-USDT`, `ETH-BTC`, etc.) | `SWAP` (Perpetual) & `FUTURES` (Expiry) | `SPOT` |
| **Creation Sub-Modes** | • **AI Strategy** (`GridShort` 7–30D, `GridMedium` 30–90D, `GridLong` 90–180D)<br/>• **Easy Mode** (`SpotGridEasyInteractor`)<br/>• **Manual Mode** (`SpotGridPlaceOrderInteractor`)<br/>• **Copy/Clone** (`SpotGridCopyCloneInteractor`) | • **AI Strategy** (`Long` / `Short` / `Neutral`)<br/>• **Manual Mode** (`ContractGridPlaceOrderInteractor`) | • **AI Strategy** (30D Backtest)<br/>• **Manual Mode** (Floor Price `minPx` + Per-Grid Profit Rate `rate`) |
| **Grid Spacing (`runType`)** | `1` = **Arithmetic (等差)**<br/>`2` = **Geometric (等比)** | `1` = **Arithmetic (等差)**<br/>`2` = **Geometric (等比)** | Always **Geometric (等比)** |
| **Direction (`direction`)** | Spot Long-only (Buy Low, Sell High) | `long` (做多), `short` (做空), `neutral` (中性) | Spot Long-only (Constant Quote Value Maintenance) |
| **Investment Currency** | Quote (`USDT`), Base (`BTC`), or Dual (`BTC + USDT`) | Margin currency (`USDT`, `USDC`, or Coin-M base) | Quote (`USDT`), Base (`BTC`), or Dual |
| **Leverage** | None (`1x` Spot) | `1x` – `100x` (subject to `BotContractInstMeta.max_lever`) | None (`1x` Spot) |
| **Trailing Grid (移动网格)** | Upward & Downward (`trailing_px`) | Upward & Downward (`ContractGridTrailingConfigInput`) | Built-in infinite upward extension |
| **Simple Earn Integration** | Yes (`SpotGridSimpleEarnJourneyInteractor`) | N/A | N/A |

---

## 2. Exact Mathematical Formulas & Risk Control Logic

### 2.1 Grid Price Ladder Generation (`minPx`, `maxPx`, `gridNum`, `runType`)

Given lower price bound $P_{\min}$ (`min_px`), upper price bound $P_{\max}$ (`max_px`), and grid count $N \in [2, 300]$ (`grid_num`, producing $N+1$ price levels $P_0, P_1, \dots, P_N$):

1. **Arithmetic Grid (等差网格, `run_type = "1"`)**:
   - Constant price difference per grid $\Delta P$:
     $$\Delta P = \frac{P_{\max} - P_{\min}}{N}$$
   - Grid price levels ($k = 0, 1, \dots, N$):
     $$P_k = \text{round}_{\text{tickSz}}\left(P_{\min} + k \cdot \Delta P\right)$$
   - Per-grid net profit percentage range $[R_{\min}, R_{\max}]$ (after Maker/Taker fee rate $f$, default `fee_rate` from `BotSpotInstMeta` / `BotContractInstMeta`, typically $0.001 = 0.10\%$ for Spot or $0.0005 = 0.05\%$ for Swap):
     $$R_{\min} = \frac{\Delta P}{P_{\max} - \Delta P} - 2f, \qquad R_{\max} = \frac{\Delta P}{P_{\min}} - 2f$$

2. **Geometric Grid (等比网格, `run_type = "2"`)**:
   - Constant price ratio $r$:
     $$r = \left(\frac{P_{\max}}{P_{\min}}\right)^{\frac{1}{N}}$$
   - Grid price levels ($k = 0, 1, \dots, N$):
     $$P_k = \text{round}_{\text{tickSz}}\left(P_{\min} \cdot r^k\right)$$
   - Constant per-grid net profit percentage $R$:
     $$R = (r - 1) - 2f$$

### 2.2 Maximum Allowed Grid Count (`maxGridNum`) Validation

To prevent negative per-grid profit after trading fees, the engine enforces a minimum price step $\Delta P_{\min}$:
$$\Delta P_{\min} = \max\left(\text{tickSz},\; P_{\text{last}} \cdot (2f + 0.0005)\right)$$
$$\text{maxGridNum} = \min\left(300,\; \left\lfloor \frac{P_{\max} - P_{\min}}{\Delta P_{\min}} \right\rfloor\right)$$
If `grid_num < 2` or `grid_num > maxGridNum`, `GridFieldValidation` emits `GridNumOutOfRange`.

### 2.3 Spot Grid Minimum Investment (`min_invest`) Formula

Let $P_{\text{cur}}$ be the current market price (`last_px`), $S_{\min}$ be the instrument minimum order size (`BotSpotInstMeta.min_sz`), and $k^*$ be the grid index directly below $P_{\text{cur}}$ (so grids $0 \dots k^*$ are Buy orders below market and grids $k^*+1 \dots N-1$ are Sell orders above market requiring initial base coin purchase at $P_{\text{cur}}$):

- Each grid requires at least $S_{\text{grid}} \ge S_{\min}$ base units (rounded up to `lot_sz`).
- **Quote currency needed for lower Buy grids ($0 \dots k^*$)**:
  $$I_{\text{buy}} = \sum_{i=0}^{k^*} S_{\text{grid}} \cdot P_i \cdot (1 + f)$$
- **Quote currency needed to buy initial Base coins at market $P_{\text{cur}}$ for upper Sell grids ($k^*+1 \dots N-1$)**:
  $$I_{\text{sell}} = (N - 1 - k^*) \cdot S_{\text{grid}} \cdot P_{\text{cur}} \cdot (1 + \alpha_{\text{buffer}})$$
  where $\alpha_{\text{buffer}} \approx 0.05$ ($5\%$ price-slippage and fee reserve buffer).
- **Total Minimum Investment (in Quote Currency)**:
  $$I_{\min} = \left(I_{\text{buy}} + I_{\text{sell}}\right) \cdot 1.02$$

### 2.4 Futures / Contract Grid Minimum Margin & Liquidation Price (`liquidate_price`)

From `ContractGridPlaceOrderState` and `ContractGridLiquidatePrice` in [`trading_bot_full_schema.rs`](file:///Users/aaron/Desktop/aaron/okx/reconstructed/rust_okcore_idl/trading_bot_full_schema.rs):

Let:
- $L$ = Leverage (`lever`, e.g. `5`, `10`, `20`)
- $C_{\text{val}}$ = Contract face value (`ct_val` from `BotContractInstMeta`, e.g. $0.01\text{ BTC}$ per contract)
- $S_{\min\text{-ct}}$ = Minimum contract size per grid (`min_sz`, typically $1$ contract)
- $\text{MMR}$ = Maintenance Margin Ratio (tier-dependent, typically $0.004 = 0.4\%$ for BTC-USDT-SWAP Tier 1)
- $M_{\text{init}}$ = User's invested margin (`sz` / `invest_margin`) + extra margin (`extra_margin`)

1. **Contract Size Per Grid ($q_{\text{grid}}$, in contracts)**:
   $$q_{\text{grid}} = \left\lfloor \frac{M_{\text{init}} \cdot L \cdot (1 - \beta_{\text{reserve}})}{\sum_{i=0}^{N-1} C_{\text{val}} \cdot P_i} \right\rfloor_{\text{lotSz}}$$
   where $\beta_{\text{reserve}} \in [0.05, 0.12]$ is the fee & funding reserve factor (`reserve_margin`).

2. **Worst-Case Full-Position Average Entry Price ($\bar{P}_{\text{entry}}$)**:
   - **Long Grid (`direction = "long"`)**: Initially opens $(N - 1 - k^*)$ long grids at $P_{\text{cur}}$, and as price drops to $P_{\min}$, all remaining $(k^* + 1)$ buy grids fill:
     $$\bar{P}_{\text{long}} = \frac{(N - 1 - k^*) \cdot P_{\text{cur}} + \sum_{i=0}^{k^*} P_i}{N}$$
   - **Short Grid (`direction = "short"`)**: Initially opens $(k^* + 1)$ short grids at $P_{\text{cur}}$, and as price rises to $P_{\max}$, all remaining $(N - 1 - k^*)$ sell grids fill:
     $$\bar{P}_{\text{short}} = \frac{(k^* + 1) \cdot P_{\text{cur}} + \sum_{i=k^*+1}^{N} P_i}{N}$$
   - **Neutral Grid (`direction = "neutral"`)**: No initial position at creation; opens Long below $P_{\text{cur}}$ down to $P_{\min}$, or opens Short above $P_{\text{cur}}$ up to $P_{\max}$:
     $$\bar{P}_{\text{neutral,long}} = \frac{\sum_{i=0}^{k^*} P_i}{k^* + 1}, \qquad \bar{P}_{\text{neutral,short}} = \frac{\sum_{i=k^*+1}^{N} P_i}{N - k^*}$$

3. **Estimated Liquidation Price ($P_{\text{liq}}$)**:
   Let $Q_{\text{total}} = N \cdot q_{\text{grid}} \cdot C_{\text{val}}$ be the full base position when all grids are filled, and $M_{\text{total}} = M_{\text{init}} + M_{\text{extra}}$:
   - **Long Liquidation Price ($P_{\text{liq,long}}$)**:
     $$P_{\text{liq,long}} = \frac{\bar{P}_{\text{long}} - \frac{M_{\text{total}}}{Q_{\text{total}}}}{1 - (\text{MMR} + f_{\text{liq}})}$$
     (If $P_{\text{liq,long}} \le 0$, displayed as `"--"`).
   - **Short Liquidation Price ($P_{\text{liq,short}}$)**:
     $$P_{\text{liq,short}} = \frac{\bar{P}_{\text{short}} + \frac{M_{\text{total}}}{Q_{\text{total}}}}{1 + (\text{MMR} + f_{\text{liq}})}$$
   - **Neutral Grid**: Displays dual liquidation bounds `(Long Liq: P_liq,long | Short Liq: P_liq,short)`.

### 2.5 Infinity Grid (天地网格 `moon_grid`) Mechanics

Infinity Grid eliminates the upper price limit $P_{\max}$:
- **User Inputs**:
  1. **Lower Price Limit (`min_px`, $P_{\min}$)**: Must satisfy $P_{\min} < P_{\text{cur}}$ (e.g., $50\%$ of $P_{\text{cur}}$).
  2. **Profit Rate Per Grid (`rate`, $R$)**: $R \in [0.2\%, 100\%]$.
  3. **Investment Amount (`sz`, $I_{\text{total}}$)**.
- **Core Invariant (Constant Valuation Holding)**:
  At price $P_{\text{cur}}$, the bot maintains a constant quote value $V_{\text{hold}}$ of the base asset:
  $$V_{\text{hold}} = I_{\text{total}} \cdot \frac{P_{\min}}{P_{\text{cur}}}$$
  Wait—more precisely, to guarantee the bot can keep buying all the way down from $P_{\text{cur}}$ to $P_{\min}$ while maintaining constant quote value $V_{\text{hold}}$ at the floor $P_{\min}$:
  - Whenever price rises by factor $(1 + R)$ from $P_k$ to $P_{k+1} = P_k(1+R)$, the value of the held base coin increases from $V_{\text{hold}}$ to $V_{\text{hold}}(1+R)$, so the bot **sells** $\Delta V = V_{\text{hold}} \cdot R$ worth of base coin (`sz = (V_hold * R) / P_{k+1}`).
  - Whenever price falls by factor $1/(1+R)$, the bot **buys** $\Delta V = V_{\text{hold}} \cdot R / (1+R)$ worth of base coin to restore the holding value to $V_{\text{hold}}$.
  - Minimum investment $I_{\min}$ ensures that the per-grid trade value $\Delta V = V_{\text{hold}} \cdot R \ge S_{\min} \cdot P_{\text{cur}} \cdot (1 + f)$.

---

## 3. Complete UniFFI Data Models & State Records (from `trading_bot_full_schema.rs`)

### 3.1 `ContractGridPlaceOrderState` (96 Fields in `libOKCore.so`)

```rust
pub struct ContractGridPlaceOrderState {
    pub inst_id: String,                      // Trading pair, e.g. "BTC-USDT-SWAP"
    pub inst_type: String,                    // "SWAP" or "FUTURES"
    pub direction: ContractGridDirection,     // Long | Short | Neutral
    pub grid_type: GridRunType,               // Arithmetic ("1") | Geometric ("2")
    pub min_px: String,                       // Lower price bound
    pub max_px: String,                       // Upper price bound
    pub grid_num: String,                     // Number of grids (2..300)
    pub lever: String,                        // Leverage string, e.g. "5"
    pub invest_margin: String,                // User input margin amount
    pub extra_margin: String,                 // Optional extra safety margin
    pub per_grid_profit_rate: BotDisplayText, // Calculated "0.35% - 0.52%"
    pub min_invest_display: BotDisplayText,   // Calculated minimum margin
    pub liquidate_price_long: BotDisplayText, // Estimated Long liquidation price
    pub liquidate_price_short: BotDisplayText,// Estimated Short liquidation price
    pub trailing_config: ContractGridTrailingConfigInput,
    pub start_condition: GridStartCondition,  // Instant | PriceTrigger | RsiTrigger
    pub stop_condition: GridStopCondition,    // Instant | PriceTrigger | RsiTrigger
    pub tp_trigger_px: String,                // Take-profit price
    pub sl_trigger_px: String,                // Stop-loss price
    pub tp_ratio: String,                     // Take-profit PnL %
    pub sl_ratio: String,                     // Stop-loss PnL %
    pub close_pos_on_stop: bool,              // Close all positions when bot stops
    // ... plus field validation states & Kline overlay projection lines
}
```

### 3.2 Post-Creation Operations (Running Grid Bot Lifecycle)
1. **Modify Parameters (`SpotGridEditParamsInteractor` / `ContractGridEditInteractor`)**:
   - Allows editing `min_px`, `max_px`, `grid_num`, `trailing_px`, and `tp_sl` while the bot is live.
   - Calls `amend-preview` to return `GridEditPreviewData { est_pos_close_pnl, current_pos_sz }` before user confirmation.
2. **Add Investment (`SpotGridAdditionalInvestmentInteractor` / `ContractGridAddMarginInteractor`)**:
   - Injects additional funds and recalculates per-grid order size & liquidation price across all open grid levels.
3. **Withdraw Realized Profit (`SpotGridWithdrawProfitInteractor`)**:
   - Transfers accumulated arbitrage profit (`withdrawable` in `WithdrawProfitDetail`) from the bot sub-account back to the user's Trading Account without stopping the grid.
4. **Stop Bot (`SpotGridStopInteractor` / `ContractGridStopInteractor`)**:
   - Offers two radio options:
     - **Option A (`StopAndSell` / Market Close)**: Cancel all grid orders and sell remaining base coins / close contract position at market price.
     - **Option B (`StopNoSell` / Keep Position)**: Cancel all grid orders and return remaining base coins / contract position to the user's Trading Account.
