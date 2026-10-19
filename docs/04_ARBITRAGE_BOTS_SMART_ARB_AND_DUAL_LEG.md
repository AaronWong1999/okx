# Deep Dive: Arbitrage Strategy Bots (Smart Arbitrage & Dual-Leg Arbitrage Order)

## 1. Overview: Two Distinct Arbitrage Engines in OKX

OKX ships two complementary arbitrage engines inside `components/biz-service/trading-bot/src/{smart_arbitrage,arbitrage}/` and `com.okinc.tradingbot.{smartarb,arbitrage}.*`:

| Dimension | 1. Smart Arbitrage (智能套利 / 屯币套利 `smart_arb`) | 2. Arbitrage Order (套利下单 `arbitrage`) |
| :--- | :--- | :--- |
| **Target User** | Retail & Institutional passive yield seekers (1-click delta-neutral bot) | Pro / Institutional traders executing custom two-leg basis or funding trades |
| **Leg 1 (Left Leg)** | Spot Buy (`BTC-USDT`) or Auto-Staking / Simple Earn (`auto_staking_pos`) | Any `SPOT`, `MARGIN`, `SWAP`, or `FUTURES` instrument (`left_inst_id`) |
| **Leg 2 (Right Leg)** | Perpetual Swap Short (`BTC-USDT-SWAP`) with matched delta ($\Delta = 0$) | Any `SWAP` or `FUTURES` instrument (`right_inst_id`) |
| **Yield Source** | Perpetual Funding Fees (`funding_income`) + On-chain Staking / Earn APY (`auto_staking_profit`) + Spread (`spread_pnl`) | Funding Rate differential or Futures-Spot Basis convergence |
| **Execution Engine** | Managed lifecycle bot with automated entry/exit spread triggers & profit withdrawal | Synchronized dual-leg algorithmic order book slicer with automatic leg-chasing |

---

## 2. Smart Arbitrage (`smart_arbitrage`) — Logic, Parameters, Math & Risk Control

### 2.1 Core Data Models (`trading_bot_full_schema.rs`)

Extracted from `UNIFFI_META_TRADING_BOT_RECORD_SMARTARBITRAGEPLACEORDERSTATE` (67 fields), `SmartArbBotSpecificInfo`, `SmartArbPositioningLegDisplay`, `SmartArbProfitDetail`, and `SmartArbStopType`:

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum SmartArbStopType {
    StopAndSell = 1,   // Sell spot coin + close perpetual short position to USDT
    StopNoSell = 2,    // Keep spot coin + close perpetual short position
    StopKeepStake = 6, // Keep staked/earn asset + close perpetual short position
}

pub struct SmartArbBotSpecificInfo {
    pub total_returned_amt: String,
    pub auto_staking_profit: Option<SmartArbProfitDetail>,
    pub enlarged_amt: String,       // Boosted (leverage-enlarged) notional investment amount
    pub interest_debt: String,      // Amount borrowed against collateral in portfolio margin
    pub auto_staking_pos: String,   // Auto-staked position size
    pub open_cost_info: Option<SmartArbCostInfo>,
    pub close_cost_info: Option<SmartArbCostInfo>,
}

pub struct SmartArbProfitDetail {
    pub ccy: String,
    pub total: String,
    pub apy: String,
    pub withdrawable: String,
    pub withdrawn: String,
    pub frozen: String,
    pub details: Vec<SmartArbProfitSubItem>,
}
```

### 2.2 Delta-Neutral Allocation & Annualized Yield (APY) Formulas

Given total user USDT investment $I_{\text{total}}$, swap leverage $L \in [1, 3]$ (default $L = 1$ or Portfolio Margin boosted), spot price $P_{\text{spot}}$, and perpetual swap price $P_{\text{swap}}$:

1. **Capital Split Between Spot Leg and Swap Short Leg**:
   To hold exactly equal base quantity $Q$ on both legs ($Q_{\text{spot}} = Q_{\text{swap}} = Q$ so net price delta $\Delta = Q_{\text{spot}} - Q_{\text{swap}} = 0$):
   $$Q \cdot P_{\text{spot}} + \frac{Q \cdot P_{\text{swap}}}{L} = I_{\text{total}} \cdot (1 - \beta_{\text{fee}})$$
   $$Q = \left\lfloor \frac{I_{\text{total}} \cdot (1 - \beta_{\text{fee}})}{P_{\text{spot}} + \frac{P_{\text{swap}}}{L}} \right\rfloor_{\text{lotSz}}$$
   - **Spot Leg Allocation**: $I_{\text{spot}} = Q \cdot P_{\text{spot}}$ (optionally staked into Simple Earn / On-Chain Earn when `auto_staking == true`).
   - **Swap Short Margin Allocation**: $M_{\text{swap}} = \frac{Q \cdot P_{\text{swap}}}{L}$.

2. **Entry & Exit Spread Rate Control (`arbitrage_condition_modify`)**:
   - **Entry Spread Rate ($S_{\text{open}}$)**:
     $$S_{\text{open}} = \frac{P_{\text{swap}} - P_{\text{spot}}}{P_{\text{spot}}} \times 100\%$$
     The bot only executes the next slice when $S_{\text{open}} \ge S_{\text{open,min}}$ (e.g. $\ge -0.10\%$ or user-configured threshold).
   - **Exit Spread Rate ($S_{\text{close}}$)**:
     $$S_{\text{close}} = \frac{P_{\text{swap}} - P_{\text{spot}}}{P_{\text{spot}}} \times 100\%$$
     When stopping via `StopAndSell`, the bot unwinds slices when $S_{\text{close}} \le S_{\text{close,max}}$.

3. **Combined Annualized Yield ($\text{APY}_{\text{total}}$)**:
   Let $\bar{r}_{\text{fund}}$ be the 3-day / 7-day / 30-day average 8-hour funding rate (`SmartArbitrageFundingRateCoinHighestApyOverview`) and $\text{APY}_{\text{stake}}$ be the staking yield on the spot leg:
   $$\text{APY}_{\text{funding}} = \frac{\bar{r}_{\text{fund}} \times 3 \times 365 \times (Q \cdot P_{\text{swap}})}{I_{\text{total}}}$$
   $$\text{APY}_{\text{staking}} = \frac{\text{APY}_{\text{stake}} \times (Q \cdot P_{\text{spot}})}{I_{\text{total}}}$$
   $$\text{APY}_{\text{total}} = \text{APY}_{\text{funding}} + \text{APY}_{\text{staking}}$$

---

## 3. Dual-Leg Arbitrage Order (`arbitrage`) — Logic, Parameters & Execution

### 3.1 Core Data Models (`ArbitragePlaceOrderState` & `ArbitragePlaceOrderInteractor`)

Extracted from `UNIFFI_META_TRADING_BOT_RECORD_ARBITRAGEPLACEORDERSTATE` and `UNIFFI_META_TRADING_BOT_INTERFACE_ARBITRAGEPLACEORDERINTERACTOR`:

- **Left & Right Leg Instruments**:
  - `left_inst_id`: Typically `SWAP` or `FUTURES` (e.g. `BTC-USDT-SWAP`).
  - `right_inst_id`: Typically `SPOT` or another `FUTURES` expiry (e.g. `BTC-USDT` or `BTC-USDT-261225`).
- **Direction Combinations (`ArbitrageDirection`)**:
  - `SellShort` + `BuyLong` (**Positive Carry / Cash-and-Carry 正向套利**: Short Swap/Futures + Buy Spot when Funding Rate $> 0$ or Futures $>$ Spot).
  - `BuyLong` + `SellShort` (**Reverse Carry 反向套利**: Long Swap/Futures + Sell/Borrow Spot when Funding Rate $< 0$).
- **Real-Time Spread & Spread Rate**:
  $$\text{Spread} = P_{\text{left}} - P_{\text{right}}, \qquad \text{SpreadRate} = \frac{P_{\text{left}} - P_{\text{right}}}{P_{\text{right}}} \times 100\%$$
- **Synchronized Slicing & Leg Chasing (`arb_trade_config`)**:
  1. Splits total arbitrage size into sub-order slices (`single_ord_sz`) so neither leg sweeps the order book.
  2. Places the less liquid leg first (or both legs simultaneously via limit/IOC orders).
  3. **Automatic Leg Chasing (自动追单)**: If one leg fills and the second leg remains unfilled after `chase_timeout_sec` (or price moves by `chase_threshold_ticks`), the engine automatically amends the unfilled leg to BBO / market price to eliminate single-leg directional exposure!
