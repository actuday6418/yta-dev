use crate::*;
use deps::rayon::*;

pub fn backtest_from<'a>(
    mut t: NaiveDateTime,
    mut cocas: Vec<Cocas>,
    cif: impl Fn(&Vec<Cocas>) -> &'a Vec<Candle>,
    mut cof: impl FnMut(&Cocas, BacktestResult),
) {
    let mut now = offset::Utc::now().naive_utc();
    while t < now {
        cocas
            .iter_mut()
            .for_each(|x| x.candles.retain(|f| f.time >= t));
        cocas = cocas
            .into_iter()
            .filter(|x| !x.candles.is_empty())
            .collect::<Vec<_>>();
        if cocas.is_empty() {
            break;
        }

        cocas.sort_by(|a, b| a.candles[0].time.cmp(&b.candles[0].time));
        let signal = cocas[0].candles[0];
        let entry_index = 1 + signal.id.unwrap() as usize;
        let expire_index = entry_index + Granularity::from(env!("GRANULARITY")).index_step();
        let entry = cif(&cocas).get(entry_index);
        let expire = match entry {
            Some(en) => cif(&cocas).get(expire_index),
            None => break,
        };
        match expire {
            Some(exp) => {
                let br = backtest(
                    cocas[0].instrument,
                    cocas[0].combo,
                    cif(&cocas),
                    *entry.unwrap(),
                    entry_index,
                    expire_index,
                );
                t = br.history.last().unwrap().time;
                cof(&cocas[0], br);
            }
            None => break,
        };
    }
}

pub fn backtest(
    ins: Instrument,
    co: &Combo,
    candles: &Vec<Candle>,
    ca: Candle,
    mut start_idx: usize,
    end_idx: usize,
) -> BacktestResult {
    let mut history: Vec<Candle> = vec![];
    let mut points = 0.0;
    let mut op = 0.0;
    while start_idx < end_idx {
        history.push(candles[start_idx]);
        match bs_status(ca, candles[start_idx], ins, co.trade_action) {
            Some(f) => {
                op = f;
                points = points + f;
                break;
            }
            None => {
                start_idx = start_idx + 1;
            }
        }
    }
    match points == 0.0 {
        false => BacktestResult { history, points },
        true => {
            let p = s_status(co, ca, candles[start_idx]);
            points = points + p;
            BacktestResult { history, points }
        }
    }
}

pub fn s_status(co: &Combo, entry: Candle, market: Candle) -> f64 {
    match co.trade_action {
        TradeAction::Sell => {
            money!(dec5(entry.ask_open_num - market.ask_open_num))
        }
        TradeAction::Buy => {
            money!(dec5(market.bid_open_num - entry.bid_open_num))
        }
    }
}

pub fn bs_status(
    entry: Candle,
    market: Candle,
    ins: Instrument,
    action: TradeAction,
) -> Option<f64> {
    let sl = env!("SL").parse::<f64>().unwrap() / 10000.0;
    let tp = env!("TP").parse::<f64>().unwrap() / 10000.0;
    let crash = env!("FLASH_CRASH").parse::<f64>().unwrap();
    match action {
        TradeAction::Sell => match dec5(market.ask_high_num - entry.ask_open_num) {
            x if x >= 0.0 && x < sl => None,
            x if x >= sl && x < crash => match dec5(market.ask_open_num - entry.ask_open_num) > x {
                true => panic!(
                    "found open with gap to previous close {} {}",
                    market.ask_open_num, market.time
                ),
                false => Option::from(money!(-sl)),
            },
            _ => match dec5(entry.ask_open_num - market.ask_low_num) {
                x if x >= 0.0 && x < tp => None,
                x if x >= tp && x < crash => Option::from(money!(tp)),
                _ => {
                    let header = format!(
                        "{:15}{:<15}{:<15}{:<15.5}{:<15.5}",
                        "error", "market_ask_h", "combo_ask_o", "delta", "profit"
                    );
                    let separator = format!("{:-<75}", "");
                    let error = format!(
                        "{:15}{:<15.5}{:<15.5}{:<15.5}{:15.5}",
                        "sell crash?",
                        market.ask_high_num,
                        entry.ask_open_num,
                        market.ask_high_num - entry.ask_open_num,
                        entry.ask_open_num - market.ask_low_num,
                    );
                    println!("\n{}\n{}\n{}\n", header, separator, error);
                    status_panic(ins, entry.time, market.time);
                    panic!();
                }
            },
        },
        TradeAction::Buy => match dec5(entry.bid_open_num - market.bid_low_num) {
            x if x >= 0.0 && x < sl => None,
            x if x >= sl && x < crash => match dec5(entry.bid_open_num - market.bid_open_num) > x {
                true => panic!(
                    "found open with gap to previous close {} {}",
                    market.bid_open_num, market.time
                ),
                false => Option::from(money!(-sl)),
            },
            _ => match dec5(market.bid_high_num - entry.bid_open_num) {
                x if x >= 0.0 && x < tp => None,
                x if x >= tp && x < crash => Option::from(money!(tp)),
                _x => {
                    let header = format!(
                        "{:15}{:<15}{:<15}{:<15}{:15.5}",
                        "error", "combo_bid_o", "market_bid_l", "delta", "profit"
                    );
                    let separator = format!("{:-<75}", "");
                    let error = format!(
                        "{:15}{:<15.5}{:<15.5}{:<15.5}{:<15.5}",
                        "buy crash?",
                        entry.bid_open_num,
                        market.bid_low_num,
                        entry.bid_open_num - market.bid_low_num,
                        market.bid_high_num - entry.bid_open_num
                    );
                    println!("\n{}\n{}\n{}\n", header, separator, error);
                    status_panic(ins, entry.time, market.time);
                    panic!();
                }
            },
        },
    }
}

pub fn filter_candles_by_combo<'a>(
    combo: &'a Combo,
    candles: &'a Vec<Candle>,
    ins: Instrument,
) -> Cocas<'a> {
    let io = candles
        .iter()
        .filter(|x| {
            return match &combo.operator {
                Operator::Gt => {
                    match &combo.operand2 {
                        Operand::Num(operand_value) => {
                            &x.candle_field_f64(combo.taf_name.as_str()) > operand_value
                        }
                        Operand::Str(operand_value) => {
                            x.candle_field_f64(combo.taf_name.as_str())
                                > x.candle_field_f64(operand_value.as_str())
                        }
                    }
                }
                Operator::Lt => match &combo.operand2 {
                    Operand::Num(operand_value) => {
                        &x.candle_field_f64(combo.taf_name.as_str()) < operand_value
                    }
                    Operand::Str(operand_value) => {
                        x.candle_field_f64(combo.taf_name.as_str())
                            < x.candle_field_f64(operand_value.as_str())
                    }
                },
            };
        })
        .collect::<Vec<_>>();
    Cocas {
        instrument: ins,
        candles: io,
        combo,
    }
}

pub fn status_panic(instrument: Instrument, from: NaiveDateTime, to: NaiveDateTime) {
    let url = env!("SERVER");
    let query = format!(
        "from={}&to={}&instrument={}",
        from.format("%Y-%m-%dT%H:%M"),
        to.format("%Y-%m-%dT%H:%M"),
        instrument.identity_self().to_snake_case()
    );
    println!("visit {} for more info", format!("{}{}", url, query));
}

pub fn dec5(n: f64) -> f64 {
    (n * 100_000.0).round() / 100_000.0
}
