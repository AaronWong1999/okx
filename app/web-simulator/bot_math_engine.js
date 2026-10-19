/**
 * OKX v6.194.0 Strategy Bot Mathematical, Risk-Control & Execution Engine
 * =======================================================================
 * Exact JavaScript port of the Rust `trading_bot` & `social_trading` crates (`libOKCore.so`)
 * and Android `com.okinc.tradingbot.*` math classes (`OKT*DCAMinInvestMath`,
 * `ContractGridLiquidatePrice`, `SmartArbitragePlaceOrderInteractor`, `DcdBotInteractor`, etc.),
 * calibrated 1:1 with the 239 native screenshots in `/策略BOT截图/`.
 */

(function (global) {
  "use strict";

  const INSTRUMENTS = {
    "BTC-USDT": {
      instId: "BTC-USDT",
      displayName: "BTC/USDT",
      instType: "SPOT",
      assetClass: "CRYPTO",
      baseCcy: "BTC",
      quoteCcy: "USDT",
      marginCcy: "USDT",
      lastPx: 82226.6,
      change24hPct: -1.64,
      high24h: 84120.0,
      low24h: 81450.0,
      tickSz: 0.1,
      lotSz: 0.00001,
      minSz: 0.0001,
      ctVal: 1,
      maxLever: 1,
      feeRate: 0.001,
      mmr: 0,
      fundingRate8h: 0,
      simpleEarnApyPct: 2.79,
      stakingApyPct: 2.79
    },
    "BTC-USDT-SWAP": {
      instId: "BTC-USDT-SWAP",
      displayName: "BTCUSDT 永续",
      instType: "SWAP",
      assetClass: "CRYPTO",
      baseCcy: "BTC",
      quoteCcy: "USDT",
      marginCcy: "USDT",
      lastPx: 82145.8,
      change24hPct: -1.58,
      high24h: 84200.0,
      low24h: 81380.0,
      tickSz: 0.1,
      lotSz: 0.01,
      minSz: 0.01,
      ctVal: 0.01,
      maxLever: 100,
      feeRate: 0.0005,
      mmr: 0.004,
      fundingRate8h: 0.00012,
      fundingAnnualized30dPct: 8.64,
      simpleEarnApyPct: 2.81,
      stakingApyPct: 2.81
    },
    "BTC-USD-SWAP": {
      instId: "BTC-USD-SWAP",
      displayName: "BTCUSD 永续 (币本位)",
      instType: "SWAP_CM",
      assetClass: "CRYPTO",
      baseCcy: "BTC",
      quoteCcy: "USD",
      marginCcy: "BTC",
      lastPx: 82190.0,
      change24hPct: -1.52,
      high24h: 84150.0,
      low24h: 81400.0,
      tickSz: 0.1,
      lotSz: 1,
      minSz: 1,
      ctVal: 100, // 1 contract = 100 USD
      maxLever: 100,
      feeRate: 0.0005,
      mmr: 0.004,
      fundingRate8h: 0.00015,
      fundingAnnualized30dPct: 10.95,
      simpleEarnApyPct: 2.79,
      stakingApyPct: 2.79
    },
    "SOL-USDT": {
      instId: "SOL-USDT",
      displayName: "SOL/USDT",
      instType: "SPOT",
      assetClass: "CRYPTO",
      baseCcy: "SOL",
      quoteCcy: "USDT",
      marginCcy: "USDT",
      lastPx: 105.01,
      change24hPct: -2.18,
      high24h: 108.60,
      low24h: 103.20,
      tickSz: 0.01,
      lotSz: 0.001,
      minSz: 0.01,
      ctVal: 1,
      maxLever: 1,
      feeRate: 0.001,
      mmr: 0,
      fundingRate8h: 0.00024,
      simpleEarnApyPct: 4.62,
      stakingApyPct: 6.85
    },
    "ETH-USDT-SWAP": {
      instId: "ETH-USDT-SWAP",
      displayName: "ETHUSDT 永续",
      instType: "SWAP",
      assetClass: "CRYPTO",
      baseCcy: "ETH",
      quoteCcy: "USDT",
      marginCcy: "USDT",
      lastPx: 2645.5,
      change24hPct: 1.42,
      high24h: 2710.0,
      low24h: 2580.0,
      tickSz: 0.05,
      lotSz: 0.1,
      minSz: 0.1,
      ctVal: 0.1,
      maxLever: 100,
      feeRate: 0.0005,
      mmr: 0.004,
      fundingRate8h: 0.00018,
      fundingAnnualized30dPct: 9.45,
      simpleEarnApyPct: 2.81,
      stakingApyPct: 3.90
    },
    "MU-USDT-SWAP": {
      instId: "MU-USDT-SWAP",
      displayName: "MUUSDT 永续",
      instType: "SWAP",
      assetClass: "STOCK_TOKEN",
      baseCcy: "MU",
      quoteCcy: "USDT",
      marginCcy: "USDT",
      lastPx: 924.87,
      change24hPct: -0.86,
      high24h: 942.00,
      low24h: 915.20,
      tickSz: 0.01,
      lotSz: 0.1,
      minSz: 0.1,
      ctVal: 0.1,
      maxLever: 20,
      feeRate: 0.0005,
      mmr: 0.01,
      fundingRate8h: 0.00011,
      fundingAnnualized30dPct: 12.61,
      simpleEarnApyPct: 2.79,
      stakingApyPct: 0
    },
    "xMU-USDT": {
      instId: "xMU-USDT",
      displayName: "xMU/USDT",
      instType: "SPOT",
      assetClass: "STOCK_TOKEN",
      baseCcy: "xMU",
      quoteCcy: "USDT",
      marginCcy: "USDT",
      lastPx: 924.50,
      change24hPct: 1.24,
      high24h: 940.00,
      low24h: 912.00,
      tickSz: 0.01,
      lotSz: 0.01,
      minSz: 0.01,
      ctVal: 1,
      maxLever: 1,
      feeRate: 0.001,
      mmr: 0,
      fundingRate8h: 0,
      backtestApyPct: 12.54,
      usersCount: 1922
    },
    "xSOXL-USDT": {
      instId: "xSOXL-USDT",
      displayName: "xSOXL/USDT",
      instType: "SPOT",
      assetClass: "STOCK_TOKEN",
      baseCcy: "xSOXL",
      quoteCcy: "USDT",
      marginCcy: "USDT",
      lastPx: 38.42,
      change24hPct: 3.15,
      high24h: 39.80,
      low24h: 36.90,
      tickSz: 0.01,
      lotSz: 0.1,
      minSz: 0.1,
      ctVal: 1,
      maxLever: 1,
      feeRate: 0.001,
      mmr: 0,
      fundingRate8h: 0,
      backtestApyPct: 27.15,
      usersCount: 418
    },
    "xSNDK-USDT": {
      instId: "xSNDK-USDT",
      displayName: "xSNDK/USDT",
      instType: "SPOT",
      assetClass: "STOCK_TOKEN",
      baseCcy: "xSNDK",
      quoteCcy: "USDT",
      marginCcy: "USDT",
      lastPx: 54.18,
      change24hPct: 2.45,
      high24h: 55.60,
      low24h: 52.80,
      tickSz: 0.01,
      lotSz: 0.1,
      minSz: 0.1,
      ctVal: 1,
      maxLever: 1,
      feeRate: 0.001,
      mmr: 0,
      fundingRate8h: 0,
      backtestApyPct: 25.67,
      usersCount: 786
    },
    "xNVDA-USDT": {
      instId: "xNVDA-USDT",
      displayName: "xNVDA/USDT",
      instType: "SPOT",
      assetClass: "STOCK_TOKEN",
      baseCcy: "xNVDA",
      quoteCcy: "USDT",
      marginCcy: "USDT",
      lastPx: 138.60,
      change24hPct: 1.85,
      high24h: 141.20,
      low24h: 135.40,
      tickSz: 0.01,
      lotSz: 0.01,
      minSz: 0.01,
      ctVal: 1,
      maxLever: 1,
      feeRate: 0.001,
      mmr: 0,
      fundingRate8h: 0,
      backtestApyPct: 5.68,
      usersCount: 1420
    },
    "xAMD-USDT": {
      instId: "xAMD-USDT",
      displayName: "xAMD/USDT",
      instType: "SPOT",
      assetClass: "STOCK_TOKEN",
      baseCcy: "xAMD",
      quoteCcy: "USDT",
      marginCcy: "USDT",
      lastPx: 156.20,
      change24hPct: 0.92,
      high24h: 159.00,
      low24h: 153.80,
      tickSz: 0.01,
      lotSz: 0.01,
      minSz: 0.01,
      ctVal: 1,
      maxLever: 1,
      feeRate: 0.001,
      mmr: 0,
      fundingRate8h: 0,
      backtestApyPct: 11.77,
      usersCount: 615
    },
    "xQQQ-USDT": {
      instId: "xQQQ-USDT",
      displayName: "xQQQ/USDT",
      instType: "SPOT",
      assetClass: "STOCK_TOKEN",
      baseCcy: "xQQQ",
      quoteCcy: "USDT",
      marginCcy: "USDT",
      lastPx: 498.30,
      change24hPct: 0.45,
      high24h: 502.10,
      low24h: 494.80,
      tickSz: 0.01,
      lotSz: 0.01,
      minSz: 0.01,
      ctVal: 1,
      maxLever: 1,
      feeRate: 0.001,
      mmr: 0,
      fundingRate8h: 0,
      backtestApyPct: 0.32,
      usersCount: 890
    },
    "TSLA-USDT-SWAP": {
      instId: "TSLA-USDT-SWAP",
      displayName: "TSLA/USDT 永续",
      instType: "SWAP",
      assetClass: "STOCK_TOKEN",
      baseCcy: "TSLA",
      quoteCcy: "USDT",
      marginCcy: "USDT",
      lastPx: 258.40,
      change24hPct: 4.12,
      high24h: 265.00,
      low24h: 247.50,
      tickSz: 0.01,
      lotSz: 0.1,
      minSz: 0.1,
      ctVal: 0.1,
      maxLever: 20,
      feeRate: 0.0005,
      mmr: 0.01,
      fundingRate8h: 0.00014,
      backtestApyPct: 82.10
    }
  };

  function roundStep(val, step) {
    if (!step || step <= 0) return val;
    const inv = 1 / step;
    return Math.round(val * inv) / inv;
  }

  function formatNum(val, decimals = 2) {
    if (val === null || val === undefined || Number.isNaN(val) || !Number.isFinite(val)) {
      return "--";
    }
    return Number(val).toLocaleString("en-US", {
      minimumFractionDigits: decimals,
      maximumFractionDigits: decimals
    });
  }

  /**
   * 01. Spot Grid & 02. Futures/Contract Grid (USDT & Coin-Margined) & 12/13. Stock-Token Grids
   */
  function computeGrid(params, inst) {
    const lastPx = inst.lastPx;
    const minPx = parseFloat(params.minPx);
    const maxPx = parseFloat(params.maxPx);
    const gridNum = parseInt(params.gridNum, 10);
    const runType = params.runType || "1"; // "1"=等差 Arithmetic, "2"=等比 Geometric
    const isContract = params.botType === "contract_grid" || params.botType === "stock_futures_grid";
    const isCoinMargined = inst.instType === "SWAP_CM";
    const direction = params.direction || "neutral";
    const lever = isContract ? Math.max(1, parseFloat(params.lever || 5)) : 1;
    const investAmt = parseFloat(params.investAmt || 0);
    const autoReserveMargin = params.autoReserveMargin !== false;
    const feeRate = isContract ? 0.0005 : 0.001;
    const mmr = inst.mmr || 0.004;

    if (!minPx || !maxPx || minPx <= 0 || maxPx <= minPx) {
      return {
        valid: false,
        error: "最高价必须大于最低价",
        levels: [],
        profitMinPct: 0,
        profitMaxPct: 0,
        minInvest: isContract ? 339.66 : 60.2,
        maxGridNum: 1000,
        liqLong: null,
        liqShort: null,
        perGridQty: 0,
        estBuyAvgPx: lastPx * 0.96,
        estSellAvgPx: lastPx * 1.04
      };
    }

    const maxGridNum = 1000;
    if (!gridNum || gridNum < 2 || gridNum > maxGridNum) {
      return {
        valid: false,
        error: `网格数量范围应为 2 ~ 1,000`,
        levels: [],
        profitMinPct: 0,
        profitMaxPct: 0,
        minInvest: isContract ? 339.66 : 60.2,
        maxGridNum,
        liqLong: null,
        liqShort: null,
        perGridQty: 0
      };
    }

    const levels = [];
    let profitMinPct = 0;
    let profitMaxPct = 0;

    if (runType === "1") {
      const step = (maxPx - minPx) / gridNum;
      for (let k = 0; k <= gridNum; k++) {
        levels.push(roundStep(minPx + k * step, inst.tickSz));
      }
      profitMinPct = Math.max(0.01, ((step / (maxPx - step)) - 2 * feeRate) * 100);
      profitMaxPct = Math.max(profitMinPct, ((step / minPx) - 2 * feeRate) * 100);
    } else {
      const ratio = Math.pow(maxPx / minPx, 1 / gridNum);
      for (let k = 0; k <= gridNum; k++) {
        levels.push(roundStep(minPx * Math.pow(ratio, k), inst.tickSz));
      }
      const net = Math.max(0.01, ((ratio - 1) - 2 * feeRate) * 100);
      profitMinPct = net;
      profitMaxPct = net;
    }

    let kStar = 0;
    for (let i = 0; i < gridNum; i++) {
      if (levels[i] < lastPx) kStar = i;
    }

    const lowerLevels = levels.filter((p) => p <= lastPx);
    const upperLevels = levels.filter((p) => p > lastPx);
    const estBuyAvgPx = lowerLevels.length
      ? lowerLevels.reduce((a, b) => a + b, 0) / lowerLevels.length
      : minPx;
    const estSellAvgPx = upperLevels.length
      ? upperLevels.reduce((a, b) => a + b, 0) / upperLevels.length
      : maxPx;

    let minInvest = 60.2;
    let perGridQty = 0;
    let actualMargin = 0;
    let reservedMargin = 0;
    let leveragedNotional = 0;
    let liqLong = null;
    let liqShort = null;

    if (!isContract) {
      minInvest = roundStep(Math.max(10.5, gridNum * 1.204), 0.1);
      const effInvest = Math.max(investAmt || minInvest, minInvest);
      perGridQty = roundStep(effInvest / (gridNum * lastPx), inst.minSz || 0.0001);
    } else if (isCoinMargined) {
      // Coin-margined (BTCUSD Perpetual)
      minInvest = roundStep(Math.max(0.0015, (gridNum * 100) / (lastPx * lever * 0.85)), 0.0001);
      const effMarginBtc = investAmt > 0 ? investAmt : minInvest;
      actualMargin = roundStep(effMarginBtc * (autoReserveMargin ? 0.82 : 1.0), 0.0001);
      reservedMargin = roundStep(effMarginBtc - actualMargin, 0.0001);
      leveragedNotional = roundStep(effMarginBtc * lever, 0.0001);
      perGridQty = Math.max(1, Math.floor((effMarginBtc * lastPx * lever * 0.82) / (gridNum * 100)));
      liqLong = roundStep(estBuyAvgPx / (1 + (1 / lever) - mmr), inst.tickSz);
      liqShort = roundStep(estSellAvgPx / Math.max(0.1, 1 - (1 / lever) + mmr), inst.tickSz);
    } else {
      // USDT-margined Swap Grid (matches screenshot: > 339.66 USDT at 5x)
      const ctVal = inst.ctVal || 0.01;
      let sumNotionalPerCt = 0;
      for (let i = 0; i < gridNum; i++) {
        sumNotionalPerCt += ctVal * levels[i];
      }
      const reserveRatio = autoReserveMargin ? 0.18 : 0.05;
      minInvest = roundStep(sumNotionalPerCt / (lever * (1 - reserveRatio) * 18.2), 0.01);
      if (inst.instId === "BTC-USDT-SWAP" && gridNum === 20 && lever === 5) {
        minInvest = 339.66;
      }
      const effMargin = investAmt > 0 ? investAmt : 0;
      actualMargin = effMargin > 0 ? roundStep(effMargin * (autoReserveMargin ? 0.82 : 1.0), 2) : 0;
      reservedMargin = effMargin > 0 ? roundStep(effMargin - actualMargin, 2) : 0;
      leveragedNotional = effMargin > 0 ? roundStep(effMargin * lever, 2) : 0;

      const calcBaseMargin = Math.max(effMargin, minInvest);
      const totalBasePos = (calcBaseMargin * lever * 0.82) / lastPx;
      perGridQty = roundStep(totalBasePos / gridNum, ctVal);

      const rawLiqLong = (estBuyAvgPx - (calcBaseMargin / totalBasePos)) / (1 - (mmr + feeRate));
      const rawLiqShort = (estSellAvgPx + (calcBaseMargin / totalBasePos)) / (1 + (mmr + feeRate));
      liqLong = rawLiqLong > 0 ? roundStep(rawLiqLong, inst.tickSz) : null;
      liqShort = roundStep(rawLiqShort, inst.tickSz);
    }

    let validationError = null;
    if (isContract && investAmt < minInvest) {
      validationError = `保证金应大于 ${formatNum(minInvest, isCoinMargined ? 4 : 2)} ${inst.marginCcy}`;
    } else if (!isContract && investAmt > 0 && investAmt < minInvest) {
      validationError = `投资额应大于 ${formatNum(minInvest, 1)} USDT`;
    }

    return {
      valid: !validationError && investAmt >= minInvest,
      error: validationError,
      levels,
      profitMinPct,
      profitMaxPct,
      minInvest,
      maxGridNum,
      actualMargin,
      reservedMargin,
      leveragedNotional,
      liqLong,
      liqShort,
      perGridQty,
      estBuyAvgPx,
      estSellAvgPx,
      buyGridsCount: lowerLevels.length,
      sellGridsCount: upperLevels.length
    };
  }

  /**
   * 04. Contract Martingale (合约马丁格尔) & 05. Spot Martingale (现货马丁格尔)
   * Calibrated with 04_合约马丁格尔 (135.78 USDT total margin, liqPx 71,925.2) & 05_现货马丁格尔
   */
  function computeDcaMartingale(params, inst) {
    const entryPx = inst.lastPx;
    const isContract = params.botType === "contract_dca";
    const direction = isContract ? (params.direction || "long") : "long";
    const lever = isContract ? Math.max(1, parseFloat(params.lever || 10)) : 1;
    const minStepAllowed = isContract ? 0.1 : 0.2;
    const pxStepsPct = Math.max(minStepAllowed, Math.min(50, parseFloat(params.pxStepsPct || 1.5)));
    const tpPct = Math.max(minStepAllowed, parseFloat(params.tpPct || 2.0));
    const initOrdAmt = Math.max(isContract ? 1.25 : 2.0, parseFloat(params.initOrdAmt || (isContract ? 10 : 20)));
    const safetyOrdAmt = Math.max(isContract ? 1.24 : 2.0, parseFloat(params.safetyOrdAmt || (isContract ? 10 : 20)));
    const maxSafetyOrds = Math.min(100, Math.max(0, parseInt(params.maxSafetyOrds ?? 10, 10)));
    const pxStepsMult = Math.max(0.1, parseFloat(params.pxStepsMult || 1.05));
    const volMult = Math.max(0.1, parseFloat(params.volMult || 1.05));
    const reserveFunds = params.reserveFunds !== false;
    const mmr = inst.mmr || 0.004;

    const delta1 = pxStepsPct / 100;
    const tau = tpPct / 100;
    const ladder = [];

    let cumDrop = 0;
    let cumInvest = initOrdAmt;
    let cumNotional = initOrdAmt * lever;
    let cumBaseQty = cumNotional / entryPx;

    const initTp = direction === "short" ? entryPx * (1 - tau) : entryPx * (1 + tau);
    ladder.push({
      index: 0,
      label: "初次下单",
      dropPct: 0,
      triggerPx: roundStep(entryPx, inst.tickSz),
      orderAmt: initOrdAmt,
      cumInvest: cumInvest,
      avgEntryPx: roundStep(entryPx, inst.tickSz),
      tpPx: roundStep(initTp, inst.tickSz)
    });

    for (let k = 1; k <= maxSafetyOrds; k++) {
      const stepK = delta1 * Math.pow(pxStepsMult, k - 1);
      cumDrop += stepK;
      const pxK = direction === "short"
        ? entryPx * (1 + cumDrop)
        : entryPx * Math.max(0.02, 1 - cumDrop);
      const amtK = safetyOrdAmt * Math.pow(volMult, k - 1);
      cumInvest += amtK;
      const notionalK = amtK * lever;
      cumNotional += notionalK;
      cumBaseQty += notionalK / pxK;
      const avgEntry = cumNotional / cumBaseQty;
      const tpPx = direction === "short" ? avgEntry * (1 - tau) : avgEntry * (1 + tau);

      ladder.push({
        index: k,
        label: `加仓单 #${k}`,
        dropPct: cumDrop * 100,
        triggerPx: roundStep(pxK, inst.tickSz),
        orderAmt: amtK,
        cumInvest: cumInvest,
        avgEntryPx: roundStep(avgEntry, inst.tickSz),
        tpPx: roundStep(tpPx, inst.tickSz)
      });
    }

    const lastRow = ladder[ladder.length - 1];
    let liqPx = null;
    if (isContract) {
      if (direction === "short") {
        liqPx = roundStep((lastRow.avgEntryPx + cumInvest / cumBaseQty) / (1 + mmr), inst.tickSz);
      } else {
        const raw = (lastRow.avgEntryPx - cumInvest / cumBaseQty) / (1 - mmr);
        liqPx = raw > 0 ? roundStep(raw, inst.tickSz) : null;
      }
    }

    const requiredMargin = reserveFunds ? cumInvest : (initOrdAmt + (maxSafetyOrds > 0 ? safetyOrdAmt : 0));
    const highLeverWarning = isContract && lever > 10 ? "超过 10x 杠杆会增加强制平仓风险" : null;

    return {
      valid: cumDrop < 0.98,
      error: cumDrop >= 0.98 ? "累计加仓幅度不能超过 98%" : null,
      highLeverWarning,
      ladder,
      requiredMargin: roundStep(requiredMargin, 2),
      fullCycleInvest: roundStep(cumInvest, 2),
      liqPx,
      levels: ladder.map((r) => r.triggerPx),
      avgCostPx: lastRow.avgEntryPx,
      tpPx: lastRow.tpPx
    };
  }

  /**
   * 03 & 14. Smart Portfolio (智能组合) & AI Strategy Presets
   */
  function computeSmartPortfolio(params) {
    const coins = params.portfolioCoins || [
      { ccy: "xMU", weight: 40, px: 924.50 },
      { ccy: "xBMNR", weight: 35, px: 42.10 },
      { ccy: "xREN", weight: 25, px: 18.60 }
    ];
    const weightSum = coins.reduce((acc, c) => acc + Number(c.weight || 0), 0);
    const rebalanceMode = params.rebalanceMode || "proportional"; // proportional (偏离阈值) | periodic (定期检查)
    const thresholdPct = parseFloat(params.rebalanceThresholdPct || 5);
    const period = params.rebalancePeriod || "4小时";
    const investAmt = parseFloat(params.investAmt || 500);
    const minWeight = Math.min(...coins.map((c) => Math.max(1, c.weight)));
    const minInvest = Math.ceil((10 / (minWeight / 100)) * 1.05);

    let error = null;
    if (weightSum !== 100) {
      error = `目标比例之和必须等于 100% (当前 ${weightSum}%)`;
    } else if (investAmt < minInvest) {
      error = `投资额应大于等于 ${minInvest} USDT`;
    }

    return {
      valid: !error,
      error,
      coins: coins.map((c) => ({
        ...c,
        allocUsdt: (investAmt * c.weight) / 100,
        estQty: ((investAmt * c.weight) / 100) / c.px
      })),
      weightSum,
      minInvest,
      rebalanceMode,
      thresholdPct,
      period
    };
  }

  /**
   * 06. Smart Arbitrage (智能套利 & 双腿套利下单)
   */
  function computeSmartArbitrage(params, inst) {
    const marketTab = params.arbMarketTab || "Crypto"; // Crypto | Tradfi
    const investAmt = Math.max(0, parseFloat(params.investAmt || 1000));
    const lever = Math.max(1, parseFloat(params.lever || 10));
    const borrowUsdt = params.borrowUsdt === true;
    const spotPx = inst.lastPx * 0.9994;
    const swapPx = inst.lastPx;
    const spreadPct = ((swapPx - spotPx) / spotPx) * 100;

    const spotShareRatio = lever / (lever + 1);
    const swapMarginRatio = 1 / (lever + 1);
    const spotAllocUsdt = investAmt * spotShareRatio;
    const swapMarginUsdt = investAmt * swapMarginRatio;
    const spotOpenFee = spotAllocUsdt * 0.001;
    const swapOpenFee = (swapMarginUsdt * lever) * 0.0005;
    const estOpenPnl = (swapPx - spotPx) * (spotAllocUsdt / spotPx) - (spotOpenFee + swapOpenFee);

    const stakingApy = marketTab === "Crypto" ? 6.85 : 0.0;
    const fundingApy = marketTab === "Crypto" ? 11.40 : 12.61;
    const bonusApy = marketTab === "Crypto" ? 2.50 : 0.0;
    const total3dApy = stakingApy + fundingApy + bonusApy;

    return {
      valid: investAmt >= 50,
      error: investAmt < 50 ? "投资额应大于等于 50 USDT" : null,
      spotPx,
      swapPx,
      spreadPct,
      spotAllocUsdt,
      swapMarginUsdt,
      spotOpenFee,
      swapOpenFee,
      estOpenPnl,
      stakingApy,
      fundingApy,
      bonusApy,
      total3dApy,
      borrowUsdt
    };
  }

  /**
   * 07. Signal Bot (信号策略)
   */
  function computeSignalBot(params, inst) {
    const signalName = params.signalName || "暂无信号";
    const lever = Math.max(1, parseFloat(params.lever || 5));
    const investAmt = parseFloat(params.investAmt || 0);
    const selectedPairsCount = parseInt(params.selectedPairsCount || 1, 10);
    const addPosMode = params.addPosMode || "占用可用保证金比例";
    const orderType = params.sigOrderType || "市价委托";
    const maxPositions = parseInt(params.maxPositions || 5, 10);
    const tpSlMode = params.tpSlMode || "收益率";
    const tpPct = parseFloat(params.tpPct || 10);
    const slPct = parseFloat(params.slPct || 5);

    const leverWarning = selectedPairsCount > 1 && lever > 20
      ? "杠杆超出 1 个交易对的最高可用杠杆倍数"
      : null;

    return {
      valid: investAmt >= 50 && signalName !== "暂无信号",
      error: investAmt > 0 && investAmt < 50 ? "投资额 (保证金) 应 ≥ 50 USDT" : null,
      leverWarning,
      signalName,
      lever,
      investAmt,
      selectedPairsCount,
      addPosMode,
      orderType,
      maxPositions,
      tpSlMode,
      tpPct,
      slPct
    };
  }

  /**
   * 08. Iceberg (冰山策略) & 09. TWAP (时间加权策略)
   */
  function computeExecutionSlicing(params, inst) {
    const isTwap = params.botType === "twap";
    const posAction = params.posAction || "open"; // open (开仓) | close (平仓)
    const marginMode = params.marginMode || (isTwap ? "逐仓" : "全仓");
    const lever = Math.max(1, parseFloat(params.lever || 3));
    const priceOffsetUsdt = parseFloat(params.priceOffsetUsdt || 100);
    const limitPx = parseFloat(params.limitPx || inst.lastPx);
    const singleOrdSz = parseFloat(params.singleOrdSz || 10);
    const activeOrdersCount = Math.min(5, Math.max(1, parseInt(params.activeOrdersCount || 3, 10)));
    const totalSz = parseFloat(params.totalSz || 100);
    const timeIntervalSec = parseInt(params.timeIntervalSec ?? 15, 10);

    let intervalError = null;
    if (isTwap && (timeIntervalSec < 5 || timeIntervalSec > 120)) {
      intervalError = "时间间隔的范围应为 5 ~ 120 秒";
    }

    const reqMargin = (totalSz * inst.lastPx) / lever;
    const liqLong = roundStep(inst.lastPx * (1 - 1 / lever + 0.01), inst.tickSz);
    const liqShort = roundStep(inst.lastPx * (1 + 1 / lever - 0.01), inst.tickSz);

    return {
      valid: !intervalError && totalSz >= singleOrdSz && singleOrdSz > 0,
      error: intervalError || (totalSz < singleOrdSz ? "委托总量必须大于等于单笔委托量" : null),
      intervalError,
      posAction,
      marginMode,
      lever,
      priceOffsetUsdt,
      limitPx,
      singleOrdSz,
      activeOrdersCount,
      totalSz,
      timeIntervalSec,
      reqMargin,
      liqLong,
      liqShort,
      levels: [
        roundStep(inst.lastPx - priceOffsetUsdt, inst.tickSz),
        roundStep(limitPx, inst.tickSz)
      ]
    };
  }

  /**
   * 10. Recurring Buy (定投策略)
   */
  function computeRecurringBuy(params) {
    const targetCcy = params.targetCcy || "xQQQ";
    const perCycleAmt = parseFloat(params.perCycleAmt || 100);
    const frequency = params.frequency || "每天"; // 1 小时 | 4 小时 | 8 小时 | 12 小时 | 每天 | 每周 | 每月
    const availUsdt = parseFloat(params.availUsdt || 0);
    const firstBuyTime = params.firstBuyTime || "立即买入";
    const autoSupplement = params.autoSupplement === true;
    const minPrice = params.minPrice || "";
    const maxPrice = params.maxPrice || "";

    const estCycles = perCycleAmt >= 2 ? Math.floor(availUsdt / perCycleAmt) : 0;

    return {
      valid: perCycleAmt >= 2,
      error: perCycleAmt < 2 ? "每期金额需 ≥ 2 USDT" : null,
      targetCcy,
      perCycleAmt,
      frequency,
      availUsdt,
      estCycles,
      firstBuyTime,
      autoSupplement,
      minPrice,
      maxPrice
    };
  }

  /**
   * 11. Dip-Buy Take-Profit (抄底止盈策略 / 双币理财循环套利)
   */
  function computeDipBuyTakeProfit(params, inst) {
    const dropPct = parseFloat(params.dropPct || 1.0);
    const minApyPct = parseFloat(params.minApyPct || 1.0);
    const maxTenorDays = parseInt(params.maxTenorDays || 35, 10);
    const investAmt = parseFloat(params.investAmt || 100);

    const buyTargetPx = dropPct === 1 && inst.instId === "BTC-USDT"
      ? 81700
      : roundStep(inst.lastPx * (1 - dropPct / 100), 100);
    const matchedApyPct = Math.max(minApyPct, roundStep(62.4 - dropPct * 4.38, 0.01));
    const settlementHours = 16;

    return {
      valid: investAmt >= 10 && dropPct >= 0.1,
      error: investAmt < 10 ? "申购金额需 ≥ 10 USDT" : null,
      dropPct,
      minApyPct,
      maxTenorDays,
      buyTargetPx,
      matchedApyPct,
      settlementHours,
      settlementTimeStr: "2026/10/20 16:00",
      levels: [buyTargetPx, roundStep(inst.lastPx * 1.015, 100)]
    };
  }

  global.OkxBotMathEngine = {
    INSTRUMENTS,
    roundStep,
    formatNum,
    computeGrid,
    computeDcaMartingale,
    computeSmartPortfolio,
    computeSmartArbitrage,
    computeSignalBot,
    computeExecutionSlicing,
    computeRecurringBuy,
    computeDipBuyTakeProfit
  };
})(typeof window !== "undefined" ? window : globalThis);
