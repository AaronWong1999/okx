package com.okinc.tradingbot.ui.screens

import androidx.compose.foundation.Canvas
import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.LazyRow
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.PathEffect
import androidx.compose.ui.text.font.FontFamily
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.okinc.tradingbot.domain.math.StrategyBotMathEngine
import uniffi.trading_bot.ContractGridPlaceOrderInteractor

/**
 * Reconstructed OKX Android v6.194.0 Strategy Bot Workstation & Interactive Bot K-Line Overlay
 * in Jetpack Compose (`com.okinc.tradingbot.*` + `com.okinc.kline.features.botkline.*`).
 */
object OkxColors {
    val BgPrimary = Color(0xFF0B0E11)
    val BgElevated = Color(0xFF161A1E)
    val BgInput = Color(0xFF1F2229)
    val TextPrimary = Color(0xFFF0F2F5)
    val TextSecondary = Color(0xFF8E929B)
    val UpGreen = Color(0xFF00B865)
    val DownRed = Color(0xFFF5455C)
    val AccentLime = Color(0xFFBCFF2F)
    val NeutralBlue = Color(0xFF3B82F6)
    val WarningAmber = Color(0xFFFFB800)
    val BorderSubtle = Color(0xFF22252C)
}

enum class OkxBotType(val id: String, val zhTitle: String, val enTitle: String, val category: String) {
    SPOT_GRID("grid", "现货网格", "Spot Grid", "网格"),
    FUTURES_GRID("contract_grid", "合约网格", "Futures Grid", "网格"),
    INFINITY_GRID("moon_grid", "天地网格", "Infinity Grid", "网格"),
    SPOT_DCA("spot_dca", "现货马丁格尔", "Spot DCA", "马丁/定投"),
    FUTURES_DCA("contract_dca", "合约马丁格尔", "Futures DCA", "马丁/定投"),
    SMART_ARB("smart_arb", "屯币套利", "Smart Arbitrage", "套利"),
    ARB_ORDER("arbitrage", "套利下单", "Arbitrage Order", "套利"),
    SMART_PORTFOLIO("smart_portfolio", "屯币宝", "Smart Portfolio", "组合/定投"),
    RECURRING_BUY("recurring", "定投策略", "Recurring Buy", "组合/定投"),
    SIGNAL_BOT("signal", "信号策略", "Signal Bot", "信号/期权"),
    DIP_SNIPER("dcd_dip", "抄底宝", "Dip Sniper", "信号/期权"),
    PEAK_SNIPER("dcd_peak", "逃顶宝", "Peak Sniper", "信号/期权"),
    FLYWHEEL("flywheel", "飞轮策略", "Flywheel", "信号/期权"),
    TWAP("twap", "时间加权 TWAP", "TWAP", "大单拆分"),
    SMART_ICEBERG("iceberg", "冰山委托", "Smart Iceberg", "大单拆分")
}

@Composable
fun StrategyBotWorkstationScreen() {
    var selectedBot by remember { mutableStateOf(OkxBotType.FUTURES_GRID) }
    val interactor = remember { ContractGridPlaceOrderInteractor("BTC-USDT-SWAP", 94850.0) }
    val gridState by interactor.stateStream.collectAsState()

    val gridCalc = remember(gridState) {
        StrategyBotMathEngine.computeGridBot(
            lastPrice = 94850.0,
            minPx = gridState.minPx.toDoubleOrNull() ?: 86000.0,
            maxPx = gridState.maxPx.toDoubleOrNull() ?: 104000.0,
            gridNum = gridState.gridNum.toIntOrNull() ?: 40,
            runType = gridState.runType,
            isContract = selectedBot != OkxBotType.SPOT_GRID,
            direction = gridState.direction,
            leverage = gridState.lever.toDoubleOrNull() ?: 10.0,
            investAmountUsdt = gridState.investMargin.toDoubleOrNull() ?: 1000.0,
            extraMarginUsdt = gridState.extraMargin.toDoubleOrNull() ?: 0.0
        )
    }

    Column(
        modifier = Modifier
            .fillMaxSize()
            .background(OkxColors.BgPrimary)
            .padding(12.dp)
    ) {
        // 1. Bot Selector Ribbon (All 15 Strategy Bots)
        LazyRow(
            horizontalArrangement = Arrangement.spacedBy(8.dp),
            modifier = Modifier.fillMaxWidth()
        ) {
            items(OkxBotType.entries) { bot ->
                val active = bot == selectedBot
                Box(
                    modifier = Modifier
                        .background(
                            if (active) OkxColors.BgInput else OkxColors.BgElevated,
                            RoundedCornerShape(6.dp)
                        )
                        .border(
                            1.dp,
                            if (active) OkxColors.TextPrimary else OkxColors.BorderSubtle,
                            RoundedCornerShape(6.dp)
                        )
                        .clickable { selectedBot = bot }
                        .padding(horizontal = 12.dp, vertical = 6.dp)
                ) {
                    Text(
                        text = "${bot.zhTitle} · ${bot.enTitle}",
                        color = if (active) OkxColors.TextPrimary else OkxColors.TextSecondary,
                        fontSize = 12.sp,
                        fontWeight = if (active) FontWeight.Bold else FontWeight.Normal
                    )
                }
            }
        }

        Spacer(modifier = Modifier.height(10.dp))

        // 2. Interactive Bot K-Line Overlay Canvas (`com.okinc.kline.features.botkline`)
        BotKlineOverlayCanvas(
            lastPrice = 94850.0,
            priceLevels = gridCalc.priceLevels,
            liqPrice = gridCalc.estimatedLiqPriceLong ?: gridCalc.estimatedLiqPriceShort,
            modifier = Modifier
                .fillMaxWidth()
                .height(190.dp)
                .background(OkxColors.BgElevated, RoundedCornerShape(8.dp))
                .border(1.dp, OkxColors.BorderSubtle, RoundedCornerShape(8.dp))
        )

        Spacer(modifier = Modifier.height(10.dp))

        // 3. Parameter Form & Risk Summary
        LazyColumn(
            verticalArrangement = Arrangement.spacedBy(10.dp),
            modifier = Modifier.weight(1f)
        ) {
            item {
                Row(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
                    StrategyBotMathEngine.BotDirection.entries.forEach { dir ->
                        val active = gridState.direction == dir
                        val activeColor = when (dir) {
                            StrategyBotMathEngine.BotDirection.LONG -> OkxColors.UpGreen
                            StrategyBotMathEngine.BotDirection.SHORT -> OkxColors.DownRed
                            StrategyBotMathEngine.BotDirection.NEUTRAL -> OkxColors.NeutralBlue
                        }
                        Button(
                            onClick = { interactor.updateParameters(direction = dir) },
                            colors = ButtonDefaults.buttonColors(
                                containerColor = if (active) activeColor else OkxColors.BgInput
                            ),
                            modifier = Modifier.weight(1f)
                        ) {
                            Text(dir.name, color = OkxColors.TextPrimary, fontSize = 12.sp)
                        }
                    }
                }
            }

            item {
                MetricSummaryCard(
                    perGridProfit = gridState.perGridProfitDisplay.render(),
                    minInvest = gridState.minInvestDisplay.render(),
                    liqPriceLong = gridState.liquidatePriceLong.render(),
                    liqPriceShort = gridState.liquidatePriceShort.render(),
                    validationError = gridState.validationError
                )
            }
        }
    }
}

@Composable
fun BotKlineOverlayCanvas(
    lastPrice: Double,
    priceLevels: List<Double>,
    liqPrice: Double?,
    modifier: Modifier = Modifier
) {
    Canvas(modifier = modifier.padding(12.dp)) {
        if (priceLevels.size < 2) return@Canvas
        val minP = priceLevels.first() * 0.985
        val maxP = priceLevels.last() * 1.015
        val span = (maxP - minP).coerceAtLeast(1.0)

        fun yForPrice(p: Double): Float {
            val norm = ((maxP - p) / span).coerceIn(0.0, 1.0)
            return (norm * size.height).toFloat()
        }

        // Draw Grid Price Ladder Lines
        val dash = PathEffect.dashPathEffect(floatArrayOf(8f, 6f), 0f)
        priceLevels.forEach { lvl ->
            val y = yForPrice(lvl)
            val color = if (lvl <= lastPrice) OkxColors.UpGreen.copy(alpha = 0.45f)
            else OkxColors.DownRed.copy(alpha = 0.45f)
            drawLine(
                color = color,
                start = Offset(0f, y),
                end = Offset(size.width, y),
                strokeWidth = 1.5f,
                pathEffect = dash
            )
        }

        // Draw Current Market Price Line
        val curY = yForPrice(lastPrice)
        drawLine(
            color = OkxColors.TextPrimary,
            start = Offset(0f, curY),
            end = Offset(size.width, curY),
            strokeWidth = 2.5f
        )

        // Draw Estimated Liquidation Price Line if within range
        if (liqPrice != null && liqPrice in minP..maxP) {
            val liqY = yForPrice(liqPrice)
            drawLine(
                color = OkxColors.WarningAmber,
                start = Offset(0f, liqY),
                end = Offset(size.width, liqY),
                strokeWidth = 2.5f
            )
        }
    }
}

@Composable
private fun MetricSummaryCard(
    perGridProfit: String,
    minInvest: String,
    liqPriceLong: String,
    liqPriceShort: String,
    validationError: String?
) {
    Column(
        modifier = Modifier
            .fillMaxWidth()
            .background(OkxColors.BgElevated, RoundedCornerShape(8.dp))
            .border(1.dp, OkxColors.BorderSubtle, RoundedCornerShape(8.dp))
            .padding(12.dp),
        verticalArrangement = Arrangement.spacedBy(6.dp)
    ) {
        MetricRow("每格利润率 (Per-Grid Profit)", perGridProfit, OkxColors.UpGreen)
        MetricRow("最低投入金额 (Min Investment)", minInvest, OkxColors.TextPrimary)
        MetricRow("预估做多强平价 (Est. Long Liq. Price)", liqPriceLong, OkxColors.WarningAmber)
        MetricRow("预估做空强平价 (Est. Short Liq. Price)", liqPriceShort, OkxColors.WarningAmber)
        if (validationError != null) {
            Text(
                text = validationError,
                color = OkxColors.DownRed,
                fontSize = 12.sp,
                fontWeight = FontWeight.Medium
            )
        }
    }
}

@Composable
private fun MetricRow(label: String, value: String, valueColor: Color) {
    Row(
        modifier = Modifier.fillMaxWidth(),
        horizontalArrangement = Arrangement.SpaceBetween
    ) {
        Text(label, color = OkxColors.TextSecondary, fontSize = 12.sp)
        Text(
            value,
            color = valueColor,
            fontSize = 12.sp,
            fontFamily = FontFamily.Monospace,
            fontWeight = FontWeight.SemiBold
        )
    }
}
