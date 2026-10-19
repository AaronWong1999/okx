# OKX Android (`com.okinc.okex.gp` v6.194.0) 全量逆向工程与「策略 Bot」100% 还原代码库

> **目标读者**：希望直接阅读本仓库逆向成果与工程代码，从零复刻整个 **OKX App**（尤其是 **策略交易 Strategy Bots 全部 15 大模块** 的核心算法、数学公式、参数校验、风控强平、UniFFI 跨语言通信与 100% 像素级界面）的工程师与 AI Agent。

---

## 1. 逆向目标与 REA 证据链概览

| 维度 | 逆向实测参数 |
| :--- | :--- |
| **源 APK 路径** | `/Users/aaron/Downloads/android.apk` |
| **应用包名 / 版本** | `com.okinc.okex.gp` · `v6.194.0` (`versionCode = 737202`) |
| **APK SHA-256** | `2b3b4e670997890ebf759d92b5904556b349de2845cfc9835c4a24d7b8addb85` |
| **SDK 版本** | `minSdkVersion = 26` (Android 8.0) · `targetSdkVersion = 35` (Android 15) |
| **逆向工具链** | [morluto/rea (`reverse-engineer-anything`)](https://github.com/morluto/rea) + JADX/Apktool MCP Bridge + Rust UniFFI Symbol Decoder |
| **DEX 字节码规模** | **67 个 DEX 文件** (`classes.dex` ~ `classes67.dex`)，共 **557,944 个类**，其中策略交易相关类 **10,290 个**（含 404 个反混淆 `o/xxx` 外层类） |
| **Rust 核心引擎** | `lib/arm64-v8a/libOKCore.so`（53.02 MB，含 `trading_bot`、`core_trading`、`social_trading`、`retail_trading`、`retail_kline` 等 **24,185 个 UniFFI 符号**，其中 `trading_bot` 独占 **4,962 个符号**） |
| **实机录屏截图** | [`策略BOT截图/`](策略BOT截图/README.md) 共 **15 个分类目录、239 张 `1180×2556` 原生分辨率实机截图** |
| **文件体积优化** | 所有超大型 JSON（原 158MB 会话快照及 4~9MB 符号表）均已按模块拆分为 **`< 550 KB`** 的分片文件，既可直接推送到 GitHub，也方便任何大模型单次完整读取 |

---

## 2. 仓库目录结构导航

```text
okx/
├── README.md                                  # 本总览手册：整包架构 + 15 大策略 Bot 完整复刻指南
├── docs/                                      # 9 篇深度逆向架构、数学公式、风控与逐图 UI 映射文档
│   ├── 00_MASTER_INDEX_AND_APP_ARCHITECTURE.md
│   ├── 01_RUST_LIBOKCORE_UNIFFI_ARCHITECTURE.md
│   ├── 02_GRID_BOTS_COMPLETE_SPEC.md
│   ├── 03_DCA_MARTINGALE_COMPLETE_SPEC.md
│   ├── 04_ARBITRAGE_BOTS_COMPLETE_SPEC.md
│   ├── 05_PORTFOLIO_AND_RECURRING_BUY_SPEC.md
│   ├── 06_SIGNAL_AND_STRUCTURED_OPTIONS_BOTS_SPEC.md
│   ├── 07_EXECUTION_SLICING_TWAP_ICEBERG_SPEC.md
│   └── 08_SCREENSHOT_BY_SCREENSHOT_UI_AND_FLOW_MAPPING.md
├── reconstructed/                             # REA 逆向提取的原始证据、Rust IDL、DEX 反编译代码与 XML 布局
│   ├── rea_evidence/                          # REA 规范化取证 JSON（00_01~07_05，单文件均 < 550 KB）
│   ├── rust_okcore_idl/                       # libOKCore.so 逆向还原的 Rust UniFFI 完整接口 (.rs + schema_chunks/*.json)
│   ├── dex_decompiled/                        # 67 个 DEX 逆向出的 9 大策略模块 .java 源码 + catalog_chunks/*.json
│   └── resources/                             # AndroidManifest.xml + 199 个策略 XML 布局 + 中英文字段映射表
├── 策略BOT截图/                                # 15 个分类目录 (00~14)，共 239 张 1180×2556 实机截图 + manifest.json
├── app/
│   ├── web-simulator/                         # 1:1 像素级可交互 Web 模拟器（含整包 5 大底栏 + 15 大策略 + 实时K线 + 239图对照）
│   └── android-kotlin/                        # Android Kotlin + Jetpack Compose + Rust UniFFI 完整工程参考实现
└── tools/
    ├── run_simulator.py                       # 一键启动本地 1:1 策略 Bot 交互式工作台与截图对照服务器
    └── rea_android_engine/                    # REA CLI 自动化取证流水线与 JADX/Apktool MCP 桥接器
```

---

## 3. OKX 整包四层架构与全部 15 大策略 Bot 全景矩阵

### 3.1 整包四层架构 (`OKX Whole-App Architecture`)

```mermaid
flowchart TD
    subgraph UI["Layer 1: Android UI & Navigation Shell (`com.okinc.okex.gp`)"]
        MainTabs["底部 5 大主导航 Tab\n欧易(Home) | 探索(Explore) | 交易(Trade) | 星球(Social) | 资产(Assets)"]
        TradeTabs["交易顶栏 6 大模式 (`UnifyTradeFragment`)\n闪兑 | 现货 | 合约 | TradFi(xStock) | 策略&跟单 | 期权"]
        BotViews["策略 Bot 视图层 (`com.okinc.tradingbot.impl.*`)\n199 个 XML 布局 + Jetpack Compose + 策略 K 线叠加层 (`botkline`)"]
    end

    subgraph VM["Layer 2: Kotlin Presenter / ViewModel & Repository"]
        Presenters["MVP Presenters & MVVM ViewModels\n`TradingBotApiImpl` · `*CreatePresenter` · `BotDetailActivity`"]
        UniFFIKt["Kotlin UniFFI 绑定层 (`uniffi.trading_bot.*`)\nJNA `UniffiLib` + `FfiConverter*` + `AutoCloseable` 生命周期管理"]
    end

    subgraph Rust["Layer 3: Native Rust Core Engine (`lib/arm64-v8a/libOKCore.so`)"]
        BotCrate["`trading_bot` crate (4,962 Symbols)\n网格/马丁/组合/套利/信号/冰山/TWAP/定投/DCD 状态机与数学风控引擎"]
        CoreCrate["`core_trading` (11,277) + `social_trading` (1,567) + `retail_kline` (2,018)\n统一订单簿、保证金强平计算、带单分润、K 线指标计算"]
    end

    subgraph Net["Layer 4: OKX Algo & Trading Gateways"]
        REST["REST v5 Algo API (`/api/v5/tradingBot/*`)"]
        WS["WebSocket Push (`bot-orders`, `positions`, `tickers`, `books5`)"]
    end

    MainTabs --> TradeTabs --> BotViews --> Presenters --> UniFFIKt --> BotCrate
    BotCrate <--> CoreCrate
    BotCrate --> REST & WS
```

### 3.2 15 个实机截图目录 (`00`~`14`) 与代码/布局/公式一一映射表

| 目录编号与名称 | 截图数 | 策略/功能定位 | 关键 Rust UniFFI Interactor (`libOKCore.so`) | 关键 Android DEX 类 (`reconstructed/dex_decompiled/`) | 核心数学与风控规则 |
| :--- | :---: | :--- | :--- | :--- | :--- |
| **[`00_公共组件`](策略BOT截图/00_公共组件_创建策略与通用功能/README.md)** | 14 | 交易顶栏、策略首页、5大快捷圆钮、`创建策略`抽屉(`全部/现货/合约`)、`我的策略/我的跟单` | `TradingBotHubInteractor` · `SocialTradingSdk` | [`core_bot_framework_and_detail.java`](reconstructed/dex_decompiled/core_bot_framework_and_detail.java) | 统一资产估值折算、运行中策略轮询与分润统计 |
| **[`01_现货网格`](策略BOT截图/01_现货网格_SpotGrid/README.md)** | 16 | AI 3档推荐(`中期均衡/短期激进/长期稳健`)、手动等差/等比、利润复投、自动赚币、随价格上下移、RSI/TV触发 | `SpotGridPlaceOrderInteractor` | [`grid_bots.java`](reconstructed/dex_decompiled/grid_bots.java) | 等差 $\Delta P = \frac{P_U-P_L}{N}$；等比 $r=(P_U/P_L)^{1/N}$；最低投资额动态校验 |
| **[`02_合约网格`](策略BOT截图/02_合约网格_FuturesGrid/README.md)** | 31 | USDT本位 & **币本位(BTCUSD)**、`做多/做空/中性`、杠杆(`1x~100x`)、自动预留保证金、**滑点控制(AI 0.0004%)** | `ContractGridPlaceOrderInteractor` | [`grid_bots.java`](reconstructed/dex_decompiled/grid_bots.java) | 做多/做空/中性双端预估强平价 $P_{\text{liq}}$、实际+预留保证金拆分 |
| **[`03_智能组合`](策略BOT截图/03_智能组合_SmartPortfolio/README.md)** | 14 | 多币种动态调仓(最多10币)、`智能分配/平均分配`、按比例(`5%`)/按时间调仓、自动赚币+质押 | `SmartPortfolioRustRequestBridge` | [`portfolio_and_recurring_bots.java`](reconstructed/dex_decompiled/portfolio_and_recurring_bots.java) | 权重和严格校验 $\sum w_i = 100\%$；最低投资额 $\ge \lceil \frac{10}{\min(w_i)}\times 1.05 \rceil$ |
| **[`04_合约马丁格尔`](策略BOT截图/04_合约马丁格尔_FuturesMartingale/README.md)** | 15 | 做多/做空分批加仓、AI参数、加仓价差/金额倍数、利润复投、**自动划转保证金(350%~2000%)** | `ContractDcaPlaceOrderInteractor` | [`dca_martingale_bots.java`](reconstructed/dex_decompiled/dca_martingale_bots.java) | 几何加仓阶梯 $P_k, A_k$、均价 $\bar{P}_k$、强平价穿透加仓线预警 |
| **[`05_现货马丁格尔`](策略BOT截图/05_现货马丁格尔_SpotMartingale/README.md)** | 16 | 现货逢低分批抄底、RSI触发、`预留足够资金`弹窗、**专属权益(赚币+质押 +4.62% 年化)** | `SpotDcaPlaceOrderInteractor` | [`dca_martingale_bots.java`](reconstructed/dex_decompiled/dca_martingale_bots.java) | 跌多少加仓(`0.2%~50%`)、累计跌幅 $<98\%$、单周期止盈目标价 |
| **[`06_智能套利`](策略BOT截图/06_智能套利_SmartArbitrage/README.md)** | 20 | 模式1：`Crypto/Tradfi` 智能套利(质押+资金费)；模式2：**`套利下单`双腿对称盘口工作台** | `SmartArbitragePlaceOrderInteractor` | [`arbitrage_bots.java`](reconstructed/dex_decompiled/arbitrage_bots.java) | Delta-Neutral 现货与永续资金配比 $\frac{L}{L+1} : \frac{1}{L+1}$、3日综合年化 |
| **[`07_信号策略`](策略BOT截图/07_信号策略_SignalBot/README.md)** | 23 | 自定义 **TradingView Webhook + Alert JSON**、多交易对(最多10个)、委托类型与止盈止损 | `SignalBotCopyCreateInteractor` | [`signal_and_dcd_bots.java`](reconstructed/dex_decompiled/signal_and_dcd_bots.java) | 保证金 $\ge 50\text{ USDT}$、多交易对最高可用杠杆上限校验 |
| **[`08_冰山策略`](策略BOT截图/08_冰山策略_Iceberg/README.md)** | 13 | 大单拆分隐藏挂单、`开仓/平仓`、`全仓/逐仓`(9页说明)、`价距/比例/固定价格`、右侧实时盘口 | `SmartIcebergDetailComposition` | [`twap_and_iceberg_slicing_bots.java`](reconstructed/dex_decompiled/twap_and_iceberg_slicing_bots.java) | 单次并发挂单数 $1 \le K \le 5$、委托总量 $\ge$ 单笔委托量 |
| **[`09_时间加权策略`](策略BOT截图/09_时间加权策略_TWAP/README.md)** | 22 | 按固定时间间隔匀速吃单、K线倒计时气泡、**深色居中 Toast 校验(`5 ~ 120 秒`)** | `TwapRustOrderConfirmFragment` | [`twap_and_iceberg_slicing_bots.java`](reconstructed/dex_decompiled/twap_and_iceberg_slicing_bots.java) | 时间间隔严格限制 $\Delta t \in [5, 120]\text{ s}$、吃单优于盘口价距保护 |
| **[`10_定投策略`](策略BOT截图/10_定投策略_RecurringBuy/README.md)** | 17 | `精选专区`(`xStock LIVE` 科技五巨头/ETF) + `手动创建`(7档频率、自动补足扣款顺序、价格区间) | `RecurringBuyAiLuaOrderService` | [`portfolio_and_recurring_bots.java`](reconstructed/dex_decompiled/portfolio_and_recurring_bots.java) | 单期金额 $\ge 2\text{ USDT}$、预计可定投次数 $\lfloor \text{Avail} / A_{\text{cycle}} \rfloor$ |
| **[`11_抄底止盈策略`](策略BOT截图/11_抄底止盈策略_DipBuyTakeProfit/README.md)** | 12 | 双币低买高卖循环套利、跌幅百分比(`0.1%/1%/5%`)、最低年化与最长期限、自动匹配产品申购弹窗 | `DcdBotInteractor` · `DcdPnlChartView` | [`signal_and_dcd_bots.java`](reconstructed/dex_decompiled/signal_and_dcd_bots.java) | 目标抄底价 $P_{\text{target}} = P_0(1-d)$、期权票息年化匹配 (`+58.02%`) |
| **[`12_股票代币现货网格`](策略BOT截图/12_股票代币现货网格_StockTokenSpotGrid/README.md)** | 11 | 锚定美股代币(`xMU/xSOXL/xSNDK/xNVDA/xAMD`)、3×3 策略风格矩阵(`长期稳健/中期进取`) | `SpotGridPlaceOrderInteractor` | [`grid_bots.java`](reconstructed/dex_decompiled/grid_bots.java) | 美股代币现货网格步长、回测年化与使用人数矩阵映射 |
| **[`13_美股代币合约网格`](策略BOT截图/13_美股代币合约网格_StockTokenFuturesGrid/README.md)** | 7 | 美股永续合约(`TSLA/HOOD/SPCX/AMZN`)、`中期进取型`、**`股票合约风险提示`强制确认弹窗** | `ContractGridPlaceOrderInteractor` | [`grid_bots.java`](reconstructed/dex_decompiled/grid_bots.java) | 美股休市跳空风控确认 (`riskAck`) + 合约网格杠杆强平模型 |
| **[`14_AI策略与AI组合`](策略BOT截图/14_AI策略与AI组合_AI_Strategy/README.md)** | 8 | `桥水基金全天候加密版(+18.14%)`、`存储与储备组合`、`AI数据与储备组合`、**`BTC 生息单` 6宫格** | `AiStrategyInteractor` | [`portfolio_and_recurring_bots.java`](reconstructed/dex_decompiled/portfolio_and_recurring_bots.java) | 官方预设权重自动填入、网格套利 × 双币生息全天候复利模型 |

---

## 4. 快速启动 1:1 交互式模拟器与实机截图对照工作台

本仓库内置了零依赖的本地服务器脚本 [`tools/run_simulator.py`](tools/run_simulator.py)，一键启动后可在浏览器中同时体验：
1. **左侧 1:1 OKX 手机模拟器**：支持底部 5 大主 Tab（`欧易 | 探索 | 交易 | 星球 | 资产`）、交易顶栏 6 大模式（`闪兑 | 现货 | 合约 | TradFi | 策略&跟单 | 期权`）以及全部 15 个策略目录的 1:1 表单、抽屉弹窗、滑点/杠杆/RSI 设置与错误 Toast。
2. **中间实时 K 线与数学/风控检查器**：实时绘制 K 线（`EMA7/14/28`、`MA5`）、网格挂单线、马丁格尔加仓阶梯、预估强平线，并展示当前策略对应的 Rust `libOKCore.so` 符号、DEX 类名、XML 布局与实时数学计算明细。
3. **右侧 239 张原生截图对照画廊**：点击任意截图即可放大查看 `1180×2556` 原图，并将截图中的实测场景参数一键同步到左侧模拟器。

```bash
# 在仓库根目录启动模拟器（默认监听 http://127.0.0.1:8688）
python3 tools/run_simulator.py
```

---

## 5. Android Kotlin + Jetpack Compose + Rust UniFFI 工程指南

位于 [`app/android-kotlin/`](app/android-kotlin/) 的工程提供了可直接用于生产级 Android 客户端开发的完整分层架构：

- **[`AndroidManifest.xml`](app/android-kotlin/app/src/main/AndroidManifest.xml)**：还原 `com.okinc.okex.gp` 应用入口与策略详情 Activity 声明。
- **[`MainActivity.kt`](app/android-kotlin/app/src/main/java/com/okinc/okex/gp/MainActivity.kt)**：实现 OKX 整包 5 大底部导航 Tab 与 6 大交易顶栏模式切换，并在启动时通过 `System.loadLibrary("OKCore")` 加载原生 Rust 引擎。
- **[`OkCoreTradingBotBridge.kt`](app/android-kotlin/app/src/main/java/uniffi/trading_bot/OkCoreTradingBotBridge.kt)**：完整实现 `libOKCore.so` 的 Kotlin UniFFI Interactor 桥接层（涵盖 `ContractGridPlaceOrderInteractor`、`SpotGridPlaceOrderInteractor`、`SmartPortfolioRustRequestBridge`、`MartingaleDcaPlaceOrderInteractor`、`SmartArbitragePlaceOrderInteractor`、`SignalBotCopyCreateInteractor`、`ExecutionSlicingOrderInteractor`、`RecurringAndDcdInteractor`）。
- **[`StrategyBotMathEngine.kt`](app/android-kotlin/app/src/main/java/com/okinc/tradingbot/domain/math/StrategyBotMathEngine.kt)**：100% 还原全部 15 大策略的数学与风控计算函数。
- **[`StrategyBotWorkstationScreen.kt`](app/android-kotlin/app/src/main/java/com/okinc/tradingbot/ui/screens/StrategyBotWorkstationScreen.kt) & [`AllStrategyBotForms.kt`](app/android-kotlin/app/src/main/java/com/okinc/tradingbot/ui/screens/AllStrategyBotForms.kt)**：Jetpack Compose 策略工作台、K 线网格/强平叠加画布 (`BotKlineOverlayCanvas`) 与全部 15 大策略的参数面板。

---

## 6. 深度逆向文档索引 (`docs/00` ~ `08`)

1. **[`docs/00_MASTER_INDEX_AND_APP_ARCHITECTURE.md`](docs/00_MASTER_INDEX_AND_APP_ARCHITECTURE.md)**：APK 身份指纹、整包模块划分与混合技术栈总览。
2. **[`docs/01_RUST_LIBOKCORE_UNIFFI_ARCHITECTURE.md`](docs/01_RUST_LIBOKCORE_UNIFFI_ARCHITECTURE.md)**：`libOKCore.so` 24,185 个符号拆解、UniFFI JNA 通信机制与 `release()` 防内存泄漏设计。
3. **[`docs/02_GRID_BOTS_COMPLETE_SPEC.md`](docs/02_GRID_BOTS_COMPLETE_SPEC.md)**：现货网格、合约网格（USDT/币本位）、天地网格、股票代币网格完整规范。
4. **[`docs/03_DCA_MARTINGALE_COMPLETE_SPEC.md`](docs/03_DCA_MARTINGALE_COMPLETE_SPEC.md)**：合约马丁格尔与现货马丁格尔几何加仓阶梯、自动划转保证金与强平公式。
5. **[`docs/04_ARBITRAGE_BOTS_COMPLETE_SPEC.md`](docs/04_ARBITRAGE_BOTS_COMPLETE_SPEC.md)**：智能套利（Crypto/TradFi）与双腿套利下单对称订单簿规范。
6. **[`docs/05_PORTFOLIO_AND_RECURRING_BUY_SPEC.md`](docs/05_PORTFOLIO_AND_RECURRING_BUY_SPEC.md)**：智能组合、AI 组合、BTC 生息单与定投策略（含 xStock LIVE）规范。
7. **[`docs/06_SIGNAL_AND_STRUCTURED_OPTIONS_BOTS_SPEC.md`](docs/06_SIGNAL_AND_STRUCTURED_OPTIONS_BOTS_SPEC.md)**：TradingView 信号策略与抄底止盈（双币 DCD 飞轮）规范。
8. **[`docs/07_EXECUTION_SLICING_TWAP_ICEBERG_SPEC.md`](docs/07_EXECUTION_SLICING_TWAP_ICEBERG_SPEC.md)**：冰山策略与时间加权（TWAP）大单拆分算法规范。
9. **[`docs/08_SCREENSHOT_BY_SCREENSHOT_UI_AND_FLOW_MAPPING.md`](docs/08_SCREENSHOT_BY_SCREENSHOT_UI_AND_FLOW_MAPPING.md)**：239 张实机录屏截图逐张 UI 状态、交互流程与代码映射表。
