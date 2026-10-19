// =============================================================================
// Decompiled OKX v6.194.0 Strategy Bot Module: bot_kline_overlays
// Classes in file: 4
// =============================================================================

// DEX: classes16.dex | Source: ProGuard/R8
public final class com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2 /* obfuscated: Lo/zqh; */ extends o.uyN implements o.PVP, o.AGS, o.zHs, o.zBY, o.zxL, o.zXY, o.zyu, o.Abg, o.zTE, o.AHf {
    // Static Fields
    public static final o.zqh$TaskDescription Companion;
    public static final int copydefault;

    // Instance Fields
    public final synthetic o.zCf AEQbTJ;
    public final synthetic o.zyt AYXKKw;
    public final synthetic o.zTq AhwBna;
    public final o.adaP AkhnZx;
    public final o.adaP AuCTel;
    public o.zkI DbNXlk;
    public final synthetic o.zFR EZpvd;
    public final synthetic o.zXD KWHzl;
    public final synthetic o.zxJ OLrzqt;
    public final synthetic o.AaQ djBIcL;
    public final o.adaP ejfBZ;
    public final o.adaP fARcdN;
    public boolean fIwbmz;
    public final o.adaP fJNWhG;
    public final o.adaP fetchVPNInfo;
    public int gEmmrt;
    public kotlin.jvm.functions.Function0 hDKMBd;
    public int isConnected;
    public final o.adaP uzCIH;
    public final synthetic o.zWY valueOf;
    public com.okinc.kline.library.data.ChartViewOutSideConfig values;

    static void <clinit>() {
        0x0000: sget v0, o.Bow.KWHzl:o.Bow
        0x0002: sget v1, com.okinc.localization.report.SupportType.CLASS:com.okinc.localization.report.SupportType
        0x0004: const-string v2, "com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2"
        0x0006: const/4 v3, 0
        0x0007: invoke-virtual o.Bow.copydefault(java.lang.String, com.okinc.localization.report.SupportType, java.lang.String):void
        0x000a: new-instance v0, o.zqh$TaskDescription
        0x000c: invoke-direct o.zqh$TaskDescription.<init>(kotlin.jvm.internal.DefaultConstructorMarker):void
        0x000f: sput v0, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.Companion:o.zqh$TaskDescription
        0x0011: const/16 v0, 8
        0x0013: sput v0, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.copydefault:int
        0x0015: return-void
    }

    public void <init>() {
        0x0000: invoke-direct o.uyN.<init>():void
        0x0003: new-instance v0, o.zFR
        0x0005: invoke-direct o.zFR.<init>():void
        0x0008: iput v0, v7, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.EZpvd:o.zFR
        0x000a: new-instance v0, o.zCf
        0x000c: invoke-direct o.zCf.<init>():void
        0x000f: iput v0, v7, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.AEQbTJ:o.zCf
        0x0011: new-instance v0, o.zxJ
        0x0013: invoke-direct o.zxJ.<init>():void
        0x0016: iput v0, v7, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.OLrzqt:o.zxJ
        0x0018: new-instance v0, o.zXD
        0x001a: invoke-direct o.zXD.<init>():void
        0x001d: iput v0, v7, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.KWHzl:o.zXD
        0x001f: new-instance v0, o.zyt
        0x0021: invoke-direct o.zyt.<init>():void
        0x0024: iput v0, v7, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.AYXKKw:o.zyt
        0x0026: new-instance v0, o.AaQ
        0x0028: invoke-direct o.AaQ.<init>():void
        0x002b: iput v0, v7, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.djBIcL:o.AaQ
        0x002d: new-instance v0, o.zWY
        0x002f: const/4 v1, 0
        0x0030: const/4 v2, 1
        0x0031: const/4 v3, 0
        0x0032: invoke-direct o.zWY.<init>(boolean, int, kotlin.jvm.internal.DefaultConstructorMarker):void
        0x0035: iput v0, v7, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.valueOf:o.zWY
        0x0037: new-instance v0, o.zTq
        0x0039: invoke-direct o.zTq.<init>():void
        0x003c: iput v0, v7, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.AhwBna:o.zTq
        0x003e: new-instance v0, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2$special$$inlined$viewModels$default$1
        0x0040: invoke-direct com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2$special$$inlined$viewModels$default$1.<init>(androidx.fragment.app.Fragment):void
        0x0043: sget v1, kotlin.LazyThreadSafetyMode.NONE:kotlin.LazyThreadSafetyMode
        0x0045: new-instance v2, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2$special$$inlined$viewModels$default$2
        0x0047: invoke-direct com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2$special$$inlined$viewModels$default$2.<init>(kotlin.jvm.functions.Function0):void
        0x004a: invoke-static o.adaR.EZpvd(kotlin.LazyThreadSafetyMode, kotlin.jvm.functions.Function0):o.adaP
        0x004e: const-class v2, o.zOd
        0x0050: invoke-static o.adfG.KWHzl(java.lang.Class):o.adgF
        0x0054: new-instance v4, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2$special$$inlined$viewModels$default$3
        0x0056: invoke-direct com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2$special$$inlined$viewModels$default$3.<init>(o.adaP):void
        0x0059: new-instance v5, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2$special$$inlined$viewModels$default$4
        0x005b: invoke-direct com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2$special$$inlined$viewModels$default$4.<init>(kotlin.jvm.functions.Function0, o.adaP):void
        0x005e: new-instance v6, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2$special$$inlined$viewModels$default$5
        0x0060: invoke-direct com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2$special$$inlined$viewModels$default$5.<init>(androidx.fragment.app.Fragment, o.adaP):void
        0x0063: invoke-static androidx.fragment.app.FragmentViewModelLazyKt.createViewModelLazy(androidx.fragment.app.Fragment, o.adgF, kotlin.jvm.functions.Function0, kotlin.jvm.functions.Function0, kotlin.jvm.functions.Function0):o.adaP
        0x0067: iput v0, v7, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.uzCIH:o.adaP
        0x0069: new-instance v0, o.zqn
        0x006b: invoke-direct o.zqn.<init>(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2):void
        0x006e: invoke-static o.adaR.KWHzl(kotlin.jvm.functions.Function0):o.adaP
        0x0072: iput v0, v7, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.ejfBZ:o.adaP
        0x0074: new-instance v0, o.zqm
        0x0076: invoke-direct o.zqm.<init>(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2):void
        0x0079: invoke-static o.adaR.KWHzl(kotlin.jvm.functions.Function0):o.adaP
        0x007d: iput v0, v7, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.fJNWhG:o.adaP
        0x007f: new-instance v0, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2$special$$inlined$viewModels$default$6
        0x0081: invoke-direct com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2$special$$inlined$viewModels$default$6.<init>(androidx.fragment.app.Fragment):void
        0x0084: new-instance v2, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2$special$$inlined$viewModels$default$7
        0x0086: invoke-direct com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2$special$$inlined$viewModels$default$7.<init>(kotlin.jvm.functions.Function0):void
        0x0089: invoke-static o.adaR.EZpvd(kotlin.LazyThreadSafetyMode, kotlin.jvm.functions.Function0):o.adaP
        0x008d: const-class v1, o.ABT
        0x008f: invoke-static o.adfG.KWHzl(java.lang.Class):o.adgF
        0x0093: new-instance v2, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2$special$$inlined$viewModels$default$8
        0x0095: invoke-direct com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2$special$$inlined$viewModels$default$8.<init>(o.adaP):void
        0x0098: new-instance v4, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2$special$$inlined$viewModels$default$9
        0x009a: invoke-direct com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2$special$$inlined$viewModels$default$9.<init>(kotlin.jvm.functions.Function0, o.adaP):void
        0x009d: new-instance v5, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2$special$$inlined$viewModels$default$10
        0x009f: invoke-direct com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2$special$$inlined$viewModels$default$10.<init>(androidx.fragment.app.Fragment, o.adaP):void
        0x00a2: invoke-static androidx.fragment.app.FragmentViewModelLazyKt.createViewModelLazy(androidx.fragment.app.Fragment, o.adgF, kotlin.jvm.functions.Function0, kotlin.jvm.functions.Function0, kotlin.jvm.functions.Function0):o.adaP
        0x00a6: iput v0, v7, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.AuCTel:o.adaP
        0x00a8: const-class v0, o.BaG
        0x00aa: invoke-static o.adfG.KWHzl(java.lang.Class):o.adgF
        0x00ae: new-instance v1, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2$special$$inlined$activityViewModels$default$1
        0x00b0: invoke-direct com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2$special$$inlined$activityViewModels$default$1.<init>(androidx.fragment.app.Fragment):void
        0x00b3: new-instance v2, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2$special$$inlined$activityViewModels$default$2
        0x00b5: invoke-direct com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2$special$$inlined$activityViewModels$default$2.<init>(kotlin.jvm.functions.Function0, androidx.fragment.app.Fragment):void
        0x00b8: new-instance v4, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2$special$$inlined$activityViewModels$default$3
        0x00ba: invoke-direct com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2$special$$inlined$activityViewModels$default$3.<init>(androidx.fragment.app.Fragment):void
        0x00bd: invoke-static androidx.fragment.app.FragmentViewModelLazyKt.createViewModelLazy(androidx.fragment.app.Fragment, o.adgF, kotlin.jvm.functions.Function0, kotlin.jvm.functions.Function0, kotlin.jvm.functions.Function0):o.adaP
        0x00c1: iput v0, v7, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.fARcdN:o.adaP
        0x00c3: const-class v0, com.okinc.kline.ui.bot.KlineBotViewModel
        0x00c5: invoke-static o.adfG.KWHzl(java.lang.Class):o.adgF
        0x00c9: new-instance v1, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2$special$$inlined$activityViewModels$default$4
        0x00cb: invoke-direct com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2$special$$inlined$activityViewModels$default$4.<init>(androidx.fragment.app.Fragment):void
        0x00ce: new-instance v2, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2$special$$inlined$activityViewModels$default$5
        0x00d0: invoke-direct com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2$special$$inlined$activityViewModels$default$5.<init>(kotlin.jvm.functions.Function0, androidx.fragment.app.Fragment):void
        0x00d3: new-instance v3, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2$special$$inlined$activityViewModels$default$6
        0x00d5: invoke-direct com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2$special$$inlined$activityViewModels$default$6.<init>(androidx.fragment.app.Fragment):void
        0x00d8: invoke-static androidx.fragment.app.FragmentViewModelLazyKt.createViewModelLazy(androidx.fragment.app.Fragment, o.adgF, kotlin.jvm.functions.Function0, kotlin.jvm.functions.Function0, kotlin.jvm.functions.Function0):o.adaP
        0x00dc: iput v0, v7, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.fetchVPNInfo:o.adaP
        0x00de: const/4 v0, -1
        0x00df: iput v0, v7, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.isConnected:int
        0x00e1: new-instance v0, o.zqo
        0x00e3: invoke-direct o.zqo.<init>(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2):void
        0x00e6: invoke-static o.adaR.KWHzl(kotlin.jvm.functions.Function0):o.adaP
        0x00ea: iput v0, v7, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.AkhnZx:o.adaP
        0x00ec: return-void
    }

    public static synthetic java.lang.String AEQbTJ(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2 p0) {
        0x0000: invoke-static com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.wlaJM(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2):java.lang.String
        0x0004: return-object v0
    }

    public static final kotlin.Unit AEQbTJ(com.okinc.kline.data.TimeIntervalItem p0) {
        0x0000: const-string v0, ""
        0x0002: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0005: invoke-virtual com.okinc.kline.data.TimeIntervalItem.getTradingType():java.lang.String
        0x0009: const/4 v0, 1
        0x000a: invoke-static o.AXL.AEQbTJ(java.lang.String, boolean):void
        0x000d: sget v1, kotlin.Unit.INSTANCE:kotlin.Unit
        0x000f: return-object v1
    }

    public static synthetic kotlin.Unit AEQbTJ(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2 p0, int p1) {
        0x0000: invoke-static com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.OLrzqt(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2, int):kotlin.Unit
        0x0004: return-object v0
    }

    public static synthetic void AEQbTJ(kotlin.jvm.functions.Function1 p0, java.lang.Object p1) {
        0x0000: invoke-static com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.djBIcL(kotlin.jvm.functions.Function1, java.lang.Object):void
        0x0003: return-void
    }

    public static final synthetic void AYXKKw(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2 p0) {
        0x0000: invoke-direct com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.QfsBiF():void
        0x0003: return-void
    }

    public static final synthetic com.okinc.kline.library.data.ChartViewOutSideConfig AhwBna(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2 p0) {
        0x0000: iget v0, v0, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.values:com.okinc.kline.library.data.ChartViewOutSideConfig
        0x0002: return-object v0
    }

    public static final void AhwBna(kotlin.jvm.functions.Function1 p0, java.lang.Object p1) {
        0x0000: invoke-interface kotlin.jvm.functions.Function1.invoke(java.lang.Object):java.lang.Object
        0x0003: return-void
    }

    public static final synthetic java.lang.String AkhnZx(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2 p0) {
        0x0000: invoke-direct com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.djSkpj():java.lang.String
        0x0004: return-object v0
    }

    public static final synthetic boolean AuCTel(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2 p0) {
        0x0000: invoke-virtual com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.wlaJM():boolean
        0x0004: return v0
    }

    public static final java.lang.String AubY(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2 p0) {
        0x0000: invoke-virtual androidx.fragment.app.Fragment.getArguments():android.os.Bundle
        0x0006: invoke-static o.zjY.djBIcL():java.lang.String
        0x000a: invoke-virtual android.os.BaseBundle.getString(java.lang.String):java.lang.String
        0x000f: const/4 v1, 0
        0x0012: const-string v1, ""
        0x0014: return-object v1
    }

    public static final synthetic o.ABT DbNXlk(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2 p0) {
        0x0000: invoke-direct com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.QbewEr():o.ABT
        0x0004: return-object v0
    }

    public static final kotlin.Unit EZpvd() {
        0x0000: const/4 v0, 1
        0x0001: invoke-static o.AXL.copydefault(boolean):void
        0x0004: sget v0, kotlin.Unit.INSTANCE:kotlin.Unit
        0x0006: return-object v0
    }

    public static synthetic kotlin.Unit EZpvd(com.okinc.kline.data.TimeIntervalItem p0) {
        0x0000: invoke-static com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.AEQbTJ(com.okinc.kline.data.TimeIntervalItem):kotlin.Unit
        0x0004: return-object v0
    }

    public static final kotlin.Unit EZpvd(com.okinc.kline.ui.view.model.MarketTypeSelect p0) {
        0x0000: const-string v0, ""
        0x0002: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0005: invoke-virtual com.okinc.kline.ui.view.model.MarketTypeSelect.getSelectTypeName():java.lang.String
        0x0009: invoke-virtual com.okinc.kline.ui.view.model.MarketTypeSelect.isSelect():boolean
        0x000d: invoke-virtual com.okinc.kline.ui.view.model.MarketTypeSelect.getGroup():int
        0x0013: const/4 v8, 1
        0x0015: const/4 v8, 0
        0x0017: const/4 v4, 1
        0x0018: const/4 v5, 0
        0x0019: const/16 v6, 16
        0x001b: const/4 v7, 0
        0x001c: invoke-static/range o.AXL.indicatorSelectFragmentEvent$default(java.lang.String, boolean, boolean, boolean, java.lang.Boolean, int, java.lang.Object):void
        0x001f: sget v8, kotlin.Unit.INSTANCE:kotlin.Unit
        0x0021: return-object v8
    }

    public static final kotlin.Unit EZpvd(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2 p0, com.okinc.kline.data.TimeIntervalItem p1) {
        0x0000: const-string v0, ""
        0x0002: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0005: iget v2, v2, com.okinc.kline.data.TimeIntervalItem.timeInterval:int
        0x0007: invoke-virtual com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.AEQbTJ(int):void
        0x000a: sget v1, kotlin.Unit.INSTANCE:kotlin.Unit
        0x000c: return-object v1
    }

    public static final kotlin.Unit EZpvd(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2 p0, java.lang.String p1) {
        0x0000: invoke-virtual androidx.fragment.app.Fragment.getActivity():androidx.fragment.app.FragmentActivity
        0x0006: invoke-static o.ADg.AEQbTJ(android.app.Activity):void
        0x0009: sget v0, kotlin.Unit.INSTANCE:kotlin.Unit
        0x000b: return-object v0
    }

    public static synthetic kotlin.Unit EZpvd(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2 p0, java.lang.Throwable p1) {
        0x0000: invoke-static com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.OLrzqt(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2, java.lang.Throwable):kotlin.Unit
        0x0004: return-object v0
    }

    public static synthetic o.zFT EZpvd(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2 p0) {
        0x0000: invoke-static com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.getFieldNames(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2):o.zFT
        0x0004: return-object v0
    }

    public static synthetic void EZpvd(kotlin.jvm.functions.Function1 p0, java.lang.Object p1) {
        0x0000: invoke-static com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.gEmmrt(kotlin.jvm.functions.Function1, java.lang.Object):void
        0x0003: return-void
    }

    public static synthetic void EZpvd(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2 p0, android.view.View p1) {
        0x0000: invoke-static com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.KWHzl(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2, android.view.View):void
        0x0003: return-void
    }

    public static final synthetic void EZpvd(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2 p0, kotlin.jvm.functions.Function0 p1) {
        0x0000: iput v1, v0, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.hDKMBd:kotlin.jvm.functions.Function0
        0x0002: return-void
    }

    public static synthetic java.lang.String KWHzl(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2 p0) {
        0x0000: invoke-static com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.AubY(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2):java.lang.String
        0x0004: return-object v0
    }

    public static synthetic kotlin.Unit KWHzl(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2 p0, com.okinc.kline.data.TimeIntervalItem p1) {
        0x0000: invoke-static com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.EZpvd(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2, com.okinc.kline.data.TimeIntervalItem):kotlin.Unit
        0x0004: return-object v0
    }

    public static synthetic kotlin.Unit KWHzl(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2 p0, java.lang.String p1) {
        0x0000: invoke-static com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.EZpvd(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2, java.lang.String):kotlin.Unit
        0x0004: return-object v0
    }

    public static final kotlin.Unit KWHzl(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2 p0, java.lang.Throwable p1) {
        0x0000: invoke-virtual o.uyN.getTAG():java.lang.String
        0x0004: const-string v0, "KLINE_TRADE_POINT_CLICK error"
        0x0006: invoke-static o.BqX.AEQbTJ(java.lang.String, java.lang.String, java.lang.Throwable):void
        0x0009: sget v1, kotlin.Unit.INSTANCE:kotlin.Unit
        0x000b: return-object v1
    }

    public static synthetic void KWHzl(kotlin.jvm.functions.Function1 p0, java.lang.Object p1) {
        0x0000: invoke-static com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.copydefault(kotlin.jvm.functions.Function1, java.lang.Object):void
        0x0003: return-void
    }

    public static final void KWHzl(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2 p0, android.view.View p1) {
        0x0000: iget v2, v1, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.DbNXlk:o.zkI
        0x0004: const-string v2, ""
        0x0006: invoke-static kotlin.jvm.internal.Intrinsics.valueOf(java.lang.String):void
        0x0009: const/4 v2, 0
        0x000a: iget v2, v2, o.zkI.OLrzqt:android.widget.LinearLayout
        0x000c: const/16 v0, 8
        0x000e: invoke-virtual android.view.View.setVisibility(int):void
        0x0011: invoke-direct com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.RcXXUw():void
        0x0014: return-void
    }

    public static final kotlin.Unit OLrzqt(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2 p0, int p1) {
        0x0000: invoke-virtual com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.OLrzqt(int):void
        0x0003: sget v0, kotlin.Unit.INSTANCE:kotlin.Unit
        0x0005: return-object v0
    }

    public static synthetic kotlin.Unit OLrzqt(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2 p0, java.lang.String p1) {
        0x0000: invoke-static com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.copydefault(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2, java.lang.String):kotlin.Unit
        0x0004: return-object v0
    }

    public static final kotlin.Unit OLrzqt(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2 p0, java.lang.Throwable p1) {
        0x0000: invoke-virtual o.uyN.getTAG():java.lang.String
        0x0004: const-string v0, "KLINE_THEME_TYPE error"
        0x0006: invoke-static o.BqX.AEQbTJ(java.lang.String, java.lang.String, java.lang.Throwable):void
        0x0009: sget v1, kotlin.Unit.INSTANCE:kotlin.Unit
        0x000b: return-object v1
    }

    public static synthetic void OLrzqt(kotlin.jvm.functions.Function1 p0, java.lang.Object p1) {
        0x0000: invoke-static com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.AhwBna(kotlin.jvm.functions.Function1, java.lang.Object):void
        0x0003: return-void
    }

    public static synthetic void OLrzqt(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2 p0) {
        0x0000: invoke-static com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.zLjUOn(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2):void
        0x0003: return-void
    }

    private final java.lang.String QUSxYX() {
        0x0000: iget v0, v1, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.ejfBZ:o.adaP
        0x0002: invoke-interface o.adaP.getValue():java.lang.Object
        0x0006: check-cast v0, java.lang.String
        0x0008: return-object v0
    }

    private final o.ABT QbewEr() {
        0x0000: iget v0, v1, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.AuCTel:o.adaP
        0x0002: invoke-interface o.adaP.getValue():java.lang.Object
        0x0006: check-cast v0, o.ABT
        0x0008: return-object v0
    }

    private final void QfsBiF() {
        0x0000: iget v0, v2, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.DbNXlk:o.zkI
        0x0004: const-string v0, ""
        0x0006: invoke-static kotlin.jvm.internal.Intrinsics.valueOf(java.lang.String):void
        0x0009: const/4 v0, 0
        0x000a: iget v0, v0, o.zkI.copydefault:o.abdw
        0x000c: const/16 v1, 8
        0x000e: invoke-virtual o.abdw.setVisibility(int):void
        0x0011: invoke-virtual o.abdw.EZpvd():void
        0x0014: return-void
    }

    private final o.BaG RJOkX() {
        0x0000: iget v0, v1, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.fARcdN:o.adaP
        0x0002: invoke-interface o.adaP.getValue():java.lang.Object
        0x0006: check-cast v0, o.BaG
        0x0008: return-object v0
    }

    private final void RcXXUw() {
        0x0000: invoke-direct com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.QUSxYX():java.lang.String
        0x0004: invoke-interface java.lang.CharSequence.length():int
        0x000a: return-void
        0x000b: invoke-direct com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.djSkpj():java.lang.String
        0x000f: invoke-interface java.lang.CharSequence.length():int
        0x0015: return-void
        0x0016: invoke-virtual o.uyN.getTAG():java.lang.String
        0x001a: invoke-direct com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.QUSxYX():java.lang.String
        0x001e: invoke-direct com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.djSkpj():java.lang.String
        0x0022: invoke-direct com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.aKErDB():long
        0x0026: new-instance v5, java.lang.StringBuilder
        0x0028: invoke-direct java.lang.StringBuilder.<init>():void
        0x002b: const-string v6, "kickoff inst="
        0x002d: invoke-virtual java.lang.StringBuilder.append(java.lang.String):java.lang.StringBuilder
        0x0030: invoke-virtual java.lang.StringBuilder.append(java.lang.String):java.lang.StringBuilder
        0x0033: const-string v1, " type="
        0x0035: invoke-virtual java.lang.StringBuilder.append(java.lang.String):java.lang.StringBuilder
        0x0038: invoke-virtual java.lang.StringBuilder.append(java.lang.String):java.lang.StringBuilder
        0x003b: const-string v1, " sid="
        0x003d: invoke-virtual java.lang.StringBuilder.append(java.lang.String):java.lang.StringBuilder
        0x0040: invoke-virtual java.lang.StringBuilder.append(long):java.lang.StringBuilder
        0x0043: invoke-virtual java.lang.StringBuilder.toString():java.lang.String
        0x0047: invoke-static o.BqX.OLrzqt(java.lang.String, java.lang.String):void
        0x004a: invoke-static androidx.lifecycle.LifecycleOwnerKt.getLifecycleScope(androidx.lifecycle.LifecycleOwner):androidx.lifecycle.LifecycleCoroutineScope
        0x004e: const/4 v3, 0
        0x004f: const/4 v4, 0
        0x0050: new-instance v5, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2$kickoffSubscription$1
        0x0052: const/4 v0, 0
        0x0053: invoke-direct com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2$kickoffSubscription$1.<init>(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2, kotlin.coroutines.Continuation):void
        0x0056: const/4 v6, 3
        0x0057: const/4 v7, 0
        0x0058: invoke-static/range kotlinx.coroutines.BuildersKt.launch$default(kotlinx.coroutines.CoroutineScope, kotlin.coroutines.CoroutineContext, kotlinx.coroutines.CoroutineStart, kotlin.jvm.functions.Function2, int, java.lang.Object):kotlinx.coroutines.Job
        0x005b: return-void
    }

    private final void UeEOUB() {
        0x0000: invoke-virtual androidx.fragment.app.Fragment.getViewLifecycleOwner():androidx.lifecycle.LifecycleOwner
        0x0004: const-string v7, ""
        0x0006: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullExpressionValue(java.lang.Object, java.lang.String):void
        0x0009: invoke-static androidx.lifecycle.LifecycleOwnerKt.getLifecycleScope(androidx.lifecycle.LifecycleOwner):androidx.lifecycle.LifecycleCoroutineScope
        0x000d: const/4 v2, 0
        0x000e: const/4 v3, 0
        0x000f: new-instance v4, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2$observeState$1
        0x0011: const/4 v8, 0
        0x0012: invoke-direct com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2$observeState$1.<init>(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2, kotlin.coroutines.Continuation):void
        0x0015: const/4 v5, 3
        0x0016: const/4 v6, 0
        0x0017: invoke-static/range kotlinx.coroutines.BuildersKt.launch$default(kotlinx.coroutines.CoroutineScope, kotlin.coroutines.CoroutineContext, kotlinx.coroutines.CoroutineStart, kotlin.jvm.functions.Function2, int, java.lang.Object):kotlinx.coroutines.Job
        0x001a: invoke-virtual androidx.fragment.app.Fragment.getViewLifecycleOwner():androidx.lifecycle.LifecycleOwner
        0x001e: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullExpressionValue(java.lang.Object, java.lang.String):void
        0x0021: invoke-static androidx.lifecycle.LifecycleOwnerKt.getLifecycleScope(androidx.lifecycle.LifecycleOwner):androidx.lifecycle.LifecycleCoroutineScope
        0x0025: new-instance v4, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2$observeState$2
        0x0027: invoke-direct com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2$observeState$2.<init>(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2, kotlin.coroutines.Continuation):void
        0x002a: invoke-static/range kotlinx.coroutines.BuildersKt.launch$default(kotlinx.coroutines.CoroutineScope, kotlin.coroutines.CoroutineContext, kotlinx.coroutines.CoroutineStart, kotlin.jvm.functions.Function2, int, java.lang.Object):kotlinx.coroutines.Job
        0x002d: invoke-virtual com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.getFieldNames():o.zOd
        0x0031: sget v2, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2$observeState$3.INSTANCE:com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2$observeState$3
        0x0033: new-instance v4, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2$observeState$4
        0x0035: invoke-direct com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2$observeState$4.<init>(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2, kotlin.coroutines.Continuation):void
        0x0038: const/4 v5, 2
        0x003a: invoke-static/range o.PVT$ActionBar.collectLatestOnLifecycle$default(o.PVT, o.PVO, o.adgO, androidx.lifecycle.Lifecycle$State, kotlin.jvm.functions.Function2, int, java.lang.Object):kotlinx.coroutines.Job
        0x003d: invoke-virtual com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.getFieldNames():o.zOd
        0x0041: sget v2, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2$observeState$5.INSTANCE:com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2$observeState$5
        0x0043: new-instance v4, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2$observeState$6
        0x0045: invoke-direct com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2$observeState$6.<init>(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2, kotlin.coroutines.Continuation):void
        0x0048: invoke-static/range o.PVT$ActionBar.collectLatestOnLifecycle$default(o.PVT, o.PVO, o.adgO, androidx.lifecycle.Lifecycle$State, kotlin.jvm.functions.Function2, int, java.lang.Object):kotlinx.coroutines.Job
        0x004b: invoke-virtual androidx.fragment.app.Fragment.getViewLifecycleOwner():androidx.lifecycle.LifecycleOwner
        0x004f: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullExpressionValue(java.lang.Object, java.lang.String):void
        0x0052: invoke-static androidx.lifecycle.LifecycleOwnerKt.getLifecycleScope(androidx.lifecycle.LifecycleOwner):androidx.lifecycle.LifecycleCoroutineScope
        0x0056: const/4 v2, 0
        0x0057: new-instance v4, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2$observeState$7
        0x0059: invoke-direct com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2$observeState$7.<init>(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2, kotlin.coroutines.Continuation):void
        0x005c: const/4 v5, 3
        0x005d: invoke-static/range kotlinx.coroutines.BuildersKt.launch$default(kotlinx.coroutines.CoroutineScope, kotlin.coroutines.CoroutineContext, kotlinx.coroutines.CoroutineStart, kotlin.jvm.functions.Function2, int, java.lang.Object):kotlinx.coroutines.Job
        0x0060: return-void
    }

    private final long aKErDB() {
        0x0000: invoke-virtual androidx.fragment.app.Fragment.getArguments():android.os.Bundle
        0x0004: const-wide/16 v1, 0
        0x0008: const-string v3, "kline_bot_sid"
        0x000a: invoke-virtual android.os.BaseBundle.getLong(java.lang.String, long):long
        0x000e: return-wide v0
        0x000f: return-wide v1
    }

    public static synthetic kotlin.Unit copydefault() {
        0x0000: invoke-static com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.EZpvd():kotlin.Unit
        0x0004: return-object v0
    }

    public static synthetic kotlin.Unit copydefault(com.okinc.kline.ui.view.model.MarketTypeSelect p0) {
        0x0000: invoke-static com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.EZpvd(com.okinc.kline.ui.view.model.MarketTypeSelect):kotlin.Unit
        0x0004: return-object v0
    }

    public static final kotlin.Unit copydefault(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2 p0, java.lang.String p1) {
        0x0000: iget v1, v0, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.hDKMBd:kotlin.jvm.functions.Function0
        0x0004: invoke-interface kotlin.jvm.functions.Function0.invoke():java.lang.Object
        0x0007: iget v0, v0, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.DbNXlk:o.zkI
        0x000b: const-string v0, ""
        0x000d: invoke-static kotlin.jvm.internal.Intrinsics.valueOf(java.lang.String):void
        0x0010: const/4 v0, 0
        0x0011: iget v0, v0, o.zkI.KWHzl:o.Acd
        0x0013: invoke-virtual o.Acd.AEQbTJ():void
        0x0016: sget v0, kotlin.Unit.INSTANCE:kotlin.Unit
        0x0018: return-object v0
    }

    public static synthetic kotlin.Unit copydefault(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2 p0, java.lang.Throwable p1) {
        0x0000: invoke-static com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.KWHzl(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2, java.lang.Throwable):kotlin.Unit
        0x0004: return-object v0
    }

    public static final void copydefault(kotlin.jvm.functions.Function1 p0, java.lang.Object p1) {
        0x0000: invoke-interface kotlin.jvm.functions.Function1.invoke(java.lang.Object):java.lang.Object
        0x0003: return-void
    }

    public static final synthetic void copydefault(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2 p0) {
        0x0000: invoke-virtual com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.KWHzl():void
        0x0003: return-void
    }

    public static final synthetic void copydefault(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2 p0, int p1) {
        0x0000: iput v1, v0, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.gEmmrt:int
        0x0002: return-void
    }

    public static final synthetic void copydefault(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2 p0, boolean p1) {
        0x0000: invoke-virtual com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.KWHzl(boolean):void
        0x0003: return-void
    }

    private final void dNCPSb() {
        0x0000: iget v0, v4, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.DbNXlk:o.zkI
        0x0002: const-string v1, ""
        0x0004: const/4 v2, 0
        0x0007: invoke-static kotlin.jvm.internal.Intrinsics.valueOf(java.lang.String):void
        0x000b: iget v0, v0, o.zkI.OLrzqt:android.widget.LinearLayout
        0x000d: invoke-virtual android.view.View.getVisibility():int
        0x0011: const/16 v3, 8
        0x0015: iget v0, v4, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.DbNXlk:o.zkI
        0x0019: invoke-static kotlin.jvm.internal.Intrinsics.valueOf(java.lang.String):void
        0x001e: iget v0, v2, o.zkI.OLrzqt:android.widget.LinearLayout
        0x0020: invoke-virtual android.view.View.setVisibility(int):void
        0x0023: return-void
    }

    public static final synthetic com.okinc.kline.library.data.ChartViewOutSideConfig djBIcL(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2 p0) {
        0x0000: invoke-virtual com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.OLrzqt():com.okinc.kline.library.data.ChartViewOutSideConfig
        0x0004: return-object v0
    }

    public static final void djBIcL(kotlin.jvm.functions.Function1 p0, java.lang.Object p1) {
        0x0000: invoke-interface kotlin.jvm.functions.Function1.invoke(java.lang.Object):java.lang.Object
        0x0003: return-void
    }

    private final java.lang.String djSkpj() {
        0x0000: iget v0, v1, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.fJNWhG:o.adaP
        0x0002: invoke-interface o.adaP.getValue():java.lang.Object
        0x0006: check-cast v0, java.lang.String
        0x0008: return-object v0
    }

    private final void dvKsVJ() {
        0x0000: invoke-virtual com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.fJNWhG():com.okinc.kline.ui.bot.KlineBotViewModel
        0x0004: invoke-direct com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.aKErDB():long
        0x0008: invoke-direct com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.QUSxYX():java.lang.String
        0x000c: invoke-virtual com.okinc.kline.ui.bot.KlineBotViewModel.OLrzqt(long, java.lang.String):o.Aww
        0x0012: return-void
        0x0013: invoke-virtual o.Aww.EZpvd():com.okinc.kline.data.BotPendingLineVo
        0x0017: const-string v2, ""
        0x0019: const/4 v3, 0
        0x001c: iget v4, v5, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.DbNXlk:o.zkI
        0x0020: invoke-static kotlin.jvm.internal.Intrinsics.valueOf(java.lang.String):void
        0x0024: iget v4, v4, o.zkI.KWHzl:o.Acd
        0x0026: invoke-virtual o.Acd.setBotOrdersData(com.okinc.kline.data.BotPendingLineVo):void
        0x0029: iget v1, v5, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.DbNXlk:o.zkI
        0x002d: invoke-static kotlin.jvm.internal.Intrinsics.valueOf(java.lang.String):void
        0x0031: iget v1, v1, o.zkI.KWHzl:o.Acd
        0x0033: invoke-virtual o.Acd.getChartManager():o.AbZ
        0x0037: invoke-virtual o.Aww.OLrzqt():o.AHi
        0x003b: invoke-virtual o.AbZ.KWHzl(o.AHi):void
        0x003e: iget v1, v5, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.DbNXlk:o.zkI
        0x0042: invoke-static kotlin.jvm.internal.Intrinsics.valueOf(java.lang.String):void
        0x0047: iget v1, v3, o.zkI.KWHzl:o.Acd
        0x0049: invoke-virtual o.Acd.getChartManager():o.AbZ
        0x004d: invoke-virtual o.Aww.KWHzl():com.okinc.kline.data.BotPendingLineVo
        0x0051: invoke-virtual o.AbZ.EZpvd(com.okinc.kline.data.BotPendingLineVo):void
        0x0054: return-void
    }

    public static final synthetic void ejfBZ(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2 p0) {
        0x0000: invoke-direct com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.RcXXUw():void
        0x0003: return-void
    }

    public static final synthetic o.zOd fARcdN(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2 p0) {
        0x0000: invoke-virtual com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.getFieldNames():o.zOd
        0x0004: return-object v0
    }

    public static final synthetic void fIwbmz(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2 p0) {
        0x0000: invoke-direct com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.dNCPSb():void
        0x0003: return-void
    }

    public static final synthetic long fJNWhG(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2 p0) {
        0x0000: invoke-direct com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.aKErDB():long
        0x0004: return-wide v0
    }

    private final void fZBcTu() {
        0x0000: iget v0, v3, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.DbNXlk:o.zkI
        0x0004: const-string v0, ""
        0x0006: invoke-static kotlin.jvm.internal.Intrinsics.valueOf(java.lang.String):void
        0x0009: const/4 v0, 0
        0x000a: iget v0, v0, o.zkI.copydefault:o.abdw
        0x000c: invoke-virtual com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.QVAiDq():void
        0x000f: const/4 v1, 0
        0x0010: invoke-virtual o.abdw.setVisibility(int):void
        0x0013: sget v1, o.aaVP$Dialog.values:int
        0x0015: invoke-virtual o.abdw.setAnimation(int):void
        0x0018: const-wide/16 v1, 0
        0x001a: invoke-virtual o.abdw.copydefault(long):void
        0x001d: return-void
    }

    public static final synthetic java.lang.String fetchVPNInfo(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2 p0) {
        0x0000: invoke-direct com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.QUSxYX():java.lang.String
        0x0004: return-object v0
    }

    private final void finit() {
        0x0000: iget v0, v2, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.DbNXlk:o.zkI
        0x0004: const-string v0, ""
        0x0006: invoke-static kotlin.jvm.internal.Intrinsics.valueOf(java.lang.String):void
        0x0009: const/4 v0, 0
        0x000a: iget v0, v0, o.zkI.OLrzqt:android.widget.LinearLayout
        0x000c: const/4 v1, 0
        0x000d: invoke-virtual android.view.View.setVisibility(int):void
        0x0010: return-void
    }

    public static final synthetic com.okinc.kline.ui.bot.KlineBotViewModel gEmmrt(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2 p0) {
        0x0000: invoke-virtual com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.fJNWhG():com.okinc.kline.ui.bot.KlineBotViewModel
        0x0004: return-object v0
    }

    public static final void gEmmrt(kotlin.jvm.functions.Function1 p0, java.lang.Object p1) {
        0x0000: invoke-interface kotlin.jvm.functions.Function1.invoke(java.lang.Object):java.lang.Object
        0x0003: return-void
    }

    public static final o.zFT getFieldNames(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2 p0) {
        0x0000: invoke-direct com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.aKErDB():long
        0x0004: const-wide/16 v2, 0
        0x000a: sget v0, o.AYO.EZpvd:o.AYO
        0x000c: invoke-virtual o.AYO.valueOf():boolean
        0x0012: sget v0, o.AWh.copydefault:o.AWh
        0x0014: invoke-virtual o.AWh.AYXKKw():boolean
        0x001a: new-instance v0, o.zFP$ActionBar
        0x001c: invoke-direct com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.aKErDB():long
        0x0020: invoke-direct o.zFP$ActionBar.<init>(long):void
        0x0024: sget v0, o.zFP$Application.KWHzl:o.zFP$Application
        0x0027: new-instance v8, o.zFT
        0x0029: const/4 v2, 0
        0x002a: const/4 v3, 0
        0x002b: const/4 v5, 0
        0x002c: const/16 v6, 11
        0x002e: const/4 v7, 0
        0x0030: invoke-direct/range o.zFT.<init>(boolean, boolean, o.zFP, boolean, int, kotlin.jvm.internal.DefaultConstructorMarker):void
        0x0033: return-object v8
    }

    public static final synthetic void getNewProxyInstance(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2 p0) {
        0x0000: invoke-direct com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.finit():void
        0x0003: return-void
    }

    public static final synthetic void hDKMBd(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2 p0) {
        0x0000: invoke-direct com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.UeEOUB():void
        0x0003: return-void
    }

    public static final synthetic o.zFT isConnected(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2 p0) {
        0x0000: invoke-virtual com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.ejfBZ():o.zFT
        0x0004: return-object v0
    }

    public static final synthetic void iwGUEr(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2 p0) {
        0x0000: invoke-direct com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.fZBcTu():void
        0x0003: return-void
    }

    public static final synthetic void uzCIH(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2 p0) {
        0x0000: invoke-virtual com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.QVAiDq():void
        0x0003: return-void
    }

    public static final synthetic o.zkI valueOf(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2 p0) {
        0x0000: iget v0, v0, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.DbNXlk:o.zkI
        0x0002: return-object v0
    }

    public static final synthetic o.BaG values(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2 p0) {
        0x0000: invoke-direct com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.RJOkX():o.BaG
        0x0004: return-object v0
    }

    public static final java.lang.String wlaJM(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2 p0) {
        0x0000: invoke-virtual androidx.fragment.app.Fragment.getArguments():android.os.Bundle
        0x0006: invoke-static o.zjY.AhwBna():java.lang.String
        0x000a: invoke-virtual android.os.BaseBundle.getString(java.lang.String):java.lang.String
        0x000f: const/4 v1, 0
        0x0012: const-string v1, ""
        0x0014: return-object v1
    }

    public static final void zLjUOn(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2 p0) {
        0x0000: invoke-virtual com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.wlaJM():boolean
        0x0006: return-void
        0x0007: invoke-virtual androidx.fragment.app.Fragment.getViewLifecycleOwner():androidx.lifecycle.LifecycleOwner
        0x000b: const-string v1, ""
        0x000d: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullExpressionValue(java.lang.Object, java.lang.String):void
        0x0010: invoke-static androidx.lifecycle.LifecycleOwnerKt.getLifecycleScope(androidx.lifecycle.LifecycleOwner):androidx.lifecycle.LifecycleCoroutineScope
        0x0014: const/4 v3, 0
        0x0015: const/4 v4, 0
        0x0016: new-instance v5, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2$initView$2$1
        0x0018: const/4 v0, 0
        0x0019: invoke-direct com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2$initView$2$1.<init>(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2, kotlin.coroutines.Continuation):void
        0x001c: const/4 v6, 3
        0x001d: const/4 v7, 0
        0x001e: invoke-static/range kotlinx.coroutines.BuildersKt.launch$default(kotlinx.coroutines.CoroutineScope, kotlin.coroutines.CoroutineContext, kotlinx.coroutines.CoroutineStart, kotlin.jvm.functions.Function2, int, java.lang.Object):kotlinx.coroutines.Job
        0x0021: return-void
    }

    public void AEQbTJ() {
        0x0000: iget v0, v1, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.EZpvd:o.zFR
        0x0002: invoke-virtual o.zFR.QbewEr():void
        0x0005: return-void
    }

    public void AEQbTJ(int p0) {
        0x0000: iget v0, v1, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.EZpvd:o.zFR
        0x0002: invoke-virtual o.zFR.AEQbTJ(int):void
        0x0005: return-void
    }

    public void AEQbTJ(androidx.fragment.app.FragmentManager p0, int p1, boolean p2, kotlin.jvm.functions.Function1 p3, o.Acd p4, kotlin.jvm.functions.Function1 p5, kotlin.jvm.functions.Function1 p6, kotlin.jvm.functions.Function1 p7, kotlin.jvm.functions.Function0 p8, kotlin.jvm.functions.Function0 p9) {
        0x0000: const-string v0, ""
        0x0003: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0008: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x000d: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0012: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0017: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x001c: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0021: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0025: iget v1, v0, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.djBIcL:o.AaQ
        0x002b: invoke-virtual/range o.AaQ.AEQbTJ(androidx.fragment.app.FragmentManager, int, boolean, kotlin.jvm.functions.Function1, o.Acd, kotlin.jvm.functions.Function1, kotlin.jvm.functions.Function1, kotlin.jvm.functions.Function1, kotlin.jvm.functions.Function0, kotlin.jvm.functions.Function0):void
        0x002e: return-void
    }

    public void AEQbTJ(androidx.lifecycle.LifecycleOwner p0, o.Acd p1, kotlin.jvm.functions.Function0 p2, boolean p3, com.okinc.kline.features.drawingline.interactor.DrawingLineStorageCase p4, kotlin.jvm.functions.Function0 p5) {
        0x0000: const-string v0, ""
        0x0002: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0005: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0008: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x000b: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x000e: iget v1, v8, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.OLrzqt:o.zxJ
        0x0016: invoke-virtual/range o.zxJ.AEQbTJ(androidx.lifecycle.LifecycleOwner, o.Acd, kotlin.jvm.functions.Function0, boolean, com.okinc.kline.features.drawingline.interactor.DrawingLineStorageCase, kotlin.jvm.functions.Function0):void
        0x0019: return-void
    }

    public void AEQbTJ(com.okinc.kline.features.indicator.precomputed.PrecomputedIndicatorTransport$StateListAnimator p0) {
        0x0000: const-string v0, ""
        0x0002: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0005: iget v0, v1, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.EZpvd:o.zFR
        0x0007: invoke-virtual o.zFR.KWHzl(com.okinc.kline.features.indicator.precomputed.PrecomputedIndicatorTransport$StateListAnimator):void
        0x000a: return-void
    }

    public void AEQbTJ(com.okinc.kline.features.settings.main.data.model.SettingEntryPoint p0) {
        0x0000: const-string v0, ""
        0x0002: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0005: iget v0, v1, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.KWHzl:o.zXD
        0x0007: invoke-virtual o.zXD.copydefault(com.okinc.kline.features.settings.main.data.model.SettingEntryPoint):void
        0x000a: return-void
    }

    public void AEQbTJ(java.lang.String p0, java.lang.String p1, java.lang.String p2, java.lang.String p3, java.lang.String p4, java.lang.String p5) {
        0x0000: const-string v0, ""
        0x0002: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0005: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0008: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x000b: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x000e: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0011: iget v1, v8, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.EZpvd:o.zFR
        0x0019: invoke-virtual/range o.zFR.AEQbTJ(java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String):void
        0x001c: return-void
    }

    public void AEQbTJ(o.Acd p0, androidx.fragment.app.FragmentManager p1, kotlin.jvm.functions.Function1 p2, kotlin.jvm.functions.Function1 p3, kotlin.jvm.functions.Function1 p4, kotlin.jvm.functions.Function0 p5, kotlin.jvm.functions.Function0 p6, kotlin.jvm.functions.Function0 p7, o.adfd p8, kotlin.jvm.functions.Function1 p9) {
        0x0000: const-string v0, ""
        0x0003: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0007: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x000b: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0010: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0015: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x001a: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x001f: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0024: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0029: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x002e: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0032: iget v1, v0, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.KWHzl:o.zXD
        0x0034: invoke-virtual/range o.zXD.AEQbTJ(o.Acd, androidx.fragment.app.FragmentManager, kotlin.jvm.functions.Function1, kotlin.jvm.functions.Function1, kotlin.jvm.functions.Function1, kotlin.jvm.functions.Function0, kotlin.jvm.functions.Function0, kotlin.jvm.functions.Function0, o.adfd, kotlin.jvm.functions.Function1):void
        0x0037: return-void
    }

    public void AEQbTJ(o.Acd p0, kotlin.jvm.functions.Function0 p1, kotlin.jvm.functions.Function1 p2, kotlin.jvm.functions.Function1 p3, boolean p4) {
        0x0000: const-string v0, ""
        0x0002: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0005: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0008: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x000b: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x000e: iget v1, v7, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.AhwBna:o.zTq
        0x0015: invoke-virtual/range o.zTq.AEQbTJ(o.Acd, kotlin.jvm.functions.Function0, kotlin.jvm.functions.Function1, kotlin.jvm.functions.Function1, boolean):void
        0x0018: return-void
    }

    public void AEQbTJ(o.Acd p0, o.zOd p1, kotlin.jvm.functions.Function0 p2, o.zFT p3, kotlin.jvm.functions.Function0 p4, kotlin.jvm.functions.Function0 p5, kotlin.jvm.functions.Function0 p6, kotlin.jvm.functions.Function0 p7, kotlin.jvm.functions.Function0 p8, kotlin.jvm.functions.Function0 p9, kotlin.jvm.functions.Function0 p10, kotlin.jvm.functions.Function2 p11, kotlin.jvm.functions.Function1 p12, kotlin.jvm.functions.Function1 p13, kotlin.jvm.functions.Function1 p14, kotlin.jvm.functions.Function1 p15, kotlin.jvm.functions.Function0 p16, o.zHP p17, o.zHJ p18, o.zHR p19, o.abmc$Application p20, kotlin.jvm.functions.Function0 p21, boolean p22, kotlin.jvm.functions.Function0 p23, o.AeZ p24) {
        0x0032: const-string v0, ""
        0x0036: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x003b: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0040: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0045: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x004a: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x004f: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0054: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0059: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x005e: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0063: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0068: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x006d: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0072: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0077: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x007c: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0081: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0086: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x008b: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0090: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0095: iget v0, v1, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.EZpvd:o.zFR
        0x0099: invoke-virtual/range o.zFR.AEQbTJ(o.Acd, o.zOd, kotlin.jvm.functions.Function0, o.zFT, kotlin.jvm.functions.Function0, kotlin.jvm.functions.Function0, kotlin.jvm.functions.Function0, kotlin.jvm.functions.Function0, kotlin.jvm.functions.Function0, kotlin.jvm.functions.Function0, kotlin.jvm.functions.Function0, kotlin.jvm.functions.Function2, kotlin.jvm.functions.Function1, kotlin.jvm.functions.Function1, kotlin.jvm.functions.Function1, kotlin.jvm.functions.Function1, kotlin.jvm.functions.Function0, o.zHP, o.zHJ, o.zHR, o.abmc$Application, kotlin.jvm.functions.Function0, boolean, kotlin.jvm.functions.Function0, o.AeZ):void
        0x009c: return-void
    }

    public void AYXKKw() {
        0x0000: iget v0, v1, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.AYXKKw:o.zyt
        0x0002: invoke-virtual o.zyt.djBIcL():void
        0x0005: return-void
    }

    public void AhwBna() {
        0x0000: iget v0, v1, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.OLrzqt:o.zxJ
        0x0002: invoke-virtual o.zxJ.AhwBna():void
        0x0005: return-void
    }

    public void AkhnZx() {
        0x0000: iget v0, v1, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.KWHzl:o.zXD
        0x0002: invoke-virtual o.zXD.DbNXlk():void
        0x0005: return-void
    }

    public void AuCTel() {
        0x0000: iget v0, v1, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.djBIcL:o.AaQ
        0x0002: invoke-virtual o.AaQ.fetchVPNInfo():void
        0x0005: return-void
    }

    public boolean AuCTelauCTel() {
        0x0000: iget v0, v1, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.OLrzqt:o.zxJ
        0x0002: invoke-virtual o.zxJ.AuCTelauCTel():boolean
        0x0006: return v0
    }

    public void AubY() {
        0x0000: iget v0, v1, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.EZpvd:o.zFR
        0x0002: invoke-virtual o.zFR.AubY():void
        0x0005: return-void
    }

    public void AwvSrB() {
        0x0000: iget v0, v1, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.EZpvd:o.zFR
        0x0002: invoke-virtual o.zFR.dxcTrN():void
        0x0005: return-void
    }

    public void AxsJAY() {
        0x0000: iget v0, v1, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.AhwBna:o.zTq
        0x0002: invoke-virtual o.zTq.AYXKKw():void
        0x0005: return-void
    }

    public final void DTwDnp() {
        0x0000: const-string v0, "kline_theme_type"
        0x0006: invoke-static com.okinc.rxutils.RxBus.EZpvd(java.lang.String[]):o.acTJ
        0x000a: const-string v1, ""
        0x000c: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullExpressionValue(java.lang.Object, java.lang.String):void
        0x000f: invoke-virtual androidx.fragment.app.Fragment.getViewLifecycleOwner():androidx.lifecycle.LifecycleOwner
        0x0013: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullExpressionValue(java.lang.Object, java.lang.String):void
        0x0016: invoke-static o.acRQ.AEQbTJ(o.acTJ, androidx.lifecycle.LifecycleOwner):o.acTJ
        0x001a: new-instance v2, o.zqq
        0x001c: invoke-direct o.zqq.<init>(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2):void
        0x001f: new-instance v3, o.zqt
        0x0021: invoke-direct o.zqt.<init>(kotlin.jvm.functions.Function1):void
        0x0024: new-instance v2, o.zqw
        0x0026: invoke-direct o.zqw.<init>(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2):void
        0x0029: new-instance v4, o.zqv
        0x002b: invoke-direct o.zqv.<init>(kotlin.jvm.functions.Function1):void
        0x002e: invoke-virtual o.acTJ.EZpvd(o.acUA, o.acUA):o.acUm
        0x0031: const-string v0, "kline_trade_point_click"
        0x0037: invoke-static com.okinc.rxutils.RxBus.EZpvd(java.lang.String[]):o.acTJ
        0x003b: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullExpressionValue(java.lang.Object, java.lang.String):void
        0x003e: invoke-virtual androidx.fragment.app.Fragment.getViewLifecycleOwner():androidx.lifecycle.LifecycleOwner
        0x0042: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullExpressionValue(java.lang.Object, java.lang.String):void
        0x0045: invoke-static o.acRQ.AEQbTJ(o.acTJ, androidx.lifecycle.LifecycleOwner):o.acTJ
        0x0049: new-instance v1, o.zqz
        0x004b: invoke-direct o.zqz.<init>(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2):void
        0x004e: new-instance v2, o.zqx
        0x0050: invoke-direct o.zqx.<init>(kotlin.jvm.functions.Function1):void
        0x0053: new-instance v1, o.zqy
        0x0055: invoke-direct o.zqy.<init>(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2):void
        0x0058: new-instance v3, o.zqk
        0x005a: invoke-direct o.zqk.<init>(kotlin.jvm.functions.Function1):void
        0x005d: invoke-virtual o.acTJ.EZpvd(o.acUA, o.acUA):o.acUm
        0x0060: return-void
    }

    public void DbNXlk() {
        0x0000: iget v0, v1, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.OLrzqt:o.zxJ
        0x0002: invoke-virtual o.zxJ.DbNXlk():void
        0x0005: return-void
    }

    public void EZpvd(androidx.lifecycle.LifecycleOwner p0, o.Acd p1, o.AOA p2, o.AQt p3, androidx.fragment.app.FragmentManager p4, kotlin.jvm.functions.Function0 p5, boolean p6, boolean p7, com.okinc.kline.features.drawingline.interactor.DrawingLineStorageCase p8, kotlin.jvm.functions.Function0 p9, kotlin.jvm.functions.Function0 p10) {
        0x0000: const-string v0, ""
        0x0003: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0007: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x000c: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0011: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0016: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x001b: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0020: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0024: iget v1, v0, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.OLrzqt:o.zxJ
        0x002e: invoke-virtual/range o.zxJ.EZpvd(androidx.lifecycle.LifecycleOwner, o.Acd, o.AOA, o.AQt, androidx.fragment.app.FragmentManager, kotlin.jvm.functions.Function0, boolean, boolean, com.okinc.kline.features.drawingline.interactor.DrawingLineStorageCase, kotlin.jvm.functions.Function0, kotlin.jvm.functions.Function0):void
        0x0031: return-void
    }

    public void EZpvd(java.lang.String p0, kotlin.jvm.functions.Function2 p1, kotlin.jvm.functions.Function1 p2, kotlin.jvm.functions.Function2 p3) {
        0x0000: const-string v0, ""
        0x0002: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0005: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0008: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x000b: iget v0, v1, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.valueOf:o.zWY
        0x000d: invoke-virtual o.zWY.EZpvd(java.lang.String, kotlin.jvm.functions.Function2, kotlin.jvm.functions.Function1, kotlin.jvm.functions.Function2):void
        0x0010: return-void
    }

    public void EZpvd(o.Acd p0, androidx.fragment.app.FragmentManager p1, kotlinx.coroutines.CoroutineScope p2, kotlin.jvm.functions.Function0 p3, kotlin.jvm.functions.Function0 p4, java.lang.String p5, boolean p6, o.BaG p7, boolean p8) {
        0x0000: const-string v0, ""
        0x0003: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0007: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x000b: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x000f: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0014: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0019: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x001e: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0022: iget v1, v0, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.AYXKKw:o.zyt
        0x0028: invoke-virtual/range o.zyt.EZpvd(o.Acd, androidx.fragment.app.FragmentManager, kotlinx.coroutines.CoroutineScope, kotlin.jvm.functions.Function0, kotlin.jvm.functions.Function0, java.lang.String, boolean, o.BaG, boolean):void
        0x002b: return-void
    }

    public void EZpvd(o.zOb$TaskDescription p0) {
        0x0000: const-string v0, ""
        0x0002: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0005: iget v0, v1, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.EZpvd:o.zFR
        0x0007: invoke-virtual o.zFR.KWHzl(o.zOb$TaskDescription):void
        0x000a: return-void
    }

    public final void KWHzl() {
        0x0000: iget v0, v2, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.isConnected:int
        0x0005: invoke-direct com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.QUSxYX():java.lang.String
        0x0009: invoke-direct com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.djSkpj():java.lang.String
        0x000d: invoke-static o.zFJ.EZpvd(java.lang.String, java.lang.String):java.lang.Integer
        0x0013: invoke-virtual java.lang.Number.intValue():int
        0x0017: iput v0, v2, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.isConnected:int
        0x0019: iget v1, v2, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.values:com.okinc.kline.library.data.ChartViewOutSideConfig
        0x001d: invoke-virtual com.okinc.kline.library.data.ChartViewOutSideConfig.setMPriceFractionDigits(int):void
        0x0020: iget v1, v2, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.DbNXlk:o.zkI
        0x0024: const-string v1, ""
        0x0026: invoke-static kotlin.jvm.internal.Intrinsics.valueOf(java.lang.String):void
        0x0029: const/4 v1, 0
        0x002a: iget v1, v1, o.zkI.KWHzl:o.Acd
        0x002c: invoke-virtual o.Acd.setPriceFractionDigits(int):void
        0x002f: return-void
    }

    public void KWHzl(androidx.lifecycle.LifecycleOwner p0, o.Acd p1, androidx.fragment.app.FragmentManager p2, boolean p3, o.ABT p4, kotlin.jvm.functions.Function0 p5, kotlin.jvm.functions.Function0 p6, kotlin.jvm.functions.Function0 p7, boolean p8, kotlin.jvm.functions.Function0 p9, o.ajnU p10, o.aAaV p11) {
        0x0000: const-string v0, ""
        0x0003: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0008: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x000d: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0012: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0017: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x001c: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0021: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0025: iget v1, v0, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.AEQbTJ:o.zCf
        0x0031: invoke-virtual/range o.zCf.KWHzl(androidx.lifecycle.LifecycleOwner, o.Acd, androidx.fragment.app.FragmentManager, boolean, o.ABT, kotlin.jvm.functions.Function0, kotlin.jvm.functions.Function0, kotlin.jvm.functions.Function0, boolean, kotlin.jvm.functions.Function0, o.ajnU, o.aAaV):void
        0x0034: return-void
    }

    public void KWHzl(o.zOb$StateListAnimator p0) {
        0x0000: const-string v0, ""
        0x0002: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0005: iget v0, v1, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.EZpvd:o.zFR
        0x0007: invoke-virtual o.zFR.OLrzqt(o.zOb$StateListAnimator):void
        0x000a: return-void
    }

    public final void KWHzl(boolean p0) {
        0x0000: iget v0, v2, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.fIwbmz:boolean
        0x0004: return-void
        0x0005: const/4 v0, 1
        0x0006: iput v0, v2, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.fIwbmz:boolean
        0x0008: const/4 v0, 2
        0x0009: const/4 v1, 0
        0x000a: invoke-static o.Fzc.reportFullyDrawn$default(androidx.fragment.app.Fragment, boolean, java.lang.String, int, java.lang.Object):void
        0x000d: return-void
    }

    public final com.okinc.kline.library.data.ChartViewOutSideConfig OLrzqt() {
        0x0002: new-instance v15, com.okinc.kline.library.data.ChartViewOutSideConfig
        0x0005: const/4 v2, 0
        0x0006: const/4 v3, 0
        0x0007: const/4 v4, 0
        0x0008: const/4 v5, 0
        0x0009: const/4 v6, 0
        0x000a: const/4 v7, 0
        0x000b: const/4 v8, 0
        0x000c: const/4 v9, 0
        0x000d: const/4 v10, 0
        0x000e: const/4 v11, 0
        0x000f: const/4 v12, 0
        0x0010: const/4 v13, 0
        0x0011: const/4 v14, 0
        0x0012: const/16 v16, 0
        0x0018: const-wide/16 v17, 0
        0x001a: const/16 v19, 0
        0x001c: const/16 v20, 0
        0x001e: const/16 v21, 0
        0x0020: const/16 v22, 0
        0x0022: const/16 v23, 0
        0x0024: const/16 v24, 0
        0x0026: const/16 v25, 0
        0x0028: const/16 v26, 0
        0x002a: const/16 v27, 0
        0x002c: const/16 v28, 0
        0x002e: const/16 v29, 0
        0x0030: const/16 v30, 0
        0x0032: const/16 v31, 0
        0x0034: const/16 v32, 0
        0x0036: const/16 v33, 0
        0x0038: const/16 v34, 0
        0x003a: const/16 v35, 0
        0x003c: const/16 v36, 0
        0x003e: const/16 v37, 0
        0x0040: const/16 v38, 0
        0x0042: const/16 v39, 0
        0x0044: const/16 v40, 0
        0x0046: const/16 v41, 0
        0x0048: const/16 v42, 0
        0x004a: const/16 v43, 0
        0x004c: const/16 v44, 0
        0x004e: const/16 v45, 0
        0x0050: const/16 v46, 0
        0x0052: const/16 v47, 0
        0x0054: const/16 v48, 0
        0x0056: const/16 v49, 0
        0x0058: const/16 v50, 0
        0x005a: const/16 v51, 0
        0x005c: const/16 v52, 0
        0x005e: const/16 v53, 0
        0x0060: const/16 v54, 0
        0x0062: const/16 v55, 0
        0x0064: const/16 v56, 0
        0x0066: const/16 v57, 0
        0x0068: const/16 v58, 0
        0x006a: const/16 v59, 0
        0x006c: const/16 v60, 0
        0x006e: const/16 v61, 0
        0x0070: const/16 v62, 0
        0x0072: const/16 v63, 0
        0x0074: const/16 v64, 0
        0x0076: const/16 v65, 0
        0x0078: const/16 v66, 0
        0x007a: const/16 v67, 0
        0x007c: const/16 v68, 0
        0x007e: const/16 v69, 0
        0x0080: const/16 v70, 0
        0x0082: const-wide/16 v71, 0
        0x0084: const/16 v73, 0
        0x0086: const/16 v74, 0
        0x0088: const/16 v75, 0
        0x008a: const/16 v76, 0
        0x008c: const/16 v77, 0
        0x008e: const/16 v78, 0
        0x0090: const/16 v79, 0
        0x0092: const/16 v80, 0
        0x0094: const/16 v81, 0
        0x0096: const/16 v82, 0
        0x0098: const/16 v83, 0
        0x009a: const/16 v84, 0
        0x009c: const/16 v85, 0
        0x009e: const/16 v86, 0
        0x00a0: const/16 v87, 0
        0x00a2: const/16 v88, 0
        0x00a4: const/16 v89, 0
        0x00a6: const/16 v90, 0
        0x00a8: const/16 v91, 0
        0x00aa: const/16 v92, 0
        0x00ac: const/16 v93, 0
        0x00ae: const/16 v94, 0
        0x00b0: const/16 v95, 0
        0x00b2: const/16 v96, 0
        0x00b4: const/16 v97, 0
        0x00b6: const/16 v98, 0
        0x00b8: const/16 v99, 0
        0x00ba: const/16 v100, 0
        0x00bc: const/16 v101, 0
        0x00be: const/16 v102, 0
        0x00c0: const/16 v103, 0
        0x00c2: const/16 v104, 0
        0x00c4: const/16 v105, 0
        0x00c6: const/16 v106, 0
        0x00c8: const/16 v107, 0
        0x00ca: const/16 v108, 0
        0x00cc: const/16 v109, 0
        0x00ce: const/16 v110, 0
        0x00d0: const/16 v111, 0
        0x00d2: const/16 v112, 0
        0x00d4: const/16 v113, 0
        0x00d6: const/16 v114, 0
        0x00d8: const/16 v115, -1
        0x00da: const/16 v116, -1
        0x00dc: const/16 v117, -1
        0x00de: const/16 v118, 32767
        0x00e0: const/16 v119, 0
        0x00e2: invoke-direct/range com.okinc.kline.library.data.ChartViewOutSideConfig.<init>(boolean, boolean, boolean, boolean, boolean, java.util.ArrayList, com.okinc.kline.library.drawline.DrawLineData$DrawLineItem, o.Adn, java.util.List, java.util.List, java.util.List, boolean, boolean, boolean, boolean, long, boolean, float, float, float, float, int, int, android.graphics.RectF, boolean, android.graphics.RectF, boolean, java.util.List, java.util.List, java.util.List, java.util.List, com.okinc.kline.data.KlineQuickOrderPriceBean, boolean, java.lang.String, java.lang.String, java.lang.String, boolean, boolean, java.lang.String, int, int, com.okinc.kline.data.BotPendingLineVo, o.AHi, com.okinc.kline.data.BotPendingLineVo, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, int, boolean, int, java.lang.String, java.lang.String, java.lang.String, java.lang.String, boolean, boolean, boolean, boolean, boolean, boolean, o.AcY, o.Acp, boolean, boolean, boolean, boolean, double, int, int, java.lang.String, boolean, java.lang.String, java.lang.String, boolean, java.util.HashMap, java.util.HashMap, java.util.HashMap, java.util.LinkedHashMap, java.util.HashMap, java.util.HashMap, boolean, boolean, java.lang.String, com.okinc.kline.data.KlineSpotAverageBean, boolean, boolean, boolean, boolean, boolean, boolean, int, int, boolean, boolean, java.lang.Integer, java.lang.Integer, java.lang.Integer, java.lang.Integer, java.lang.Boolean, java.lang.Boolean, java.lang.Boolean, java.lang.Boolean, java.lang.Boolean, java.lang.Boolean, java.lang.Boolean, java.lang.Boolean, java.lang.Boolean, java.lang.Boolean, boolean, int, int, int, int, kotlin.jvm.internal.DefaultConstructorMarker):void
        0x00e5: const-string v1, "sp_timeinterval_okx_pro"
        0x00e9: invoke-virtual com.okinc.kline.library.data.ChartViewOutSideConfig.setChartTag(java.lang.String):void
        0x00ec: const-string v1, "pro_mode"
        0x00ee: invoke-virtual com.okinc.kline.library.data.ChartViewOutSideConfig.setMode(java.lang.String):void
        0x00f1: const-string v1, "NORMAL"
        0x00f3: invoke-virtual com.okinc.kline.library.data.ChartViewOutSideConfig.setCurrentMode(java.lang.String):void
        0x00f6: const/4 v1, 0
        0x00f7: invoke-virtual com.okinc.kline.library.data.ChartViewOutSideConfig.setScreen(boolean):void
        0x00fa: const/4 v3, 1
        0x00fb: invoke-virtual com.okinc.kline.library.data.ChartViewOutSideConfig.setPortraitChart(boolean):void
        0x00fe: invoke-virtual com.okinc.kline.library.data.ChartViewOutSideConfig.setBotKLine(boolean):void
        0x0101: invoke-virtual com.okinc.kline.library.data.ChartViewOutSideConfig.setChartMode(int):void
        0x0104: invoke-virtual com.okinc.kline.library.data.ChartViewOutSideConfig.setFullScreenMode(boolean):void
        0x0107: invoke-virtual com.okinc.kline.library.data.ChartViewOutSideConfig.setMChartHistoryOrder(boolean):void
        0x010a: invoke-virtual/range com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.fJNWhG():com.okinc.kline.ui.bot.KlineBotViewModel
        0x010e: invoke-direct/range com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.aKErDB():long
        0x0112: invoke-direct/range com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.QUSxYX():java.lang.String
        0x0116: invoke-virtual com.okinc.kline.ui.bot.KlineBotViewModel.OLrzqt(long, java.lang.String):o.Aww
        0x011a: const/4 v4, 0
        0x011d: invoke-virtual o.Aww.EZpvd():com.okinc.kline.data.BotPendingLineVo
        0x0123: invoke-virtual com.okinc.kline.library.data.ChartViewOutSideConfig.setBotOrdersBean(com.okinc.kline.data.BotPendingLineVo):void
        0x0128: invoke-virtual o.Aww.OLrzqt():o.AHi
        0x012e: invoke-virtual com.okinc.kline.library.data.ChartViewOutSideConfig.setGridPriceData(o.AHi):void
        0x0133: invoke-virtual o.Aww.KWHzl():com.okinc.kline.data.BotPendingLineVo
        0x0137: invoke-virtual com.okinc.kline.library.data.ChartViewOutSideConfig.setGridPendingData(com.okinc.kline.data.BotPendingLineVo):void
        0x013a: invoke-direct/range com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.QUSxYX():java.lang.String
        0x013e: invoke-direct/range com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.djSkpj():java.lang.String
        0x0142: invoke-static o.zFJ.EZpvd(java.lang.String, java.lang.String):java.lang.Integer
        0x0148: invoke-virtual java.lang.Number.intValue():int
        0x014c: invoke-virtual com.okinc.kline.library.data.ChartViewOutSideConfig.setMPriceFractionDigits(int):void
        0x014f: iput v3, v0, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.isConnected:int
        0x0151: sget v3, o.AYO.EZpvd:o.AYO
        0x0153: invoke-direct/range com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.QUSxYX():java.lang.String
        0x0157: invoke-virtual o.AYO.EZpvd(java.lang.String):boolean
        0x015b: invoke-direct/range com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.djSkpj():java.lang.String
        0x015f: invoke-static o.AZh.copydefault(boolean, boolean, java.lang.String):java.lang.String
        0x0163: invoke-virtual com.okinc.kline.library.data.ChartViewOutSideConfig.setTradFiType(java.lang.String):void
        0x0166: iput v2, v0, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.values:com.okinc.kline.library.data.ChartViewOutSideConfig
        0x0168: return-object v2
    }

    public void OLrzqt(int p0) {
        0x0000: iget v0, v1, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.djBIcL:o.AaQ
        0x0002: invoke-virtual o.AaQ.KWHzl(int):void
        0x0005: return-void
    }

    public void OLrzqt(java.lang.String p0, java.lang.String p1) {
        0x0000: invoke-static o.AGS$StateListAnimator.KWHzl(o.AGS, java.lang.String, java.lang.String):void
        0x0003: return-void
    }

    public void OLrzqt(o.zOb$Application p0) {
        0x0000: const-string v0, ""
        0x0002: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0005: iget v0, v1, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.EZpvd:o.zFR
        0x0007: invoke-virtual o.zFR.copydefault(o.zOb$Application):void
        0x000a: return-void
    }

    public void OLrzqt(boolean p0) {
        0x0000: iget v0, v1, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.OLrzqt:o.zxJ
        0x0002: invoke-virtual o.zxJ.OLrzqt(boolean):void
        0x0005: return-void
    }

    public void ORxRYg() {
        0x0000: iget v0, v1, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.EZpvd:o.zFR
        0x0002: invoke-virtual o.zFR.ORxRYg():void
        0x0005: return-void
    }

    public void OcIXYQ() {
        0x0000: new-instance v0, o.zqr
        0x0002: invoke-direct o.zqr.<init>():void
        0x0005: invoke-virtual com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.copydefault(kotlin.jvm.functions.Function1):void
        0x0008: return-void
    }

    public java.lang.String QKVWgx() {
        0x0000: iget v0, v1, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.EZpvd:o.zFR
        0x0002: invoke-virtual o.zFR.finit():java.lang.String
        0x0006: return-object v0
    }

    public void QOLQEE() {
        0x0000: invoke-virtual/range com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.wlaJM():boolean
        0x0006: invoke-virtual/range androidx.fragment.app.Fragment.getChildFragmentManager():androidx.fragment.app.FragmentManager
        0x000a: invoke-virtual androidx.fragment.app.FragmentManager.isStateSaved():boolean
        0x0011: sget v2, com.okinc.kline.ui.bean.EntryType.PRO_BOT:com.okinc.kline.ui.bean.EntryType
        0x0013: invoke-direct/range com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.QUSxYX():java.lang.String
        0x0017: invoke-direct/range com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.djSkpj():java.lang.String
        0x001b: new-instance v0, com.okinc.kline.features.settings.main.data.model.SettingEntryPoint
        0x001e: const/4 v5, 0
        0x001f: const/4 v6, 0
        0x0020: const/4 v7, 0
        0x0021: const/4 v8, 0
        0x0022: const/4 v9, 0
        0x0023: const/4 v10, 0
        0x0024: const/4 v11, 0
        0x0025: const/4 v12, 0
        0x0026: const/4 v13, 0
        0x0027: const/4 v14, 0
        0x0028: const/4 v15, 0
        0x0029: const/16 v16, 0
        0x002b: const/16 v17, 0
        0x002d: const/16 v18, 0
        0x002f: const/16 v19, 0
        0x0031: const/16 v20, 0
        0x0033: const v21, 524280
        0x0036: const/16 v22, 0
        0x0038: invoke-direct/range com.okinc.kline.features.settings.main.data.model.SettingEntryPoint.<init>(com.okinc.kline.ui.bean.EntryType, java.lang.String, java.lang.String, java.lang.String, boolean, boolean, boolean, boolean, boolean, boolean, boolean, boolean, boolean, boolean, boolean, boolean, boolean, boolean, java.lang.Boolean, int, kotlin.jvm.internal.DefaultConstructorMarker):void
        0x003d: invoke-virtual com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.AEQbTJ(com.okinc.kline.features.settings.main.data.model.SettingEntryPoint):void
        0x0043: return-void
    }

    public final void QVAiDq() {
        0x0000: iget v0, v6, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.DbNXlk:o.zkI
        0x0004: return-void
        0x0005: const/4 v1, 0
        0x0006: const-string v2, ""
        0x000a: invoke-static kotlin.jvm.internal.Intrinsics.valueOf(java.lang.String):void
        0x000e: iget v0, v0, o.zkI.copydefault:o.abdw
        0x0010: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullExpressionValue(java.lang.Object, java.lang.String):void
        0x0013: iget v3, v6, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.DbNXlk:o.zkI
        0x0017: invoke-static kotlin.jvm.internal.Intrinsics.valueOf(java.lang.String):void
        0x001b: iget v3, v3, o.zkI.KWHzl:o.Acd
        0x001d: iget v4, v6, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.gEmmrt:int
        0x001f: iget v5, v6, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.DbNXlk:o.zkI
        0x0023: invoke-static kotlin.jvm.internal.Intrinsics.valueOf(java.lang.String):void
        0x0028: iget v1, v1, o.zkI.KWHzl:o.Acd
        0x002a: invoke-virtual o.Acd.getProfitSideAreaHeight():int
        0x002e: invoke-static java.lang.Integer.valueOf(int):java.lang.Integer
        0x0032: invoke-static java.lang.Integer.valueOf(int):java.lang.Integer
        0x0036: invoke-static o.AXn.EZpvd(o.abdw, o.Acd, java.lang.Integer, java.lang.Integer):void
        0x0039: return-void
    }

    public kotlinx.coroutines.Job collectLatestOnLifecycle(o.PVO p0, androidx.lifecycle.Lifecycle$State p1, kotlin.jvm.functions.Function2 p2) {
        0x0000: invoke-static o.PVP$Application.EZpvd(o.PVP, o.PVO, androidx.lifecycle.Lifecycle$State, kotlin.jvm.functions.Function2):kotlinx.coroutines.Job
        0x0004: return-object v1
    }

    public kotlinx.coroutines.Job collectLatestOnLifecycle(o.PVO p0, o.adgO p1, androidx.lifecycle.Lifecycle$State p2, kotlin.jvm.functions.Function2 p3) {
        0x0000: invoke-static o.PVP$Application.OLrzqt(o.PVP, o.PVO, o.adgO, androidx.lifecycle.Lifecycle$State, kotlin.jvm.functions.Function2):kotlinx.coroutines.Job
        0x0004: return-object v1
    }

    public kotlinx.coroutines.Job collectLatestOnLifecycle(o.PVO p0, o.adgO p1, o.adgO p2, androidx.lifecycle.Lifecycle$State p3, o.adfd p4) {
        0x0000: invoke-static/range o.PVP$Application.AEQbTJ(o.PVP, o.PVO, o.adgO, o.adgO, androidx.lifecycle.Lifecycle$State, o.adfd):kotlinx.coroutines.Job
        0x0004: return-object v1
    }

    public kotlinx.coroutines.Job collectLatestOnLifecycle(o.PVO p0, o.adgO p1, o.adgO p2, o.adgO p3, androidx.lifecycle.Lifecycle$State p4, o.adfe p5) {
        0x0000: invoke-static/range o.PVP$Application.copydefault(o.PVP, o.PVO, o.adgO, o.adgO, o.adgO, androidx.lifecycle.Lifecycle$State, o.adfe):kotlinx.coroutines.Job
        0x0004: return-object v1
    }

    public kotlinx.coroutines.Job collectLatestOnLifecycle(o.PVO p0, o.adgO p1, o.adgO p2, o.adgO p3, o.adgO p4, androidx.lifecycle.Lifecycle$State p5, o.adfj p6) {
        0x0000: invoke-static/range o.PVP$Application.KWHzl(o.PVP, o.PVO, o.adgO, o.adgO, o.adgO, o.adgO, androidx.lifecycle.Lifecycle$State, o.adfj):kotlinx.coroutines.Job
        0x0004: return-object v1
    }

    public kotlinx.coroutines.Job collectLatestOnLifecycle(o.PVO p0, o.adgO p1, o.adgO p2, o.adgO p3, o.adgO p4, o.adgO p5, androidx.lifecycle.Lifecycle$State p6, o.adfg p7) {
        0x0000: invoke-static/range o.PVP$Application.AEQbTJ(o.PVP, o.PVO, o.adgO, o.adgO, o.adgO, o.adgO, o.adgO, androidx.lifecycle.Lifecycle$State, o.adfg):kotlinx.coroutines.Job
        0x0004: return-object v1
    }

    public kotlinx.coroutines.Job collectLatestOnLifecycle(o.PVO p0, o.adgO p1, o.adgO p2, o.adgO p3, o.adgO p4, o.adgO p5, o.adgO p6, androidx.lifecycle.Lifecycle$State p7, o.adfh p8) {
        0x0000: invoke-static/range o.PVP$Application.copydefault(o.PVP, o.PVO, o.adgO, o.adgO, o.adgO, o.adgO, o.adgO, o.adgO, androidx.lifecycle.Lifecycle$State, o.adfh):kotlinx.coroutines.Job
        0x0004: return-object v1
    }

    public kotlinx.coroutines.Job collectLatestWhenLifecycle(o.PVO p0, androidx.lifecycle.Lifecycle$State p1, kotlin.jvm.functions.Function2 p2) {
        0x0000: invoke-static o.PVP$Application.OLrzqt(o.PVP, o.PVO, androidx.lifecycle.Lifecycle$State, kotlin.jvm.functions.Function2):kotlinx.coroutines.Job
        0x0004: return-object v1
    }

    public kotlinx.coroutines.Job collectLatestWhenLifecycle(o.PVO p0, o.adgO p1, androidx.lifecycle.Lifecycle$State p2, kotlin.jvm.functions.Function2 p3) {
        0x0000: invoke-static o.PVP$Application.EZpvd(o.PVP, o.PVO, o.adgO, androidx.lifecycle.Lifecycle$State, kotlin.jvm.functions.Function2):kotlinx.coroutines.Job
        0x0004: return-object v1
    }

    public kotlinx.coroutines.Job collectLatestWhenLifecycle(o.PVO p0, o.adgO p1, o.adgO p2, androidx.lifecycle.Lifecycle$State p3, o.adfd p4) {
        0x0000: invoke-static/range o.PVP$Application.EZpvd(o.PVP, o.PVO, o.adgO, o.adgO, androidx.lifecycle.Lifecycle$State, o.adfd):kotlinx.coroutines.Job
        0x0004: return-object v1
    }

    public kotlinx.coroutines.Job collectLatestWhenLifecycle(o.PVO p0, o.adgO p1, o.adgO p2, o.adgO p3, androidx.lifecycle.Lifecycle$State p4, o.adfe p5) {
        0x0000: invoke-static/range o.PVP$Application.AEQbTJ(o.PVP, o.PVO, o.adgO, o.adgO, o.adgO, androidx.lifecycle.Lifecycle$State, o.adfe):kotlinx.coroutines.Job
        0x0004: return-object v1
    }

    public kotlinx.coroutines.Job collectLatestWhenLifecycle(o.PVO p0, o.adgO p1, o.adgO p2, o.adgO p3, o.adgO p4, androidx.lifecycle.Lifecycle$State p5, o.adfj p6) {
        0x0000: invoke-static/range o.PVP$Application.copydefault(o.PVP, o.PVO, o.adgO, o.adgO, o.adgO, o.adgO, androidx.lifecycle.Lifecycle$State, o.adfj):kotlinx.coroutines.Job
        0x0004: return-object v1
    }

    public kotlinx.coroutines.Job collectLatestWhenLifecycle(o.PVO p0, o.adgO p1, o.adgO p2, o.adgO p3, o.adgO p4, o.adgO p5, androidx.lifecycle.Lifecycle$State p6, o.adfg p7) {
        0x0000: invoke-static/range o.PVP$Application.EZpvd(o.PVP, o.PVO, o.adgO, o.adgO, o.adgO, o.adgO, o.adgO, androidx.lifecycle.Lifecycle$State, o.adfg):kotlinx.coroutines.Job
        0x0004: return-object v1
    }

    public kotlinx.coroutines.Job collectLatestWhenLifecycle(o.PVO p0, o.adgO p1, o.adgO p2, o.adgO p3, o.adgO p4, o.adgO p5, o.adgO p6, androidx.lifecycle.Lifecycle$State p7, o.adfh p8) {
        0x0000: invoke-static/range o.PVP$Application.EZpvd(o.PVP, o.PVO, o.adgO, o.adgO, o.adgO, o.adgO, o.adgO, o.adgO, androidx.lifecycle.Lifecycle$State, o.adfh):kotlinx.coroutines.Job
        0x0004: return-object v1
    }

    public void copydefault(androidx.fragment.app.FragmentManager p0, int p1, boolean p2, kotlin.jvm.functions.Function1 p3, kotlin.jvm.functions.Function0 p4, kotlin.jvm.functions.Function1 p5, kotlin.jvm.functions.Function1 p6, kotlin.jvm.functions.Function0 p7, kotlin.jvm.functions.Function0 p8) {
        0x0000: const-string v0, ""
        0x0003: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0007: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x000c: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0011: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0016: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x001b: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x001f: iget v1, v0, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.djBIcL:o.AaQ
        0x0025: invoke-virtual/range o.AaQ.copydefault(androidx.fragment.app.FragmentManager, int, boolean, kotlin.jvm.functions.Function1, kotlin.jvm.functions.Function0, kotlin.jvm.functions.Function1, kotlin.jvm.functions.Function1, kotlin.jvm.functions.Function0, kotlin.jvm.functions.Function0):void
        0x0028: return-void
    }

    public void copydefault(com.okinc.kline.ui.bean.IBotOrdersDataBean p0) {
        0x0000: const-string v0, ""
        0x0002: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0005: invoke-virtual androidx.fragment.app.Fragment.isAdded():boolean
        0x000b: return-void
        0x000c: invoke-static androidx.lifecycle.LifecycleOwnerKt.getLifecycleScope(androidx.lifecycle.LifecycleOwner):androidx.lifecycle.LifecycleCoroutineScope
        0x0010: const/4 v2, 0
        0x0011: const/4 v3, 0
        0x0012: new-instance v4, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2$setBotOrdersData$1
        0x0014: const/4 v0, 0
        0x0015: invoke-direct com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2$setBotOrdersData$1.<init>(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2, com.okinc.kline.ui.bean.IBotOrdersDataBean, kotlin.coroutines.Continuation):void
        0x0018: const/4 v5, 3
        0x0019: const/4 v6, 0
        0x001a: invoke-static/range kotlinx.coroutines.BuildersKt.launch$default(kotlinx.coroutines.CoroutineScope, kotlin.coroutines.CoroutineContext, kotlinx.coroutines.CoroutineStart, kotlin.jvm.functions.Function2, int, java.lang.Object):kotlinx.coroutines.Job
        0x001d: return-void
    }

    public void copydefault(com.okinc.tradingbot.api.model.PendingOrderLine p0) {
        0x0000: invoke-static o.AGS$StateListAnimator.EZpvd(o.AGS, com.okinc.tradingbot.api.model.PendingOrderLine):void
        0x0003: return-void
    }

    public void copydefault(java.util.List p0) {
        0x0000: iget v0, v1, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.AYXKKw:o.zyt
        0x0002: invoke-virtual o.zyt.KWHzl(java.util.List):void
        0x0005: return-void
    }

    public void copydefault(kotlin.jvm.functions.Function1 p0) {
        0x0000: const-string v0, ""
        0x0002: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0005: iget v0, v1, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.AEQbTJ:o.zCf
        0x0007: invoke-virtual o.zCf.copydefault(kotlin.jvm.functions.Function1):void
        0x000a: return-void
    }

    public void copydefault(boolean p0) {
        0x0000: iget v0, v1, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.OLrzqt:o.zxJ
        0x0002: invoke-virtual o.zxJ.copydefault(boolean):void
        0x0005: return-void
    }

    public void djBIcL() {
        0x0000: iget v0, v1, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.AEQbTJ:o.zCf
        0x0002: invoke-virtual o.zCf.DbNXlk():void
        0x0005: return-void
    }

    public final o.zFT ejfBZ() {
        0x0000: iget v0, v1, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.AkhnZx:o.adaP
        0x0002: invoke-interface o.adaP.getValue():java.lang.Object
        0x0006: check-cast v0, o.zFT
        0x0008: return-object v0
    }

    public void fARcdN() {
        0x0000: iget v0, v1, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.KWHzl:o.zXD
        0x0002: invoke-virtual o.zXD.fetchVPNInfo():void
        0x0005: return-void
    }

    public void fIwbmz() {
        0x0000: iget v0, v1, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.EZpvd:o.zFR
        0x0002: invoke-virtual o.zFR.dNCPSb():void
        0x0005: return-void
    }

    public final com.okinc.kline.ui.bot.KlineBotViewModel fJNWhG() {
        0x0000: iget v0, v1, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.fetchVPNInfo:o.adaP
        0x0002: invoke-interface o.adaP.getValue():java.lang.Object
        0x0006: check-cast v0, com.okinc.kline.ui.bot.KlineBotViewModel
        0x0008: return-object v0
    }

    public void fetchVPNInfo() {
        0x0000: iget v0, v1, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.OLrzqt:o.zxJ
        0x0002: invoke-virtual o.zxJ.fetchVPNInfo():void
        0x0005: return-void
    }

    public void gEmmrt() {
        0x0000: iget v0, v1, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.EZpvd:o.zFR
        0x0002: invoke-virtual o.zFR.RJOkX():void
        0x0005: return-void
    }

    public void gHZMYf() {
        0x0000: iget v0, v1, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.AEQbTJ:o.zCf
        0x0002: invoke-virtual o.zCf.fetchVPNInfo():void
        0x0005: return-void
    }

    public final o.zOd getFieldNames() {
        0x0000: iget v0, v1, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.uzCIH:o.adaP
        0x0002: invoke-interface o.adaP.getValue():java.lang.Object
        0x0006: check-cast v0, o.zOd
        0x0008: return-object v0
    }

    public int getLayoutId() {
        0x0000: sget v0, o.zih$ActionBar.uzCIH:int
        0x0002: return v0
    }

    public void getNewProxyInstance() {
        0x0000: iget v0, v1, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.EZpvd:o.zFR
        0x0002: invoke-virtual o.zFR.djSkpj():void
        0x0005: return-void
    }

    public java.lang.Object getUiState(o.PVO p0, kotlin.coroutines.Continuation p1) {
        0x0000: invoke-static o.PVP$Application.OLrzqt(o.PVP, o.PVO, kotlin.coroutines.Continuation):java.lang.Object
        0x0004: return-object v1
    }

    public void hDKMBd() {
        0x0000: iget v0, v1, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.OLrzqt:o.zxJ
        0x0002: invoke-virtual o.zxJ.hDKMBd():void
        0x0005: return-void
    }

    public void initView(android.view.View p0, android.os.Bundle p1) {
        0x0000: const-string v5, ""
        0x0002: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0005: sget v0, com.okinc.kline.util.KlineUIManager.copydefault:com.okinc.kline.util.KlineUIManager
        0x0007: sget v1, o.ATW.EZpvd:o.ATW
        0x0009: invoke-virtual o.ATW.copydefault():java.lang.String
        0x000d: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullExpressionValue(java.lang.Object, java.lang.String):void
        0x0010: invoke-virtual com.okinc.kline.util.KlineUIManager.AEQbTJ(java.lang.String, o.AHh):void
        0x0013: invoke-virtual o.ATW.AYXKKw():void
        0x0016: invoke-static o.zkI.KWHzl(android.view.View):o.zkI
        0x001a: iput v0, v3, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.DbNXlk:o.zkI
        0x001c: invoke-virtual com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.iwGUEr():void
        0x001f: invoke-virtual com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.DTwDnp():void
        0x0022: iget v0, v3, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.DbNXlk:o.zkI
        0x0026: invoke-static kotlin.jvm.internal.Intrinsics.valueOf(java.lang.String):void
        0x0029: const/4 v0, 0
        0x002a: iget v5, v0, o.zkI.AEQbTJ:o.abeF
        0x002c: new-instance v0, o.zqg
        0x002e: invoke-direct o.zqg.<init>(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2):void
        0x0031: invoke-virtual o.abeF.setRetryClickListener(android.view.View$OnClickListener):void
        0x0034: new-instance v5, o.zqj
        0x0036: invoke-direct o.zqj.<init>(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2):void
        0x0039: invoke-virtual android.view.View.post(java.lang.Runnable):boolean
        0x003c: return-void
    }

    public void isConnected() {
        0x0000: iget v0, v1, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.valueOf:o.zWY
        0x0002: invoke-virtual o.zWY.AEQbTJ():void
        0x0005: return-void
    }

    public final void iwGUEr() {
        0x0000: iget v0, v14, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.DbNXlk:o.zkI
        0x0002: const-string v1, ""
        0x0004: const/4 v2, 0
        0x0007: invoke-static kotlin.jvm.internal.Intrinsics.valueOf(java.lang.String):void
        0x000b: iget v0, v0, o.zkI.djBIcL:o.APO
        0x000d: new-instance v3, o.zql
        0x000f: invoke-direct o.zql.<init>(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2):void
        0x0012: invoke-virtual o.APO.setOnItemSelectListener(kotlin.jvm.functions.Function1):void
        0x0015: invoke-virtual androidx.fragment.app.Fragment.getChildFragmentManager():androidx.fragment.app.FragmentManager
        0x0019: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullExpressionValue(java.lang.Object, java.lang.String):void
        0x001c: iget v0, v14, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.DbNXlk:o.zkI
        0x0020: invoke-static kotlin.jvm.internal.Intrinsics.valueOf(java.lang.String):void
        0x0024: iget v5, v0, o.zkI.KWHzl:o.Acd
        0x0026: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullExpressionValue(java.lang.Object, java.lang.String):void
        0x0029: iget v0, v14, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.DbNXlk:o.zkI
        0x002d: invoke-static kotlin.jvm.internal.Intrinsics.valueOf(java.lang.String):void
        0x0032: new-instance v6, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2$initTimeTitleBar$2
        0x0034: iget v0, v2, o.zkI.djBIcL:o.APO
        0x0036: invoke-direct com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2$initTimeTitleBar$2.<init>(java.lang.Object):void
        0x0039: const/4 v2, 4
        0x003a: const/4 v4, 1
        0x003b: const/4 v7, 0
        0x003c: new-instance v8, o.zqp
        0x003e: invoke-direct o.zqp.<init>(com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2):void
        0x0041: new-instance v9, o.zqs
        0x0043: invoke-direct o.zqs.<init>():void
        0x0046: new-instance v10, o.zqu
        0x0048: invoke-direct o.zqu.<init>():void
        0x004b: const/4 v11, 0
        0x004c: const/16 v12, 520
        0x004e: const/4 v13, 0
        0x0059: invoke-static/range o.Abg$ActionBar.attachTimePeriod$default(o.Abg, androidx.fragment.app.FragmentManager, int, boolean, kotlin.jvm.functions.Function1, o.Acd, kotlin.jvm.functions.Function1, kotlin.jvm.functions.Function1, kotlin.jvm.functions.Function1, kotlin.jvm.functions.Function0, kotlin.jvm.functions.Function0, int, java.lang.Object):void
        0x005c: return-void
    }

    public void onDestroyView() {
        0x0000: iget v0, v2, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.DbNXlk:o.zkI
        0x0004: const/4 v1, 0
        0x0007: const-string v0, ""
        0x0009: invoke-static kotlin.jvm.internal.Intrinsics.valueOf(java.lang.String):void
        0x000d: iget v0, v0, o.zkI.KWHzl:o.Acd
        0x000f: invoke-virtual o.Acd.setKlineEventClickListener(o.Acd$FragmentManager):void
        0x0012: invoke-virtual o.Acd.copydefault():void
        0x0015: invoke-virtual o.Acd.djSkpj():void
        0x0018: invoke-virtual o.Acd.values():void
        0x001b: invoke-virtual com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.djBIcL():void
        0x001e: invoke-virtual com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.fetchVPNInfo():void
        0x0021: invoke-virtual com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.AkhnZx():void
        0x0024: invoke-virtual com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.AYXKKw():void
        0x0027: invoke-virtual com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.AuCTel():void
        0x002a: invoke-virtual com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.valueOf():void
        0x002d: invoke-virtual com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.gEmmrt():void
        0x0030: invoke-virtual com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.isConnected():void
        0x0033: invoke-super androidx.fragment.app.Fragment.onDestroyView():void
        0x0036: return-void
    }

    public java.lang.Object onEach(o.PVO p0, androidx.lifecycle.Lifecycle$State p1, kotlin.jvm.functions.Function2 p2, kotlin.coroutines.Continuation p3) {
        0x0000: invoke-static o.PVP$Application.OLrzqt(o.PVP, o.PVO, androidx.lifecycle.Lifecycle$State, kotlin.jvm.functions.Function2, kotlin.coroutines.Continuation):java.lang.Object
        0x0004: return-object v1
    }

    public kotlinx.coroutines.flow.Flow onEach(o.PVO p0, o.adgO p1, androidx.lifecycle.Lifecycle$State p2, kotlin.jvm.functions.Function2 p3) {
        0x0000: invoke-static o.PVP$Application.KWHzl(o.PVP, o.PVO, o.adgO, androidx.lifecycle.Lifecycle$State, kotlin.jvm.functions.Function2):kotlinx.coroutines.flow.Flow
        0x0004: return-object v1
    }

    public kotlinx.coroutines.flow.Flow onEach(o.PVO p0, o.adgO p1, o.adgO p2, androidx.lifecycle.Lifecycle$State p3, o.adfd p4) {
        0x0000: invoke-static/range o.PVP$Application.copydefault(o.PVP, o.PVO, o.adgO, o.adgO, androidx.lifecycle.Lifecycle$State, o.adfd):kotlinx.coroutines.flow.Flow
        0x0004: return-object v1
    }

    public kotlinx.coroutines.flow.Flow onEach(o.PVO p0, o.adgO p1, o.adgO p2, o.adgO p3, androidx.lifecycle.Lifecycle$State p4, o.adfe p5) {
        0x0000: invoke-static/range o.PVP$Application.OLrzqt(o.PVP, o.PVO, o.adgO, o.adgO, o.adgO, androidx.lifecycle.Lifecycle$State, o.adfe):kotlinx.coroutines.flow.Flow
        0x0004: return-object v1
    }

    public kotlinx.coroutines.flow.Flow onEach(o.PVO p0, o.adgO p1, o.adgO p2, o.adgO p3, o.adgO p4, androidx.lifecycle.Lifecycle$State p5, o.adfj p6) {
        0x0000: invoke-static/range o.PVP$Application.EZpvd(o.PVP, o.PVO, o.adgO, o.adgO, o.adgO, o.adgO, androidx.lifecycle.Lifecycle$State, o.adfj):kotlinx.coroutines.flow.Flow
        0x0004: return-object v1
    }

    public kotlinx.coroutines.flow.Flow onEach(o.PVO p0, o.adgO p1, o.adgO p2, o.adgO p3, o.adgO p4, o.adgO p5, androidx.lifecycle.Lifecycle$State p6, o.adfg p7) {
        0x0000: invoke-static/range o.PVP$Application.OLrzqt(o.PVP, o.PVO, o.adgO, o.adgO, o.adgO, o.adgO, o.adgO, androidx.lifecycle.Lifecycle$State, o.adfg):kotlinx.coroutines.flow.Flow
        0x0004: return-object v1
    }

    public kotlinx.coroutines.flow.Flow onEach(o.PVO p0, o.adgO p1, o.adgO p2, o.adgO p3, o.adgO p4, o.adgO p5, o.adgO p6, androidx.lifecycle.Lifecycle$State p7, o.adfh p8) {
        0x0000: invoke-static/range o.PVP$Application.AEQbTJ(o.PVP, o.PVO, o.adgO, o.adgO, o.adgO, o.adgO, o.adgO, o.adgO, androidx.lifecycle.Lifecycle$State, o.adfh):kotlinx.coroutines.flow.Flow
        0x0004: return-object v1
    }

    public void onPause() {
        0x0000: invoke-virtual com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.sSMYrx():void
        0x0003: invoke-super o.uyN.onPause():void
        0x0006: return-void
    }

    public void onResume() {
        0x0000: invoke-static android.os.SystemClock.elapsedRealtime():long
        0x0004: invoke-super o.uyN.onResume():void
        0x0007: iget v0, v6, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.DbNXlk:o.zkI
        0x000d: const-string v0, ""
        0x000f: invoke-static kotlin.jvm.internal.Intrinsics.valueOf(java.lang.String):void
        0x0012: const/4 v0, 0
        0x0013: iget v0, v0, o.zkI.KWHzl:o.Acd
        0x0015: invoke-static kotlin.jvm.internal.Intrinsics.AEQbTJ(java.lang.Object):void
        0x0018: iget v1, v6, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.values:com.okinc.kline.library.data.ChartViewOutSideConfig
        0x001c: const-string v1, "com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2"
        0x001e: const-string v2, "onResume"
        0x0020: const-string v3, "()V"
        0x0023: invoke-static/range o.Bdb.OLrzqt(java.lang.Object, java.lang.String, java.lang.String, java.lang.String, long):void
        0x0026: return-void
        0x0027: invoke-virtual o.Acd.copydefault(com.okinc.kline.library.data.ChartViewOutSideConfig):void
        0x002a: invoke-virtual com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.zuBGHE():void
        0x002d: invoke-direct com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.dvKsVJ():void
        0x0030: invoke-virtual com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.fARcdN():void
        0x0033: invoke-virtual com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.ORxRYg():void
        0x0036: invoke-virtual com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.AEQbTJ():void
        0x0039: invoke-virtual com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.AwvSrB():void
        0x003c: invoke-virtual com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.AxsJAY():void
        0x003f: const-string v1, "com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2"
        0x0041: const-string v2, "onResume"
        0x0043: const-string v3, "()V"
        0x0046: invoke-static/range o.Bdb.OLrzqt(java.lang.Object, java.lang.String, java.lang.String, java.lang.String, long):void
        0x0049: return-void
        0x004a: const-string v1, "com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2"
        0x004c: const-string v2, "onResume"
        0x004e: const-string v3, "()V"
        0x0051: invoke-static/range o.Bdb.OLrzqt(java.lang.Object, java.lang.String, java.lang.String, java.lang.String, long):void
        0x0054: return-void
    }

    public void onStop() {
        0x0000: invoke-super o.uyN.onStop():void
        0x0003: iget v0, v1, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.DbNXlk:o.zkI
        0x0009: const-string v0, ""
        0x000b: invoke-static kotlin.jvm.internal.Intrinsics.valueOf(java.lang.String):void
        0x000e: const/4 v0, 0
        0x000f: iget v0, v0, o.zkI.KWHzl:o.Acd
        0x0011: invoke-virtual o.Acd.DTwDnp():void
        0x0014: return-void
    }

    public void sSMYrx() {
        0x0000: iget v0, v1, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.EZpvd:o.zFR
        0x0002: invoke-virtual o.zFR.fFgQHt():void
        0x0005: return-void
    }

    public kotlinx.coroutines.flow.Flow uiStateFlow(o.PVO p0) {
        0x0000: invoke-static o.PVP$Application.OLrzqt(o.PVP, o.PVO):kotlinx.coroutines.flow.Flow
        0x0004: return-object v1
    }

    public void uzCIH() {
        0x0000: iget v0, v1, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.OLrzqt:o.zxJ
        0x0002: invoke-virtual o.zxJ.uzCIH():void
        0x0005: return-void
    }

    public void valueOf() {
        0x0000: iget v0, v1, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.AhwBna:o.zTq
        0x0002: invoke-virtual o.zTq.KWHzl():void
        0x0005: return-void
    }

    public o.AbZ values() {
        0x0000: invoke-virtual androidx.fragment.app.Fragment.getView():android.view.View
        0x0004: const/4 v1, 0
        0x0007: iget v0, v2, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.DbNXlk:o.zkI
        0x000d: const-string v0, ""
        0x000f: invoke-static kotlin.jvm.internal.Intrinsics.valueOf(java.lang.String):void
        0x0014: iget v0, v1, o.zkI.KWHzl:o.Acd
        0x0016: invoke-virtual o.Acd.getChartManager():o.AbZ
        0x001a: return-object v0
        0x001b: return-object v1
    }

    public final boolean wlaJM() {
        0x0000: invoke-virtual androidx.fragment.app.Fragment.isAdded():boolean
        0x0006: invoke-virtual androidx.fragment.app.Fragment.getView():android.view.View
        0x000c: const/4 v0, 1
        0x000d: return v0
        0x000e: const/4 v0, 0
        0x000f: return v0
    }

    public void zLjUOn() {
        0x0000: iget v0, v1, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.EZpvd:o.zFR
        0x0002: invoke-virtual o.zFR.zLjUOn():void
        0x0005: return-void
    }

    public void zsXlph() {
        0x0000: iget v0, v1, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.OLrzqt:o.zxJ
        0x0002: invoke-virtual o.zxJ.zsXlph():void
        0x0005: return-void
    }

    public void zuBGHE() {
        0x0000: iget v0, v1, com.okinc.kline.features.botkline.v2.ui.BotKLineFullScreenFragmentV2.djBIcL:o.AaQ
        0x0002: invoke-virtual o.AaQ.AkhnZx():void
        0x0005: return-void
    }

}

// DEX: classes29.dex | Source: ProGuard/R8
public final class com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity /* obfuscated: Lo/aajh; */ extends o.aajQ {
    // Static Fields
    private static final byte[] $$d;
    private static final int $$e;
    private static int $10;
    private static int $11;
    private static int AubY;
    public static final o.aajh$TaskDescription Companion;
    public static final int OLrzqt;
    private static char ejfBZ;
    private static char fARcdN;
    private static char fIwbmz;
    private static char getFieldNames;
    private static char[] getNewProxyInstance;
    private static int hDKMBd;
    private static char iwGUEr;
    private static int uzCIH;
    private static int wlaJM;

    // Instance Fields
    public o.aajR AYXKKw;
    public o.APx AhwBna;
    public long AkhnZx;
    public o.Xsh DbNXlk;
    public androidx.fragment.app.Fragment EZpvd;
    public boolean KWHzl;
    public o.abuj copydefault;
    public final o.adaP djBIcL;
    public boolean gEmmrt;
    public boolean isConnected;
    public o.PPT valueOf;

    private static java.lang.String $$f(int p0, byte p1, int p2) {
        0x000a: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.$$d:byte[]
        0x000c: new-array v0, byte[]
        0x000e: const/4 v2, 0
    }

    static void <clinit>() {
        0x0000: const/4 v0, 4
        0x0001: new-array v0, byte[]
        0x0006: sput v0, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.$$d:byte[]
        0x0008: const/16 v0, 225
        0x000a: sput v0, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.$$e:int
        0x000c: const/4 v0, 0
        0x000d: sput v0, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.$10:int
        0x000f: const/4 v1, 1
        0x0010: sput v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.$11:int
        0x0012: sput v0, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.AubY:int
        0x0014: sput v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.wlaJM:int
        0x0016: sput v0, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0018: sput v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x001a: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd():void
        0x001d: sget v0, o.Bow.KWHzl:o.Bow
        0x001f: const-string v1, "com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity"
        0x0021: sget v2, com.okinc.localization.report.SupportType.CLASS:com.okinc.localization.report.SupportType
        0x0023: const/4 v3, 0
        0x0024: invoke-virtual o.Bow.copydefault(java.lang.String, com.okinc.localization.report.SupportType, java.lang.String):void
        0x0027: new-instance v0, o.aajh$TaskDescription
        0x0029: invoke-direct o.aajh$TaskDescription.<init>(kotlin.jvm.internal.DefaultConstructorMarker):void
        0x002c: sput v0, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.Companion:o.aajh$TaskDescription
        0x002e: const/16 v0, 8
        0x0030: sput v0, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.OLrzqt:int
        0x0032: sget v0, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.AubY:int
        0x0038: sput v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.wlaJM:int
        0x003c: return-void
    }

    public void <init>() {
        0x0000: invoke-direct o.aajQ.<init>():void
        0x0003: new-instance v0, o.aajR
        0x0005: const-wide/16 v1, 0
        0x0007: invoke-direct o.aajR.<init>(long):void
        0x000a: iput v0, v14, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.AYXKKw:o.aajR
        0x000c: const/4 v0, 1
        0x000d: iput v0, v14, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.KWHzl:boolean
        0x000f: iput v0, v14, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.gEmmrt:boolean
        0x0011: sget v2, o.WSQ$StateListAnimator.hOMIpD:int
        0x0013: new-instance v0, o.APx
        0x0015: const/4 v3, 0
        0x0016: const/4 v4, 0
        0x0017: const/4 v5, 0
        0x0018: const/4 v6, 0
        0x0019: const/4 v7, 0
        0x001a: const/4 v8, 0
        0x001b: const/4 v9, 0
        0x001c: const/4 v10, 0
        0x001d: const/4 v11, 0
        0x001e: const/16 v12, 894
        0x0020: const/4 v13, 0
        0x0022: invoke-direct/range o.APx.<init>(int, java.lang.CharSequence, int, int, com.okinc.kline.ui.view.KlineBottomActionBarButtonStyle, boolean, float, boolean, boolean, java.lang.CharSequence, int, kotlin.jvm.internal.DefaultConstructorMarker):void
        0x0025: iput v0, v14, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.AhwBna:o.APx
        0x0027: new-instance v0, o.aajL
        0x0029: invoke-direct o.aajL.<init>():void
        0x002c: invoke-static o.adaR.KWHzl(kotlin.jvm.functions.Function0):o.adaP
        0x0030: iput v0, v14, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.djBIcL:o.adaP
        0x0032: return-void
    }

    private static synthetic java.lang.Object AEQbTJ(java.lang.Object[] p0) {
        0x0000: const/4 v0, 0
        0x0005: const/4 v1, 1
        0x000a: const/4 v3, 2
        0x0013: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0016: invoke-virtual com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH():void
        0x0019: sget v0, kotlin.Unit.INSTANCE:kotlin.Unit
        0x001b: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0021: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0026: return-object v0
        0x0027: const/4 v3, 0
    }

    public static synthetic kotlin.Unit AEQbTJ(java.lang.Throwable p0) {
        0x0000: const/4 v0, 2
        0x0003: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0009: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x000c: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.EZpvd(java.lang.Throwable):kotlin.Unit
        0x0010: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0016: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x001b: const/16 v0, 76
        0x001f: return-object v3
    }

    public static synthetic kotlin.Unit AEQbTJ(o.PIG p0, com.okinc.share.bean.image.ImageShareParams p1) {
        0x0000: const/4 v0, 2
        0x0003: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0009: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x000c: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.OLrzqt(o.PIG, com.okinc.share.bean.image.ImageShareParams):kotlin.Unit
        0x0010: sget v4, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0016: sput v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x001b: return-object v3
        0x001c: const/4 v3, 0
    }

    public static synthetic kotlin.Unit AEQbTJ(com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity p0) {
        0x0000: const/4 v0, 2
        0x0003: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0009: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x000e: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.DbNXlk(com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity):kotlin.Unit
        0x0012: return-object v3
        0x0013: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.DbNXlk(com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity):kotlin.Unit
        0x0016: const/4 v3, 0
    }

    public static synthetic kotlin.Unit AEQbTJ(com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity p0, java.lang.String p1) {
        0x0000: const/4 v0, 2
        0x0003: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0009: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0012: invoke-static o.Hac.copydefault():int
        0x0016: invoke-static o.Hac.copydefault():int
        0x001a: invoke-static o.Hac.copydefault():int
        0x001e: invoke-static o.Hac.copydefault():int
        0x0022: const v8, -994144937
        0x0025: const v7, 994144937
        0x0028: invoke-static/range com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.copydefault(int, int, java.lang.Object[], int, int, int, int):java.lang.Object
        0x002c: check-cast v9, kotlin.Unit
        0x002e: const/4 v10, 1
        0x0036: invoke-static o.Hac.copydefault():int
        0x003a: invoke-static o.Hac.copydefault():int
        0x003e: invoke-static o.Hac.copydefault():int
        0x0042: invoke-static o.Hac.copydefault():int
        0x0046: const v6, -994144937
        0x0049: const v5, 994144937
        0x004c: invoke-static/range com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.copydefault(int, int, java.lang.Object[], int, int, int, int):java.lang.Object
        0x0050: check-cast v9, kotlin.Unit
        0x0052: return-object v9
    }

    public static synthetic kotlin.Unit AEQbTJ(com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity p0, o.PIG p1) {
        0x0004: invoke-static o.Hac.copydefault():int
        0x0008: invoke-static o.Hac.copydefault():int
        0x000c: invoke-static o.Hac.copydefault():int
        0x0010: invoke-static o.Hac.copydefault():int
        0x0014: const v6, 388836678
        0x0017: const v5, -388836675
        0x001a: invoke-static/range com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.copydefault(int, int, java.lang.Object[], int, int, int, int):java.lang.Object
        0x001e: check-cast v7, kotlin.Unit
        0x0020: return-object v7
    }

    public static synthetic kotlin.Unit AEQbTJ(com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity p0, o.PIG p1, java.io.File p2) {
        0x0000: const/4 v0, 2
        0x0003: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0009: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x000c: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.copydefault(com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity, o.PIG, java.io.File):kotlin.Unit
        0x0010: sget v4, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0016: sput v5, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0019: return-object v3
    }

    public static synthetic void AEQbTJ(kotlin.jvm.functions.Function1 p0, java.lang.Object p1) {
        0x0000: const/4 v0, 2
        0x0003: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0009: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0010: invoke-static o.Hac.copydefault():int
        0x0014: invoke-static o.Hac.copydefault():int
        0x0018: invoke-static o.Hac.copydefault():int
        0x001c: invoke-static o.Hac.copydefault():int
        0x0020: const v8, -545550193
        0x0023: const v7, 545550212
        0x0026: invoke-static/range com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.copydefault(int, int, java.lang.Object[], int, int, int, int):java.lang.Object
        0x0029: sget v9, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x002f: sput v10, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0034: return-void
        0x0035: const/4 v9, 0
    }

    private static synthetic java.lang.Object AYXKKw(java.lang.Object[] p0) {
        0x0000: const/4 v0, 0
        0x0003: check-cast v10, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity
        0x0005: const/4 v1, 2
        0x0008: sget v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x000e: sput v3, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0017: invoke-static o.Hac.copydefault():int
        0x001b: invoke-static o.Hac.copydefault():int
        0x001f: invoke-static o.Hac.copydefault():int
        0x0023: invoke-static o.Hac.copydefault():int
        0x0027: const v9, 1678229524
        0x002a: const v8, -1678229519
        0x002d: invoke-static/range com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.copydefault(int, int, java.lang.Object[], int, int, int, int):java.lang.Object
        0x0030: const/16 v10, 45
        0x0038: invoke-static o.Hac.copydefault():int
        0x003c: invoke-static o.Hac.copydefault():int
        0x0040: invoke-static o.Hac.copydefault():int
        0x0044: invoke-static o.Hac.copydefault():int
        0x0048: const v6, 1678229524
        0x004b: const v5, -1678229519
        0x004e: invoke-static/range com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.copydefault(int, int, java.lang.Object[], int, int, int, int):java.lang.Object
        0x0051: const/4 v10, 0
        0x0052: return-object v10
    }

    public static final java.lang.String AYXKKw(com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity p0) {
        0x0000: const/4 v0, 2
        0x0003: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0009: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x000c: invoke-virtual o.abQb.UeEOUB():com.okinc.unify_trade.refactor.presenter.AbsPresenter
        0x0010: check-cast v3, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter
        0x0012: invoke-virtual com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.AEQbTJ():java.lang.String
        0x0016: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x001c: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0021: return-object v3
        0x0022: const/4 v3, 0
        0x0023: invoke-virtual java.lang.Object.hashCode():int
    }

    public static final kotlin.Unit AYXKKw(com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity p0, java.lang.String p1) {
        0x0004: invoke-static o.Hac.copydefault():int
        0x0008: invoke-static o.Hac.copydefault():int
        0x000c: invoke-static o.Hac.copydefault():int
        0x0010: invoke-static o.Hac.copydefault():int
        0x0014: const v6, 1802337805
        0x0017: const v5, -1802337797
        0x001a: invoke-static/range com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.copydefault(int, int, java.lang.Object[], int, int, int, int):java.lang.Object
        0x001e: check-cast v7, kotlin.Unit
        0x0020: return-object v7
    }

    public static final void AYXKKw(kotlin.jvm.functions.Function1 p0, java.lang.Object p1) {
        0x0000: const/4 v0, 2
        0x0003: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0009: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x000c: invoke-interface kotlin.jvm.functions.Function1.invoke(java.lang.Object):java.lang.Object
        0x000f: sget v3, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0015: sput v4, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0018: return-void
    }

    private static synthetic java.lang.Object AhwBna(java.lang.Object[] p0) {
        0x0000: const/4 v0, 0
        0x0003: check-cast v4, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity
        0x0005: const/4 v1, 2
        0x0008: sget v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x000e: sput v3, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0013: invoke-virtual o.abQb.UeEOUB():com.okinc.unify_trade.refactor.presenter.AbsPresenter
        0x0017: check-cast v2, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter
        0x0019: invoke-virtual com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.fetchVPNInfo():void
        0x001c: invoke-virtual o.abQb.UeEOUB():com.okinc.unify_trade.refactor.presenter.AbsPresenter
        0x0020: check-cast v2, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter
        0x0022: invoke-virtual com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.isConnected():void
        0x0025: invoke-static java.lang.System.currentTimeMillis():long
        0x0029: iput v2, v4, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.AkhnZx:long
        0x002b: invoke-virtual o.abQb.UeEOUB():com.okinc.unify_trade.refactor.presenter.AbsPresenter
        0x002f: check-cast v4, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter
        0x0031: invoke-virtual com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.valueOf():void
        0x0034: const/16 v4, 44
        0x0038: invoke-virtual o.abQb.UeEOUB():com.okinc.unify_trade.refactor.presenter.AbsPresenter
        0x003c: check-cast v0, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter
        0x003e: invoke-virtual com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.fetchVPNInfo():void
        0x0041: invoke-virtual o.abQb.UeEOUB():com.okinc.unify_trade.refactor.presenter.AbsPresenter
        0x0045: check-cast v0, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter
        0x0047: invoke-virtual com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.isConnected():void
        0x004a: invoke-static java.lang.System.currentTimeMillis():long
        0x004e: iput v2, v4, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.AkhnZx:long
        0x0050: invoke-virtual o.abQb.UeEOUB():com.okinc.unify_trade.refactor.presenter.AbsPresenter
        0x0054: check-cast v4, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter
        0x0056: invoke-virtual com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.valueOf():void
        0x0059: sget v4, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x005f: sput v0, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0062: const/4 v4, 0
        0x0063: return-object v4
    }

    public static final void AhwBna(kotlin.jvm.functions.Function1 p0, java.lang.Object p1) {
        0x0004: invoke-static o.Hac.copydefault():int
        0x0008: invoke-static o.Hac.copydefault():int
        0x000c: invoke-static o.Hac.copydefault():int
        0x0010: invoke-static o.Hac.copydefault():int
        0x0014: const v6, -729889287
        0x0017: const v5, 729889302
        0x001a: invoke-static/range com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.copydefault(int, int, java.lang.Object[], int, int, int, int):java.lang.Object
        0x001d: return-void
    }

    public static final synthetic void AhwBna(com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity p0) {
        0x0000: const/4 v0, 2
        0x0003: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0009: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x000c: invoke-virtual com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.AuCTel():void
        0x0011: const/16 v3, 11
        0x0015: sget v3, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x001b: sput v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0020: const/16 v3, 39
        0x0024: return-void
    }

    private static synthetic java.lang.Object AkhnZx(java.lang.Object[] p0) {
        0x0000: const/4 v0, 0
        0x0005: const/4 v0, 2
        0x0008: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x000e: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0011: const-string v0, ""
        0x0015: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0018: sget v3, kotlin.Unit.INSTANCE:kotlin.Unit
        0x001a: return-object v3
        0x001b: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x001e: sget v3, kotlin.Unit.INSTANCE:kotlin.Unit
        0x0020: const/4 v3, 0
    }

    private static synthetic java.lang.Object AuCTel(java.lang.Object[] p0) {
        0x0000: const/4 v0, 0
        0x0005: const/4 v2, 1
        0x0013: sput v4, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0016: invoke-virtual androidx.fragment.app.FragmentActivity.getSupportFragmentManager():androidx.fragment.app.FragmentManager
        0x001a: invoke-virtual androidx.fragment.app.FragmentManager.beginTransaction():androidx.fragment.app.FragmentTransaction
        0x001e: sget v3, o.WSQ$StateListAnimator.setIconBitmap:int
        0x0020: invoke-virtual java.lang.Object.getClass():java.lang.Class
        0x0024: invoke-virtual java.lang.Class.getName():java.lang.String
        0x0028: invoke-virtual androidx.fragment.app.FragmentTransaction.replace(int, androidx.fragment.app.Fragment, java.lang.String):androidx.fragment.app.FragmentTransaction
        0x002c: invoke-virtual androidx.fragment.app.FragmentTransaction.commitAllowingStateLoss():int
        0x002f: sget v5, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0035: sput v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0038: const/4 v1, 0
        0x003b: const/16 v5, 56
        0x003e: return-object v1
    }

    private final void AuCTelauCTel() {
        0x0000: const/4 v0, 2
        0x0003: sget v1, o.WSQ$TaskDescription.gasjUx:int
        0x0005: const/4 v2, 0
        0x0006: invoke-static android.view.View.inflate(android.content.Context, int, android.view.ViewGroup):android.view.View
        0x000a: new-instance v3, android.util.DisplayMetrics
        0x000c: invoke-direct android.util.DisplayMetrics.<init>():void
        0x000f: invoke-virtual android.app.Activity.getWindowManager():android.view.WindowManager
        0x0013: invoke-interface android.view.WindowManager.getDefaultDisplay():android.view.Display
        0x0017: invoke-virtual android.view.Display.getMetrics(android.util.DisplayMetrics):void
        0x001a: iget v4, v3, android.util.DisplayMetrics.widthPixels:int
        0x001c: iget v3, v3, android.util.DisplayMetrics.heightPixels:int
        0x001e: invoke-static o.uIr.AEQbTJ(android.view.View, int, int):void
        0x0021: sget v3, o.WSQ$StateListAnimator.svUkWZ:int
        0x0023: invoke-virtual android.view.View.findViewById(int):android.view.View
        0x0027: check-cast v1, o.PPT
        0x0029: iput v1, v5, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.valueOf:o.PPT
        0x002b: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0031: sput v3, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0036: return-void
    }

    private final void AubY() {
        0x0000: const/4 v0, 2
        0x0003: invoke-virtual o.abQb.UeEOUB():com.okinc.unify_trade.refactor.presenter.AbsPresenter
        0x0007: check-cast v1, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter
        0x0009: invoke-virtual android.app.Activity.getIntent():android.content.Intent
        0x000d: const-string v3, "instId"
        0x000f: invoke-virtual android.content.Intent.getStringExtra(java.lang.String):java.lang.String
        0x0013: const-string v3, ""
        0x0018: invoke-virtual com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.OLrzqt(java.lang.String):void
        0x001b: invoke-virtual o.abQb.UeEOUB():com.okinc.unify_trade.refactor.presenter.AbsPresenter
        0x001f: check-cast v1, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter
        0x0021: invoke-virtual android.app.Activity.getIntent():android.content.Intent
        0x0025: const-string v4, "bizType"
        0x0027: invoke-virtual android.content.Intent.getStringExtra(java.lang.String):java.lang.String
        0x002d: sget v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0033: sput v4, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0037: invoke-virtual com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.KWHzl(java.lang.String):void
        0x003a: invoke-virtual o.abQb.UeEOUB():com.okinc.unify_trade.refactor.presenter.AbsPresenter
        0x003e: check-cast v1, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter
        0x0040: invoke-virtual android.app.Activity.getIntent():android.content.Intent
        0x0044: const-string v4, "source"
        0x0046: invoke-virtual android.content.Intent.getStringExtra(java.lang.String):java.lang.String
        0x004e: invoke-virtual com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.EZpvd(java.lang.String):void
        0x0051: new-instance v1, o.abuj
        0x0053: invoke-virtual o.abQb.UeEOUB():com.okinc.unify_trade.refactor.presenter.AbsPresenter
        0x0057: check-cast v2, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter
        0x0059: invoke-virtual com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.OLrzqt():java.lang.String
        0x005d: invoke-virtual o.abQb.UeEOUB():com.okinc.unify_trade.refactor.presenter.AbsPresenter
        0x0061: check-cast v3, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter
        0x0063: invoke-virtual com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.KWHzl():java.lang.String
        0x0067: invoke-direct o.abuj.<init>(java.lang.String, java.lang.String):void
        0x006a: iput v1, v6, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.copydefault:o.abuj
        0x006c: sget v1, android.os.Build$VERSION.SDK_INT:int
        0x006e: const/16 v2, 33
        0x0070: const-string v3, "tactics_state_data"
        0x0072: const/4 v4, 0
        0x0075: invoke-virtual o.abQb.UeEOUB():com.okinc.unify_trade.refactor.presenter.AbsPresenter
        0x0079: check-cast v1, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter
        0x007b: invoke-virtual android.app.Activity.getIntent():android.content.Intent
        0x007f: const-class v5, com.okinc.unify_trade.bot.data.TacticsData
        0x0081: invoke-static androidx.credentials.provider.IntentHandlerConverters$$ExternalSyntheticApiModelOutline1.m(android.content.Intent, java.lang.String, java.lang.Class):java.lang.Object
        0x0085: check-cast v2, com.okinc.unify_trade.bot.data.TacticsData
        0x0087: invoke-virtual com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.EZpvd(com.okinc.unify_trade.bot.data.TacticsData):void
        0x008b: invoke-virtual o.abQb.UeEOUB():com.okinc.unify_trade.refactor.presenter.AbsPresenter
        0x008f: check-cast v1, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter
        0x0091: invoke-virtual android.app.Activity.getIntent():android.content.Intent
        0x0095: invoke-virtual android.content.Intent.getParcelableExtra(java.lang.String):android.os.Parcelable
        0x0099: instance-of v3, com.okinc.unify_trade.bot.data.TacticsData
        0x009d: sget v3, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x00a3: sput v5, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x00a6: check-cast v2, com.okinc.unify_trade.bot.data.TacticsData
        0x00aa: invoke-virtual com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.EZpvd(com.okinc.unify_trade.bot.data.TacticsData):void
        0x00ad: invoke-virtual o.abQb.UeEOUB():com.okinc.unify_trade.refactor.presenter.AbsPresenter
        0x00b1: check-cast v1, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter
        0x00b3: invoke-virtual com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.gEmmrt():com.okinc.unify_trade.bot.data.TacticsData
        0x00b9: sget v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x00bf: sput v3, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x00c4: invoke-virtual com.okinc.unify_trade.bot.data.TacticsData.getAccountId():java.lang.Long
        0x00ca: invoke-virtual java.lang.Long.longValue():long
        0x00cf: invoke-virtual com.okinc.unify_trade.bot.data.TacticsData.getAccountId():java.lang.Long
        0x00d2: invoke-virtual java.lang.Object.hashCode():int
        0x00d6: const-wide/16 v0, 0
        0x00d8: new-instance v2, o.aajR
        0x00da: invoke-direct o.aajR.<init>(long):void
        0x00dd: iput v2, v6, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.AYXKKw:o.aajR
        0x00df: return-void
    }

    private final void AxsJAY() {
        0x0000: const/4 v0, 2
        0x0003: invoke-virtual o.abQb.UeEOUB():com.okinc.unify_trade.refactor.presenter.AbsPresenter
        0x0007: check-cast v1, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter
        0x0009: invoke-virtual com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.AYXKKw():androidx.lifecycle.TradeLiveData
        0x000d: new-instance v2, o.aajK$Activity
        0x000f: new-instance v3, o.aajr
        0x0011: invoke-direct o.aajr.<init>(com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity):void
        0x0014: invoke-direct o.aajK$Activity.<init>(kotlin.jvm.functions.Function1):void
        0x0017: invoke-virtual androidx.lifecycle.TradeLiveData.observe(androidx.lifecycle.LifecycleOwner, androidx.lifecycle.Observer):void
        0x001a: invoke-virtual o.abQb.UeEOUB():com.okinc.unify_trade.refactor.presenter.AbsPresenter
        0x001e: check-cast v1, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter
        0x0020: invoke-virtual com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.EZpvd():androidx.lifecycle.TradeLiveData
        0x0024: new-instance v2, o.aajK$Activity
        0x0026: new-instance v3, o.aajo
        0x0028: invoke-direct o.aajo.<init>(com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity):void
        0x002b: invoke-direct o.aajK$Activity.<init>(kotlin.jvm.functions.Function1):void
        0x002e: invoke-virtual androidx.lifecycle.TradeLiveData.observe(androidx.lifecycle.LifecycleOwner, androidx.lifecycle.Observer):void
        0x0031: invoke-virtual o.abQb.UeEOUB():com.okinc.unify_trade.refactor.presenter.AbsPresenter
        0x0035: check-cast v1, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter
        0x0037: invoke-virtual com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.copydefault():androidx.lifecycle.TradeLiveData
        0x003b: new-instance v2, o.aajK$Activity
        0x003d: new-instance v3, o.aajs
        0x003f: invoke-direct o.aajs.<init>(com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity):void
        0x0042: invoke-direct o.aajK$Activity.<init>(kotlin.jvm.functions.Function1):void
        0x0045: invoke-virtual androidx.lifecycle.TradeLiveData.observe(androidx.lifecycle.LifecycleOwner, androidx.lifecycle.Observer):void
        0x0048: invoke-virtual o.abQb.UeEOUB():com.okinc.unify_trade.refactor.presenter.AbsPresenter
        0x004c: check-cast v1, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter
        0x004e: invoke-virtual com.okinc.unify_trade.refactor.presenter.AbsPresenter.getAggregateLoadingLiveData():androidx.lifecycle.LiveData
        0x0052: new-instance v2, o.aajK$Activity
        0x0054: new-instance v3, o.aajv
        0x0056: invoke-direct o.aajv.<init>(com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity):void
        0x0059: invoke-direct o.aajK$Activity.<init>(kotlin.jvm.functions.Function1):void
        0x005c: invoke-virtual androidx.lifecycle.LiveData.observe(androidx.lifecycle.LifecycleOwner, androidx.lifecycle.Observer):void
        0x005f: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0065: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x006a: return-void
        0x006b: const/4 v0, 0
        0x006c: invoke-virtual java.lang.Object.hashCode():int
    }

    private static synthetic java.lang.Object DbNXlk(java.lang.Object[] p0) {
        0x0000: const/4 v0, 0
        0x0003: check-cast v0, kotlin.jvm.functions.Function1
        0x0005: const/4 v1, 1
        0x0009: check-cast v1, java.lang.Object
        0x000b: const/4 v1, 2
        0x000e: sget v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0014: sput v3, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0017: invoke-interface kotlin.jvm.functions.Function1.invoke(java.lang.Object):java.lang.Object
        0x001a: sget v4, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0020: sput v0, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0023: const/4 v0, 0
        0x0026: return-object v0
        0x0027: invoke-virtual java.lang.Object.hashCode():int
    }

    public static final kotlin.Unit DbNXlk(com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity p0) {
        0x0000: const/4 v0, 2
        0x0003: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0009: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x000c: invoke-virtual android.app.Activity.getWindow():android.view.Window
        0x0010: invoke-virtual android.view.Window.getDecorView():android.view.View
        0x0014: iget v2, v5, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.valueOf:o.PPT
        0x0016: invoke-static o.uIr.AEQbTJ(android.view.View, android.view.View):android.graphics.Bitmap
        0x001c: sget v2, com.okinc.biz.share.ShareFileProvider.Companion:com.okinc.biz.share.ShareFileProvider$Activity
        0x001e: new-instance v3, o.aajx
        0x0020: invoke-direct o.aajx.<init>():void
        0x0023: new-instance v4, o.aajw
        0x0025: invoke-direct o.aajw.<init>(com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity):void
        0x0028: invoke-virtual com.okinc.biz.share.ShareFileProvider$Activity.AEQbTJ(android.graphics.Bitmap, androidx.lifecycle.LifecycleOwner, kotlin.jvm.functions.Function1, kotlin.jvm.functions.Function1):void
        0x002b: sget v5, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0031: sput v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0034: sget v5, kotlin.Unit.INSTANCE:kotlin.Unit
        0x0036: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x003c: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0041: return-object v5
        0x0042: const/4 v5, 0
    }

    private static synthetic java.lang.Object EZpvd(java.lang.Object[] p0) {
        0x0000: const/4 v0, 0
        0x0003: check-cast v0, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity
        0x0005: const/4 v1, 1
        0x0008: check-cast v4, o.PIG
        0x000a: const/4 v1, 2
        0x000d: sget v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0013: sput v3, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0018: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.KWHzl(com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity, o.PIG):kotlin.Unit
        0x001c: return-object v4
        0x001d: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.KWHzl(com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity, o.PIG):kotlin.Unit
        0x0020: const/4 v4, 0
        0x0021: invoke-virtual java.lang.Object.hashCode():int
    }

    public static final kotlin.Unit EZpvd(java.lang.Throwable p0) {
        0x0000: const/4 v0, 2
        0x0003: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0009: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x000c: const-string v2, ""
        0x0010: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0013: sget v3, kotlin.Unit.INSTANCE:kotlin.Unit
        0x0015: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x001b: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0020: const/16 v0, 9
        0x0024: return-object v3
        0x0025: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0028: sget v3, kotlin.Unit.INSTANCE:kotlin.Unit
        0x002a: const/4 v3, 0
        0x002b: invoke-virtual java.lang.Object.hashCode():int
    }

    public static synthetic kotlin.Unit EZpvd(o.EJo p0) {
        0x0000: const/4 v0, 2
        0x0003: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0009: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0010: invoke-static o.Hac.copydefault():int
        0x0016: invoke-static o.Hac.copydefault():int
        0x001a: invoke-static o.Hac.copydefault():int
        0x001e: invoke-static o.Hac.copydefault():int
        0x0022: const v8, 989395512
        0x0025: const v7, -989395496
        0x0028: invoke-static/range com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.copydefault(int, int, java.lang.Object[], int, int, int, int):java.lang.Object
        0x002c: check-cast v9, kotlin.Unit
        0x002e: const/16 v1, 31
        0x0033: invoke-static o.Hac.copydefault():int
        0x0037: invoke-static o.Hac.copydefault():int
        0x003b: invoke-static o.Hac.copydefault():int
        0x003f: const v8, 989395512
        0x0042: const v7, -989395496
        0x0045: invoke-static/range com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.copydefault(int, int, java.lang.Object[], int, int, int, int):java.lang.Object
        0x0049: check-cast v9, kotlin.Unit
        0x004b: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0051: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0054: return-object v9
    }

    public static synthetic kotlin.Unit EZpvd(com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity p0) {
        0x0000: const/4 v0, 2
        0x0003: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0009: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x000c: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.isConnected(com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity):kotlin.Unit
        0x0010: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0016: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x001b: return-object v3
        0x001c: const/4 v3, 0
    }

    public static synthetic kotlin.Unit EZpvd(com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity p0, java.io.File p1) {
        0x0000: const/4 v0, 2
        0x0003: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0009: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x000e: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.OLrzqt(com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity, java.io.File):kotlin.Unit
        0x0012: return-object v3
        0x0013: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.OLrzqt(com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity, java.io.File):kotlin.Unit
        0x0016: const/4 v3, 0
        0x0017: invoke-virtual java.lang.Object.hashCode():int
    }

    public static final kotlin.Unit EZpvd(com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity p0, java.lang.String p1) {
        0x0004: invoke-static o.Hac.copydefault():int
        0x0008: invoke-static o.Hac.copydefault():int
        0x000c: invoke-static o.Hac.copydefault():int
        0x0010: invoke-static o.Hac.copydefault():int
        0x0014: const v6, -994144937
        0x0017: const v5, 994144937
        0x001a: invoke-static/range com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.copydefault(int, int, java.lang.Object[], int, int, int, int):java.lang.Object
        0x001e: check-cast v7, kotlin.Unit
        0x0020: return-object v7
    }

    public static synthetic o.Bpy EZpvd() {
        0x0000: const/4 v0, 2
        0x0003: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0009: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x000c: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.valueOf():o.Bpy
        0x0010: sget v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0016: sput v3, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x001b: const/16 v0, 26
        0x001f: return-object v1
    }

    public static synthetic void EZpvd(kotlin.jvm.functions.Function1 p0, java.lang.Object p1) {
        0x0004: invoke-static o.Hac.copydefault():int
        0x0008: invoke-static o.Hac.copydefault():int
        0x000c: invoke-static o.Hac.copydefault():int
        0x0010: invoke-static o.Hac.copydefault():int
        0x0014: const v6, -1973296232
        0x0017: const v5, 1973296236
        0x001a: invoke-static/range com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.copydefault(int, int, java.lang.Object[], int, int, int, int):java.lang.Object
        0x001d: return-void
    }

    private static synthetic java.lang.Object KWHzl(java.lang.Object[] p0) {
        0x0000: const/4 v0, 0
        0x0003: check-cast v0, kotlin.jvm.functions.Function1
        0x0005: const/4 v1, 1
        0x0009: check-cast v1, java.lang.Object
        0x000b: const/4 v1, 2
        0x000e: sget v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0014: sput v3, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0017: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.gEmmrt(kotlin.jvm.functions.Function1, java.lang.Object):void
        0x001a: sget v4, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0020: sput v0, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0023: const/4 v4, 0
        0x0024: return-object v4
    }

    public static synthetic kotlin.Unit KWHzl(com.okinc.share.bean.image.ImageShareParams p0) {
        0x0000: const/4 v0, 2
        0x0003: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0009: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x000c: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.copydefault(com.okinc.share.bean.image.ImageShareParams):kotlin.Unit
        0x0010: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0016: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x001b: const/16 v0, 57
        0x001f: return-object v3
    }

    public static synthetic kotlin.Unit KWHzl(java.lang.Throwable p0) {
        0x0000: const/4 v0, 2
        0x0003: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0009: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x000e: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.copydefault(java.lang.Throwable):kotlin.Unit
        0x0012: return-object v3
        0x0013: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.copydefault(java.lang.Throwable):kotlin.Unit
        0x0016: const/4 v3, 0
        0x0017: invoke-virtual java.lang.Object.hashCode():int
    }

    public static final kotlin.Unit KWHzl(com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity p0, androidx.core.util.Pair p1) {
        0x0000: const/4 v0, 2
        0x0003: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0009: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x000e: iget v4, v4, androidx.core.util.Pair.second:java.lang.Object
        0x0010: check-cast v4, java.lang.Boolean
        0x0012: invoke-virtual java.lang.Boolean.booleanValue():boolean
        0x0016: const/16 v1, 76
        0x001d: iget v4, v4, androidx.core.util.Pair.second:java.lang.Object
        0x001f: check-cast v4, java.lang.Boolean
        0x0021: invoke-virtual java.lang.Boolean.booleanValue():boolean
        0x0027: sget v4, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x002d: sput v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0032: invoke-virtual o.uyI.showLoading():void
        0x0035: const/16 v3, 31
        0x003a: invoke-virtual o.uyI.showLoading():void
        0x003e: invoke-virtual o.uyI.dismissLoading():void
        0x0041: sget v3, kotlin.Unit.INSTANCE:kotlin.Unit
        0x0043: return-object v3
    }

    public static final kotlin.Unit KWHzl(com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity p0, com.okinc.unify_trade.bot.data.GridPendingOrderData p1) {
        0x0000: const-string v0, ""
        0x0002: const/4 v1, 2
        0x0007: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x000a: invoke-virtual/range com.okinc.unify_trade.bot.data.GridPendingOrderData.getBuyList():java.util.List
        0x000e: check-cast v3, java.lang.Iterable
        0x0010: new-instance v4, java.util.ArrayList
        0x0012: const/16 v5, 10
        0x0014: invoke-static o.adbE.AEQbTJ(java.lang.Iterable, int):int
        0x0018: invoke-direct java.util.ArrayList.<init>(int):void
        0x001b: invoke-interface java.lang.Iterable.iterator():java.util.Iterator
        0x001f: invoke-interface java.util.Iterator.hasNext():boolean
        0x0025: invoke-interface java.util.Iterator.next():java.lang.Object
        0x0029: check-cast v6, com.okinc.unify_trade.bot.data.GridPendingItemData
        0x002b: new-instance v7, com.okinc.kline.ui.bean.BotOrderItem
        0x002d: invoke-direct com.okinc.kline.ui.bean.BotOrderItem.<init>():void
        0x0030: invoke-virtual com.okinc.unify_trade.bot.data.GridPendingItemData.getTitle():java.lang.String
        0x0034: invoke-virtual com.okinc.kline.ui.bean.BotOrderItem.setPrice(java.lang.String):void
        0x0037: invoke-virtual com.okinc.unify_trade.bot.data.GridPendingItemData.getAmount():java.lang.String
        0x003e: invoke-virtual com.okinc.kline.ui.bean.BotOrderItem.setAmount(java.lang.String):void
        0x0041: invoke-interface java.util.Collection.add(java.lang.Object):boolean
        0x0045: invoke-virtual/range com.okinc.unify_trade.bot.data.GridPendingOrderData.getSellList():java.util.List
        0x0049: check-cast v2, java.lang.Iterable
        0x004b: new-instance v3, java.util.ArrayList
        0x004d: invoke-static o.adbE.AEQbTJ(java.lang.Iterable, int):int
        0x0051: invoke-direct java.util.ArrayList.<init>(int):void
        0x0054: invoke-interface java.lang.Iterable.iterator():java.util.Iterator
        0x0058: invoke-interface java.util.Iterator.hasNext():boolean
        0x005e: invoke-interface java.util.Iterator.next():java.lang.Object
        0x0062: check-cast v5, com.okinc.unify_trade.bot.data.GridPendingItemData
        0x0064: new-instance v6, com.okinc.kline.ui.bean.BotOrderItem
        0x0066: invoke-direct com.okinc.kline.ui.bean.BotOrderItem.<init>():void
        0x0069: invoke-virtual com.okinc.unify_trade.bot.data.GridPendingItemData.getTitle():java.lang.String
        0x006d: invoke-virtual com.okinc.kline.ui.bean.BotOrderItem.setPrice(java.lang.String):void
        0x0070: invoke-virtual com.okinc.unify_trade.bot.data.GridPendingItemData.getAmount():java.lang.String
        0x0077: invoke-virtual com.okinc.kline.ui.bean.BotOrderItem.setAmount(java.lang.String):void
        0x007a: invoke-interface java.util.Collection.add(java.lang.Object):boolean
        0x0080: iget v2, v5, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.EZpvd:androidx.fragment.app.Fragment
        0x0084: invoke-virtual/range o.abQb.UeEOUB():com.okinc.unify_trade.refactor.presenter.AbsPresenter
        0x0088: check-cast v6, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter
        0x008a: invoke-virtual com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.gEmmrt():com.okinc.unify_trade.bot.data.TacticsData
        0x0090: invoke-virtual com.okinc.unify_trade.bot.data.TacticsData.getTriggerParams():java.util.List
        0x0096: check-cast v6, java.lang.Iterable
        0x0098: new-instance v8, java.util.ArrayList
        0x009a: invoke-direct java.util.ArrayList.<init>():void
        0x009d: invoke-interface java.lang.Iterable.iterator():java.util.Iterator
        0x00a1: invoke-interface java.util.Iterator.hasNext():boolean
        0x00a7: invoke-interface java.util.Iterator.next():java.lang.Object
        0x00ac: check-cast v10, com.okinc.unify_trade.biz.DcaTriggerParam
        0x00b0: invoke-virtual com.okinc.unify_trade.biz.DcaTriggerParam.getTriggerAction():java.lang.String
        0x00b5: const/4 v10, 0
        0x00b6: const-string v11, "stop"
        0x00b8: invoke-static kotlin.jvm.internal.Intrinsics.KWHzl(java.lang.Object, java.lang.Object):boolean
        0x00be: invoke-interface java.util.Collection.add(java.lang.Object):boolean
        0x00c2: const/4 v8, 0
        0x00c3: invoke-static o.AtZ.OLrzqt():o.adaP
        0x00c7: invoke-interface o.adaP.getValue():java.lang.Object
        0x00cb: check-cast v6, o.Aqk
        0x00cd: new-instance v15, com.okinc.kline.ui.bean.IBotOrdersDataBean
        0x00cf: const/4 v10, 0
        0x00d0: const/4 v11, 0
        0x00d1: const/4 v12, 0
        0x00d2: const/4 v13, 0
        0x00d3: const/4 v14, 0
        0x00d4: const/16 v16, 0
        0x00d6: const/16 v17, 0
        0x00d8: const/16 v18, 0
        0x00da: const/16 v19, 0
        0x00dc: const/16 v20, 0
        0x00de: const/16 v21, 0
        0x00e0: const/16 v22, 2047
        0x00e2: const/16 v23, 0
        0x00f6: invoke-direct/range com.okinc.kline.ui.bean.IBotOrdersDataBean.<init>(java.lang.String, java.lang.String, java.util.ArrayList, java.util.ArrayList, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, com.okinc.kline.ui.bean.BotOrderItem, java.lang.String, int, kotlin.jvm.internal.DefaultConstructorMarker):void
        0x00f9: invoke-virtual/range o.abQb.UeEOUB():com.okinc.unify_trade.refactor.presenter.AbsPresenter
        0x00fd: check-cast v9, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter
        0x00ff: invoke-virtual com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.gEmmrt():com.okinc.unify_trade.bot.data.TacticsData
        0x0105: invoke-virtual com.okinc.unify_trade.bot.data.TacticsData.getOrderType():java.lang.String
        0x010c: invoke-virtual com.okinc.kline.ui.bean.IBotOrdersDataBean.setType(java.lang.String):void
        0x010f: invoke-virtual com.okinc.kline.ui.bean.IBotOrdersDataBean.setBuyList(java.util.ArrayList):void
        0x0112: invoke-virtual com.okinc.kline.ui.bean.IBotOrdersDataBean.setSellList(java.util.ArrayList):void
        0x0115: invoke-virtual/range o.abQb.UeEOUB():com.okinc.unify_trade.refactor.presenter.AbsPresenter
        0x0119: check-cast v3, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter
        0x011b: invoke-virtual com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.gEmmrt():com.okinc.unify_trade.bot.data.TacticsData
        0x0121: invoke-virtual com.okinc.unify_trade.bot.data.TacticsData.getTpTriggerPx():java.lang.String
        0x0126: const/4 v3, 0
        0x0129: sget v3, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x012f: sput v4, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0133: invoke-virtual com.okinc.kline.ui.bean.IBotOrdersDataBean.setTpPrice(java.lang.String):void
        0x0136: invoke-virtual/range o.abQb.UeEOUB():com.okinc.unify_trade.refactor.presenter.AbsPresenter
        0x013a: check-cast v3, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter
        0x013c: invoke-virtual com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.gEmmrt():com.okinc.unify_trade.bot.data.TacticsData
        0x0142: sget v4, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0148: sput v9, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x014d: invoke-virtual com.okinc.unify_trade.bot.data.TacticsData.getSlTriggerPx():java.lang.String
        0x0152: const/4 v3, 0
        0x0154: invoke-virtual com.okinc.unify_trade.bot.data.TacticsData.getSlTriggerPx():java.lang.String
        0x0157: const/4 v3, 0
        0x0158: invoke-virtual java.lang.Object.hashCode():int
        0x015c: const/4 v3, 0
        0x0161: invoke-virtual com.okinc.kline.ui.bean.IBotOrdersDataBean.setSlPrice(java.lang.String):void
        0x0166: invoke-interface java.util.List.size():int
        0x016c: sget v4, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0172: sput v9, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x017b: sput v4, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0180: invoke-static kotlin.collections.CollectionsKt.EZpvd(java.util.List):java.lang.Object
        0x0184: check-cast v4, com.okinc.unify_trade.biz.DcaTriggerParam
        0x0186: const/16 v8, 64
        0x018d: invoke-static kotlin.collections.CollectionsKt.EZpvd(java.util.List):java.lang.Object
        0x0191: check-cast v4, com.okinc.unify_trade.biz.DcaTriggerParam
        0x0195: invoke-virtual com.okinc.unify_trade.biz.DcaTriggerParam.getTriggerPx():java.lang.String
        0x0199: invoke-static o.uAv.gEmmrt(java.lang.Object):java.lang.String
        0x019f: invoke-virtual com.okinc.kline.ui.bean.IBotOrdersDataBean.setStopTriggerPrice(java.lang.String):void
        0x01a2: invoke-virtual/range o.abQb.UeEOUB():com.okinc.unify_trade.refactor.presenter.AbsPresenter
        0x01a6: check-cast v3, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter
        0x01a8: invoke-virtual com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.gEmmrt():com.okinc.unify_trade.bot.data.TacticsData
        0x01ae: sget v4, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x01b4: sput v5, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x01b9: invoke-virtual com.okinc.unify_trade.bot.data.TacticsData.getLiqPx():java.lang.String
        0x01bd: const/16 v3, 54
        0x01c4: invoke-virtual com.okinc.unify_trade.bot.data.TacticsData.getLiqPx():java.lang.String
        0x01cc: invoke-virtual com.okinc.kline.ui.bean.IBotOrdersDataBean.setLiqPrice(java.lang.String):void
        0x01cf: sget v0, kotlin.Unit.INSTANCE:kotlin.Unit
        0x01d1: invoke-interface o.Aqk.AEQbTJ(androidx.fragment.app.Fragment, com.okinc.kline.ui.bean.IBotOrdersDataBean):void
        0x01d4: sget v0, kotlin.Unit.INSTANCE:kotlin.Unit
        0x01d6: return-object v0
    }

    public static synthetic kotlin.Unit KWHzl(com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity p0, java.lang.String p1) {
        0x0000: const/4 v0, 2
        0x0003: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0009: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x000c: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.copydefault(com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity, java.lang.String):kotlin.Unit
        0x0010: sget v4, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0016: sput v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x001b: const/16 v4, 11
        0x001f: return-object v3
    }

    public static final kotlin.Unit KWHzl(com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity p0, java.util.List p1) {
        0x0000: const/4 v0, 2
        0x0003: const-string v1, ""
        0x0005: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0008: invoke-virtual o.abQb.UeEOUB():com.okinc.unify_trade.refactor.presenter.AbsPresenter
        0x000c: check-cast v2, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter
        0x000e: invoke-static kotlin.collections.CollectionsKt.EZpvd(java.util.List):java.lang.Object
        0x0012: check-cast v6, com.okinc.unify_trade.biz.subscribe.TickersData
        0x0016: sget v3, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x001c: sput v4, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x001f: invoke-virtual com.okinc.unify_trade.biz.subscribe.TickersData.getLast():java.lang.String
        0x0028: sget v6, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x002e: sput v3, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0031: invoke-virtual com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.copydefault(java.lang.String):kotlin.Pair
        0x0035: invoke-virtual o.abQb.UeEOUB():com.okinc.unify_trade.refactor.presenter.AbsPresenter
        0x0039: check-cast v1, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter
        0x003b: invoke-virtual com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.djBIcL():androidx.lifecycle.TradeLiveData
        0x003f: invoke-virtual androidx.lifecycle.MutableLiveData.postValue(java.lang.Object):void
        0x0042: invoke-virtual o.abQb.UeEOUB():com.okinc.unify_trade.refactor.presenter.AbsPresenter
        0x0046: check-cast v1, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter
        0x0048: invoke-virtual com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.gEmmrt():com.okinc.unify_trade.bot.data.TacticsData
        0x004e: invoke-virtual com.okinc.unify_trade.bot.data.TacticsData.getOrderType():java.lang.String
        0x0054: const-string v2, "grid"
        0x0056: invoke-static kotlin.jvm.internal.Intrinsics.KWHzl(java.lang.Object, java.lang.Object):boolean
        0x005c: const-string v2, "contract_grid"
        0x005e: invoke-static kotlin.jvm.internal.Intrinsics.KWHzl(java.lang.Object, java.lang.Object):boolean
        0x0064: invoke-virtual o.abQb.UeEOUB():com.okinc.unify_trade.refactor.presenter.AbsPresenter
        0x0068: check-cast v5, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter
        0x006a: invoke-virtual kotlin.Pair.getFirst():java.lang.Object
        0x006e: check-cast v6, java.lang.String
        0x0070: invoke-virtual com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.AEQbTJ(java.lang.String):void
        0x0073: sget v5, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0079: sput v6, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x007c: sget v5, kotlin.Unit.INSTANCE:kotlin.Unit
        0x007e: return-object v5
    }

    public static final kotlin.Unit KWHzl(com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity p0, kotlin.jvm.functions.Function0 p1, o.APx p2) {
        0x0000: const/4 v0, 2
        0x0003: const-string v1, ""
        0x0005: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0008: invoke-virtual o.APx.AYXKKw():int
        0x000c: sget v1, o.WSQ$StateListAnimator.hlXVux:int
        0x0010: sget v10, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0016: sput v11, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x001b: invoke-virtual com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.AkhnZx():void
        0x001e: const/16 v9, 84
        0x0023: invoke-virtual com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.AkhnZx():void
        0x0027: sget v1, o.WSQ$StateListAnimator.hOMIpD:int
        0x002f: invoke-static o.Hac.copydefault():int
        0x0033: invoke-static o.Hac.copydefault():int
        0x0037: invoke-static o.Hac.copydefault():int
        0x003b: invoke-static o.Hac.copydefault():int
        0x003f: const v8, 1678229524
        0x0042: const v7, -1678229519
        0x0045: invoke-static/range com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.copydefault(int, int, java.lang.Object[], int, int, int, int):java.lang.Object
        0x0049: sget v1, o.WSQ$StateListAnimator.processStrongAuthMessage:int
        0x004d: invoke-interface kotlin.jvm.functions.Function0.invoke():java.lang.Object
        0x0051: check-cast v10, java.lang.String
        0x0053: invoke-virtual com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.OLrzqt(java.lang.String):void
        0x0056: sget v9, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x005c: sput v10, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0060: sget v10, o.WSQ$StateListAnimator.init:int
        0x0064: sget v10, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x006a: sput v11, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x006d: invoke-virtual com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.fJNWhG():void
        0x0070: sget v9, kotlin.Unit.INSTANCE:kotlin.Unit
        0x0072: sget v10, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0078: sput v11, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x007b: return-object v9
    }

    public static final kotlin.Unit KWHzl(com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity p0, o.PIG p1) {
        0x0000: const/4 v0, 2
        0x0003: invoke-virtual o.PIG.AEQbTJ():java.lang.String
        0x0007: new-instance v2, java.lang.StringBuilder
        0x0009: invoke-direct java.lang.StringBuilder.<init>():void
        0x000c: const-string v3, " result.shortLink = "
        0x000e: invoke-virtual java.lang.StringBuilder.append(java.lang.String):java.lang.StringBuilder
        0x0011: invoke-virtual java.lang.StringBuilder.append(java.lang.String):java.lang.StringBuilder
        0x0014: const-string v1, "WMYYYY"
        0x0016: invoke-virtual java.lang.StringBuilder.toString():java.lang.String
        0x001a: invoke-static o.BqX.copydefault(java.lang.String, java.lang.String):void
        0x001d: invoke-virtual android.app.Activity.getWindow():android.view.Window
        0x0021: invoke-virtual android.view.Window.getDecorView():android.view.View
        0x0025: iget v2, v5, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.valueOf:o.PPT
        0x0027: invoke-static o.uIr.AEQbTJ(android.view.View, android.view.View):android.graphics.Bitmap
        0x002d: sget v2, com.okinc.biz.share.ShareFileProvider.Companion:com.okinc.biz.share.ShareFileProvider$Activity
        0x002f: new-instance v3, o.aajH
        0x0031: invoke-direct o.aajH.<init>():void
        0x0034: new-instance v4, o.aajD
        0x0036: invoke-direct o.aajD.<init>(com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity, o.PIG):void
        0x0039: invoke-virtual com.okinc.biz.share.ShareFileProvider$Activity.AEQbTJ(android.graphics.Bitmap, androidx.lifecycle.LifecycleOwner, kotlin.jvm.functions.Function1, kotlin.jvm.functions.Function1):void
        0x003c: sget v5, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0042: sput v6, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0045: sget v5, kotlin.Unit.INSTANCE:kotlin.Unit
        0x0047: return-object v5
    }

    public static final kotlin.Unit KWHzl(com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity p0, o.abrD p1) {
        0x0000: const/4 v0, 2
        0x0003: const-string v1, ""
        0x0005: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0008: invoke-virtual o.abrD.AEQbTJ():java.lang.Object
        0x000c: check-cast v1, com.okinc.unify_trade.biz.StrategyDetailsResponse
        0x000e: invoke-static java.lang.System.currentTimeMillis():long
        0x0012: iput v2, v10, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.AkhnZx:long
        0x0014: iget v2, v10, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.AYXKKw:o.aajR
        0x0016: invoke-virtual com.okinc.unify_trade.biz.StrategyDetailsResponse.getAccountId():java.lang.Long
        0x001a: invoke-virtual o.aajR.KWHzl(java.lang.Long):boolean
        0x0024: invoke-static o.Hac.copydefault():int
        0x0028: invoke-static o.Hac.copydefault():int
        0x002c: invoke-static o.Hac.copydefault():int
        0x0030: invoke-static o.Hac.copydefault():int
        0x0034: const v9, 520186186
        0x0037: const v8, -520186176
        0x003a: invoke-static/range com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.copydefault(int, int, java.lang.Object[], int, int, int, int):java.lang.Object
        0x003e: check-cast v2, o.aajI
        0x0040: invoke-virtual com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.AEQbTJ(o.aajI):void
        0x0043: sget v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0049: sput v3, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x004c: invoke-virtual o.abQb.UeEOUB():com.okinc.unify_trade.refactor.presenter.AbsPresenter
        0x0050: check-cast v2, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter
        0x0052: invoke-virtual com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.gEmmrt():com.okinc.unify_trade.bot.data.TacticsData
        0x0058: sget v3, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x005e: sput v4, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0063: invoke-virtual com.okinc.unify_trade.bot.data.TacticsData.getOrderType():java.lang.String
        0x0069: sget v3, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x006f: sput v4, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0072: const-string v0, "contract_dca"
        0x0076: invoke-static kotlin.jvm.internal.Intrinsics.KWHzl(java.lang.Object, java.lang.Object):boolean
        0x007a: const/16 v3, 31
        0x0081: invoke-static kotlin.jvm.internal.Intrinsics.KWHzl(java.lang.Object, java.lang.Object):boolean
        0x0087: const-string v0, "spot_dca"
        0x0089: invoke-static kotlin.jvm.internal.Intrinsics.KWHzl(java.lang.Object, java.lang.Object):boolean
        0x0090: invoke-virtual com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.OLrzqt(o.abrD):void
        0x0094: invoke-virtual com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.OLrzqt(com.okinc.unify_trade.biz.StrategyDetailsResponse, o.abrD):void
        0x0098: invoke-virtual com.okinc.unify_trade.bot.data.TacticsData.getOrderType():java.lang.String
        0x009b: const/4 v10, 0
        0x009d: sget v10, kotlin.Unit.INSTANCE:kotlin.Unit
        0x009f: return-object v10
    }

    private final void KWHzl(androidx.fragment.app.Fragment p0) {
        0x0004: invoke-static o.Hac.copydefault():int
        0x0008: invoke-static o.Hac.copydefault():int
        0x000c: invoke-static o.Hac.copydefault():int
        0x0010: invoke-static o.Hac.copydefault():int
        0x0014: const v6, 415126634
        0x0017: const v5, -415126617
        0x001a: invoke-static/range com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.copydefault(int, int, java.lang.Object[], int, int, int, int):java.lang.Object
        0x001d: return-void
    }

    public static synthetic void KWHzl(kotlin.jvm.functions.Function1 p0, java.lang.Object p1) {
        0x0000: const/4 v0, 2
        0x0003: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0009: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x000c: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.djBIcL(kotlin.jvm.functions.Function1, java.lang.Object):void
        0x000f: sget v3, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0015: sput v4, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0018: return-void
    }

    public static synthetic void KWHzl(o.aaYh p0, android.view.View p1) {
        0x0000: const/4 v0, 2
        0x0003: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0009: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x000c: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.OLrzqt(o.aaYh, android.view.View):void
        0x000f: sget v3, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0015: sput v4, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0018: return-void
    }

    public static synthetic void KWHzl(com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity p0) {
        0x0000: const/4 v0, 2
        0x0003: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0009: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x000c: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.values(com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity):void
        0x000f: sget v3, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0015: sput v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x001a: const/16 v3, 43
        0x001e: return-void
    }

    private static synthetic java.lang.Object OLrzqt(java.lang.Object[] p0) {
        0x0000: const/4 v0, 0
        0x0003: check-cast v0, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity
        0x0005: const/4 v1, 1
        0x0008: check-cast v4, com.okinc.unify_trade.bot.data.GridPendingOrderData
        0x000a: const/4 v1, 2
        0x000d: sget v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0013: sput v3, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0016: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.KWHzl(com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity, com.okinc.unify_trade.bot.data.GridPendingOrderData):kotlin.Unit
        0x001a: sget v0, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0020: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0025: return-object v4
        0x0026: const/4 v4, 0
    }

    public static synthetic java.lang.String OLrzqt(com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity p0) {
        0x0000: const/4 v0, 2
        0x0003: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0009: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x000c: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.AYXKKw(com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity):java.lang.String
        0x0010: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0016: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x001b: return-object v3
        0x001c: const/4 v3, 0
    }

    public static synthetic kotlin.Unit OLrzqt(java.lang.Throwable p0) {
        0x0000: const/4 v0, 2
        0x0003: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0009: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x000c: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.djBIcL(java.lang.Throwable):kotlin.Unit
        0x0010: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0016: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x001b: return-object v3
        0x001c: const/4 v3, 0
        0x001d: invoke-virtual java.lang.Object.hashCode():int
    }

    public static final kotlin.Unit OLrzqt(o.PIG p0, com.okinc.share.bean.image.ImageShareParams p1) {
        0x0000: const/4 v0, 2
        0x0003: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0009: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x000c: const-string v1, ""
        0x000e: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0011: const-string v1, "BOT_ORDER_KLINE_SHARE_FROM"
        0x0013: invoke-virtual com.okinc.share.api.params.BaseShareParams.setShareFrom(java.lang.String):void
        0x0016: invoke-virtual o.PIG.AEQbTJ():java.lang.String
        0x001a: invoke-virtual com.okinc.share.api.params.BaseShareParams.setBody(java.lang.String):void
        0x001d: sget v3, kotlin.Unit.INSTANCE:kotlin.Unit
        0x001f: sget v4, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0025: sput v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0028: return-object v3
    }

    public static synthetic kotlin.Unit OLrzqt(com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity p0, androidx.core.util.Pair p1) {
        0x0000: const/4 v0, 2
        0x0003: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0009: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x000c: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.KWHzl(com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity, androidx.core.util.Pair):kotlin.Unit
        0x0012: const/16 v4, 99
        0x0016: sget v4, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x001c: sput v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0021: return-object v3
        0x0022: const/4 v3, 0
        0x0023: invoke-virtual java.lang.Object.hashCode():int
    }

    public static synthetic kotlin.Unit OLrzqt(com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity p0, com.okinc.unify_trade.bot.data.GridPendingOrderData p1) {
        0x0004: invoke-static o.Hac.copydefault():int
        0x0008: invoke-static o.Hac.copydefault():int
        0x000c: invoke-static o.Hac.copydefault():int
        0x0010: invoke-static o.Hac.copydefault():int
        0x0014: const v6, -2021313717
        0x0017: const v5, 2021313718
        0x001a: invoke-static/range com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.copydefault(int, int, java.lang.Object[], int, int, int, int):java.lang.Object
        0x001e: check-cast v7, kotlin.Unit
        0x0020: return-object v7
    }

    public static final kotlin.Unit OLrzqt(com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity p0, java.io.File p1) {
        0x0000: const/4 v0, 2
        0x0003: const-string v1, ""
        0x0005: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0008: sget v2, com.okinc.share.bean.image.ImageShareParams.Companion:com.okinc.share.bean.image.ImageShareParams$StateListAnimator
        0x000a: invoke-virtual java.io.File.getAbsolutePath():java.lang.String
        0x000e: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullExpressionValue(java.lang.Object, java.lang.String):void
        0x0011: new-instance v1, o.aajE
        0x0013: invoke-direct o.aajE.<init>():void
        0x0016: invoke-virtual com.okinc.share.bean.image.ImageShareParams$StateListAnimator.EZpvd(java.lang.String, kotlin.jvm.functions.Function1):com.okinc.share.bean.image.ImageShareParams
        0x001a: sget v1, com.okinc.share.bean.ShareConfig.Companion:com.okinc.share.bean.ShareConfig$StateListAnimator
        0x001c: const/4 v2, 0
        0x001d: invoke-static com.okinc.share.bean.ShareConfig$StateListAnimator.create$default(com.okinc.share.bean.ShareConfig$StateListAnimator, com.okinc.share.api.params.IShareParams, kotlin.jvm.functions.Function1, int, java.lang.Object):com.okinc.share.bean.ShareConfig
        0x0021: invoke-static o.POg.EZpvd():o.POe
        0x0025: invoke-interface o.POe.OLrzqt(o.uyI, com.okinc.share.bean.ShareConfig):o.PIE
        0x0028: sget v3, kotlin.Unit.INSTANCE:kotlin.Unit
        0x002a: sget v4, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0030: sput v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0035: return-object v3
    }

    public static synthetic kotlin.Unit OLrzqt(com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity p0, java.lang.String p1) {
        0x0000: const/4 v0, 2
        0x0003: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0009: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0010: invoke-static o.Hac.copydefault():int
        0x0014: invoke-static o.Hac.copydefault():int
        0x0018: invoke-static o.Hac.copydefault():int
        0x001c: invoke-static o.Hac.copydefault():int
        0x0020: const v8, 1802337805
        0x0023: const v7, -1802337797
        0x0026: invoke-static/range com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.copydefault(int, int, java.lang.Object[], int, int, int, int):java.lang.Object
        0x002a: check-cast v9, kotlin.Unit
        0x002c: sget v10, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0032: sput v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0035: return-object v9
    }

    public static synthetic kotlin.Unit OLrzqt(com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity p0, java.util.List p1) {
        0x0004: invoke-static o.Hac.copydefault():int
        0x0008: invoke-static o.Hac.copydefault():int
        0x000c: invoke-static o.Hac.copydefault():int
        0x0010: invoke-static o.Hac.copydefault():int
        0x0014: const v6, -1327886333
        0x0017: const v5, 1327886342
        0x001a: invoke-static/range com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.copydefault(int, int, java.lang.Object[], int, int, int, int):java.lang.Object
        0x001e: check-cast v7, kotlin.Unit
        0x0020: return-object v7
    }

    public static synthetic kotlin.Unit OLrzqt(com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity p0, kotlin.jvm.functions.Function0 p1, o.APx p2) {
        0x0000: const/4 v0, 2
        0x0003: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0009: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x000c: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.KWHzl(com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity, kotlin.jvm.functions.Function0, o.APx):kotlin.Unit
        0x0012: const/16 v4, 87
        0x0016: return-object v3
    }

    public static synthetic kotlin.Unit OLrzqt(com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity p0, o.abrD p1) {
        0x0000: const/4 v0, 2
        0x0003: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0009: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x000c: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.KWHzl(com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity, o.abrD):kotlin.Unit
        0x0010: sget v4, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0016: sput v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x001b: return-object v3
        0x001c: const/4 v3, 0
    }

    public static synthetic void OLrzqt(kotlin.jvm.functions.Function1 p0, java.lang.Object p1) {
        0x0004: invoke-static o.Hac.copydefault():int
        0x0008: invoke-static o.Hac.copydefault():int
        0x000c: invoke-static o.Hac.copydefault():int
        0x0010: invoke-static o.Hac.copydefault():int
        0x0014: const v6, -262085746
        0x0017: const v5, 262085757
        0x001a: invoke-static/range com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.copydefault(int, int, java.lang.Object[], int, int, int, int):java.lang.Object
        0x001d: return-void
    }

    public static final void OLrzqt(o.aaYh p0, android.view.View p1) {
        0x0000: const/4 v3, 2
        0x0009: sput v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x000c: invoke-virtual o.aaYh.OLrzqt():o.aaYh
        0x0011: const/16 v2, 31
        0x0015: return-void
    }

    public static final synthetic void OLrzqt(com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity p0, com.okinc.unify_trade.bot.data.BenefitDetails p1, java.lang.String p2) {
        0x0000: const/4 v0, 2
        0x0003: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0009: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x000c: invoke-virtual com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.OLrzqt(com.okinc.unify_trade.bot.data.BenefitDetails, java.lang.String):void
        0x000f: sget v3, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0015: sput v4, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x001a: return-void
        0x001b: const/4 v3, 0
        0x001c: invoke-virtual java.lang.Object.hashCode():int
    }

    private static void c(char[] p0, int p1, java.lang.Object[] p2) {
        0x0002: const/4 v1, 2
        0x0005: new-instance v2, o.OO
        0x0007: invoke-direct o.OO.<init>():void
        0x000b: new-array v3, char[]
        0x000d: const/4 v4, 0
        0x000e: iput v4, v2, o.OO.KWHzl:int
        0x0010: new-array v5, char[]
        0x0012: iget v6, v2, o.OO.KWHzl:int
        0x0017: iget v6, v2, o.OO.KWHzl:int
        0x001f: const/4 v7, 1
        0x0026: const-string/jumbo v111, ""
        0x002b: sput v8, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.$11:int
        0x002e: const v6, 58224
        0x0032: const/16 v9, 16
        0x0034: const/4 v10, 0
        0x0037: sget v9, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.$10:int
        0x003d: sput v11, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.$11:int
        0x0048: sget v14, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.fIwbmz:char
        0x004b: const-wide v16, -6053090142764936957
        0x0051: return-void
        0x0058: sget v13, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.getFieldNames:char
        0x005a: const/4 v14, 4
        0x005b: new-array v15, java.lang.Object[]
        0x005d: invoke-static java.lang.Integer.valueOf(int):java.lang.Integer
        0x0061: const/16 v18, 3
        0x0064: return v18
        0x0065: invoke-static java.lang.Integer.valueOf(int):java.lang.Integer
        0x006a: return v1
        0x006b: invoke-static java.lang.Integer.valueOf(int):java.lang.Integer
        0x0070: return v7
        0x0071: invoke-static java.lang.Integer.valueOf(int):java.lang.Integer
        0x0076: return v4
        0x0077: const v9, 1019547968
        0x007a: invoke-static o.OY.AEQbTJ(int):java.lang.Object
        0x007e: const/16 v12, 17
        0x0082: const v19, 38577
        0x0085: const/16 v20, 4
        0x0087: const/16 v21, 209
        0x0089: const v22, -1676111872
        0x008c: const/16 v23, 0
        0x0091: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.$$f(int, byte, int):java.lang.String
        0x0095: new-array v11, java.lang.Class[]
        0x0097: sget v12, java.lang.Integer.TYPE:java.lang.Class
        0x009b: sget v12, java.lang.Integer.TYPE:java.lang.Class
        0x009f: sget v12, java.lang.Integer.TYPE:java.lang.Class
        0x00a3: sget v12, java.lang.Integer.TYPE:java.lang.Class
        0x00a9: invoke-static/range o.OY.KWHzl(char, int, int, int, boolean, java.lang.String, java.lang.Class[]):java.lang.Object
        0x00ad: check-cast v11, java.lang.reflect.Method
        0x00af: invoke-virtual java.lang.reflect.Method.invoke(java.lang.Object, java.lang.Object[]):java.lang.Object
        0x00b3: check-cast v11, java.lang.Character
        0x00b5: invoke-virtual java.lang.Character.charValue():char
        0x00c1: sget v10, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.ejfBZ:char
        0x00cd: sget v11, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.fARcdN:char
        0x00cf: new-array v13, java.lang.Object[]
        0x00d1: invoke-static java.lang.Integer.valueOf(int):java.lang.Integer
        0x00d7: invoke-static java.lang.Integer.valueOf(int):java.lang.Integer
        0x00dd: invoke-static java.lang.Integer.valueOf(int):java.lang.Integer
        0x00e3: invoke-static java.lang.Integer.valueOf(int):java.lang.Integer
        0x00e9: const v9, 1019547968
        0x00ec: invoke-static o.OY.AEQbTJ(int):java.lang.Object
        0x00f2: const v26, 38577
        0x00f5: const/16 v27, 4
        0x00f7: const/16 v28, 209
        0x00f9: const v29, -1676111872
        0x00fc: const/16 v30, 0
        0x00fe: const/16 v9, 17
        0x0103: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.$$f(int, byte, int):java.lang.String
        0x0107: new-array v9, java.lang.Class[]
        0x0109: sget v10, java.lang.Integer.TYPE:java.lang.Class
        0x011b: invoke-static/range o.OY.KWHzl(char, int, int, int, boolean, java.lang.String, java.lang.Class[]):java.lang.Object
        0x011f: check-cast v9, java.lang.reflect.Method
        0x0121: const/4 v10, 0
        0x0122: invoke-virtual java.lang.reflect.Method.invoke(java.lang.Object, java.lang.Object[]):java.lang.Object
        0x0126: check-cast v9, java.lang.Character
        0x0128: invoke-virtual java.lang.Character.charValue():char
        0x0136: iget v6, v2, o.OO.KWHzl:int
        0x0145: new-instance v0, com.okinc.tradingbot.impl.market_place.home.list.ongoing.OngoingSelectionConfig$OngoingTypeSelection$Creator
        0x0147: const v8, -1573246999
        0x014a: invoke-static o.OY.AEQbTJ(int):java.lang.Object
        0x0150: const/16 v9, 24091
        0x0152: const/4 v10, 4
        0x0153: const/16 v11, 1341
        0x0155: const v12, 48735913
        0x0158: const/4 v13, 0
        0x0159: const-string v14, "g"
        0x015b: new-array v15, java.lang.Class[]
        0x015d: const-class v8, java.lang.Object
        0x0160: return v4
        0x0161: const-class v8, java.lang.Object
        0x0164: return v7
        0x0165: invoke-static/range o.OY.KWHzl(char, int, int, int, boolean, java.lang.String, java.lang.Class[]):java.lang.Object
        0x0169: check-cast v8, java.lang.reflect.Method
        0x016b: const/4 v7, 0
        0x016c: invoke-virtual java.lang.reflect.Method.invoke(java.lang.Object, java.lang.Object[]):java.lang.Object
        0x0172: invoke-virtual java.lang.Throwable.getCause():java.lang.Throwable
        0x017a: new-instance v0, java.lang.String
        0x017e: invoke-direct java.lang.String.<init>(char[], int, int):void
        0x0182: new-array v4, android.app.Application$ActivityLifecycleCallbacks
    }

    public static synthetic java.lang.Object copydefault(int p0, int p1, java.lang.Object[] p2, int p3, int p4, int p5, int p6) {
        0x0000: const v0, 1650805025
        0x0004: const/high16 v1, 0xeeba0000
        0x0007: const v1, -1513965855
        0x000f: const v2, -565098208
        0x0018: const v2, 565098208
        0x0023: const/high16 v2, 0x84140000
        0x0027: const/high16 v2, 0x6cb00000
        0x002b: const/high16 v2, 0x317c0000
        0x0032: const v4, 1132004924
        0x0037: const v4, -2047965933
        0x003d: const/high16 v4, 0xbbda0000
        0x0041: const v4, -767560105
        0x0045: const v4, -1188649921
        0x0049: const v4, -767559017
        0x0057: const v10, -767559561
        0x005c: const v8, 1544553956
        0x0061: const v8, -1468578859
        0x0066: const/high16 v7, 0x82560000
        0x006b: const/high16 v7, 0x84460000
        0x006f: const/4 v7, 0
        0x0070: const/4 v8, 2
        0x0074: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.AEQbTJ(java.lang.Object[]):java.lang.Object
        0x007a: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.fJNWhG(java.lang.Object[]):java.lang.Object
        0x0080: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.fARcdN(java.lang.Object[]):java.lang.Object
        0x0086: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.AuCTel(java.lang.Object[]):java.lang.Object
        0x008c: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.AkhnZx(java.lang.Object[]):java.lang.Object
        0x0092: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.DbNXlk(java.lang.Object[]):java.lang.Object
        0x0098: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.values(java.lang.Object[]):java.lang.Object
        0x009e: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.fetchVPNInfo(java.lang.Object[]):java.lang.Object
        0x00a4: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.isConnected(java.lang.Object[]):java.lang.Object
        0x00aa: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.djBIcL(java.lang.Object[]):java.lang.Object
        0x00ba: check-cast v9, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter
        0x00bc: invoke-virtual com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.KWHzl():java.lang.String
        0x00c0: invoke-virtual o.abQb.UeEOUB():com.okinc.unify_trade.refactor.presenter.AbsPresenter
        0x00c4: check-cast v9, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter
        0x00c6: invoke-virtual com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.OLrzqt():java.lang.String
        0x00ca: iget v9, v7, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.AYXKKw:o.aajR
        0x00cc: invoke-virtual o.aajR.OLrzqt():long
        0x00d0: new-instance v5, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity$legacyChartInputs$1
        0x00d2: invoke-direct com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity$legacyChartInputs$1.<init>(java.lang.Object):void
        0x00d5: new-instance v7, o.aajI
        0x00d8: invoke-direct/range o.aajI.<init>(java.lang.String, java.lang.String, long, kotlin.jvm.functions.Function0):void
        0x00db: sget v9, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x00e1: sput v10, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x00e6: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.gEmmrt(java.lang.Object[]):java.lang.Object
        0x00ec: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.valueOf(java.lang.Object[]):java.lang.Object
        0x00f2: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.AhwBna(java.lang.Object[]):java.lang.Object
        0x00f8: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.AYXKKw(java.lang.Object[]):java.lang.Object
        0x0108: check-cast v9, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter
        0x010a: invoke-virtual com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.gEmmrt():com.okinc.unify_trade.bot.data.TacticsData
        0x0110: sget v10, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0116: sput v11, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0119: invoke-virtual com.okinc.unify_trade.bot.data.TacticsData.getOrderType():java.lang.String
        0x011f: invoke-virtual java.lang.String.hashCode():int
        0x0123: const v11, -1831183611
        0x0128: sget v11, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x012e: sput v12, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0131: const v8, -1402017003
        0x0136: const v8, 3181382
        0x013c: const-string v8, "grid"
        0x013e: invoke-virtual java.lang.String.equals(java.lang.Object):boolean
        0x0145: const-string v8, "contract_dca"
        0x0147: invoke-virtual java.lang.String.equals(java.lang.Object):boolean
        0x014e: sget v8, o.absU.EZpvd:o.absU
        0x0150: invoke-virtual o.abQb.UeEOUB():com.okinc.unify_trade.refactor.presenter.AbsPresenter
        0x0154: check-cast v9, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter
        0x0156: invoke-virtual com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.AEQbTJ():java.lang.String
        0x015a: const-string v10, "futures"
        0x015c: invoke-virtual o.absU.KWHzl(java.lang.String, java.lang.String):void
        0x0160: const-string v8, "spot_dca"
        0x0162: invoke-virtual java.lang.String.equals(java.lang.Object):boolean
        0x0169: sget v8, o.absU.EZpvd:o.absU
        0x016b: invoke-virtual o.abQb.UeEOUB():com.okinc.unify_trade.refactor.presenter.AbsPresenter
        0x016f: check-cast v9, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter
        0x0171: invoke-virtual com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.AEQbTJ():java.lang.String
        0x0175: const-string v10, "spot"
        0x0177: invoke-virtual o.absU.KWHzl(java.lang.String, java.lang.String):void
        0x017a: invoke-virtual o.abQb.UeEOUB():com.okinc.unify_trade.refactor.presenter.AbsPresenter
        0x017e: check-cast v8, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter
        0x0180: invoke-virtual com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.KWHzl():java.lang.String
        0x0184: invoke-virtual o.abQb.UeEOUB():com.okinc.unify_trade.refactor.presenter.AbsPresenter
        0x0188: check-cast v9, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter
        0x018a: invoke-virtual com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.OLrzqt():java.lang.String
        0x0192: invoke-static o.Hac.copydefault():int
        0x0196: invoke-static o.Hac.copydefault():int
        0x019a: invoke-static o.Hac.copydefault():int
        0x019e: invoke-static o.Hac.copydefault():int
        0x01a2: const v6, -1951062827
        0x01a5: const v5, 1951062839
        0x01a8: invoke-static/range com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.copydefault(int, int, java.lang.Object[], int, int, int, int):java.lang.Object
        0x01ab: const/4 v7, 0
        0x01ad: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.KWHzl(java.lang.Object[]):java.lang.Object
        0x01b2: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.EZpvd(java.lang.Object[]):java.lang.Object
        0x01b7: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.copydefault(java.lang.Object[]):java.lang.Object
        0x01bc: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.OLrzqt(java.lang.Object[]):java.lang.Object
        0x01c0: return-object v7
    }

    private static synthetic java.lang.Object copydefault(java.lang.Object[] p0) {
        0x0000: const/4 v0, 0
        0x0005: const/4 v1, 2
        0x0008: invoke-virtual o.abQb.RcXXUw():androidx.databinding.ViewDataBinding
        0x000c: check-cast v2, o.XBx
        0x000e: iget v2, v2, o.XBx.AkhnZx:androidx.appcompat.widget.AppCompatTextView
        0x0010: const-string v3, ""
        0x0012: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullExpressionValue(java.lang.Object, java.lang.String):void
        0x0015: iget v6, v6, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.copydefault:o.abuj
        0x0017: const/4 v3, 0
        0x001a: invoke-virtual o.abuj.fetchVPNInfo():com.okinc.unify_trade.biz.BizInstrument
        0x0020: invoke-virtual com.okinc.unify_trade.biz.BizInstrument.isPreMarketPair():boolean
        0x0024: const/4 v4, 1
        0x0027: invoke-static androidx.core.view.ViewCompat.isLaidOut(android.view.View):boolean
        0x002c: const/4 v5, 7
        0x002f: sget v6, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0035: sput v4, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x003a: invoke-virtual android.view.View.isLayoutRequested():boolean
        0x0043: invoke-virtual android.view.View.isLayoutRequested():boolean
        0x0049: invoke-virtual android.widget.TextView.getPaint():android.text.TextPaint
        0x004d: invoke-virtual androidx.appcompat.widget.AppCompatTextView.getText():java.lang.CharSequence
        0x0051: invoke-virtual java.lang.Object.toString():java.lang.String
        0x0055: invoke-virtual android.graphics.Paint.measureText(java.lang.String):float
        0x0059: invoke-virtual android.view.View.getWidth():int
        0x0062: sget v6, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0068: sput v0, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x006b: const/high16 v6, 0x41200000
        0x006d: invoke-virtual android.widget.TextView.setTextSize(float):void
        0x0070: invoke-virtual android.widget.TextView.setMaxLines(int):void
        0x0073: return-object v3
        0x0074: new-instance v6, o.aajh$ActionBar
        0x0076: invoke-direct o.aajh$ActionBar.<init>(androidx.appcompat.widget.AppCompatTextView):void
        0x0079: invoke-virtual android.view.View.addOnLayoutChangeListener(android.view.View$OnLayoutChangeListener):void
        0x007c: sget v6, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0081: sput v0, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0084: return-object v3
    }

    public static synthetic kotlin.Unit copydefault() {
        0x0000: const/4 v0, 2
        0x0003: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0009: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x000e: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.djBIcL():kotlin.Unit
        0x0012: sget v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0018: sput v3, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x001b: return-object v1
        0x001c: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.djBIcL():kotlin.Unit
        0x001f: const/4 v0, 0
        0x0020: invoke-virtual java.lang.Object.hashCode():int
    }

    public static final kotlin.Unit copydefault(com.okinc.share.bean.image.ImageShareParams p0) {
        0x0000: const/4 v0, 2
        0x0003: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0009: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x000c: const-string v1, ""
        0x000e: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0011: const-string v1, "BOT_ORDER_KLINE_SHARE_FROM"
        0x0013: invoke-virtual com.okinc.share.api.params.BaseShareParams.setShareFrom(java.lang.String):void
        0x0016: sget v3, kotlin.Unit.INSTANCE:kotlin.Unit
        0x0018: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x001e: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0021: return-object v3
    }

    public static final kotlin.Unit copydefault(java.lang.Throwable p0) {
        0x0000: const/4 v0, 2
        0x0003: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0009: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x000c: const-string v0, ""
        0x0010: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0013: sget v3, kotlin.Unit.INSTANCE:kotlin.Unit
        0x0015: return-object v3
        0x0016: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0019: sget v3, kotlin.Unit.INSTANCE:kotlin.Unit
        0x001b: const/4 v3, 0
        0x001c: invoke-virtual java.lang.Object.hashCode():int
    }

    public static final kotlin.Unit copydefault(o.EJo p0) {
        0x0004: invoke-static o.Hac.copydefault():int
        0x0008: invoke-static o.Hac.copydefault():int
        0x000c: invoke-static o.Hac.copydefault():int
        0x0010: invoke-static o.Hac.copydefault():int
        0x0014: const v6, 989395512
        0x0017: const v5, -989395496
        0x001a: invoke-static/range com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.copydefault(int, int, java.lang.Object[], int, int, int, int):java.lang.Object
        0x001e: check-cast v7, kotlin.Unit
        0x0020: return-object v7
    }

    public static synthetic kotlin.Unit copydefault(com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity p0) {
        0x0000: const/4 v0, 2
        0x0003: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0009: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x000e: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.fetchVPNInfo(com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity):kotlin.Unit
        0x0012: return-object v3
        0x0013: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.fetchVPNInfo(com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity):kotlin.Unit
        0x0016: const/4 v3, 0
        0x0017: invoke-virtual java.lang.Object.hashCode():int
    }

    public static final kotlin.Unit copydefault(com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity p0, java.lang.String p1) {
        0x0000: const/4 v3, 2
        0x0009: sput v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x000c: invoke-static o.AtZ.OLrzqt():o.adaP
        0x0010: invoke-interface o.adaP.getValue():java.lang.Object
        0x0014: check-cast v0, o.Aqk
        0x0016: invoke-interface o.Aqk.djBIcL():boolean
        0x001a: invoke-virtual com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.OLrzqt(boolean):void
        0x001d: sget v2, kotlin.Unit.INSTANCE:kotlin.Unit
        0x001f: sget v0, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0025: sput v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x002a: const/16 v3, 59
        0x002e: return-object v2
    }

    public static final kotlin.Unit copydefault(com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity p0, o.PIG p1, java.io.File p2) {
        0x0000: const-string v0, ""
        0x0002: const/4 v1, 2
        0x0005: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0008: sget v2, com.okinc.share.bean.image.ImageShareParams.Companion:com.okinc.share.bean.image.ImageShareParams$StateListAnimator
        0x000a: invoke-virtual java.io.File.getAbsolutePath():java.lang.String
        0x000e: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullExpressionValue(java.lang.Object, java.lang.String):void
        0x0011: new-instance v0, o.aajG
        0x0013: invoke-direct o.aajG.<init>(o.PIG):void
        0x0016: invoke-virtual com.okinc.share.bean.image.ImageShareParams$StateListAnimator.EZpvd(java.lang.String, kotlin.jvm.functions.Function1):com.okinc.share.bean.image.ImageShareParams
        0x001a: sget v5, com.okinc.share.bean.ShareConfig.Companion:com.okinc.share.bean.ShareConfig$StateListAnimator
        0x001c: const/4 v0, 0
        0x001d: invoke-static com.okinc.share.bean.ShareConfig$StateListAnimator.create$default(com.okinc.share.bean.ShareConfig$StateListAnimator, com.okinc.share.api.params.IShareParams, kotlin.jvm.functions.Function1, int, java.lang.Object):com.okinc.share.bean.ShareConfig
        0x0021: invoke-static o.POg.EZpvd():o.POe
        0x0025: invoke-interface o.POe.OLrzqt(o.uyI, com.okinc.share.bean.ShareConfig):o.PIE
        0x0028: sget v3, kotlin.Unit.INSTANCE:kotlin.Unit
        0x002a: sget v4, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0030: sput v5, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0035: return-object v3
        0x0036: invoke-virtual java.lang.Object.hashCode():int
    }

    public static synthetic void copydefault(kotlin.jvm.functions.Function1 p0, java.lang.Object p1) {
        0x0000: const/4 v0, 2
        0x0003: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0009: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0012: invoke-static o.Hac.copydefault():int
        0x0016: invoke-static o.Hac.copydefault():int
        0x001a: invoke-static o.Hac.copydefault():int
        0x001e: invoke-static o.Hac.copydefault():int
        0x0022: const v8, -729889287
        0x0025: const v7, 729889302
        0x0028: invoke-static/range com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.copydefault(int, int, java.lang.Object[], int, int, int, int):java.lang.Object
        0x002b: const/16 v9, 92
        0x0034: invoke-static o.Hac.copydefault():int
        0x0038: invoke-static o.Hac.copydefault():int
        0x003c: invoke-static o.Hac.copydefault():int
        0x0040: invoke-static o.Hac.copydefault():int
        0x0044: const v7, -729889287
        0x0047: const v6, 729889302
        0x004a: invoke-static/range com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.copydefault(int, int, java.lang.Object[], int, int, int, int):java.lang.Object
        0x004d: sget v9, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0053: sput v10, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0056: return-void
    }

    private static void d(int p0, char[] p1, byte p2, java.lang.Object[] p3) {
        0x0002: const/4 v1, 2
        0x0005: new-instance v2, o.OR
        0x0007: invoke-direct o.OR.<init>():void
        0x000a: sget v3, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.getNewProxyInstance:char[]
        0x000c: const v4, 540927370
        0x000f: const/4 v5, 0
        0x0010: const/4 v6, 0
        0x0011: const/4 v7, 1
        0x0015: new-array v9, char[]
        0x001e: invoke-static java.lang.Integer.valueOf(int):java.lang.Integer
        0x0024: invoke-static o.OY.AEQbTJ(int):java.lang.Object
        0x002a: const/4 v13, 0
        0x002b: const/16 v14, 20
        0x002d: const/16 v15, 436
        0x002f: const v16, -2132777782
        0x0032: const/16 v17, 0
        0x0037: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.$$f(int, byte, int):java.lang.String
        0x003b: new-array v1, java.lang.Class[]
        0x003d: sget v4, java.lang.Integer.TYPE:java.lang.Class
        0x0043: invoke-static/range o.OY.KWHzl(char, int, int, int, boolean, java.lang.String, java.lang.Class[]):java.lang.Object
        0x0047: check-cast v11, java.lang.reflect.Method
        0x0049: invoke-virtual java.lang.reflect.Method.invoke(java.lang.Object, java.lang.Object[]):java.lang.Object
        0x004d: check-cast v1, java.lang.Character
        0x004f: invoke-virtual java.lang.Character.charValue():char
        0x0057: const/4 v1, 2
        0x0058: const v4, 540927370
        0x005d: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.iwGUEr:char
        0x005f: new-array v4, java.lang.Object[]
        0x0061: invoke-static java.lang.Integer.valueOf(int):java.lang.Integer
        0x0067: const v1, 540927370
        0x006a: invoke-static o.OY.AEQbTJ(int):java.lang.Object
        0x0070: const/4 v8, 0
        0x0071: const/16 v9, 20
        0x0073: const/16 v10, 436
        0x0075: const v11, -2132777782
        0x0078: const/4 v12, 0
        0x007c: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.$$f(int, byte, int):java.lang.String
        0x0080: new-array v14, java.lang.Class[]
        0x0082: sget v1, java.lang.Integer.TYPE:java.lang.Class
        0x0085: return-void
        0x0086: invoke-static/range o.OY.KWHzl(char, int, int, int, boolean, java.lang.String, java.lang.Class[]):java.lang.Object
        0x008a: check-cast v1, java.lang.reflect.Method
        0x008c: invoke-virtual java.lang.reflect.Method.invoke(java.lang.Object, java.lang.Object[]):java.lang.Object
        0x0090: check-cast v1, java.lang.Character
        0x0092: invoke-virtual java.lang.Character.charValue():char
        0x0096: new-array v4, char[]
        0x009f: check-cast v8, com.okinc.tradingbot.impl.order.strategy.smartarbitrage.presenter.SmartArbitrageListSorter$TaskDescription
        0x00a9: iput v6, v2, o.OR.AEQbTJ:int
        0x00ab: iget v9, v2, o.OR.AEQbTJ:int
        0x00af: iget v9, v2, o.OR.AEQbTJ:int
        0x00b2: check-cast v9, com.okinc.tradingbot.impl.order.strategy.smartarbitrage.fragment.SmartArbCustomOrderRustFragment
        0x00b4: iget v11, v1, com.okinc.tradingbot.impl.contractdca.ui.BotContractDcaEditConfirmDialog.KWHzl:boolean
        0x00b6: iget v11, v1, androidx.core.widget.TextViewCompat.setTextAppearance:o.Ytc
        0x00b9: check-cast v9, com.okinc.tradingbot.impl.order.strategy.smartarbitrage.fragment.SmartArbCustomOrderRustFragment
        0x00bb: iget v11, v1, com.okinc.tradingbot.impl.contractdca.syncfollower.ContractDcaSyncFollowerViewModelImpl.AEQbTJ:java.lang.String
        0x00bd: iget v11, v1, o.XBS.KWHzl:android.util.SparseIntArray
        0x00bf: iget v11, v1, com.okinc.tradingbot.impl.order.strategy.withdraw.smartarbitrage.SmartArbWithdrawProfitsViewModel.QKVWgx:com.okinc.tradingbot.impl.nmp.home.NmpBotSquareFragment$TaskDescription
        0x00c1: const-wide v0, 1970371390583679314
        0x00cb: iget v9, v2, o.OR.AEQbTJ:int
        0x00ce: iget v10, v2, o.OR.EZpvd:char
        0x00d8: const/16 v9, 13
        0x00da: new-array v10, java.lang.Object[]
        0x00dc: const/16 v11, 12
        0x00e0: invoke-static java.lang.Integer.valueOf(int):java.lang.Integer
        0x00e4: const/16 v13, 11
        0x00e8: const/16 v12, 10
        0x00ec: const/16 v14, 9
        0x00f0: invoke-static java.lang.Integer.valueOf(int):java.lang.Integer
        0x00f4: const/16 v16, 8
        0x00f8: const/4 v15, 7
        0x00fb: const/16 v17, 6
        0x00ff: invoke-static java.lang.Integer.valueOf(int):java.lang.Integer
        0x0103: const/16 v19, 5
        0x0107: const/16 v18, 4
        0x010b: const/16 v21, 3
        0x010f: invoke-static java.lang.Integer.valueOf(int):java.lang.Integer
        0x0113: const/16 v20, 2
        0x011b: const v22, -744298197
        0x011e: invoke-static/range o.OY.AEQbTJ(int):java.lang.Object
        0x0124: const v23, 56960
        0x0127: const/16 v24, 4
        0x0129: const/16 v25, 597
        0x012b: const v26, 1937729643
        0x012e: const/16 v27, 0
        0x0130: sget v22, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.$$e:int
        0x0139: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.$$f(int, byte, int):java.lang.String
        0x013d: new-array v9, java.lang.Class[]
        0x013f: const-class v11, java.lang.Object
        0x0149: const/4 v13, 2
        0x016e: const/16 v13, 11
        0x0174: const/16 v13, 12
        0x017c: invoke-static/range o.OY.KWHzl(char, int, int, int, boolean, java.lang.String, java.lang.Class[]):java.lang.Object
        0x0182: check-cast v5, java.lang.reflect.Method
        0x0184: const/4 v9, 0
        0x0185: invoke-virtual java.lang.reflect.Method.invoke(java.lang.Object, java.lang.Object[]):java.lang.Object
        0x0189: check-cast v5, java.lang.Integer
        0x018b: invoke-virtual java.lang.Integer.intValue():int
        0x018f: iget v9, v2, o.OR.djBIcL:int
        0x0193: sget v5, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.$11:int
        0x0199: sput v9, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.$10:int
        0x019b: const/4 v9, 2
        0x019d: const/16 v5, 11
        0x019f: new-array v9, java.lang.Object[]
        0x01ab: invoke-static java.lang.Integer.valueOf(int):java.lang.Integer
        0x01c5: const/4 v10, 2
        0x01ce: sput v5, com.okinc.tradingbot.impl.order.strategy.recurring.presenter.RecurringBuyModifyInvestmentViewModel.fIwbmz:o.adaP
        0x01d0: return-wide v72
        0x01d5: const/16 v22, 0
        0x01d7: const/16 v23, 4
        0x01d9: const/16 v24, 275
        0x01db: const v25, -1514768433
        0x01de: const/16 v26, 0
        0x01e0: const/4 v5, 2
        0x01e6: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.$$f(int, byte, int):java.lang.String
        0x01ea: const/16 v5, 11
        0x01ec: new-array v5, java.lang.Class[]
        0x01ee: const-class v10, java.lang.Object
        0x01f7: const-wide v27, 748172604372495122
        0x01fc: const-wide v27, 1053290478579288653
        0x0208: const-wide v27, 1952321854773725773
        0x0214: const-wide v27, 1053290478461848141
        0x021e: return-object v72
        0x021f: const-wide/16 v0, 1292
        0x0221: check-cast v5, java.lang.reflect.Method
        0x0223: const/4 v10, 0
        0x0224: invoke-virtual java.lang.reflect.Method.invoke(java.lang.Object, java.lang.Object[]):java.lang.Object
        0x0228: check-cast v5, java.lang.Integer
        0x022a: invoke-virtual java.lang.Integer.intValue():int
        0x022e: iget v9, v2, o.OR.KWHzl:int
        0x0231: iget v11, v2, o.OR.djBIcL:int
        0x0234: iget v11, v2, o.OR.AEQbTJ:int
        0x023a: iget v5, v2, o.OR.AEQbTJ:int
        0x0241: sget v5, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.$10:int
        0x0247: sput v9, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.$11:int
        0x0249: const/4 v9, 2
        0x024c: const/4 v10, 0
        0x024d: iget v5, v2, o.OR.copydefault:int
        0x024f: iget v9, v2, o.OR.KWHzl:int
        0x0253: iget v5, v2, o.OR.valueOf:int
        0x0258: iput v5, v2, o.OR.valueOf:int
        0x025a: iget v5, v2, o.OR.djBIcL:int
        0x025f: iput v5, v2, o.OR.djBIcL:int
        0x0261: iget v5, v2, o.OR.copydefault:int
        0x0264: iget v9, v2, o.OR.valueOf:int
        0x0267: iget v9, v2, o.OR.KWHzl:int
        0x026a: iget v11, v2, o.OR.djBIcL:int
        0x026d: iget v11, v2, o.OR.AEQbTJ:int
        0x0273: iget v5, v2, o.OR.AEQbTJ:int
        0x027b: iget v5, v2, o.OR.copydefault:int
        0x027e: iget v9, v2, o.OR.djBIcL:int
        0x0281: iget v9, v2, o.OR.KWHzl:int
        0x0284: iget v11, v2, o.OR.valueOf:int
        0x0287: iget v11, v2, o.OR.AEQbTJ:int
        0x028d: iget v5, v2, o.OR.AEQbTJ:int
        0x0294: iget v5, v2, o.OR.AEQbTJ:int
        0x0296: const/4 v9, 2
        0x0298: iput v5, v2, o.OR.AEQbTJ:int
        0x029d: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.$10:int
        0x02a3: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.$11:int
        0x02a5: const/4 v2, 2
        0x02b4: new-instance v0, java.lang.String
        0x02b6: invoke-direct java.lang.String.<init>(char[]):void
        0x02bb: return-void
        0x02bd: invoke-virtual java.lang.Throwable.getCause():java.lang.Throwable
    }

    public static final synthetic com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter djBIcL(com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity p0) {
        0x0000: const/4 v0, 2
        0x0003: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0009: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x000c: invoke-virtual o.abQb.UeEOUB():com.okinc.unify_trade.refactor.presenter.AbsPresenter
        0x0010: check-cast v3, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter
        0x0014: const/16 v1, 75
        0x0018: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x001e: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0023: const/16 v0, 64
        0x0027: return-object v3
    }

    private static synthetic java.lang.Object djBIcL(java.lang.Object[] p0) {
        0x0000: const/4 v0, 0
        0x0003: check-cast v0, kotlin.jvm.functions.Function1
        0x0005: const/4 v1, 1
        0x0009: check-cast v1, java.lang.Object
        0x000b: const/4 v1, 2
        0x000e: sget v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0014: sput v3, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0017: const/4 v1, 0
        0x0018: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.AYXKKw(kotlin.jvm.functions.Function1, java.lang.Object):void
        0x001d: return-object v1
        0x001e: invoke-virtual java.lang.Object.hashCode():int
    }

    public static final kotlin.Unit djBIcL() {
        0x0000: const/4 v0, 2
        0x0003: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0009: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x000c: const-string v1, "bot_kline_setting_tips_shown"
        0x000e: sget v2, java.lang.Boolean.TRUE:java.lang.Boolean
        0x0010: invoke-static com.okinc.core.util.SPUtils.put(java.lang.String, java.lang.Object):void
        0x0013: sget v1, kotlin.Unit.INSTANCE:kotlin.Unit
        0x0015: sget v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x001b: sput v3, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0020: return-object v1
        0x0021: const/4 v0, 0
        0x0022: invoke-virtual java.lang.Object.hashCode():int
    }

    public static final kotlin.Unit djBIcL(java.lang.Throwable p0) {
        0x0000: const/4 v0, 2
        0x0003: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0009: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x000e: invoke-virtual java.lang.Throwable.printStackTrace():void
        0x0011: sget v3, kotlin.Unit.INSTANCE:kotlin.Unit
        0x0013: const/16 v1, 70
        0x0018: invoke-virtual java.lang.Throwable.printStackTrace():void
        0x001b: sget v3, kotlin.Unit.INSTANCE:kotlin.Unit
        0x001d: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0023: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0028: return-object v3
        0x0029: const/4 v3, 0
    }

    public static final void djBIcL(kotlin.jvm.functions.Function1 p0, java.lang.Object p1) {
        0x0000: const/4 v0, 2
        0x0003: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0009: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x000c: invoke-interface kotlin.jvm.functions.Function1.invoke(java.lang.Object):java.lang.Object
        0x000f: sget v3, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0015: sput v4, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x001a: return-void
        0x001b: const/4 v3, 0
    }

    private static synthetic java.lang.Object fARcdN(java.lang.Object[] p0) {
        0x0000: const/4 v0, 0
        0x0005: const/4 v1, 2
        0x0008: const-string v2, "kline_full_screen"
        0x000e: invoke-static com.okinc.rxutils.RxBus.EZpvd(java.lang.String[]):o.acTJ
        0x0012: const-string v3, ""
        0x0014: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullExpressionValue(java.lang.Object, java.lang.String):void
        0x0017: invoke-static o.acRQ.AEQbTJ(o.acTJ, androidx.lifecycle.LifecycleOwner):o.acTJ
        0x001b: new-instance v4, o.aajg
        0x001d: new-instance v5, o.aajf
        0x001f: invoke-direct o.aajf.<init>(com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity):void
        0x0022: invoke-direct o.aajg.<init>(kotlin.jvm.functions.Function1):void
        0x0025: invoke-virtual o.acTJ.OLrzqt(o.acUA):o.acUm
        0x0028: const-string v2, "kline_portrait_full_screen"
        0x002e: invoke-static com.okinc.rxutils.RxBus.EZpvd(java.lang.String[]):o.acTJ
        0x0032: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullExpressionValue(java.lang.Object, java.lang.String):void
        0x0035: invoke-static o.acRQ.AEQbTJ(o.acTJ, androidx.lifecycle.LifecycleOwner):o.acTJ
        0x0039: new-instance v3, o.aajA
        0x003b: new-instance v4, o.aajp
        0x003d: invoke-direct o.aajp.<init>(com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity):void
        0x0040: invoke-direct o.aajA.<init>(kotlin.jvm.functions.Function1):void
        0x0043: invoke-virtual o.acTJ.OLrzqt(o.acUA):o.acUm
        0x0046: sget v6, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x004c: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x004f: const/4 v1, 0
        0x0052: const/16 v6, 90
        0x0055: return-object v1
    }

    private static synthetic java.lang.Object fJNWhG(java.lang.Object[] p0) {
        0x0000: const/4 v0, 0
        0x0003: check-cast v0, kotlin.jvm.functions.Function1
        0x0005: const/4 v1, 1
        0x0009: check-cast v1, java.lang.Object
        0x000b: const/4 v1, 2
        0x000e: sget v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0014: sput v3, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0017: invoke-interface kotlin.jvm.functions.Function1.invoke(java.lang.Object):java.lang.Object
        0x001a: sget v4, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0020: sput v0, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0023: const/4 v0, 0
        0x0026: return-object v0
        0x0027: invoke-virtual java.lang.Object.hashCode():int
    }

    private static synthetic java.lang.Object fetchVPNInfo(java.lang.Object[] p0) {
        0x0000: const/4 v0, 0
        0x0002: instance-of v0, androidx.compose.runtime.IntState
        0x0005: const/4 v2, 2
        0x000e: sput v4, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0011: invoke-virtual o.abQb.RcXXUw():androidx.databinding.ViewDataBinding
        0x0015: check-cast v3, o.XBx
        0x0017: iget v4, v3, o.XBx.AEQbTJ:android.widget.LinearLayout
        0x0019: const-string v5, ""
        0x001b: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullExpressionValue(java.lang.Object, java.lang.String):void
        0x001e: iget v5, v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.KWHzl:boolean
        0x0024: const/16 v5, 8
        0x0026: invoke-virtual android.view.View.setVisibility(int):void
        0x0029: iget v4, v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.KWHzl:boolean
        0x002b: const/4 v5, 0
        0x002e: iget v0, v3, o.XBx.OLrzqt:o.APC
        0x0030: sget v1, o.APu.Companion:o.APu$Activity
        0x0032: invoke-virtual o.APu$Activity.OLrzqt():o.APu
        0x0036: invoke-virtual o.APC.KWHzl(o.APu):void
        0x0039: return-object v5
        0x003a: sget v7, o.WSQ$StateListAnimator.processStrongAuthMessage:int
        0x003c: sget v4, o.WSQ$PendingIntent.zlvcHA:int
        0x003e: invoke-static o.uAq.AYXKKw(int):java.lang.String
        0x0042: sget v9, o.aaVP$Application.DCUTEIddSDPG:int
        0x0044: new-instance v4, o.APx
        0x0046: const/4 v10, 0
        0x0047: const/4 v11, 0
        0x0048: const/4 v12, 0
        0x0049: const/4 v13, 0
        0x004a: const/4 v14, 0
        0x004b: const/4 v15, 0
        0x004c: const/16 v16, 0
        0x004e: const/16 v17, 1016
        0x0050: const/16 v18, 0
        0x0053: invoke-direct/range o.APx.<init>(int, java.lang.CharSequence, int, int, com.okinc.kline.ui.view.KlineBottomActionBarButtonStyle, boolean, float, boolean, boolean, java.lang.CharSequence, int, kotlin.jvm.internal.DefaultConstructorMarker):void
        0x0056: sget v20, o.WSQ$StateListAnimator.init:int
        0x0058: sget v6, o.WSQ$PendingIntent.fTEjYi:int
        0x005a: invoke-static o.uAq.AYXKKw(int):java.lang.String
        0x005e: sget v22, o.aaVP$Application.zUtmHU:int
        0x0060: new-instance v6, o.APx
        0x0062: const/16 v23, 0
        0x0064: const/16 v24, 0
        0x0066: const/16 v25, 0
        0x0068: const/16 v26, 0
        0x006a: const/16 v27, 0
        0x006c: const/16 v28, 0
        0x006e: const/16 v29, 0
        0x0070: const/16 v30, 1016
        0x0072: const/16 v31, 0
        0x0076: invoke-direct/range o.APx.<init>(int, java.lang.CharSequence, int, int, com.okinc.kline.ui.view.KlineBottomActionBarButtonStyle, boolean, float, boolean, boolean, java.lang.CharSequence, int, kotlin.jvm.internal.DefaultConstructorMarker):void
        0x0079: iget v7, v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.AhwBna:o.APx
        0x007b: const/4 v8, 3
        0x007c: new-array v8, o.APx[]
        0x0089: check-cast v0, java.lang.Iterable
        0x008b: new-instance v2, java.util.ArrayList
        0x008d: invoke-direct java.util.ArrayList.<init>():void
        0x0090: invoke-interface java.lang.Iterable.iterator():java.util.Iterator
        0x0094: invoke-interface java.util.Iterator.hasNext():boolean
        0x009a: invoke-interface java.util.Iterator.next():java.lang.Object
        0x009f: check-cast v6, o.APx
        0x00a1: invoke-virtual o.APx.djBIcL():boolean
        0x00a7: sget v6, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x00ad: sput v7, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x00b1: invoke-interface java.util.Collection.add(java.lang.Object):boolean
        0x00b5: iget v0, v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.gEmmrt:boolean
        0x00bb: invoke-static kotlin.collections.CollectionsKt.hDKMBd(java.lang.Iterable):java.util.List
        0x00c0: iget v0, v3, o.XBx.OLrzqt:o.APC
        0x00c2: iget v1, v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.gEmmrt:boolean
        0x00c6: sget v1, com.okinc.kline.ui.view.KlineBottomActionBarLayout.PRIMARY_START:com.okinc.kline.ui.view.KlineBottomActionBarLayout
        0x00c9: sget v1, com.okinc.kline.ui.view.KlineBottomActionBarLayout.PRIMARY_END:com.okinc.kline.ui.view.KlineBottomActionBarLayout
        0x00cc: sget v12, o.WSQ$StateListAnimator.hlXVux:int
        0x00ce: sget v1, o.WSQ$PendingIntent.fromMediaItemList:int
        0x00d0: invoke-static o.uAq.AYXKKw(int):java.lang.String
        0x00d4: new-instance v8, o.APx
        0x00d6: const/4 v14, 0
        0x00d7: const/4 v15, 0
        0x00d8: const/16 v16, 0
        0x00da: const/16 v17, 0
        0x00dc: const/16 v18, 0
        0x00de: const/16 v19, 0
        0x00e0: const/16 v20, 0
        0x00e2: const/16 v21, 0
        0x00e4: const/16 v22, 1020
        0x00e6: const/16 v23, 0
        0x00e9: invoke-direct/range o.APx.<init>(int, java.lang.CharSequence, int, int, com.okinc.kline.ui.view.KlineBottomActionBarButtonStyle, boolean, float, boolean, boolean, java.lang.CharSequence, int, kotlin.jvm.internal.DefaultConstructorMarker):void
        0x00ec: new-instance v1, o.APu
        0x00ee: const/4 v9, 0
        0x00ef: const/4 v11, 0
        0x00f0: const/16 v12, 20
        0x00f2: const/4 v13, 0
        0x00f4: invoke-direct/range o.APu.<init>(com.okinc.kline.ui.view.KlineBottomActionBarLayout, o.APx, java.util.List, java.util.List, boolean, int, kotlin.jvm.internal.DefaultConstructorMarker):void
        0x00f7: invoke-virtual o.APC.KWHzl(o.APu):void
        0x00fa: return-object v5
    }

    public static final kotlin.Unit fetchVPNInfo(com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity p0) {
        0x0000: const/4 v0, 2
        0x0003: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0009: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x000e: invoke-virtual/range o.abQb.UeEOUB():com.okinc.unify_trade.refactor.presenter.AbsPresenter
        0x0012: check-cast v1, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter
        0x0014: invoke-virtual com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.gEmmrt():com.okinc.unify_trade.bot.data.TacticsData
        0x0018: const/16 v2, 31
        0x001f: invoke-virtual/range o.abQb.UeEOUB():com.okinc.unify_trade.refactor.presenter.AbsPresenter
        0x0023: check-cast v1, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter
        0x0025: invoke-virtual com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.gEmmrt():com.okinc.unify_trade.bot.data.TacticsData
        0x002b: invoke-virtual com.okinc.unify_trade.bot.data.TacticsData.getAlgoId():java.lang.String
        0x0031: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0037: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x003a: const/4 v0, 0
        0x003d: sget v2, o.XlO.OLrzqt:o.XlO
        0x003f: invoke-virtual/range o.abQb.UeEOUB():com.okinc.unify_trade.refactor.presenter.AbsPresenter
        0x0043: check-cast v1, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter
        0x0045: invoke-virtual com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.gEmmrt():com.okinc.unify_trade.bot.data.TacticsData
        0x004b: invoke-virtual com.okinc.unify_trade.bot.data.TacticsData.getOrderType():java.lang.String
        0x0051: const-string v0, ""
        0x0054: invoke-virtual/range o.abQb.UeEOUB():com.okinc.unify_trade.refactor.presenter.AbsPresenter
        0x0058: check-cast v0, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter
        0x005a: invoke-virtual com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.gEmmrt():com.okinc.unify_trade.bot.data.TacticsData
        0x005e: const/4 v6, 0
        0x005f: const/4 v7, 0
        0x0060: const/4 v8, 0
        0x0061: const/4 v9, 0
        0x0062: const/4 v10, 0
        0x0063: const/4 v12, 0
        0x0064: const/4 v13, 0
        0x0065: const/16 v14, 1784
        0x0067: const/4 v15, 0
        0x006a: invoke-static/range o.XlO.openBotDetail$default(o.XlO, android.content.Context, java.lang.String, java.lang.String, boolean, java.lang.String, java.lang.String, java.lang.Integer, java.lang.String, java.lang.Object, boolean, boolean, int, java.lang.Object):void
        0x006e: sget v1, o.XlO.OLrzqt:o.XlO
        0x0070: invoke-virtual/range o.abQb.UeEOUB():com.okinc.unify_trade.refactor.presenter.AbsPresenter
        0x0074: check-cast v1, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter
        0x0076: invoke-virtual com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.gEmmrt():com.okinc.unify_trade.bot.data.TacticsData
        0x0079: invoke-virtual java.lang.Object.hashCode():int
        0x007d: sget v0, kotlin.Unit.INSTANCE:kotlin.Unit
        0x007f: return-object v0
    }

    private static synthetic java.lang.Object gEmmrt(java.lang.Object[] p0) {
        0x0000: const/4 v0, 0
        0x0005: const/4 v2, 1
        0x000a: const/4 v2, 2
        0x0013: sput v4, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0016: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.KWHzl(com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity, java.util.List):kotlin.Unit
        0x001a: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0020: sput v3, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0025: const/16 v1, 46
        0x0028: return-object v5
    }

    public static final void gEmmrt(kotlin.jvm.functions.Function1 p0, java.lang.Object p1) {
        0x0000: const/4 v0, 2
        0x0003: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0009: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x000c: invoke-interface kotlin.jvm.functions.Function1.invoke(java.lang.Object):java.lang.Object
        0x000f: sget v3, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0015: sput v4, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x001a: return-void
        0x001b: const/4 v3, 0
        0x001c: invoke-virtual java.lang.Object.hashCode():int
    }

    public static final synthetic void gEmmrt(com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity p0) {
        0x0000: const/4 v0, 2
        0x0003: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0009: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x000c: invoke-virtual com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.AkhnZx():void
        0x0011: return-void
        0x0012: const/4 v3, 0
    }

    static void hDKMBd() {
        0x0000: const v0, 47153
        0x0003: sput v0, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.ejfBZ:char
        0x0005: const v0, 53116
        0x0008: sput v0, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.fARcdN:char
        0x000a: const/16 v0, 32496
        0x000c: sput v0, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.fIwbmz:char
        0x000e: const/16 v0, 18590
        0x0010: sput v0, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.getFieldNames:char
        0x0012: const/16 v0, 25
        0x0014: new-array v0, char[]
        0x0019: sput v0, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.getNewProxyInstance:char[]
        0x001b: const v0, 42713
        0x001e: sput v0, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.iwGUEr:char
        0x0020: return-void
    }

    private static synthetic java.lang.Object isConnected(java.lang.Object[] p0) {
        0x0000: const/4 v0, 0
        0x0005: const/4 v1, 1
        0x000a: const/4 v3, 2
        0x0013: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0016: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0019: const-class v4, o.WpS
        0x001b: invoke-static o.EIy.KWHzl(java.lang.Class):o.EIM
        0x001f: check-cast v4, o.WpS
        0x0021: const/4 v5, 0
        0x0024: sget v6, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x002a: sput v7, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x002f: sget v6, com.okinc.tradeapi.framework.constant.TradeKey.MANAGER:com.okinc.tradeapi.framework.constant.TradeKey
        0x0031: invoke-interface o.WpS.EZpvd(com.okinc.tradeapi.framework.constant.TradeKey):o.WpQ
        0x0035: check-cast v4, o.abTZ
        0x0038: sget v8, com.okinc.tradeapi.framework.constant.TradeKey.MANAGER:com.okinc.tradeapi.framework.constant.TradeKey
        0x003a: invoke-interface o.WpS.EZpvd(com.okinc.tradeapi.framework.constant.TradeKey):o.WpQ
        0x003e: check-cast v8, o.abTZ
        0x0044: invoke-interface o.abTZ.valueOf():boolean
        0x004a: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0050: sput v4, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0056: invoke-interface o.abTZ.AkhnZx():boolean
        0x005c: invoke-virtual com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.OLrzqt(java.lang.String, java.lang.String):void
        0x005f: return-object v5
    }

    public static final kotlin.Unit isConnected(com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity p0) {
        0x0000: const/4 v0, 2
        0x0003: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0009: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x000c: invoke-virtual o.abQb.UeEOUB():com.okinc.unify_trade.refactor.presenter.AbsPresenter
        0x0010: check-cast v0, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter
        0x0012: invoke-virtual com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.KWHzl():java.lang.String
        0x0018: invoke-virtual o.abQb.UeEOUB():com.okinc.unify_trade.refactor.presenter.AbsPresenter
        0x001c: check-cast v1, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter
        0x001e: invoke-virtual com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.OLrzqt():java.lang.String
        0x0022: invoke-virtual com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.copydefault(java.lang.String, java.lang.String):void
        0x0025: sget v3, kotlin.Unit.INSTANCE:kotlin.Unit
        0x0027: const/16 v0, 9
        0x002c: invoke-virtual o.abQb.UeEOUB():com.okinc.unify_trade.refactor.presenter.AbsPresenter
        0x0030: check-cast v1, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter
        0x0032: invoke-virtual com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.OLrzqt():java.lang.String
        0x0036: invoke-virtual com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.copydefault(java.lang.String, java.lang.String):void
        0x0039: sget v3, kotlin.Unit.INSTANCE:kotlin.Unit
        0x003b: return-object v3
    }

    private final void sSMYrx() {
        0x0004: invoke-static o.Hac.copydefault():int
        0x0008: invoke-static o.Hac.copydefault():int
        0x000c: invoke-static o.Hac.copydefault():int
        0x0010: invoke-static o.Hac.copydefault():int
        0x0014: const v6, 42588235
        0x0017: const v5, -42588228
        0x001a: invoke-static/range com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.copydefault(int, int, java.lang.Object[], int, int, int, int):java.lang.Object
        0x001d: return-void
    }

    private static synthetic java.lang.Object valueOf(java.lang.Object[] p0) {
        0x0000: const/4 v0, 0
        0x0003: check-cast v0, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity
        0x0005: const/4 v1, 1
        0x0008: check-cast v4, java.lang.String
        0x000a: const/4 v4, 2
        0x000d: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0013: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0016: const/4 v2, 0
        0x0017: invoke-virtual com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.iwGUEr():void
        0x001c: sget v0, kotlin.Unit.INSTANCE:kotlin.Unit
        0x001e: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0024: sput v3, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0029: return-object v0
        0x002b: sget v4, kotlin.Unit.INSTANCE:kotlin.Unit
        0x002d: invoke-virtual java.lang.Object.hashCode():int
    }

    public static final o.Bpy valueOf() {
        0x0000: const/4 v0, 2
        0x0003: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0009: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x000c: sget v1, o.uxF.AEQbTJ:o.uxF
        0x000e: invoke-virtual o.uxF.copydefault():android.app.Application
        0x0012: const-class v2, com.okinc.di.LocalizeDIModule$StateListAnimator
        0x0014: invoke-static o.acSd.EZpvd(java.lang.Object, java.lang.Class):java.lang.Object
        0x0018: check-cast v1, com.okinc.di.LocalizeDIModule$StateListAnimator
        0x001a: invoke-interface com.okinc.di.LocalizeDIModule$StateListAnimator.DCUTEIddSDPG():o.Bpy
        0x001e: sget v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0024: sput v3, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0029: return-object v1
        0x002a: const/4 v0, 0
    }

    public static final void valueOf(kotlin.jvm.functions.Function1 p0, java.lang.Object p1) {
        0x0004: invoke-static o.Hac.copydefault():int
        0x0008: invoke-static o.Hac.copydefault():int
        0x000c: invoke-static o.Hac.copydefault():int
        0x0010: invoke-static o.Hac.copydefault():int
        0x0014: const v6, -545550193
        0x0017: const v5, 545550212
        0x001a: invoke-static/range com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.copydefault(int, int, java.lang.Object[], int, int, int, int):java.lang.Object
        0x001d: return-void
    }

    public static final synthetic void valueOf(com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity p0) {
        0x0004: invoke-static o.Hac.copydefault():int
        0x0008: invoke-static o.Hac.copydefault():int
        0x000c: invoke-static o.Hac.copydefault():int
        0x0010: invoke-static o.Hac.copydefault():int
        0x0014: const v6, 1964978932
        0x0017: const v5, -1964978926
        0x001a: invoke-static/range com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.copydefault(int, int, java.lang.Object[], int, int, int, int):java.lang.Object
        0x001d: return-void
    }

    private static synthetic java.lang.Object values(java.lang.Object[] p0) {
        0x0000: const/4 v0, 0
        0x0003: check-cast v11, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity
        0x0005: const/4 v1, 2
        0x0008: sget v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x000e: sput v3, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0011: const/4 v3, 0
        0x0014: invoke-direct com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.AubY():void
        0x0017: invoke-direct com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.zuBGHE():void
        0x001a: invoke-direct com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.wlaJM():void
        0x001d: invoke-direct com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.AxsJAY():void
        0x0024: invoke-static com.appsflyer.internal.AFi1bSDK$3$$ExternalSyntheticLambda0.EZpvd():int
        0x0028: invoke-static o.Hac.copydefault():int
        0x002c: invoke-static o.Hac.copydefault():int
        0x0030: invoke-static o.Hac.copydefault():int
        0x0034: const v10, 487838831
        0x0037: const v9, -487838813
        0x003a: invoke-static/range com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.copydefault(int, int, java.lang.Object[], int, int, int, int):java.lang.Object
        0x003d: sget v2, o.absU.EZpvd:o.absU
        0x003f: invoke-virtual o.abQb.UeEOUB():com.okinc.unify_trade.refactor.presenter.AbsPresenter
        0x0043: check-cast v4, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter
        0x0045: invoke-virtual com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.AhwBna():java.lang.String
        0x0049: invoke-virtual o.absU.valueOf(java.lang.String):void
        0x004c: const/4 v2, 3
        0x004d: invoke-static o.Fzc.reportFullyDrawn$default(android.app.Activity, boolean, java.lang.String, int, java.lang.Object):void
        0x0051: invoke-direct com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.AubY():void
        0x0054: invoke-direct com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.zuBGHE():void
        0x0057: invoke-direct com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.wlaJM():void
        0x005a: invoke-direct com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.AxsJAY():void
        0x0061: invoke-static com.appsflyer.internal.AFi1bSDK$3$$ExternalSyntheticLambda0.EZpvd():int
        0x0065: invoke-static o.Hac.copydefault():int
        0x0069: invoke-static o.Hac.copydefault():int
        0x006d: invoke-static o.Hac.copydefault():int
        0x0071: const v10, 487838831
        0x0074: const v9, -487838813
        0x0077: invoke-static/range com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.copydefault(int, int, java.lang.Object[], int, int, int, int):java.lang.Object
        0x007a: sget v2, o.absU.EZpvd:o.absU
        0x007c: invoke-virtual o.abQb.UeEOUB():com.okinc.unify_trade.refactor.presenter.AbsPresenter
        0x0080: check-cast v4, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter
        0x0082: invoke-virtual com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.AhwBna():java.lang.String
        0x0086: invoke-virtual o.absU.valueOf(java.lang.String):void
        0x0089: const/4 v2, 1
        0x008a: invoke-static o.Fzc.reportFullyDrawn$default(android.app.Activity, boolean, java.lang.String, int, java.lang.Object):void
        0x008d: sget v11, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0093: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0098: const/16 v11, 87
        0x009b: return-object v3
    }

    public static final void values(com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity p0) {
        0x0000: const/4 v0, 2
        0x0003: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0009: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x000c: invoke-virtual o.abQb.RcXXUw():androidx.databinding.ViewDataBinding
        0x0010: check-cast v1, o.XBx
        0x0012: iget v1, v1, o.XBx.OLrzqt:o.APC
        0x0014: sget v2, o.WSQ$StateListAnimator.init:int
        0x0016: invoke-virtual o.APC.OLrzqt(int):android.view.View
        0x001e: invoke-virtual android.app.Activity.isFinishing():boolean
        0x0022: const/4 v3, 1
        0x0025: invoke-virtual android.app.Activity.isDestroyed():boolean
        0x002c: new-instance v2, o.aaYh
        0x002e: invoke-direct o.aaYh.<init>(android.content.Context):void
        0x0031: new-instance v4, o.aaYh$StateListAnimator
        0x0033: invoke-direct o.aaYh$StateListAnimator.<init>(android.view.View):void
        0x0036: const/4 v1, 3
        0x0037: invoke-virtual o.aaYh$StateListAnimator.OLrzqt(int):void
        0x003a: sget v1, o.WSQ$PendingIntent.describeContents:int
        0x003c: invoke-static o.uAq.AYXKKw(int):java.lang.String
        0x0040: invoke-virtual o.aaYh$StateListAnimator.KWHzl(java.lang.CharSequence):void
        0x0043: sget v1, o.WSQ$PendingIntent.isBrowsable:int
        0x0045: invoke-static o.uAq.AYXKKw(int):java.lang.String
        0x0049: invoke-virtual o.aaYh$StateListAnimator.OLrzqt(java.lang.CharSequence):void
        0x004c: invoke-virtual o.aaYh.AEQbTJ(o.aaYh$StateListAnimator):o.aaYh
        0x004f: invoke-virtual o.aaYh.OLrzqt(int):o.aaYh
        0x0052: const/high16 v4, 0xc0400000
        0x0054: const/4 v1, 0
        0x0055: invoke-static o.abip.dp2px$default(float, android.content.Context, int, java.lang.Object):int
        0x0059: invoke-virtual o.aaYh.AEQbTJ(int):o.aaYh
        0x005c: new-instance v4, o.aajm
        0x005e: invoke-direct o.aajm.<init>():void
        0x0061: invoke-virtual o.aaYh.OLrzqt(kotlin.jvm.functions.Function0):void
        0x0064: invoke-virtual o.aaYh.fARcdN():void
        0x0067: invoke-virtual o.aaYh.AkhnZx():o.aaXx
        0x006d: sget v1, o.WSQ$PendingIntent.getDescription:int
        0x006f: invoke-static o.uAq.AYXKKw(int):java.lang.String
        0x0073: invoke-virtual android.widget.TextView.setText(java.lang.CharSequence):void
        0x0076: invoke-virtual o.aaYh.AkhnZx():o.aaXx
        0x007c: new-instance v1, o.aajj
        0x007e: invoke-direct o.aajj.<init>(o.aaYh):void
        0x0081: invoke-virtual android.view.View.setOnClickListener(android.view.View$OnClickListener):void
        0x0084: invoke-virtual o.aaYh.values():o.aaXx
        0x008a: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0090: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0095: const/16 v0, 76
        0x0097: invoke-virtual android.view.View.setVisibility(int):void
        0x009b: const/16 v0, 8
        0x009e: return-void
    }

    private final void wlaJM() {
        0x0000: const/4 v0, 2
        0x0003: new-instance v1, o.aajB
        0x0005: invoke-direct o.aajB.<init>(com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity):void
        0x0008: new-instance v2, o.aajz
        0x000a: invoke-direct o.aajz.<init>(com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity):void
        0x000d: new-instance v3, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity$initListener$3
        0x000f: invoke-direct com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity$initListener$3.<init>(java.lang.Object):void
        0x0012: new-instance v4, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity$initListener$4
        0x0014: invoke-direct com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity$initListener$4.<init>(java.lang.Object):void
        0x0017: invoke-virtual com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.OLrzqt(kotlin.jvm.functions.Function0, kotlin.jvm.functions.Function0, kotlin.jvm.functions.Function0, kotlin.jvm.functions.Function0):void
        0x001a: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0020: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0025: return-void
        0x0026: const/4 v0, 0
        0x0027: invoke-virtual java.lang.Object.hashCode():int
    }

    private final o.Bpy zLjUOn() {
        0x0000: const/4 v0, 2
        0x0003: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0009: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x000c: iget v1, v4, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.djBIcL:o.adaP
        0x000e: invoke-interface o.adaP.getValue():java.lang.Object
        0x0012: check-cast v1, o.Bpy
        0x0014: sget v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x001a: sput v3, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x001d: return-object v1
    }

    private final void zuBGHE() {
        0x0000: const/4 v0, 2
        0x0003: sget v2, o.acat.AEQbTJ:o.acat
        0x0005: invoke-virtual o.abQb.UeEOUB():com.okinc.unify_trade.refactor.presenter.AbsPresenter
        0x0009: check-cast v1, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter
        0x000b: invoke-virtual com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.KWHzl():java.lang.String
        0x000f: invoke-virtual o.abQb.UeEOUB():com.okinc.unify_trade.refactor.presenter.AbsPresenter
        0x0013: check-cast v1, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter
        0x0015: invoke-virtual com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.OLrzqt():java.lang.String
        0x0019: const/4 v5, 0
        0x001a: const/4 v6, 0
        0x001b: const/16 v7, 12
        0x001d: const/4 v8, 0
        0x001e: invoke-static/range o.acat.getTitleByIdAndType$default(o.acat, java.lang.String, java.lang.String, boolean, boolean, int, java.lang.Object):java.lang.String
        0x0022: invoke-virtual com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.KWHzl(java.lang.String):void
        0x0025: invoke-virtual o.abQb.RcXXUw():androidx.databinding.ViewDataBinding
        0x0029: check-cast v1, o.XBx
        0x002b: iget v1, v1, o.XBx.AYXKKw:android.widget.ImageView
        0x002d: const/4 v2, 0
        0x002e: invoke-virtual android.widget.ImageView.setVisibility(int):void
        0x0031: invoke-virtual o.abQb.UeEOUB():com.okinc.unify_trade.refactor.presenter.AbsPresenter
        0x0035: check-cast v1, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter
        0x0037: invoke-virtual com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.gEmmrt():com.okinc.unify_trade.bot.data.TacticsData
        0x003b: const-string v3, ""
        0x003f: invoke-virtual com.okinc.unify_trade.bot.data.TacticsData.getOrderType():java.lang.String
        0x0045: sget v4, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x004b: sput v5, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x004e: invoke-virtual o.abQb.RcXXUw():androidx.databinding.ViewDataBinding
        0x0052: check-cast v4, o.XBx
        0x0054: iget v4, v4, o.XBx.KWHzl:androidx.appcompat.widget.AppCompatTextView
        0x0056: invoke-virtual java.lang.String.hashCode():int
        0x005e: const-string v2, "grid"
        0x0060: invoke-virtual java.lang.String.equals(java.lang.Object):boolean
        0x0067: sget v1, o.WSQ$PendingIntent.postMessage:int
        0x0069: invoke-static o.uAq.AYXKKw(int):java.lang.String
        0x006e: const-string v2, "contract_grid"
        0x0070: invoke-virtual java.lang.String.equals(java.lang.Object):boolean
        0x0077: sget v1, o.WSQ$PendingIntent.mayLaunchUrl:int
        0x0079: invoke-static o.uAq.AYXKKw(int):java.lang.String
        0x007e: const-string v2, "contract_dca"
        0x0080: invoke-virtual java.lang.String.equals(java.lang.Object):boolean
        0x0087: sget v1, o.WSQ$PendingIntent.getInterfaceDescriptor:int
        0x0089: invoke-static o.uAq.AYXKKw(int):java.lang.String
        0x008e: const-string v5, "spot_dca"
        0x0090: invoke-virtual java.lang.String.equals(java.lang.Object):boolean
        0x0099: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x009f: sput v5, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x00a4: sget v1, o.WSQ$PendingIntent.extraCommand:int
        0x00a6: invoke-static o.uAq.AYXKKw(int):java.lang.String
        0x00aa: const/16 v5, 92
        0x00ae: sget v1, o.WSQ$PendingIntent.extraCommand:int
        0x00b0: invoke-static o.uAq.AYXKKw(int):java.lang.String
        0x00b4: sget v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x00ba: sput v5, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x00bf: invoke-virtual android.widget.TextView.setText(java.lang.CharSequence):void
        0x00c2: invoke-virtual o.abQb.RcXXUw():androidx.databinding.ViewDataBinding
        0x00c6: check-cast v1, o.XBx
        0x00c8: iget v1, v1, o.XBx.valueOf:o.abdw
        0x00ca: const-wide/16 v4, 30
        0x00cc: invoke-virtual o.abdw.copydefault(long):void
        0x00cf: invoke-virtual o.abQb.UeEOUB():com.okinc.unify_trade.refactor.presenter.AbsPresenter
        0x00d3: check-cast v1, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter
        0x00d5: invoke-virtual com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.gEmmrt():com.okinc.unify_trade.bot.data.TacticsData
        0x00db: invoke-virtual com.okinc.unify_trade.bot.data.TacticsData.getOrderType():java.lang.String
        0x00e0: const/4 v1, 0
        0x00e5: invoke-virtual com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.EZpvd(java.lang.String):void
        0x00ec: invoke-static o.Hac.copydefault():int
        0x00f0: invoke-static o.Hac.copydefault():int
        0x00f4: invoke-static o.Hac.copydefault():int
        0x00f8: invoke-static o.Hac.copydefault():int
        0x00fc: const v10, 520186186
        0x00ff: const v9, -520186176
        0x0102: invoke-static/range com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.copydefault(int, int, java.lang.Object[], int, int, int, int):java.lang.Object
        0x0106: check-cast v1, o.aajI
        0x0108: invoke-virtual com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.AEQbTJ(o.aajI):void
        0x010b: invoke-virtual com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.fIwbmz():void
        0x010e: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0114: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0117: return-void
    }

    public final void AEQbTJ(java.lang.String p0, int p1, kotlin.jvm.functions.Function0 p2) {
        0x0000: const/4 v0, 2
        0x0003: const-string v1, ""
        0x0005: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0008: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x000b: invoke-virtual o.abQb.RcXXUw():androidx.databinding.ViewDataBinding
        0x000f: check-cast v12, o.XBx
        0x0011: iget v12, v12, o.XBx.valueOf:o.abdw
        0x0013: invoke-virtual o.abdw.EZpvd():void
        0x0016: invoke-virtual o.abQb.RcXXUw():androidx.databinding.ViewDataBinding
        0x001a: check-cast v12, o.XBx
        0x001c: iget v12, v12, o.XBx.valueOf:o.abdw
        0x001e: const/16 v1, 8
        0x0020: invoke-virtual o.abdw.setVisibility(int):void
        0x0023: sget v12, o.WSQ$PendingIntent.setDefaultImpl:int
        0x0025: invoke-static o.uAq.AYXKKw(int):java.lang.String
        0x0029: invoke-direct com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.zLjUOn():o.Bpy
        0x002d: invoke-interface o.Bpy.copydefault():com.okinc.localization2.bean.AppLocale
        0x0031: invoke-virtual com.okinc.localization2.bean.AppLocale.getLocale():java.util.Locale
        0x0035: invoke-static o.Bpc.copydefault(java.util.Locale):boolean
        0x0039: const/4 v2, 0
        0x003c: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0042: sput v3, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0045: const/4 v1, 0
        0x0046: const-string v3, "："
        0x0048: invoke-static o.adxw.endsWith$default(java.lang.String, java.lang.String, boolean, int, java.lang.Object):boolean
        0x004e: invoke-static kotlin.text.StringsKt.AEQbTJ(java.lang.String, java.lang.CharSequence):java.lang.String
        0x0052: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0058: sput v3, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x005b: new-instance v1, java.lang.StringBuilder
        0x005d: invoke-direct java.lang.StringBuilder.<init>():void
        0x0060: invoke-virtual java.lang.StringBuilder.append(java.lang.String):java.lang.StringBuilder
        0x0063: const-string v12, " :"
        0x0065: invoke-virtual java.lang.StringBuilder.append(java.lang.String):java.lang.StringBuilder
        0x0068: invoke-virtual java.lang.StringBuilder.toString():java.lang.String
        0x006c: new-instance v1, java.lang.StringBuilder
        0x006e: invoke-direct java.lang.StringBuilder.<init>():void
        0x0071: invoke-virtual java.lang.StringBuilder.append(java.lang.String):java.lang.StringBuilder
        0x0074: const-string v12, " "
        0x0076: invoke-virtual java.lang.StringBuilder.append(java.lang.String):java.lang.StringBuilder
        0x0079: invoke-virtual java.lang.StringBuilder.append(java.lang.String):java.lang.StringBuilder
        0x007c: invoke-virtual java.lang.StringBuilder.toString():java.lang.String
        0x0080: new-instance v12, android.text.SpannableString
        0x0082: invoke-direct android.text.SpannableString.<init>(java.lang.CharSequence):void
        0x0085: const/4 v5, 0
        0x0086: const/4 v6, 0
        0x0087: const/4 v7, 6
        0x0088: const/4 v8, 0
        0x008a: invoke-static/range kotlin.text.StringsKt.indexOf$default(java.lang.CharSequence, java.lang.String, int, boolean, int, java.lang.Object):int
        0x008e: const/4 v3, -1
        0x0091: new-instance v3, android.text.style.ForegroundColorSpan
        0x0093: invoke-direct android.text.style.ForegroundColorSpan.<init>(int):void
        0x0096: invoke-virtual java.lang.String.length():int
        0x009b: const/16 v11, 33
        0x009d: invoke-virtual android.text.SpannableString.setSpan(java.lang.Object, int, int, int):void
        0x00a0: invoke-virtual o.abQb.RcXXUw():androidx.databinding.ViewDataBinding
        0x00a4: check-cast v10, o.XBx
        0x00a6: iget v10, v10, o.XBx.EZpvd:androidx.appcompat.widget.AppCompatTextView
        0x00a8: invoke-virtual android.widget.TextView.setText(java.lang.CharSequence):void
        0x00ab: invoke-virtual o.abQb.RcXXUw():androidx.databinding.ViewDataBinding
        0x00af: check-cast v10, o.XBx
        0x00b1: iget v10, v10, o.XBx.fetchVPNInfo:androidx.appcompat.widget.LinearLayoutCompat
        0x00b3: new-instance v11, o.aajh$StateListAnimator
        0x00b5: const-wide/16 v3, 1000
        0x00b7: invoke-direct o.aajh$StateListAnimator.<init>(android.view.View, long, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity):void
        0x00ba: invoke-virtual android.view.View.setOnClickListener(android.view.View$OnClickListener):void
        0x00bd: sget v10, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x00c3: sput v11, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x00c8: const/16 v10, 47
        0x00cb: return-void
    }

    public final void AEQbTJ(o.aajI p0) {
        0x0000: const/4 v0, 2
        0x0003: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0009: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x000c: const-string v1, ""
        0x000e: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0011: iget v1, v7, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.KWHzl:boolean
        0x0015: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x001b: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0020: invoke-virtual com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.copydefault(o.aajI):androidx.fragment.app.Fragment
        0x0025: invoke-virtual com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.copydefault(o.aajI):androidx.fragment.app.Fragment
        0x0028: const/4 v8, 0
        0x002a: invoke-virtual com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.EZpvd(o.aajI):androidx.fragment.app.Fragment
        0x002e: iput v8, v7, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.EZpvd:androidx.fragment.app.Fragment
        0x0034: invoke-static o.Hac.copydefault():int
        0x0038: invoke-static o.Hac.copydefault():int
        0x003c: invoke-static o.Hac.copydefault():int
        0x0040: invoke-static o.Hac.copydefault():int
        0x0044: const v6, 415126634
        0x0047: const v5, -415126617
        0x004a: invoke-static/range com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.copydefault(int, int, java.lang.Object[], int, int, int, int):java.lang.Object
        0x004d: return-void
    }

    public int AYXKKw() {
        0x0000: const/4 v0, 2
        0x0003: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0009: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x000c: const/4 v2, 0
        0x000f: sget v1, o.WSQ$TaskDescription.OLrzqt:int
        0x0011: sget v3, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0017: sput v4, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x001c: return v1
        0x001e: sget v0, o.WSQ$TaskDescription.OLrzqt:int
        0x0020: invoke-virtual java.lang.Object.hashCode():int
    }

    public final void AhwBna() {
        0x0004: invoke-static com.appsflyer.internal.AFi1bSDK$3$$ExternalSyntheticLambda0.EZpvd():int
        0x0008: invoke-static o.Hac.copydefault():int
        0x000c: invoke-static com.appsflyer.internal.AFi1bSDK$3$$ExternalSyntheticLambda0.EZpvd():int
        0x0010: invoke-static o.Hac.copydefault():int
        0x0014: const v6, -586725243
        0x0017: const v5, 586725245
        0x001a: invoke-static/range com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.copydefault(int, int, java.lang.Object[], int, int, int, int):java.lang.Object
        0x001d: return-void
    }

    public final void AkhnZx() {
        0x0000: const-string v0, "grid"
        0x0002: const/4 v1, 2
        0x0005: sget v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x000b: sput v3, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0010: invoke-virtual/range o.abQb.UeEOUB():com.okinc.unify_trade.refactor.presenter.AbsPresenter
        0x0014: check-cast v2, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter
        0x0016: invoke-virtual com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.gEmmrt():com.okinc.unify_trade.bot.data.TacticsData
        0x001a: const/16 v3, 58
        0x0021: invoke-virtual/range o.abQb.UeEOUB():com.okinc.unify_trade.refactor.presenter.AbsPresenter
        0x0025: check-cast v2, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter
        0x0027: invoke-virtual com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.gEmmrt():com.okinc.unify_trade.bot.data.TacticsData
        0x002e: invoke-virtual com.okinc.unify_trade.bot.data.TacticsData.getOrderType():java.lang.String
        0x0032: invoke-static kotlin.jvm.internal.Intrinsics.KWHzl(java.lang.Object, java.lang.Object):boolean
        0x0038: sget v3, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x003e: sput v4, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0041: sget v3, o.aaSv.EZpvd:o.aaSv
        0x0043: invoke-virtual o.aaSv.gEmmrt(java.lang.String):boolean
        0x0049: sget v0, o.aajN.copydefault:o.aajN
        0x004b: invoke-virtual o.aajN.copydefault(com.okinc.unify_trade.bot.data.TacticsData):com.okinc.tradingbot.impl.order.strategy.grid.entry.SpotGridRustRawExternalEntry
        0x0050: sget v3, com.okinc.tradingbot.impl.order.strategy.BotTradeActivity.Companion:o.YAK$Application
        0x0052: invoke-virtual com.okinc.tradingbot.impl.order.strategy.grid.entry.SpotGridRustRawExternalEntry.copydefault():java.lang.String
        0x0056: invoke-virtual com.okinc.tradingbot.impl.order.strategy.grid.entry.SpotGridRustRawExternalEntry.EZpvd():java.lang.String
        0x005a: invoke-virtual com.okinc.tradingbot.impl.order.strategy.grid.entry.SpotGridRustRawExternalEntry.hDKMBd():java.lang.String
        0x005e: const/4 v9, 0
        0x005f: const-string v10, "bot_order_on_kline"
        0x0061: const/4 v11, 0
        0x0062: const/4 v12, 0
        0x0063: const/4 v13, 0
        0x0064: const/4 v14, 0
        0x0065: const/4 v15, 0
        0x0066: const/16 v16, 1
        0x0068: const/16 v17, 0
        0x006a: const/16 v18, 0
        0x006c: const/16 v19, 28576
        0x006e: const/16 v20, 0
        0x0072: invoke-static/range o.YAK$Application.startSpotGridRustExternalEntry$OKTradingBot_trading_bot_impl$default(o.YAK$Application, android.content.Context, java.lang.String, java.lang.String, java.lang.String, com.okinc.tradingbot.impl.order.strategy.grid.entry.SpotGridRustRawExternalEntry, java.util.HashMap, java.lang.String, java.lang.Integer, java.lang.String, java.lang.String, java.lang.String, boolean, boolean, boolean, o.aFfA, int, java.lang.Object):void
        0x0076: invoke-static o.aajK.KWHzl(com.okinc.unify_trade.bot.data.TacticsData):com.okinc.unify_trade.biz.BotTradeData
        0x007c: sget v0, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0082: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0087: return-void
        0x0088: const/4 v0, 0
        0x008a: sget v21, com.okinc.tradingbot.impl.order.strategy.BotTradeActivity.Companion:o.YAK$Application
        0x008c: const/16 v24, 0
        0x008e: const-string v25, "bot_order_on_kline"
        0x0090: const/16 v26, 0
        0x0092: const/16 v27, 0
        0x0094: const/16 v28, 0
        0x0096: const/16 v29, 0
        0x0098: const/16 v30, 0
        0x009a: const/16 v31, 0
        0x009c: const/16 v32, 0
        0x009e: const/16 v33, 0
        0x00a0: const/16 v34, 0
        0x00a2: const/16 v35, 0
        0x00a4: const/16 v36, 0
        0x00a6: const/16 v37, 32756
        0x00a8: const/16 v38, 0
        0x00ac: invoke-static/range o.YAK$Application.startActivity$default(o.YAK$Application, android.content.Context, com.okinc.unify_trade.biz.BotTradeData, java.util.HashMap, java.lang.String, java.lang.Integer, java.lang.String, android.os.Parcelable, java.lang.String, boolean, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, int, java.lang.Object):void
        0x00af: sget v0, o.absU.EZpvd:o.absU
        0x00b1: invoke-virtual/range o.abQb.UeEOUB():com.okinc.unify_trade.refactor.presenter.AbsPresenter
        0x00b5: check-cast v1, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter
        0x00b7: invoke-virtual com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.AEQbTJ():java.lang.String
        0x00bb: const-string v2, "create_bot"
        0x00bd: invoke-virtual o.absU.KWHzl(java.lang.String, java.lang.String):void
        0x00c0: return-void
    }

    public final void AuCTel() {
        0x0000: const/4 v0, 2
        0x0003: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0009: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x000e: sget v1, o.aajM.AEQbTJ:o.aajM
        0x0010: invoke-virtual o.abQb.UeEOUB():com.okinc.unify_trade.refactor.presenter.AbsPresenter
        0x0014: check-cast v2, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter
        0x0016: invoke-virtual com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.gEmmrt():com.okinc.unify_trade.bot.data.TacticsData
        0x001a: invoke-virtual o.aajM.KWHzl(android.content.Context, com.okinc.unify_trade.bot.data.TacticsData):void
        0x001d: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0023: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0026: return-void
        0x0027: sget v0, o.aajM.AEQbTJ:o.aajM
        0x0029: invoke-virtual o.abQb.UeEOUB():com.okinc.unify_trade.refactor.presenter.AbsPresenter
        0x002d: check-cast v1, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter
        0x002f: invoke-virtual com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.gEmmrt():com.okinc.unify_trade.bot.data.TacticsData
        0x0033: invoke-virtual o.aajM.KWHzl(android.content.Context, com.okinc.unify_trade.bot.data.TacticsData):void
        0x0036: const/4 v0, 0
        0x0037: invoke-virtual java.lang.Object.hashCode():int
    }

    public final void DbNXlk() {
        0x0004: invoke-static com.appsflyer.internal.AFi1bSDK$3$$ExternalSyntheticLambda0.EZpvd():int
        0x0008: invoke-static o.Hac.copydefault():int
        0x000c: invoke-static o.Hac.copydefault():int
        0x0010: invoke-static o.Hac.copydefault():int
        0x0014: const v6, 487838831
        0x0017: const v5, -487838813
        0x001a: invoke-static/range com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.copydefault(int, int, java.lang.Object[], int, int, int, int):java.lang.Object
        0x001d: return-void
    }

    public final float EZpvd(int p0) {
        0x0000: const/4 v0, 2
        0x0003: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0009: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x000e: const/16 v1, 52
        0x0010: invoke-static java.lang.Integer.valueOf(int):java.lang.Integer
        0x0014: invoke-static java.lang.Integer.valueOf(int):java.lang.Integer
        0x0018: invoke-static o.uAv.AYXKKw(java.lang.Object, java.lang.Object):boolean
        0x001f: const/16 v1, 14
        0x0021: invoke-static java.lang.Integer.valueOf(int):java.lang.Integer
        0x0025: invoke-static java.lang.Integer.valueOf(int):java.lang.Integer
        0x0029: invoke-static o.uAv.AYXKKw(java.lang.Object, java.lang.Object):boolean
        0x002f: const/high16 v6, 0x41900000
        0x0031: return v6
        0x0032: invoke-static java.lang.Integer.valueOf(int):java.lang.Integer
        0x0036: invoke-static o.uAv.copydefault(java.lang.Object, java.lang.Object):boolean
        0x003a: invoke-static java.lang.Integer.valueOf(int):java.lang.Integer
        0x003e: const/16 v3, 20
        0x0040: invoke-static java.lang.Integer.valueOf(int):java.lang.Integer
        0x0044: invoke-static o.uAv.AYXKKw(java.lang.Object, java.lang.Object):boolean
        0x0049: const/4 v2, 1
        0x004c: invoke-static java.lang.Integer.valueOf(int):java.lang.Integer
        0x0050: invoke-static java.lang.Integer.valueOf(int):java.lang.Integer
        0x0054: invoke-static o.uAv.copydefault(java.lang.Object, java.lang.Object):boolean
        0x0058: invoke-static java.lang.Integer.valueOf(int):java.lang.Integer
        0x005c: const/16 v2, 26
        0x005e: invoke-static java.lang.Integer.valueOf(int):java.lang.Integer
        0x0062: invoke-static o.uAv.AYXKKw(java.lang.Object, java.lang.Object):boolean
        0x0069: sget v6, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x006f: sput v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0072: const/high16 v6, 0x41600000
        0x0074: return v6
        0x0075: const/high16 v6, 0x41400000
        0x0077: return v6
        0x0078: const/high16 v6, 0x41800000
        0x007a: return v6
    }

    public final androidx.fragment.app.Fragment EZpvd(o.aajI p0) {
        0x0000: const/4 v0, 2
        0x0003: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0009: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x000c: invoke-static o.AtZ.OLrzqt():o.adaP
        0x0010: invoke-interface o.adaP.getValue():java.lang.Object
        0x0015: check-cast v2, o.Aqk
        0x0017: invoke-virtual o.aajI.AEQbTJ():java.lang.String
        0x001b: invoke-virtual o.aajI.copydefault():java.lang.String
        0x001f: sget v8, com.okinc.kline.ui.bean.EntryType.PRO_BOT_LANDSCAPE:com.okinc.kline.ui.bean.EntryType
        0x0021: invoke-virtual o.aajI.KWHzl():long
        0x0025: invoke-virtual o.aajI.EZpvd():kotlin.jvm.functions.Function0
        0x0029: const/4 v5, 0
        0x002a: const/4 v6, 0
        0x002b: const/4 v7, 0
        0x002c: invoke-interface/range o.Aqk.OLrzqt(java.lang.String, java.lang.String, boolean, boolean, boolean, com.okinc.kline.ui.bean.EntryType, long, kotlin.jvm.functions.Function0):androidx.fragment.app.Fragment
        0x0030: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0036: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x003b: const/16 v0, 74
        0x003f: return-object v13
    }

    public final void EZpvd(java.lang.String p0) {
        0x0002: const/4 v1, 2
        0x0005: sget v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x000b: sput v3, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x000e: const-string v3, ""
        0x0012: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0015: invoke-virtual/range java.lang.String.hashCode():int
        0x0019: const/16 v3, 51
        0x0021: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0024: invoke-virtual/range java.lang.String.hashCode():int
        0x002e: const-string v1, "grid"
        0x0030: invoke-virtual java.lang.String.equals(java.lang.Object):boolean
        0x0037: const-string v1, "contract_grid"
        0x0039: invoke-virtual java.lang.String.equals(java.lang.Object):boolean
        0x0040: const-string v2, "contract_dca"
        0x0042: invoke-virtual java.lang.String.equals(java.lang.Object):boolean
        0x0048: sget v0, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x004e: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0052: sget v0, o.aaVP$Application.gkZNMa:int
        0x0054: invoke-static java.lang.Integer.valueOf(int):java.lang.Integer
        0x0058: sget v1, o.WSQ$PendingIntent.zOIQXb:int
        0x005a: invoke-static java.lang.Integer.valueOf(int):java.lang.Integer
        0x005e: invoke-static o.adaT.copydefault(java.lang.Object, java.lang.Object):kotlin.Pair
        0x0063: const-string v1, "spot_dca"
        0x0065: invoke-virtual java.lang.String.equals(java.lang.Object):boolean
        0x006d: sget v0, o.aaVP$Application.DkIxYv:int
        0x006f: invoke-static java.lang.Integer.valueOf(int):java.lang.Integer
        0x0073: sget v1, o.WSQ$PendingIntent.zhUgOk:int
        0x0075: invoke-static java.lang.Integer.valueOf(int):java.lang.Integer
        0x0079: invoke-static o.adaT.copydefault(java.lang.Object, java.lang.Object):kotlin.Pair
        0x007d: invoke-virtual kotlin.Pair.component1():java.lang.Object
        0x0081: check-cast v1, java.lang.Number
        0x0083: invoke-virtual java.lang.Number.intValue():int
        0x0087: invoke-virtual kotlin.Pair.component2():java.lang.Object
        0x008b: check-cast v0, java.lang.Number
        0x008d: invoke-virtual java.lang.Number.intValue():int
        0x0091: sget v3, o.WSQ$StateListAnimator.hOMIpD:int
        0x0093: invoke-static o.uAq.AYXKKw(int):java.lang.String
        0x0097: new-instance v0, o.APx
        0x0099: const/4 v6, 0
        0x009a: const/4 v7, 0
        0x009b: const/4 v8, 0
        0x009c: const/4 v9, 0
        0x009d: const/4 v10, 0
        0x009e: const/4 v11, 0
        0x009f: const/4 v12, 0
        0x00a0: const/16 v13, 1016
        0x00a2: const/4 v14, 0
        0x00a4: invoke-direct/range o.APx.<init>(int, java.lang.CharSequence, int, int, com.okinc.kline.ui.view.KlineBottomActionBarButtonStyle, boolean, float, boolean, boolean, java.lang.CharSequence, int, kotlin.jvm.internal.DefaultConstructorMarker):void
        0x00a8: iput v0, v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.AhwBna:o.APx
        0x00ae: invoke-static o.Hac.copydefault():int
        0x00b2: invoke-static o.Hac.copydefault():int
        0x00b6: invoke-static o.Hac.copydefault():int
        0x00ba: invoke-static o.Hac.copydefault():int
        0x00be: const v9, -482262387
        0x00c1: const v8, 482262400
        0x00c4: invoke-static/range com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.copydefault(int, int, java.lang.Object[], int, int, int, int):java.lang.Object
        0x00c7: return-void
        0x00c8: sget v0, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x00ce: sput v3, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x00d1: return-void
    }

    public final void EZpvd(java.lang.String p0, java.lang.String p1) {
        0x0004: invoke-static o.Hac.copydefault():int
        0x0008: invoke-static o.Hac.copydefault():int
        0x000c: invoke-static o.Hac.copydefault():int
        0x0010: invoke-static o.Hac.copydefault():int
        0x0014: const v6, -1951062827
        0x0017: const v5, 1951062839
        0x001a: invoke-static/range com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.copydefault(int, int, java.lang.Object[], int, int, int, int):java.lang.Object
        0x001d: return-void
    }

    public final java.lang.String KWHzl(java.lang.String p0, java.lang.String p1) {
        0x0000: const/4 v0, 2
        0x0003: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0009: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x000c: const-string v1, "FUTURES"
        0x000e: invoke-static kotlin.jvm.internal.Intrinsics.KWHzl(java.lang.Object, java.lang.Object):boolean
        0x0012: const-string v2, "/"
        0x0014: const-string v3, "https://www.okx.com/trade-"
        0x0016: const-string v4, ""
        0x001a: iget v1, v12, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.copydefault:o.abuj
        0x001c: const/4 v5, 0
        0x001f: sget v6, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0025: sput v7, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0028: invoke-virtual o.abuj.fetchVPNInfo():com.okinc.unify_trade.biz.BizInstrument
        0x002d: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0033: sput v6, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0037: instance-of v6, com.okinc.unify_trade.biz.FutureInstrument
        0x003b: sget v6, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0041: sput v7, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0047: check-cast v5, com.okinc.unify_trade.biz.FutureInstrument
        0x004a: check-cast v1, com.okinc.unify_trade.biz.FutureInstrument
        0x004f: return-object v4
        0x0050: invoke-virtual com.okinc.unify_trade.biz.FutureInstrument.getAlias():java.lang.String
        0x0054: invoke-static o.uAv.EZpvd(java.lang.CharSequence):boolean
        0x005a: invoke-virtual java.lang.String.hashCode():int
        0x0062: const-string v5, "next_quarter"
        0x0064: invoke-virtual java.lang.String.equals(java.lang.Object):boolean
        0x006b: const-string v1, "biquarterly"
        0x006e: const-string v5, "next_week"
        0x0070: invoke-virtual java.lang.String.equals(java.lang.Object):boolean
        0x0077: const-string v1, "biweekly"
        0x007a: const-string v5, "quarter"
        0x007c: invoke-virtual java.lang.String.equals(java.lang.Object):boolean
        0x0083: const-string v1, "quarterly"
        0x0086: const-string v5, "this_month"
        0x0088: invoke-virtual java.lang.String.equals(java.lang.Object):boolean
        0x008f: const-string v1, "bimonthly"
        0x0092: const-string v5, "this_week"
        0x0094: invoke-virtual java.lang.String.equals(java.lang.Object):boolean
        0x009b: const-string v1, "weekly"
        0x009e: const-string v5, "next_month"
        0x00a0: invoke-virtual java.lang.String.equals(java.lang.Object):boolean
        0x00a7: const-string v1, "monthly"
        0x00a9: const-string v5, "-"
        0x00af: const/4 v8, 0
        0x00b0: const/4 v9, 0
        0x00b1: const/4 v10, 6
        0x00b2: const/4 v11, 0
        0x00b4: invoke-static/range kotlin.text.StringsKt.split$default(java.lang.CharSequence, java.lang.String[], boolean, int, int, java.lang.Object):java.util.List
        0x00b8: invoke-interface java.util.List.size():int
        0x00bc: const/4 v7, 3
        0x00bf: const/4 v6, 0
        0x00c0: invoke-interface java.util.List.get(int):java.lang.Object
        0x00c4: const/4 v7, 1
        0x00c5: invoke-interface java.util.List.get(int):java.lang.Object
        0x00c9: new-instance v7, java.lang.StringBuilder
        0x00cb: invoke-direct java.lang.StringBuilder.<init>():void
        0x00ce: invoke-virtual java.lang.StringBuilder.append(java.lang.Object):java.lang.StringBuilder
        0x00d1: invoke-virtual java.lang.StringBuilder.append(java.lang.String):java.lang.StringBuilder
        0x00d4: invoke-virtual java.lang.StringBuilder.append(java.lang.Object):java.lang.StringBuilder
        0x00d7: invoke-virtual java.lang.StringBuilder.append(java.lang.String):java.lang.StringBuilder
        0x00da: invoke-virtual java.lang.StringBuilder.append(java.lang.String):java.lang.StringBuilder
        0x00dd: invoke-virtual java.lang.StringBuilder.toString():java.lang.String
        0x00e1: sget v1, java.util.Locale.ROOT:java.util.Locale
        0x00e3: invoke-virtual java.lang.String.toLowerCase(java.util.Locale):java.lang.String
        0x00e7: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullExpressionValue(java.lang.Object, java.lang.String):void
        0x00ea: invoke-virtual java.lang.String.toLowerCase(java.util.Locale):java.lang.String
        0x00ee: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullExpressionValue(java.lang.Object, java.lang.String):void
        0x00f1: new-instance v1, java.lang.StringBuilder
        0x00f3: invoke-direct java.lang.StringBuilder.<init>():void
        0x00f6: invoke-virtual java.lang.StringBuilder.append(java.lang.String):java.lang.StringBuilder
        0x00f9: invoke-virtual java.lang.StringBuilder.append(java.lang.String):java.lang.StringBuilder
        0x00fc: invoke-virtual java.lang.StringBuilder.append(java.lang.String):java.lang.StringBuilder
        0x00ff: invoke-virtual java.lang.StringBuilder.append(java.lang.String):java.lang.StringBuilder
        0x0102: invoke-virtual java.lang.StringBuilder.toString():java.lang.String
        0x0106: sget v14, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x010c: sput v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x010f: return-object v13
        0x0110: return-object v4
        0x0111: sget v0, java.util.Locale.ROOT:java.util.Locale
        0x0113: invoke-virtual java.lang.String.toLowerCase(java.util.Locale):java.lang.String
        0x0117: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullExpressionValue(java.lang.Object, java.lang.String):void
        0x011a: invoke-virtual java.lang.String.toLowerCase(java.util.Locale):java.lang.String
        0x011e: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullExpressionValue(java.lang.Object, java.lang.String):void
        0x0121: new-instance v0, java.lang.StringBuilder
        0x0123: invoke-direct java.lang.StringBuilder.<init>():void
        0x0126: invoke-virtual java.lang.StringBuilder.append(java.lang.String):java.lang.StringBuilder
        0x0129: invoke-virtual java.lang.StringBuilder.append(java.lang.String):java.lang.StringBuilder
        0x012c: invoke-virtual java.lang.StringBuilder.append(java.lang.String):java.lang.StringBuilder
        0x012f: invoke-virtual java.lang.StringBuilder.append(java.lang.String):java.lang.StringBuilder
        0x0132: invoke-virtual java.lang.StringBuilder.toString():java.lang.String
        0x0136: return-object v13
    }

    public final void KWHzl(java.lang.String p0) {
        0x0000: const/4 v0, 2
        0x0003: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0009: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x000c: const-string v1, ""
        0x000e: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0011: invoke-virtual o.abQb.RcXXUw():androidx.databinding.ViewDataBinding
        0x0015: check-cast v1, o.XBx
        0x0017: iget v1, v1, o.XBx.AkhnZx:androidx.appcompat.widget.AppCompatTextView
        0x0019: invoke-virtual java.lang.String.length():int
        0x001d: invoke-virtual com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.EZpvd(int):float
        0x0021: invoke-virtual android.widget.TextView.setTextSize(float):void
        0x0024: invoke-virtual o.abQb.RcXXUw():androidx.databinding.ViewDataBinding
        0x0028: check-cast v1, o.XBx
        0x002a: iget v1, v1, o.XBx.AkhnZx:androidx.appcompat.widget.AppCompatTextView
        0x002c: invoke-virtual android.widget.TextView.setText(java.lang.CharSequence):void
        0x0033: invoke-static com.appsflyer.internal.AFi1bSDK$3$$ExternalSyntheticLambda0.EZpvd():int
        0x0037: invoke-static o.Hac.copydefault():int
        0x003b: invoke-static com.appsflyer.internal.AFi1bSDK$3$$ExternalSyntheticLambda0.EZpvd():int
        0x003f: invoke-static o.Hac.copydefault():int
        0x0043: const v8, -586725243
        0x0046: const v7, 586725245
        0x0049: invoke-static/range com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.copydefault(int, int, java.lang.Object[], int, int, int, int):java.lang.Object
        0x004c: sget v10, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0052: sput v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0057: const/16 v10, 73
        0x005b: return-void
    }

    public final void OLrzqt(com.okinc.unify_trade.biz.StrategyDetailsResponse p0, o.abrD p1) {
        0x0002: const/4 v1, 2
        0x0005: invoke-virtual/range o.abQb.UeEOUB():com.okinc.unify_trade.refactor.presenter.AbsPresenter
        0x0009: check-cast v2, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter
        0x000d: invoke-virtual com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.AEQbTJ(com.okinc.unify_trade.biz.StrategyDetailsResponse):void
        0x0010: invoke-virtual/range com.okinc.unify_trade.biz.StrategyDetailsResponse.getTriggerParams():java.util.List
        0x0014: const/4 v4, 1
        0x0017: check-cast v2, java.lang.Iterable
        0x0019: new-instance v6, java.util.ArrayList
        0x001b: invoke-direct java.util.ArrayList.<init>():void
        0x001e: invoke-interface java.lang.Iterable.iterator():java.util.Iterator
        0x0022: invoke-interface java.util.Iterator.hasNext():boolean
        0x0029: sget v7, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x002f: sput v8, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0032: invoke-interface java.util.Iterator.next():java.lang.Object
        0x0037: check-cast v8, com.okinc.unify_trade.biz.DcaTriggerParam
        0x0039: invoke-virtual com.okinc.unify_trade.biz.DcaTriggerParam.getTriggerAction():java.lang.String
        0x003d: const-string v9, "start"
        0x003f: invoke-static kotlin.jvm.internal.Intrinsics.KWHzl(java.lang.Object, java.lang.Object):boolean
        0x0046: invoke-interface java.util.Collection.add(java.lang.Object):boolean
        0x004a: const/4 v6, 0
        0x004b: invoke-virtual/range com.okinc.unify_trade.biz.StrategyDetailsResponse.getTriggerParams():java.util.List
        0x0051: check-cast v2, java.lang.Iterable
        0x0053: new-instance v7, java.util.ArrayList
        0x0055: invoke-direct java.util.ArrayList.<init>():void
        0x0058: invoke-interface java.lang.Iterable.iterator():java.util.Iterator
        0x005c: invoke-interface java.util.Iterator.hasNext():boolean
        0x0064: sget v8, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x006a: sput v9, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x006d: invoke-interface java.util.Iterator.next():java.lang.Object
        0x0072: check-cast v9, com.okinc.unify_trade.biz.DcaTriggerParam
        0x0074: invoke-virtual com.okinc.unify_trade.biz.DcaTriggerParam.getTriggerAction():java.lang.String
        0x0078: const-string v10, "stop"
        0x007a: invoke-static kotlin.jvm.internal.Intrinsics.KWHzl(java.lang.Object, java.lang.Object):boolean
        0x0080: invoke-interface java.util.Collection.add(java.lang.Object):boolean
        0x0084: const/4 v7, 0
        0x0085: invoke-virtual/range com.okinc.unify_trade.biz.StrategyDetailsResponse.getInitPx():java.lang.String
        0x0089: invoke-interface java.lang.CharSequence.length():int
        0x008d: const/16 v8, 10
        0x0091: invoke-virtual/range com.okinc.unify_trade.biz.StrategyDetailsResponse.getInitPx():java.lang.String
        0x0095: invoke-static o.uAv.copydefault(java.lang.Object):java.math.BigDecimal
        0x009d: new-instance v2, java.util.ArrayList
        0x009f: invoke-static o.adbE.AEQbTJ(java.lang.Iterable, int):int
        0x00a3: invoke-direct java.util.ArrayList.<init>(int):void
        0x00a6: invoke-interface java.lang.Iterable.iterator():java.util.Iterator
        0x00aa: invoke-interface java.util.Iterator.hasNext():boolean
        0x00b0: sget v10, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x00b6: sput v11, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x00b9: invoke-interface java.util.Iterator.next():java.lang.Object
        0x00bd: check-cast v10, com.okinc.unify_trade.biz.DcaTriggerParam
        0x00bf: invoke-virtual com.okinc.unify_trade.biz.DcaTriggerParam.getTriggerPx():java.lang.String
        0x00c3: invoke-interface java.util.Collection.add(java.lang.Object):boolean
        0x00c7: invoke-static kotlin.collections.CollectionsKt.AEQbTJ(java.util.List):java.lang.Object
        0x00cb: check-cast v2, java.lang.String
        0x00ce: const/4 v2, 0
        0x00cf: invoke-static o.uAv.copydefault(java.lang.Object):java.math.BigDecimal
        0x00d4: sget v2, java.math.BigDecimal.ZERO:java.math.BigDecimal
        0x00d6: invoke-virtual java.math.BigDecimal.compareTo(java.math.BigDecimal):int
        0x00de: sget v9, o.abxD.copydefault:o.abxD
        0x00e0: invoke-virtual/range com.okinc.unify_trade.biz.StrategyDetailsResponse.getDirection():java.lang.String
        0x00e4: const-string v10, "short"
        0x00e6: invoke-static kotlin.jvm.internal.Intrinsics.KWHzl(java.lang.Object, java.lang.Object):boolean
        0x00ea: invoke-virtual/range com.okinc.unify_trade.biz.StrategyDetailsResponse.getInitOrdAmt():java.lang.String
        0x00ee: invoke-static o.uAv.copydefault(java.lang.Object):java.math.BigDecimal
        0x00f2: invoke-virtual/range com.okinc.unify_trade.biz.StrategyDetailsResponse.getSafetyOrdAmt():java.lang.String
        0x00f6: invoke-static o.uAv.copydefault(java.lang.Object):java.math.BigDecimal
        0x00fa: invoke-virtual/range com.okinc.unify_trade.biz.StrategyDetailsResponse.getPxSteps():java.lang.String
        0x00fe: invoke-static o.uAv.copydefault(java.lang.Object):java.math.BigDecimal
        0x0102: invoke-virtual/range com.okinc.unify_trade.biz.StrategyDetailsResponse.getMaxSafetyOrds():java.lang.String
        0x0106: invoke-static o.uAv.djBIcL(java.lang.Object):int
        0x010a: invoke-virtual/range com.okinc.unify_trade.biz.StrategyDetailsResponse.getTpPct():java.lang.String
        0x010e: invoke-static o.uAv.copydefault(java.lang.Object):java.math.BigDecimal
        0x0112: invoke-virtual/range com.okinc.unify_trade.biz.StrategyDetailsResponse.getPxStepsMult():java.lang.String
        0x0116: invoke-static o.uAv.copydefault(java.lang.Object):java.math.BigDecimal
        0x011a: invoke-virtual/range com.okinc.unify_trade.biz.StrategyDetailsResponse.getVolMult():java.lang.String
        0x011e: invoke-static o.uAv.copydefault(java.lang.Object):java.math.BigDecimal
        0x0122: invoke-virtual/range o.abxD.OLrzqt(boolean, java.math.BigDecimal, java.math.BigDecimal, java.math.BigDecimal, java.math.BigDecimal, int, java.math.BigDecimal, java.math.BigDecimal, java.math.BigDecimal):java.util.List
        0x0126: invoke-virtual/range com.okinc.unify_trade.biz.StrategyDetailsResponse.getFillSafetyOrds():java.lang.String
        0x012a: invoke-static o.uAv.djBIcL(java.lang.Object):int
        0x012e: invoke-virtual/range com.okinc.unify_trade.biz.StrategyDetailsResponse.getFillManualAddOrds():java.lang.String
        0x0132: invoke-static o.uAv.djBIcL(java.lang.Object):int
        0x0137: invoke-interface java.util.List.size():int
        0x013e: new-instance v10, java.util.ArrayList
        0x0140: invoke-direct java.util.ArrayList.<init>():void
        0x0143: new-instance v11, java.util.ArrayList
        0x0145: invoke-direct java.util.ArrayList.<init>():void
        0x014d: sget v12, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0153: sput v13, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0156: invoke-virtual/range com.okinc.unify_trade.biz.StrategyDetailsResponse.getAlgoOrdType():java.lang.String
        0x015a: const-string v13, "contract_dca"
        0x015c: invoke-static kotlin.jvm.internal.Intrinsics.KWHzl(java.lang.Object, java.lang.Object):boolean
        0x0162: sget v12, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0168: sput v13, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x016b: invoke-virtual/range com.okinc.unify_trade.biz.StrategyDetailsResponse.getDirection():java.lang.String
        0x016f: const-string v13, "long"
        0x0171: invoke-static kotlin.jvm.internal.Intrinsics.KWHzl(java.lang.Object, java.lang.Object):boolean
        0x0178: check-cast v12, java.lang.Iterable
        0x017a: invoke-static kotlin.collections.CollectionsKt.EZpvd(java.lang.Iterable, int):java.util.List
        0x017e: check-cast v4, java.lang.Iterable
        0x0180: new-instance v12, java.util.ArrayList
        0x0182: invoke-static o.adbE.AEQbTJ(java.lang.Iterable, int):int
        0x0186: invoke-direct java.util.ArrayList.<init>(int):void
        0x0189: invoke-interface java.lang.Iterable.iterator():java.util.Iterator
        0x018d: sget v13, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0193: sput v14, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0196: invoke-interface java.util.Iterator.hasNext():boolean
        0x019c: invoke-interface java.util.Iterator.next():java.lang.Object
        0x01a0: check-cast v13, com.okinc.unify_trade.bot.util.dca.DcaPreviewData
        0x01a2: new-instance v14, com.okinc.kline.ui.bean.BotOrderItem
        0x01a4: invoke-direct com.okinc.kline.ui.bean.BotOrderItem.<init>():void
        0x01a7: invoke-virtual com.okinc.unify_trade.bot.util.dca.DcaPreviewData.OLrzqt():java.math.BigDecimal
        0x01ab: invoke-static o.uAv.gEmmrt(java.lang.Object):java.lang.String
        0x01af: invoke-virtual com.okinc.kline.ui.bean.BotOrderItem.setPrice(java.lang.String):void
        0x01b2: invoke-virtual com.okinc.unify_trade.bot.util.dca.DcaPreviewData.AhwBna():java.math.BigDecimal
        0x01b6: invoke-static o.uAv.gEmmrt(java.lang.Object):java.lang.String
        0x01ba: invoke-virtual com.okinc.kline.ui.bean.BotOrderItem.setAmount(java.lang.String):void
        0x01bd: invoke-interface java.util.Collection.add(java.lang.Object):boolean
        0x01c1: invoke-virtual java.util.ArrayList.addAll(java.util.Collection):boolean
        0x01c7: check-cast v12, java.lang.Iterable
        0x01c9: invoke-static kotlin.collections.CollectionsKt.EZpvd(java.lang.Iterable, int):java.util.List
        0x01cd: check-cast v4, java.lang.Iterable
        0x01cf: new-instance v12, java.util.ArrayList
        0x01d1: invoke-static o.adbE.AEQbTJ(java.lang.Iterable, int):int
        0x01d5: invoke-direct java.util.ArrayList.<init>(int):void
        0x01d8: invoke-interface java.lang.Iterable.iterator():java.util.Iterator
        0x01dc: invoke-interface java.util.Iterator.hasNext():boolean
        0x01e2: invoke-interface java.util.Iterator.next():java.lang.Object
        0x01e6: check-cast v13, com.okinc.unify_trade.bot.util.dca.DcaPreviewData
        0x01e8: new-instance v14, com.okinc.kline.ui.bean.BotOrderItem
        0x01ea: invoke-direct com.okinc.kline.ui.bean.BotOrderItem.<init>():void
        0x01ed: invoke-virtual com.okinc.unify_trade.bot.util.dca.DcaPreviewData.OLrzqt():java.math.BigDecimal
        0x01f1: invoke-static o.uAv.gEmmrt(java.lang.Object):java.lang.String
        0x01f5: invoke-virtual com.okinc.kline.ui.bean.BotOrderItem.setPrice(java.lang.String):void
        0x01f8: invoke-virtual com.okinc.unify_trade.bot.util.dca.DcaPreviewData.AhwBna():java.math.BigDecimal
        0x01fc: invoke-static o.uAv.gEmmrt(java.lang.Object):java.lang.String
        0x0200: invoke-virtual com.okinc.kline.ui.bean.BotOrderItem.setAmount(java.lang.String):void
        0x0203: invoke-interface java.util.Collection.add(java.lang.Object):boolean
        0x0207: invoke-virtual java.util.ArrayList.addAll(java.util.Collection):boolean
        0x020c: check-cast v12, java.lang.Iterable
        0x020e: invoke-static kotlin.collections.CollectionsKt.EZpvd(java.lang.Iterable, int):java.util.List
        0x0212: check-cast v4, java.lang.Iterable
        0x0214: new-instance v12, java.util.ArrayList
        0x0216: invoke-static o.adbE.AEQbTJ(java.lang.Iterable, int):int
        0x021a: invoke-direct java.util.ArrayList.<init>(int):void
        0x021d: invoke-interface java.lang.Iterable.iterator():java.util.Iterator
        0x0221: invoke-interface java.util.Iterator.hasNext():boolean
        0x0227: invoke-interface java.util.Iterator.next():java.lang.Object
        0x022b: check-cast v13, com.okinc.unify_trade.bot.util.dca.DcaPreviewData
        0x022d: new-instance v14, com.okinc.kline.ui.bean.BotOrderItem
        0x022f: invoke-direct com.okinc.kline.ui.bean.BotOrderItem.<init>():void
        0x0232: invoke-virtual com.okinc.unify_trade.bot.util.dca.DcaPreviewData.OLrzqt():java.math.BigDecimal
        0x0236: invoke-static o.uAv.gEmmrt(java.lang.Object):java.lang.String
        0x023a: invoke-virtual com.okinc.kline.ui.bean.BotOrderItem.setPrice(java.lang.String):void
        0x023d: invoke-virtual com.okinc.unify_trade.bot.util.dca.DcaPreviewData.AhwBna():java.math.BigDecimal
        0x0241: invoke-static o.uAv.gEmmrt(java.lang.Object):java.lang.String
        0x0245: invoke-virtual com.okinc.kline.ui.bean.BotOrderItem.setAmount(java.lang.String):void
        0x0248: invoke-interface java.util.Collection.add(java.lang.Object):boolean
        0x024c: invoke-virtual java.util.ArrayList.addAll(java.util.Collection):boolean
        0x024f: iget v4, v0, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.EZpvd:androidx.fragment.app.Fragment
        0x0253: new-instance v15, com.okinc.kline.ui.bean.IBotOrdersDataBean
        0x0255: const/4 v13, 0
        0x0256: const/4 v14, 0
        0x0257: const/16 v16, 0
        0x0259: const/16 v17, 0
        0x025b: const/16 v18, 0
        0x025d: const/16 v19, 0
        0x025f: const/16 v20, 0
        0x0261: const/16 v21, 0
        0x0263: const/16 v22, 0
        0x0265: const/16 v23, 0
        0x0267: const/16 v24, 0
        0x0269: const/16 v25, 2047
        0x026b: const/16 v26, 0
        0x0285: invoke-direct/range com.okinc.kline.ui.bean.IBotOrdersDataBean.<init>(java.lang.String, java.lang.String, java.util.ArrayList, java.util.ArrayList, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, com.okinc.kline.ui.bean.BotOrderItem, java.lang.String, int, kotlin.jvm.internal.DefaultConstructorMarker):void
        0x0288: invoke-virtual/range o.abQb.UeEOUB():com.okinc.unify_trade.refactor.presenter.AbsPresenter
        0x028c: check-cast v12, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter
        0x028e: invoke-virtual com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.gEmmrt():com.okinc.unify_trade.bot.data.TacticsData
        0x0292: const-string v13, ""
        0x0296: invoke-virtual com.okinc.unify_trade.bot.data.TacticsData.getOrderType():java.lang.String
        0x029d: invoke-virtual com.okinc.kline.ui.bean.IBotOrdersDataBean.setType(java.lang.String):void
        0x02a0: invoke-virtual com.okinc.kline.ui.bean.IBotOrdersDataBean.setBuyList(java.util.ArrayList):void
        0x02a3: invoke-virtual com.okinc.kline.ui.bean.IBotOrdersDataBean.setSellList(java.util.ArrayList):void
        0x02a6: invoke-virtual/range com.okinc.unify_trade.biz.StrategyDetailsResponse.getTpPx():java.lang.String
        0x02aa: invoke-virtual com.okinc.kline.ui.bean.IBotOrdersDataBean.setTpPrice(java.lang.String):void
        0x02ad: invoke-virtual/range com.okinc.unify_trade.biz.StrategyDetailsResponse.getSlPx():java.lang.String
        0x02b1: invoke-virtual com.okinc.kline.ui.bean.IBotOrdersDataBean.setSlPrice(java.lang.String):void
        0x02b6: invoke-interface java.util.List.size():int
        0x02bc: sget v10, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x02c2: sput v11, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x02c5: invoke-virtual/range com.okinc.unify_trade.biz.StrategyDetailsResponse.getInitPx():java.lang.String
        0x02c9: invoke-interface java.lang.CharSequence.length():int
        0x02d1: new-instance v10, java.util.ArrayList
        0x02d3: invoke-static o.adbE.AEQbTJ(java.lang.Iterable, int):int
        0x02d7: invoke-direct java.util.ArrayList.<init>(int):void
        0x02da: invoke-interface java.lang.Iterable.iterator():java.util.Iterator
        0x02de: invoke-interface java.util.Iterator.hasNext():boolean
        0x02e4: sget v11, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x02ea: sput v12, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x02ef: invoke-interface java.util.Iterator.next():java.lang.Object
        0x02f3: check-cast v11, com.okinc.unify_trade.biz.DcaTriggerParam
        0x02f5: invoke-virtual com.okinc.unify_trade.biz.DcaTriggerParam.getTriggerPx():java.lang.String
        0x02f9: invoke-interface java.util.Collection.add(java.lang.Object):boolean
        0x02fc: const/16 v11, 28
        0x0301: invoke-interface java.util.Iterator.next():java.lang.Object
        0x0305: check-cast v11, com.okinc.unify_trade.biz.DcaTriggerParam
        0x0307: invoke-virtual com.okinc.unify_trade.biz.DcaTriggerParam.getTriggerPx():java.lang.String
        0x030b: invoke-interface java.util.Collection.add(java.lang.Object):boolean
        0x030f: invoke-static kotlin.collections.CollectionsKt.AEQbTJ(java.util.List):java.lang.Object
        0x0313: check-cast v6, java.lang.String
        0x0316: const/4 v6, 0
        0x0317: invoke-static o.uAv.gEmmrt(java.lang.Object):java.lang.String
        0x031d: invoke-virtual com.okinc.kline.ui.bean.IBotOrdersDataBean.setStartTriggerPrice(java.lang.String):void
        0x0322: invoke-interface java.util.List.size():int
        0x032a: new-instance v6, java.util.ArrayList
        0x032c: invoke-static o.adbE.AEQbTJ(java.lang.Iterable, int):int
        0x0330: invoke-direct java.util.ArrayList.<init>(int):void
        0x0333: invoke-interface java.lang.Iterable.iterator():java.util.Iterator
        0x0337: invoke-interface java.util.Iterator.hasNext():boolean
        0x033d: sget v8, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0343: sput v10, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0346: invoke-interface java.util.Iterator.next():java.lang.Object
        0x034a: check-cast v8, com.okinc.unify_trade.biz.DcaTriggerParam
        0x034c: invoke-virtual com.okinc.unify_trade.biz.DcaTriggerParam.getTriggerPx():java.lang.String
        0x0350: invoke-interface java.util.Collection.add(java.lang.Object):boolean
        0x0354: invoke-static kotlin.collections.CollectionsKt.AEQbTJ(java.util.List):java.lang.Object
        0x0358: check-cast v6, java.lang.String
        0x035b: const/4 v6, 0
        0x035c: invoke-static o.uAv.gEmmrt(java.lang.Object):java.lang.String
        0x0362: invoke-virtual com.okinc.kline.ui.bean.IBotOrdersDataBean.setStopTriggerPrice(java.lang.String):void
        0x0365: invoke-virtual/range com.okinc.unify_trade.biz.StrategyDetailsResponse.getInitPx():java.lang.String
        0x0369: invoke-interface java.lang.CharSequence.length():int
        0x036f: sget v3, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0375: sput v6, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x037a: new-instance v1, com.okinc.kline.ui.bean.BotOrderItem
        0x037c: invoke-direct com.okinc.kline.ui.bean.BotOrderItem.<init>():void
        0x037f: invoke-static kotlin.collections.CollectionsKt.AEQbTJ(java.util.List):java.lang.Object
        0x0383: check-cast v3, com.okinc.unify_trade.bot.util.dca.DcaPreviewData
        0x0385: invoke-virtual com.okinc.unify_trade.bot.util.dca.DcaPreviewData.OLrzqt():java.math.BigDecimal
        0x0389: invoke-static o.uAv.gEmmrt(java.lang.Object):java.lang.String
        0x038d: invoke-virtual com.okinc.kline.ui.bean.BotOrderItem.setPrice(java.lang.String):void
        0x0390: invoke-static kotlin.collections.CollectionsKt.AEQbTJ(java.util.List):java.lang.Object
        0x0394: check-cast v2, com.okinc.unify_trade.bot.util.dca.DcaPreviewData
        0x0396: invoke-virtual com.okinc.unify_trade.bot.util.dca.DcaPreviewData.AhwBna():java.math.BigDecimal
        0x039a: invoke-static o.uAv.gEmmrt(java.lang.Object):java.lang.String
        0x039e: invoke-virtual com.okinc.kline.ui.bean.BotOrderItem.setAmount(java.lang.String):void
        0x03a1: invoke-virtual com.okinc.kline.ui.bean.IBotOrdersDataBean.setInitialOrder(com.okinc.kline.ui.bean.BotOrderItem):void
        0x03a5: const/4 v1, 0
        0x03a6: invoke-virtual java.lang.Object.hashCode():int
        0x03aa: invoke-virtual/range o.abQb.UeEOUB():com.okinc.unify_trade.refactor.presenter.AbsPresenter
        0x03ae: check-cast v1, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter
        0x03b0: invoke-virtual com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.gEmmrt():com.okinc.unify_trade.bot.data.TacticsData
        0x03b6: invoke-virtual com.okinc.unify_trade.bot.data.TacticsData.getLiqPx():java.lang.String
        0x03be: invoke-virtual com.okinc.kline.ui.bean.IBotOrdersDataBean.setLiqPrice(java.lang.String):void
        0x03c1: invoke-static java.lang.Integer.valueOf(int):java.lang.Integer
        0x03c5: invoke-static o.uAv.gEmmrt(java.lang.Object):java.lang.String
        0x03c9: invoke-virtual com.okinc.kline.ui.bean.IBotOrdersDataBean.setFillSafetyOrds(java.lang.String):void
        0x03cc: invoke-static o.AtZ.OLrzqt():o.adaP
        0x03d0: invoke-interface o.adaP.getValue():java.lang.Object
        0x03d4: check-cast v1, o.Aqk
        0x03d6: invoke-interface o.Aqk.AEQbTJ(androidx.fragment.app.Fragment, com.okinc.kline.ui.bean.IBotOrdersDataBean):void
        0x03db: invoke-virtual com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.OLrzqt(o.abrD):void
        0x03de: return-void
    }

    public final void OLrzqt(com.okinc.unify_trade.bot.data.BenefitDetails p0, java.lang.String p1) {
        0x0000: const/4 v0, 2
        0x0003: sget v1, o.abxa.EZpvd:o.abxa
        0x0005: invoke-virtual com.okinc.unify_trade.bot.data.BenefitDetails.getPnlRatio():java.lang.String
        0x0009: const/4 v3, 1
        0x000a: invoke-virtual o.abxa.OLrzqt(java.lang.String, boolean):java.lang.String
        0x000e: invoke-virtual com.okinc.unify_trade.bot.data.BenefitDetails.getTotalPnl():java.lang.String
        0x0012: new-instance v3, java.lang.StringBuilder
        0x0014: invoke-direct java.lang.StringBuilder.<init>():void
        0x0017: invoke-virtual java.lang.StringBuilder.append(java.lang.String):java.lang.StringBuilder
        0x001a: const-string v2, " "
        0x001c: invoke-virtual java.lang.StringBuilder.append(java.lang.String):java.lang.StringBuilder
        0x001f: invoke-virtual java.lang.StringBuilder.append(java.lang.String):java.lang.StringBuilder
        0x0022: const-string v6, " ("
        0x0024: invoke-virtual java.lang.StringBuilder.append(java.lang.String):java.lang.StringBuilder
        0x0027: invoke-virtual java.lang.StringBuilder.append(java.lang.String):java.lang.StringBuilder
        0x002a: const-string v6, ")"
        0x002c: invoke-virtual java.lang.StringBuilder.append(java.lang.String):java.lang.StringBuilder
        0x002f: invoke-virtual java.lang.StringBuilder.toString():java.lang.String
        0x0033: invoke-virtual com.okinc.unify_trade.bot.data.BenefitDetails.getPnlRatio():java.lang.String
        0x0037: invoke-static o.abxs.copydefault(java.lang.Object):int
        0x003b: new-instance v1, o.aajF
        0x003d: invoke-direct o.aajF.<init>(com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity):void
        0x0040: invoke-virtual com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.AEQbTJ(java.lang.String, int, kotlin.jvm.functions.Function0):void
        0x0043: sget v5, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0049: sput v6, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x004c: return-void
    }

    public final void OLrzqt(java.lang.String p0) {
        0x0000: const/4 v0, 2
        0x0003: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0009: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x000c: sget v1, o.absU.EZpvd:o.absU
        0x000e: const-string v2, "indicators"
        0x0010: invoke-virtual o.absU.KWHzl(java.lang.String, java.lang.String):void
        0x0013: iget v4, v3, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.EZpvd:androidx.fragment.app.Fragment
        0x0017: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x001d: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0020: invoke-static o.AtZ.OLrzqt():o.adaP
        0x0024: invoke-interface o.adaP.getValue():java.lang.Object
        0x0028: check-cast v0, o.Aqk
        0x002a: invoke-interface o.Aqk.copydefault(androidx.fragment.app.Fragment):void
        0x002d: return-void
    }

    public final void OLrzqt(java.lang.String p0, java.lang.String p1) {
        0x0000: const/4 v0, 2
        0x0003: const-string v1, ""
        0x0005: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0008: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x000b: sget v2, o.acat.AEQbTJ:o.acat
        0x000d: invoke-virtual o.acat.EZpvd(java.lang.String):int
        0x0011: sget v2, o.PWj.EZpvd:o.PWj
        0x0013: invoke-virtual o.PWj.AYXKKw():o.PWs
        0x0017: invoke-interface o.PWs.KWHzl():java.lang.String
        0x001b: new-instance v3, java.lang.StringBuilder
        0x001d: invoke-direct java.lang.StringBuilder.<init>():void
        0x0020: invoke-virtual java.lang.StringBuilder.append(java.lang.String):java.lang.StringBuilder
        0x0023: const-string v2, "?bizType="
        0x0025: invoke-virtual java.lang.StringBuilder.append(java.lang.String):java.lang.StringBuilder
        0x0028: invoke-virtual java.lang.StringBuilder.append(int):java.lang.StringBuilder
        0x002b: const-string v13, "&instId="
        0x002d: invoke-virtual java.lang.StringBuilder.append(java.lang.String):java.lang.StringBuilder
        0x0030: invoke-virtual java.lang.StringBuilder.append(java.lang.String):java.lang.StringBuilder
        0x0033: const-string v12, "&fromPageSession="
        0x0035: invoke-virtual java.lang.StringBuilder.append(java.lang.String):java.lang.StringBuilder
        0x0038: invoke-virtual java.lang.StringBuilder.append(java.lang.String):java.lang.StringBuilder
        0x003b: const-string v12, "&fromPageSubSource="
        0x003d: invoke-virtual java.lang.StringBuilder.append(java.lang.String):java.lang.StringBuilder
        0x0040: invoke-virtual java.lang.StringBuilder.append(java.lang.String):java.lang.StringBuilder
        0x0043: invoke-virtual java.lang.StringBuilder.toString():java.lang.String
        0x0047: const-class v12, o.EJm
        0x0049: invoke-static o.EIy.copydefault(java.lang.Class):o.EIN
        0x004e: check-cast v4, o.EJm
        0x0050: const/4 v7, 0
        0x0051: new-instance v8, o.aajq
        0x0053: invoke-direct o.aajq.<init>():void
        0x0056: const/4 v9, 4
        0x0057: const/4 v10, 0
        0x0059: invoke-static/range o.EJm$Activity.processDeeplink$default(o.EJm, android.content.Context, java.lang.String, java.util.Map, kotlin.jvm.functions.Function1, int, java.lang.Object):void
        0x005c: sget v12, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0062: sput v13, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0067: return-void
        0x0068: const/4 v12, 0
    }

    public final void OLrzqt(kotlin.jvm.functions.Function0 p0, kotlin.jvm.functions.Function0 p1, kotlin.jvm.functions.Function0 p2, kotlin.jvm.functions.Function0 p3) {
        0x0000: const/4 v0, 2
        0x0003: const-string v1, ""
        0x0005: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0008: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x000b: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x000e: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0011: invoke-virtual o.abQb.RcXXUw():androidx.databinding.ViewDataBinding
        0x0015: check-cast v7, o.XBx
        0x0017: iget v7, v7, o.XBx.gEmmrt:android.widget.ImageView
        0x0019: new-instance v8, o.aajh$Application
        0x001b: const-wide/16 v2, 1000
        0x001d: invoke-direct o.aajh$Application.<init>(android.view.View, long, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity):void
        0x0020: invoke-virtual android.view.View.setOnClickListener(android.view.View$OnClickListener):void
        0x0023: invoke-virtual o.abQb.RcXXUw():androidx.databinding.ViewDataBinding
        0x0027: check-cast v7, o.XBx
        0x0029: iget v7, v7, o.XBx.AYXKKw:android.widget.ImageView
        0x002b: new-instance v8, o.aajh$Activity
        0x002d: invoke-direct o.aajh$Activity.<init>(android.view.View, long, kotlin.jvm.functions.Function0):void
        0x0030: invoke-virtual android.view.View.setOnClickListener(android.view.View$OnClickListener):void
        0x0033: invoke-virtual o.abQb.RcXXUw():androidx.databinding.ViewDataBinding
        0x0037: check-cast v6, o.XBx
        0x0039: iget v6, v6, o.XBx.OLrzqt:o.APC
        0x003b: new-instance v7, o.aajt
        0x003d: invoke-direct o.aajt.<init>(com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity, kotlin.jvm.functions.Function0):void
        0x0040: invoke-virtual o.APC.setOnActionClick(kotlin.jvm.functions.Function1):void
        0x0043: invoke-static o.AtZ.OLrzqt():o.adaP
        0x0047: invoke-interface o.adaP.getValue():java.lang.Object
        0x004b: check-cast v5, o.Aqk
        0x004d: invoke-interface o.Aqk.djBIcL():boolean
        0x0051: invoke-virtual com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.OLrzqt(boolean):void
        0x0054: const-string v5, "trade_button_position"
        0x005a: invoke-static com.okinc.rxutils.RxBus.EZpvd(java.lang.String[]):o.acTJ
        0x005e: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullExpressionValue(java.lang.Object, java.lang.String):void
        0x0061: invoke-static o.acRQ.AEQbTJ(o.acTJ, androidx.lifecycle.LifecycleOwner):o.acTJ
        0x0065: new-instance v6, o.aajC
        0x0067: new-instance v7, o.aajy
        0x0069: invoke-direct o.aajy.<init>(com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity):void
        0x006c: invoke-direct o.aajC.<init>(kotlin.jvm.functions.Function1):void
        0x006f: invoke-virtual o.acTJ.OLrzqt(o.acUA):o.acUm
        0x0072: sget v5, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0078: sput v6, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x007b: return-void
    }

    public final void OLrzqt(o.abrD p0) {
        0x0000: const/4 v0, 2
        0x0003: invoke-static androidx.lifecycle.LifecycleOwnerKt.getLifecycleScope(androidx.lifecycle.LifecycleOwner):androidx.lifecycle.LifecycleCoroutineScope
        0x0007: const/4 v3, 0
        0x0008: const/4 v4, 0
        0x0009: new-instance v5, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity$loadTopPnlView$1
        0x000b: const/4 v1, 0
        0x000c: invoke-direct com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity$loadTopPnlView$1.<init>(com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity, o.abrD, kotlin.coroutines.Continuation):void
        0x000f: const/4 v6, 3
        0x0010: const/4 v7, 0
        0x0011: invoke-static/range kotlinx.coroutines.BuildersKt.launch$default(kotlinx.coroutines.CoroutineScope, kotlin.coroutines.CoroutineContext, kotlinx.coroutines.CoroutineStart, kotlin.jvm.functions.Function2, int, java.lang.Object):kotlinx.coroutines.Job
        0x0014: sget v9, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x001a: sput v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x001f: const/16 v9, 44
        0x0023: return-void
    }

    public final void OLrzqt(boolean p0) {
        0x0000: const/4 v0, 2
        0x0003: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0009: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x000e: iput v10, v9, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.gEmmrt:boolean
        0x0014: invoke-static o.Hac.copydefault():int
        0x0018: invoke-static o.Hac.copydefault():int
        0x001c: invoke-static o.Hac.copydefault():int
        0x0020: invoke-static o.Hac.copydefault():int
        0x0024: const v8, -482262387
        0x0027: const v7, 482262400
        0x002a: invoke-static/range com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.copydefault(int, int, java.lang.Object[], int, int, int, int):java.lang.Object
        0x002d: const/16 v10, 78
        0x0032: iput v10, v9, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.gEmmrt:boolean
        0x0038: invoke-static o.Hac.copydefault():int
        0x003c: invoke-static o.Hac.copydefault():int
        0x0040: invoke-static o.Hac.copydefault():int
        0x0044: invoke-static o.Hac.copydefault():int
        0x0048: const v7, -482262387
        0x004b: const v6, 482262400
        0x004e: invoke-static/range com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.copydefault(int, int, java.lang.Object[], int, int, int, int):java.lang.Object
        0x0051: sget v10, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0057: sput v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x005a: return-void
    }

    public void attachBaseContext(android.content.Context p0) {
        0x0002: const/4 v1, 2
        0x0005: sget v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x000b: sput v3, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x000e: const-string v2, ""
        0x0010: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0013: invoke-virtual/range androidx.appcompat.app.AppCompatActivity.getDelegate():androidx.appcompat.app.AppCompatDelegate
        0x0017: invoke-static o.AtZ.OLrzqt():o.adaP
        0x001b: invoke-interface o.adaP.getValue():java.lang.Object
        0x001f: check-cast v3, o.Aqk
        0x0021: invoke-interface o.Aqk.valueOf():int
        0x0025: invoke-virtual androidx.appcompat.app.AppCompatDelegate.setLocalNightMode(int):void
        0x0028: invoke-super/range o.uyI.attachBaseContext(android.content.Context):void
        0x002b: const/16 v2, 18
        0x002d: new-array v3, char[]
        0x0032: const/4 v4, 1
        0x0033: new-array v5, java.lang.Object[]
        0x0035: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.c(char[], int, java.lang.Object[]):void
        0x0038: const/4 v2, 0
        0x003d: invoke-static java.lang.Class.forName(java.lang.String):java.lang.Class
        0x0041: const/4 v5, 5
        0x0042: new-array v6, char[]
        0x0047: new-array v7, java.lang.Object[]
        0x0049: const/16 v8, 48
        0x004b: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.d(int, char[], byte, java.lang.Object[]):void
        0x0050: check-cast v6, java.lang.String
        0x0052: new-array v7, java.lang.Class[]
        0x0054: invoke-virtual java.lang.Class.getDeclaredMethod(java.lang.String, java.lang.Class[]):java.lang.reflect.Method
        0x0058: new-array v6, java.lang.Object[]
        0x005a: const/4 v7, 0
        0x005b: invoke-virtual java.lang.reflect.Method.invoke(java.lang.Object, java.lang.Object[]):java.lang.Object
        0x005f: check-cast v3, java.lang.Integer
        0x0061: invoke-virtual java.lang.Integer.intValue():int
        0x0065: const v6, 100000
        0x0069: const v6, 99000
        0x006e: const v6, 99999
        0x0075: instance-of v3, android.content.ContextWrapper
        0x0079: sget v3, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x007f: sput v6, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0083: check-cast v3, android.content.ContextWrapper
        0x0085: invoke-virtual android.content.ContextWrapper.getBaseContext():android.content.Context
        0x008e: invoke-virtual/range android.content.Context.getApplicationContext():android.content.Context
        0x0094: const v3, -1573426948
        0x0097: invoke-static o.OY.AEQbTJ(int):java.lang.Object
        0x009d: const/16 v9, 920
        0x009f: const/4 v10, 4
        0x00a0: const/16 v11, 1105
        0x00a2: const v12, 48949692
        0x00a5: const/4 v13, 0
        0x00a6: const-string v14, "OLrzqt"
        0x00a8: new-array v15, java.lang.Class[]
        0x00aa: invoke-static/range o.OY.KWHzl(char, int, int, int, boolean, java.lang.String, java.lang.Class[]):java.lang.Object
        0x00ae: check-cast v3, java.lang.reflect.Method
        0x00b0: invoke-virtual java.lang.reflect.Method.invoke(java.lang.Object, java.lang.Object[]):java.lang.Object
        0x00b4: new-array v6, char[]
        0x00b9: new-array v7, java.lang.Object[]
        0x00bb: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.c(char[], int, java.lang.Object[]):void
        0x00c0: check-cast v6, java.lang.String
        0x00c2: const/16 v7, 64
        0x00c4: new-array v8, char[]
        0x00c9: new-array v9, java.lang.Object[]
        0x00cb: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.c(char[], int, java.lang.Object[]):void
        0x00d2: new-array v9, char[]
        0x00d7: new-array v10, java.lang.Object[]
        0x00d9: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.c(char[], int, java.lang.Object[]):void
        0x00de: check-cast v7, java.lang.String
        0x00e0: const/16 v9, 60
        0x00e2: new-array v9, char[]
        0x00e7: new-array v10, java.lang.Object[]
        0x00e9: const/16 v11, 59
        0x00eb: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.c(char[], int, java.lang.Object[]):void
        0x00f0: check-cast v9, java.lang.String
        0x00f2: const/4 v10, 6
        0x00f3: new-array v11, char[]
        0x00f8: new-array v12, java.lang.Object[]
        0x00fa: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.c(char[], int, java.lang.Object[]):void
        0x00ff: check-cast v11, java.lang.String
        0x0101: const/16 v12, 36
        0x0103: new-array v12, char[]
        0x0108: new-array v13, java.lang.Object[]
        0x010a: const/16 v14, 36
        0x010c: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.c(char[], int, java.lang.Object[]):void
        0x0111: check-cast v12, java.lang.String
        0x0113: const/16 v13, 9
        0x0115: new-array v13, java.lang.Object[]
        0x0117: const v14, 86400
        0x011a: invoke-static java.lang.Integer.valueOf(int):java.lang.Integer
        0x011e: const/16 v15, 8
        0x0122: const/4 v14, 7
        0x0127: invoke-static java.lang.Boolean.valueOf(boolean):java.lang.Boolean
        0x012d: const/4 v11, 4
        0x0130: const/4 v9, 3
        0x0139: const v0, 1646393991
        0x013c: invoke-static o.OY.AEQbTJ(int):java.lang.Object
        0x0142: const/4 v15, 0
        0x0143: const/16 v16, 4
        0x0145: const/16 v17, 1101
        0x0147: const v18, -1023640633
        0x014a: const/16 v19, 0
        0x014c: const-string v20, "OLrzqt"
        0x014e: const/16 v0, 9
        0x0150: new-array v0, java.lang.Class[]
        0x0152: const-class v6, android.content.Context
    }

    public final androidx.fragment.app.Fragment copydefault(o.aajI p0) {
        0x0000: const/4 v0, 2
        0x0003: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0009: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x000c: invoke-static o.AtZ.OLrzqt():o.adaP
        0x0010: invoke-interface o.adaP.getValue():java.lang.Object
        0x0015: check-cast v2, o.Aqk
        0x0017: invoke-virtual o.aajI.AEQbTJ():java.lang.String
        0x001b: invoke-virtual o.aajI.copydefault():java.lang.String
        0x001f: invoke-virtual o.aajI.KWHzl():long
        0x0023: invoke-virtual o.aajI.EZpvd():kotlin.jvm.functions.Function0
        0x0027: invoke-interface/range o.Aqk.EZpvd(java.lang.String, java.lang.String, long, kotlin.jvm.functions.Function0):androidx.fragment.app.Fragment
        0x002b: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0031: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0034: return-object v9
    }

    public final void copydefault(java.lang.String p0, java.lang.String p1) {
        0x0000: const/4 v0, 2
        0x0003: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0009: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x000c: const/4 v2, 0
        0x000d: const-string v3, ""
        0x0011: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0014: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0017: invoke-direct com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.AuCTelauCTel():void
        0x001a: const-class v1, o.WpS
        0x001c: invoke-static o.EIy.KWHzl(java.lang.Class):o.EIM
        0x0020: check-cast v1, o.WpS
        0x0022: const/16 v3, 41
        0x0029: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x002c: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x002f: invoke-direct com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.AuCTelauCTel():void
        0x0032: const-class v1, o.WpS
        0x0034: invoke-static o.EIy.KWHzl(java.lang.Class):o.EIM
        0x0038: check-cast v1, o.WpS
        0x003c: sget v3, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0042: sput v4, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0047: sget v3, com.okinc.tradeapi.framework.constant.TradeKey.MANAGER:com.okinc.tradeapi.framework.constant.TradeKey
        0x0049: invoke-interface o.WpS.EZpvd(com.okinc.tradeapi.framework.constant.TradeKey):o.WpQ
        0x004d: check-cast v1, o.abTZ
        0x004f: const/16 v3, 78
        0x0056: sget v3, com.okinc.tradeapi.framework.constant.TradeKey.MANAGER:com.okinc.tradeapi.framework.constant.TradeKey
        0x0058: invoke-interface o.WpS.EZpvd(com.okinc.tradeapi.framework.constant.TradeKey):o.WpQ
        0x005c: check-cast v1, o.abTZ
        0x0060: invoke-interface o.abTZ.AkhnZx():boolean
        0x0064: const/4 v3, 1
        0x0067: iget v13, v12, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.valueOf:o.PPT
        0x006b: new-instance v14, o.aaje
        0x006d: invoke-direct o.aaje.<init>(com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity):void
        0x0070: invoke-static o.PII$TaskDescription.setSrcFrom$default(o.PII, java.lang.String, kotlin.jvm.functions.Function0, int, java.lang.Object):void
        0x0073: return-void
        0x0074: new-instance v1, java.lang.StringBuilder
        0x0076: invoke-direct java.lang.StringBuilder.<init>():void
        0x0079: const-string v3, "okx://pro/trade_kline?id="
        0x007b: invoke-virtual java.lang.StringBuilder.append(java.lang.String):java.lang.StringBuilder
        0x007e: invoke-virtual java.lang.StringBuilder.append(java.lang.String):java.lang.StringBuilder
        0x0081: const-string v3, "&type="
        0x0083: invoke-virtual java.lang.StringBuilder.append(java.lang.String):java.lang.StringBuilder
        0x0086: invoke-virtual java.lang.StringBuilder.append(java.lang.String):java.lang.StringBuilder
        0x0089: const-string v3, "&source=share"
        0x008b: invoke-virtual java.lang.StringBuilder.append(java.lang.String):java.lang.StringBuilder
        0x008e: invoke-virtual java.lang.StringBuilder.toString():java.lang.String
        0x0092: invoke-virtual com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.KWHzl(java.lang.String, java.lang.String):java.lang.String
        0x0096: iget v14, v12, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.valueOf:o.PPT
        0x009a: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x00a0: sput v3, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x00a5: invoke-static o.PKf.copydefault():o.PKg
        0x00a9: invoke-interface o.PKg.KWHzl():o.PLv
        0x00ad: invoke-interface o.PLv.djBIcL():java.lang.String
        0x00b1: invoke-virtual o.PPR.setQRCode(java.lang.String):void
        0x00b5: invoke-static o.PKf.copydefault():o.PKg
        0x00b9: invoke-interface o.PKg.KWHzl():o.PLv
        0x00bd: invoke-interface o.PLv.djBIcL():java.lang.String
        0x00c1: invoke-virtual o.PPR.setQRCode(java.lang.String):void
        0x00c5: invoke-static o.PKf.copydefault():o.PKg
        0x00c9: new-instance v0, o.PIH
        0x00cb: const-string v5, "BOT_ORDER_KLINE_SHARE_FROM"
        0x00cd: const/4 v7, 0
        0x00ce: const/4 v8, 0
        0x00cf: const/4 v9, 0
        0x00d0: const/16 v10, 28
        0x00d2: const/4 v11, 0
        0x00d4: invoke-direct/range o.PIH.<init>(java.lang.String, java.lang.String, boolean, com.okinc.share.api.RedirectBehavior, java.lang.String, int, kotlin.jvm.internal.DefaultConstructorMarker):void
        0x00d7: invoke-interface o.PKg.EZpvd(java.lang.String, o.PIH):o.acTJ
        0x00db: invoke-static o.acRQ.AEQbTJ(o.acTJ, androidx.lifecycle.LifecycleOwner):o.acTJ
        0x00df: new-instance v14, o.aajk
        0x00e1: new-instance v0, o.aaji
        0x00e3: invoke-direct o.aaji.<init>(com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity):void
        0x00e6: invoke-direct o.aajk.<init>(kotlin.jvm.functions.Function1):void
        0x00e9: new-instance v0, o.aajn
        0x00eb: invoke-direct o.aajn.<init>():void
        0x00ee: new-instance v1, o.aajl
        0x00f0: invoke-direct o.aajl.<init>(kotlin.jvm.functions.Function1):void
        0x00f3: invoke-virtual o.acTJ.EZpvd(o.acUA, o.acUA):o.acUm
        0x00f6: return-void
    }

    public final void ejfBZ() {
        0x0004: invoke-static o.Hac.copydefault():int
        0x0008: invoke-static o.Hac.copydefault():int
        0x000c: invoke-static o.Hac.copydefault():int
        0x0010: invoke-static o.Hac.copydefault():int
        0x0014: const v6, -482262387
        0x0017: const v5, 482262400
        0x001a: invoke-static/range com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.copydefault(int, int, java.lang.Object[], int, int, int, int):java.lang.Object
        0x001d: return-void
    }

    public final void fARcdN() {
        0x0004: invoke-static o.Hac.copydefault():int
        0x0008: invoke-static o.Hac.copydefault():int
        0x000c: invoke-static o.Hac.copydefault():int
        0x0010: invoke-static o.Hac.copydefault():int
        0x0014: const v6, 1678229524
        0x0017: const v5, -1678229519
        0x001a: invoke-static/range com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.copydefault(int, int, java.lang.Object[], int, int, int, int):java.lang.Object
        0x001d: return-void
    }

    public final void fIwbmz() {
        0x0000: const/4 v0, 2
        0x0003: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0009: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x000c: const-string v2, "bot_kline_setting_tips_shown"
        0x0010: const/4 v1, 1
        0x0011: invoke-static com.okinc.core.util.SPUtils.getBoolean(java.lang.String, boolean):boolean
        0x0018: const/4 v1, 0
        0x0019: invoke-static com.okinc.core.util.SPUtils.getBoolean(java.lang.String, boolean):boolean
        0x001f: invoke-virtual o.abQb.RcXXUw():androidx.databinding.ViewDataBinding
        0x0023: check-cast v1, o.XBx
        0x0025: iget v1, v1, o.XBx.OLrzqt:o.APC
        0x0027: new-instance v2, o.aaju
        0x0029: invoke-direct o.aaju.<init>(com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity):void
        0x002c: invoke-virtual android.view.View.post(java.lang.Runnable):boolean
        0x002f: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0035: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0038: return-void
    }

    public final void fJNWhG() {
        0x0000: const/4 v0, 2
        0x0003: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0009: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x000c: iget v1, v3, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.EZpvd:androidx.fragment.app.Fragment
        0x0010: invoke-static o.AtZ.OLrzqt():o.adaP
        0x0014: invoke-interface o.adaP.getValue():java.lang.Object
        0x0018: check-cast v2, o.Aqk
        0x001a: invoke-interface o.Aqk.AEQbTJ(androidx.fragment.app.Fragment):void
        0x001d: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0023: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0026: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x002c: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0031: return-void
        0x0032: const/4 v0, 0
    }

    public final androidx.fragment.app.Fragment fetchVPNInfo() {
        0x0000: const/4 v0, 2
        0x0003: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0009: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x000c: iget v1, v4, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.EZpvd:androidx.fragment.app.Fragment
        0x0012: sput v3, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0017: return-object v1
        0x0018: const/4 v0, 0
    }

    public final void gEmmrt() {
        0x0000: const/4 v0, 2
        0x0003: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0009: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x000e: iget v1, v8, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.KWHzl:boolean
        0x0010: const/16 v2, 1024
        0x0014: invoke-virtual android.app.Activity.getWindow():android.view.Window
        0x0018: invoke-virtual android.view.Window.clearFlags(int):void
        0x001c: invoke-virtual android.app.Activity.getWindow():android.view.Window
        0x0020: invoke-virtual android.view.Window.addFlags(int):void
        0x0023: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0029: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x002c: iget v1, v8, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.KWHzl:boolean
        0x0030: const/4 v1, 0
        0x0032: const/16 v1, 8
        0x0034: invoke-virtual o.abQb.RcXXUw():androidx.databinding.ViewDataBinding
        0x0038: check-cast v2, o.XBx
        0x003a: iget v2, v2, o.XBx.copydefault:androidx.constraintlayout.widget.ConstraintLayout
        0x003c: invoke-virtual android.view.View.setVisibility(int):void
        0x003f: invoke-virtual o.abQb.RcXXUw():androidx.databinding.ViewDataBinding
        0x0043: check-cast v2, o.XBx
        0x0045: iget v2, v2, o.XBx.fetchVPNInfo:androidx.appcompat.widget.LinearLayoutCompat
        0x0047: invoke-virtual android.view.View.setVisibility(int):void
        0x004a: iget v1, v8, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.KWHzl:boolean
        0x004e: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0054: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0057: invoke-static o.AtZ.OLrzqt():o.adaP
        0x005b: invoke-interface o.adaP.getValue():java.lang.Object
        0x005f: check-cast v0, o.Aqk
        0x0061: invoke-interface o.Aqk.djBIcL():boolean
        0x0065: invoke-virtual com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.OLrzqt(boolean):void
        0x0068: return-void
        0x006d: invoke-static o.Hac.copydefault():int
        0x0071: invoke-static o.Hac.copydefault():int
        0x0075: invoke-static o.Hac.copydefault():int
        0x0079: invoke-static o.Hac.copydefault():int
        0x007d: const v7, -482262387
        0x0080: const v6, 482262400
        0x0083: invoke-static/range com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.copydefault(int, int, java.lang.Object[], int, int, int, int):java.lang.Object
        0x0086: return-void
        0x0087: const/4 v0, 0
    }

    public final o.aajI isConnected() {
        0x0004: invoke-static o.Hac.copydefault():int
        0x0008: invoke-static o.Hac.copydefault():int
        0x000c: invoke-static o.Hac.copydefault():int
        0x0010: invoke-static o.Hac.copydefault():int
        0x0014: const v6, 520186186
        0x0017: const v5, -520186176
        0x001a: invoke-static/range com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.copydefault(int, int, java.lang.Object[], int, int, int, int):java.lang.Object
        0x001e: check-cast v0, o.aajI
        0x0020: return-object v0
    }

    public final void iwGUEr() {
        0x0000: const/4 v0, 2
        0x0003: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0009: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x000c: iget v1, v3, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.KWHzl:boolean
        0x0010: return-void
        0x0011: const/4 v1, 1
        0x0012: iput v1, v3, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.KWHzl:boolean
        0x0014: invoke-virtual o.uyI.setRequestedOrientation(int):void
        0x0017: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x001d: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0022: return-void
        0x0023: const/4 v0, 0
    }

    public void onConfigurationChanged(android.content.res.Configuration p0) {
        0x0000: const/4 v0, 2
        0x0003: const-string v1, ""
        0x0005: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0008: invoke-super o.uyI.onConfigurationChanged(android.content.res.Configuration):void
        0x000b: iget v9, v8, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.KWHzl:boolean
        0x000d: invoke-virtual o.uyI.setRequestedOrientation(int):void
        0x0010: invoke-virtual com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.gEmmrt():void
        0x0013: iget v9, v8, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.DbNXlk:o.Xsh
        0x0017: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x001d: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0020: invoke-virtual o.Xsh.EZpvd():o.aajI
        0x002a: invoke-static o.Hac.copydefault():int
        0x002e: invoke-static o.Hac.copydefault():int
        0x0032: invoke-static o.Hac.copydefault():int
        0x0036: invoke-static o.Hac.copydefault():int
        0x003a: const v7, 520186186
        0x003d: const v6, -520186176
        0x0040: invoke-static/range com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.copydefault(int, int, java.lang.Object[], int, int, int, int):java.lang.Object
        0x0044: check-cast v9, o.aajI
        0x0046: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x004c: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0053: invoke-virtual com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.AEQbTJ(o.aajI):void
        0x0056: return-void
    }

    public void onCreate(android.os.Bundle p0) {
        0x0002: const/4 v0, 2
        0x0005: const/16 v2, 18
        0x0007: new-array v3, char[]
        0x000c: const/4 v4, 1
        0x000d: new-array v5, java.lang.Object[]
        0x000f: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.c(char[], int, java.lang.Object[]):void
        0x0012: const/4 v3, 0
        0x0017: invoke-static java.lang.Class.forName(java.lang.String):java.lang.Class
        0x001b: const/4 v6, 5
        0x001c: new-array v7, char[]
        0x0021: new-array v8, java.lang.Object[]
        0x0023: const/16 v9, 48
        0x0025: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.d(int, char[], byte, java.lang.Object[]):void
        0x002c: new-array v8, java.lang.Class[]
        0x002e: invoke-virtual java.lang.Class.getDeclaredMethod(java.lang.String, java.lang.Class[]):java.lang.reflect.Method
        0x0032: new-array v7, java.lang.Object[]
        0x0034: const/4 v8, 0
        0x0035: invoke-virtual java.lang.reflect.Method.invoke(java.lang.Object, java.lang.Object[]):java.lang.Object
        0x0039: check-cast v5, java.lang.Integer
        0x003b: invoke-virtual java.lang.Integer.intValue():int
        0x003f: const v7, 100000
        0x0043: const v7, 99000
        0x0048: sget v7, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x004e: sput v10, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0051: const v10, 99999
        0x0056: const/16 v7, 9
        0x005e: invoke-virtual/range com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.getBaseContext():android.content.Context
        0x0064: const/16 v5, 26
        0x0066: new-array v5, char[]
        0x006b: new-array v7, java.lang.Object[]
        0x006d: const/16 v10, 26
        0x006f: const/16 v11, 24
        0x0071: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.d(int, char[], byte, java.lang.Object[]):void
        0x0076: check-cast v5, java.lang.String
        0x0078: invoke-static java.lang.Class.forName(java.lang.String):java.lang.Class
        0x007c: new-array v7, char[]
        0x0081: new-array v10, java.lang.Object[]
        0x0083: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.c(char[], int, java.lang.Object[]):void
        0x0088: check-cast v2, java.lang.String
        0x008a: new-array v7, java.lang.Class[]
        0x008c: invoke-virtual java.lang.Class.getMethod(java.lang.String, java.lang.Class[]):java.lang.reflect.Method
        0x0091: check-cast v5, java.lang.Object[]
        0x0093: invoke-virtual java.lang.reflect.Method.invoke(java.lang.Object, java.lang.Object[]):java.lang.Object
        0x0098: check-cast v5, android.content.Context
        0x009c: sget v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x00a2: sput v7, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x00a7: instance-of v2, android.content.ContextWrapper
        0x00a9: const/16 v7, 48
        0x00af: instance-of v2, android.content.ContextWrapper
        0x00b5: check-cast v2, android.content.ContextWrapper
        0x00b7: invoke-virtual android.content.ContextWrapper.getBaseContext():android.content.Context
        0x00bd: invoke-virtual android.content.Context.getApplicationContext():android.content.Context
        0x00c2: sget v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x00c8: sput v5, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x00ce: const v2, -1573426948
        0x00d1: invoke-static o.OY.AEQbTJ(int):java.lang.Object
        0x00d7: const/16 v10, 920
        0x00d9: const/4 v11, 4
        0x00da: const/16 v12, 1105
        0x00dc: const v13, 48949692
        0x00df: const/4 v14, 0
        0x00e0: const-string v15, "OLrzqt"
        0x00e2: new-array v2, java.lang.Class[]
        0x00e6: invoke-static/range o.OY.KWHzl(char, int, int, int, boolean, java.lang.String, java.lang.Class[]):java.lang.Object
        0x00ea: check-cast v2, java.lang.reflect.Method
        0x00ec: invoke-virtual java.lang.reflect.Method.invoke(java.lang.Object, java.lang.Object[]):java.lang.Object
        0x00f0: new-array v7, char[]
        0x00f5: new-array v10, java.lang.Object[]
        0x00f7: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.c(char[], int, java.lang.Object[]):void
        0x00fc: check-cast v7, java.lang.String
        0x00fe: const/16 v9, 64
        0x0100: new-array v10, char[]
        0x0105: new-array v11, java.lang.Object[]
        0x0107: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.c(char[], int, java.lang.Object[]):void
        0x010c: check-cast v10, java.lang.String
        0x010e: new-array v11, char[]
        0x0113: new-array v12, java.lang.Object[]
        0x0115: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.c(char[], int, java.lang.Object[]):void
        0x011a: check-cast v9, java.lang.String
        0x011c: const/16 v11, 60
        0x011e: new-array v11, char[]
        0x0123: new-array v12, java.lang.Object[]
        0x0125: const/16 v13, 59
        0x0127: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.c(char[], int, java.lang.Object[]):void
        0x012c: check-cast v11, java.lang.String
        0x012e: const/4 v12, 6
        0x012f: new-array v13, char[]
        0x0134: new-array v14, java.lang.Object[]
        0x0136: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.c(char[], int, java.lang.Object[]):void
        0x013a: return-void
        0x013b: check-cast v13, java.lang.String
        0x013d: const/16 v14, 36
        0x013f: new-array v14, char[]
        0x0144: new-array v15, java.lang.Object[]
        0x0146: const/16 v8, 36
        0x0148: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.c(char[], int, java.lang.Object[]):void
        0x014c: return v3
        0x014d: check-cast v8, java.lang.String
        0x014f: const/16 v14, 9
        0x0151: new-array v15, java.lang.Object[]
        0x0153: const v17, 86400
        0x0156: invoke-static/range java.lang.Integer.valueOf(int):java.lang.Integer
        0x015a: const/16 v18, 8
        0x015d: return v18
        0x015e: const/16 v17, 7
        0x0161: return v17
        0x0163: return v12
        0x0164: invoke-static java.lang.Boolean.valueOf(boolean):java.lang.Boolean
        0x0169: return v6
        0x016a: const/4 v8, 4
        0x016c: return v8
        0x016d: const/4 v11, 3
        0x016f: return v11
        0x0171: return v0
        0x0173: return v4
        0x0175: return v3
        0x0176: const v5, 1646393991
        0x0179: invoke-static o.OY.AEQbTJ(int):java.lang.Object
        0x017f: const/16 v19, 0
        0x0181: const/16 v20, 4
        0x0183: const/16 v21, 1101
        0x0185: const v22, -1023640633
        0x0188: const/16 v23, 0
        0x018a: const-string v24, "OLrzqt"
        0x018c: new-array v5, java.lang.Class[]
        0x018e: const-class v7, android.content.Context
        0x01a3: const-wide/32 v27, 100991821
        0x01a6: const-class v3, java.lang.String
        0x01af: const-wide v27, 1434897336304461
        0x01b4: invoke-static/range o.OY.KWHzl(char, int, int, int, boolean, java.lang.String, java.lang.Class[]):java.lang.Object
        0x01b8: check-cast v5, java.lang.reflect.Method
        0x01ba: invoke-virtual java.lang.reflect.Method.invoke(java.lang.Object, java.lang.Object[]):java.lang.Object
        0x01bd: sget v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x01c3: sput v3, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x01c8: invoke-virtual java.lang.Throwable.getCause():java.lang.Throwable
        0x01d0: sget v2, o.XrV.AEQbTJ:o.XrV
        0x01d2: invoke-virtual/range android.app.Activity.getIntent():android.content.Intent
        0x01d6: const-string v5, ""
        0x01d8: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullExpressionValue(java.lang.Object, java.lang.String):void
        0x01db: invoke-virtual o.XrV.copydefault(android.content.Intent):boolean
        0x01df: iput v2, v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.isConnected:boolean
        0x01e1: invoke-super/range o.aajQ.onCreate(android.os.Bundle):void
        0x01e4: iget v2, v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.isConnected:boolean
        0x01e8: new-instance v2, o.Xsh
        0x01ea: invoke-virtual/range o.abQb.RcXXUw():androidx.databinding.ViewDataBinding
        0x01ee: check-cast v3, o.XBx
        0x01f0: invoke-direct o.Xsh.<init>(com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity, o.XBx):void
        0x01f3: iput v2, v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.DbNXlk:o.Xsh
        0x01f5: invoke-virtual o.Xsh.OLrzqt():void
        0x01fc: invoke-static com.appsflyer.internal.AFi1bSDK$3$$ExternalSyntheticLambda0.EZpvd():int
        0x0200: invoke-static o.Hac.copydefault():int
        0x0204: invoke-static o.Hac.copydefault():int
        0x0208: invoke-static o.Hac.copydefault():int
        0x020c: const v11, 487838831
        0x020f: const v10, -487838813
        0x0212: invoke-static/range com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.copydefault(int, int, java.lang.Object[], int, int, int, int):java.lang.Object
        0x0215: sget v2, o.absU.EZpvd:o.absU
        0x0217: invoke-virtual/range android.app.Activity.getIntent():android.content.Intent
        0x021b: const-string v5, "source"
        0x021d: invoke-virtual android.content.Intent.getStringExtra(java.lang.String):java.lang.String
        0x0223: const-string v3, ""
        0x0225: invoke-virtual o.absU.valueOf(java.lang.String):void
        0x0228: const/4 v2, 0
        0x0229: invoke-static o.Fzc.reportFullyDrawn$default(android.app.Activity, boolean, java.lang.String, int, java.lang.Object):void
        0x022c: return-void
        0x0231: invoke-static o.Hac.copydefault():int
        0x0235: invoke-static com.appsflyer.internal.AFi1bSDK$3$$ExternalSyntheticLambda0.EZpvd():int
        0x0239: invoke-static com.appsflyer.internal.AFi1bSDK$3$$ExternalSyntheticLambda0.EZpvd():int
        0x023d: invoke-static com.appsflyer.internal.AFi1bSDK$3$$ExternalSyntheticLambda0.EZpvd():int
        0x0241: const v11, 2103153730
        0x0244: const v10, -2103153716
        0x0247: invoke-static/range com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.copydefault(int, int, java.lang.Object[], int, int, int, int):java.lang.Object
        0x024a: return-void
    }

    public void onDestroy() {
        0x0000: const/4 v0, 2
        0x0003: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0009: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x000c: iget v1, v3, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.DbNXlk:o.Xsh
        0x0010: invoke-virtual android.app.Activity.isChangingConfigurations():boolean
        0x0014: invoke-virtual o.Xsh.EZpvd(boolean):void
        0x0017: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x001d: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0020: const/4 v0, 0
        0x0021: iput v0, v3, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.DbNXlk:o.Xsh
        0x0023: invoke-super o.aajQ.onDestroy():void
        0x0026: return-void
    }

    public void onNewIntent(android.content.Intent p0) {
        0x0000: const/4 v0, 2
        0x0003: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0009: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x000c: const-string v2, ""
        0x0010: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0013: invoke-super o.uyI.onNewIntent(android.content.Intent):void
        0x0016: sget v1, o.XrV.AEQbTJ:o.XrV
        0x0018: invoke-virtual o.XrV.copydefault(android.content.Intent):boolean
        0x001c: iget v3, v8, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.isConnected:boolean
        0x001e: const/16 v4, 82
        0x0025: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0028: invoke-super o.uyI.onNewIntent(android.content.Intent):void
        0x002b: sget v1, o.XrV.AEQbTJ:o.XrV
        0x002d: invoke-virtual o.XrV.copydefault(android.content.Intent):boolean
        0x0031: iget v3, v8, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.isConnected:boolean
        0x0037: invoke-virtual o.XrV.OLrzqt(android.content.Intent):java.lang.String
        0x003b: iget v4, v8, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.DbNXlk:o.Xsh
        0x003d: const/4 v5, 0
        0x0040: sget v6, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0046: sput v7, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x004b: invoke-virtual o.Xsh.AEQbTJ():java.lang.String
        0x0050: invoke-virtual o.Xsh.AEQbTJ():java.lang.String
        0x0053: invoke-virtual java.lang.Object.hashCode():int
        0x0057: sget v4, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x005d: sput v6, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0060: invoke-static kotlin.jvm.internal.Intrinsics.KWHzl(java.lang.Object, java.lang.Object):boolean
        0x0068: sget v0, o.XrX.copydefault:o.XrX
        0x006a: invoke-virtual o.XrV.AEQbTJ(android.content.Intent):long
        0x006e: invoke-virtual o.XrX.KWHzl(long):void
        0x0071: return-void
        0x0072: invoke-virtual android.app.Activity.setIntent(android.content.Intent):void
        0x0075: invoke-virtual o.uyI.recreate():void
        0x0078: return-void
    }

    public void onPause() {
        0x0000: const/4 v0, 2
        0x0003: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0009: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x000c: const/4 v2, 0
        0x000f: invoke-virtual com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.getBaseContext():android.content.Context
        0x0013: const/4 v3, 1
        0x0014: const/4 v4, 0
        0x0017: const/16 v1, 26
        0x0019: new-array v5, char[]
        0x001e: new-array v6, java.lang.Object[]
        0x0020: const/16 v7, 24
        0x0022: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.d(int, char[], byte, java.lang.Object[]):void
        0x0029: invoke-static java.lang.Class.forName(java.lang.String):java.lang.Class
        0x002d: const/16 v5, 18
        0x002f: new-array v6, char[]
        0x0034: new-array v7, java.lang.Object[]
        0x0036: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.c(char[], int, java.lang.Object[]):void
        0x003b: check-cast v5, java.lang.String
        0x003d: new-array v6, java.lang.Class[]
        0x003f: invoke-virtual java.lang.Class.getMethod(java.lang.String, java.lang.Class[]):java.lang.reflect.Method
        0x0044: check-cast v5, java.lang.Object[]
        0x0046: invoke-virtual java.lang.reflect.Method.invoke(java.lang.Object, java.lang.Object[]):java.lang.Object
        0x004a: check-cast v1, android.content.Context
        0x004c: sget v5, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0052: sput v6, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0057: sget v5, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x005d: sput v6, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0062: instance-of v5, android.content.ContextWrapper
        0x0064: const/16 v6, 82
        0x006a: instance-of v5, android.content.ContextWrapper
        0x006f: check-cast v5, android.content.ContextWrapper
        0x0071: invoke-virtual android.content.ContextWrapper.getBaseContext():android.content.Context
        0x007a: invoke-virtual android.content.Context.getApplicationContext():android.content.Context
        0x0080: const v5, -1573426948
        0x0083: invoke-static o.OY.AEQbTJ(int):java.lang.Object
        0x0089: const/16 v6, 920
        0x008b: const/4 v7, 4
        0x008c: const/16 v8, 1105
        0x008e: const v9, 48949692
        0x0091: const/4 v10, 0
        0x0092: const-string v11, "OLrzqt"
        0x0094: new-array v12, java.lang.Class[]
        0x0096: invoke-static/range o.OY.KWHzl(char, int, int, int, boolean, java.lang.String, java.lang.Class[]):java.lang.Object
        0x009a: check-cast v5, java.lang.reflect.Method
        0x009c: invoke-virtual java.lang.reflect.Method.invoke(java.lang.Object, java.lang.Object[]):java.lang.Object
        0x00a4: const v5, -722776020
        0x00a7: invoke-static o.OY.AEQbTJ(int):java.lang.Object
        0x00ad: const/4 v6, 0
        0x00ae: const/4 v7, 4
        0x00af: const/16 v8, 1101
        0x00b1: const v9, 1949749612
        0x00b4: const/4 v10, 0
        0x00b5: const-string v11, "KWHzl"
        0x00b7: new-array v12, java.lang.Class[]
        0x00b9: const-class v3, android.content.Context
        0x00bd: invoke-static/range o.OY.KWHzl(char, int, int, int, boolean, java.lang.String, java.lang.Class[]):java.lang.Object
        0x00c1: check-cast v5, java.lang.reflect.Method
        0x00c3: invoke-virtual java.lang.reflect.Method.invoke(java.lang.Object, java.lang.Object[]):java.lang.Object
        0x00c6: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x00cc: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x00d1: invoke-virtual java.lang.Throwable.getCause():java.lang.Throwable
        0x00d9: invoke-super o.aajQ.onPause():void
        0x00dc: return-void
        0x00dd: invoke-virtual com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.getBaseContext():android.content.Context
    }

    public void onResume() {
        0x0000: const/4 v0, 2
        0x0003: invoke-virtual com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.getBaseContext():android.content.Context
        0x0007: const/4 v2, 1
        0x0008: const/4 v3, 0
        0x0009: const/4 v4, 0
        0x000c: const/16 v1, 26
        0x000e: new-array v5, char[]
        0x0013: new-array v6, java.lang.Object[]
        0x0015: const/16 v7, 24
        0x0017: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.d(int, char[], byte, java.lang.Object[]):void
        0x001e: invoke-static java.lang.Class.forName(java.lang.String):java.lang.Class
        0x0022: const/16 v5, 18
        0x0024: new-array v6, char[]
        0x0029: new-array v7, java.lang.Object[]
        0x002b: invoke-static com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.c(char[], int, java.lang.Object[]):void
        0x0030: check-cast v5, java.lang.String
        0x0032: new-array v6, java.lang.Class[]
        0x0034: invoke-virtual java.lang.Class.getMethod(java.lang.String, java.lang.Class[]):java.lang.reflect.Method
        0x0039: check-cast v5, java.lang.Object[]
        0x003b: invoke-virtual java.lang.reflect.Method.invoke(java.lang.Object, java.lang.Object[]):java.lang.Object
        0x003f: check-cast v1, android.content.Context
        0x0043: instance-of v5, android.content.ContextWrapper
        0x0048: sget v5, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x004e: sput v6, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0052: check-cast v5, android.content.ContextWrapper
        0x0054: invoke-virtual android.content.ContextWrapper.getBaseContext():android.content.Context
        0x005a: invoke-virtual android.content.Context.getApplicationContext():android.content.Context
        0x005f: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0065: sput v5, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x006b: sget v5, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0071: sput v6, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0074: const v6, -369943057
        0x0077: const v7, -1573426948
        0x007c: invoke-static o.OY.AEQbTJ(int):java.lang.Object
        0x0082: const/16 v7, 920
        0x0084: const/4 v8, 4
        0x0085: const/16 v9, 1105
        0x0087: const v10, 48949692
        0x008a: const/4 v11, 0
        0x008b: const-string v12, "OLrzqt"
        0x008d: new-array v13, java.lang.Class[]
        0x008f: invoke-static/range o.OY.KWHzl(char, int, int, int, boolean, java.lang.String, java.lang.Class[]):java.lang.Object
        0x0093: check-cast v0, java.lang.reflect.Method
        0x0095: invoke-virtual java.lang.reflect.Method.invoke(java.lang.Object, java.lang.Object[]):java.lang.Object
        0x009d: invoke-static o.OY.AEQbTJ(int):java.lang.Object
        0x00a3: const/4 v6, 0
        0x00a4: const/4 v7, 4
        0x00a5: const/16 v8, 1101
        0x00a7: const v9, 1227791535
        0x00aa: const/4 v10, 0
        0x00ab: const-string v11, "AEQbTJ"
        0x00ad: new-array v12, java.lang.Class[]
        0x00af: const-class v2, android.content.Context
        0x00b3: invoke-static/range o.OY.KWHzl(char, int, int, int, boolean, java.lang.String, java.lang.Class[]):java.lang.Object
        0x00b7: check-cast v5, java.lang.reflect.Method
        0x00b9: invoke-virtual java.lang.reflect.Method.invoke(java.lang.Object, java.lang.Object[]):java.lang.Object
        0x00bd: invoke-static o.OY.AEQbTJ(int):java.lang.Object
        0x00c3: const/16 v7, 920
        0x00c5: const/4 v8, 4
        0x00c6: const/16 v9, 1105
        0x00c8: const v10, 48949692
        0x00cb: const/4 v11, 0
        0x00cc: const-string v12, "OLrzqt"
        0x00ce: new-array v13, java.lang.Class[]
        0x00d0: invoke-static/range o.OY.KWHzl(char, int, int, int, boolean, java.lang.String, java.lang.Class[]):java.lang.Object
        0x00d4: check-cast v5, java.lang.reflect.Method
        0x00d6: invoke-virtual java.lang.reflect.Method.invoke(java.lang.Object, java.lang.Object[]):java.lang.Object
        0x00de: invoke-static o.OY.AEQbTJ(int):java.lang.Object
        0x00e4: const/4 v6, 0
        0x00e5: const/4 v7, 4
        0x00e6: const/16 v8, 1101
        0x00e8: const v9, 1227791535
        0x00eb: const/4 v10, 0
        0x00ec: const-string v11, "AEQbTJ"
        0x00ee: new-array v12, java.lang.Class[]
        0x00f0: const-class v2, android.content.Context
        0x00f4: invoke-static/range o.OY.KWHzl(char, int, int, int, boolean, java.lang.String, java.lang.Class[]):java.lang.Object
        0x00f8: check-cast v5, java.lang.reflect.Method
        0x00fa: invoke-virtual java.lang.reflect.Method.invoke(java.lang.Object, java.lang.Object[]):java.lang.Object
        0x00ff: invoke-virtual java.lang.Throwable.getCause():java.lang.Throwable
        0x0107: invoke-super o.uyI.onResume():void
        0x010a: iget v1, v14, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.DbNXlk:o.Xsh
        0x010e: sget v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0114: sput v3, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x0117: invoke-virtual o.Xsh.djBIcL():void
        0x011a: return-void
        0x011f: invoke-static o.Hac.copydefault():int
        0x0123: invoke-static o.Hac.copydefault():int
        0x0127: invoke-static o.Hac.copydefault():int
        0x012b: invoke-static o.Hac.copydefault():int
        0x012f: const v9, 42588235
        0x0132: const v8, -42588228
        0x0135: invoke-static/range com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.copydefault(int, int, java.lang.Object[], int, int, int, int):java.lang.Object
        0x0138: return-void
    }

    public void onStart() {
        0x0000: const/4 v0, 2
        0x0003: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0009: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x000c: invoke-super o.aajQ.onStart():void
        0x0011: const/16 v0, 56
        0x0015: return-void
    }

    public final void uzCIH() {
        0x0000: const/4 v0, 2
        0x0003: sget v1, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.hDKMBd:int
        0x0009: sput v3, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x000c: iget v2, v4, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.KWHzl:boolean
        0x000e: const/4 v3, 0
        0x0015: sput v2, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.uzCIH:int
        0x001a: const/16 v0, 90
        0x001d: return-void
        0x001e: iput v3, v4, com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.KWHzl:boolean
        0x0020: invoke-virtual o.uyI.setRequestedOrientation(int):void
        0x0023: return-void
    }

    public final void values() {
        0x0004: invoke-static o.Hac.copydefault():int
        0x0008: invoke-static com.appsflyer.internal.AFi1bSDK$3$$ExternalSyntheticLambda0.EZpvd():int
        0x000c: invoke-static com.appsflyer.internal.AFi1bSDK$3$$ExternalSyntheticLambda0.EZpvd():int
        0x0010: invoke-static com.appsflyer.internal.AFi1bSDK$3$$ExternalSyntheticLambda0.EZpvd():int
        0x0014: const v6, 2103153730
        0x0017: const v5, -2103153716
        0x001a: invoke-static/range com.okinc.tradingbot.impl.strategy.BotOrderKlineActivity.copydefault(int, int, java.lang.Object[], int, int, int, int):java.lang.Object
        0x001d: return-void
    }

}

// DEX: classes30.dex | Source: ProGuard/R8
public final class com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment /* obfuscated: Lo/aaHX; */ extends o.abQe {
    // Static Fields
    public static final int AEQbTJ;
    public static final o.aaHX$ActionBar Companion;

    // Instance Fields
    public final o.adaP EZpvd;
    public final com.okinc.tradingbot.impl.strategy.init.MarketPlaceInitPresenter KWHzl;
    public final o.EJT copydefault;

    static void <clinit>() {
        0x0000: sget v0, o.Bow.KWHzl:o.Bow
        0x0002: sget v1, com.okinc.localization.report.SupportType.CLASS:com.okinc.localization.report.SupportType
        0x0004: const-string v2, "com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment"
        0x0006: const/4 v3, 0
        0x0007: invoke-virtual o.Bow.copydefault(java.lang.String, com.okinc.localization.report.SupportType, java.lang.String):void
        0x000a: new-instance v0, o.aaHX$ActionBar
        0x000c: invoke-direct o.aaHX$ActionBar.<init>(kotlin.jvm.internal.DefaultConstructorMarker):void
        0x000f: sput v0, com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment.Companion:o.aaHX$ActionBar
        0x0011: const/16 v0, 8
        0x0013: sput v0, com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment.AEQbTJ:int
        0x0015: return-void
    }

    public void <init>() {
        0x0000: invoke-direct o.abQe.<init>():void
        0x0003: new-instance v0, o.aaIe
        0x0005: invoke-direct o.aaIe.<init>(com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment):void
        0x0008: invoke-static o.adaR.KWHzl(kotlin.jvm.functions.Function0):o.adaP
        0x000c: iput v0, v4, com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment.EZpvd:o.adaP
        0x000e: new-instance v0, com.okinc.tradingbot.impl.strategy.init.MarketPlaceInitPresenter
        0x0010: invoke-direct com.okinc.tradingbot.impl.strategy.init.MarketPlaceInitPresenter.<init>():void
        0x0013: iput v0, v4, com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment.KWHzl:com.okinc.tradingbot.impl.strategy.init.MarketPlaceInitPresenter
        0x0015: new-instance v0, o.EJT
        0x0017: invoke-direct o.EJT.<init>():void
        0x001a: const-class v1, com.okinc.unify_trade.biz.RecommendBulletMPDataB
        0x001c: new-instance v2, o.YiT
        0x001e: new-instance v3, o.aaIf
        0x0020: invoke-direct o.aaIf.<init>(com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment):void
        0x0023: invoke-direct o.YiT.<init>(kotlin.jvm.functions.Function1):void
        0x0026: invoke-virtual o.adyQ.register(java.lang.Class, o.adyS):void
        0x0029: const-class v1, com.okinc.unify_trade.biz.NmpBotSquareGuideItem
        0x002b: new-instance v2, o.YiW
        0x002d: new-instance v3, o.aaIi
        0x002f: invoke-direct o.aaIi.<init>(com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment):void
        0x0032: invoke-direct o.YiW.<init>(kotlin.jvm.functions.Function0):void
        0x0035: invoke-virtual o.adyQ.register(java.lang.Class, o.adyS):void
        0x0038: iput v0, v4, com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment.copydefault:o.EJT
        0x003a: return-void
    }

    public static synthetic java.lang.String AEQbTJ(com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment p0) {
        0x0000: invoke-static com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment.AhwBna(com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment):java.lang.String
        0x0004: return-object v0
    }

    public static synthetic kotlin.Unit AEQbTJ(o.EJo p0) {
        0x0000: invoke-static com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment.copydefault(o.EJo):kotlin.Unit
        0x0004: return-object v0
    }

    public static final kotlin.Unit AEQbTJ(com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment p0, java.lang.String p1) {
        0x0000: const-string v0, ""
        0x0002: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0005: invoke-virtual androidx.fragment.app.Fragment.getActivity():androidx.fragment.app.FragmentActivity
        0x000b: sget v8, kotlin.Unit.INSTANCE:kotlin.Unit
        0x000d: return-object v8
        0x000e: const-class v0, o.EJm
        0x0010: invoke-static o.EIy.copydefault(java.lang.Class):o.EIN
        0x0015: check-cast v1, o.EJm
        0x0017: sget v0, o.abxe.djBIcL:o.abxe
        0x0019: invoke-direct com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment.KWHzl():java.lang.String
        0x001d: invoke-virtual o.abxe.AEQbTJ(java.lang.String, java.lang.String):java.lang.String
        0x0021: const/4 v4, 0
        0x0022: new-instance v5, o.aaIh
        0x0024: invoke-direct o.aaIh.<init>():void
        0x0027: const/4 v6, 4
        0x0028: const/4 v7, 0
        0x0029: invoke-static/range o.EJm$Activity.processDeeplink$default(o.EJm, android.content.Context, java.lang.String, java.util.Map, kotlin.jvm.functions.Function1, int, java.lang.Object):void
        0x002c: sget v8, kotlin.Unit.INSTANCE:kotlin.Unit
        0x002e: return-object v8
    }

    public static final java.lang.String AhwBna(com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment p0) {
        0x0000: invoke-virtual androidx.fragment.app.Fragment.getArguments():android.os.Bundle
        0x0006: const-string v0, "channel"
        0x0008: invoke-virtual android.os.BaseBundle.getString(java.lang.String):java.lang.String
        0x000d: const/4 v1, 0
        0x0010: const-string v1, ""
        0x0012: return-object v1
    }

    public static synthetic kotlin.Unit EZpvd(com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment p0, java.lang.String p1) {
        0x0000: invoke-static com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment.AEQbTJ(com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment, java.lang.String):kotlin.Unit
        0x0004: return-object v0
    }

    public static final kotlin.Unit EZpvd(com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment p0, java.util.ArrayList p1) {
        0x0000: invoke-static androidx.lifecycle.LifecycleOwnerKt.getLifecycleScope(androidx.lifecycle.LifecycleOwner):androidx.lifecycle.LifecycleCoroutineScope
        0x0004: const/4 v1, 0
        0x0005: const/4 v2, 0
        0x0006: new-instance v3, com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment$setListener$2$1
        0x0008: const/4 v7, 0
        0x0009: invoke-direct com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment$setListener$2$1.<init>(com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment, kotlin.coroutines.Continuation):void
        0x000c: const/4 v4, 3
        0x000d: const/4 v5, 0
        0x000e: invoke-static/range kotlinx.coroutines.BuildersKt.launch$default(kotlinx.coroutines.CoroutineScope, kotlin.coroutines.CoroutineContext, kotlinx.coroutines.CoroutineStart, kotlin.jvm.functions.Function2, int, java.lang.Object):kotlinx.coroutines.Job
        0x0011: sget v6, kotlin.Unit.INSTANCE:kotlin.Unit
        0x0013: return-object v6
    }

    public static final kotlin.Unit EZpvd(com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment p0, kotlin.Pair p1) {
        0x0000: const-string v0, ""
        0x0002: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0005: invoke-static androidx.lifecycle.LifecycleOwnerKt.getLifecycleScope(androidx.lifecycle.LifecycleOwner):androidx.lifecycle.LifecycleCoroutineScope
        0x0009: const/4 v2, 0
        0x000a: const/4 v3, 0
        0x000b: new-instance v4, com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment$setListener$1$1
        0x000d: const/4 v0, 0
        0x000e: invoke-direct com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment$setListener$1$1.<init>(kotlin.Pair, com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment, kotlin.coroutines.Continuation):void
        0x0011: const/4 v5, 3
        0x0012: const/4 v6, 0
        0x0013: invoke-static/range kotlinx.coroutines.BuildersKt.launch$default(kotlinx.coroutines.CoroutineScope, kotlin.coroutines.CoroutineContext, kotlinx.coroutines.CoroutineStart, kotlin.jvm.functions.Function2, int, java.lang.Object):kotlinx.coroutines.Job
        0x0016: sget v7, kotlin.Unit.INSTANCE:kotlin.Unit
        0x0018: return-object v7
    }

    public static final synthetic o.EJT EZpvd(com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment p0) {
        0x0000: iget v0, v0, com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment.copydefault:o.EJT
        0x0002: return-object v0
    }

    public static synthetic void EZpvd(com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment p0, android.view.View p1) {
        0x0000: invoke-static com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment.OLrzqt(com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment, android.view.View):void
        0x0003: return-void
    }

    private final java.lang.String KWHzl() {
        0x0000: iget v0, v1, com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment.EZpvd:o.adaP
        0x0002: invoke-interface o.adaP.getValue():java.lang.Object
        0x0006: check-cast v0, java.lang.String
        0x0008: return-object v0
    }

    public static final kotlin.Unit KWHzl(o.EJo p0) {
        0x0000: const-string v0, ""
        0x0002: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0005: sget v1, kotlin.Unit.INSTANCE:kotlin.Unit
        0x0007: return-object v1
    }

    public static synthetic kotlin.Unit KWHzl(com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment p0) {
        0x0000: invoke-static com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment.valueOf(com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment):kotlin.Unit
        0x0004: return-object v0
    }

    public static final kotlin.Unit KWHzl(com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment p0, java.lang.Exception p1) {
        0x0000: const-string v0, ""
        0x0002: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0005: const/4 v3, 0
        0x0006: const/4 v0, 2
        0x0007: const/4 v1, 0
        0x0008: invoke-static o.Fzc.reportFullyDrawn$default(androidx.fragment.app.Fragment, boolean, java.lang.String, int, java.lang.Object):void
        0x000b: sget v2, kotlin.Unit.INSTANCE:kotlin.Unit
        0x000d: return-object v2
    }

    public static final kotlin.Unit KWHzl(com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment p0, java.lang.String p1) {
        0x0000: const-string v0, ""
        0x0002: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0005: invoke-virtual androidx.fragment.app.Fragment.getViewLifecycleOwner():androidx.lifecycle.LifecycleOwner
        0x0009: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullExpressionValue(java.lang.Object, java.lang.String):void
        0x000c: invoke-static androidx.lifecycle.LifecycleOwnerKt.getLifecycleScope(androidx.lifecycle.LifecycleOwner):androidx.lifecycle.LifecycleCoroutineScope
        0x0010: const/4 v3, 0
        0x0011: const/4 v4, 0
        0x0012: new-instance v5, com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment$initView$1$1$1$1
        0x0014: const/4 v0, 0
        0x0015: invoke-direct com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment$initView$1$1$1$1.<init>(com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment, java.lang.String, kotlin.coroutines.Continuation):void
        0x0018: const/4 v6, 3
        0x0019: const/4 v7, 0
        0x001a: invoke-static/range kotlinx.coroutines.BuildersKt.launch$default(kotlinx.coroutines.CoroutineScope, kotlin.coroutines.CoroutineContext, kotlinx.coroutines.CoroutineStart, kotlin.jvm.functions.Function2, int, java.lang.Object):kotlinx.coroutines.Job
        0x001d: sget v8, kotlin.Unit.INSTANCE:kotlin.Unit
        0x001f: return-object v8
    }

    public static synthetic kotlin.Unit KWHzl(com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment p0, kotlin.Pair p1) {
        0x0000: invoke-static com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment.EZpvd(com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment, kotlin.Pair):kotlin.Unit
        0x0004: return-object v0
    }

    public static final synthetic com.okinc.tradingbot.impl.market_place.home.list.presenter.BotFeaturedPresenter OLrzqt(com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment p0) {
        0x0000: invoke-virtual o.abQe.cP_():com.okinc.unify_trade.refactor.presenter.AbsPresenter
        0x0004: check-cast v0, com.okinc.tradingbot.impl.market_place.home.list.presenter.BotFeaturedPresenter
        0x0006: return-object v0
    }

    public static final synthetic java.util.List OLrzqt(com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment p0, java.util.List p1) {
        0x0000: invoke-virtual com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment.OLrzqt(java.util.List):java.util.List
        0x0004: return-object v0
    }

    public static synthetic kotlin.Unit OLrzqt(o.EJo p0) {
        0x0000: invoke-static com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment.KWHzl(o.EJo):kotlin.Unit
        0x0004: return-object v0
    }

    public static final void OLrzqt(com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment p0, android.view.View p1) {
        0x0000: invoke-virtual com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment.OLrzqt():void
        0x0003: return-void
    }

    public static final kotlin.Unit copydefault(o.EJo p0) {
        0x0000: const-string v0, ""
        0x0002: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0005: sget v1, kotlin.Unit.INSTANCE:kotlin.Unit
        0x0007: return-object v1
    }

    public static synthetic kotlin.Unit copydefault(com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment p0, java.lang.Exception p1) {
        0x0000: invoke-static com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment.KWHzl(com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment, java.lang.Exception):kotlin.Unit
        0x0004: return-object v0
    }

    public static synthetic kotlin.Unit copydefault(com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment p0, java.lang.String p1) {
        0x0000: invoke-static com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment.KWHzl(com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment, java.lang.String):kotlin.Unit
        0x0004: return-object v0
    }

    public static synthetic kotlin.Unit copydefault(com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment p0, java.util.ArrayList p1) {
        0x0000: invoke-static com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment.EZpvd(com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment, java.util.ArrayList):kotlin.Unit
        0x0004: return-object v0
    }

    public static final synthetic com.okinc.tradingbot.impl.strategy.init.MarketPlaceInitPresenter copydefault(com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment p0) {
        0x0000: iget v0, v0, com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment.KWHzl:com.okinc.tradingbot.impl.strategy.init.MarketPlaceInitPresenter
        0x0002: return-object v0
    }

    public static final synthetic o.XEh djBIcL(com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment p0) {
        0x0000: invoke-virtual o.abQe.QHmsKR():androidx.databinding.ViewDataBinding
        0x0004: check-cast v0, o.XEh
        0x0006: return-object v0
    }

    public static final kotlin.Unit valueOf(com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment p0) {
        0x0000: invoke-virtual androidx.fragment.app.Fragment.getActivity():androidx.fragment.app.FragmentActivity
        0x0006: sget v8, kotlin.Unit.INSTANCE:kotlin.Unit
        0x0008: return-object v8
        0x0009: const-class v8, o.EJm
        0x000b: invoke-static o.EIy.copydefault(java.lang.Class):o.EIN
        0x0010: check-cast v0, o.EJm
        0x0012: sget v2, o.abxe.djBIcL:o.abxe
        0x0014: const/4 v3, 0
        0x0015: const/4 v4, 0
        0x0016: const/4 v5, 0
        0x0017: const/4 v6, 7
        0x0018: const/4 v7, 0
        0x0019: invoke-static/range o.abxe.getBotMpPagePath$default(o.abxe, java.lang.String, java.lang.String, boolean, int, java.lang.Object):java.lang.String
        0x001d: new-instance v4, o.aaId
        0x001f: invoke-direct o.aaId.<init>():void
        0x0022: const/4 v5, 4
        0x0023: const/4 v6, 0
        0x0024: invoke-static/range o.EJm$Activity.processDeeplink$default(o.EJm, android.content.Context, java.lang.String, java.util.Map, kotlin.jvm.functions.Function1, int, java.lang.Object):void
        0x0027: sget v8, o.absU.EZpvd:o.absU
        0x0029: invoke-virtual o.absU.OLrzqt():void
        0x002c: sget v8, kotlin.Unit.INSTANCE:kotlin.Unit
        0x002e: return-object v8
    }

    public final void EZpvd() {
        0x0000: invoke-static o.aaIn.EZpvd(androidx.fragment.app.Fragment):boolean
        0x0007: invoke-virtual androidx.fragment.app.Fragment.getChildFragmentManager():androidx.fragment.app.FragmentManager
        0x000b: invoke-virtual androidx.fragment.app.FragmentManager.getFragments():java.util.List
        0x000f: const-string v1, ""
        0x0011: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullExpressionValue(java.lang.Object, java.lang.String):void
        0x0014: check-cast v0, java.lang.Iterable
        0x0016: new-instance v1, java.util.ArrayList
        0x0018: invoke-direct java.util.ArrayList.<init>():void
        0x001b: invoke-interface java.lang.Iterable.iterator():java.util.Iterator
        0x001f: invoke-interface java.util.Iterator.hasNext():boolean
        0x0025: invoke-interface java.util.Iterator.next():java.lang.Object
        0x0029: instance-of v3, o.Yfp
        0x002d: invoke-interface java.util.Collection.add(java.lang.Object):boolean
        0x0031: new-instance v0, java.util.ArrayList
        0x0033: invoke-direct java.util.ArrayList.<init>():void
        0x0036: invoke-interface java.lang.Iterable.iterator():java.util.Iterator
        0x003a: invoke-interface java.util.Iterator.hasNext():boolean
        0x0040: invoke-interface java.util.Iterator.next():java.lang.Object
        0x0045: check-cast v3, o.Yfp
        0x0047: invoke-virtual androidx.fragment.app.Fragment.isAdded():boolean
        0x004d: invoke-interface java.util.Collection.add(java.lang.Object):boolean
        0x0051: invoke-interface java.lang.Iterable.iterator():java.util.Iterator
        0x0055: invoke-interface java.util.Iterator.hasNext():boolean
        0x005b: invoke-interface java.util.Iterator.next():java.lang.Object
        0x005f: check-cast v1, o.Yfp
        0x0061: invoke-virtual com.google.android.material.bottomsheet.BottomSheetDialogFragment.dismissAllowingStateLoss():void
        0x0065: return-void
    }

    public final java.util.List OLrzqt(java.util.List p0) {
        0x0001: check-cast v0, java.lang.Iterable
        0x0003: new-instance v1, java.util.ArrayList
        0x0005: invoke-direct java.util.ArrayList.<init>():void
        0x0008: invoke-interface java.lang.Iterable.iterator():java.util.Iterator
        0x000c: invoke-interface java.util.Iterator.hasNext():boolean
        0x0012: invoke-interface java.util.Iterator.next():java.lang.Object
        0x0016: instance-of v4, com.okinc.unify_trade.biz.BotCreateCardItem
        0x001a: invoke-interface java.util.Collection.add(java.lang.Object):boolean
        0x001e: invoke-interface java.util.List.isEmpty():boolean
        0x0024: return-object v6
        0x0025: new-instance v6, o.YiD
        0x0027: invoke-direct o.YiD.<init>(java.util.List):void
        0x002a: invoke-static o.adbA.KWHzl(java.lang.Object):java.util.List
        0x002e: check-cast v6, java.util.Collection
        0x0030: new-instance v1, java.util.ArrayList
        0x0032: invoke-direct java.util.ArrayList.<init>():void
        0x0035: invoke-interface java.lang.Iterable.iterator():java.util.Iterator
        0x0039: invoke-interface java.util.Iterator.hasNext():boolean
        0x003f: invoke-interface java.util.Iterator.next():java.lang.Object
        0x0043: instance-of v3, com.okinc.unify_trade.biz.BotCreateCardItem
        0x0047: invoke-interface java.util.Collection.add(java.lang.Object):boolean
        0x004b: invoke-static kotlin.collections.CollectionsKt.AEQbTJ(java.util.Collection, java.lang.Iterable):java.util.List
        0x004f: return-object v6
    }

    public final void OLrzqt() {
        0x0000: invoke-static androidx.lifecycle.LifecycleOwnerKt.getLifecycleScope(androidx.lifecycle.LifecycleOwner):androidx.lifecycle.LifecycleCoroutineScope
        0x0004: const/4 v1, 0
        0x0005: const/4 v2, 0
        0x0006: new-instance v3, com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment$loadBotData$1
        0x0008: const/4 v4, 0
        0x0009: invoke-direct com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment$loadBotData$1.<init>(com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment, kotlin.coroutines.Continuation):void
        0x000c: const/4 v4, 3
        0x000d: const/4 v5, 0
        0x000e: invoke-static/range kotlinx.coroutines.BuildersKt.launch$default(kotlinx.coroutines.CoroutineScope, kotlin.coroutines.CoroutineContext, kotlinx.coroutines.CoroutineStart, kotlin.jvm.functions.Function2, int, java.lang.Object):kotlinx.coroutines.Job
        0x0011: return-void
    }

    public final o.aaHX$Application copydefault() {
        0x0000: new-instance v0, o.aaHX$Application
        0x0002: invoke-direct o.aaHX$Application.<init>():void
        0x0005: return-object v0
    }

    public int getLayoutId() {
        0x0000: sget v0, o.WSQ$TaskDescription.fjfviF:int
        0x0002: return v0
    }

    public void initView(android.view.View p0, android.os.Bundle p1) {
        0x0000: const-string v6, ""
        0x0002: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0005: invoke-virtual o.abQe.QHmsKR():androidx.databinding.ViewDataBinding
        0x0009: check-cast v5, o.XEh
        0x000b: iget v6, v5, o.XEh.AEQbTJ:androidx.recyclerview.widget.RecyclerView
        0x000d: new-instance v0, androidx.recyclerview.widget.GridLayoutManager
        0x000f: invoke-virtual android.view.View.getContext():android.content.Context
        0x0013: const/4 v2, 4
        0x0014: invoke-direct androidx.recyclerview.widget.GridLayoutManager.<init>(android.content.Context, int):void
        0x0017: invoke-virtual com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment.copydefault():o.aaHX$Application
        0x001b: invoke-virtual androidx.recyclerview.widget.GridLayoutManager.setSpanSizeLookup(androidx.recyclerview.widget.GridLayoutManager$SpanSizeLookup):void
        0x001e: invoke-virtual androidx.recyclerview.widget.RecyclerView.setLayoutManager(androidx.recyclerview.widget.RecyclerView$LayoutManager):void
        0x0021: iget v0, v4, com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment.copydefault:o.EJT
        0x0023: invoke-virtual androidx.recyclerview.widget.RecyclerView.setAdapter(androidx.recyclerview.widget.RecyclerView$Adapter):void
        0x0026: iget v0, v4, com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment.copydefault:o.EJT
        0x0028: invoke-virtual android.view.View.getPaddingStart():int
        0x002c: invoke-virtual android.view.View.getPaddingEnd():int
        0x0030: new-instance v2, o.aaIc
        0x0032: invoke-direct o.aaIc.<init>(com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment):void
        0x0035: new-instance v3, o.YiE
        0x0037: invoke-direct o.YiE.<init>(int, int, kotlin.jvm.functions.Function1):void
        0x003a: const-class v6, o.YiD
        0x003c: invoke-virtual o.adyQ.register(java.lang.Class, o.adyS):void
        0x003f: iget v5, v5, o.XEh.OLrzqt:o.abeF
        0x0041: new-instance v6, o.aaIg
        0x0043: invoke-direct o.aaIg.<init>(com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment):void
        0x0046: invoke-virtual o.abeF.setRetryClickListener(android.view.View$OnClickListener):void
        0x0049: invoke-virtual com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment.OLrzqt():void
        0x004c: return-void
    }

    public void onConfigurationChanged(android.content.res.Configuration p0) {
        0x0000: const-string v0, ""
        0x0002: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0005: invoke-super androidx.fragment.app.Fragment.onConfigurationChanged(android.content.res.Configuration):void
        0x0008: invoke-virtual com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment.EZpvd():void
        0x000b: return-void
    }

    public void onDestroy() {
        0x0000: invoke-super o.abyl.onDestroy():void
        0x0003: return-void
    }

    public void onResume() {
        0x0000: invoke-static android.os.SystemClock.elapsedRealtime():long
        0x0004: invoke-super o.uyW.onResume():void
        0x0007: invoke-static o.Fzc.copydefault(androidx.fragment.app.Fragment):void
        0x000a: const-string v1, "com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment"
        0x000c: const-string v2, "onResume"
        0x000e: const-string v3, "()V"
        0x0011: invoke-static/range o.Bdb.OLrzqt(java.lang.Object, java.lang.String, java.lang.String, java.lang.String, long):void
        0x0014: return-void
    }

    public void setListener() {
        0x0000: invoke-super o.uyN.setListener():void
        0x0003: iget v0, v5, com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment.KWHzl:com.okinc.tradingbot.impl.strategy.init.MarketPlaceInitPresenter
        0x0005: invoke-virtual com.okinc.tradingbot.impl.strategy.init.MarketPlaceInitPresenter.KWHzl():o.abyf
        0x0009: invoke-virtual androidx.fragment.app.Fragment.getViewLifecycleOwner():androidx.lifecycle.LifecycleOwner
        0x000d: const-string v2, ""
        0x000f: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullExpressionValue(java.lang.Object, java.lang.String):void
        0x0012: new-instance v3, o.aaHZ
        0x0014: invoke-direct o.aaHZ.<init>(com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment):void
        0x0017: invoke-virtual o.abyf.copydefault(androidx.lifecycle.LifecycleOwner, kotlin.jvm.functions.Function1):void
        0x001a: invoke-virtual o.abQe.cP_():com.okinc.unify_trade.refactor.presenter.AbsPresenter
        0x001e: check-cast v0, com.okinc.tradingbot.impl.market_place.home.list.presenter.BotFeaturedPresenter
        0x0020: invoke-virtual com.okinc.tradingbot.impl.market_place.home.list.presenter.BotFeaturedPresenter.OLrzqt():androidx.lifecycle.MutableLiveData
        0x0024: invoke-virtual androidx.fragment.app.Fragment.getViewLifecycleOwner():androidx.lifecycle.LifecycleOwner
        0x0028: new-instance v3, o.aaIn$Activity
        0x002a: new-instance v4, o.aaIb
        0x002c: invoke-direct o.aaIb.<init>(com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment):void
        0x002f: invoke-direct o.aaIn$Activity.<init>(kotlin.jvm.functions.Function1):void
        0x0032: invoke-virtual androidx.lifecycle.LiveData.observe(androidx.lifecycle.LifecycleOwner, androidx.lifecycle.Observer):void
        0x0035: invoke-virtual o.abQe.cP_():com.okinc.unify_trade.refactor.presenter.AbsPresenter
        0x0039: check-cast v0, com.okinc.tradingbot.impl.market_place.home.list.presenter.BotFeaturedPresenter
        0x003b: invoke-virtual com.okinc.tradingbot.impl.market_place.home.list.presenter.BotFeaturedPresenter.copydefault():o.abCj
        0x003f: invoke-virtual o.abQx.DbNXlk():o.abxR
        0x0043: invoke-virtual androidx.fragment.app.Fragment.getViewLifecycleOwner():androidx.lifecycle.LifecycleOwner
        0x0047: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullExpressionValue(java.lang.Object, java.lang.String):void
        0x004a: new-instance v2, o.aaIn$Activity
        0x004c: new-instance v3, o.aaIa
        0x004e: invoke-direct o.aaIa.<init>(com.okinc.tradingbot.impl.strategy.guide.fragment.BotKLineFragment):void
        0x0051: invoke-direct o.aaIn$Activity.<init>(kotlin.jvm.functions.Function1):void
        0x0054: invoke-virtual o.abxR.observe(androidx.lifecycle.LifecycleOwner, androidx.lifecycle.Observer):void
        0x0057: return-void
    }

}

// DEX: classes31.dex | Source: ProGuard/R8
public final class com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter extends com.okinc.unify_trade.refactor.presenter.AbsPresenter {
    // Static Fields
    public static final int KWHzl;

    // Instance Fields
    public final androidx.lifecycle.TradeLiveData AEQbTJ;
    public final androidx.lifecycle.TradeLiveData AYXKKw;
    public java.lang.String AhwBna;
    public final androidx.lifecycle.TradeLiveData AkhnZx;
    public com.okinc.unify_trade.bot.data.TacticsData DbNXlk;
    public final o.abGd EZpvd;
    public final o.abHC OLrzqt;
    public com.okinc.unify_trade.biz.StrategyDetailsResponse copydefault;
    public java.lang.String djBIcL;
    public final o.abHL fetchVPNInfo;
    public java.lang.String gEmmrt;
    public final androidx.lifecycle.TradeLiveData isConnected;
    public java.lang.String valueOf;
    public final o.abHU values;

    static void <clinit>() {
        0x0000: sget v0, o.Bow.KWHzl:o.Bow
        0x0002: sget v1, com.okinc.localization.report.SupportType.CLASS:com.okinc.localization.report.SupportType
        0x0004: const/4 v2, 0
        0x0005: const-string v3, "com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter"
        0x0007: invoke-virtual o.Bow.copydefault(java.lang.String, com.okinc.localization.report.SupportType, java.lang.String):void
        0x000a: const/16 v0, 8
        0x000c: sput v0, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.KWHzl:int
        0x000e: return-void
    }

    public void <init>(android.app.Application p0, androidx.lifecycle.SavedStateHandle p1) {
        0x0002: const-string v1, ""
        0x0006: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x000b: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x000e: invoke-direct/range com.okinc.unify_trade.refactor.presenter.AbsPresenter.<init>(android.app.Application, androidx.lifecycle.SavedStateHandle):void
        0x0011: iput v1, v0, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.valueOf:java.lang.String
        0x0013: iput v1, v0, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.AhwBna:java.lang.String
        0x0015: iput v1, v0, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.djBIcL:java.lang.String
        0x0017: new-instance v15, com.okinc.unify_trade.biz.StrategyDetailsResponse
        0x001a: const/4 v3, 0
        0x001b: const/4 v4, 0
        0x001c: const/4 v5, 0
        0x001d: const/4 v6, 0
        0x001e: const/4 v7, 0
        0x001f: const/4 v8, 0
        0x0020: const/4 v9, 0
        0x0021: const/4 v10, 0
        0x0022: const/4 v11, 0
        0x0023: const/4 v12, 0
        0x0024: const/4 v13, 0
        0x0025: const/4 v14, 0
        0x0026: const/16 v16, 0
        0x002c: const/16 v17, 0
        0x002e: const/16 v18, 0
        0x0030: const/16 v19, 0
        0x0032: const/16 v20, 0
        0x0034: const/16 v21, 0
        0x0036: const/16 v22, 0
        0x0038: const/16 v23, 0
        0x003a: const/16 v24, 0
        0x003c: const/16 v25, 0
        0x003e: const/16 v26, 0
        0x0040: const/16 v27, 0
        0x0042: const/16 v28, 0
        0x0044: const/16 v29, 0
        0x0046: const/16 v30, 0
        0x0048: const/16 v31, 0
        0x004a: const/16 v32, 0
        0x004c: const/16 v33, 0
        0x004e: const/16 v34, 0
        0x0050: const/16 v35, 0
        0x0052: const/16 v36, 0
        0x0054: const/16 v37, 0
        0x0056: const/16 v38, 0
        0x0058: const/16 v39, 0
        0x005a: const/16 v40, 0
        0x005c: const/16 v41, 0
        0x005e: const/16 v42, 0
        0x0060: const/16 v43, 0
        0x0062: const/16 v44, 0
        0x0064: const/16 v45, 0
        0x0066: const/16 v46, 0
        0x0068: const/16 v47, 0
        0x006a: const/16 v48, 0
        0x006c: const/16 v49, 0
        0x006e: const/16 v50, 0
        0x0070: const/16 v51, 0
        0x0072: const/16 v52, 0
        0x0074: const/16 v53, 0
        0x0076: const/16 v54, 0
        0x0078: const/16 v55, 0
        0x007a: const/16 v56, 0
        0x007c: const/16 v57, 0
        0x007e: const/16 v58, 0
        0x0080: const/16 v59, 0
        0x0082: const/16 v60, 0
        0x0084: const/16 v61, 0
        0x0086: const/16 v62, 0
        0x0088: const/16 v63, 0
        0x008a: const/16 v64, 0
        0x008c: const/16 v65, 0
        0x008e: const/16 v66, 0
        0x0090: const/16 v67, 0
        0x0092: const/16 v68, 0
        0x0094: const/16 v69, 0
        0x0096: const/16 v70, 0
        0x0098: const/16 v71, 0
        0x009a: const/16 v72, 0
        0x009c: const/16 v73, 0
        0x009e: const/16 v74, 0
        0x00a0: const/16 v75, 0
        0x00a2: const/16 v76, 0
        0x00a4: const/16 v77, 0
        0x00a6: const/16 v78, 0
        0x00a8: const/16 v79, 0
        0x00aa: const/16 v80, 0
        0x00ac: const/16 v81, 0
        0x00ae: const/16 v82, 0
        0x00b0: const/16 v83, 0
        0x00b2: const/16 v84, 0
        0x00b4: const/16 v85, 0
        0x00b6: const/16 v86, 0
        0x00b8: const/16 v87, 0
        0x00ba: const/16 v88, 0
        0x00bc: const/16 v89, 0
        0x00be: const/16 v90, 0
        0x00c0: const/16 v91, 0
        0x00c2: const/16 v92, 0
        0x00c4: const/16 v93, 0
        0x00c6: const/16 v94, 0
        0x00c8: const/16 v95, 0
        0x00ca: const/16 v96, 0
        0x00cc: const/16 v97, 0
        0x00ce: const/16 v98, 0
        0x00d0: const/16 v99, 0
        0x00d2: const/16 v100, 0
        0x00d4: const/16 v101, 0
        0x00d6: const/16 v102, 0
        0x00d8: const/16 v103, 0
        0x00da: const/16 v104, 0
        0x00dc: const/16 v105, 0
        0x00de: const/16 v106, 0
        0x00e0: const/16 v107, 0
        0x00e2: const/16 v108, 0
        0x00e4: const/16 v109, 0
        0x00e6: const/16 v110, 0
        0x00e8: const/16 v111, 0
        0x00ea: const/16 v112, 0
        0x00ec: const/16 v113, 0
        0x00ee: const/16 v114, 0
        0x00f0: const/16 v115, 0
        0x00f2: const/16 v116, 0
        0x00f4: const/16 v117, 0
        0x00f6: const/16 v118, 0
        0x00f8: const/16 v119, 0
        0x00fa: const/16 v120, 0
        0x00fc: const/16 v121, 0
        0x00fe: const/16 v122, 0
        0x0100: const/16 v123, 0
        0x0102: const/16 v124, 0
        0x0104: const/16 v125, 0
        0x0106: const/16 v126, 0
        0x0108: const/16 v127, 0
        0x010a: const/16 v128, 0
        0x010c: const/16 v129, 0
        0x010e: const/16 v130, 0
        0x0110: const/16 v131, 0
        0x0112: const/16 v132, 0
        0x0114: const/16 v133, 0
        0x0116: const/16 v134, 0
        0x0118: const/16 v135, 0
        0x011a: const/16 v136, 0
        0x011c: const/16 v137, 0
        0x011e: const/16 v138, 0
        0x0120: const/16 v139, 0
        0x0122: const/16 v140, 0
        0x0124: const/16 v141, 0
        0x0126: const/16 v142, 0
        0x0128: const/16 v143, 0
        0x012a: const/16 v144, 0
        0x012c: const/16 v145, 0
        0x012e: const/16 v146, 0
        0x0130: const/16 v147, 0
        0x0132: const/16 v148, 0
        0x0134: const/16 v149, 0
        0x0136: const/16 v150, 0
        0x0138: const/16 v151, 0
        0x013a: const/16 v152, 0
        0x013c: const/16 v153, 0
        0x013e: const/16 v154, 0
        0x0140: const/16 v155, 0
        0x0142: const/16 v156, 0
        0x0144: const/16 v157, 0
        0x0146: const/16 v158, 0
        0x0148: const/16 v159, 0
        0x014a: const/16 v160, 0
        0x014c: const/16 v161, 0
        0x014e: const/16 v162, 0
        0x0150: const/16 v163, 0
        0x0152: const/16 v164, 0
        0x0154: const/16 v165, 0
        0x0156: const/16 v166, 0
        0x0158: const/16 v167, 0
        0x015a: const/16 v168, 0
        0x015c: const/16 v169, 0
        0x015e: const/16 v170, 0
        0x0160: const/16 v171, 0
        0x0162: const/16 v172, 0
        0x0164: const/16 v173, 0
        0x0166: const/16 v174, 0
        0x0168: const/16 v175, 0
        0x016a: const/16 v176, 0
        0x016c: const/16 v177, 0
        0x016e: const/16 v178, 0
        0x0170: const/16 v179, 0
        0x0172: const/16 v180, 0
        0x0174: const/16 v181, 0
        0x0176: const/16 v182, 0
        0x0178: const/16 v183, 0
        0x017a: const/16 v184, 0
        0x017c: const/16 v185, 0
        0x017e: const/16 v186, 0
        0x0180: const/16 v187, 0
        0x0182: const/16 v188, 0
        0x0184: const/16 v189, 0
        0x0186: const/16 v190, 0
        0x0188: const/16 v191, 0
        0x018a: const/16 v192, 0
        0x018c: const/16 v193, 0
        0x018e: const/16 v194, 0
        0x0190: const/16 v195, 0
        0x0192: const/16 v196, 0
        0x0194: const/16 v197, 0
        0x0196: const/16 v198, 0
        0x0198: const/16 v199, 0
        0x019a: const/16 v200, 0
        0x019c: const/16 v201, 0
        0x019e: const/16 v202, 0
        0x01a0: const/16 v203, 0
        0x01a2: const/16 v204, 0
        0x01a4: const/16 v205, 0
        0x01a6: const/16 v206, 0
        0x01a8: const/16 v207, 0
        0x01aa: const/16 v208, 0
        0x01ac: const/16 v209, 0
        0x01ae: const/16 v210, 0
        0x01b0: const/16 v211, 0
        0x01b2: const/16 v212, 0
        0x01b4: const/16 v213, 0
        0x01b6: const/16 v214, 0
        0x01b8: const/16 v215, 0
        0x01ba: const/16 v216, 0
        0x01bc: const/16 v217, 0
        0x01be: const/16 v218, 0
        0x01c0: const/16 v219, 0
        0x01c2: const/16 v220, 0
        0x01c4: const/16 v221, 0
        0x01c6: const/16 v222, 0
        0x01c8: const/16 v223, 0
        0x01ca: const/16 v224, 0
        0x01cc: const/16 v225, 0
        0x01ce: const/16 v226, 0
        0x01d0: const/16 v227, 0
        0x01d2: const/16 v228, 0
        0x01d4: const/16 v229, 0
        0x01d6: const/16 v230, 0
        0x01d8: const/16 v231, 0
        0x01da: const/16 v232, 0
        0x01dc: const/16 v233, 0
        0x01de: const/16 v234, 0
        0x01e0: const/16 v235, 0
        0x01e2: const/16 v236, 0
        0x01e4: const/16 v237, 0
        0x01e6: const/16 v238, 0
        0x01e8: const/16 v239, 0
        0x01ea: const/16 v240, 0
        0x01ec: const/16 v241, 0
        0x01ee: const/16 v242, 0
        0x01f0: const/16 v243, 0
        0x01f2: const/16 v244, 0
        0x01f4: const/16 v245, -1
        0x01f6: const/16 v246, -1
        0x01f8: const/16 v247, -1
        0x01fa: const/16 v248, -1
        0x01fc: const/16 v249, -1
        0x01fe: const/16 v250, -1
        0x0200: const/16 v251, -1
        0x0202: const v252, 262143
        0x0205: const/16 v253, 0
        0x0207: invoke-direct/range com.okinc.unify_trade.biz.StrategyDetailsResponse.<init>(java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, com.okinc.unify_trade.biz.bot.TpSlTriggerParamReqResp, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.util.ArrayList, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, boolean, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.util.ArrayList, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.util.ArrayList, java.util.ArrayList, boolean, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.util.List, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, com.okinc.unify_trade.biz.TrailingConfig, com.okinc.unify_trade.biz.TrailingConfig, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.util.List, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.Boolean, java.lang.Integer, java.lang.String, java.lang.String, com.okinc.unify_trade.biz.EntrySignalOrderAmtParam, com.okinc.unify_trade.biz.ExitSignalOrderAmtParam, java.lang.String, java.lang.String, boolean, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, boolean, boolean, java.lang.String, com.okinc.unify_trade.biz.ExtendedBusinessInfo, java.lang.String, java.lang.String, java.util.ArrayList, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.Boolean, java.util.List, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.util.List, java.lang.String, java.lang.String, java.lang.Long, java.util.ArrayList, java.util.ArrayList, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, com.okinc.unify_trade.biz.bot.SmartEarnParam, com.okinc.unify_trade.biz.ArbitrageTotalPnlInfo, java.lang.Float, java.lang.String, boolean, java.util.List, java.lang.String, java.lang.String, com.okinc.unify_trade.biz.GridProfitAutoReinvestResponse, java.util.List, com.okinc.unify_trade.biz.StrategyProfitResponse, java.lang.String, java.lang.String, java.lang.String, com.okinc.unify_trade.biz.ProfitDetailResponse, com.okinc.unify_trade.biz.ProfitDetailResponse, com.okinc.unify_trade.biz.bot.BalanceDetailInfoDto, java.lang.String, java.lang.String, com.okinc.unify_trade.biz.BotVo, com.okinc.unify_trade.biz.subscribe.StrategyDcaEditResp, java.lang.String, java.lang.String, java.lang.String, int, int, int, int, int, int, int, int, kotlin.jvm.internal.DefaultConstructorMarker):void
        0x020c: iput v2, v0, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.copydefault:com.okinc.unify_trade.biz.StrategyDetailsResponse
        0x020e: new-instance v2, o.abHL
        0x0210: const/4 v3, 1
        0x0211: const/4 v5, 0
        0x0212: invoke-direct o.abHL.<init>(boolean, int, kotlin.jvm.internal.DefaultConstructorMarker):void
        0x0215: iput v2, v0, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.fetchVPNInfo:o.abHL
        0x0217: new-instance v2, androidx.lifecycle.TradeLiveData
        0x0219: invoke-direct androidx.lifecycle.TradeLiveData.<init>():void
        0x021c: iput v2, v0, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.AkhnZx:androidx.lifecycle.TradeLiveData
        0x021e: new-instance v2, o.abHU
        0x0220: invoke-direct o.abHU.<init>():void
        0x0223: iput v2, v0, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.values:o.abHU
        0x0225: new-instance v2, androidx.lifecycle.TradeLiveData
        0x0227: invoke-direct androidx.lifecycle.TradeLiveData.<init>():void
        0x022a: iput v2, v0, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.isConnected:androidx.lifecycle.TradeLiveData
        0x022c: new-instance v2, o.abHC
        0x022e: invoke-direct o.abHC.<init>():void
        0x0231: iput v2, v0, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.OLrzqt:o.abHC
        0x0233: new-instance v2, androidx.lifecycle.TradeLiveData
        0x0235: invoke-direct androidx.lifecycle.TradeLiveData.<init>():void
        0x0238: iput v2, v0, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.AYXKKw:androidx.lifecycle.TradeLiveData
        0x023a: new-instance v2, o.abGd
        0x023c: invoke-direct o.abGd.<init>():void
        0x023f: iput v2, v0, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.EZpvd:o.abGd
        0x0241: new-instance v2, androidx.lifecycle.TradeLiveData
        0x0243: invoke-direct androidx.lifecycle.TradeLiveData.<init>():void
        0x0246: iput v2, v0, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.AEQbTJ:androidx.lifecycle.TradeLiveData
        0x0248: iput v1, v0, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.gEmmrt:java.lang.String
        0x024a: return-void
    }

    private final java.lang.String AkhnZx() {
        0x0000: iget v0, v1, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.DbNXlk:com.okinc.unify_trade.bot.data.TacticsData
        0x0004: invoke-virtual com.okinc.unify_trade.bot.data.TacticsData.getOrderType():java.lang.String
        0x0008: return-object v0
        0x0009: const-string v0, ""
        0x000b: return-object v0
    }

    public final java.lang.String AEQbTJ() {
        0x0000: iget v0, v3, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.DbNXlk:com.okinc.unify_trade.bot.data.TacticsData
        0x0002: const-string v1, ""
        0x0006: invoke-virtual com.okinc.unify_trade.bot.data.TacticsData.getOrderType():java.lang.String
        0x000a: invoke-virtual java.lang.String.hashCode():int
        0x0012: const-string v2, "grid"
        0x0014: invoke-virtual java.lang.String.equals(java.lang.Object):boolean
        0x001b: const-string v0, "spot_grid"
        0x001d: return-object v0
        0x001e: const-string v2, "contract_grid"
        0x0020: invoke-virtual java.lang.String.equals(java.lang.Object):boolean
        0x0027: const-string v0, "futures_grid"
        0x0029: return-object v0
        0x002a: const-string v2, "contract_dca"
        0x002c: invoke-virtual java.lang.String.equals(java.lang.Object):boolean
        0x0033: const-string v0, "futures_dca"
        0x0035: return-object v0
        0x0036: const-string v2, "spot_dca"
        0x0038: invoke-virtual java.lang.String.equals(java.lang.Object):boolean
        0x003f: return-object v2
        0x0040: return-object v1
    }

    public final void AEQbTJ(com.okinc.unify_trade.biz.StrategyDetailsResponse p0) {
        0x0000: const-string v0, ""
        0x0002: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0005: iput v2, v1, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.copydefault:com.okinc.unify_trade.biz.StrategyDetailsResponse
        0x0007: return-void
    }

    public final void AEQbTJ(java.lang.String p0) {
        0x0004: const-string v2, ""
        0x0006: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0009: iget v2, v0, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.DbNXlk:com.okinc.unify_trade.bot.data.TacticsData
        0x000d: iget v3, v0, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.OLrzqt:o.abHC
        0x000f: iget v4, v0, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.AYXKKw:androidx.lifecycle.TradeLiveData
        0x0011: new-instance v5, o.abur
        0x0013: invoke-direct o.abur.<init>():void
        0x0016: iget v6, v0, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.AhwBna:java.lang.String
        0x0018: iget v7, v0, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.valueOf:java.lang.String
        0x001a: invoke-static o.abxs.copydefault(java.lang.String, java.lang.String, java.lang.String):java.lang.String
        0x001e: invoke-virtual o.abur.OLrzqt(java.lang.String):void
        0x0021: iget v1, v0, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.valueOf:java.lang.String
        0x0023: invoke-virtual o.abur.KWHzl(java.lang.String):void
        0x0026: iget v1, v0, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.AhwBna:java.lang.String
        0x0028: invoke-virtual o.abur.copydefault(java.lang.String):void
        0x002b: sget v1, kotlin.Unit.INSTANCE:kotlin.Unit
        0x002d: invoke-virtual o.abQx.AEQbTJ(androidx.lifecycle.MutableLiveData, o.abQf):void
        0x0030: const/4 v1, 1
        0x0031: invoke-virtual o.abHC.EZpvd(boolean):o.abHC
        0x0034: invoke-virtual com.okinc.unify_trade.bot.data.TacticsData.getAlgoId():java.lang.String
        0x0038: invoke-direct/range com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.AkhnZx():java.lang.String
        0x003c: new-instance v2, com.okinc.unify_trade.biz.StrategyOrderBillDetailsReq
        0x003e: const-string v6, "0"
        0x0040: const/4 v7, 0
        0x0041: const/4 v8, 0
        0x0042: const/4 v9, 0
        0x0043: const/4 v10, 0
        0x0044: const/4 v11, 0
        0x0045: const/4 v12, 0
        0x0046: const/4 v14, 0
        0x0047: const/16 v15, 764
        0x0049: const/16 v16, 0
        0x004c: invoke-direct/range com.okinc.unify_trade.biz.StrategyOrderBillDetailsReq.<init>(java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, java.lang.String, int, kotlin.jvm.internal.DefaultConstructorMarker):void
        0x004f: invoke-virtual o.abHC.EZpvd(com.okinc.unify_trade.biz.StrategyOrderBillDetailsReq):o.abHC
        0x0052: const-wide/16 v4, 0
        0x0054: const/4 v2, 0
        0x0055: invoke-static o.abQy$TaskDescription.execute$default(o.abQy, long, int, java.lang.Object):void
        0x0058: return-void
    }

    public final androidx.lifecycle.TradeLiveData AYXKKw() {
        0x0000: iget v0, v1, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.AkhnZx:androidx.lifecycle.TradeLiveData
        0x0002: return-object v0
    }

    public final java.lang.String AhwBna() {
        0x0000: iget v0, v1, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.djBIcL:java.lang.String
        0x0002: return-object v0
    }

    public final androidx.lifecycle.TradeLiveData EZpvd() {
        0x0000: iget v0, v1, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.AEQbTJ:androidx.lifecycle.TradeLiveData
        0x0002: return-object v0
    }

    public final void EZpvd(com.okinc.unify_trade.bot.data.TacticsData p0) {
        0x0000: iput v1, v0, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.DbNXlk:com.okinc.unify_trade.bot.data.TacticsData
        0x0002: return-void
    }

    public final void EZpvd(java.lang.String p0) {
        0x0000: const-string v0, ""
        0x0002: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0005: iput v2, v1, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.djBIcL:java.lang.String
        0x0007: return-void
    }

    public final com.okinc.unify_trade.bot.data.BenefitDetails KWHzl(o.abrD p0) {
        0x0000: const-string v0, ""
        0x0002: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0005: invoke-virtual o.abrD.AEQbTJ():java.lang.Object
        0x0009: check-cast v0, com.okinc.unify_trade.biz.StrategyDetailsResponse
        0x000b: invoke-virtual com.okinc.unify_trade.biz.StrategyDetailsResponse.getAlgoOrdType():java.lang.String
        0x000f: invoke-virtual java.lang.String.hashCode():int
        0x0018: const-string v1, "dcd_bot"
        0x001a: invoke-virtual java.lang.String.equals(java.lang.Object):boolean
        0x0022: new-instance v0, o.abuz
        0x0024: invoke-direct o.abuz.<init>():void
        0x0029: const-string v1, "recurring"
        0x002b: invoke-virtual java.lang.String.equals(java.lang.Object):boolean
        0x0033: new-instance v0, o.abuO
        0x0035: invoke-direct o.abuO.<init>():void
        0x003a: const-string v1, "grid"
        0x003c: invoke-virtual java.lang.String.equals(java.lang.Object):boolean
        0x0044: const-string v1, "contract_grid"
        0x0046: invoke-virtual java.lang.String.equals(java.lang.Object):boolean
        0x004e: new-instance v0, o.abuw
        0x0050: const/4 v1, 1
        0x0051: const/4 v2, 0
        0x0052: const/4 v3, 0
        0x0053: invoke-direct o.abuw.<init>(boolean, int, kotlin.jvm.internal.DefaultConstructorMarker):void
        0x0058: const-string v1, "smart_arbitrage"
        0x005a: invoke-virtual java.lang.String.equals(java.lang.Object):boolean
        0x0061: new-instance v0, o.abuX
        0x0063: invoke-direct o.abuX.<init>():void
        0x0067: const-string v1, "signal_bot"
        0x0069: invoke-virtual java.lang.String.equals(java.lang.Object):boolean
        0x0070: new-instance v0, o.abuR
        0x0072: invoke-direct o.abuR.<init>():void
        0x0076: const-string v1, "smart_portfolio"
        0x0078: invoke-virtual java.lang.String.equals(java.lang.Object):boolean
        0x007f: new-instance v0, o.abvc
        0x0081: invoke-direct o.abvc.<init>():void
        0x0085: const-string v1, "contract_dca"
        0x0087: invoke-virtual java.lang.String.equals(java.lang.Object):boolean
        0x008e: new-instance v0, o.abut
        0x0090: invoke-direct o.abut.<init>():void
        0x0094: const-string v1, "infinite_grid"
        0x0096: invoke-virtual java.lang.String.equals(java.lang.Object):boolean
        0x009d: new-instance v0, o.abvj
        0x009f: invoke-direct o.abvj.<init>():void
        0x00a3: const-string v1, "spot_dca"
        0x00a5: invoke-virtual java.lang.String.equals(java.lang.Object):boolean
        0x00ac: new-instance v0, o.abvb
        0x00ae: invoke-direct o.abvb.<init>():void
        0x00b1: iget v1, v4, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.copydefault:com.okinc.unify_trade.biz.StrategyDetailsResponse
        0x00b3: invoke-virtual com.okinc.unify_trade.biz.StrategyDetailsResponse.isHistory():boolean
        0x00b7: invoke-virtual o.abvb.AEQbTJ(boolean):void
        0x00bb: new-instance v0, o.abuF
        0x00bd: invoke-direct o.abuF.<init>():void
        0x00c0: invoke-interface o.abQf.EZpvd(java.lang.Object):java.lang.Object
        0x00c4: check-cast v5, com.okinc.unify_trade.bot.data.BenefitDetails
        0x00c6: return-object v5
    }

    public final java.lang.String KWHzl() {
        0x0000: iget v0, v1, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.valueOf:java.lang.String
        0x0002: return-object v0
    }

    public final void KWHzl(java.lang.String p0) {
        0x0000: const-string v0, ""
        0x0002: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0005: iput v2, v1, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.AhwBna:java.lang.String
        0x0007: return-void
    }

    public final java.lang.String OLrzqt() {
        0x0000: iget v0, v1, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.AhwBna:java.lang.String
        0x0002: return-object v0
    }

    public final void OLrzqt(java.lang.String p0) {
        0x0000: const-string v0, ""
        0x0002: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0005: iput v2, v1, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.valueOf:java.lang.String
        0x0007: return-void
    }

    public final androidx.lifecycle.TradeLiveData copydefault() {
        0x0000: iget v0, v1, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.AYXKKw:androidx.lifecycle.TradeLiveData
        0x0002: return-object v0
    }

    public final kotlin.Pair copydefault(java.lang.String p0) {
        0x0000: const-string v0, ""
        0x0002: invoke-static kotlin.jvm.internal.Intrinsics.checkNotNullParameter(java.lang.Object, java.lang.String):void
        0x0005: iget v0, v4, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.isConnected:androidx.lifecycle.TradeLiveData
        0x0007: invoke-virtual androidx.lifecycle.LiveData.getValue():java.lang.Object
        0x000b: check-cast v0, kotlin.Pair
        0x000d: const/4 v1, 0
        0x0010: invoke-virtual kotlin.Pair.getFirst():java.lang.Object
        0x0014: check-cast v0, java.lang.String
        0x0018: invoke-static o.uAv.copydefault(java.lang.Object, java.lang.Object):boolean
        0x001c: const/4 v2, 1
        0x001d: const/4 v3, 0
        0x0020: sget v0, o.abxe.djBIcL:o.abxe
        0x0022: invoke-static o.abxe.getRiseUpColor$default(o.abxe, float, int, java.lang.Object):int
        0x0026: invoke-static java.lang.Integer.valueOf(int):java.lang.Integer
        0x002a: invoke-static o.adaT.copydefault(java.lang.Object, java.lang.Object):kotlin.Pair
        0x002e: return-object v5
        0x002f: sget v0, o.abxe.djBIcL:o.abxe
        0x0031: invoke-static o.abxe.getFallDownColor$default(o.abxe, float, int, java.lang.Object):int
        0x0035: invoke-static java.lang.Integer.valueOf(int):java.lang.Integer
        0x0039: invoke-static o.adaT.copydefault(java.lang.Object, java.lang.Object):kotlin.Pair
        0x003d: return-object v5
    }

    public final androidx.lifecycle.TradeLiveData djBIcL() {
        0x0000: iget v0, v1, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.isConnected:androidx.lifecycle.TradeLiveData
        0x0002: return-object v0
    }

    public final void fetchVPNInfo() {
        0x0000: iget v0, v5, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.DbNXlk:com.okinc.unify_trade.bot.data.TacticsData
        0x0004: iget v1, v5, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.EZpvd:o.abGd
        0x0006: iget v2, v5, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.AEQbTJ:androidx.lifecycle.TradeLiveData
        0x0008: invoke-virtual o.abQx.copydefault(androidx.lifecycle.MutableLiveData):void
        0x000b: invoke-virtual com.okinc.unify_trade.bot.data.TacticsData.getInstId():java.lang.String
        0x000f: invoke-virtual o.abGd.OLrzqt(java.lang.String):o.abGd
        0x0012: const/4 v0, 1
        0x0013: const/4 v2, 0
        0x0014: const-wide/16 v3, 0
        0x0016: invoke-static o.abQy$TaskDescription.execute$default(o.abQy, long, int, java.lang.Object):void
        0x0019: return-void
    }

    public final com.okinc.unify_trade.bot.data.TacticsData gEmmrt() {
        0x0000: iget v0, v1, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.DbNXlk:com.okinc.unify_trade.bot.data.TacticsData
        0x0002: return-object v0
    }

    public java.util.List getUseCaseList() {
        0x0000: iget v0, v6, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.values:o.abHU
        0x0002: iget v1, v6, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.fetchVPNInfo:o.abHL
        0x0004: iget v2, v6, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.OLrzqt:o.abHC
        0x0006: iget v3, v6, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.EZpvd:o.abGd
        0x0008: const/4 v4, 4
        0x0009: new-array v4, o.abQx[]
        0x000b: const/4 v5, 0
        0x000e: const/4 v0, 1
        0x0011: const/4 v0, 2
        0x0014: const/4 v0, 3
        0x0017: invoke-static o.adbF.copydefault(java.lang.Object[]):java.util.List
        0x001b: return-object v0
    }

    public final void isConnected() {
        0x0000: iget v0, v5, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.DbNXlk:com.okinc.unify_trade.bot.data.TacticsData
        0x0004: iget v1, v5, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.values:o.abHU
        0x0006: iget v2, v5, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.isConnected:androidx.lifecycle.TradeLiveData
        0x0008: new-instance v3, o.abvt
        0x000a: invoke-direct o.abvt.<init>():void
        0x000d: invoke-virtual o.abQx.AEQbTJ(androidx.lifecycle.MutableLiveData, o.abQf):void
        0x0010: invoke-virtual com.okinc.unify_trade.bot.data.TacticsData.getInstId():java.lang.String
        0x0014: invoke-virtual o.abHU.AEQbTJ(java.lang.String):o.abHU
        0x0017: const/4 v0, 1
        0x0018: const/4 v2, 0
        0x0019: const-wide/16 v3, 0
        0x001b: invoke-static o.abQy$TaskDescription.execute$default(o.abQy, long, int, java.lang.Object):void
        0x001e: return-void
    }

    public final void valueOf() {
        0x0000: iget v0, v5, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.DbNXlk:com.okinc.unify_trade.bot.data.TacticsData
        0x0004: iget v1, v5, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.fetchVPNInfo:o.abHL
        0x0006: iget v2, v5, com.okinc.unify_trade.bot.presenter.BotOrderKlinePresenter.AkhnZx:androidx.lifecycle.TradeLiveData
        0x0008: invoke-virtual o.abQx.copydefault(androidx.lifecycle.MutableLiveData):void
        0x000b: invoke-virtual com.okinc.unify_trade.bot.data.TacticsData.getAlgoId():java.lang.String
        0x000f: invoke-virtual o.abHL.copydefault(java.lang.String):void
        0x0012: const/4 v0, 1
        0x0013: const/4 v2, 0
        0x0014: const-wide/16 v3, 0
        0x0016: invoke-static o.abQy$TaskDescription.execute$default(o.abQy, long, int, java.lang.Object):void
        0x0019: return-void
    }

}
