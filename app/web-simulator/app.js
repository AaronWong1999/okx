/**
 * OKX v6.194.0 strategy-bot workstation.
 * Wires the phone UI, math engine, and the 239 recording screenshots.
 */
(function () {
  "use strict";

  const M = window.OkxBotMathEngine;
  const FOLDERS = window.OKX_BOT_CATALOG.BOT_FOLDERS;
  const SHOTS = (window.OKX_SCREENSHOTS_MANIFEST && window.OKX_SCREENSHOTS_MANIFEST.shots) || [];

  const CREATE_MENU = [
    { title: "现货网格", desc: "低买高卖，震荡向上行情", folder: "01_现货网格_SpotGrid", badge: "" },
    { title: "合约网格", desc: "循环套利，可加杠杆，牛熊皆宜", folder: "02_合约网格_FuturesGrid", badge: "" },
    { title: "合约网格 (币本位)", desc: "投币赚币，自动赚币复利，获取更高收益", folder: "02_合约网格_FuturesGrid", badge: "New", instId: "BTC-USD-SWAP" },
    { title: "智能组合", desc: "多币组合，动态调仓", folder: "03_智能组合_SmartPortfolio", badge: "New" },
    { title: "抄底止盈策略", desc: "双币低买高卖，循环套利", folder: "11_抄底止盈策略_DipBuyTakeProfit", group: "热门策略" },
    { title: "合约马丁格尔", desc: "分批加仓，单周期止盈", folder: "04_合约马丁格尔_FuturesMartingale", group: "热门策略" },
    { title: "现货马丁格尔", desc: "现货抄底摊薄成本", folder: "05_现货马丁格尔_SpotMartingale", group: "热门策略" },
    { title: "信号策略", desc: "TradingView 警报自动下单", folder: "07_信号策略_SignalBot", group: "热门策略" },
    { title: "智能套利", desc: "现货 + 合约资金费", folder: "06_智能套利_SmartArbitrage", group: "热门策略", badge: "Tradfi" },
    { title: "冰山策略", desc: "大单拆成多笔小单", folder: "08_冰山策略_Iceberg", group: "大单拆分" },
    { title: "时间加权策略", desc: "按时间间隔匀速成交", folder: "09_时间加权策略_TWAP", group: "大单拆分" },
    { title: "定投策略", desc: "定期定额买入", folder: "10_定投策略_RecurringBuy", group: "进阶策略" },
    { title: "股票代币现货网格", desc: "锚定股票代币波动", folder: "12_股票代币现货网格_StockTokenSpotGrid", badge: "New" },
    { title: "美股代币合约网格", desc: "中期进取型合约网格", folder: "13_美股代币合约网格_StockTokenFuturesGrid" },
    { title: "AI 策略与组合", desc: "官方预设组合与 BTC 生息单", folder: "14_AI策略与AI组合_AI_Strategy" }
  ];

  const STOCK_SPOT = [
    ["xMU-USDT", 12.54, 1922],
    ["xSOXL-USDT", 27.15, 418],
    ["xSNDK-USDT", 25.67, 786],
    ["xNVDA-USDT", 5.68, 1420],
    ["xAMD-USDT", 11.77, 615]
  ];

  const state = {
    folderId: "01_现货网格_SpotGrid",
    priceScale: 1,
    sheet: null,
    filter: "全部",
    running: [
      { id: 1, name: "现货网格 BTC/USDT", botType: "spot_grid", pnl: 12.4, status: "运行中" },
      { id: 2, name: "合约网格 BTCUSDT 5x", botType: "contract_grid", pnl: -3.1, status: "运行中" },
      { id: 3, name: "定投 xQQQ", botType: "recurring_buy", pnl: 1.8, status: "运行中" },
      { id: 4, name: "信号策略 BTCUSDT", botType: "signal_bot", pnl: 0, status: "等待信号" }
    ],
    nextId: 5,
    params: {}
  };

  function folderById(id) {
    return FOLDERS.find((f) => f.id === id) || FOLDERS[1];
  }

  function currentFolder() {
    return folderById(state.folderId);
  }

  function defaultsFor(botType) {
    const common = { botType, modeTab: "manual" };
    if (botType === "spot_grid" || botType === "stock_spot_grid") {
      return Object.assign(common, {
        minPx: botType === "stock_spot_grid" ? 40 : 75000,
        maxPx: botType === "stock_spot_grid" ? 70 : 90000,
        gridNum: botType === "stock_spot_grid" ? 20 : 50,
        runType: "1",
        investAmt: "",
        profitReinvest: false,
        autoEarn: true,
        earnAlloc: "未设置",
        moveUp: "未设置",
        moveDown: "未设置",
        startCond: "未预设",
        stopCond: "未预设",
        tp: "未设置",
        sl: "未设置",
        instId: botType === "stock_spot_grid" ? "xSNDK-USDT" : "BTC-USDT",
        modeTab: botType === "stock_spot_grid" ? "landing" : "manual"
      });
    }
    if (botType === "contract_grid" || botType === "stock_futures_grid") {
      return Object.assign(common, {
        minPx: botType === "stock_futures_grid" ? 200 : 75000,
        maxPx: botType === "stock_futures_grid" ? 320 : 92000,
        gridNum: 20,
        runType: "1",
        investAmt: 0,
        lever: 5,
        direction: "neutral",
        autoReserveMargin: true,
        slippageOn: true,
        slippagePct: 0.0004,
        autoEarn: true,
        moveUp: "未设置",
        tp: "未设置",
        sl: "未设置",
        instId: botType === "stock_futures_grid" ? "TSLA-USDT-SWAP" : "BTC-USDT-SWAP",
        riskAck: false
      });
    }
    if (botType === "contract_dca" || botType === "spot_dca") {
      return Object.assign(common, {
        direction: "long",
        lever: botType === "contract_dca" ? 19 : 1,
        pxStepsPct: botType === "contract_dca" ? 1.2 : 1.5,
        tpPct: 1.5,
        initOrdAmt: botType === "contract_dca" ? 10 : 20,
        safetyOrdAmt: botType === "contract_dca" ? 10 : 20,
        maxSafetyOrds: 8,
        pxStepsMult: 1.05,
        volMult: 1.05,
        reserveFunds: true,
        rsiTrigger: 30,
        instId: botType === "contract_dca" ? "BTC-USDT-SWAP" : "SOL-USDT"
      });
    }
    if (botType === "smart_portfolio" || botType === "ai_strategy") {
      return Object.assign(common, {
        investAmt: 500,
        rebalanceMode: "proportional",
        rebalanceThresholdPct: 5,
        rebalancePeriod: "4小时",
        autoEarn: true,
        portfolioCoins: [
          { ccy: "xMU", weight: 40, px: 924.5 },
          { ccy: "xBMNR", weight: 35, px: 42.1 },
          { ccy: "xREN", weight: 25, px: 18.6 }
        ],
        preset: "桥水基金全天候加密版"
      });
    }
    if (botType === "smart_arbitrage") {
      return Object.assign(common, {
        investAmt: 1000,
        lever: 10,
        arbMarketTab: "Crypto",
        borrowUsdt: false,
        legMode: "smart",
        spotPriceType: "排队价",
        swapPriceType: "对手价",
        instId: "BTC-USDT-SWAP"
      });
    }
    if (botType === "signal_bot") {
      return Object.assign(common, {
        signalName: "暂无信号",
        signalIntro: "",
        lever: 5,
        investAmt: "",
        selectedPairsCount: 1,
        addPosMode: "占用可用保证金比例",
        sigOrderType: "市价委托",
        maxPositions: 5,
        tpSlMode: "收益率",
        tpPct: 10,
        slPct: 5,
        instId: "BTC-USDT-SWAP"
      });
    }
    if (botType === "iceberg" || botType === "twap") {
      return Object.assign(common, {
        posAction: "open",
        side: "long",
        marginMode: botType === "twap" ? "逐仓" : "全仓",
        lever: 3,
        priceOffsetUsdt: 100,
        limitPx: botType === "iceberg" ? 924.87 : 924.87,
        singleOrdSz: 10,
        activeOrdersCount: 3,
        totalSz: 100,
        timeIntervalSec: 15,
        instId: "MU-USDT-SWAP"
      });
    }
    if (botType === "recurring_buy") {
      return Object.assign(common, {
        targetCcy: "xQQQ",
        perCycleAmt: 100,
        frequency: "每天",
        availUsdt: 0,
        firstBuyTime: "立即买入",
        autoSupplement: false,
        minPrice: "",
        maxPrice: "",
        modeTab: "featured",
        instId: "xQQQ-USDT"
      });
    }
    if (botType === "dcd_dip_tp") {
      return Object.assign(common, {
        dropPct: 1,
        minApyPct: 1,
        maxTenorDays: 35,
        investAmt: 100,
        modeTab: "ai",
        instId: "BTC-USDT"
      });
    }
    return Object.assign(common, { investAmt: 100, instId: "BTC-USDT" });
  }

  function params() {
    const bot = currentFolder().botType;
    if (!state.params[bot]) state.params[bot] = defaultsFor(bot);
    return state.params[bot];
  }

  function instChoices(botType) {
    const map = {
      spot_grid: ["BTC-USDT", "SOL-USDT"],
      contract_grid: ["BTC-USDT-SWAP", "BTC-USD-SWAP", "ETH-USDT-SWAP"],
      stock_spot_grid: ["xMU-USDT", "xSOXL-USDT", "xSNDK-USDT", "xNVDA-USDT", "xAMD-USDT"],
      stock_futures_grid: ["TSLA-USDT-SWAP"],
      iceberg: ["MU-USDT-SWAP", "BTC-USDT-SWAP"],
      twap: ["MU-USDT-SWAP", "BTC-USDT-SWAP"],
      signal_bot: ["BTC-USDT-SWAP", "ETH-USDT-SWAP"],
      smart_arbitrage: ["BTC-USDT-SWAP", "SOL-USDT"],
      contract_dca: ["BTC-USDT-SWAP", "ETH-USDT-SWAP"],
      spot_dca: ["SOL-USDT", "BTC-USDT"],
      recurring_buy: ["xQQQ-USDT", "BTC-USDT", "xMU-USDT"],
      dcd_dip_tp: ["BTC-USDT"]
    };
    return map[botType] || [currentFolder().defaultInst];
  }

  function activeInst() {
    const p = params();
    const id = p.instId || currentFolder().defaultInst;
    const base = M.INSTRUMENTS[id] || M.INSTRUMENTS["BTC-USDT"];
    return Object.assign({}, base, { lastPx: base.lastPx * state.priceScale });
  }

  function compute() {
    const p = params();
    const inst = activeInst();
    const t = p.botType;
    if (t === "spot_grid" || t === "contract_grid" || t === "stock_spot_grid" || t === "stock_futures_grid") {
      return M.computeGrid(p, inst);
    }
    if (t === "spot_dca" || t === "contract_dca") return M.computeDcaMartingale(p, inst);
    if (t === "smart_portfolio" || t === "ai_strategy") return M.computeSmartPortfolio(p);
    if (t === "smart_arbitrage") return M.computeSmartArbitrage(p, inst);
    if (t === "signal_bot") return M.computeSignalBot(p, inst);
    if (t === "iceberg" || t === "twap") return M.computeExecutionSlicing(p, inst);
    if (t === "recurring_buy") return M.computeRecurringBuy(p);
    if (t === "dcd_dip_tp") return M.computeDipBuyTakeProfit(p, inst);
    if (t === "hub_common") return { valid: true, levels: [] };
    return { valid: false, error: "未实现", levels: [] };
  }

  function fmt(n, d) {
    if (n === null || n === undefined || Number.isNaN(Number(n))) return "--";
    return M.formatNum(Number(n), d === undefined ? 2 : d);
  }

  function esc(s) {
    return String(s == null ? "" : s)
      .replace(/&/g, "&amp;")
      .replace(/</g, "&lt;")
      .replace(/"/g, "&quot;");
  }

  function field(label, key, unit, placeholder) {
    const v = params()[key];
    return (
      '<div class="okx-field-block">' +
      '<div class="okx-field-header"><span>' + esc(label) + "</span></div>" +
      '<label class="okx-gray-input"><span class="okx-input-row">' +
      '<input data-act="input" data-key="' + esc(key) + '" value="' + esc(v) + '" placeholder="' + esc(placeholder || "") + '" inputmode="decimal" />' +
      (unit ? '<span class="okx-input-unit">' + esc(unit) + "</span>" : "") +
      "</span></label></div>"
    );
  }

  function row(label, value, extra) {
    return (
      '<div class="okx-kv-row"><span class="okx-kv-label' + (extra || "") + '">' + esc(label) +
      '</span><span class="okx-kv-val" data-live="' + esc(label) + '">' + value + "</span></div>"
    );
  }

  function toggle(label, key) {
    const on = !!params()[key];
    return (
      '<div class="okx-kv-row"><span class="okx-kv-label">' + esc(label) + "</span>" +
      '<label class="okx-switch"><input type="checkbox" data-act="check" data-key="' + esc(key) + '"' + (on ? " checked" : "") + " />" +
      '<span class="okx-switch-slider"></span></label></div>'
    );
  }

  function chips(key, options) {
    const cur = String(params()[key]);
    return (
      '<div class="okx-chip-grid">' +
      options.map((o) => {
        const val = Array.isArray(o) ? o[0] : o;
        const label = Array.isArray(o) ? o[1] : o;
        return '<button type="button" class="okx-chip-option' + (cur === String(val) ? " selected" : "") +
          '" data-act="set" data-key="' + esc(key) + '" data-val="' + esc(val) + '">' + esc(label) + "</button>";
      }).join("") +
      "</div>"
    );
  }

  function seg(key, options) {
    const cur = String(params()[key]);
    return (
      '<div class="okx-segmented-pill" style="grid-template-columns:repeat(' + options.length + ',1fr)">' +
      options.map((o) => {
        const val = o[0];
        return '<button type="button" class="okx-seg-btn' + (cur === String(val) ? " active" : "") +
          '" data-act="set" data-key="' + esc(key) + '" data-val="' + esc(val) + '">' + esc(o[1]) + "</button>";
      }).join("") +
      "</div>"
    );
  }

  function linkRow(label, key) {
    return (
      '<button type="button" class="okx-kv-row" data-act="sheet" data-sheet="' + esc(key) + '" style="width:100%;background:none;border:none;cursor:pointer;text-align:left">' +
      '<span class="okx-kv-label">' + esc(label) + '</span><span class="okx-kv-val">' + esc(params()[key] || "未设置") + " ›</span></button>"
    );
  }

  function shell(title, body, cta) {
    const inst = activeInst();
    const down = inst.change24hPct < 0;
    return (
      '<header class="okx-nav-header">' +
      '<button class="okx-nav-back" data-act="folder" data-id="00_公共组件_创建策略与通用功能">‹</button>' +
      '<button class="okx-nav-title-btn" data-act="sheet" data-sheet="create">' + esc(title) + " ▾</button>" +
      '<div class="okx-nav-icons"><span class="okx-icon-btn">▣</span><span class="okx-icon-btn">···</span></div></header>' +
      '<div class="okx-pair-strip"><button class="okx-pair-name" data-act="cycle-inst" style="background:none;border:none">' +
      esc(inst.displayName) + " ▾</button>" +
      '<div class="okx-pair-price-cluster"><span class="' + (down ? "price-down" : "price-up") + '">' +
      fmt(inst.lastPx, inst.tickSz < 0.1 ? 2 : 1) + (down ? " ↓" : " ↑") + "</span></div></div>" +
      '<div class="phone-body" id="phoneBody">' + body + "</div>" +
      '<div class="phone-bottom-cta">' + cta + "</div>"
    );
  }

  function cta(label, enabled) {
    return '<button class="okx-black-pill-btn" data-act="create"' + (enabled ? "" : " disabled style=\"opacity:.45\"") + ">" + esc(label) + "</button>";
  }

  function renderGridForm(contract) {
    const p = params();
    const calc = compute();
    const inst = activeInst();
    const unit = inst.marginCcy;
    const minTxt = contract
      ? "> " + fmt(calc.minInvest, inst.instType === "SWAP_CM" ? 4 : 2) + " (" + unit + ")"
      : "> " + fmt(calc.minInvest || 60.2, 1);
    const profit = calc.profitMinPct
      ? fmt(calc.profitMinPct, 2) + "% ~ " + fmt(calc.profitMaxPct, 2) + "%"
      : "--";
    let html = "";
    if (!contract) {
      html += seg("modeTab", [["ai", "AI 策略"], ["manual", "手动创建"]]);
    }
    if (p.modeTab === "ai" && !contract) {
      html += '<div class="okx-border-card"><div style="font-weight:700;margin-bottom:6px">AI 推荐参数</div>' +
        "<div>回测年化 <b class=\"price-up\">+18.6%</b> · 网格 " + esc(p.gridNum) + " · 等差</div>" +
        "<div style=\"color:var(--okx-text-secondary);margin-top:6px\">区间 " + fmt(p.minPx, 1) + " – " + fmt(p.maxPx, 1) + "</div></div>";
    }
    if (contract) {
      html += '<div class="okx-field-header"><span>方向</span></div>' + chips("direction", [["long", "做多"], ["short", "做空"], ["neutral", "中性"]]);
      html += '<div class="okx-kv-row"><span class="okx-kv-label">杠杆</span><button class="okx-chip-option selected" data-act="sheet" data-sheet="lever">' + esc(p.lever) + "x ▾</button></div>";
    }
    html += field(contract ? "投资额 (保证金)" : "投资额", "investAmt", unit + " ▾", minTxt);
    html += '<div class="okx-error-text" data-error style="' + (calc.error ? "" : "display:none") + '">' + esc(calc.error || "") + "</div>";
    html += '<div class="okx-slider-wrap"><input type="range" min="0" max="' + (contract ? 5000 : 2000) + '" step="1" data-act="input" data-key="investAmt" value="' + esc(p.investAmt || 0) + '" /></div>';
    html += row("可用", "0 " + unit);
    if (contract) {
      html += toggle("自动预留保证金", "autoReserveMargin");
      html += row("实际保证金 + 预留保证金", p.investAmt > 0 ? fmt(calc.actualMargin, 2) + " + " + fmt(calc.reservedMargin, 2) : "--");
      html += row("含杠杆的实际投入", p.investAmt > 0 ? fmt(calc.leveragedNotional, 2) + " " + unit : "--");
      html += row("预估强平价 (多)", calc.liqLong ? fmt(calc.liqLong, 1) : "--");
      html += row("预估强平价 (空)", calc.liqShort ? fmt(calc.liqShort, 1) : "--");
      html += '<button class="okx-kv-row" data-act="sheet" data-sheet="slippage" style="width:100%;background:none;border:none;cursor:pointer"><span class="okx-kv-label">滑点控制</span><span class="okx-kv-val">' + (p.slippageOn ? esc(p.slippagePct) + "%" : "关") + " ›</span></button>";
    } else {
      html += toggle("利润复投", "profitReinvest");
      html += '<div class="okx-kv-row"><span class="okx-kv-label">自动赚币</span><span class="okx-kv-val price-up">+' + fmt(inst.simpleEarnApyPct || 2.79, 2) + "% 年化 ›</span></div>";
    }
    html += '<div class="okx-field-header" style="margin-top:8px"><span>价格区间</span></div>';
    html += field("最低价", "minPx", unit === "BTC" ? "USD" : "USDT");
    html += field("最高价", "maxPx", unit === "BTC" ? "USD" : "USDT");
    html += field("网格数量 (2–1,000)", "gridNum", "格");
    html += '<div class="okx-field-header"><span>网格模式</span></div>' + chips("runType", [["1", "等差"], ["2", "等比"]]);
    html += row("单网格收益率", profit);
    html += row("每格数量", fmt(calc.perGridQty, 6));
    if (!contract) {
      html += linkRow("简单赚币分配", "earnAlloc");
      html += linkRow("网格随价格上移", "moveUp");
      html += linkRow("网格随价格下移", "moveDown");
      html += linkRow("预设开始条件", "startCond");
      html += linkRow("预设停止条件", "stopCond");
      html += linkRow("止盈", "tp");
      html += linkRow("止损", "sl");
    }
    const title = contract ? (p.instId === "BTC-USD-SWAP" ? "合约网格 (币本位)" : currentFolder().navTitle) : currentFolder().navTitle;
    return shell(title, html, cta("创建策略", calc.valid));
  }

  function renderDca() {
    const p = params();
    const calc = compute();
    const contract = p.botType === "contract_dca";
    let html = seg("modeTab", [["ai", "AI 策略"], ["manual", "手动创建"]]);
    if (p.modeTab === "ai") {
      html += '<div class="okx-border-card"><b>短期进取建仓 · ' + (p.direction === "short" ? "做空" : "做多") + "</b><div style=\"margin-top:6px;color:var(--okx-text-secondary)\">加仓间距 " + esc(p.pxStepsPct) + "% · 止盈 " + esc(p.tpPct) + "% · 最多 " + esc(p.maxSafetyOrds) + " 笔</div></div>";
    }
    if (contract) {
      html += chips("direction", [["long", "做多"], ["short", "做空"]]);
      html += '<div class="okx-kv-row"><span class="okx-kv-label">杠杆</span><button class="okx-chip-option selected" data-act="sheet" data-sheet="lever">' + esc(p.lever) + "x ▾</button></div>";
      if (calc.highLeverWarning) html += '<div class="okx-error-text">' + esc(calc.highLeverWarning) + "</div>";
    }
    html += field("跌多少加仓", "pxStepsPct", "%", contract ? "0.1–50" : "0.2–50");
    html += field("单周期止盈目标", "tpPct", "%");
    html += field("初次下单金额", "initOrdAmt", "USDT");
    html += field("加仓单金额", "safetyOrdAmt", "USDT");
    html += field("最大加仓次数", "maxSafetyOrds", "次");
    html += field("加仓价差倍数", "pxStepsMult", "x");
    html += field("加仓金额倍数", "volMult", "x");
    html += toggle("预留足够资金", "reserveFunds");
    html += '<button class="okx-kv-row" data-act="sheet" data-sheet="rsi" style="width:100%;background:none;border:none;cursor:pointer"><span class="okx-kv-label">RSI-14 触发</span><span class="okx-kv-val">' + esc(p.rsiTrigger) + " ›</span></button>";
    html += row("整轮所需保证金", fmt(calc.requiredMargin, 2) + " USDT");
    html += row("预估强平价", calc.liqPx ? fmt(calc.liqPx, 1) : "现货无强平");
    if (calc.error) html += '<div class="okx-error-text">' + esc(calc.error) + "</div>";
    html += '<div class="okx-field-header" style="margin-top:10px"><span>委托阶梯</span></div>';
    html += '<table class="okx-table"><tr><th>#</th><th>触发价</th><th>金额</th><th>止盈价</th></tr>' +
      (calc.ladder || []).slice(0, 6).map((r) => "<tr><td>" + esc(r.label) + "</td><td>" + fmt(r.triggerPx, 2) + "</td><td>" + fmt(r.orderAmt, 2) + "</td><td>" + fmt(r.tpPx, 2) + "</td></tr>").join("") +
      "</table>";
    return shell(currentFolder().navTitle, html, cta("创建策略", calc.valid));
  }

  function renderPortfolio() {
    const p = params();
    const calc = compute();
    let html = '<div class="okx-field-header"><span>目标比例（合计必须 100%）</span></div>';
    p.portfolioCoins.forEach((c, i) => {
      html += '<div class="okx-gray-input" style="margin-bottom:8px"><span class="okx-input-row"><span class="okx-input-mini-label">' + esc(c.ccy) +
        '</span><input data-act="weight" data-idx="' + i + '" value="' + esc(c.weight) + '" inputmode="decimal" /><span class="okx-input-unit">%</span></span></div>';
    });
    html += field("投资额", "investAmt", "USDT", "≥ " + (calc.minInvest || 10));
    if (calc.error) html += '<div class="okx-error-text">' + esc(calc.error) + "</div>";
    html += '<div class="okx-field-header"><span>再平衡触发</span></div>' + chips("rebalanceMode", [["proportional", "偏离阈值"], ["periodic", "定期检查"]]);
    if (p.rebalanceMode === "proportional") html += field("偏离阈值", "rebalanceThresholdPct", "%");
    else html += field("检查周期", "rebalancePeriod", "");
    html += toggle("自动赚币", "autoEarn");
    html += toggle("自动质押", "autoStake");
    (calc.coins || []).forEach((c) => {
      html += row(c.ccy + " 配置", fmt(c.allocUsdt, 2) + " USDT");
    });
    return shell(currentFolder().navTitle, html, cta("创建组合", calc.valid));
  }

  function renderArb() {
    const p = params();
    const calc = compute();
    let html = seg("legMode", [["smart", "智能套利"], ["dual", "套利下单"]]);
    html += chips("arbMarketTab", ["Crypto", "Tradfi"]);
    html += field("投资额", "investAmt", "USDT", "≥ 50");
    html += '<div class="okx-kv-row"><span class="okx-kv-label">杠杆</span><button class="okx-chip-option selected" data-act="sheet" data-sheet="lever">' + esc(p.lever) + "x ▾</button></div>";
    html += toggle("借币补充保证金", "borrowUsdt");
    if (calc.error) html += '<div class="okx-error-text">' + esc(calc.error) + "</div>";
    html += row("现货开仓", fmt(calc.spotAllocUsdt, 2) + " USDT");
    html += row("合约保证金", fmt(calc.swapMarginUsdt, 2) + " USDT");
    html += row("价差", fmt(calc.spreadPct, 3) + "%");
    html += row("质押年化", fmt(calc.stakingApy, 2) + "%");
    html += row("资金费年化", fmt(calc.fundingApy, 2) + "%");
    html += row("3 日综合年化", fmt(calc.total3dApy, 2) + "%");
    html += row("预估开仓盈亏", fmt(calc.estOpenPnl, 2) + " USDT");
    if (p.legMode === "dual") {
      html += '<div class="okx-field-header"><span>左腿 · 现货</span></div>' + chips("spotPriceType", ["排队价", "超价", "对手价", "市价"]);
      html += '<div class="okx-field-header"><span>右腿 · 永续</span></div>' + chips("swapPriceType", ["排队价", "超价", "对手价", "市价"]);
    }
    return shell("智能套利", html, cta(p.legMode === "dual" ? "预览双腿" : "策略预览", calc.valid));
  }

  function renderSignal() {
    const p = params();
    const calc = compute();
    let html = field("信号名称", "signalName", "", "最多 50 字");
    html += '<div class="okx-field-block"><div class="okx-field-header"><span>简介</span></div><label class="okx-gray-input"><span class="okx-input-row"><input data-act="input" data-key="signalIntro" value="' + esc(p.signalIntro) + '" placeholder="最多 1500 字" /></span></label></div>';
    html += field("交易对数量", "selectedPairsCount", "个", "最多 10");
    html += '<div class="okx-kv-row"><span class="okx-kv-label">杠杆</span><button class="okx-chip-option selected" data-act="sheet" data-sheet="lever">' + esc(p.lever) + "x ▾</button></div>";
    if (calc.leverWarning) html += '<div class="okx-error-text">' + esc(calc.leverWarning) + "</div>";
    html += field("投资额 (保证金)", "investAmt", "USDT", "≥ 50");
    if (calc.error) html += '<div class="okx-error-text">' + esc(calc.error) + "</div>";
    html += '<div class="okx-field-header"><span>加仓设定</span></div>' + chips("addPosMode", ["占用可用保证金比例", "固定保证金"]);
    html += '<div class="okx-field-header"><span>委托类型</span></div>' + chips("sigOrderType", ["市价委托", "限价委托"]);
    html += field("最大持仓数", "maxPositions", "个");
    html += '<div class="okx-field-header"><span>止盈止损</span></div>' + chips("tpSlMode", ["收益率", "涨跌幅"]);
    html += field("止盈", "tpPct", "%");
    html += field("止损", "slPct", "%");
    return shell("信号策略", html, cta("创建信号策略", calc.valid));
  }

  function renderSlice() {
    const p = params();
    const calc = compute();
    const twap = p.botType === "twap";
    let html = '<div class="okx-open-close-bar">' +
      '<button class="okx-oc-btn' + (p.posAction === "open" ? " active-open" : "") + '" data-act="set" data-key="posAction" data-val="open">开仓</button>' +
      '<button class="okx-oc-btn' + (p.posAction === "close" ? " active-close" : "") + '" data-act="set" data-key="posAction" data-val="close">平仓</button></div>';
    html += chips("side", [["long", "可开多"], ["short", "可开空"]]);
    html += '<div class="okx-field-header"><span>保证金模式</span></div>' + chips("marginMode", ["全仓", "逐仓"]);
    html += '<button class="okx-kv-row" data-act="sheet" data-sheet="marginHelp" style="width:100%;background:none;border:none;cursor:pointer"><span class="okx-kv-label dotted">保证金模式说明</span><span class="okx-kv-val">9 页教程 ›</span></button>';
    html += '<div class="okx-kv-row"><span class="okx-kv-label">杠杆</span><button class="okx-chip-option selected" data-act="sheet" data-sheet="lever">' + esc(p.lever) + "x ▾</button></div>";
    html += field("吃单价优于盘口", "priceOffsetUsdt", "USDT");
    html += field("吃单限制价", "limitPx", "USDT");
    html += field("单笔委托量", "singleOrdSz", "张");
    if (!twap) html += field("单次挂单数 (≤5)", "activeOrdersCount", "笔");
    html += field("委托总量", "totalSz", "张");
    if (twap) html += field("下单间隔", "timeIntervalSec", "秒", "5–120");
    html += '<div class="okx-center-toast" data-error style="' + (calc.error ? "" : "display:none") + '">' + esc(calc.error || "") + "</div>";
    html += row("所需保证金", fmt(calc.reqMargin, 2) + " USDT");
    html += row("预估强平价 (多/空)", fmt(calc.liqLong, 2) + " / " + fmt(calc.liqShort, 2));
    return shell(twap ? "时间加权策略" : "冰山策略", html, cta(twap ? "确认 TWAP" : "确认冰山", calc.valid));
  }

  function renderRecurring() {
    const p = params();
    const calc = compute();
    let html = seg("modeTab", [["featured", "精选专区"], ["manual", "手动创建"]]);
    if (p.modeTab === "featured") {
      html += '<div class="okx-border-card"><div class="badge-lime">xStock LIVE</div><div style="font-weight:700;margin:8px 0 4px">科技五巨头组合</div><div class="price-up">+51.52% 回测</div></div>';
      html += '<div class="okx-border-card"><div style="font-weight:700">ETF 核心指数配置</div><div style="color:var(--okx-text-secondary)">xQQQ 每日定投</div><button class="okx-chip-option selected" data-act="set" data-key="modeTab" data-val="manual" style="margin-top:8px">用此组合手动创建</button></div>';
    }
    html += field("每期金额", "perCycleAmt", "USDT", "≥ 2");
    if (calc.error) html += '<div class="okx-error-text">' + esc(calc.error) + "</div>";
    html += '<div class="okx-field-header"><span>频率</span></div>' + chips("frequency", ["1小时", "4小时", "8小时", "12小时", "每天", "每周", "每月"]);
    html += '<div class="okx-field-header"><span>首次买入</span></div>' + chips("firstBuyTime", ["立即买入", "下个整点", "自选时间"]);
    html += toggle("其他账户自动补足", "autoSupplement");
    html += field("价格下限", "minPrice", "USDT", "可选");
    html += field("价格上限", "maxPrice", "USDT", "可选");
    html += row("可用", fmt(p.availUsdt, 2) + " USDT");
    html += row("预计可执行期数", String(calc.estCycles || 0));
    return shell("定投策略", html, cta("创建定投", calc.valid));
  }

  function renderDcd() {
    const p = params();
    const calc = compute();
    let html = seg("modeTab", [["ai", "AI 策略"], ["manual", "手动创建"]]);
    html += '<div class="okx-field-header"><span>抄底跌幅</span></div>' + chips("dropPct", [["0.1", "0.1%"], ["1", "1%"], ["5", "5%"]]);
    html += field("最低年化收益率", "minApyPct", "%");
    html += field("最长期限", "maxTenorDays", "天");
    html += field("申购金额", "investAmt", "USDT", "≥ 10");
    if (calc.error) html += '<div class="okx-error-text">' + esc(calc.error) + "</div>";
    html += '<div class="okx-border-card"><div style="font-weight:700">自动匹配产品</div>' +
      "<div>目标价 " + fmt(calc.buyTargetPx, 0) + " USDT</div>" +
      '<div class="price-up">年化 +' + fmt(calc.matchedApyPct, 2) + "%</div>" +
      "<div>结算 " + esc(calc.settlementTimeStr || "") + " · " + esc(calc.settlementHours) + " 小时</div></div>";
    return shell("抄底止盈策略", html, cta("申购", calc.valid));
  }

  function renderLanding(kind) {
    const p = params();
    if (kind === "stock-spot") {
      let html = '<div style="font-weight:800;font-size:18px;margin-bottom:6px">股票代币现货网格</div><div style="color:var(--okx-text-secondary);margin-bottom:12px">锚定股票代币波动特性，网格自动捕捉每一次起落</div>';
      STOCK_SPOT.forEach((row) => {
        const id = row[0];
        html += '<div class="okx-border-card" style="display:flex;justify-content:space-between;align-items:center"><div><b>' + esc(M.INSTRUMENTS[id].displayName) + '</b><div class="price-up">+' + row[1].toFixed(2) + '%</div><div style="color:var(--okx-text-secondary)">' + row[2] + " 人</div></div>" +
          '<button class="okx-black-pill-btn" style="width:auto;padding:0 16px;height:34px" data-act="pick-inst" data-id="' + id + '">创建</button></div>';
      });
      if (p.modeTab === "manual") return renderGridForm(false);
      return shell("股票代币现货网格", html, '<button class="okx-black-pill-btn" data-act="set" data-key="modeTab" data-val="manual">手动创建</button>');
    }
    if (kind === "stock-fut") {
      const cards = [
        ["TSLA-USDT-SWAP", 82.1],
        ["TSLA-USDT-SWAP", 248.64],
        ["TSLA-USDT-SWAP", 38.2],
        ["TSLA-USDT-SWAP", 294.39]
      ];
      const names = ["TSLA +82.10%", "HOOD +248.64%", "SPCX +38.20%", "AMZN +294.39%"];
      let html = '<div style="font-weight:800;font-size:18px">美股代币合约网格</div><div style="color:var(--okx-text-secondary);margin:6px 0 12px">中期进取型 · 累计 5,299 人 · 投入 4.07M</div>';
      names.forEach((n) => {
        html += '<div class="okx-border-card"><b>' + esc(n) + '</b><div><button class="okx-chip-option selected" data-act="sheet" data-sheet="stockRisk">创建</button></div></div>';
      });
      if (p.riskAck && p.modeTab === "manual") return renderGridForm(true);
      return shell("美股代币合约网格", html, '<button class="okx-black-pill-btn" data-act="sheet" data-sheet="stockRisk">阅读风险须知</button>');
    }
    const presets = [
      ["桥水基金全天候加密版", 18.14],
      ["存储与储备组合", 6.2],
      ["AI数据与储备组合", 8.39]
    ];
    let html = '<div style="font-weight:800;font-size:18px;margin-bottom:8px">AI 策略与 AI 组合</div>';
    presets.forEach((pr) => {
      html += '<button class="okx-border-card" data-act="preset" data-name="' + esc(pr[0]) + '" style="width:100%;text-align:left;cursor:pointer"><b>' + esc(pr[0]) + '</b><div class="price-up">+' + pr[1].toFixed(2) + '%</div><div style="color:var(--okx-text-secondary)">偏离 5% 自动再平衡</div></button>';
    });
    html += '<div class="okx-border-card"><b>BTC 生息单</b><div style="display:grid;grid-template-columns:1fr 1fr;gap:8px;margin-top:8px">' +
      ["网格套利", "双币生息", "自动赚币", "利润复投", "随价格移动", "预设停止"].map((x) => '<span class="badge-soft">' + x + "</span>").join("") +
      "</div></div>";
    const calc = compute();
    if (calc.error) html += '<div class="okx-error-text" data-error>' + esc(calc.error) + "</div>";
    html += field("投资额", "investAmt", "USDT", "≥ " + (calc.minInvest || 10));
    html += row("比例合计", (calc.weightSum || 0) + "%");
    return shell("AI 策略与组合", html, cta("按此组合创建", calc.valid));
  }

  function renderHub() {
    const cards = [
      ["xMU/USDT", "+12.54%", "1,922", "12_股票代币现货网格_StockTokenSpotGrid", "xMU-USDT"],
      ["xSOXL/USDT", "+27.15%", "418", "12_股票代币现货网格_StockTokenSpotGrid", "xSOXL-USDT"],
      ["xSNDK/USDT", "+25.67%", "786", "12_股票代币现货网格_StockTokenSpotGrid", "xSNDK-USDT"]
    ];
    const quick = [
      ["＋", "创建策略", "create"],
      ["格", "现货网格", "01_现货网格_SpotGrid"],
      ["5x", "合约网格", "02_合约网格_FuturesGrid"],
      ["组", "智能组合", "03_智能组合_SmartPortfolio"],
      ["期", "定投策略", "10_定投策略_RecurringBuy"]
    ];
    let html = '<div style="padding:8px 16px 0"><div class="okx-kv-label">总资产估值</div><div style="font-size:28px;font-weight:800">0.00 USD</div><div class="okx-kv-label dotted">$0.00 今日收益</div></div>';
    html += '<div class="okx-circle-grid-4" style="padding:12px 8px">';
    quick.forEach((q, i) => {
      const act = q[2] === "create" ? 'data-act="sheet" data-sheet="create"' : 'data-act="folder" data-id="' + q[2] + '"';
      html += '<button class="okx-circle-item" ' + act + ' style="background:none;border:none;cursor:pointer"><div class="okx-circle-icon' + (i === 0 ? " black" : "") + '">' + q[0] + "</div><span>" + q[1] + (q[1] === "智能组合" ? ' <span class="badge-lime">New</span>' : "") + "</span></button>";
    });
    html += "</div>";
    html += '<div class="okx-chip-grid" style="padding:0 16px 8px"><span class="okx-chip-option selected">推荐</span><span class="okx-chip-option">我的策略 ▾</span><span class="okx-chip-option">我的跟单</span></div>';
    html += '<div class="phone-body" id="phoneBody" style="padding-top:0"><div class="okx-border-card"><div style="font-weight:800">股票代币现货网格</div><div style="color:var(--okx-text-secondary);margin:4px 0 8px">锚定股票代币波动特性，网格自动捕捉每一次起落</div><span class="badge-soft">稳健型</span> <span style="color:var(--okx-text-secondary)">3,591 人已使用</span>';
    cards.forEach((c) => {
      html += '<div class="okx-kv-row"><div><b>' + c[0] + '</b><div class="price-up">' + c[1] + '</div><div class="okx-kv-label">' + c[2] + ' 人</div></div><button class="okx-black-pill-btn" style="width:auto;height:32px;padding:0 14px" data-act="pick-inst" data-folder="' + c[3] + '" data-id="' + c[4] + '">创建</button></div>';
    });
    html += "</div>";
    html += '<div class="okx-border-card">欧易独家！网格套利 × 双币生息，让资金全天候生息</div>';
    html += '<div class="okx-border-card">选择交给自己，再平衡交给规则。全球资产配置，智能组合陪你穿越周期</div>';
    html += "</div>";
    return html;
  }

  function renderPhone() {
    const f = currentFolder();
    const mount = document.getElementById("phoneScreenMount");
    let html = "";
    if (f.botType === "hub_common") html = renderHub();
    else if (f.botType === "spot_grid") html = renderGridForm(false);
    else if (f.botType === "contract_grid") html = renderGridForm(true);
    else if (f.botType === "spot_dca" || f.botType === "contract_dca") html = renderDca();
    else if (f.botType === "smart_portfolio") html = renderPortfolio();
    else if (f.botType === "smart_arbitrage") html = renderArb();
    else if (f.botType === "signal_bot") html = renderSignal();
    else if (f.botType === "iceberg" || f.botType === "twap") html = renderSlice();
    else if (f.botType === "recurring_buy") html = renderRecurring();
    else if (f.botType === "dcd_dip_tp") html = renderDcd();
    else if (f.botType === "stock_spot_grid") html = renderLanding("stock-spot");
    else if (f.botType === "stock_futures_grid") html = renderLanding("stock-fut");
    else if (f.botType === "ai_strategy") html = renderLanding("ai");
    else html = shell(f.navTitle, "<p>" + esc(f.desc) + "</p>", cta("创建", false));
    if (state.sheet) html += renderSheet();
    mount.innerHTML = html;
    document.getElementById("phoneClock").textContent = f.clock || "22:56";
  }

  function renderSheet() {
    const kind = state.sheet;
    const p = params();
    let inner = '<div style="display:flex;justify-content:space-between;align-items:center"><h3>' + esc(sheetTitle(kind)) + '</h3><button class="okx-icon-btn" data-act="close-sheet">✕</button></div>';
    if (kind === "create") {
      inner += chips("filterSheet", []);
      inner += '<div class="okx-chip-grid">' + ["全部", "现货", "合约"].map((t) =>
        '<button class="okx-chip-option' + (state.filter === t ? " selected" : "") + '" data-act="filter" data-val="' + t + '">' + t + "</button>").join("") + "</div>";
      let group = "";
      CREATE_MENU.forEach((item) => {
        if (state.filter === "现货" && /合约|冰山|时间加权|信号/.test(item.title + item.desc)) return;
        if (state.filter === "合约" && /现货网格|定投|抄底|智能组合|股票代币现货/.test(item.title)) return;
        if (item.group && item.group !== group) {
          group = item.group;
          inner += '<div class="okx-field-header"><span>' + esc(group) + "</span></div>";
        }
        inner += '<button class="okx-border-card" data-act="menu" data-folder="' + esc(item.folder) + '" data-inst="' + esc(item.instId || "") + '" style="width:100%;text-align:left;cursor:pointer"><b>' + esc(item.title) + "</b> " +
          (item.badge ? '<span class="badge-lime">' + esc(item.badge) + "</span>" : "") +
          '<div style="color:var(--okx-text-secondary)">' + esc(item.desc) + "</div></button>";
      });
    } else if (kind === "lever") {
      const max = activeInst().maxLever || 100;
      const opts = [1, 2, 3, 5, 10, 20, 30, 50, 75, 100].filter((n) => n <= max);
      inner += chips("lever", opts.map((n) => [String(n), n + "x"]));
      if (Number(p.lever) > 10) inner += '<div class="okx-error-text">超过 10x 杠杆会增加强制平仓风险</div>';
      inner += '<button class="okx-green-pill-btn" data-act="close-sheet" style="margin-top:12px">确认</button>';
    } else if (kind === "slippage") {
      inner += toggle("滑点控制", "slippageOn");
      inner += field("滑点 0.0001–5 (%)", "slippagePct", "%");
      inner += chips("slippagePct", [["0.05", "0.05%"], ["0.2", "0.2%"], ["0.5", "0.5%"], ["1", "1%"], ["5", "5%"]]);
      inner += row("AI 推荐滑点", "0.0004%");
      inner += row("当前价格", fmt(activeInst().lastPx, 1) + " USDT");
      inner += '<p style="color:var(--okx-text-secondary);margin:8px 0">滑点控制可让成交价更接近预期价格。若不设置，系统以最快速度执行。</p>';
      inner += '<button class="okx-green-pill-btn" data-act="close-sheet">确认</button>';
    } else if (kind === "rsi" || kind === "startCond") {
      inner += field("RSI-14 低于", "rsiTrigger", "", "例如 30");
      inner += '<p style="color:var(--okx-text-secondary)">也可改用价格触发、TradingView 信号或延迟开始。</p>';
      inner += '<button class="okx-black-pill-btn" data-act="apply-cond" data-key="startCond" data-val="RSI-14">设为开始条件</button>';
    } else if (kind === "stopCond") {
      inner += '<button class="okx-black-pill-btn" data-act="apply-cond" data-key="stopCond" data-val="价格触发">价格触发</button>';
    } else if (kind === "tp" || kind === "sl" || kind === "moveUp" || kind === "moveDown" || kind === "earnAlloc") {
      inner += field(kind, kind === "earnAlloc" ? "earnAlloc" : kind, "", "输入后点确认");
      inner += '<button class="okx-black-pill-btn" data-act="close-sheet">确认</button>';
    } else if (kind === "stockRisk") {
      inner += "<p>股票代币合约的波动、交易时段和流动性与加密永续不同。强平、资金费和节假日跳空都会计入风险。确认后才能创建。</p>";
      inner += '<button class="okx-green-pill-btn" data-act="ack-risk">我已了解风险</button>';
    } else if (kind === "marginHelp") {
      inner += "<p>全仓：账户内保证金共享，一个仓位的亏损会占用其他仓位的保证金。</p><p style=\"margin-top:8px\">逐仓：每个仓位单独计算保证金，亏损不会自动占用其他仓位。</p><p style=\"margin-top:8px\">录屏中的官方说明共 9 页，核心就是这两条风控边界。</p>";
      inner += '<button class="okx-black-pill-btn" data-act="close-sheet">知道了</button>';
    }
    return '<div class="phone-sheet-mask" data-act="close-sheet"></div><div class="phone-sheet">' + inner + "</div>";
  }

  function sheetTitle(kind) {
    return {
      create: "创建策略",
      lever: "调整杠杆",
      slippage: "滑点控制设置",
      rsi: "RSI-14 触发",
      startCond: "预设开始条件",
      stopCond: "预设停止条件",
      tp: "止盈",
      sl: "止损",
      moveUp: "网格随价格上移",
      moveDown: "网格随价格下移",
      earnAlloc: "简单赚币分配",
      stockRisk: "股票合约风险须知",
      marginHelp: "全仓 / 逐仓"
    }[kind] || "设置";
  }

  function renderNav() {
    const box = document.getElementById("folderNavContainer");
    box.innerHTML = FOLDERS.map((f) => {
      const n = SHOTS.filter((s) => s.bot === f.id).length;
      return '<button class="folder-pill' + (f.id === state.folderId ? " active" : "") + '" data-act="folder" data-id="' + esc(f.id) + '">' +
        esc(f.shortTitle) + '<span class="shot-count">' + n + "</span></button>";
    }).join("");
  }

  function renderGallery() {
    const f = currentFolder();
    const list = SHOTS.filter((s) => s.bot === f.id);
    document.getElementById("galleryFolderTitle").textContent = f.shortTitle;
    document.getElementById("galleryCountBadge").textContent = list.length + " 张";
    document.getElementById("galleryFolderDesc").textContent = f.desc;
    const grid = document.getElementById("screenshotGalleryGrid");
    grid.innerHTML = list.map((s, i) => {
      const src = "/screenshots/" + s.file.split("/").map(encodeURIComponent).join("/");
      return '<button class="shot-thumb-card" data-act="shot" data-idx="' + i + '" type="button">' +
        '<img alt="' + esc(s.label) + '" src="' + src + '" />' +
        '<div class="shot-thumb-caption">' + esc(s.label) + "</div></button>";
    }).join("");
  }

  function renderInspector() {
    const f = currentFolder();
    const calc = compute();
    const inst = activeInst();
    document.getElementById("activeRustSymbolBadge").textContent = f.rustSymbol;
    const lines = [
      ["目录", f.shortTitle],
      ["bot_type", f.botType],
      ["DEX", f.dexClass],
      ["XML", f.xmlLayout],
      ["标的", inst.instId],
      ["最新价", fmt(inst.lastPx, 2)],
      ["校验", calc.valid ? "通过" : (calc.error || "参数未填完整")]
    ];
    Object.keys(calc).forEach((k) => {
      if (k === "levels" || k === "ladder" || k === "coins") return;
      const v = calc[k];
      if (v && typeof v === "object") return;
      lines.push([k, String(v)]);
    });
    document.getElementById("mathInspectorBody").innerHTML = lines.map((kv) =>
      '<div class="okx-kv-row"><span class="okx-kv-label">' + esc(kv[0]) + '</span><span class="okx-kv-val">' + esc(kv[1]) + "</span></div>"
    ).join("");
    drawKline(inst, calc);
    const meta = document.getElementById("klineMetaStrip");
    const lv = calc.levels || [];
    meta.innerHTML = "<span>" + esc(inst.displayName) + "</span><span>图层 " + lv.length + " 条</span><span>" + esc(f.rustSymbol) + "</span>";
  }

  function drawKline(inst, calc) {
    const canvas = document.getElementById("botKlineCanvas");
    if (!canvas) return;
    const ctx = canvas.getContext("2d");
    const w = canvas.width;
    const h = canvas.height;
    ctx.clearRect(0, 0, w, h);
    ctx.fillStyle = getComputedStyle(document.body).getPropertyValue("--okx-bg-primary") || "#fff";
    ctx.fillRect(0, 0, w, h);
    const px = inst.lastPx;
    const levels = (calc.levels || []).filter((n) => typeof n === "number" && isFinite(n));
    let lo = px * 0.9;
    let hi = px * 1.1;
    levels.forEach((n) => {
      lo = Math.min(lo, n);
      hi = Math.max(hi, n);
    });
    const pad = (hi - lo) * 0.08 || px * 0.02;
    lo -= pad;
    hi += pad;
    const yOf = (price) => 16 + ((hi - price) / (hi - lo)) * (h - 32);
    ctx.strokeStyle = "rgba(120,120,120,0.25)";
    ctx.setLineDash([4, 4]);
    levels.slice(0, 40).forEach((n) => {
      const y = yOf(n);
      ctx.beginPath();
      ctx.moveTo(8, y);
      ctx.lineTo(w - 8, y);
      ctx.stroke();
    });
    ctx.setLineDash([]);
    const bars = 48;
    let price = px * 0.97;
    for (let i = 0; i < bars; i++) {
      const drift = Math.sin(i / 5 + px) * px * 0.004;
      const open = price;
      const close = price + drift;
      const high = Math.max(open, close) + Math.abs(drift) * 0.4;
      const low = Math.min(open, close) - Math.abs(drift) * 0.4;
      const x = 16 + i * ((w - 32) / bars);
      ctx.strokeStyle = close >= open ? "#25a750" : "#e53935";
      ctx.fillStyle = ctx.strokeStyle;
      ctx.beginPath();
      ctx.moveTo(x + 4, yOf(high));
      ctx.lineTo(x + 4, yOf(low));
      ctx.stroke();
      const top = yOf(Math.max(open, close));
      const bot = yOf(Math.min(open, close));
      ctx.fillRect(x, top, 8, Math.max(1, bot - top));
      price = close;
    }
    ctx.strokeStyle = "#e53935";
    ctx.beginPath();
    ctx.moveTo(8, yOf(px));
    ctx.lineTo(w - 8, yOf(px));
    ctx.stroke();
  }

  function renderRunning() {
    document.getElementById("runningBotsCount").textContent = state.running.length + " 运行中";
    document.getElementById("runningBotsList").innerHTML = state.running.map((b) =>
      '<div class="okx-kv-row"><span><b>' + esc(b.name) + '</b><div class="okx-kv-label">' + esc(b.status) + '</div></span><span class="' + (b.pnl >= 0 ? "price-up" : "price-down") + '">' + (b.pnl >= 0 ? "+" : "") + fmt(b.pnl, 2) + " USDT</span></div>"
    ).join("") || '<div class="okx-kv-label">还没有运行中的策略。在手机里点创建。</div>';
  }

  function renderAll() {
    renderNav();
    renderPhone();
    renderGallery();
    renderInspector();
    renderRunning();
  }

  function openFolder(id, instId) {
    state.folderId = id;
    state.sheet = null;
    const p = params();
    if (instId) p.instId = instId;
    renderAll();
  }

  function applyShot(shot) {
    const label = shot.label || "";
    const p = params();
    if (/币本位|BTCUSD/.test(label)) p.instId = "BTC-USD-SWAP";
    if (/RSI/.test(label)) {
      p.rsiTrigger = 30;
      p.startCond = "RSI-14";
    }
    if (/价格触发/.test(label) && /开始/.test(label)) p.startCond = "价格触发";
    if (/停止/.test(label)) p.stopCond = "价格触发";
    if (/利润复投|随价格/.test(label)) p.profitReinvest = true;
    if (/滑点/.test(label)) state.sheet = "slippage";
    if (/杠杆/.test(label)) state.sheet = "lever";
    if (/投资额输入校验|保证金校验/.test(label)) p.investAmt = 0;
    if (/AI推荐|AI 策略/.test(label)) p.modeTab = "ai";
    if (/手动/.test(label)) p.modeTab = "manual";
    if (/做空|开空/.test(label)) p.direction = p.side = "short";
    if (/全仓/.test(label)) p.marginMode = "全仓";
    if (/逐仓/.test(label)) p.marginMode = "逐仓";
    if (/间隔/.test(label)) p.timeIntervalSec = 3;
    renderAll();
  }

  function onCreate() {
    const f = currentFolder();
    const calc = compute();
    if (!calc.valid) return;
    const inst = activeInst();
    state.running.unshift({
      id: state.nextId++,
      name: f.navTitle + " " + inst.displayName,
      botType: f.botType,
      pnl: 0,
      status: "运行中"
    });
    state.sheet = null;
    renderAll();
  }

  document.body.addEventListener("click", (ev) => {
    const t = ev.target.closest("[data-act]");
    if (!t) return;
    const act = t.getAttribute("data-act");
    const p = params();
    if (act === "folder") openFolder(t.getAttribute("data-id"));
    else if (act === "set") {
      p[t.getAttribute("data-key")] = t.getAttribute("data-val");
      renderAll();
    } else if (act === "sheet") {
      state.sheet = t.getAttribute("data-sheet");
      renderPhone();
    } else if (act === "close-sheet") {
      state.sheet = null;
      renderPhone();
    } else if (act === "filter") {
      state.filter = t.getAttribute("data-val");
      renderPhone();
    } else if (act === "menu") {
      openFolder(t.getAttribute("data-folder"), t.getAttribute("data-inst") || undefined);
    } else if (act === "cycle-inst") {
      const list = instChoices(p.botType);
      const i = Math.max(0, list.indexOf(p.instId));
      p.instId = list[(i + 1) % list.length];
      renderAll();
    } else if (act === "pick-inst") {
      const folder = t.getAttribute("data-folder") || state.folderId;
      openFolder(folder, t.getAttribute("data-id"));
      params().modeTab = "manual";
      renderAll();
    } else if (act === "create") onCreate();
    else if (act === "ack-risk") {
      p.riskAck = true;
      p.modeTab = "manual";
      state.sheet = null;
      renderAll();
    } else if (act === "apply-cond") {
      p[t.getAttribute("data-key")] = t.getAttribute("data-val");
      state.sheet = null;
      renderAll();
    } else if (act === "preset") {
      p.preset = t.getAttribute("data-name");
      if (/存储/.test(p.preset)) {
        p.portfolioCoins = [
          { ccy: "xMU", weight: 40, px: 924.5 },
          { ccy: "xBMNR", weight: 35, px: 42.1 },
          { ccy: "xREN", weight: 25, px: 18.6 }
        ];
      }
      renderAll();
    } else if (act === "shot") {
      const list = SHOTS.filter((s) => s.bot === state.folderId);
      const shot = list[Number(t.getAttribute("data-idx"))];
      if (!shot) return;
      applyShot(shot);
      const src = "/screenshots/" + shot.file.split("/").map(encodeURIComponent).join("/");
      const box = document.createElement("div");
      box.className = "shot-lightbox";
      box.innerHTML = '<img alt="' + esc(shot.label) + '" src="' + src + '" />';
      box.addEventListener("click", () => box.remove());
      document.body.appendChild(box);
    }
  });

  document.body.addEventListener("input", (ev) => {
    const t = ev.target;
    if (!t.getAttribute) return;
    const act = t.getAttribute("data-act");
    const p = params();
    if (act === "input") {
      const key = t.getAttribute("data-key");
      const raw = t.value;
      p[key] = raw === "" ? "" : (/name|Intro|Period|Time|Cond|mode|Type/.test(key) ? raw : raw);
      if (["investAmt", "minPx", "maxPx", "gridNum", "lever", "pxStepsPct", "tpPct", "initOrdAmt", "safetyOrdAmt", "maxSafetyOrds", "pxStepsMult", "volMult", "perCycleAmt", "timeIntervalSec", "singleOrdSz", "totalSz", "activeOrdersCount", "dropPct", "minApyPct", "maxTenorDays", "priceOffsetUsdt", "limitPx", "slippagePct", "rebalanceThresholdPct", "selectedPairsCount", "tpPct", "slPct", "rsiTrigger"].indexOf(key) >= 0) {
        p[key] = raw === "" ? "" : Number(raw);
      } else {
        p[key] = raw;
      }
      renderInspector();
      const calc = compute();
      document.querySelectorAll("[data-error]").forEach((err) => {
        const msg = calc.error || "";
        err.textContent = msg;
        err.style.display = msg ? "" : "none";
      });
      const btn = document.querySelector('[data-act="create"]');
      if (btn) {
        btn.disabled = !calc.valid;
        btn.style.opacity = calc.valid ? "" : "0.45";
      }
    } else if (act === "weight") {
      const idx = Number(t.getAttribute("data-idx"));
      p.portfolioCoins[idx].weight = Number(t.value);
      renderInspector();
    }
  });

  document.body.addEventListener("change", (ev) => {
    const t = ev.target;
    if (t.getAttribute && t.getAttribute("data-act") === "check") {
      params()[t.getAttribute("data-key")] = t.checked;
      renderInspector();
      renderPhone();
    }
  });

  document.getElementById("openCreateStrategyDrawerBtn").addEventListener("click", () => {
    state.sheet = "create";
    if (currentFolder().botType !== "hub_common") openFolder("00_公共组件_创建策略与通用功能");
    state.sheet = "create";
    renderPhone();
  });

  document.getElementById("toggleGalleryBtn").addEventListener("click", () => {
    document.getElementById("mainWorkspace").classList.toggle("hide-gallery");
    document.getElementById("screenshotGalleryAside").style.display =
      document.getElementById("mainWorkspace").classList.contains("hide-gallery") ? "none" : "";
  });

  document.getElementById("toggleThemeBtn").addEventListener("click", () => {
    document.body.classList.toggle("dark-theme");
    drawKline(activeInst(), compute());
  });

  document.getElementById("simPriceDropBtn").addEventListener("click", () => {
    state.priceScale *= 0.985;
    renderAll();
  });
  document.getElementById("simPriceRiseBtn").addEventListener("click", () => {
    state.priceScale *= 1.015;
    renderAll();
  });
  document.getElementById("simPriceResetBtn").addEventListener("click", () => {
    state.priceScale = 1;
    renderAll();
  });
  document.getElementById("stepSimulationBtn").addEventListener("click", () => {
    state.priceScale *= 1 + (Math.random() - 0.48) * 0.02;
    state.running.forEach((b) => {
      if (b.status === "运行中") b.pnl = Math.round((b.pnl + (Math.random() - 0.42) * 4) * 100) / 100;
    });
    renderAll();
  });

  renderAll();
})();
