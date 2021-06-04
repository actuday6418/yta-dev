#![recursion_limit = "256"]

#[macro_use]
extern crate yorex_technical_analysis_macros;

pub mod enums;
pub mod fns;
pub mod impls;
pub mod macros;
pub mod structs;
pub mod traits;

pub use deps::*;
pub use enums::*;
pub use fns::*;
pub use impls::*;

pub use structs::*;
pub use traits::*;

use deps::rayon::*;
use std::sync::{Arc, RwLock};
use std::time::Instant;

use std::env;

use std::env::var;
use std::ops::Deref;

pub fn ta() {
    let file = std::fs::read("y2.bin").unwrap();
    let yi_unsafe = bincode::deserialize::<YorexInstrument2>(&file).unwrap();
    let y2_arc = Arc::new(RwLock::new(yi_unsafe));

    let combos2 = {
        let yi_arc = Arc::clone(&y2_arc);
        let mut yi = yi_arc.read().unwrap();

        let mut t10 = yi.top10();
        let sets = vec![(t10, 0)];

        sets.into_iter()
            .map(|(v, n)| {
                let mut pset = v.par_pset_ner(n);
                let par_iter: rayon::slice::Iter<Vec<&ComboGroup2>> = pset.par_iter();
                let ll = par_iter
                    .fold(
                        || vec![],
                        |a: Vec<Combo>, b| {
                            let now = offset::Utc::now().naive_utc();
                            let mut t =
                                NaiveDateTime::parse_from_str(env!("TIME_FROM"), "%Y-%m-%dT%H:%M")
                                    .unwrap();
                            let mut c = b
                                .iter()
                                .flat_map(|cg| {
                                    cg.combos
                                        .iter()
                                        .map(|co| {
                                            let mut combo = Combo::default();
                                            combo.taf_name = co.taf_name.clone();
                                            combo.trade_action = co.trade_action;
                                            combo.operand2 = co.operand2.clone();
                                            combo.operator = co.operator.clone();
                                            combo
                                        })
                                        .collect::<Vec<_>>()
                                })
                                .collect::<Vec<_>>();
                            let mut ic = b.iter().fold(vec![], |a: Vec<CandleInstrumentRef>, b| {
                                let ins: Instrument =
                                    Instrument::new(b.name.to_snake_case().as_str());
                                let ci: &Vec<Candle> = ins.instrument_candles(&yi);
                                let cir = CandleInstrumentRef {
                                    instrument: ins,
                                    candles: ci,
                                };
                                let inss = a.iter().map(|aa| aa.instrument).collect::<Vec<_>>();
                                if inss.contains(&ins) {
                                    a
                                } else {
                                    a.into_iter().chain(vec![cir]).collect::<Vec<_>>()
                                }
                            });
                            let mut cocas = b
                                .iter()
                                .map(|cg| {
                                    let ins: Instrument =
                                        Instrument::new(cg.name.to_snake_case().as_str());
                                    let ci: &Vec<Candle> = ins.instrument_candles(&yi);
                                    cg.combos
                                        .iter()
                                        .map(|c| filter_candles_by_combo(c, &ci, ins))
                                        .collect::<Vec<_>>()
                                })
                                .flatten()
                                .collect::<Vec<_>>();

                            backtest_from(
                                t,
                                cocas,
                                |cocas| {
                                    ic.iter()
                                        .find(|c| c.instrument == cocas[0].instrument)
                                        .unwrap()
                                        .candles
                                },
                                |cocas, br| {
                                    let c_mut = c
                                        .iter_mut()
                                        .find(|cc| cocas.combo.id() == cc.id())
                                        .unwrap();
                                    c_mut.br(cocas.instrument, br);
                                },
                            );

                            let icins = ic.iter().map(|f| f.instrument).collect::<Vec<_>>();

                            let bpoints: f64 =
                                c.iter().map(|cc| Instrument::combo_all_points(cc)).sum();
                            let apoints = a.iter().map(|cc| Instrument::combo_all_points(cc)).sum();

                            if bpoints > apoints {
                                c
                            } else {
                                a
                            }
                        },
                    )
                    .reduce(
                        || vec![],
                        |a, b| {
                            let eligible = a.len() > 0 && b.len() > 0 && b.len() < a.len();
                            let bpoints: f64 =
                                b.iter().map(|cc| Instrument::combo_all_points(cc)).sum();
                            let apoints = a.iter().map(|cc| Instrument::combo_all_points(cc)).sum();

                            if bpoints > apoints {
                                b
                            } else if b.len() < a.len() {
                                b
                            } else {
                                a
                            }
                        },
                    );
                ll
            })
            .collect::<Vec<_>>()
    };
}
