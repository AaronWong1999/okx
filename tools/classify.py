#!/usr/bin/env python3
"""Classify each deduped screen into (bot family, interface) using OCR text."""
import json
import re
from collections import OrderedDict
from pathlib import Path

SCREENS = Path("/tmp/okxbots/screens.json")
OUT = Path("/tmp/okxbots/classified.json")

# Ordered: more specific first.  Each entry -> (folder slug, display name)
BOT_MARKERS = OrderedDict([
    ("股票代币现货网格", ("08_股票代币现货网格", "股票代币现货网格 (Stock-Token Spot Grid)")),
    ("美股代币合约网格", ("09_美股代币合约网格", "美股代币合约网格 (Stock-Token Futures Grid)")),
    ("抄底止盈策略",     ("10_抄底止盈策略", "抄底止盈策略 (Dip-Buy Take-Profit)")),
    ("现货网格",         ("01_现货网格", "现货网格 (Spot Grid)")),
    ("合约网格",         ("02_合约网格", "合约网格 (Futures Grid)")),
    ("现货马丁格尔",     ("05_现货马丁格尔", "现货马丁格尔 (Spot Martingale / DCA)")),
    ("合约马丁格尔",     ("04_合约马丁格尔", "合约马丁格尔 (Futures Martingale / DCA)")),
    ("智能组合",         ("11_智能组合", "智能组合 (Smart Portfolio)")),
    ("信号策略",         ("12_信号策略", "信号策略 (Signal Bot)")),
    ("智能套利",         ("13_智能套利", "智能套利 (Smart Arbitrage)")),
    ("冰山策略",         ("06_冰山策略", "冰山策略 (Iceberg)")),
    ("时间加权策略",     ("07_时间加权策略", "时间加权策略 (TWAP)")),
    ("定投策略",         ("14_定投策略", "定投策略 (Recurring Buy)")),
    ("现货网格策略",     ("01_现货网格", "现货网格 (Spot Grid)")),
    ("网格",             ("01_现货网格", "网格 (Grid)")),
])

# interface keyword -> label
SCREEN_RULES = [
    ("预设开始条件", "预设开始条件 (Start Condition)"),
    ("预设停止条件", "预设停止条件 (Stop Condition)"),
    ("调整杠杆", "调整杠杆 (Adjust Leverage)"),
    ("智能参数", "智能参数 (Smart Parameters)"),
    ("高级设置", "高级设置 (Advanced Settings)"),
    ("单网格收益率", "单网格收益率 (Per-Grid ROI)"),
    ("网格随价格上移", "网格移动设置 (Grid Trailing)"),
    ("利润复投", "利润复投 (Profit Reinvestment)"),
    ("自动赚币", "自动赚币 (Auto-Earn)"),
    ("简单赚币分配", "简单赚币分配 (Simple Earn Allocation)"),
    ("回测年化收益率", "回测数据 (Backtest Results)"),
    ("收益预估", "收益预估 (Return Estimate)"),
    ("投资额", "投资额 (Investment Amount)"),
    ("单网格", "网格参数 (Grid Params)"),
    ("选择信号", "选择信号 (Select Signal)"),
    ("TradingView", "TradingView 信号 (TV Signal)"),
    ("加仓设定", "加仓设定 (Add-Position Rules)"),
    ("选择交易对", "选择交易对 (Select Pair)"),
    ("信号名称", "信号名称 (Signal Name)"),
    ("设置参数", "设置参数 (Set Parameters)"),
    ("手动创建", "手动创建 (Manual Creation)"),
    ("精选专区", "精选专区 (Featured Zone)"),
    ("创建策略", "创建策略 (Create Strategy)"),
    ("专区介绍", "专区介绍 (Zone Intro)"),
    ("跟随人数", "跟单/信号方 (Copy Traders)"),
    ("3日预估总年化", "智能套利收益 (Arb Yield)"),
    ("下单", "下单 (Place Order)"),
    ("合约网格", "合约网格 (Futures Grid)"),
    ("现货网格", "现货网格 (Spot Grid)"),
    ("智能组合", "智能组合 (Smart Portfolio)"),
    ("已开市", "行情页 (Market Page)"),
    ("行情", "行情页 (Market Page)"),
    ("币种", "币种选择 (Asset Picker)"),
    ("总资产估值", "资产总览 (Assets Overview)"),
    ("如何", "帮助说明 (Help Sheet)"),
    ("全仓", "保证金模式 (Margin Mode)"),
    ("预设", "预设条件 (Preset Condition)"),
    ("学习", "学习/说明 (Learn More)"),
    ("风险", "风险提示 (Risk Notice)"),
    ("简介", "策略简介 (Strategy Intro)"),
    ("总览", "总览 (Overview)"),
    ("历史", "历史记录 (History)"),
    ("事件", "事件历史 (Event History)"),
    ("订单", "订单 (Orders)"),
    ("持仓", "持仓 (Positions)"),
    ("参数", "参数设置 (Parameters)"),
    ("详情", "详情 (Details)"),
    ("列表", "列表 (List)"),
    ("记录", "记录 (Records)"),
]


def detect_bot(text):
    for marker, meta in BOT_MARKERS.items():
        if marker in text:
            return meta
    return None


def detect_screen(text):
    for kw, label in SCREEN_RULES:
        if kw in text:
            return label
    return None


def main():
    screens = json.loads(SCREENS.read_text())
    out = {}
    for vid, rows in screens.items():
        recs = []
        for r in rows:
            blob = r["header"] + " || " + " ".join(r["body"])
            bot = detect_bot(blob)
            scr = detect_screen(blob)
            recs.append({
                "file": r["file"],
                "n_dupes": r["n_dupes"],
                "header": r["header"],
                "body": r["body"],
                "bot": bot[0] if bot else None,
                "bot_name": bot[1] if bot else None,
                "screen": scr,
            })
        out[vid] = recs

    OUT.write_text(json.dumps(out, ensure_ascii=False, indent=1))

    # report
    for vid, recs in out.items():
        print(f"\n{'='*76}\n{vid}   ({len(recs)} screens)")
        for r in recs:
            b = r["bot"] or "——"
            s = r["screen"] or "——"
            print(f"  {r['file']} x{r['n_dupes']:<2} [{b:<22}] {s}")


if __name__ == "__main__":
    main()
