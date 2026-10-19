/**
 * OKX v6.194.0 Whole-App Shell & Non-Bot Trading Modes (1:1 Reconstructed Views)
 * ==============================================================================
 * Provides the complete OKX 5-Tab Bottom Navigation (`欧易 | 探索 | 交易 | 星球 | 资产`)
 * and 6-Tab Trade Top Bar (`闪兑 | 现货 | 合约 | TradFi | 策略&跟单 | 期权`),
 * reconstructed from `com.okinc.okex.gp` (`MainActivity`, `UnifyTradeFragment`,
 * `MarketHomeFragment`, `AssetsOverviewFragment`, `SocialCopyTradingFragment`).
 */
(function (global) {
  "use strict";

  const TOP_TRADE_TABS = ["闪兑", "现货", "合约", "TradFi", "策略&跟单", "期权"];
  const BOTTOM_MAIN_TABS = [
    { id: "home", icon: "◎", label: "欧易" },
    { id: "explore", icon: "🧭", label: "探索" },
    { id: "trade", icon: "⇄", label: "交易" },
    { id: "planet", icon: "🌐", label: "星球" },
    { id: "assets", icon: "💼", label: "资产" }
  ];

  function renderTopTradeBar(activeTradeTab) {
    return (
      '<div class="okx-trade-top-tabs">' +
      TOP_TRADE_TABS.map((t) =>
        '<span class="' + (t === activeTradeTab ? "active" : "") +
        '" data-act="trade-tab" data-tab="' + t + '">' + t + "</span>"
      ).join("") +
      "</div>"
    );
  }

  function renderBottomAppTabBar(activeAppTab) {
    return (
      '<nav class="okx-bottom-tabbar">' +
      BOTTOM_MAIN_TABS.map((t) =>
        '<div class="tab-item' + (t.id === activeAppTab ? " active" : "") +
        '" data-act="app-tab" data-tab="' + t.id + '">' +
        '<span style="font-size:15px">' + t.icon + "</span>" +
        "<span>" + t.label + "</span></div>"
      ).join("") +
      "</nav>"
    );
  }

  function renderHomeView(M) {
    const tickers = ["BTC-USDT", "ETH-USDT-SWAP", "SOL-USDT", "xNVDA-USDT", "xMU-USDT", "TSLA-USDT-SWAP"];
    return (
      '<header class="okx-nav-header">' +
      '<div style="display:flex;align-items:center;gap:8px"><span style="font-weight:900;font-size:16px">OKX 交易所</span><span class="badge-soft">Web3 钱包</span></div>' +
      '<div class="okx-nav-icons"><span>🔍</span><span>🎧</span><span>🔔</span></div></header>' +
      '<div class="phone-body">' +
      '<div style="padding:8px 0 12px"><div class="okx-kv-label">总资产估值 (USD)</div>' +
      '<div style="font-size:28px;font-weight:900" class="tabular">$12,480.50</div>' +
      '<div class="price-up" style="font-size:12.5px;font-weight:600">+$284.20 (+2.33%) 今日收益</div></div>' +
      '<div class="okx-circle-grid-4">' +
      '<div class="okx-circle-item" data-act="app-tab" data-tab="assets"><div class="okx-circle-icon black">↓</div><span>充币/买币</span></div>' +
      '<div class="okx-circle-item" data-act="trade-tab" data-tab="策略&跟单"><div class="okx-circle-icon">🤖</div><span>策略交易</span></div>' +
      '<div class="okx-circle-item" data-act="trade-tab" data-tab="TradFi"><div class="okx-circle-icon">📈</div><span>美股代币</span></div>' +
      '<div class="okx-circle-item" data-act="app-tab" data-tab="explore"><div class="okx-circle-icon">💰</div><span>简单赚币</span></div>' +
      "</div>" +
      '<div class="okx-border-card" data-act="trade-tab" data-tab="策略&跟单">' +
      '<div style="display:flex;justify-content:space-between;align-items:center"><b>策略 Bot 智能工作台</b><span class="badge-lime">15 大策略</span></div>' +
      '<div style="color:var(--okx-text-secondary);font-size:12px;margin-top:4px">网格 · 马丁格尔 · 智能组合 · 套利 · 信号 · 冰山 · TWAP · 定投 · 抄底止盈</div></div>' +
      '<div class="okx-field-header" style="margin-top:10px"><span>热门行情 (Crypto &amp; xStock)</span><span>24h 涨跌</span></div>' +
      tickers.map((id) => {
        const inst = M.INSTRUMENTS[id];
        if (!inst) return "";
        const up = inst.change24hPct >= 0;
        return (
          '<div class="okx-kv-row" style="cursor:pointer" data-act="pick-inst" data-folder="01_现货网格_SpotGrid" data-id="' + id + '">' +
          '<div><b>' + inst.displayName + '</b><div style="font-size:11px;color:var(--okx-text-secondary)">' + inst.assetClass + "</div></div>" +
          '<div style="text-align:right"><div class="tabular" style="font-weight:700">' + M.formatNum(inst.lastPx, 2) + "</div>" +
          '<span class="' + (up ? "price-up" : "price-down") + '" style="font-size:12px">' + (up ? "+" : "") + inst.change24hPct.toFixed(2) + "%</span></div></div>"
        );
      }).join("") +
      "</div>"
    );
  }

  function renderExploreView() {
    return (
      '<header class="okx-nav-header"><span style="font-weight:800;font-size:17px">探索 · 赚币与结构化产品</span><span>🔍</span></header>' +
      '<div class="phone-body">' +
      '<div class="okx-border-card" data-act="folder" data-id="11_抄底止盈策略_DipBuyTakeProfit">' +
      '<div style="display:flex;justify-content:space-between"><b>抄底止盈 (双币循环套利)</b><span class="price-up" style="font-weight:800">+58.02% 年化</span></div>' +
      '<div style="font-size:12px;color:var(--okx-text-secondary);margin-top:4px">目标价 81,700 USDT 低价抄底 BTC，未成交立享高额票息</div></div>' +
      '<div class="okx-border-card" data-act="folder" data-id="01_现货网格_SpotGrid">' +
      '<div style="display:flex;justify-content:space-between"><b>简单赚币 (策略闲置资金自动生息)</b><span class="price-up" style="font-weight:800">+2.79% ~ +4.62%</span></div>' +
      '<div style="font-size:12px;color:var(--okx-text-secondary);margin-top:4px">网格/马丁格尔/信号策略未挂单资金自动划入余币宝复利生息</div></div>' +
      '<div class="okx-border-card" data-act="folder" data-id="06_智能套利_SmartArbitrage">' +
      '<div style="display:flex;justify-content:space-between"><b>SOL 质押 + 资金费率套利</b><span class="price-up" style="font-weight:800">+20.75% 综合年化</span></div>' +
      '<div style="font-size:12px;color:var(--okx-text-secondary);margin-top:4px">现货买入质押 +6.85% + 永续做空吃资金费 +11.40%</div></div>' +
      "</div>"
    );
  }

  function renderPlanetView() {
    return (
      '<header class="okx-nav-header"><span style="font-weight:800;font-size:17px">星球 · 策略跟单社区 (`social_trading`)</span><span>🔔</span></header>' +
      '<div class="phone-body">' +
      '<div class="okx-border-card" data-act="folder" data-id="14_AI策略与AI组合_AI_Strategy">' +
      '<div style="display:flex;justify-content:space-between"><b>桥水基金全天候加密版</b><span class="price-up">+18.14%</span></div>' +
      '<div style="font-size:12px;color:var(--okx-text-secondary);margin-top:4px">官方 AI 组合 · BTC / ETH / SOL / PAXG 动态再平衡</div></div>' +
      '<div class="okx-border-card" data-act="folder" data-id="12_股票代币现货网格_StockTokenSpotGrid">' +
      '<div style="display:flex;justify-content:space-between"><b>xStock 半导体网格带单王 (xMU / xSOXL)</b><span class="price-up">+27.15%</span></div>' +
      '<div style="font-size:12px;color:var(--okx-text-secondary);margin-top:4px">3,591 人跟单 · 利润分成 10% (`SocialTradingSdk`)</div></div>' +
      '<div class="okx-border-card" data-act="folder" data-id="07_信号策略_SignalBot">' +
      '<div style="display:flex;justify-content:space-between"><b>TradingView 量化信号源广场</b><span class="badge-lime">Webhook</span></div>' +
      '<div style="font-size:12px;color:var(--okx-text-secondary);margin-top:4px">支持自定义 Alert JSON 信号触发多交易对自动开平仓</div></div>' +
      "</div>"
    );
  }

  function renderAssetsView(runningBots) {
    return (
      '<header class="okx-nav-header"><span style="font-weight:800;font-size:17px">资产总览 (`AssetsOverview`)</span><span>⇄ 划转</span></header>' +
      '<div class="phone-body">' +
      '<div style="padding:6px 0 12px"><div class="okx-kv-label">账户总资产 (USD)</div>' +
      '<div style="font-size:26px;font-weight:900" class="tabular">12,480.50 USD</div></div>' +
      '<div class="okx-kv-row"><span>资金账户 (`Funding`)</span><b class="tabular">3,200.00 USDT</b></div>' +
      '<div class="okx-kv-row"><span>交易账户 (`Unified Trading`)</span><b class="tabular">6,850.50 USDT</b></div>' +
      '<div class="okx-kv-row"><span>策略账户 (`Trading Bot Isolated`)</span><b class="tabular">2,430.00 USDT</b></div>' +
      '<div class="okx-field-header" style="margin-top:12px"><span>策略机器人占用明细 (' + runningBots.length + ')</span></div>' +
      runningBots.map((b) =>
        '<div class="okx-kv-row"><span>' + b.name + ' <span class="badge-soft">' + b.status + '</span></span>' +
        '<span class="' + (b.pnl >= 0 ? "price-up" : "price-down") + ' tabular">' + (b.pnl >= 0 ? "+" : "") + b.pnl.toFixed(2) + " USDT</span></div>"
      ).join("") +
      "</div>"
    );
  }

  function renderNonBotTradeMode(tradeTab, inst, M) {
    return (
      renderTopTradeBar(tradeTab) +
      '<div class="okx-pair-strip"><span class="okx-pair-name">' + inst.displayName + " ▾</span>" +
      '<span class="price-up tabular">' + M.formatNum(inst.lastPx, 2) + "</span></div>" +
      '<div class="phone-body">' +
      '<div class="okx-border-card">' +
      '<b>当前处于「' + tradeTab + '」标准交易面板 (`uniffi::core_trading`)</b>' +
      '<p style="font-size:12px;color:var(--okx-text-secondary);margin-top:6px">OKX 统一交易引擎 (`libOKCore.so` 中的 `core_trading` crate) 与策略 Bot 共享底层行情订阅、盘口深度、保证金计算与下单通道。</p>' +
      '<button class="okx-black-pill-btn" style="margin-top:10px;height:38px" data-act="trade-tab" data-tab="策略&跟单">切回「策略&amp;跟单」模块 →</button>' +
      "</div></div>"
    );
  }

  global.OkxWholeAppViews = {
    renderTopTradeBar,
    renderBottomAppTabBar,
    renderHomeView,
    renderExploreView,
    renderPlanetView,
    renderAssetsView,
    renderNonBotTradeMode
  };
})(window);
