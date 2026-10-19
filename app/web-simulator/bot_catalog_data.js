/**
 * OKX v6.194.0 — 15 Screenshot Folders & Strategy Bot Catalog Metadata
 * =====================================================================
 * Maps every folder (00..14) in `/策略BOT截图/` to its exact UI state,
 * default parameters from the screen recordings, Rust `libOKCore.so`
 * UniFFI Interactor, and decompiled Android DEX Activity/Fragment.
 */

(function (global) {
  "use strict";

  const BOT_FOLDERS = [
    {
      id: "00_公共组件_创建策略与通用功能",
      shortTitle: "00 公共组件·创建策略",
      navTitle: "策略&跟单",
      botType: "hub_common",
      defaultInst: "BTC-USDT",
      clock: "22:56",
      rustSymbol: "TradingBotMarketplaceInteractor",
      dexClass: "com.okinc.tradingbot.marketplace.TradingBotMarketFragment",
      xmlLayout: "bots_fragment_marketplace.xml / bots_dialog_create_strategy_selector.xml",
      desc: "资产/交易页「策略&跟单」总入口、快捷金刚区、股票代币现货网格推荐流、以及「创建策略」底部选择抽屉（含全部/现货/合约分类）。"
    },
    {
      id: "01_现货网格_SpotGrid",
      shortTitle: "01 现货网格",
      navTitle: "现货网格",
      botType: "spot_grid",
      defaultInst: "BTC-USDT",
      clock: "22:56",
      rustSymbol: "SpotGridPlaceOrderInteractor",
      dexClass: "com.okinc.tradingbot.grid.spot.SpotGridCreateFragment",
      xmlLayout: "bots_fragment_spot_grid_manual.xml",
      desc: "在指定价格区间内按等差/等比铺设网格低买高卖。含利润复投、自动赚币(+2.79%年化)、简单赚币分配、网格随价格上移/下移、预设开始/停止条件(价格/RSI-14)。"
    },
    {
      id: "02_合约网格_FuturesGrid",
      shortTitle: "02 合约网格 (U本位/币本位)",
      navTitle: "合约网格",
      botType: "contract_grid",
      defaultInst: "BTC-USDT-SWAP",
      clock: "22:57",
      rustSymbol: "ContractGridPlaceOrderInteractor",
      dexClass: "com.okinc.tradingbot.grid.contract.ContractGridCreateFragment",
      xmlLayout: "bots_fragment_contract_grid_manual.xml",
      desc: "支持 USDT 永续与 BTCUSD 币本位(CM)永续。含 1x-100x 杠杆选择器、保证金门槛校验(>339.66 USDT)、自动预留保证金、预估强平价、滑点控制设置(0.0004%)、止盈止损。"
    },
    {
      id: "03_智能组合_SmartPortfolio",
      shortTitle: "03 智能组合",
      navTitle: "智能组合",
      botType: "smart_portfolio",
      defaultInst: "BTC-USDT",
      clock: "23:36",
      rustSymbol: "SmartPortfolioPlaceOrderInteractor",
      dexClass: "com.okinc.tradingbot.portfolio.SmartPortfolioCreateActivity",
      xmlLayout: "bots_activity_smart_portfolio.xml",
      desc: "多币种恒定比例动态调仓。支持 AI 策略（预设组合）与手动创建（自选 2-10 币种权重=100%），支持按偏离阈值(5%)或定期检查触发自动再平衡，含自动赚币和自动质押。"
    },
    {
      id: "04_合约马丁格尔_FuturesMartingale",
      shortTitle: "04 合约马丁格尔",
      navTitle: "合约马丁格尔",
      botType: "contract_dca",
      defaultInst: "BTC-USDT-SWAP",
      clock: "23:27",
      rustSymbol: "ContractDcaPlaceOrderInteractor",
      dexClass: "com.okinc.tradingbot.dca.contract.ContractDcaFragment",
      xmlLayout: "bots_fragment_contract_dca.xml",
      desc: "梯子式分批加仓摊薄成本，达单周期止盈目标即平仓。含 AI 策略小助手(短期进取建仓-做空)、19x 高倍杠杆强平警告、自动划转保证金(划转上限 + 目标保证金率 350%-2,000%)。"
    },
    {
      id: "05_现货马丁格尔_SpotMartingale",
      shortTitle: "05 现货马丁格尔",
      navTitle: "现货马丁格尔",
      botType: "spot_dca",
      defaultInst: "SOL-USDT",
      clock: "23:28",
      rustSymbol: "SpotDcaPlaceOrderInteractor",
      dexClass: "com.okinc.tradingbot.dca.spot.SpotDcaFragment",
      xmlLayout: "bots_fragment_spot_dca.xml",
      desc: "现货分批抄底加仓（跌多少加仓 0.2%-50%，单周期止盈 0.2%-1,000%）。含「预留足够资金」弹窗说明、专属权益(赚币+质押 +4.62%年化)、RSI-14 超卖线触发与委托阶梯预览。"
    },
    {
      id: "06_智能套利_SmartArbitrage",
      shortTitle: "06 智能套利 & 套利下单",
      navTitle: "智能套利",
      botType: "smart_arbitrage",
      defaultInst: "BTC-USDT-SWAP",
      clock: "23:38",
      rustSymbol: "SmartArbitragePlaceOrderInteractor",
      dexClass: "com.okinc.tradingbot.arbitrage.SmartArbFragment",
      xmlLayout: "bots_fragment_smart_arbitrage.xml / bots_fragment_dual_leg_arb.xml",
      desc: "涵盖新版一键「智能套利」（Crypto SOL质押OKSOL+资金费 / Tradfi xMU/xMRVL/xAMD 套利 + 策略预览弹窗）与专业版「套利下单」左右双腿对称工作台（排队价/超价/对手价/市价）。"
    },
    {
      id: "07_信号策略_SignalBot",
      shortTitle: "07 信号策略",
      navTitle: "信号策略",
      botType: "signal_bot",
      defaultInst: "BTC-USDT-SWAP",
      clock: "23:29",
      rustSymbol: "SignalBotPlaceOrderInteractor",
      dexClass: "com.okinc.tradingbot.signal.SignalBotCreateFragment",
      xmlLayout: "bots_fragment_signal_bot_create.xml",
      desc: "接入 TradingView 警报信号自动下单。支持新增自定义信号(名称≤50字、简介≤1500字)、最多 10 个交易对联动、加仓设定(保证金比例/固定保证金)、止盈止损(收益率/涨跌幅)。"
    },
    {
      id: "08_冰山策略_Iceberg",
      shortTitle: "08 冰山策略",
      navTitle: "冰山策略",
      botType: "iceberg",
      defaultInst: "MU-USDT-SWAP",
      clock: "23:32",
      rustSymbol: "IcebergPlaceOrderInteractor",
      dexClass: "com.okinc.tradingbot.iceberg.IcebergOrderFragment",
      xmlLayout: "bots_fragment_iceberg_order.xml",
      desc: "大单拆分为多笔小单隐藏真实委托量。含开仓/平仓切换、全仓/逐仓模式(附 9 页官方保证金风控教程)、单笔委托量、单次挂单数(≤5)、委托总量、吃单价优于盘口与限制价。"
    },
    {
      id: "09_时间加权策略_TWAP",
      shortTitle: "09 时间加权策略 (TWAP)",
      navTitle: "时间加权策略",
      botType: "twap",
      defaultInst: "MU-USDT-SWAP",
      clock: "23:34",
      rustSymbol: "TwapPlaceOrderInteractor",
      dexClass: "com.okinc.tradingbot.twap.TwapOrderFragment",
      xmlLayout: "bots_fragment_twap_order.xml",
      desc: "按固定时间间隔匀速拆分执行大额委托。含价格区间(价距/比例)、吃单价优于盘口(USDT)、吃单限制价(USDT)、下单间隔严格校验(5 ~ 120 秒居中弹窗提示)与 K 线倒计时图层。"
    },
    {
      id: "10_定投策略_RecurringBuy",
      shortTitle: "10 定投策略",
      navTitle: "定投策略",
      botType: "recurring_buy",
      defaultInst: "xQQQ-USDT",
      clock: "23:35",
      rustSymbol: "RecurringBuyPlaceOrderInteractor",
      dexClass: "com.okinc.tradingbot.recurring.RecurringBuyActivity",
      xmlLayout: "bots_activity_recurring_buy.xml",
      desc: "涵盖「精选专区」（xStock LIVE、热门币种/ETF/大宗商品/精选个股、科技五巨头组合 +51.52%、ETF核心指数配置）与「手动创建」（1-20币种、每期金额≥2 USDT、7档频率、其他账户自动补足）。"
    },
    {
      id: "11_抄底止盈策略_DipBuyTakeProfit",
      shortTitle: "11 抄底止盈策略",
      navTitle: "抄底止盈策略",
      botType: "dcd_dip_tp",
      defaultInst: "BTC-USDT",
      clock: "23:25",
      rustSymbol: "DcdFlywheelInteractor",
      dexClass: "com.okinc.tradingbot.dcd.DcdBotCreateActivity",
      xmlLayout: "bots_activity_dcd_bot.xml",
      desc: "基于双币理财低买高卖循环套利。支持 AI 策略与手动创建：设定跌幅百分比抄底(0.1%/1%/5%)、最低年化收益率、最长期限(35天)，自动匹配高年化产品(81,700 USDT / +58.02% / 16小时)。"
    },
    {
      id: "12_股票代币现货网格_StockTokenSpotGrid",
      shortTitle: "12 股票代币现货网格",
      navTitle: "股票代币现货网格",
      botType: "stock_spot_grid",
      defaultInst: "xSNDK-USDT",
      clock: "22:58",
      rustSymbol: "StockTokenSpotGridInteractor",
      dexClass: "com.okinc.tradingbot.grid.stock.StockTokenGridLandingActivity",
      xmlLayout: "bots_activity_stock_token_grid.xml",
      desc: "锚定美股代币波动特性的长期稳健型现货网格专题页。展示 xMU(+12.54%)、xSOXL(+27.15%)、xSNDK(+25.67%)、xNVDA(+5.68%)、xAMD(+11.77%) 回测年化与一键创建入口。"
    },
    {
      id: "13_美股代币合约网格_StockTokenFuturesGrid",
      shortTitle: "13 美股代币合约网格",
      navTitle: "美股代币合约网格",
      botType: "stock_futures_grid",
      defaultInst: "TSLA-USDT-SWAP",
      clock: "22:58",
      rustSymbol: "StockTokenContractGridInteractor",
      dexClass: "com.okinc.tradingbot.grid.stock.StockTokenContractGridFragment",
      xmlLayout: "bots_dialog_stock_contract_risk.xml",
      desc: "拓展美股多元配置的「中期进取型」合约网格专题页（累计 5,299 人使用、投入 4.07M）。含 TSLA(+82.10%)、HOOD(+248.64%)、SPCX(+38.20%)、AMZN(+294.39%) 及股票合约风险须知弹窗。"
    },
    {
      id: "14_AI策略与AI组合_AI_Strategy",
      shortTitle: "14 AI策略与BTC生息单",
      navTitle: "AI 策略与组合",
      botType: "ai_strategy",
      defaultInst: "BTC-USDT",
      clock: "23:37",
      rustSymbol: "AiPortfolioPresetInteractor",
      dexClass: "com.okinc.tradingbot.portfolio.AiPortfolioPresetDetailActivity",
      xmlLayout: "bots_activity_ai_portfolio_detail.xml / bots_activity_btc_yield_grid.xml",
      desc: "涵盖官方 AI 精选组合（桥水基金全天候加密版 +18.14%、存储与储备组合 xMU/xBMNR/xREN 偏离5%自动再平衡、AI数据与储备组合 +8.39%）以及「BTC 生息单」六宫格特性展示页。"
    }
  ];

  global.OKX_BOT_CATALOG = {
    BOT_FOLDERS
  };
})(typeof window !== "undefined" ? window : globalThis);
