/**
 * OKX v6.194.0 1:1 Screenshot-Exact Bot Screen Renderers (Part 2: Folders 06–14)
 * ==============================================================================
 * Calibrated 1:1 against `/策略BOT截图/06_*` through `14_*`.
 */
(function (global) {
  "use strict";

  function renderOrderBookColumn(inst, M) {
    const px = inst.lastPx;
    const step = inst.tickSz || 0.01;
    const asks = [5, 4, 3, 2, 1].map((i) => [M.formatNum(px + i * step * 12, 2), (1.2 + i * 0.7).toFixed(1)]);
    const bids = [1, 2, 3, 4, 5].map((i) => [M.formatNum(px - i * step * 12, 2), (1.5 + i * 0.8).toFixed(1)]);
    return (
      '<div class="okx-mini-ob">' +
      '<div class="okx-mini-ob-row" style="color:var(--okx-text-tertiary);margin-bottom:4px"><span>价格(USDT)</span><span>数量(' + inst.baseCcy + ")</span></div>" +
      asks.map((r) => '<div class="okx-mini-ob-row"><span class="price-down">' + r[0] + "</span><span>" + r[1] + "</span></div>").join("") +
      '<div style="padding:6px 4px;margin:4px 0;border-top:1px solid var(--okx-border);border-bottom:1px solid var(--okx-border)">' +
      '<div style="font-size:14px;font-weight:800" class="price-down">' + M.formatNum(px, 2) + "</div>" +
      '<div style="font-size:10px;color:var(--okx-text-secondary)">指数 ' + M.formatNum(px * 0.9998, 2) + "</div></div>" +
      bids.map((r) => '<div class="okx-mini-ob-row"><span class="price-up">' + r[0] + "</span><span>" + r[1] + "</span></div>").join("") +
      '<div class="okx-mini-ob-row" style="margin-top:6px"><span class="badge-soft">0.01 ▾</span><span class="badge-soft">深度</span></div>' +
      "</div>"
    );
  }

  function renderArbScreen(p, calc, inst, H) {
    let html = H.seg("legMode", [["smart", "智能套利"], ["dual", "套利下单"]]);
    if (p.legMode !== "dual") {
      html += H.chips("arbMarketTab", ["Crypto", "Tradfi"]);
      const cards = p.arbMarketTab === "Tradfi"
        ? [["xMU", "+12.61%", "费率套利"], ["xMRVL", "+13.28%", "费率套利"], ["xAMD", "+12.64%", "费率套利"]]
        : [["SOL", "+20.75%", "质押 6.85% + 费率 11.40% + 奖励 2.50%"], ["BTC", "+11.43%", "现货 + 永续资金费率"], ["ETH", "+13.35%", "质押 3.90% + 费率 9.45%"]];
      cards.forEach((c) => {
        html += '<div class="okx-border-card" style="display:flex;justify-content:space-between;align-items:center">' +
          '<div><b>' + c[0] + ' 套利</b> <span class="badge-soft">' + c[2] + '</span><div class="price-up" style="font-weight:800;margin-top:3px">' + c[1] + " 3日年化</div></div>" +
          '<button class="okx-chip-option selected" data-act="sheet" data-sheet="arbPreview">策略预览</button></div>';
      });
      html += '<div class="okx-invest-lever-row"><div style="flex:1">' + H.field("投资额 (≥ 50 USDT)", "investAmt", "USDT", "≥ 50") + '</div>' +
        '<button class="okx-lever-box" data-act="sheet" data-sheet="lever">' + H.esc(p.lever) + "x ▾</button></div>";
      html += H.toggle("借币补充保证金 (VIP 专享)", "borrowUsdt");
      if (calc.error) html += '<div class="okx-error-text" data-error>' + H.esc(calc.error) + "</div>";
      html += H.row("现货买入分配", H.fmt(calc.spotAllocUsdt, 2) + " USDT");
      html += H.row("永续做空保证金", H.fmt(calc.swapMarginUsdt, 2) + " USDT");
      html += H.row("两腿价差率", H.fmt(calc.spreadPct, 3) + "%");
      html += H.row("预估开仓净盈亏", H.fmt(calc.estOpenPnl, 2) + " USDT");
      return H.shell("智能套利", html, H.cta("策略预览", calc.valid));
    }

    // Mode 2: 套利下单 (Dual-Leg Order Book & Execution Workbench)
    html += '<div class="okx-border-card" style="background:var(--okx-bg-elevated);padding:8px 12px;font-size:12px;display:flex;justify-content:space-between;align-items:center">' +
      '<span>✨ 体验新版智能套利，一键自动平衡两腿仓位</span><button class="okx-chip-option" data-act="set" data-key="legMode" data-val="smart">去体验</button></div>';
    html += '<div style="display:grid;grid-template-columns:1fr 1fr;gap:8px;margin-bottom:8px">' +
      '<div class="okx-border-card" style="margin:0"><span class="badge-soft" style="color:var(--okx-down-red)">卖</span> <b>BTCUSDT 永续</b><div class="tabular">' + H.fmt(calc.swapPx, 1) + "</div></div>" +
      '<div class="okx-border-card" style="margin:0"><span class="badge-soft" style="color:var(--okx-up-green)">买</span> <b>BTC/USDT 现货</b><div class="tabular">' + H.fmt(calc.spotPx, 1) + "</div></div></div>";
    html += '<div class="okx-kv-row"><span>本期资金费率 (00:52:48)</span><span class="price-up">+0.0120% (年化 +13.14%)</span></div>';
    html += '<div style="display:grid;grid-template-columns:1fr 1fr;gap:8px">' +
      '<div><div class="okx-field-header"><span>左腿·永续委托</span></div>' + H.chips("swapPriceType", ["排队价", "超价", "对手价", "市价"]) + "</div>" +
      '<div><div class="okx-field-header"><span>右腿·现货委托</span></div>' + H.chips("spotPriceType", ["排队价", "超价", "对手价", "市价"]) + "</div></div>";
    html += H.field("双腿同步委托金额", "investAmt", "USDT", "≥ 50");
    html += '<div class="okx-kv-row"><label><input type="checkbox" checked /> 同数量</label><label><input type="checkbox" checked /> 同金额</label><label><input type="checkbox" checked /> 只减仓保护</label></div>';
    const dualCta = '<div class="okx-dual-btn-row"><button class="okx-btn-short" data-act="create">卖出永续 (左腿)</button><button class="okx-btn-long" data-act="create">买入现货 (右腿)</button></div>';
    return H.shell("套利下单", html, dualCta);
  }

  function renderSliceScreen(p, calc, inst, H, M) {
    const twap = p.botType === "twap";
    let left = '<div class="okx-open-close-bar">' +
      '<button class="okx-oc-btn' + (p.posAction === "open" ? " active-open" : "") + '" data-act="set" data-key="posAction" data-val="open">开仓</button>' +
      '<button class="okx-oc-btn' + (p.posAction === "close" ? " active-close" : "") + '" data-act="set" data-key="posAction" data-val="close">平仓</button></div>';
    left += '<div style="display:flex;gap:6px;margin-bottom:10px">' +
      '<button class="okx-lever-box" style="flex:1;height:36px;justify-content:center" data-act="sheet" data-sheet="marginHelp">' + H.esc(p.marginMode) + " ▾</button>" +
      '<button class="okx-lever-box" style="flex:1;height:36px;justify-content:center" data-act="sheet" data-sheet="lever">' + H.esc(p.lever) + "x ▾</button></div>";
    left += '<div class="okx-field-header"><span>' + (twap ? "价格区间 ⓘ" : "挂单设置 ⓘ") + '</span><span class="badge-soft">价距 (USDT) ▾</span></div>';
    left += H.field("吃单价优于盘口", "priceOffsetUsdt", "USDT");
    left += H.field("吃单限制价", "limitPx", "USDT");
    if (twap) {
      left += H.field("订单的时间间隔 (5~120)", "timeIntervalSec", "秒", "5 ~ 120");
    } else {
      left += H.field("单次挂单数 (1~5)", "activeOrdersCount", "笔", "≤ 5");
    }
    left += H.field("单笔委托量", "singleOrdSz", inst.baseCcy);
    left += H.field("委托总量", "totalSz", inst.baseCcy);
    left += H.row("可用", "0.00 USDT");
    left += H.row("可开多 / 可开空", "0 " + inst.baseCcy + " / 0 " + inst.baseCcy);
    const splitHtml = '<div class="okx-orderbook-split"><div>' + left + "</div>" + renderOrderBookColumn(inst, M) + "</div>" +
      '<div class="okx-center-toast" data-error style="' + (calc.error ? "" : "display:none") + '">⊗ ' + H.esc(calc.error || "") + "</div>";
    const dualBtns = '<div class="okx-dual-btn-row">' +
      '<button class="okx-btn-long" data-act="create"' + (calc.valid ? "" : ' disabled style="opacity:.45"') + ">开多</button>" +
      '<button class="okx-btn-short" data-act="create"' + (calc.valid ? "" : ' disabled style="opacity:.45"') + ">开空</button></div>";
    return H.shell(twap ? "时间加权委托" : "冰山委托", splitHtml, dualBtns);
  }

  function renderStockMatrixScreen(kind, p, calc, H, M) {
    if (p.modeTab === "manual") {
      return null; // Handled by 1:1 Grid form
    }
    const isFut = kind === "stock-fut";
    const styleTabs = ["长期稳健型", "中期进取型", "短期高频型"];
    const curStyle = p.matrixStyle || (isFut ? "中期进取型" : "长期稳健型");
    let html = H.seg("modeTab", [["landing", "推荐专区"], ["manual", "手动创建"]]);
    html += '<div class="okx-border-card" style="background:var(--okx-bg-elevated)">' +
      '<div style="display:flex;justify-content:space-between;align-items:center"><b>' + (isFut ? "美股代币合约网格" : "股票代币现货网格") + '</b><span class="badge-lime">xStock</span></div>' +
      '<div style="font-size:12px;color:var(--okx-text-secondary);margin-top:4px">锚定美股科技龙头波动，累计 5,299 人使用 · 历史投入 4.07M USDT</div></div>';
    html += '<div class="okx-3x3-matrix">' +
      styleTabs.map((st) =>
        '<div class="okx-matrix-cell' + (curStyle === st ? " selected" : "") + '" data-act="set" data-key="matrixStyle" data-val="' + st + '">' + st + "</div>"
      ).join("") + "</div>";

    const items = isFut
      ? [["TSLA-USDT-SWAP", "TSLA 永续 5x", "+82.10%", "1,840 人"], ["TSLA-USDT-SWAP", "HOOD 永续 5x", "+248.64%", "1,420 人"], ["TSLA-USDT-SWAP", "SPCX 永续 3x", "+38.20%", "912 人"], ["TSLA-USDT-SWAP", "AMZN 永续 5x", "+294.39%", "1,127 人"]]
      : [["xMU-USDT", "xMU/USDT", "+12.54%", "1,922 人"], ["xSOXL-USDT", "xSOXL/USDT", "+27.15%", "418 人"], ["xSNDK-USDT", "xSNDK/USDT", "+25.67%", "786 人"], ["xNVDA-USDT", "xNVDA/USDT", "+5.68%", "1,420 人"], ["xAMD-USDT", "xAMD/USDT", "+11.77%", "615 人"]];
    items.forEach((it) => {
      html += '<div class="okx-border-card" style="display:flex;justify-content:space-between;align-items:center">' +
        '<div><b>' + it[1] + '</b> <span class="badge-soft">' + curStyle + '</span><div class="price-up" style="font-size:16px;font-weight:800;margin-top:2px">' + it[2] + ' <span style="font-size:11px;color:var(--okx-text-secondary)">回测年化 · ' + it[3] + "</span></div></div>" +
        '<button class="okx-black-pill-btn" style="width:auto;height:34px;padding:0 16px" data-act="' + (isFut ? "sheet" : "pick-inst") + '" data-sheet="stockRisk" data-id="' + it[0] + '">创建</button></div>';
    });
    return H.shell(isFut ? "美股代币合约网格" : "股票代币现货网格", html,
      '<button class="okx-black-pill-btn" data-act="' + (isFut ? "sheet" : "set") + '" data-sheet="stockRisk" data-key="modeTab" data-val="manual">手动创建网格策略</button>');
  }

  global.OkxBotScreens1To1 = {
    renderOrderBookColumn,
    renderArbScreen,
    renderSliceScreen,
    renderStockMatrixScreen
  };
})(window);
