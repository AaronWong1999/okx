package com.okinc.tradingbot.ui.screens

import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontFamily
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.okinc.tradingbot.domain.math.StrategyBotMathEngine

/**
 * 1:1 Jetpack Compose Parameter & Risk-Control Forms for All 15 OKX Strategy Bot Categories
 * (`00_公共组件` through `14_AI策略与AI组合`).
 */
@Composable
fun RenderSelectedBotComposeForm(selectedBot: OkxBotType) {
    when (selectedBot) {
        OkxBotType.SPOT_DCA, OkxBotType.FUTURES_DCA -> MartingaleDcaComposePanel(
            isContract = selectedBot == OkxBotType.FUTURES_DCA
        )
        OkxBotType.SMART_ARB, OkxBotType.ARB_ORDER -> SmartArbitrageComposePanel(
            isDualLeg = selectedBot == OkxBotType.ARB_ORDER
        )
        OkxBotType.SMART_PORTFOLIO -> SmartPortfolioComposePanel()
        OkxBotType.RECURRING_BUY -> RecurringBuyComposePanel()
        OkxBotType.SIGNAL_BOT -> SignalBotComposePanel()
        OkxBotType.DIP_SNIPER, OkxBotType.PEAK_SNIPER, OkxBotType.FLYWHEEL -> DipBuyDcdComposePanel()
        OkxBotType.TWAP, OkxBotType.SMART_ICEBERG -> IcebergAndTwapComposePanel(
            isTwap = selectedBot == OkxBotType.TWAP
        )
        else -> {
            // Default Grid summary note (Spot Grid, Futures Grid, Infinity Grid)
            Text(
                text = "当前展示 ${selectedBot.zhTitle} (`uniffi.trading_bot.ContractGridPlaceOrderInteractor` / `SpotGridPlaceOrderInteractor`)",
                color = OkxColors.TextSecondary,
                fontSize = 12.sp
            )
        }
    }
}

@Composable
fun MartingaleDcaComposePanel(isContract: Boolean) {
    val calc = remember(isContract) {
        StrategyBotMathEngine.computeDcaMartingaleBot(
            entryPrice = if (isContract) 82145.8 else 105.01,
            direction = StrategyBotMathEngine.BotDirection.LONG,
            isContract = isContract,
            leverage = if (isContract) 19.0 else 1.0,
            pxStepsPct = if (isContract) 1.2 else 1.5,
            tpPct = 1.5,
            initOrdAmtUsdt = if (isContract) 10.0 else 20.0,
            safetyOrdAmtUsdt = if (isContract) 10.0 else 20.0,
            maxSafetyOrds = 8,
            pxStepsMult = 1.05,
            volMult = 1.05
        )
    }
    BotPanelCard(title = if (isContract) "04. 合约马丁格尔 (Contract DCA 19x)" else "05. 现货马丁格尔 (Spot DCA)") {
        KVRow("触发条件", "AI 参数 (RSI-14 < 30 / 短期进取建仓)")
        KVRow("跌多少加仓 / 单周期止盈", "${if (isContract) "1.2%" else "1.5%"} / 1.5%")
        KVRow("整轮所需保证金 (预留足够资金)", String.format("%.2f USDT", calc.minInvestmentUsdt))
        KVRow("预估强平价", calc.estimatedLiqPrice?.let { String.format("%.1f USDT", it) } ?: "现货无强平")
    }
}

@Composable
fun SmartArbitrageComposePanel(isDualLeg: Boolean) {
    val calc = remember {
        StrategyBotMathEngine.computeSmartArbitrage(
            totalInvestUsdt = 1000.0,
            spotPrice = 82100.0,
            swapPrice = 82145.8,
            swapLeverage = 10.0,
            stakingApyPct = 6.85
        )
    }
    BotPanelCard(title = if (isDualLeg) "06. 套利下单 (Dual-Leg Orderbook)" else "06. 智能套利 (Crypto / TradFi)") {
        KVRow("左腿 (现货买入 / 排队价)", String.format("%.2f USDT", calc.spotLegNotionalUsdt))
        KVRow("右腿 (永续做空保证金 10x)", String.format("%.2f USDT", calc.swapMarginUsdt))
        KVRow("3日综合年化 (质押+资金费)", String.format("+%.2f%%", calc.combinedApyPct))
        KVRow("两腿当前价差率", String.format("%.3f%%", calc.currentSpreadPct))
    }
}

@Composable
fun SmartPortfolioComposePanel() {
    val coins = remember {
        listOf(
            StrategyBotMathEngine.PortfolioCoinWeight("xMU", 40, 924.5),
            StrategyBotMathEngine.PortfolioCoinWeight("xBMNR", 35, 42.1),
            StrategyBotMathEngine.PortfolioCoinWeight("xREN", 25, 18.6)
        )
    }
    val calc = remember { StrategyBotMathEngine.computeSmartPortfolio(coins, 500.0) }
    BotPanelCard(title = "03 & 14. 智能组合 & AI 策略 (`SmartPortfolioRustRequestBridge`)") {
        KVRow("配置权重 (xMU / xBMNR / xREN)", "40% / 35% / 25% (合计 ${calc.weightSumPct}%)")
        KVRow("最低投资额要求", "≥ ${calc.minInvestmentUsdt.toInt()} USDT")
        KVRow("调仓条件", "按比例偏离 5% | 自动赚币 +4.62% 年化")
    }
}

@Composable
fun SignalBotComposePanel() {
    val calc = remember { StrategyBotMathEngine.computeSignalBot("TV_BTC_Breakout", 3, 5.0, 200.0) }
    BotPanelCard(title = "07. 信号策略 (`SignalBotCopyCreatePresenter` · TradingView Webhook)") {
        KVRow("自定义信号名称 (≤50字)", "TV_BTC_Breakout")
        KVRow("已选交易对 / 杠杆", "3 / 10 个交易对 · 5x")
        KVRow("Webhook Alert JSON", """{"action":"ENTER_LONG","signalToken":"okx_tv"}""")
        KVRow("参数校验状态", if (calc.valid) "通过 (≥ 50 USDT)" else (calc.validationError ?: ""))
    }
}

@Composable
fun IcebergAndTwapComposePanel(isTwap: Boolean) {
    val calc = remember(isTwap) {
        StrategyBotMathEngine.computeExecutionSlicing(
            isTwap = isTwap,
            lastPrice = 924.87,
            leverage = 3.0,
            singleOrderSz = 10.0,
            totalOrderSz = 100.0,
            activeOrdersCount = 3,
            timeIntervalSec = 15
        )
    }
    BotPanelCard(title = if (isTwap) "09. 时间加权策略 (TWAP 5~120秒)" else "08. 冰山策略 (Iceberg ≤5笔挂单)") {
        KVRow("开平方向 / 保证金模式", "开仓 (开多/开空) · ${if (isTwap) "逐仓" else "全仓"} 3x")
        KVRow("吃单价优于盘口 / 限制价", "100 USDT / 924.87 USDT")
        KVRow(if (isTwap) "下单时间间隔" else "单次并发挂单数", if (isTwap) "15 秒 (范围 5~120 秒)" else "3 笔 (上限 5 笔)")
        KVRow("所需保证金", String.format("%.2f USDT", calc.requiredMarginUsdt))
    }
}

@Composable
fun RecurringBuyComposePanel() {
    val calc = remember { StrategyBotMathEngine.computeRecurringBuy(100.0, 1200.0) }
    BotPanelCard(title = "10. 定投策略 (`RecurringTradeContainerFragment` · xStock LIVE)") {
        KVRow("精选专区组合", "科技五巨头组合 (+51.52%) / ETF 核心指数 (xQQQ)")
        KVRow("定投频率 (7档)", "1小时 | 4小时 | 8小时 | 12小时 | 每天 | 每周 | 每月")
        KVRow("单次金额 / 预计可投次数", "100 USDT (≥ 2 USDT) / ${calc.estimatedCycles} 次")
    }
}

@Composable
fun DipBuyDcdComposePanel() {
    val calc = remember { StrategyBotMathEngine.computeDipBuyTakeProfit(82226.6, 1.0, 1.0, 500.0) }
    BotPanelCard(title = "11. 抄底止盈策略 (`DcdDetailComposition` · 双币低买高卖)") {
        KVRow("买入方式 (跌幅百分比)", "1% (可选 0.1% / 1% / 5%)")
        KVRow("自动匹配目标买入价", String.format("%.0f USDT", calc.buyTargetPrice))
        KVRow("匹配产品参考年化", String.format("+%.2f%% (%d 小时)", calc.matchedReferenceApyPct, calc.settlementHours))
    }
}

@Composable
private fun BotPanelCard(title: String, content: @Composable ColumnScope.() -> Unit) {
    Column(
        modifier = Modifier
            .fillMaxWidth()
            .background(OkxColors.BgElevated, RoundedCornerShape(8.dp))
            .border(1.dp, OkxColors.BorderSubtle, RoundedCornerShape(8.dp))
            .padding(12.dp),
        verticalArrangement = Arrangement.spacedBy(6.dp)
    ) {
        Text(title, color = OkxColors.AccentLime, fontSize = 13.sp, fontWeight = FontWeight.Bold)
        content()
    }
}

@Composable
private fun KVRow(label: String, value: String) {
    Row(modifier = Modifier.fillMaxWidth(), horizontalArrangement = Arrangement.SpaceBetween) {
        Text(label, color = OkxColors.TextSecondary, fontSize = 12.sp)
        Text(value, color = OkxColors.TextPrimary, fontSize = 12.sp, fontFamily = FontFamily.Monospace, fontWeight = FontWeight.SemiBold)
    }
}
