# OKX 策略 BOT 界面截图全集

共 **15 个目录**、**239 张截图**（1180×2556 原生分辨率），
全部来自 2026-10-19 的 7 段 OKX App 录屏。

## BOT 一览

| # | 目录 | BOT | 截图 | 策略类型 |
|---|---|---|---|---|
| 1 | [`00_公共组件_创建策略与通用功能/`](00_公共组件_创建策略与通用功能/README.md) | 公共组件：创建策略入口与策略中心 | 36 | 入口 / 通用 |
| 2 | [`01_现货网格_SpotGrid/`](01_现货网格_SpotGrid/README.md) | 现货网格 (Spot Grid) | 16 | 网格 |
| 3 | [`02_合约网格_FuturesGrid/`](02_合约网格_FuturesGrid/README.md) | 合约网格 (Futures Grid) | 28 | 网格 |
| 4 | [`03_智能组合_SmartPortfolio/`](03_智能组合_SmartPortfolio/README.md) | 智能组合 (Smart Portfolio) | 5 | 组合 / 调仓 |
| 5 | [`04_合约马丁格尔_FuturesMartingale/`](04_合约马丁格尔_FuturesMartingale/README.md) | 合约马丁格尔 (Futures Martingale / DCA) | 13 | 马丁格尔 / DCA |
| 6 | [`05_现货马丁格尔_SpotMartingale/`](05_现货马丁格尔_SpotMartingale/README.md) | 现货马丁格尔 (Spot Martingale / DCA) | 10 | 马丁格尔 / DCA |
| 7 | [`06_智能套利_SmartArbitrage/`](06_智能套利_SmartArbitrage/README.md) | 智能套利 (Smart Arbitrage) | 18 | 套利 |
| 8 | [`07_信号策略_SignalBot/`](07_信号策略_SignalBot/README.md) | 信号策略 (Signal Bot) | 16 | 信号跟单 |
| 9 | [`08_冰山策略_Iceberg/`](08_冰山策略_Iceberg/README.md) | 冰山策略 (Iceberg) | 23 | 拆单执行 |
| 10 | [`09_时间加权策略_TWAP/`](09_时间加权策略_TWAP/README.md) | 时间加权策略 (TWAP) | 6 | 拆单执行 |
| 11 | [`10_定投策略_RecurringBuy/`](10_定投策略_RecurringBuy/README.md) | 定投策略 (Recurring Buy) | 30 | 定投 |
| 12 | [`11_抄底止盈策略_DipBuyTakeProfit/`](11_抄底止盈策略_DipBuyTakeProfit/README.md) | 抄底止盈策略 (Dip-Buy Take-Profit) | 14 | 双币理财套利 |
| 13 | [`12_股票代币现货网格_StockTokenSpotGrid/`](12_股票代币现货网格_StockTokenSpotGrid/README.md) | 股票代币现货网格 (Stock-Token Spot Grid) | 10 | 网格（股票代币） |
| 14 | [`13_美股代币合约网格_StockTokenFuturesGrid/`](13_美股代币合约网格_StockTokenFuturesGrid/README.md) | 美股代币合约网格 (Stock-Token Futures Grid) | 4 | 网格（股票代币） |
| 15 | [`14_AI策略与AI组合_AI_Strategy/`](14_AI策略与AI组合_AI_Strategy/README.md) | AI 策略与 AI 组合 (AI Strategy & Portfolio) | 10 | AI 策略 / 预设组合 |

## 按功能归类

- **网格类** — 现货网格、合约网格、股票代币现货网格、美股代币合约网格：在价格区间内自动铺网格吃价差。
- **摊薄成本类** — 现货马丁格尔、合约马丁格尔：下跌加仓摊平成本，达到单周期止盈目标即平仓。
- **资产配置类** — 智能组合、AI 策略与 AI 组合、定投策略：多币动态调仓或定期定额买入。
- **价差/对冲类** — 智能套利、抄底止盈策略：现货 + 合约双腿对冲，或双币理财低买高卖。
- **拆单执行类** — 冰山策略、时间加权策略(TWAP)：把大单拆小，降低市场冲击。
- **信号跟随类** — 信号策略：接 TradingView 警报自动下单。

## 阅读顺序建议

1. 先看 `00_公共组件_创建策略与通用功能/`——理解整个策略交易入口与 bot 目录结构。
2. 再挑一个目标 bot 深入看它的 `README.md`（含策略原理、关键参数、操作流程）。
3. 对比同族 bot 的参数面板差异（如现货网格 vs 合约网格、现货马丁格尔 vs 合约马丁格尔）。

## 文件命名规则

```
<视频时间戳>_<界面名称>.png
```

例：`22-53-19_参数面板-完整参数总览.png` = 22:53:19 那段录屏里的「参数面板-完整参数总览」界面。

`manifest.json` 是机器可读的索引，含每张图的 bot 归属、界面标签、来源视频与源帧号。

## 复现方式

```bash
python3 tools/extract_candidates.py   # 低分辨率抽帧 + 感知哈希去重
python3 tools/extract_full.py         # 全分辨率重抽
swiftc -O tools/ocr.swift -o tools/ocr # macOS Vision OCR
python3 tools/curate.py               # 按 bot 归档
python3 tools/write_docs.py            # 生成本 README
```

---

> 录屏源文件（*.MP4，约 1 GB）体积过大，已在 `.gitignore` 中排除，不随仓库上传。
