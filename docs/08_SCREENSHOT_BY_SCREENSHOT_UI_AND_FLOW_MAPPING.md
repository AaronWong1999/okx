# 08 — 239 张实机录屏截图与 APK 逆向代码 1:1 映射全集 (`策略BOT截图` ↔ DEX / XML / Rust UniFFI)

本文档将 `/Users/aaron/Desktop/aaron/okx/策略BOT截图/` 目录下的 **15 个子目录、239 张 `1180×2556` 原生分辨率实机截图** 与从 `/Users/aaron/Downloads/android.apk` (`v6.194.0`) 逆向提取出的：
1. **Rust `libOKCore.so` UniFFI 跨端状态机与算法符号** (`reconstructed/rust_okcore_idl/trading_bot_full_schema.rs` & `social_trading_full_schema.rs`)
2. **DEX 反编译 Java/Kotlin 视图控制器与 ViewModel** (`reconstructed/dex_decompiled/*.java`)
3. **ARSC 解码 XML 布局与多语言字符串资源** (`reconstructed/resources/layouts/*.xml` & `bot_strings_zh_CN.json`)

进行 **1:1 逐模块、逐界面、逐字段、逐校验规则** 的完整映射。

---

## 1. 15 个截图目录与 APK 逆向模块总表

| 目录编号与名称 | 截图数 | 对应策略类型 (`bot_type`) | 核心 Rust UniFFI Interactor (`libOKCore.so`) | 核心 Android DEX 类 (`reconstructed/dex_decompiled/`) | 核心 XML 布局 (`reconstructed/resources/layouts/`) |
| :--- | :---: | :--- | :--- | :--- | :--- |
| [`00_公共组件_创建策略与通用功能`](../策略BOT截图/00_公共组件_创建策略与通用功能/README.md) | 36 | 入口 / 策略&跟单 / 创建策略抽屉 | `TradingBotMarketplaceInteractor`, `BotEntryRouter` | `TradingBotMarketFragment`, `BotTypeSelectBottomSheet` | `bots_fragment_marketplace.xml`, `bots_dialog_create_strategy_selector.xml` |
| [`01_现货网格_SpotGrid`](../策略BOT截图/01_现货网格_SpotGrid/README.md) | 16 | `spot_grid` | `SpotGridInteractor`, `SpotGridAiInteractor` | `SpotGridCreateFragment`, `SpotGridManualViewModel` | `bots_fragment_spot_grid_manual.xml`, `bots_layout_grid_advanced_settings.xml` |
| [`02_合约网格_FuturesGrid`](../策略BOT截图/02_合约网格_FuturesGrid/README.md) | 28 | `contract_grid` (USDT永续 & 币本位CM) | `ContractGridInteractor`, `GridSlippageConfig` | `ContractGridCreateFragment`, `ContractGridLeverageDialog` | `bots_fragment_contract_grid_manual.xml`, `bots_dialog_slippage_control.xml` |
| [`03_智能组合_SmartPortfolio`](../策略BOT截图/03_智能组合_SmartPortfolio/README.md) | 5 | `smart_portfolio` | `SmartPortfolioInteractor`, `RebalanceTriggerRule` | `SmartPortfolioCreateActivity`, `PortfolioCoinRatioAdapter` | `bots_activity_smart_portfolio.xml`, `bots_item_portfolio_coin_weight.xml` |
| [`04_合约马丁格尔_FuturesMartingale`](../策略BOT截图/04_合约马丁格尔_FuturesMartingale/README.md) | 13 | `contract_dca` | `ContractDcaInteractor`, `DcaMarginTransferConfig` | `ContractDcaFragment`, `DcaAiHelperCardView` | `bots_fragment_contract_dca.xml`, `bots_layout_dca_margin_transfer.xml` |
| [`05_现货马丁格尔_SpotMartingale`](../策略BOT截图/05_现货马丁格尔_SpotMartingale/README.md) | 10 | `spot_dca` | `SpotDcaInteractor`, `DcaTriggerIndicatorConfig` | `SpotDcaFragment`, `SpotDcaLadderPreviewDialog` | `bots_fragment_spot_dca.xml`, `bots_dialog_rsi_trigger_config.xml` |
| [`06_智能套利_SmartArbitrage`](../策略BOT截图/06_智能套利_SmartArbitrage/README.md) | 18 | `smart_arbitrage` / `arbitrage_order` | `SmartArbitrageInteractor`, `DualLegArbInteractor` | `SmartArbFragment`, `ArbDualLegOrderView`, `ArbPreviewDialog` | `bots_fragment_smart_arbitrage.xml`, `bots_fragment_dual_leg_arb.xml` |
| [`07_信号策略_SignalBot`](../策略BOT截图/07_信号策略_SignalBot/README.md) | 16 | `signal_trading` | `SignalBotInteractor`, `TradingViewSignalParser` | `SignalBotCreateFragment`, `AddTradingViewSignalActivity` | `bots_fragment_signal_bot_create.xml`, `bots_activity_add_tv_signal.xml` |
| [`08_冰山策略_Iceberg`](../策略BOT截图/08_冰山策略_Iceberg/README.md) | 23 | `iceberg` | `IcebergInteractor`, `IcebergSliceConfig` | `IcebergOrderFragment`, `MarginModeHelpBottomSheet` | `bots_fragment_iceberg_order.xml`, `bots_dialog_margin_mode_guide.xml` |
| [`09_时间加权策略_TWAP`](../策略BOT截图/09_时间加权策略_TWAP/README.md) | 6 | `twap` | `TwapInteractor`, `TwapIntervalValidator` | `TwapOrderFragment`, `TwapCountdownChartOverlay` | `bots_fragment_twap_order.xml` |
| [`10_定投策略_RecurringBuy`](../策略BOT截图/10_定投策略_RecurringBuy/README.md) | 30 | `recurring_buy` | `RecurringBuyInteractor`, `RecurringAccountDeductOrder` | `RecurringBuyActivity`, `RecurringFeaturedFragment` | `bots_activity_recurring_buy.xml`, `bots_fragment_recurring_featured.xml` |
| [`11_抄底止盈策略_DipBuyTakeProfit`](../策略BOT截图/11_抄底止盈策略_DipBuyTakeProfit/README.md) | 14 | `dcd_bot` (Flywheel / 双币循环套利) | `DcdBotInteractor`, `DcdProductMatcher` | `DcdBotCreateActivity`, `DcdGuideDialogFragment` | `bots_activity_dcd_bot.xml`, `bots_dialog_dcd_product_match.xml` |
| [`12_股票代币现货网格_StockTokenSpotGrid`](../策略BOT截图/12_股票代币现货网格_StockTokenSpotGrid/README.md) | 10 | `spot_grid` (`asset_class=StockToken`) | `StockTokenGridInteractor`, `SpotGridInteractor` | `StockTokenGridLandingActivity` | `bots_activity_stock_token_grid.xml` |
| [`13_美股代币合约网格_StockTokenFuturesGrid`](../策略BOT截图/13_美股代币合约网格_StockTokenFuturesGrid/README.md) | 4 | `contract_grid` (`asset_class=StockToken`) | `StockTokenContractGridInteractor` | `StockTokenContractGridFragment`, `StockTokenRiskNoticeDialog` | `bots_dialog_stock_contract_risk.xml` |
| [`14_AI策略与AI组合_AI_Strategy`](../策略BOT截图/14_AI策略与AI组合_AI_Strategy/README.md) | 10 | `ai_portfolio` / `btc_yield_grid` | `AiPortfolioPresetInteractor`, `BtcYieldGridInteractor` | `AiPortfolioPresetDetailActivity`, `BtcYieldGridShowcaseActivity` | `bots_activity_ai_portfolio_detail.xml`, `bots_activity_btc_yield_grid.xml` |

---

## 2. 逐目录界面细节、参数范围与逆向代码精确对照

### 2.1 `00_公共组件_创建策略与通用功能`（36 张截图）

1. **交易/资产页顶部与「策略&跟单」主页** (`22-53-19_策略跟单-创建策略入口.png`, `23-36-18_策略跟单-我的策略与我的跟单.png`)
   - **一级导航栏**：`闪兑 | 现货 | 合约 | TradFi | 策略&跟单 | 期权`（当前选中 `策略&跟单`，粗体纯黑 `#000000`）。
   - **资产估值卡**：
     - `总资产估值 👁` -> `0.00 USD` (`32sp` Bold)
     - `$0.00 今日收益 ⌄`（带虚线下划线可点击查看收益明细弹窗）
   - **快捷金刚区（横向圆标）**：
     - `+ 创建策略`（纯黑实心圆底 `#121212` + 白色加号）
     - `现货网格`（浅灰圆底 `#F5F5F5` + 网格折线图标）
     - `合约网格`（浅灰圆底 `#F5F5F5` + `5x` 网格图标）
     - `智能组合`（右上角荧光绿角标 `New` `#BCFF2F`）
     - `定投策略`（日历定投图标）
   - **二级 Tab 栏**：`推荐 | 我的策略 ▾ | 我的跟单`（右侧带历史委托图标）。
   - **推荐流卡片**：
     - 「**股票代币现货网格**」：`锚定股票代币波动特性，网格自动捕捉每一次起落`
     - 胶囊标签：`|||| 稳健型` + 头像组 `3,591 人已使用`
     - 标的列表：
       - `xMU/USDT`：年化收益率 `+12.54%` | `1,922` 人 | 右侧黑色胶囊按钮 `[创建]`
       - `xSOXL/USDT`：年化收益率 `+27.15%` | `418` 人 | `[创建]`
       - `xSNDK/USDT`：年化收益率 `+25.67%` | `786` 人 | `[创建]`
     - 底部通栏轮播横幅：
       - 横幅 1：`欧易独家！网格套利 × 双币生息，让资金全天候生息`
       - 横幅 2：`选择交给自己，再平衡交给规则。全球资产配置，智能组合陪你穿越周期`

2. **「创建策略」底部抽屉 (`BotTypeSelectBottomSheet`)** (`22-53-19_热门策略-策略类型总览.png`, `23-30-45_热门策略-抄底止盈与马丁格尔等.png`, `23-30-45_热门策略-信号策略与智能组合.png`)
   - **顶部过滤 Tab**：`全部 | 现货 | 合约`
   - **顶部大卡片区（边框圆角卡 `12dp`）**：
     1. **股票代币现货网格** `New`：`锚定股票代币波动特性，网格自动捕捉每一次起落`
     2. **现货网格**：`低买高卖，震荡向上行情`
     3. **合约网格**：`循环套利，可加杠杆，牛熊皆宜`
     4. **合约网格 (币本位)** `New`：`投币赚币，自动赚币复利，获取更高收益`
     5. **智能组合** `New`：`多币组合，动态调仓`
   - **分组宫格区（4 列圆形图标）**：
     - **热门策略**：`抄底止盈策略`、`合约马丁格尔`、`现货马丁格尔`、`信号策略`、`智能套利`（带荧光绿 `Tradfi` 角标）
     - **大单拆分**：`冰山策略`、`时间加权策略`
     - **进阶策略**：`定投策略`、`智能组合`（带 `New` 角标）、`套利下单`

---

### 2.2 `01_现货网格_SpotGrid`（16 张截图）

- **核心截图**：
  - `22-53-19_AI推荐参数-收益预估与回测年化.png`
  - `22-53-19_参数面板-网格数量与单网格收益率.png`
  - `22-53-19_参数面板-完整参数总览.png`
  - `22-53-19_预设开始条件-价格触发.png` / `RSI14触发.png`
  - `22-53-19_预设停止条件.png`
- **精确字段与交互细节**：
  - **顶部行情条**：`BTC/USDT ▾` | `82,226.6 ↓`（跌红 `#E53935` / 涨绿 `#16A34A`）| K线展开开关 `🕯 ^`（展开后显示 `EMA / MA` 指标与网格线叠加图层）。
  - **价格区间**：`最低价 (USDT)` — `最高价 (USDT)`。
  - **网格数量**：`2 ~ 1,000`，右侧或下方实时回显 **`单网格收益率`**（已扣手续费，如 `0.15% - 0.28%`）。
  - **投资额**：灰底圆角输入框 `#F5F5F5`，左侧动态提示最小起投门槛（如 `> 60.2 USDT`），右侧币种选择器 `USDT ▾`（支持单币 `USDT`、单币 `BTC` 或双币 `BTC+USDT` 投入）；下方 5 节点百分比滑杆 (`0% / 25% / 50% / 75% / 100%`)。
  - **开关项**：
    - `利润复投 ⓘ`（Switch 开关，开启后每格套利利润自动并入本金放大后续网格挂单量）
    - `自动赚币`（显示实时赚币年化 `+2.79% 年化 >`，将未成交网格的闲置资金转入简单赚币生息）
  - **高级设置 ⓘ（可折叠面板）**：
    - `简单赚币分配`：`未设置 >`
    - `网格随价格上移`（Trailing Up）：`未设置 >`（支持按价格或按比例设置上移最高价上限）
    - `网格随价格下移`（Trailing Down）：`未设置 >`
    - `预设开始条件`：`未预设 >`（弹窗含：`立即触发`、`价格触发`、`RSI (14, 70/30)` 指标触发、`TradingView 信号`、以及 `延迟开始` 秒数）
    - `预设停止条件`：`未预设 >`（价格触发、RSI 触发、TradingView 信号、停止时是否卖出所持币种）
    - `止盈`：`未设置 >`
    - `止损`：`未设置 >`
    - `网格模式`：胶囊分段选择器 `等差`（白色浮起滑块）| `等比`
  - **底部主按钮**：全宽黑色圆角胶囊按钮 `创建策略` (`height=48dp`, `border-radius=999px`)。

---

### 2.3 `02_合约网格_FuturesGrid`（28 张截图）

- **涵盖两个子变体**：
  1. **USDT 本位永续合约网格**（视频 `22-53-19`，标的 `BTCUSDT 永续`，保证金单位 `USDT`）
  2. **币本位 (CM) 永续合约网格**（视频 `22-58-20`，标的 `BTCUSD 永续`，保证金单位 `BTC`，在创建策略抽屉中标记为 `合约网格 (币本位) New`）
- **精确字段与交互细节**：
  - **方向 Tab**：`做多 | 做空 | 中性`
  - **投资额 (保证金)**：
    - 右侧直接内嵌杠杆选择器按钮 `5x ▾`（点击弹出 **调整杠杆弹窗**：滑杆 + 快捷档位 `1x / 5x / 10x / 20x / 30x / 50x / 100x` + 强平风险提示）
    - 输入框内动态灰字门槛：`> 339.66 (USDT)`
    - 校验报错（红框 + 红字）：`保证金应大于 339.66 USDT`
    - 复选框：`☑ 自动预留保证金`
  - **实时风控指标行**：
    - `实际保证金 + 预留保证金`
    - `可用`：`0 USDT ⊕`
    - `含杠杆的实际投入`
    - `预估强平价`
    - 币本位额外展示：`预估买入均价`、`预估卖出均价`、`30 日年化资金费率`
  - **滑点控制设置弹窗 (`22-53-19_参数面板-实际保证金与预估强平价.png` / `滑点控制.png`)**：
    - 顶部标题：`滑点控制设置` + 关闭 `✕`
    - `滑点控制` Switch 开关
    - 输入框：`滑点 0.0001-5 (%)`，默认值 `0.0004`
    - 5 个快捷档位按钮：`0.05%` | `0.2%` | `0.5%` | `1%` | `5%`
    - 信息行：`AI 推荐滑点 0.0004%`、`当前价格 82,145.8 USDT`
    - 底部深绿胶囊按钮 `#1B5E20`：`确认`
  - **止盈止损三模式切换 (`22-58-20_参数面板-止盈止损与价格区间.png`)**：
    - 支持按 `最新价`、`价格区间`、`止盈收益率 (%)` 三种维度配置止盈止损。

---

### 2.4 `03_智能组合_SmartPortfolio`（5 张截图）与 `14_AI策略与AI组合_AI_Strategy`（10 张截图）

- **顶部双模式胶囊切换**：`AI 策略 | 手动创建`
- **AI 策略（预设组合卡片流，见 `14_AI策略与AI组合`）**：
  - **桥水基金全天候加密版配置**：标签 `加密 | 全天候 | 精选`；`6月回测年化 +18.14%`；`跟随人数 23`；绿色面积净值曲线；简介 `“参考桥水全天候配置理念构建的精选加密资产组合。”`；右侧黑色胶囊按钮 `[使用]`。
  - **存储与储备组合**：标签 `股票`（荧光绿底）`存储 | 储备`；`1月回测年化 -291.57%`；`跟随人数 203`；构成标的 `xMU 40% / xBMNR 35% / xREN 25%`；再平衡规则：`定期检查权重，当任一标的偏离目标比例达到 5% 时自动再平衡`。
  - **AI 数据与储备组合**：`+8.39%`，跟随 `351` 人。
  - **BTC 生息单 (`23-36-18_BTC生息单-利润复投与自动赚币.png`)**：
    - 6 宫格 Bento 特性展示页：`修改参数`、`利润复投`、`暂停/重启`、`欧易独家 自动赚币`、`简单赚币分配`、`动态挂单`，底部深绿主按钮 `创建策略`。
- **手动创建（自选多币动态调仓，见 `03_智能组合`）**：
  - 添加 2–10 个币种，设定每个币种的目标权重百分比（总和必须等于 `100%`）。
  - 调仓模式：
    - **按比例调仓（阈值偏离）**：当币种占比偏离设定阈值（`1% ~ 20%`，默认 `5%`）时触发买卖再平衡。
    - **按时间调仓（定期调仓）**：按固定时间周期（`30分钟 / 1小时 / 4小时 / 1天 / 7天`）检查并恢复目标权重。
  - 专属权益开关：`自动赚币和自动质押`。

---

### 2.5 `04_合约马丁格尔_FuturesMartingale`（13 张截图）

- **核心截图**：
  - `23-24-54_AI策略小助手-短期进取建仓做空.png`
  - `23-24-54_调整杠杆-19倍风险提示.png`
  - `23-24-54_参数-加仓单保证金与最大加仓次数.png`
- **精确字段与交互细节**：
  - **AI 策略小助手**：支持一键填入 `短期进取建仓-做空` / `稳健建仓-做多` 等回测参数。
  - **触发条件**：
    - `跌多少加仓`（做多）/ `涨多少加仓`（做空）：范围 `0.1% - 50%`
    - `单周期止盈目标`：范围 `≥ 0.1%`
  - **保证金与加仓阶梯**：
    - `初次下单保证金`：`≥ 1.25 USDT`
    - `加仓单保证金`：`≥ 1.24 USDT`
    - `最大自动加仓次数`：`0 - 100`（如 `10` 次）
    - `加仓差价倍数` 与 `加仓金额倍数`（高级设置）
  - **风控与资金联动行**：
    - `投入保证金`：实时汇总（如 `135.78 USDT`）
    - `可用`：`0 USDT ⊕`（不足时红字提示 `可用余额不足，建议您充币或划转`）
    - `预估强平价`：如 `71,925.2 USDT`
    - `下次投资使用利润复投 ⓘ`：Switch 开关
    - `自动划转保证金 ⓘ`：Switch 开关，开启后展开输入框：
      - `划转上限 (USDT)`
      - `目标保证金率`：范围 `350% - 2,000%`
    - `止损条件`：支持按价格或按亏损金额/比例止损。
  - **高倍杠杆风险提示**：当杠杆 `> 10x`（如截图中 `19x`）时显示橙色/红色警告：`超过 10x 杠杆会增加强制平仓风险`。

---

### 2.6 `05_现货马丁格尔_SpotMartingale`（10 张截图）

- **核心截图**：
  - `23-27-54_参数-触发条件与单周期止盈目标.png`
  - `23-27-54_预设开始条件-指标触发.png` / `超卖线.png`
- **与合约马丁格尔的关键差异**：
  - `跌多少加仓`：范围 **`0.2% - 50%`**（合约版为 `0.1% - 50%`）
  - `单周期止盈目标`：范围 **`0.2% - 1,000%`**
  - `初次下单金额 (USDT)`：`2 - 50,000 USDT`
  - `加仓单金额 (USDT)`：`≥ 2 USDT`
  - `最大加仓次数`：`0 - 100`
  - **`☑ 预留足够资金` 复选框说明弹窗**：
    - 勾选时：交易周期内所有加仓单所需资金预先冻结占用，确保极端下跌时每一笔加仓单 100% 下单成功。
    - 不勾选时：只占用初始单 + 首个加仓单资金，提升资金利用率，但后续加仓时若账户余额不足会导致加仓失败。
  - **专属权益 (赚币 + 质押)**：如 `SOL/USDT` 显示 `+4.62% 年化 >`。
  - **高级设置**：
    - `预设开始条件`：支持 `立即触发` / `价格触发` / **`RSI 指标触发`**（可设指标周期 `14`、触发条件 `向下穿过 / 向上穿过`、超卖线阈值 `30`、K线周期 `3分钟/15分钟/1小时/4小时/1天`、延迟开始秒数）
    - `加仓设置`：回显 `1.00倍 | 1.00倍 >`（加仓差价倍数 | 加仓金额倍数）
    - `止损条件`：`未设置 >`
  - **底部双按钮栏**：左侧灰底胶囊 `预览`（查看每一档加仓的价格、数量、摊薄后均价、触发跌幅表格）+ 右侧黑底胶囊 `创建策略`。

---

### 2.7 `06_智能套利_SmartArbitrage`（18 张截图）

- **涵盖两种界面形态**：
  1. **新版一键「智能套利」** (`23-36-18_智能套利-套利策略列表.png`, `23-30-45_策略预览-仓位与收益预览.png`)：
     - 顶部市场切换胶囊：`Crypto | Tradfi`
     - **Crypto 模式**：如 `SOL 套利策略`（买入现货 `SOL` 同时质押为 `OKSOL` 赚取质押收益 + 做空 `SOLUSDT 永续` 赚取资金费率并对冲币价波动；支持借入 `USDT` 放大投资）。收益构成标签：`质押奖励 | 资金费收益 | 额外年化加成`。
     - **Tradfi 模式**：
       - `xMU 套利策略`：3日预估总年化 `+12.61%` | 下次参考年化 `0.00%` | `[创建]`
       - `xMRVL 套利策略`：3日预估总年化 `+13.28%` | `[创建]`
       - `xAMD 套利策略`：3日预估总年化 `+12.64%` | `[创建]`
     - **策略预览弹窗**：展示 `仓位预览`（现货买入量 vs 合约做空量）、`收益预览`、`预计开仓盈亏`、`预计开仓价差`、`现货开仓手续费`、`预计投资额分布`。
  2. **专业版「套利下单」双腿工作台** (`23-36-18_智能套利-双腿套利买现货卖合约.png`)：
     - 顶部横幅：`ⓘ 体验新版智能套利 — 全新智能套利策略已上线。一键完成双腿套利，实现低风险、稳定收益。立即体验 →`
     - 组合标题：`[卖] BTCUSDT 永续` + `[买] BTC/USDT` + 右侧 K线按钮
     - 可展开 `深度图表`
     - **左右双栏对称下单面板**：
       - 左栏（合约腿 `BTCUSDT 永续`）：`全仓` | `10.00x` | 方向 `开空 ▾` | `价格 (USDT)` | 4 宫格快捷价：`排队价 / 超价 / 对手价 / 市价` | `设置价格 ▾ 价差率 ⓘ` | `数量 (张)` + `☐ 同数量` | `金额 (USDT)` + `☐ 同金额` | 底部红色按钮 `卖出`
       - 右栏（现货腿 `BTC/USDT`）：方向 `买入 ▾` | `价格` | 4 宫格快捷价：`排队价 / 超价 / 对手价 / 市价` | `数量` | `金额` | 底部按钮。

---

### 2.8 `07_信号策略_SignalBot`（16 张截图）

- **核心截图**：
  - `23-27-54_创建信号策略-交易对与投资额.png`
  - `23-27-54_新增TradingView信号-信号名称.png` / `信号简介.png`
  - `23-27-54_加仓设定-保证金比例.png`
  - `23-27-54_止盈止损-收益率与涨跌幅.png`
- **精确字段与交互细节**：
  - **选择信号**：下拉框（默认 `暂无信号`），点击打开 **选择信号抽屉**：
    - 可选已有信号，或点击 `+ 新增自定义信号` 打开 **新增 TradingView 信号页**：
      - Step 1：`信号名称`（必填，`≤ 50` 字符）+ `信号简介`（选填，`≤ 1500` 字符）
      - Step 2：系统自动生成专属 `Webhook URL` 与 `Alert Message JSON 模板`（含 `action`, `instrument`, `signalToken`, `timestamp`, `maxLag`）
  - **交易对选择**：`⊕ 选择交易对`（最多同时挂载 **10 个合约交易对**；若所选杠杆超过某个交易对的最大可用杠杆，顶部提示 `杠杆超出 X 个交易对的最高可用杠杆倍数`）。
  - **投资额 (保证金)**：输入框 `≥ 50 USDT` + 右侧杠杆 `5x ▾` + 百分比滑杆。
  - **自动赚币**：`+2.81% 年化 >`
  - **高级设置 ⓘ**：
    - `委托类型`：`市价委托` 或 `限价委托`（限价时需设 `距离买一/卖一价差比例`）
    - `加仓设定`：支持 `占用可用保证金比例 (%)`、`初始投资额百分比 (%)`、`固定保证金 (USDT)`、`按 TradingView 信号指定委托量`
    - `止盈止损`：支持按 `收益率 (%)` 或按 `价格涨跌幅 (%)` 设置止盈与止损
    - `最大持仓数量`：限制同时持有的最大子仓位数

---

### 2.9 `08_冰山策略_Iceberg`（23 张截图）

- **核心截图**：
  - `23-30-45_下单-挂单设置与高级设置.png`
  - `23-30-45_帮助-全仓与逐仓模式.png` 至 `仓位模式注意事项.png`（9 张官方全仓/逐仓保证金风控长图）
- **精确字段与交互细节**：
  - **标的**：如 `MUUSDT 永续`（美股代币永续合约）或加密永续/现货。
  - **开平仓切换**：顶部双色胶囊 `开仓`（绿底 `#25A750` 白字）| `平仓`
  - **保证金模式与杠杆**：`全仓 ▾`（或 `逐仓`）| `3x ▾`
  - **挂单设置 ⓘ**：
    - 限价控制模式切换：`价距 (USDT)` / `比例 (%)` / `固定价格`
    - `吃单价优于盘口 (USDT)`（或挂单价距）
    - `吃单限制价 (USDT)`（最高买入限价 / 最低卖出限价，超过则暂停挂单）
    - `单笔委托量 (MU)`（或设置随机浮动区间 `±20%`）
    - `单次挂单数`：`≤ 5`（盘口同时保持的冰山子单笔数，`1 ~ 5`）
    - `委托总量 (MU)`
  - **高级设置 ⓘ**：预设开始条件（立即触发 / 价格触发 / RSI 触发）。
  - **底部双向执行按钮**：
    - 左按钮（绿 `#25A750`）：`开多`（上方显示 `可开多 0 MU`）
    - 右按钮（红 `#E53935`）：`开空`（上方显示 `可开空 0 MU`）

---

### 2.10 `09_时间加权策略_TWAP`（6 张截图）

- **核心截图**：
  - `23-34-06_参数-吃单价优于盘口与限价.png`
  - `23-34-06_策略K线图-下单倒计时.png`
- **精确字段与交互细节**：
  - **开平仓切换**：`开仓` | `平仓`
  - **保证金与杠杆**：`逐仓 ▾` | `3x 3x ▾`
  - **价格区间**（右侧下拉 `价距 ▾` 或 `比例 ▾`）：
    - `吃单价优于盘口 (USDT)`：如 `100`
    - `吃单限制价 (USDT)`：如 `100`
  - **订单的时间间隔与数量**：
    - `下单间隔`：输入框 + **严格校验范围 `5 ~ 120 秒`**（输入 `1` 时弹出深色居中 Toast 报错 `⊗ 时间间隔的范围应为 5 ~ 120 秒` 并在输入框下方显示红字 `时间间隔的范围应为 5 ~ 120 秒`）
    - `单笔数量` 与 `委托总量`
  - **K 线图下单倒计时图层**：在 K 线图上直接渲染下次 TWAP 子单拆单执行的倒计时气泡。

---

### 2.11 `10_定投策略_RecurringBuy`（30 张截图）

- **核心截图**：
  - `23-34-06_定投-精选专区xStockLIVE.png`
  - `23-34-06_定投-科技五巨头组合.png` / `ETF核心指数配置.png`
  - `23-34-06_手动创建-买入标的与每期金额.png`
  - `23-34-06_手动创建-选择账户与自定义扣款顺序.png`
- **顶部双模式胶囊切换**：`精选专区 | 手动创建`
- **路径 A：精选专区（预设组合与热门资产）**：
  - 分类 Tab：`热门币种 | ETF | 大宗商品 | 精选个股`（含 `xStock LIVE` 标识）
  - 热门单资产定投：`BTC-USDT (+8.05%)`、`xTSLA-USDT (+6.03%)`、`xQQQ-USDT (+0.32%)`
  - 精选组合卡片：
    - **科技五巨头组合**：3月回测年化 `+51.52%`（一篮子定投美股科技五巨头代币）
    - **ETF 核心指数配置**：双指数宽基定投组合
- **路径 B：手动创建 (`23-34-06_手动创建-买入标的与每期金额.png`)**：
  - **买入币种**：支持选择 **`1 ~ 20 个币种`**（弹窗含 `数字货币 / 新币种 / TradFi` 分类，多币种时分配各自比例 total=100%，单币种时显示胶囊 `买入 xQQQ ▾`）。
  - **每期金额**：大号字体输入框 `≥ 2 USDT`
  - **定投频率**：7 个圆角矩形按钮（2 行排列）：
    - 第一行：`1 小时` | `4 小时` | `8 小时` | `12 小时`
    - 第二行：`每天`（默认选中，黑框白底）| `每周` | `每月`
  - **首次买入时间**：`立即买入 >`（可切换为自定义具体时间点）
  - **资金账户与扣款卡片**：
    - `其他账户自动补足`：`未开启 >`（开启后支持勾选 `交易账户`、`资金账户`、`简单赚币余额自动赎回`，并可拖拽调整 **自定义扣款顺序**）
    - `可用`：`0 USDT ⊕`
    - `预计定投次数`：`= floor(定投可用总金额 / 每期定投金额)`
  - **高级设置（可展开）**：
    - `买入价格区间`：`最低价 (USDT)` ~ `最高价 (USDT)`（超出价格区间时暂停当期定投扣款）

---

### 2.12 `11_抄底止盈策略_DipBuyTakeProfit`（14 张截图）

- **对应 APK 内部代号**：`dcd_bot` / `Flywheel`（双币理财低买高卖循环套利飞轮策略）
- **核心截图**：
  - `23-24-54_AI推荐-激进型参数与收益预估.png`
  - `23-24-54_买入方式设置.png`
  - `23-24-54_其他条件-最低年化与匹配产品.png`
  - `23-24-54_产品概述-双币理财循环套利.png`
  - `23-24-54_申购-以低价抄底BTC.png`
- **顶部双模式胶囊切换**：`AI 策略 | 手动创建`
- **路径 A：AI 策略**：
  - 提供 `激进型` / `稳健型` 推荐卡片，展示 `回测年化收益率`、目标抄底价、目标止盈价及 K 线图可视化挂钩线。
- **路径 B：手动创建 (`23-24-54_买入方式设置.png`)**：
  - **买入方式**：右上角下拉切换 `跌幅百分比 ▾`（市价下跌百分比抄底）或 `绝对价格`
    - 输入框：`1 %` + 右侧快捷档位按钮：`0.1%` | `1%`（黑框选中）| `5%`
  - **止盈目标**：以抄底成功时的买币价格为止盈目标（或设定最小止盈加价幅度）。
  - **其他条件（可折叠）**：
    - `最低年化收益率 ⓘ`：输入框 `%` + 快捷档位（弹窗说明：`您能接受的最低年化收益率。`）
    - `最长期限 ⓘ`：输入框 `35 天`
  - **匹配到的产品预览卡**（灰底圆角卡 `#F5F5F5`）：
    - `买入价格`：`81,700 USDT`
    - `参考年化`：`+58.02%`（绿字）
    - `期限`：`16 小时`（结算时间 `2026/10/20 16:00`）
  - **底部主按钮**：`下一步` -> 进入 **申购确认页**（输入申购金额 `≥ 10 USDT`，确认「以低价抄底 BTC -> 抄底成功后自动高卖止盈」循环套利）。

---

### 2.13 `12_股票代币现货网格_StockTokenSpotGrid`（10 张截图）与 `13_美股代币合约网格_StockTokenFuturesGrid`（4 张截图）

- **核心截图**：
  - `12_股票代币现货网格_StockTokenSpotGrid/22-58-20_匹配标的与回测年化收益率.png`
  - `13_美股代币合约网格_StockTokenFuturesGrid/22-58-20_策略介绍-美股代币合约网格.png`
  - `13_美股代币合约网格_StockTokenFuturesGrid/22-58-20_风险须知-股票合约风险提示.png`
- **专题落地页结构 (`StockTokenGridLandingActivity`)**：
  - **顶部双 Tab 胶囊**：`股票代币现货网格 | 美股代币合约网格`
  - **Tab 1：股票代币现货网格**：
    - 定位矩阵九宫格：`长期稳健型`（左下高亮绿格）
    - 匹配标的与回测年化列表：
      - `xMU/USDT` (`+12.54%`)
      - `xSOXL/USDT` (`+27.15%`)
      - `xSNDK/USDT` (`+25.67%`)
      - `xNVDA/USDT` (`+5.68%`)
      - `xAMD/USDT` (`+11.77%`)
    - 底部引导横幅：`想要放大您的收益？不妨试试合约网格。循环套利，可加杠杆，牛熊皆宜。>`
    - 科普介绍区：`什么是股票代币现货网格` + 美股科技品牌图标矩阵（Apple, IBM, NVIDIA, Tesla, Meta）。
  - **Tab 2：美股代币合约网格**：
    - 大标题：`拓展美股多元配置，网格全天候赚取利润`
    - Bento 统计卡：
      - 左卡：`中期进取型` + 3×3 九宫格（中右格高亮翠绿 `#25A750`，横轴 `稳健 / 均衡 / 进取`，纵轴 `短期 / 中期 / 长期`）
      - 右上卡：`5,299 累计使用人数` + 头像组
      - 右下卡：`4.07M 累计投入金额`
    - 热门策略推荐（右侧 `自定义参数 >`）：
      - `TSLA/USDT`：回测年化收益率 `+82.10%` | `[创建]`
      - `HOOD/USDT`：回测年化收益率 `+248.64%` | `[创建]`
      - `SPCX/USDT`：回测年化收益率 `+38.20%` | `[创建]`
      - `AMZN/USDT`：回测年化收益率 `+294.39%` | `[创建]`
    - **风险须知弹窗 (`StockTokenRiskNoticeDialog`)**：首次进入或点击创建时弹出股票合约风险提示（说明股票代币合约锚定标的指数价格，不构成对底层美股股票的实际股东投票权或分红所有权，注意杠杆强平风险）。
