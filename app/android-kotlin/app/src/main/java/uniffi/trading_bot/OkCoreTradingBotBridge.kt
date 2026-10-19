package uniffi.trading_bot

import com.okinc.tradingbot.domain.math.StrategyBotMathEngine
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow

/**
 * Reconstructed Kotlin UniFFI / JNA Bridge for `libOKCore.so` (`trading_bot` crate).
 * Mirrors the exact Interactor & Record contracts extracted from
 * `reconstructed/rust_okcore_idl/trading_bot_full_schema.rs`.
 */

enum class ActionResultState {
    IDLE, LOADING, SUCCESS, ERROR
}

enum class AiTier {
    GRID_SHORT,
    GRID_MEDIUM,
    GRID_LONG,
    FLYWHEEL_AGGRESSIVE,
    FLYWHEEL_BALANCED
}

enum class AiDcdInvestMethod {
    TARGET_PRICE,
    PRICE_DROP
}

enum class SmartArbStopType(val wireCode: String) {
    STOP_AND_SELL("1"),
    STOP_NO_SELL("2"),
    STOP_KEEP_STAKE("6")
}

sealed class BotDisplayText {
    data class Plain(val text: String) : BotDisplayText()
    data object Placeholder : BotDisplayText()
    data class Localized(val cdKey: String, val args: List<String> = emptyList()) : BotDisplayText()

    fun render(): String = when (this) {
        is Plain -> text
        is Placeholder -> "--"
        is Localized -> cdKey
    }
}

data class StrategyRecommendation(
    val leverage: String,
    val direction: String,
    val maxDrawdown: String,
    val pnlRatio: String,
    val followerCount: String,
    val username: String,
    val stgyName: String,
    val stgyParamJson: String
)

data class ContractGridTrailingConfigInput(
    val enabled: Boolean = false,
    val trailingUpCapPx: String = "",
    val trailingDownFloorPx: String = ""
)

data class ContractGridPlaceOrderState(
    val instId: String = "BTC-USDT-SWAP",
    val instType: String = "SWAP",
    val direction: StrategyBotMathEngine.BotDirection = StrategyBotMathEngine.BotDirection.LONG,
    val runType: StrategyBotMathEngine.GridRunType = StrategyBotMathEngine.GridRunType.ARITHMETIC,
    val minPx: String = "86000",
    val maxPx: String = "104000",
    val gridNum: String = "40",
    val lever: String = "10",
    val investMargin: String = "1000",
    val extraMargin: String = "0",
    val tpTriggerPx: String = "",
    val slTriggerPx: String = "",
    val closePosOnStop: Boolean = true,
    val trailingConfig: ContractGridTrailingConfigInput = ContractGridTrailingConfigInput(),
    val perGridProfitDisplay: BotDisplayText = BotDisplayText.Placeholder,
    val minInvestDisplay: BotDisplayText = BotDisplayText.Placeholder,
    val liquidatePriceLong: BotDisplayText = BotDisplayText.Placeholder,
    val liquidatePriceShort: BotDisplayText = BotDisplayText.Placeholder,
    val submitState: ActionResultState = ActionResultState.IDLE,
    val submitAttempt: UInt = 0u,
    val validationError: String? = null
)

/**
 * Cycle-free reactive Interactor matching `ContractGridPlaceOrderInteractor` in `libOKCore.so`.
 */
class ContractGridPlaceOrderInteractor(
    initialInstId: String = "BTC-USDT-SWAP",
    private var lastPrice: Double = 94850.0
) : AutoCloseable {

    private val _state = MutableStateFlow(ContractGridPlaceOrderState(instId = initialInstId))
    val stateStream: StateFlow<ContractGridPlaceOrderState> = _state.asStateFlow()

    init {
        recompute()
    }

    fun updateParameters(
        direction: StrategyBotMathEngine.BotDirection = _state.value.direction,
        runType: StrategyBotMathEngine.GridRunType = _state.value.runType,
        minPx: String = _state.value.minPx,
        maxPx: String = _state.value.maxPx,
        gridNum: String = _state.value.gridNum,
        lever: String = _state.value.lever,
        investMargin: String = _state.value.investMargin,
        extraMargin: String = _state.value.extraMargin,
        tpTriggerPx: String = _state.value.tpTriggerPx,
        slTriggerPx: String = _state.value.slTriggerPx
    ) {
        _state.value = _state.value.copy(
            direction = direction,
            runType = runType,
            minPx = minPx,
            maxPx = maxPx,
            gridNum = gridNum,
            lever = lever,
            investMargin = investMargin,
            extraMargin = extraMargin,
            tpTriggerPx = tpTriggerPx,
            slTriggerPx = slTriggerPx
        )
        recompute()
    }

    private fun recompute() {
        val s = _state.value
        val calc = StrategyBotMathEngine.computeGridBot(
            lastPrice = lastPrice,
            minPx = s.minPx.toDoubleOrNull() ?: 0.0,
            maxPx = s.maxPx.toDoubleOrNull() ?: 0.0,
            gridNum = s.gridNum.toIntOrNull() ?: 0,
            runType = s.runType,
            isContract = s.instType != "SPOT",
            direction = s.direction,
            leverage = s.lever.toDoubleOrNull() ?: 1.0,
            investAmountUsdt = s.investMargin.toDoubleOrNull() ?: 0.0,
            extraMarginUsdt = s.extraMargin.toDoubleOrNull() ?: 0.0
        )
        val profitStr = if (calc.validationError == null) {
            if (s.runType == StrategyBotMathEngine.GridRunType.GEOMETRIC) {
                String.format("%.2f%%", calc.perGridProfitMinPct)
            } else {
                String.format("%.2f%% ~ %.2f%%", calc.perGridProfitMinPct, calc.perGridProfitMaxPct)
            }
        } else "--"

        _state.value = s.copy(
            perGridProfitDisplay = if (profitStr == "--") BotDisplayText.Placeholder else BotDisplayText.Plain(profitStr),
            minInvestDisplay = if (calc.minInvestmentUsdt > 0) BotDisplayText.Plain(String.format("%.2f USDT", calc.minInvestmentUsdt)) else BotDisplayText.Placeholder,
            liquidatePriceLong = calc.estimatedLiqPriceLong?.let { BotDisplayText.Plain(String.format("%.1f USDT", it)) } ?: BotDisplayText.Placeholder,
            liquidatePriceShort = calc.estimatedLiqPriceShort?.let { BotDisplayText.Plain(String.format("%.1f USDT", it)) } ?: BotDisplayText.Placeholder,
            validationError = calc.validationError
        )
    }

    fun submitOrder(): Boolean {
        val s = _state.value
        val nextAttempt = s.submitAttempt + 1u
        if (s.validationError != null) {
            _state.value = s.copy(submitState = ActionResultState.ERROR, submitAttempt = nextAttempt)
            return false
        }
        _state.value = s.copy(submitState = ActionResultState.SUCCESS, submitAttempt = nextAttempt)
        return true
    }

    fun release() {
        // Breaks foreign callback observer handleMap cycles before destroy()
    }

    override fun close() {
        release()
    }
}

/**
 * 01 & 12. Spot Grid & Stock-Token Spot Grid Interactor (`SpotGridPlaceOrderInteractor`)
 */
class SpotGridPlaceOrderInteractor(
    val instId: String = "BTC-USDT",
    val isStockToken: Boolean = instId.startsWith("x"),
    var lastPrice: Double = 82226.6
) : AutoCloseable {
    fun evaluate(minPx: Double, maxPx: Double, gridNum: Int, runType: StrategyBotMathEngine.GridRunType, investUsdt: Double) =
        StrategyBotMathEngine.computeGridBot(
            lastPrice = lastPrice,
            minPx = minPx,
            maxPx = maxPx,
            gridNum = gridNum,
            runType = runType,
            isContract = false,
            investAmountUsdt = investUsdt
        )
    override fun close() {}
}

/**
 * 03 & 14. Smart Portfolio & AI Strategy Rust Bridge (`SmartPortfolioRustRequestBridge`)
 */
class SmartPortfolioRustRequestBridge : AutoCloseable {
    fun evaluatePortfolio(coins: List<StrategyBotMathEngine.PortfolioCoinWeight>, investUsdt: Double) =
        StrategyBotMathEngine.computeSmartPortfolio(coins, investUsdt)
    override fun close() {}
}

/**
 * 04 & 05. Futures & Spot Martingale / DCA Interactor (`ContractDcaPlaceOrderInteractor` / `SpotDcaPlaceOrderInteractor`)
 */
class MartingaleDcaPlaceOrderInteractor(
    val instId: String = "BTC-USDT-SWAP",
    val isContract: Boolean = true,
    var entryPrice: Double = 82145.8
) : AutoCloseable {
    fun evaluateLadder(
        direction: StrategyBotMathEngine.BotDirection,
        leverage: Double,
        pxStepsPct: Double,
        tpPct: Double,
        initOrdAmt: Double,
        safetyOrdAmt: Double,
        maxSafetyOrds: Int,
        pxStepsMult: Double = 1.05,
        volMult: Double = 1.05,
        reserveFunds: Boolean = true
    ) = StrategyBotMathEngine.computeDcaMartingaleBot(
        entryPrice = entryPrice,
        direction = direction,
        isContract = isContract,
        leverage = leverage,
        pxStepsPct = pxStepsPct,
        tpPct = tpPct,
        initOrdAmtUsdt = initOrdAmt,
        safetyOrdAmtUsdt = safetyOrdAmt,
        maxSafetyOrds = maxSafetyOrds,
        pxStepsMult = pxStepsMult,
        volMult = volMult,
        reserveFunds = reserveFunds
    )
    override fun close() {}
}

/**
 * 06. Smart Arbitrage & Dual-Leg Arbitrage Interactor (`SmartArbitragePlaceOrderInteractor`)
 */
class SmartArbitragePlaceOrderInteractor(
    val instId: String = "BTC-USDT-SWAP",
    var spotPrice: Double = 82100.0,
    var swapPrice: Double = 82145.8
) : AutoCloseable {
    fun evaluate(investUsdt: Double, leverage: Double = 10.0, stakingApyPct: Double = 6.85) =
        StrategyBotMathEngine.computeSmartArbitrage(
            totalInvestUsdt = investUsdt,
            spotPrice = spotPrice,
            swapPrice = swapPrice,
            swapLeverage = leverage,
            stakingApyPct = stakingApyPct
        )
    override fun close() {}
}

/**
 * 07. Signal Bot TradingView Webhook Interactor (`SignalBotCopyCreateInteractor`)
 */
class SignalBotCopyCreateInteractor : AutoCloseable {
    fun evaluate(signalName: String, pairsCount: Int, leverage: Double, investUsdt: Double) =
        StrategyBotMathEngine.computeSignalBot(signalName, pairsCount, leverage, investUsdt)
    override fun close() {}
}

/**
 * 08 & 09. Smart Iceberg & TWAP Execution Slicing Interactor (`SmartIcebergDetailComposition` / `TwapRustOrderConfirmFragment`)
 */
class ExecutionSlicingOrderInteractor(val isTwap: Boolean) : AutoCloseable {
    fun evaluate(
        lastPrice: Double,
        leverage: Double,
        singleOrdSz: Double,
        totalOrdSz: Double,
        activeOrdersCount: Int = 3,
        timeIntervalSec: Int = 15
    ) = StrategyBotMathEngine.computeExecutionSlicing(
        isTwap = isTwap,
        lastPrice = lastPrice,
        leverage = leverage,
        singleOrderSz = singleOrdSz,
        totalOrderSz = totalOrdSz,
        activeOrdersCount = activeOrdersCount,
        timeIntervalSec = timeIntervalSec
    )
    override fun close() {}
}

/**
 * 10 & 11. Recurring Buy (`RecurringBuyAiOrderService`) & Dip-Buy Take-Profit (`DcdBotInteractor`)
 */
class RecurringAndDcdInteractor : AutoCloseable {
    fun evaluateRecurring(perCycleUsdt: Double, availUsdt: Double) =
        StrategyBotMathEngine.computeRecurringBuy(perCycleUsdt, availUsdt)

    fun evaluateDipBuyDcd(lastPrice: Double, dropPct: Double, minApyPct: Double, investUsdt: Double) =
        StrategyBotMathEngine.computeDipBuyTakeProfit(lastPrice, dropPct, minApyPct, investUsdt)

    override fun close() {}
}

