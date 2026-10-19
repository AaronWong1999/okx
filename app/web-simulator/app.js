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
    appTab: "trade",
    tradeTab: "策略&跟单",
    hubSubTab: "推荐",
    klineExpanded: false,
    klinePeriod: "4时",
    klineIndicator: "EMA",
    advExpanded: true,
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

  function shell(title, body, ctaHtml) {
    const inst = activeInst();
    const down = inst.change24hPct < 0;
    const klineDrawer = state.klineExpanded
      ? '<div class="okx-phone-kline-drawer">' +
        '<div style="display:flex;justify-content:space-between;font-size:11.5px;margin-bottom:6px">' +
        ["15分", "1时", "4时", "1日"].map((pd) =>
          '<span style="cursor:pointer;font-weight:' + (state.klinePeriod === pd ? "800;color:var(--okx-text-primary)" : "500;color:var(--okx-text-secondary)") +
          '" data-act="kline-pd" data-val="' + pd + '">' + pd + "</span>"
        ).join("") +
        '<span style="cursor:pointer;font-weight:700" data-act="kline-ind">' + esc(state.klineIndicator) + " ▾</span></div>" +
        '<div style="font-size:10.5px;color:#d97706;margin-bottom:4px" class="tabular">' +
        (state.klineIndicator === "EMA"
          ? "EMA7: " + fmt(inst.lastPx * 0.998, 1) + "  EMA14: " + fmt(inst.lastPx * 0.994, 1) + "  EMA28: " + fmt(inst.lastPx * 0.989, 1)
          : "MA(5): " + fmt(inst.lastPx * 0.997, 1) + "  MA(10): " + fmt(inst.lastPx * 0.993, 1) + "  MA(20): " + fmt(inst.lastPx * 0.988, 1)) +
        "</div></div>"
      : "";
    return (
      '<header class="okx-nav-header">' +
      '<button class="okx-nav-back" data-act="folder" data-id="00_公共组件_创建策略与通用功能">‹</button>' +
      '<button class="okx-nav-title-btn" data-act="sheet" data-sheet="create">' + esc(title) + " ▾</button>" +
      '<div class="okx-nav-icons"><span class="okx-icon-btn" data-act="sheet" data-sheet="tutorial">▣</span><span class="okx-icon-btn">···</span></div></header>' +
      '<div class="okx-pair-strip"><button class="okx-pair-name" data-act="cycle-inst" style="background:none;border:none">' +
      esc(inst.displayName) + " ▾</button>" +
      '<div class="okx-pair-price-cluster"><span class="' + (down ? "price-down" : "price-up") + ' tabular">' +
      fmt(inst.lastPx, inst.tickSz < 0.1 ? 2 : 1) + (down ? " ↓" : " ↑") +
      '</span><button class="okx-icon-btn" data-act="toggle-kline" title="展开/收起K线">📈 ' + (state.klineExpanded ? "▴" : "▾") + "</button></div></div>" +
      klineDrawer +
      '<div class="phone-body" id="phoneBody">' + body + "</div>" +
      '<div class="phone-bottom-cta">' + ctaHtml + "</div>"
    );
  }

  function cta(label, enabled) {
    return '<button class="okx-black-pill-btn" data-act="create"' + (enabled ? "" : ' disabled style="opacity:.45"') + ">" + esc(label) + "</button>";
  }

  function renderGridForm(contract) {
    const p = params();
    const calc = compute();
    const inst = activeInst();
    const unit = inst.marginCcy;
    const isCm = inst.instType === "SWAP_CM";
    const minTxt = contract
      ? "> " + fmt(calc.minInvest, isCm ? 4 : 2) + " " + unit
      : "> " + fmt(calc.minInvest || 60.2, 1) + " USDT";
    const profit = calc.profitMinPct
      ? fmt(calc.profitMinPct, 2) + "% ~ " + fmt(calc.profitMaxPct, 2) + "%"
      : "--";
    let html = "";
    if (!contract) {
      html += seg("modeTab", [["ai", "AI 策略"], ["manual", "手动创建"]]);
    }
    if (p.modeTab === "ai" && !contract) {
      const tiers = [
        ["中期均衡型", "+33.27%", "1~2个月", "73,544.5 - 89,779.6", 75],
        ["短期激进型", "+16.77%", "7~30天", "77,800.0 - 86,200.0", 42],
        ["长期稳健型", "+10.47%", "3~6个月", "68,000.0 - 96,000.0", 120]
      ];
      tiers.forEach((tr, idx) => {
        html += '<div class="okx-border-card"><div style="display:flex;justify-content:space-between"><b>' + tr[0] + '</b><span class="badge-soft">' + tr[2] + "</span></div>" +
          '<div class="price-up" style="font-size:19px;font-weight:800;margin:4px 0">' + tr[1] + ' <span style="font-size:11px;color:var(--okx-text-secondary)">回测年化收益率</span></div>' +
          '<div style="font-size:12px;color:var(--okx-text-secondary)">价格区间 ' + tr[3] + " · 网格数量 " + tr[4] + "</div>" +
          (idx === 0 ? '<div class="okx-chip-grid" style="margin-top:8px">' +
            ["500", "1000", "5000"].map((amt) => '<button class="okx-chip-option" data-act="set" data-key="investAmt" data-val="' + amt + '">' + amt + "</button>").join("") +
            '<button class="okx-chip-option selected" data-act="set" data-key="modeTab" data-val="manual">填入参数</button></div>' : "") +
          "</div>";
      });
      return shell(currentFolder().navTitle, html, cta("创建 AI 网格策略", true));
    }

    if (contract) {
      html += '<div class="okx-dir-3">' +
        '<button class="okx-dir-btn' + (p.direction === "long" ? " active-long" : "") + '" data-act="set" data-key="direction" data-val="long">做多</button>' +
        '<button class="okx-dir-btn' + (p.direction === "short" ? " active-short" : "") + '" data-act="set" data-key="direction" data-val="short">做空</button>' +
        '<button class="okx-dir-btn' + (p.direction === "neutral" ? " active-neutral" : "") + '" data-act="set" data-key="direction" data-val="neutral">中性</button></div>';
    }

    html += '<div class="okx-field-header"><span>价格区间 (' + (isCm ? "USD" : "USDT") + ')</span>' +
      '<button class="okx-icon-btn" style="font-size:12px;color:var(--okx-text-secondary)" data-act="clear-grid">🗑 清除参数</button></div>';
    html += '<div class="okx-range-pair">' +
      '<label class="okx-gray-input"><span class="okx-input-mini-label">最低价</span><span class="okx-input-row"><input data-act="input" data-key="minPx" value="' + esc(p.minPx) + '" inputmode="decimal" /></span></label>' +
      '<span class="okx-range-dash">—</span>' +
      '<label class="okx-gray-input"><span class="okx-input-mini-label">最高价</span><span class="okx-input-row"><input data-act="input" data-key="maxPx" value="' + esc(p.maxPx) + '" inputmode="decimal" /></span></label></div>';

    html += field("网格数量 (2 - 1,000) ⓘ", "gridNum", "格 (" + (p.runType === "2" ? "等比" : "等差") + ")");
    html += row("单网格收益率 (已扣手续费)", profit);

    if (contract && isCm) {
      html += '<div class="okx-border-card" style="background:var(--okx-bg-elevated);font-size:12px">' +
        row("预估卖出均价", fmt(calc.estSellAvgPx, 1) + " USD") +
        row("预估买入均价", fmt(calc.estBuyAvgPx, 1) + " USD") +
        row("30日年化资金费率", '<span class="price-up">+5.50%</span>') + "</div>";
    }

    if (contract) {
      html += '<div class="okx-invest-lever-row"><div style="flex:1">' + field("投资额 (保证金)", "investAmt", unit + " ▾", minTxt) + '</div>' +
        '<button class="okx-lever-box" data-act="sheet" data-sheet="lever">' + esc(p.lever) + "x ▾</button></div>";
    } else {
      html += field("投资额", "investAmt", unit + " ▾", minTxt);
    }
    html += '<div class="okx-error-text" data-error style="' + (calc.error ? "" : "display:none") + '">' + esc(calc.error || "") + "</div>";
    html += '<div class="okx-slider-wrap"><input type="range" min="0" max="' + (contract ? 5000 : 2000) + '" step="1" data-act="input" data-key="investAmt" value="' + esc(p.investAmt || 0) + '" /></div>';
    html += row("可用", "0.00 " + unit + " ⊕");

    if (contract) {
      html += toggle("☑ 自动预留保证金 ⓘ", "autoReserveMargin");
      html += row("实际保证金 + 预留保证金", p.investAmt > 0 ? fmt(calc.actualMargin, isCm ? 4 : 2) + " + " + fmt(calc.reservedMargin, isCm ? 4 : 2) : "--");
      html += row("含杠杆的实际投入", p.investAmt > 0 ? fmt(calc.leveragedNotional, isCm ? 4 : 2) + " " + unit : "--");
      html += row("预估强平价 (多 / 空)", (calc.liqLong ? fmt(calc.liqLong, 1) : "--") + " / " + (calc.liqShort ? fmt(calc.liqShort, 1) : "--"));
      html += '<button class="okx-kv-row" data-act="sheet" data-sheet="slippage" style="width:100%;background:none;border:none;cursor:pointer"><span class="okx-kv-label dotted">滑点控制</span><span class="okx-kv-val">' + (p.slippageOn ? "AI 推荐 " + esc(p.slippagePct) + "%" : "关") + " ›</span></button>";
    } else {
      html += toggle("利润复投 ⓘ", "profitReinvest");
      html += '<div class="okx-kv-row"><span class="okx-kv-label">自动赚币</span><span class="okx-kv-val price-up">+' + fmt(inst.simpleEarnApyPct || 2.79, 2) + "% 年化 ›</span></div>";
    }

    html += '<div class="okx-field-header" style="margin-top:8px;cursor:pointer" data-act="toggle-adv"><span>高级设置 ⓘ</span><span>' + (state.advExpanded ? "▴" : "▾") + "</span></div>";
    if (state.advExpanded) {
      html += '<div class="okx-field-header"><span>网格模式</span></div>' + chips("runType", [["1", "等差"], ["2", "等比"]]);
      if (!contract) html += linkRow("简单赚币分配", "earnAlloc");
      html += linkRow("网格随价格上移", "moveUp");
      if (!contract) html += linkRow("网格随价格下移", "moveDown");
      html += linkRow("预设开始条件", "startCond");
      html += linkRow("预设停止条件", "stopCond");
      html += linkRow("止盈 / 止损 (最新价/区间/收益率)", "tp");
    }

    const title = contract ? (isCm ? "合约网格 (币本位)" : currentFolder().navTitle) : currentFolder().navTitle;
    let btnHtml = cta("创建策略", calc.valid);
    if (contract && p.direction === "long") {
      btnHtml = '<button class="okx-btn-long" style="width:100%" data-act="create"' + (calc.valid ? "" : ' disabled style="opacity:.45"') + ">创建策略 (做多)</button>";
    } else if (contract && p.direction === "short") {
      btnHtml = '<button class="okx-btn-short" style="width:100%" data-act="create"' + (calc.valid ? "" : ' disabled style="opacity:.45"') + ">创建策略 (做空)</button>";
    }
    return shell(title, html, btnHtml);
  }

  function renderDca() {
    const p = params();
    const calc = compute();
    const contract = p.botType === "contract_dca";
    let html = "";
    if (contract) {
      html += '<div class="okx-dir-3" style="grid-template-columns:1fr 1fr">' +
        '<button class="okx-dir-btn' + (p.direction === "long" ? " active-long" : "") + '" data-act="set" data-key="direction" data-val="long">做多</button>' +
        '<button class="okx-dir-btn' + (p.direction === "short" ? " active-short" : "") + '" data-act="set" data-key="direction" data-val="short">做空</button></div>';
    }
    html += '<div class="okx-kv-row"><span class="okx-kv-label strong">触发条件 ⓘ</span>' +
      '<label style="font-size:12.5px;font-weight:700;cursor:pointer"><input type="checkbox" checked data-act="sheet" data-sheet="rsi" /> AI 参数 (' + (p.direction === "short" ? "短期进取建仓-做空" : "智能建仓") + ")</label></div>";
    html += '<div style="display:grid;grid-template-columns:1fr 1fr;gap:8px">' +
      field("跌多少加仓 (" + (contract ? "0.1-50%" : "0.2-50%") + ")", "pxStepsPct", "%") +
      field("单周期止盈目标", "tpPct", "%") + "</div>";
    html += '<div style="display:grid;grid-template-columns:1fr 1fr;gap:8px">' +
      field(contract ? "初次下单保证金" : "初次下单金额", "initOrdAmt", "USDT") +
      field(contract ? "加仓单保证金" : "加仓单金额", "safetyOrdAmt", "USDT") + "</div>";
    html += field("最大自动加仓次数 (0-100)", "maxSafetyOrds", "次");
    if (contract) {
      html += '<div class="okx-kv-row"><span class="okx-kv-label">杠杆倍数</span><button class="okx-lever-box" style="height:32px" data-act="sheet" data-sheet="lever">' + esc(p.lever) + "x ▾</button></div>";
      if (calc.highLeverWarning) html += '<div class="okx-error-text">' + esc(calc.highLeverWarning) + "</div>";
    } else {
      html += '<div class="okx-kv-row"><span class="okx-kv-label">专属权益 (赚币 + 质押)</span><span class="price-up" style="font-weight:700">+4.62% 年化 ›</span></div>';
    }
    html += '<div style="display:grid;grid-template-columns:1fr 1fr;gap:8px">' +
      field("加仓价差倍数", "pxStepsMult", "倍") +
      field("加仓金额倍数", "volMult", "倍") + "</div>";
    html += toggle("☑ 预留足够资金 ⓘ", "reserveFunds");
    if (contract) {
      html += toggle("下次投资使用利润复投", "profitReinvest");
      html += '<div class="okx-kv-row"><span class="okx-kv-label">自动划转保证金</span><span class="okx-kv-val">目标保证金率 350% ›</span></div>';
    }
    html += row(contract ? "投入保证金" : "当前周期最大投资额", fmt(calc.requiredMargin, 2) + " USDT");
    html += row("预估强平价", calc.liqPx ? fmt(calc.liqPx, 1) + " USDT" : "现货无强平");
    if (calc.error) html += '<div class="okx-error-text">' + esc(calc.error) + "</div>";
    html += '<div class="okx-field-header" style="margin-top:8px"><span>加仓委托阶梯预览</span></div>';
    html += '<table class="okx-table"><tr><th>轮次</th><th>触发价</th><th>金额</th><th>止盈目标</th></tr>' +
      (calc.ladder || []).slice(0, 6).map((r) => "<tr><td>" + esc(r.label) + "</td><td>" + fmt(r.triggerPx, 2) + "</td><td>" + fmt(r.orderAmt, 2) + "</td><td>" + fmt(r.tpPx, 2) + "</td></tr>").join("") +
      "</table>";
    const dcaBtns = !contract
      ? '<div class="okx-dual-btn-row"><button class="okx-chip-option selected" style="height:44px">预览</button>' + cta("创建策略", calc.valid) + "</div>"
      : cta("创建策略", calc.valid);
    return shell(currentFolder().navTitle, html, dcaBtns);
  }

  function renderPortfolio() {
    const p = params();
    const calc = compute();
    let html = seg("modeTab", [["ai", "AI 策略"], ["manual", "手动创建"]]);
    if (p.modeTab === "ai") {
      return renderLanding("ai");
    }
    const rem = Math.max(0, 100 - (calc.weightSum || 0));
    html += '<div class="okx-field-header"><span>还需配置 ' + rem + '% / 100%</span>' +
      '<div style="display:flex;gap:6px"><button class="okx-chip-option" style="padding:3px 8px;font-size:11px" data-act="equal-weights">平均分配</button><button class="okx-chip-option selected" style="padding:3px 8px;font-size:11px">⊕ 新增币种</button></div></div>';
    p.portfolioCoins.forEach((c, i) => {
      html += '<div class="okx-gray-input" style="margin-bottom:8px"><span class="okx-input-row"><b style="min-width:64px">' + esc(c.ccy) +
        '</b><input data-act="weight" data-idx="' + i + '" value="' + esc(c.weight) + '" inputmode="decimal" /><span class="okx-input-unit">%</span></span></div>';
    });
    html += field("投资额 (≥ " + (calc.minInvest || 30.31) + " USDT)", "investAmt", "USDT", "≥ " + (calc.minInvest || 30.31));
    html += toggle("☐ 允许使用交易币", "allowBaseCoins");
    if (calc.error) html += '<div class="okx-error-text" data-error>' + esc(calc.error) + "</div>";
    html += '<div class="okx-field-header"><span>自动调仓条件 (再平衡模式)</span></div>' + chips("rebalanceMode", [["proportional", "按比例 (5%)"], ["periodic", "按时间 (4小时)"]]);
    if (p.rebalanceMode === "proportional") html += field("偏离阈值", "rebalanceThresholdPct", "%");
    else html += field("检查周期", "rebalancePeriod", "");
    html += toggle("自动赚币和自动质押 (+4.62% 年化)", "autoEarn");
    (calc.coins || []).forEach((c) => {
      html += row(c.ccy + " 初始分配", fmt(c.allocUsdt, 2) + " USDT");
    });
    return shell(currentFolder().navTitle, html, cta("创建策略", calc.valid));
  }

  const H = { field, row, toggle, chips, seg, linkRow, shell, cta, fmt, esc };

  function renderArb() {
    return window.OkxBotScreens1To1.renderArbScreen(params(), compute(), activeInst(), H);
  }

  function renderSignal() {
    const p = params();
    const calc = compute();
    let html = '<div class="okx-field-header"><span>选择信号</span><button class="okx-chip-option selected" style="padding:3px 10px;font-size:11.5px" data-act="sheet" data-sheet="customSignal">＋ 新增自定义信号</button></div>';
    html += '<button class="okx-gray-input" data-act="sheet" data-sheet="customSignal" style="width:100%;text-align:left;cursor:pointer;margin-bottom:10px"><div class="okx-input-row"><span>' + esc(p.signalName) + '</span><span>▾</span></div></button>';
    html += '<div class="okx-field-header"><span>交易对</span><span class="badge-soft">⊕ 已选 ' + esc(p.selectedPairsCount) + " / 10 个 ›</span></div>";
    html += field("交易对数量 (最多 10 个)", "selectedPairsCount", "个");
    html += '<div class="okx-invest-lever-row"><div style="flex:1">' + field("投资额 (保证金 ≥ 50)", "investAmt", "USDT", "≥ 50") + '</div>' +
      '<button class="okx-lever-box" data-act="sheet" data-sheet="lever">' + esc(p.lever) + "x ▾</button></div>";
    if (calc.leverWarning) html += '<div class="okx-error-text">' + esc(calc.leverWarning) + "</div>";
    if (calc.error) html += '<div class="okx-error-text" data-error>' + esc(calc.error) + "</div>";
    html += '<div class="okx-kv-row"><span class="okx-kv-label">自动赚币</span><span class="price-up" style="font-weight:700">+2.81% 年化 ›</span></div>';
    html += '<div class="okx-field-header" style="margin-top:8px"><span>高级设置 ⓘ</span></div>';
    html += '<div class="okx-field-header"><span>委托类型</span></div>' + chips("sigOrderType", ["市价委托", "限价委托"]);
    html += '<div class="okx-field-header"><span>加仓设定</span></div>' + chips("addPosMode", ["占用可用保证金比例", "固定保证金"]);
    html += field("最大持仓数量", "maxPositions", "个");
    html += '<div class="okx-field-header"><span>止盈止损基准</span></div>' + chips("tpSlMode", ["收益率", "涨跌幅"]);
    html += '<div style="display:grid;grid-template-columns:1fr 1fr;gap:8px">' + field("止盈", "tpPct", "%") + field("止损", "slPct", "%") + "</div>";
    return shell("信号策略", html, cta("创建策略", calc.valid));
  }

  function renderSlice() {
    return window.OkxBotScreens1To1.renderSliceScreen(params(), compute(), activeInst(), H, M);
  }

  function renderRecurring() {
    const p = params();
    const calc = compute();
    let html = seg("modeTab", [["featured", "精选专区"], ["manual", "手动创建"]]);
    if (p.modeTab === "featured") {
      html += '<div class="okx-chip-grid" style="margin-bottom:10px"><span class="okx-chip-option selected">热门币种</span><span class="okx-chip-option">ETF</span><span class="okx-chip-option">大宗商品</span><span class="okx-chip-option">精选个股 <span class="badge-lime">xStock LIVE</span></span></div>';
      html += '<div class="okx-border-card"><div style="display:flex;justify-content:space-between"><b>科技五巨头组合 (xNVDA/xAAPL/xMSFT)</b><span class="badge-lime">xStock</span></div><div class="price-up" style="font-size:18px;font-weight:800;margin:4px 0">+51.52% <span style="font-size:11px;color:var(--okx-text-secondary)">3年历史回测</span></div><button class="okx-chip-option selected" data-act="set" data-key="modeTab" data-val="manual">一键跟投此组合</button></div>';
      html += '<div class="okx-border-card"><b>ETF 核心指数配置 (xQQQ / xSPY)</b><div style="color:var(--okx-text-secondary);font-size:12px;margin:4px 0">纳斯达克100 与标普500 链上代币每日定投</div><button class="okx-chip-option selected" data-act="set" data-key="modeTab" data-val="manual">用此组合手动创建</button></div>';
    }
    html += '<div class="okx-field-header"><span>定投币种 (1~20 个)</span><span class="badge-soft">买入 ' + esc(p.targetCcy) + " 100% ▾</span></div>";
    html += field("单次定投金额 (≥ 2 USDT)", "perCycleAmt", "USDT", "≥ 2");
    if (calc.error) html += '<div class="okx-error-text" data-error>' + esc(calc.error) + "</div>";
    html += '<div class="okx-field-header"><span>定投频率</span></div>' + chips("frequency", ["1小时", "4小时", "8小时", "12小时", "每天", "每周", "每月"]);
    html += '<div class="okx-field-header"><span>首次买入时间</span></div>' + chips("firstBuyTime", ["立即买入", "下个整点", "自选时间"]);
    html += toggle("其他账户自动补足 (自定义扣款顺序)", "autoSupplement");
    html += '<div class="okx-field-header"><span>买入价格区间 (高级可选)</span></div>';
    html += '<div class="okx-range-pair"><label class="okx-gray-input"><span class="okx-input-mini-label">最低价</span><span class="okx-input-row"><input data-act="input" data-key="minPrice" value="' + esc(p.minPrice) + '" placeholder="不限" /></span></label><span class="okx-range-dash">—</span><label class="okx-gray-input"><span class="okx-input-mini-label">最高价</span><span class="okx-input-row"><input data-act="input" data-key="maxPrice" value="' + esc(p.maxPrice) + '" placeholder="不限" /></span></label></div>';
    html += row("可用资金", fmt(p.availUsdt, 2) + " USDT ⊕");
    html += row("预计可定投次数", String(calc.estCycles || 0) + " 次");
    return shell("定投策略", html, cta("创建策略", calc.valid));
  }

  function renderDcd() {
    const p = params();
    const calc = compute();
    let html = seg("modeTab", [["ai", "AI 策略"], ["manual", "手动创建"]]);
    html += '<div class="okx-field-header"><span>买入方式 · 跌幅百分比 ▾</span></div>' + chips("dropPct", [["0.1", "0.1%"], ["1", "1%"], ["5", "5%"]]);
    html += '<div class="okx-field-header" style="margin-top:8px"><span>其他条件</span></div>';
    html += field("最低年化收益率 ⓘ (≥ 1%)", "minApyPct", "%");
    html += field("最长期限 (1~365 天)", "maxTenorDays", "天");
    html += '<div class="okx-border-card" style="background:var(--okx-bg-elevated)"><div style="display:flex;justify-content:space-between"><b>匹配到的双币产品 (BTC-USDT)</b><span class="badge-lime">自动匹配</span></div>' +
      row("买入目标价格", "<b>" + fmt(calc.buyTargetPx, 0) + " USDT</b>") +
      row("参考年化收益率", '<span class="price-up" style="font-weight:800">+' + fmt(calc.matchedApyPct, 2) + "%</span>") +
      row("投资期限", esc(calc.settlementHours || 16) + " 小时 (" + esc(calc.settlementTimeStr || "次日 16:00") + ")") + "</div>";
    html += field("申购投资额 (≥ 10 USDT)", "investAmt", "USDT", "≥ 10");
    if (calc.error) html += '<div class="okx-error-text" data-error>' + esc(calc.error) + "</div>";
    return shell("抄底止盈", html, '<button class="okx-black-pill-btn" data-act="sheet" data-sheet="dcdSub">下一步 (申购低价抄底 BTC)</button>');
  }

  function renderLanding(kind) {
    const p = params();
    if (kind === "stock-spot" || kind === "stock-fut") {
      const m = window.OkxBotScreens1To1.renderStockMatrixScreen(kind, p, compute(), H, M);
      if (m) return m;
      return renderGridForm(kind === "stock-fut");
    }
    const presets = [
      ["桥水基金全天候加密版", 18.14, "BTC 40% · PAXG 30% · ETH 20% · SOL 10%"],
      ["存储与储备组合", 12.60, "xMU 40% · xBMNR 35% · xREN 25%"],
      ["AI数据与储备组合", 8.39, "xNVDA 45% · xAMD 30% · BTC 25%"]
    ];
    let html = '<div style="font-weight:800;font-size:17px;margin-bottom:8px">官方 AI 组合与 BTC 生息单</div>';
    presets.forEach((pr) => {
      html += '<button class="okx-border-card" data-act="preset" data-name="' + esc(pr[0]) + '" style="width:100%;text-align:left;cursor:pointer"><div style="display:flex;justify-content:space-between"><b>' + esc(pr[0]) + '</b><span class="price-up" style="font-weight:800">+' + pr[1].toFixed(2) + '%</span></div><div style="color:var(--okx-text-secondary);font-size:12px;margin-top:4px">' + pr[2] + " · 偏离 5% 自动调仓</div></button>";
    });
    html += '<div class="okx-border-card"><b>BTC 生息单 · 6 大全天候收益引擎</b><div style="display:grid;grid-template-columns:1fr 1fr;gap:6px;margin-top:8px">' +
      [["网格套利", "捕捉震荡价差"], ["双币生息", "挂单享高额票息"], ["自动赚币", "+2.79% 余币宝"], ["利润复投", "网格收益滚入本金"], ["随价格移动", "破区间自动上移"], ["预设停止", "触发止盈自动赎回"]].map((x) =>
        '<div style="background:var(--okx-bg-elevated);padding:6px 8px;border-radius:8px"><div style="font-weight:700;font-size:12px">' + x[0] + '</div><div style="font-size:10.5px;color:var(--okx-text-secondary)">' + x[1] + "</div></div>"
      ).join("") + "</div></div>";
    const calc = compute();
    if (calc.error) html += '<div class="okx-error-text" data-error>' + esc(calc.error) + "</div>";
    html += field("组合投资额", "investAmt", "USDT", "≥ " + (calc.minInvest || 30));
    html += row("当前权重合计", (calc.weightSum || 100) + "%");
    return shell("AI 策略与组合", html, cta("一键创建此 AI 组合", calc.valid));
  }

  function renderHub() {
    const W = window.OkxWholeAppViews;
    const cards = [
      ["xMU/USDT", "+12.54%", "1,922", "12_股票代币现货网格_StockTokenSpotGrid", "xMU-USDT"],
      ["xSOXL/USDT", "+27.15%", "418", "12_股票代币现货网格_StockTokenSpotGrid", "xSOXL-USDT"],
      ["xSNDK/USDT", "+25.67%", "786", "12_股票代币现货网格_StockTokenSpotGrid", "xSNDK-USDT"]
    ];
    const quick = [
      ["＋", "创建策略", "create"],
      ["▦", "现货网格", "01_现货网格_SpotGrid"],
      ["5x", "合约网格", "02_合约网格_FuturesGrid"],
      ["❖", "智能组合", "03_智能组合_SmartPortfolio"],
      ["📅", "定投策略", "10_定投策略_RecurringBuy"]
    ];
    let html = W.renderTopTradeBar(state.tradeTab);
    html += '<div style="padding:10px 16px 2px"><div class="okx-kv-label">总资产估值 👁</div><div style="font-size:28px;font-weight:900" class="tabular">0.00 <span style="font-size:14px">USD ▾</span></div><div class="okx-kv-label dotted">$0.00 今日收益 ⌄</div></div>';
    html += '<div class="okx-circle-grid-5">';
    quick.forEach((q, i) => {
      const act = q[2] === "create" ? 'data-act="sheet" data-sheet="create"' : 'data-act="folder" data-id="' + q[2] + '"';
      html += '<button class="okx-circle-item" ' + act + ' style="background:none;border:none;cursor:pointer"><div class="okx-circle-icon' + (i === 0 ? " black" : "") + '">' + q[0] + "</div><span>" + q[1] + (q[1] === "智能组合" ? ' <span class="badge-lime">New</span>' : "") + "</span></button>";
    });
    html += "</div>";
    html += '<div class="okx-chip-grid" style="padding:0 16px 8px">' +
      ["推荐", "我的策略 (" + state.running.length + ") ▾", "我的跟单"].map((tab) => {
        const key = tab.startsWith("我的策略") ? "我的策略" : tab;
        return '<button class="okx-chip-option' + (state.hubSubTab === key ? " selected" : "") + '" data-act="hub-subtab" data-val="' + key + '">' + tab + "</button>";
      }).join("") + "</div>";

    html += '<div class="phone-body" id="phoneBody" style="padding-top:0">';
    if (state.hubSubTab === "我的策略") {
      state.running.forEach((b) => {
        html += '<div class="okx-border-card" style="display:flex;justify-content:space-between;align-items:center">' +
          '<div><b>' + esc(b.name) + '</b> <span class="badge-soft">' + esc(b.status) + '</span><div style="font-size:11.5px;color:var(--okx-text-secondary);margin-top:3px">Rust Interactor · 实时撮合中</div></div>' +
          '<div style="text-align:right"><div class="' + (b.pnl >= 0 ? "price-up" : "price-down") + ' tabular" style="font-weight:800">' + (b.pnl >= 0 ? "+" : "") + fmt(b.pnl, 2) + " USDT</div></div></div>";
      });
    } else if (state.hubSubTab === "我的跟单") {
      html += '<div class="okx-border-card"><b>分润跟单 (`SocialTradingSdk`)</b><div style="color:var(--okx-text-secondary);font-size:12px;margin-top:4px">当前暂无跟单持仓，前往「星球」或推荐列表一键复制带单员策略。</div></div>';
    } else {
      html += '<div class="okx-border-card"><div style="display:flex;justify-content:space-between"><b>股票代币现货网格</b><span class="badge-lime">xStock</span></div><div style="color:var(--okx-text-secondary);font-size:12px;margin:4px 0 8px">锚定股票代币波动特性，网格自动捕捉每一次起落</div><span class="badge-soft">|||| 稳健型</span> <span style="color:var(--okx-text-secondary);font-size:12px">3,591 人已使用</span>';
      cards.forEach((c) => {
        html += '<div class="okx-kv-row"><div><b>' + c[0] + '</b><div class="price-up" style="font-weight:800">' + c[1] + '</div><div class="okx-kv-label" style="font-size:11px">' + c[2] + ' 人已使用</div></div><button class="okx-black-pill-btn" style="width:auto;height:32px;padding:0 14px" data-act="pick-inst" data-folder="' + c[3] + '" data-id="' + c[4] + '">创建</button></div>';
      });
      html += "</div>";
      html += '<div class="okx-border-card" data-act="folder" data-id="14_AI策略与AI组合_AI_Strategy"><b>BTC 生息单</b> · 欧易独家！网格套利 × 双币生息，让资金全天候生息</div>';
      html += '<div class="okx-border-card" data-act="folder" data-id="03_智能组合_SmartPortfolio"><b>智能组合</b> · 选择交给自己，再平衡交给规则，陪你穿越周期</div>';
    }
    html += "</div>";
    html += W.renderBottomAppTabBar(state.appTab);
    return html;
  }

  function renderPhone() {
    const W = window.OkxWholeAppViews;
    const f = currentFolder();
    const mount = document.getElementById("phoneScreenMount");
    let html = "";
    if (state.appTab === "home") html = W.renderHomeView(M) + W.renderBottomAppTabBar(state.appTab);
    else if (state.appTab === "explore") html = W.renderExploreView() + W.renderBottomAppTabBar(state.appTab);
    else if (state.appTab === "planet") html = W.renderPlanetView() + W.renderBottomAppTabBar(state.appTab);
    else if (state.appTab === "assets") html = W.renderAssetsView(state.running) + W.renderBottomAppTabBar(state.appTab);
    else if (state.tradeTab !== "策略&跟单" && f.botType === "hub_common") {
      html = W.renderNonBotTradeMode(state.tradeTab, activeInst(), M) + W.renderBottomAppTabBar(state.appTab);
    } else if (f.botType === "hub_common") html = renderHub();
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
      inner += '<div class="okx-chip-grid" style="margin-bottom:10px">' + ["全部", "现货", "合约"].map((t) =>
        '<button class="okx-chip-option' + (state.filter === t ? " selected" : "") + '" data-act="filter" data-val="' + t + '">' + t + "</button>").join("") + "</div>";
      let group = "";
      CREATE_MENU.forEach((item) => {
        if (state.filter === "现货" && /合约|冰山|时间加权|信号/.test(item.title + item.desc)) return;
        if (state.filter === "合约" && /现货网格|定投|抄底|智能组合|股票代币现货/.test(item.title)) return;
        if (item.group && item.group !== group) {
          group = item.group;
          inner += '<div class="okx-field-header" style="margin-top:8px"><span>' + esc(group) + "</span></div>";
        }
        inner += '<button class="okx-border-card" data-act="menu" data-folder="' + esc(item.folder) + '" data-inst="' + esc(item.instId || "") + '" style="width:100%;text-align:left;cursor:pointer"><b>' + esc(item.title) + "</b> " +
          (item.badge ? '<span class="badge-lime">' + esc(item.badge) + "</span>" : "") +
          '<div style="color:var(--okx-text-secondary);font-size:12px">' + esc(item.desc) + "</div></button>";
      });
    } else if (kind === "lever") {
      const max = activeInst().maxLever || 100;
      const opts = [1, 2, 3, 5, 10, 20, 30, 50, 75, 100].filter((n) => n <= max);
      inner += chips("lever", opts.map((n) => [String(n), n + "x"]));
      if (Number(p.lever) > 10) inner += '<div class="okx-error-text">⚠ 超过 10x 杠杆会显著增加强制平仓风险</div>';
      inner += '<button class="okx-green-pill-btn" data-act="close-sheet" style="margin-top:12px">确认</button>';
    } else if (kind === "slippage") {
      inner += toggle("滑点控制", "slippageOn");
      inner += field("滑点 0.0001–5 (%)", "slippagePct", "%");
      inner += chips("slippagePct", [["0.05", "0.05%"], ["0.2", "0.2%"], ["0.5", "0.5%"], ["1", "1%"], ["5", "5%"]]);
      inner += row("AI 推荐滑点", "0.0004%");
      inner += row("当前价格", fmt(activeInst().lastPx, 1) + " USDT");
      inner += '<p style="color:var(--okx-text-secondary);margin:8px 0;font-size:12px">滑点控制可让成交价更接近预期价格。若不设置，系统以最快速度执行。</p>';
      inner += '<button class="okx-green-pill-btn" data-act="close-sheet">确认</button>';
    } else if (kind === "customSignal") {
      inner += field("信号名称 (≤50 字符)", "signalName", "", "例如 TV_BTC_Breakout");
      inner += field("信号简介 (≤1500 字符)", "signalIntro", "", "TradingView 趋势突破策略");
      inner += '<div class="okx-border-card" style="font-size:11.5px;font-family:var(--okx-mono);background:var(--okx-bg-elevated)">Webhook: https://www.okx.com/pap/algo/signal/webhook<br/>Alert JSON: {"action":"ENTER_LONG","instrument":"BTC-USDT-SWAP","signalToken":"okx_tv_v2"}</div>';
      inner += '<button class="okx-black-pill-btn" data-act="close-sheet">创建并绑定信号</button>';
    } else if (kind === "arbPreview") {
      const calc = compute();
      inner += row("左腿 (现货买入)", fmt(calc.spotAllocUsdt, 2) + " USDT");
      inner += row("右腿 (永续做空保证金)", fmt(calc.swapMarginUsdt, 2) + " USDT");
      inner += row("3日综合年化", '<span class="price-up">+' + fmt(calc.total3dApy, 2) + "%</span>");
      inner += '<button class="okx-black-pill-btn" data-act="create" style="margin-top:10px">确认投入套利</button>';
    } else if (kind === "dcdSub") {
      const calc = compute();
      inner += row("抄底目标价", fmt(calc.buyTargetPx, 0) + " USDT");
      inner += row("参考年化收益率", '<span class="price-up">+' + fmt(calc.matchedApyPct, 2) + "%</span>");
      inner += row("申购金额", fmt(p.investAmt, 2) + " USDT");
      inner += '<button class="okx-black-pill-btn" data-act="create" style="margin-top:10px">确认申购</button>';
    } else if (kind === "rsi" || kind === "startCond") {
      inner += field("RSI-14 阈值 (3 分钟 / 低于触发)", "rsiTrigger", "", "例如 30");
      inner += '<p style="color:var(--okx-text-secondary);font-size:12px;margin:6px 0">支持立即触发、价格触发、RSI 技术指标或 TradingView 信号触发。</p>';
      inner += '<button class="okx-black-pill-btn" data-act="apply-cond" data-key="startCond" data-val="RSI-14 < 30">确认设为开始条件</button>';
    } else if (kind === "stopCond") {
      inner += '<button class="okx-black-pill-btn" data-act="apply-cond" data-key="stopCond" data-val="价格触发">设为价格触发停止</button>';
    } else if (kind === "tp" || kind === "sl" || kind === "moveUp" || kind === "moveDown" || kind === "earnAlloc") {
      inner += '<div class="okx-chip-grid" style="margin-bottom:8px"><span class="okx-chip-option selected">最新价</span><span class="okx-chip-option">价格区间</span><span class="okx-chip-option">止盈止损收益率</span></div>';
      inner += field(sheetTitle(kind), kind, "", "输入数值后点确认");
      inner += '<button class="okx-black-pill-btn" data-act="close-sheet">确认</button>';
    } else if (kind === "stockRisk") {
      inner += "<p style=\"font-size:12.5px;line-height:1.6\">股票代币合约锚定美股标的，请注意休市跳空、盘前盘后流动性变化、资金费率结算与强平风险。勾选确认后可进入网格参数配置。</p>";
      inner += '<button class="okx-green-pill-btn" style="margin-top:12px" data-act="ack-risk">我已阅读并同意风险提示</button>';
    } else if (kind === "marginHelp" || kind === "tutorial") {
      inner += "<p style=\"font-size:12.5px\">1. 全仓模式：账户内所有可用保证金共享抗风险。<br/>2. 逐仓模式：单笔策略独立核算保证金与强平价。<br/>3. 策略交易支持闲置资金自动转入简单赚币复利生息。</p>";
      inner += '<button class="okx-black-pill-btn" style="margin-top:10px" data-act="close-sheet">知道了</button>';
    }
    return '<div class="phone-sheet-mask" data-act="close-sheet"></div><div class="phone-sheet">' + inner + "</div>";
  }

  function sheetTitle(kind) {
    return {
      create: "创建策略",
      lever: "调整杠杆",
      slippage: "滑点控制设置",
      customSignal: "新增自定义 TradingView 信号",
      arbPreview: "智能套利策略预览",
      dcdSub: "申购 - 以低价抄底 BTC",
      rsi: "RSI-14 触发条件设置",
      startCond: "预设开始条件",
      stopCond: "预设停止条件",
      tp: "止盈 / 止损设置",
      sl: "止损设置",
      moveUp: "网格随价格上移",
      moveDown: "网格随价格下移",
      earnAlloc: "简单赚币分配阈值",
      stockRisk: "股票合约风险提示",
      marginHelp: "保证金模式说明 (1/9)",
      tutorial: "策略交易图文教程"
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
    state.appTab = "trade";
    state.tradeTab = "策略&跟单";
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
    if (/套利下单|双腿/.test(label)) p.legMode = "dual";
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
    else if (act === "app-tab") {
      state.appTab = t.getAttribute("data-tab");
      renderPhone();
    } else if (act === "trade-tab") {
      state.appTab = "trade";
      state.tradeTab = t.getAttribute("data-tab");
      if (state.tradeTab === "策略&跟单" && currentFolder().botType !== "hub_common") {
        state.folderId = "00_公共组件_创建策略与通用功能";
      }
      renderAll();
    } else if (act === "hub-subtab") {
      state.hubSubTab = t.getAttribute("data-val");
      renderPhone();
    } else if (act === "toggle-kline") {
      state.klineExpanded = !state.klineExpanded;
      renderPhone();
    } else if (act === "kline-pd") {
      state.klinePeriod = t.getAttribute("data-val");
      renderPhone();
    } else if (act === "kline-ind") {
      state.klineIndicator = state.klineIndicator === "EMA" ? "MA" : "EMA";
      renderPhone();
    } else if (act === "toggle-adv") {
      state.advExpanded = !state.advExpanded;
      renderPhone();
    } else if (act === "clear-grid") {
      p.minPx = "";
      p.maxPx = "";
      p.gridNum = "";
      renderAll();
    } else if (act === "equal-weights") {
      const n = (p.portfolioCoins || []).length || 1;
      const base = Math.floor(100 / n);
      p.portfolioCoins.forEach((c, i) => {
        c.weight = i === 0 ? 100 - base * (n - 1) : base;
      });
      renderAll();
    } else if (act === "set") {
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
