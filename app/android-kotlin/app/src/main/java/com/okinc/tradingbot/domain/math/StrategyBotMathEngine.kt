package com.okinc.tradingbot.domain.math

import kotlin.math.abs
import kotlin.math.ceil
import kotlin.math.floor
import kotlin.math.max
import kotlin.math.min
import kotlin.math.pow

/**
 * Reconstructed OKX v6.194.0 Strategy Bot Mathematical, Validation & Risk-Control Engine.
 * Mirrors the exact algorithms inside `libOKCore.so` (`components/biz-service/trading-bot/src/`)
 * and `com.okinc.tradingbot.*` (`OKT*DCAMinInvestMath`, `ContractGridLiquidatePrice`, etc.).
 */
object StrategyBotMathEngine {

    enum class GridRunType(val wireCode: String) {
        ARITHMETIC("1"),
        GEOMETRIC("2")
    }

    enum class BotDirection(val wireCode: String) {
        LONG("long"),
        SHORT("short"),
        NEUTRAL("neutral")
    }

    data class GridCalculationResult(
        val priceLevels: List<Double>,
        val perGridProfitMinPct: Double,
        val perGridProfitMaxPct: Double,
        val minInvestmentUsdt: Double,
        val maxAllowedGrids: Int,
        val estimatedLiqPriceLong: Double?,
        val estimatedLiqPriceShort: Double?,
        val perGridBaseQty: Double,
        val validationError: String? = null
    )

    data class DcaSafetyOrderRow(
        val orderIndex: Int,          // 0 = Initial Order, 1..M = Safety Orders
        val triggerDropPct: Double,   // Cumulative price deviation % from P_0
        val triggerPrice: Double,     // Execution trigger price P_k
        val orderAmountUsdt: Double,  // Margin / Quote amount A_k
        val cumulativeInvestUsdt: Double,
        val averageEntryPrice: Double,// Volume-weighted average holding price \bar{P}_k
        val cycleTakeProfitPrice: Double,
        val requiredBouncePct: Double
    )

    data class DcaCalculationResult(
        val ladder: List<DcaSafetyOrderRow>,
        val minInvestmentUsdt: Double,
        val maxCumulativeDropPct: Double,
        val estimatedLiqPrice: Double?,
        val riskWarning: String? = null,
        val validationError: String? = null
    )

    data class SmartArbCalculationResult(
        val spotLegNotionalUsdt: Double,
        val swapMarginUsdt: Double,
        val hedgedBaseQty: Double,
        val annualizedFundingApyPct: Double,
        val annualizedStakingApyPct: Double,
        val combinedApyPct: Double,
        val currentSpreadPct: Double,
        val swapLiquidationPrice: Double
    )

    private fun roundToStep(value: Double, step: Double): Double {
        if (step <= 0.0) return value
        return floor(value / step + 1e-9) * step
    }

    /**
     * 1-3. Spot Grid & Futures/Contract Grid Calculation Engine
     */
    fun computeGridBot(
        lastPrice: Double,
        minPx: Double,
        maxPx: Double,
        gridNum: Int,
        runType: GridRunType,
        isContract: Boolean,
        direction: BotDirection = BotDirection.LONG,
        leverage: Double = 1.0,
        investAmountUsdt: Double = 0.0,
        extraMarginUsdt: Double = 0.0,
        tickSz: Double = 0.1,
        minSz: Double = 0.0001,
        ctVal: Double = 0.01,
        feeRate: Double = if (isContract) 0.0005 else 0.001,
        mmr: Double = 0.004
    ): GridCalculationResult {
        if (minPx <= 0.0 || maxPx <= minPx) {
            return GridCalculationResult(
                emptyList(), 0.0, 0.0, 0.0, 2, null, null, 0.0,
                "Upper price must be greater than lower price"
            )
        }
        val minStep = max(tickSz, lastPrice * (2.0 * feeRate + 0.0005))
        val maxAllowedGrids = min(300, max(2, floor((maxPx - minPx) / minStep).toInt()))
        if (gridNum < 2 || gridNum > maxAllowedGrids) {
            return GridCalculationResult(
                emptyList(), 0.0, 0.0, 0.0, maxAllowedGrids, null, null, 0.0,
                "Grid count must be between 2 and $maxAllowedGrids"
            )
        }

        val levels = ArrayList<Double>(gridNum + 1)
        val profitMinPct: Double
        val profitMaxPct: Double

        if (runType == GridRunType.ARITHMETIC) {
            val step = (maxPx - minPx) / gridNum.toDouble()
            for (k in 0..gridNum) {
                levels.add(roundToStep(minPx + k * step, tickSz))
            }
            profitMinPct = ((step / (maxPx - step)) - 2.0 * feeRate) * 100.0
            profitMaxPct = ((step / minPx) - 2.0 * feeRate) * 100.0
        } else {
            val ratio = (maxPx / minPx).pow(1.0 / gridNum.toDouble())
            for (k in 0..gridNum) {
                levels.add(roundToStep(minPx * ratio.pow(k.toDouble()), tickSz))
            }
            val net = ((ratio - 1.0) - 2.0 * feeRate) * 100.0
            profitMinPct = net
            profitMaxPct = net
        }

        // Minimum Investment & Liquidation Price
        val kStar = levels.indexOfLast { it < lastPrice }.coerceIn(0, gridNum - 1)
        val minInvestUsdt: Double
        var liqLong: Double? = null
        var liqShort: Double? = null
        val perGridQty: Double

        if (!isContract) {
            // Spot Grid Min Investment
            val buySideQuote = (0..kStar).sumOf { i -> minSz * levels[i] * (1.0 + feeRate) }
            val sellGridsCount = max(0, gridNum - 1 - kStar)
            val sellSideQuote = sellGridsCount * minSz * lastPrice * 1.05
            minInvestUsdt = (buySideQuote + sellSideQuote) * 1.02
            val effectiveInvest = max(investAmountUsdt, minInvestUsdt)
            perGridQty = roundToStep(effectiveInvest / (gridNum * lastPrice), minSz)
        } else {
            // Futures / Contract Grid Min Margin & Liquidation Price
            val sumNotionalPerCt = (0 until gridNum).sumOf { i -> ctVal * levels[i] }
            val reserveFactor = 0.08
            minInvestUsdt = (sumNotionalPerCt * max(1.0, minSz)) / (leverage * (1.0 - reserveFactor))
            val effectiveMargin = max(investAmountUsdt, minInvestUsdt)
            val ctPerGrid = max(1.0, floor((effectiveMargin * leverage * (1.0 - reserveFactor)) / sumNotionalPerCt))
            perGridQty = ctPerGrid * ctVal
            val totalBasePos = gridNum * perGridQty
            val totalMargin = effectiveMargin + extraMarginUsdt

            val sellGridsAbove = max(0, gridNum - 1 - kStar)
            val avgLongEntry = (sellGridsAbove * lastPrice + (0..kStar).sumOf { levels[it] }) / gridNum.toDouble()
            val buyGridsBelow = kStar + 1
            val avgShortEntry = (buyGridsBelow * lastPrice + ((kStar + 1)..gridNum).sumOf { levels[it] }) / gridNum.toDouble()

            val rawLiqLong = (avgLongEntry - (totalMargin / totalBasePos)) / (1.0 - (mmr + feeRate))
            val rawLiqShort = (avgShortEntry + (totalMargin / totalBasePos)) / (1.0 + (mmr + feeRate))

            when (direction) {
                BotDirection.LONG -> liqLong = if (rawLiqLong > 0.0) rawLiqLong else null
                BotDirection.SHORT -> liqShort = rawLiqShort
                BotDirection.NEUTRAL -> {
                    liqLong = if (rawLiqLong > 0.0) rawLiqLong else null
                    liqShort = rawLiqShort
                }
            }
        }

        return GridCalculationResult(
            priceLevels = levels,
            perGridProfitMinPct = profitMinPct,
            perGridProfitMaxPct = profitMaxPct,
            minInvestmentUsdt = minInvestUsdt,
            maxAllowedGrids = maxAllowedGrids,
            estimatedLiqPriceLong = liqLong,
            estimatedLiqPriceShort = liqShort,
            perGridBaseQty = perGridQty
        )
    }

    /**
     * 4-5. Spot DCA & Contract DCA (Martingale) Calculation Engine (`OKT*DCAMinInvestMath`)
     */
    fun computeDcaMartingaleBot(
        entryPrice: Double,
        direction: BotDirection,
        isContract: Boolean,
        leverage: Double,
        pxStepsPct: Double,        // e.g. 1.5 (%)
        tpPct: Double,             // e.g. 2.0 (%)
        initOrdAmtUsdt: Double,    // A_0
        safetyOrdAmtUsdt: Double,  // A_1
        maxSafetyOrds: Int,        // M
        pxStepsMult: Double,       // m_p
        volMult: Double,           // m_v
        reserveFunds: Boolean = true,
        extraMarginUsdt: Double = 0.0,
        mmr: Double = 0.004
    ): DcaCalculationResult {
        val m = maxSafetyOrds.coerceIn(1, 100)
        val delta1 = pxStepsPct / 100.0
        val tau = tpPct / 100.0
        val ladder = ArrayList<DcaSafetyOrderRow>(m + 1)

        var cumDrop = 0.0
        var cumInvest = initOrdAmtUsdt
        var cumNotional = initOrdAmtUsdt * leverage
        var cumBaseQty = cumNotional / entryPrice

        val initTp = if (direction == BotDirection.SHORT) entryPrice * (1.0 - tau) else entryPrice * (1.0 + tau)
        ladder.add(
            DcaSafetyOrderRow(
                orderIndex = 0,
                triggerDropPct = 0.0,
                triggerPrice = entryPrice,
                orderAmountUsdt = initOrdAmtUsdt,
                cumulativeInvestUsdt = cumInvest,
                averageEntryPrice = entryPrice,
                cycleTakeProfitPrice = initTp,
                requiredBouncePct = tpPct
            )
        )

        for (k in 1..m) {
            val stepK = delta1 * pxStepsMult.pow((k - 1).toDouble())
            cumDrop += stepK
            val pxK = if (direction == BotDirection.SHORT) {
                entryPrice * (1.0 + cumDrop)
            } else {
                entryPrice * max(0.0001, 1.0 - cumDrop)
            }
            val amtK = safetyOrdAmtUsdt * volMult.pow((k - 1).toDouble())
            cumInvest += amtK
            val notionalK = amtK * leverage
            cumNotional += notionalK
            cumBaseQty += notionalK / pxK
            val avgEntry = cumNotional / cumBaseQty
            val tpPrice = if (direction == BotDirection.SHORT) {
                avgEntry * (1.0 - tau)
            } else {
                avgEntry * (1.0 + tau)
            }
            val bouncePct = abs(tpPrice - pxK) / pxK * 100.0

            ladder.add(
                DcaSafetyOrderRow(
                    orderIndex = k,
                    triggerDropPct = cumDrop * 100.0,
                    triggerPrice = pxK,
                    orderAmountUsdt = amtK,
                    cumulativeInvestUsdt = cumInvest,
                    averageEntryPrice = avgEntry,
                    cycleTakeProfitPrice = tpPrice,
                    requiredBouncePct = bouncePct
                )
            )
        }

        val valError = if (direction != BotDirection.SHORT && cumDrop >= 0.99) {
            "Cumulative safety order price drop (${String.format("%.1f%%", cumDrop * 100)}) cannot exceed 99%"
        } else null

        val minInvest = if (reserveFunds) cumInvest else (initOrdAmtUsdt + safetyOrdAmtUsdt)
        var liqPx: Double? = null
        var warning: String? = null

        if (isContract) {
            val lastRow = ladder.last()
            val totalMargin = cumInvest + extraMarginUsdt
            if (direction == BotDirection.SHORT) {
                liqPx = (lastRow.averageEntryPrice + (totalMargin / cumBaseQty)) / (1.0 + mmr)
                if (liqPx <= lastRow.triggerPrice) {
                    warning = "Warning: Short liquidation price is below Safety Order #$m trigger price"
                }
            } else {
                val raw = (lastRow.averageEntryPrice - (totalMargin / cumBaseQty)) / (1.0 - mmr)
                liqPx = if (raw > 0.0) raw else null
                if (liqPx != null && liqPx >= lastRow.triggerPrice) {
                    warning = "Warning: Long liquidation price is above Safety Order #$m trigger price"
                }
            }
        }

        return DcaCalculationResult(
            ladder = ladder,
            minInvestmentUsdt = minInvest,
            maxCumulativeDropPct = cumDrop * 100.0,
            estimatedLiqPrice = liqPx,
            riskWarning = warning,
            validationError = valError
        )
    }

    /**
     * 6. Smart Arbitrage Delta-Neutral Hedging Engine
     */
    fun computeSmartArbitrage(
        totalInvestUsdt: Double,
        spotPrice: Double,
        swapPrice: Double,
        swapLeverage: Double = 1.0,
        avg8hFundingRate: Double = 0.00018, // 0.018% per 8h
        stakingApyPct: Double = 3.8,
        autoStaking: Boolean = true
    ): SmartArbCalculationResult {
        val feeBuffer = 0.0015
        val usable = totalInvestUsdt * (1.0 - feeBuffer)
        val unitCost = spotPrice + (swapPrice / swapLeverage)
        val baseQty = if (unitCost > 0) usable / unitCost else 0.0
        val spotNotional = baseQty * spotPrice
        val swapMargin = (baseQty * swapPrice) / swapLeverage
        val swapNotional = baseQty * swapPrice

        val fundingApy = if (totalInvestUsdt > 0) {
            (avg8hFundingRate * 3.0 * 365.0 * swapNotional / totalInvestUsdt) * 100.0
        } else 0.0
        val stakeApy = if (autoStaking && totalInvestUsdt > 0) {
            stakingApyPct * (spotNotional / totalInvestUsdt)
        } else 0.0

        val spreadPct = ((swapPrice - spotPrice) / spotPrice) * 100.0
        val swapLiqPx = swapPrice * (1.0 + (1.0 / swapLeverage) - 0.005)

        return SmartArbCalculationResult(
            spotLegNotionalUsdt = spotNotional,
            swapMarginUsdt = swapMargin,
            hedgedBaseQty = baseQty,
            annualizedFundingApyPct = fundingApy,
            annualizedStakingApyPct = stakeApy,
            combinedApyPct = fundingApy + stakeApy,
            currentSpreadPct = spreadPct,
            swapLiquidationPrice = swapLiqPx
        )
    }
}
