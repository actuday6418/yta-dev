use crate::*;
use deps::*;

pub struct Backtest;

#[derive(Debug, Clone)]
pub struct Cocas<'a> {
    pub combo: &'a Combo,
    pub instrument: Instrument,
    pub candles: Vec<&'a Candle>,
}

#[serde(crate = "deps")]
#[derive(CandleDerive, Debug, Clone, Copy, Identity, Serialize, Deserialize, PartialEq)]
pub struct Candle {
    pub id: Option<i64>,
    pub time: NaiveDateTime,
    #[serde(default)]
    pub bid_open_num: f64,
    #[serde(default)]
    pub bid_close_num: f64,
    #[serde(default)]
    pub bid_high_num: f64,
    #[serde(default)]
    pub bid_low_num: f64,
    #[serde(default)]
    pub ask_open_num: f64,
    #[serde(default)]
    pub ask_close_num: f64,
    #[serde(default)]
    pub ask_high_num: f64,
    #[serde(default)]
    pub ask_low_num: f64,
    #[serde(default)]
    pub mid_open_num: f64,
    #[serde(default)]
    pub mid_close_num: f64,
    #[serde(default)]
    pub mid_high_num: f64,
    #[serde(default)]
    pub mid_low_num: f64,

    pub volume: u64,
    #[serde(default)]
    pub acos_out_real: f64,
    #[serde(default)]
    pub ad_out_real: f64,
    #[serde(default)]
    pub add_out_real: f64,
    #[serde(default)]
    pub adosc_out_real: f64,
    #[serde(default)]
    pub adx_out_real: f64,
    #[serde(default)]
    pub adxr_out_real: f64,
    #[serde(default)]
    pub apo_out_real: f64,
    #[serde(default)]
    pub aroon_out_aroon_down: f64,
    #[serde(default)]
    pub aroon_out_aroon_up: f64,
    #[serde(default)]
    pub aroonosc_out_real: f64,
    #[serde(default)]
    pub asin_out_real: f64,
    #[serde(default)]
    pub atan_out_real: f64,
    #[serde(default)]
    pub atr_out_real: f64,
    #[serde(default)]
    pub avgprice_out_real: f64,
    #[serde(default)]
    pub bbands_out_real_upper_band: f64,
    #[serde(default)]
    pub bbands_out_real_middle_band: f64,
    #[serde(default)]
    pub bbands_out_real_lower_band: f64,
    #[serde(default)]
    pub beta_out_real: f64,
    #[serde(default)]
    pub bop_out_real: f64,
    #[serde(default)]
    pub cci_out_real: f64,
    #[serde(default)]
    pub cdl2crows_out_integer: f64,
    #[serde(default)]
    pub cdl3blackcrows_out_integer: f64,
    #[serde(default)]
    pub cdl3inside_out_integer: f64,
    #[serde(default)]
    pub cdl3linestrike_out_integer: f64,
    #[serde(default)]
    pub cdl3outside_out_integer: f64,
    #[serde(default)]
    pub cdl3starsinsouth_out_integer: f64,
    #[serde(default)]
    pub cdl3whitesoldiers_out_integer: f64,
    #[serde(default)]
    pub cdlabandonedbaby_out_integer: f64,
    #[serde(default)]
    pub cdladvanceblock_out_integer: f64,
    #[serde(default)]
    pub cdlbelthold_out_integer: f64,
    #[serde(default)]
    pub cdlbreakaway_out_integer: f64,
    #[serde(default)]
    pub cdlclosingmarubozu_out_integer: f64,
    #[serde(default)]
    pub cdlconcealbabyswall_out_integer: f64,
    #[serde(default)]
    pub cdlcounterattack_out_integer: f64,
    #[serde(default)]
    pub cdldarkcloudcover_out_integer: f64,
    #[serde(default)]
    pub cdldoji_out_integer: f64,
    #[serde(default)]
    pub cdldojistar_out_integer: f64,
    #[serde(default)]
    pub cdldragonflydoji_out_integer: f64,
    #[serde(default)]
    pub cdlengulfing_out_integer: f64,
    #[serde(default)]
    pub cdleveningdojistar_out_integer: f64,
    #[serde(default)]
    pub cdleveningstar_out_integer: f64,
    #[serde(default)]
    pub cdlgapsidesidewhite_out_integer: f64,
    #[serde(default)]
    pub cdlgravestonedoji_out_integer: f64,
    #[serde(default)]
    pub cdlhammer_out_integer: f64,
    #[serde(default)]
    pub cdlhangingman_out_integer: f64,
    #[serde(default)]
    pub cdlharami_out_integer: f64,
    #[serde(default)]
    pub cdlharamicross_out_integer: f64,
    #[serde(default)]
    pub cdlhighwave_out_integer: f64,
    #[serde(default)]
    pub cdlhikkake_out_integer: f64,
    #[serde(default)]
    pub cdlhikkakemod_out_integer: f64,
    #[serde(default)]
    pub cdlhomingpigeon_out_integer: f64,
    #[serde(default)]
    pub cdlidentical3crows_out_integer: f64,
    #[serde(default)]
    pub cdlinneck_out_integer: f64,
    #[serde(default)]
    pub cdlinvertedhammer_out_integer: f64,
    #[serde(default)]
    pub cdlkicking_out_integer: f64,
    #[serde(default)]
    pub cdlkickingbylength_out_integer: f64,
    #[serde(default)]
    pub cdlladderbottom_out_integer: f64,
    #[serde(default)]
    pub cdllongleggeddoji_out_integer: f64,
    #[serde(default)]
    pub cdllongline_out_integer: f64,
    #[serde(default)]
    pub cdlmarubozu_out_integer: f64,
    #[serde(default)]
    pub cdlmatchinglow_out_integer: f64,
    #[serde(default)]
    pub cdlmathold_out_integer: f64,
    #[serde(default)]
    pub cdlmorningdojistar_out_integer: f64,
    #[serde(default)]
    pub cdlmorningstar_out_integer: f64,
    #[serde(default)]
    pub cdlonneck_out_integer: f64,
    #[serde(default)]
    pub cdlpiercing_out_integer: f64,
    #[serde(default)]
    pub cdlrickshawman_out_integer: f64,
    #[serde(default)]
    pub cdlrisefall3methods_out_integer: f64,
    #[serde(default)]
    pub cdlseparatinglines_out_integer: f64,
    #[serde(default)]
    pub cdlshootingstar_out_integer: f64,
    #[serde(default)]
    pub cdlshortline_out_integer: f64,
    #[serde(default)]
    pub cdlspinningtop_out_integer: f64,
    #[serde(default)]
    pub cdlstalledpattern_out_integer: f64,
    #[serde(default)]
    pub cdlsticksandwich_out_integer: f64,
    #[serde(default)]
    pub cdltakuri_out_integer: f64,
    #[serde(default)]
    pub cdltasukigap_out_integer: f64,
    #[serde(default)]
    pub cdlthrusting_out_integer: f64,
    #[serde(default)]
    pub cdltristar_out_integer: f64,
    #[serde(default)]
    pub cdlunique3river_out_integer: f64,
    #[serde(default)]
    pub cdlupsidegap2crows_out_integer: f64,
    #[serde(default)]
    pub cdlxsidegap3methods_out_integer: f64,
    #[serde(default)]
    pub ceil_out_real: f64,
    #[serde(default)]
    pub cmo_out_real: f64,
    #[serde(default)]
    pub correl_out_real: f64,
    #[serde(default)]
    pub cos_out_real: f64,
    #[serde(default)]
    pub cosh_out_real: f64,
    #[serde(default)]
    pub dema_out_real: f64,
    #[serde(default)]
    pub div_out_real: f64,
    #[serde(default)]
    pub dx_out_real: f64,
    #[serde(default)]
    pub ema_out_real: f64,
    #[serde(default)]
    pub exp_out_real: f64,
    #[serde(default)]
    pub floor_out_real: f64,
    #[serde(default)]
    pub ht_dcperiod_out_real: f64,
    #[serde(default)]
    pub ht_dcphase_out_real: f64,
    #[serde(default)]
    pub ht_phasor_out_in_phase: f64,
    #[serde(default)]
    pub ht_phasor_out_quadrature: f64,
    #[serde(default)]
    pub ht_sine_out_sine: f64,
    #[serde(default)]
    pub ht_sine_out_lead_sine: f64,
    #[serde(default)]
    pub ht_trendline_out_real: f64,
    #[serde(default)]
    pub ht_trendmode_out_integer: f64,
    #[serde(default)]
    pub kama_out_real: f64,
    #[serde(default)]
    pub linearreg_out_real: f64,
    #[serde(default)]
    pub linearreg_angle_out_real: f64,
    #[serde(default)]
    pub linearreg_intercept_out_real: f64,
    #[serde(default)]
    pub linearreg_slope_out_real: f64,
    #[serde(default)]
    pub ln_out_real: f64,
    #[serde(default)]
    pub log10_out_real: f64,
    #[serde(default)]
    pub ma_out_real: f64,
    #[serde(default)]
    pub macd_out_macd: f64,
    #[serde(default)]
    pub macd_out_macd_signal: f64,
    #[serde(default)]
    pub macd_out_macd_hist: f64,
    #[serde(default)]
    pub macdext_out_macd: f64,
    #[serde(default)]
    pub macdext_out_macd_signal: f64,
    #[serde(default)]
    pub macdext_out_macd_hist: f64,
    #[serde(default)]
    pub macdfix_out_macd: f64,
    #[serde(default)]
    pub macdfix_out_macd_signal: f64,
    #[serde(default)]
    pub macdfix_out_macd_hist: f64,
    #[serde(default)]
    pub mama_out_mama: f64,
    #[serde(default)]
    pub mama_out_fama: f64,
    #[serde(default)]
    pub mavp_out_real: f64,
    #[serde(default)]
    pub max_out_real: f64,
    #[serde(default)]
    pub maxindex_out_integer: f64,
    #[serde(default)]
    pub medprice_out_real: f64,
    #[serde(default)]
    pub mfi_out_real: f64,
    #[serde(default)]
    pub midpoint_out_real: f64,
    #[serde(default)]
    pub midprice_out_real: f64,
    #[serde(default)]
    pub min_out_real: f64,
    #[serde(default)]
    pub minindex_out_integer: f64,
    #[serde(default)]
    pub minmax_out_min: f64,
    #[serde(default)]
    pub minmax_out_max: f64,
    #[serde(default)]
    pub minmaxindex_out_min_idx: f64,
    #[serde(default)]
    pub minmaxindex_out_max_idx: f64,
    #[serde(default)]
    pub minus_di_out_real: f64,
    #[serde(default)]
    pub minus_dm_out_real: f64,
    #[serde(default)]
    pub mom_out_real: f64,
    #[serde(default)]
    pub mult_out_real: f64,
    #[serde(default)]
    pub natr_out_real: f64,
    #[serde(default)]
    pub obv_out_real: f64,
    #[serde(default)]
    pub plus_di_out_real: f64,
    #[serde(default)]
    pub plus_dm_out_real: f64,
    #[serde(default)]
    pub ppo_out_real: f64,
    #[serde(default)]
    pub roc_out_real: f64,
    #[serde(default)]
    pub rocp_out_real: f64,
    #[serde(default)]
    pub rocr_out_real: f64,
    #[serde(default)]
    pub rocr100_out_real: f64,
    #[serde(default)]
    pub rsi_out_real: f64,
    #[serde(default)]
    pub sar_out_real: f64,
    #[serde(default)]
    pub sarext_out_real: f64,
    #[serde(default)]
    pub sin_out_real: f64,
    #[serde(default)]
    pub sinh_out_real: f64,
    #[serde(default)]
    pub sma_out_real: f64,
    #[serde(default)]
    pub sqrt_out_real: f64,
    #[serde(default)]
    pub stddev_out_real: f64,
    #[serde(default)]
    pub stoch_out_slow_k: f64,
    #[serde(default)]
    pub stoch_out_slow_d: f64,
    #[serde(default)]
    pub stochf_out_fast_k: f64,
    #[serde(default)]
    pub stochf_out_fast_d: f64,
    #[serde(default)]
    pub stochrsi_out_fast_k: f64,
    #[serde(default)]
    pub stochrsi_out_fast_d: f64,
    #[serde(default)]
    pub sub_out_real: f64,
    #[serde(default)]
    pub sum_out_real: f64,
    #[serde(default)]
    pub t3_out_real: f64,
    #[serde(default)]
    pub tan_out_real: f64,
    #[serde(default)]
    pub tanh_out_real: f64,
    #[serde(default)]
    pub tema_out_real: f64,
    #[serde(default)]
    pub trange_out_real: f64,
    #[serde(default)]
    pub trima_out_real: f64,
    #[serde(default)]
    pub trix_out_real: f64,
    #[serde(default)]
    pub tsf_out_real: f64,
    #[serde(default)]
    pub typprice_out_real: f64,
    #[serde(default)]
    pub ultosc_out_real: f64,
    #[serde(default)]
    pub var_out_real: f64,
    #[serde(default)]
    pub wclprice_out_real: f64,
    #[serde(default)]
    pub willr_out_real: f64,
    #[serde(default)]
    pub wma_out_real: f64,
}

#[serde(crate = "deps")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComboInstrument {
    pub instrument: Instrument,
    pub points: f64,
    pub history: Vec<Vec<Candle>>,
}

#[serde(crate = "deps")]
#[derive(Debug, Clone, Identity, Serialize, Deserialize)]
pub struct ComboGroup {
    pub name: String,
    pub combos: Vec<Combo>,
    pub id: usize,
}

#[serde(crate = "deps")]
#[derive(Debug, Clone, Identity, Serialize, Deserialize)]
pub struct Combo {
    pub instrument: Vec<Instrument>,
    pub taf_name: String,
    pub trade_action: TradeAction,
    pub operator: Operator,
    pub operand2: Operand,
    pub aud_cad: ComboInstrument,
    pub aud_chf: ComboInstrument,
    pub aud_hkd: ComboInstrument,
    pub aud_jpy: ComboInstrument,
    pub aud_nzd: ComboInstrument,
    pub aud_sgd: ComboInstrument,
    pub aud_usd: ComboInstrument,
    pub cad_chf: ComboInstrument,
    pub cad_hkd: ComboInstrument,
    pub cad_jpy: ComboInstrument,
    pub cad_sgd: ComboInstrument,
    pub chf_hkd: ComboInstrument,
    pub chf_jpy: ComboInstrument,
    pub eur_aud: ComboInstrument,
    pub eur_cad: ComboInstrument,
    pub eur_chf: ComboInstrument,
    pub eur_gbp: ComboInstrument,
    pub eur_hkd: ComboInstrument,
    pub eur_jpy: ComboInstrument,
    pub eur_nzd: ComboInstrument,
    pub eur_sgd: ComboInstrument,
    pub eur_usd: ComboInstrument,
    pub gbp_aud: ComboInstrument,
    pub gbp_cad: ComboInstrument,
    pub gbp_chf: ComboInstrument,
    pub gbp_hkd: ComboInstrument,
    pub gbp_jpy: ComboInstrument,
    pub gbp_nzd: ComboInstrument,
    pub gbp_sgd: ComboInstrument,
    pub gbp_usd: ComboInstrument,
    pub hkd_jpy: ComboInstrument,
    pub nzd_cad: ComboInstrument,
    pub nzd_chf: ComboInstrument,
    pub nzd_hkd: ComboInstrument,
    pub nzd_jpy: ComboInstrument,
    pub nzd_sgd: ComboInstrument,
    pub nzd_usd: ComboInstrument,
    pub sgd_chf: ComboInstrument,
    pub sgd_hkd: ComboInstrument,
    pub sgd_jpy: ComboInstrument,
    pub usd_cad: ComboInstrument,
    pub usd_chf: ComboInstrument,
    pub usd_hkd: ComboInstrument,
    pub usd_jpy: ComboInstrument,
    pub usd_sgd: ComboInstrument,
}

#[derive(Debug, Clone)]
pub struct BacktestResult {
    pub history: Vec<Candle>,
    pub points: f64,
}

#[serde(crate = "deps")]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Hlocv {
    pub high: Vec<f64>,
    pub low: Vec<f64>,
    pub open: Vec<f64>,
    pub close: Vec<f64>,
    pub volume: Vec<f64>,
}

#[serde(crate = "deps")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YorexInstrument2 {
    pub instruments: Vec<Vec<Instrument>>,
    pub candles: CandleInstrument,
    pub hlocv: Hlocv,
    pub combos: Vec<ComboGroup>,
    pub has_order: bool,
    pub _ig_cst: String,
    pub _ig_xst: String,
    pub _deal_reference: String,
    pub html_single: bool,
}

#[derive(Debug, Clone)]
pub struct ComboGroup2<'a> {
    pub name: String,
    pub combos: Vec<&'a Combo>,
    pub id: usize,
}

pub struct CandleInstrumentRef<'a> {
    pub instrument: Instrument,
    pub candles: &'a Vec<Candle>,
}

#[serde(crate = "deps")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandleInstrument {
    pub aud_cad: Vec<Candle>,
    pub aud_chf: Vec<Candle>,
    pub aud_hkd: Vec<Candle>,
    pub aud_jpy: Vec<Candle>,
    pub aud_nzd: Vec<Candle>,
    pub aud_sgd: Vec<Candle>,
    pub aud_usd: Vec<Candle>,
    pub cad_chf: Vec<Candle>,
    pub cad_hkd: Vec<Candle>,
    pub cad_jpy: Vec<Candle>,
    pub cad_sgd: Vec<Candle>,
    pub chf_hkd: Vec<Candle>,
    pub chf_jpy: Vec<Candle>,
    pub eur_aud: Vec<Candle>,
    pub eur_cad: Vec<Candle>,
    pub eur_chf: Vec<Candle>,
    pub eur_gbp: Vec<Candle>,
    pub eur_hkd: Vec<Candle>,
    pub eur_jpy: Vec<Candle>,
    pub eur_nzd: Vec<Candle>,
    pub eur_sgd: Vec<Candle>,
    pub eur_usd: Vec<Candle>,
    pub gbp_aud: Vec<Candle>,
    pub gbp_cad: Vec<Candle>,
    pub gbp_chf: Vec<Candle>,
    pub gbp_hkd: Vec<Candle>,
    pub gbp_jpy: Vec<Candle>,
    pub gbp_nzd: Vec<Candle>,
    pub gbp_sgd: Vec<Candle>,
    pub gbp_usd: Vec<Candle>,
    pub hkd_jpy: Vec<Candle>,
    pub nzd_cad: Vec<Candle>,
    pub nzd_chf: Vec<Candle>,
    pub nzd_hkd: Vec<Candle>,
    pub nzd_jpy: Vec<Candle>,
    pub nzd_sgd: Vec<Candle>,
    pub nzd_usd: Vec<Candle>,
    pub sgd_chf: Vec<Candle>,
    pub sgd_hkd: Vec<Candle>,
    pub sgd_jpy: Vec<Candle>,
    pub usd_cad: Vec<Candle>,
    pub usd_chf: Vec<Candle>,
    pub usd_hkd: Vec<Candle>,
    pub usd_jpy: Vec<Candle>,
    pub usd_sgd: Vec<Candle>,
}
