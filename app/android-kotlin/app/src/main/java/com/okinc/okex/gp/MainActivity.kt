package com.okinc.okex.gp

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.font.FontFamily
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.okinc.tradingbot.ui.screens.OkxColors
import com.okinc.tradingbot.ui.screens.StrategyBotWorkstationScreen

/**
 * Reconstructed OKX Android v6.194.0 Whole-App Shell (`MainActivity`).
 * Implements the 5-Tab Bottom Navigation (`欧易 | 探索 | 交易 | 星球 | 资产`)
 * and 6-Tab Unified Trading Top Bar (`闪兑 | 现货 | 合约 | TradFi | 策略&跟单 | 期权`),
 * bridging `uniffi.core_trading`, `uniffi.trading_bot`, and `uniffi.social_trading`
 * inside `lib/arm64-v8a/libOKCore.so`.
 */
class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        // Load Rust UniFFI core library (`libOKCore.so`) when running on Android device
        runCatching { System.loadLibrary("OKCore") }
        setContent {
            MaterialTheme {
                OkxWholeAppRoot()
            }
        }
    }
}

enum class OkxMainBottomTab(val label: String, val iconGlyph: String) {
    HOME("欧易", "◎"),
    EXPLORE("探索", "🧭"),
    TRADE("交易", "⇄"),
    PLANET("星球", "🌐"),
    ASSETS("资产", "💼")
}

enum class OkxTradeTopTab(val label: String) {
    CONVERT("闪兑"),
    SPOT("现货"),
    FUTURES("合约"),
    TRADFI("TradFi"),
    STRATEGY_AND_COPY("策略&跟单"),
    OPTIONS("期权")
}

@Composable
fun OkxWholeAppRoot() {
    var activeMainTab by remember { mutableStateOf(OkxMainBottomTab.TRADE) }
    var activeTradeTab by remember { mutableStateOf(OkxTradeTopTab.STRATEGY_AND_COPY) }

    Scaffold(
        containerColor = OkxColors.BgPrimary,
        bottomBar = {
            Row(
                modifier = Modifier
                    .fillMaxWidth()
                    .background(OkxColors.BgElevated)
                    .padding(vertical = 8.dp),
                horizontalArrangement = Arrangement.SpaceEvenly
            ) {
                OkxMainBottomTab.entries.forEach { tab ->
                    val selected = tab == activeMainTab
                    Column(
                        horizontalAlignment = Alignment.CenterHorizontally,
                        modifier = Modifier
                            .clickable { activeMainTab = tab }
                            .padding(horizontal = 12.dp, vertical = 2.dp)
                    ) {
                        Text(tab.iconGlyph, fontSize = 15.sp, color = if (selected) OkxColors.AccentLime else OkxColors.TextSecondary)
                        Text(
                            text = tab.label,
                            fontSize = 11.sp,
                            fontWeight = if (selected) FontWeight.Bold else FontWeight.Normal,
                            color = if (selected) OkxColors.TextPrimary else OkxColors.TextSecondary
                        )
                    }
                }
            }
        }
    ) { innerPadding ->
        Column(
            modifier = Modifier
                .fillMaxSize()
                .padding(innerPadding)
        ) {
            when (activeMainTab) {
                OkxMainBottomTab.HOME -> OkxHomeOverviewScreen(
                    onJumpToBot = {
                        activeMainTab = OkxMainBottomTab.TRADE
                        activeTradeTab = OkxTradeTopTab.STRATEGY_AND_COPY
                    }
                )
                OkxMainBottomTab.EXPLORE -> OkxExploreEarnScreen(
                    onOpenDcd = {
                        activeMainTab = OkxMainBottomTab.TRADE
                        activeTradeTab = OkxTradeTopTab.STRATEGY_AND_COPY
                    }
                )
                OkxMainBottomTab.TRADE -> {
                    OkxTradeTopTabBar(
                        activeTab = activeTradeTab,
                        onSelectTab = { activeTradeTab = it }
                    )
                    if (activeTradeTab == OkxTradeTopTab.STRATEGY_AND_COPY) {
                        StrategyBotWorkstationScreen()
                    } else {
                        OkxStandardTradeScreen(
                            tradeTab = activeTradeTab,
                            onSwitchToStrategy = { activeTradeTab = OkxTradeTopTab.STRATEGY_AND_COPY }
                        )
                    }
                }
                OkxMainBottomTab.PLANET -> OkxSocialCopyTradingScreen(
                    onCopyStrategy = {
                        activeMainTab = OkxMainBottomTab.TRADE
                        activeTradeTab = OkxTradeTopTab.STRATEGY_AND_COPY
                    }
                )
                OkxMainBottomTab.ASSETS -> OkxAssetsOverviewScreen()
            }
        }
    }
}

@Composable
private fun OkxTradeTopTabBar(
    activeTab: OkxTradeTopTab,
    onSelectTab: (OkxTradeTopTab) -> Unit
) {
    Row(
        modifier = Modifier
            .fillMaxWidth()
            .background(OkxColors.BgPrimary)
            .padding(horizontal = 12.dp, vertical = 8.dp),
        horizontalArrangement = Arrangement.spacedBy(14.dp)
    ) {
        OkxTradeTopTab.entries.forEach { tab ->
            val selected = tab == activeTab
            Text(
                text = tab.label,
                color = if (selected) OkxColors.TextPrimary else OkxColors.TextSecondary,
                fontSize = if (selected) 15.sp else 13.sp,
                fontWeight = if (selected) FontWeight.ExtraBold else FontWeight.Medium,
                modifier = Modifier.clickable { onSelectTab(tab) }
            )
        }
    }
}

@Composable
private fun OkxHomeOverviewScreen(onJumpToBot: () -> Unit) {
    LazyColumn(
        modifier = Modifier.fillMaxSize().padding(16.dp),
        verticalArrangement = Arrangement.spacedBy(12.dp)
    ) {
        item {
            Text("总资产估值 (USD)", color = OkxColors.TextSecondary, fontSize = 12.sp)
            Text(
                "$12,480.50",
                color = OkxColors.TextPrimary,
                fontSize = 28.sp,
                fontWeight = FontWeight.Black,
                fontFamily = FontFamily.Monospace
            )
            Text("+$284.20 (+2.33%) 今日收益", color = OkxColors.UpGreen, fontSize = 12.sp)
        }
        item {
            Card(
                colors = CardDefaults.cardColors(containerColor = OkxColors.BgElevated),
                shape = RoundedCornerShape(12.dp),
                modifier = Modifier.fillMaxWidth().clickable { onJumpToBot() }
            ) {
                Column(modifier = Modifier.padding(14.dp)) {
                    Text("策略 Bot 智能工作台 (15 大策略全集)", color = OkxColors.TextPrimary, fontWeight = FontWeight.Bold)
                    Text(
                        "现货网格 · 合约网格(USDT/币本位) · 智能组合 · 合约/现货马丁 · 智能套利 · 信号 · 冰山 · TWAP · 定投 · 抄底止盈 · 美股代币网格",
                        color = OkxColors.TextSecondary,
                        fontSize = 12.sp
                    )
                }
            }
        }
    }
}

@Composable
private fun OkxExploreEarnScreen(onOpenDcd: () -> Unit) {
    Column(modifier = Modifier.fillMaxSize().padding(16.dp), verticalArrangement = Arrangement.spacedBy(12.dp)) {
        Text("探索 · 简单赚币与双币结构化产品", color = OkxColors.TextPrimary, fontSize = 18.sp, fontWeight = FontWeight.Bold)
        Card(
            colors = CardDefaults.cardColors(containerColor = OkxColors.BgElevated),
            modifier = Modifier.fillMaxWidth().clickable { onOpenDcd() }
        ) {
            Column(modifier = Modifier.padding(14.dp)) {
                Text("抄底止盈 (双币低买高卖循环套利)", color = OkxColors.TextPrimary, fontWeight = FontWeight.Bold)
                Text("参考年化 +58.02% · 目标价 81,700 USDT 自动匹配双币产品", color = OkxColors.UpGreen, fontSize = 12.sp)
            }
        }
    }
}

@Composable
private fun OkxSocialCopyTradingScreen(onCopyStrategy: () -> Unit) {
    Column(modifier = Modifier.fillMaxSize().padding(16.dp), verticalArrangement = Arrangement.spacedBy(12.dp)) {
        Text("星球 · 策略跟单广场 (`uniffi.social_trading`)", color = OkxColors.TextPrimary, fontSize = 18.sp, fontWeight = FontWeight.Bold)
        Card(
            colors = CardDefaults.cardColors(containerColor = OkxColors.BgElevated),
            modifier = Modifier.fillMaxWidth().clickable { onCopyStrategy() }
        ) {
            Column(modifier = Modifier.padding(14.dp)) {
                Text("桥水基金全天候加密版 · +18.14%", color = OkxColors.TextPrimary, fontWeight = FontWeight.Bold)
                Text("xStock 半导体网格带单王 (xMU / xSOXL) · +27.15% · 3,591 人已使用", color = OkxColors.UpGreen, fontSize = 12.sp)
            }
        }
    }
}

@Composable
private fun OkxAssetsOverviewScreen() {
    Column(modifier = Modifier.fillMaxSize().padding(16.dp), verticalArrangement = Arrangement.spacedBy(10.dp)) {
        Text("资产总览 (`AssetsOverviewFragment`)", color = OkxColors.TextPrimary, fontSize = 18.sp, fontWeight = FontWeight.Bold)
        Text("资金账户 (Funding): 3,200.00 USDT", color = OkxColors.TextPrimary, fontSize = 13.sp)
        Text("统一交易账户 (Unified Trading): 6,850.50 USDT", color = OkxColors.TextPrimary, fontSize = 13.sp)
        Text("策略Bot独立持仓账户 (Bot Isolated): 2,430.00 USDT", color = OkxColors.UpGreen, fontSize = 13.sp)
    }
}

@Composable
private fun OkxStandardTradeScreen(tradeTab: OkxTradeTopTab, onSwitchToStrategy: () -> Unit) {
    Column(modifier = Modifier.fillMaxSize().padding(16.dp), verticalArrangement = Arrangement.spacedBy(12.dp)) {
        Text("标准交易面板 · ${tradeTab.label} (`uniffi.core_trading`)", color = OkxColors.TextPrimary, fontSize = 16.sp, fontWeight = FontWeight.Bold)
        Text("与策略 Bot 共享底层 `libOKCore.so` 行情订阅、盘口深度与保证金风控引擎。", color = OkxColors.TextSecondary, fontSize = 12.sp)
        Button(onClick = onSwitchToStrategy, colors = ButtonDefaults.buttonColors(containerColor = Color.Black)) {
            Text("切换至「策略&跟单」工作台", color = Color.White)
        }
    }
}
