use crate::Operand::Str;
use crate::*;
use deps::native_tls::TlsConnector;
use std::env::var;
use std::fmt;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::ops::AddAssign;
use std::sync::Arc;
use std::time::Instant;

impl<T> Powerset<T> for Vec<T> {
    fn pset_ner(&self) -> Vec<Vec<&T>>
    where
        T: Clone,
    {
        self.iter().fold(Vec::new(), |mut acc, x| {
            match acc.is_empty() {
                true => vec![vec![x]],
                false => {
                    let mut new_set = acc
                        .iter()
                        .map(|v| {
                            let mut q = v.iter().map(|&f| f).collect::<Vec<_>>();
                            q.push(x);
                            q
                        })
                        .collect::<Vec<_>>();
                    new_set.push(vec![x]);
                    new_set = new_set.into_iter().chain(acc).collect::<Vec<_>>();
                    new_set
                }
            }
        })
    }
    fn par_pset_ner(&self, min_set: usize) -> Vec<Vec<&T>>
    where
        T: Sync + Sized + Clone,
    {
        self.iter()
            .enumerate()
            .fold(Vec::new(), |acc, (u, x)| match acc.is_empty() {
                true => vec![vec![x]],
                false => {
                    let acc_iter: rayon::slice::Iter<Vec<&T>> = acc.par_iter();
                    let mut new_set = acc_iter
                        .map(|v| {
                            let mut q = v.iter().map(|&f| f).collect::<Vec<_>>();
                            q.push(x);
                            q
                        })
                        .collect::<Vec<_>>();
                    new_set.push(vec![x]);
                    let new_set_iter: rayon::vec::IntoIter<Vec<&T>> = new_set.into_par_iter();
                    new_set = new_set_iter.chain(acc).collect::<Vec<_>>();
                    if u > min_set {
                        new_set
                            .into_iter()
                            .filter(|v| v.len() > min_set)
                            .collect::<Vec<_>>()
                    } else {
                        new_set
                    }
                }
            })
    }
}

impl Combo {
    pub fn top_ci(&self, ins: &Vec<Instrument>) -> &ComboInstrument {
        let mut init = &self.aud_cad;
        ins.iter().for_each(|&ins| {
            let ci: &ComboInstrument = Instrument::ci(ins, &self);
            if ci.points > init.points {
                init = ci
            }
        });
        init
    }
    pub fn br(&mut self, ins: Instrument, br: BacktestResult) {
        match ins {
            Instrument::AudCad => {
                self.aud_cad.points.add_assign(br.points);
                self.aud_cad.history.extend(vec![br.history]);
            }
            Instrument::AudChf => {
                self.aud_chf.points.add_assign(br.points);
                self.aud_chf.history.extend(vec![br.history]);
            }
            Instrument::AudHkd => {
                self.aud_hkd.points.add_assign(br.points);
                self.aud_hkd.history.extend(vec![br.history]);
            }
            Instrument::AudJpy => {
                self.aud_jpy.points.add_assign(br.points);
                self.aud_jpy.history.extend(vec![br.history]);
            }
            Instrument::AudNzd => {
                self.aud_nzd.points.add_assign(br.points);
                self.aud_nzd.history.extend(vec![br.history]);
            }
            Instrument::AudSgd => {
                self.aud_sgd.points.add_assign(br.points);
                self.aud_sgd.history.extend(vec![br.history]);
            }
            Instrument::AudUsd => {
                self.aud_usd.points.add_assign(br.points);
                self.aud_usd.history.extend(vec![br.history]);
            }
            Instrument::CadChf => {
                self.cad_chf.points.add_assign(br.points);
                self.cad_chf.history.extend(vec![br.history]);
            }
            Instrument::CadHkd => {
                self.cad_hkd.points.add_assign(br.points);
                self.cad_hkd.history.extend(vec![br.history]);
            }
            Instrument::CadJpy => {
                self.cad_jpy.points.add_assign(br.points);
                self.cad_jpy.history.extend(vec![br.history]);
            }
            Instrument::CadSgd => {
                self.cad_sgd.points.add_assign(br.points);
                self.cad_sgd.history.extend(vec![br.history]);
            }
            Instrument::ChfHkd => {
                self.chf_hkd.points.add_assign(br.points);
                self.chf_hkd.history.extend(vec![br.history]);
            }
            Instrument::ChfJpy => {
                self.chf_jpy.points.add_assign(br.points);
                self.chf_jpy.history.extend(vec![br.history]);
            }
            Instrument::EurAud => {
                self.eur_aud.points.add_assign(br.points);
                self.eur_aud.history.extend(vec![br.history]);
            }
            Instrument::EurCad => {
                self.eur_cad.points.add_assign(br.points);
                self.eur_cad.history.extend(vec![br.history]);
            }
            Instrument::EurChf => {
                self.eur_chf.points.add_assign(br.points);
                self.eur_chf.history.extend(vec![br.history]);
            }
            Instrument::EurGbp => {
                self.eur_gbp.points.add_assign(br.points);
                self.eur_gbp.history.extend(vec![br.history]);
            }
            Instrument::EurHkd => {
                self.eur_hkd.points.add_assign(br.points);
                self.eur_hkd.history.extend(vec![br.history]);
            }
            Instrument::EurJpy => {
                self.eur_jpy.points.add_assign(br.points);
                self.eur_jpy.history.extend(vec![br.history]);
            }
            Instrument::EurNzd => {
                self.eur_nzd.points.add_assign(br.points);
                self.eur_nzd.history.extend(vec![br.history]);
            }
            Instrument::EurSgd => {
                self.eur_sgd.points.add_assign(br.points);
                self.eur_sgd.history.extend(vec![br.history]);
            }
            Instrument::EurUsd => {
                self.eur_usd.points.add_assign(br.points);
                self.eur_usd.history.extend(vec![br.history]);
            }
            Instrument::GbpAud => {
                self.gbp_aud.points.add_assign(br.points);
                self.gbp_aud.history.extend(vec![br.history]);
            }
            Instrument::GbpCad => {
                self.gbp_cad.points.add_assign(br.points);
                self.gbp_cad.history.extend(vec![br.history]);
            }
            Instrument::GbpChf => {
                self.gbp_chf.points.add_assign(br.points);
                self.gbp_chf.history.extend(vec![br.history]);
            }
            Instrument::GbpHkd => {
                self.gbp_hkd.points.add_assign(br.points);
                self.gbp_hkd.history.extend(vec![br.history]);
            }
            Instrument::GbpJpy => {
                self.gbp_jpy.points.add_assign(br.points);
                self.gbp_jpy.history.extend(vec![br.history]);
            }
            Instrument::GbpNzd => {
                self.gbp_nzd.points.add_assign(br.points);
                self.gbp_nzd.history.extend(vec![br.history]);
            }
            Instrument::GbpSgd => {
                self.gbp_sgd.points.add_assign(br.points);
                self.gbp_sgd.history.extend(vec![br.history]);
            }
            Instrument::GbpUsd => {
                self.gbp_usd.points.add_assign(br.points);
                self.gbp_usd.history.extend(vec![br.history]);
            }
            Instrument::HkdJpy => {
                self.hkd_jpy.points.add_assign(br.points);
                self.hkd_jpy.history.extend(vec![br.history]);
            }
            Instrument::NzdCad => {
                self.nzd_cad.points.add_assign(br.points);
                self.nzd_cad.history.extend(vec![br.history]);
            }
            Instrument::NzdChf => {
                self.nzd_chf.points.add_assign(br.points);
                self.nzd_chf.history.extend(vec![br.history]);
            }
            Instrument::NzdHkd => {
                self.nzd_hkd.points.add_assign(br.points);
                self.nzd_hkd.history.extend(vec![br.history]);
            }
            Instrument::NzdJpy => {
                self.nzd_jpy.points.add_assign(br.points);
                self.nzd_jpy.history.extend(vec![br.history]);
            }
            Instrument::NzdSgd => {
                self.nzd_sgd.points.add_assign(br.points);
                self.nzd_sgd.history.extend(vec![br.history]);
            }
            Instrument::NzdUsd => {
                self.nzd_usd.points.add_assign(br.points);
                self.nzd_usd.history.extend(vec![br.history]);
            }
            Instrument::SgdChf => {
                self.sgd_chf.points.add_assign(br.points);
                self.sgd_chf.history.extend(vec![br.history]);
            }
            Instrument::SgdHkd => {
                self.sgd_hkd.points.add_assign(br.points);
                self.sgd_hkd.history.extend(vec![br.history]);
            }
            Instrument::SgdJpy => {
                self.sgd_jpy.points.add_assign(br.points);
                self.sgd_jpy.history.extend(vec![br.history]);
            }
            Instrument::UsdCad => {
                self.usd_cad.points.add_assign(br.points);
                self.usd_cad.history.extend(vec![br.history]);
            }
            Instrument::UsdChf => {
                self.usd_chf.points.add_assign(br.points);
                self.usd_chf.history.extend(vec![br.history]);
            }
            Instrument::UsdHkd => {
                self.usd_hkd.points.add_assign(br.points);
                self.usd_hkd.history.extend(vec![br.history]);
            }
            Instrument::UsdJpy => {
                self.usd_jpy.points.add_assign(br.points);
                self.usd_jpy.history.extend(vec![br.history]);
            }
            Instrument::UsdSgd => {
                self.usd_sgd.points.add_assign(br.points);
                self.usd_sgd.history.extend(vec![br.history]);
            }
        }
    }
    pub fn id(&self) -> String {
        format!(
            "{}{:?}{:?}{:?}",
            self.taf_name, self.operator, self.operand2, self.trade_action
        )
    }
}

impl Default for ComboInstrument {
    fn default() -> Self {
        ComboInstrument {
            points: 0.0,
            history: vec![],
            instrument: Instrument::AudCad,
        }
    }
}

impl Default for Combo {
    fn default() -> Self {
        Combo {
            instrument: vec![],
            taf_name: "".to_string(),
            trade_action: TradeAction::Sell,
            operator: Operator::Gt,
            operand2: Operand::Num(f64::NAN),
            aud_cad: ComboInstrument {
                points: 0.0,
                history: vec![],
                instrument: Instrument::AudCad,
            },
            aud_chf: ComboInstrument {
                points: 0.0,
                history: vec![],
                instrument: Instrument::AudChf,
            },
            aud_hkd: ComboInstrument {
                points: 0.0,
                history: vec![],
                instrument: Instrument::AudHkd,
            },
            aud_jpy: ComboInstrument {
                points: 0.0,
                history: vec![],
                instrument: Instrument::AudJpy,
            },
            aud_nzd: ComboInstrument {
                points: 0.0,
                history: vec![],
                instrument: Instrument::AudNzd,
            },
            aud_sgd: ComboInstrument {
                points: 0.0,
                history: vec![],
                instrument: Instrument::AudSgd,
            },
            aud_usd: ComboInstrument {
                points: 0.0,
                history: vec![],
                instrument: Instrument::AudUsd,
            },
            cad_chf: ComboInstrument {
                points: 0.0,
                history: vec![],
                instrument: Instrument::CadChf,
            },
            cad_hkd: ComboInstrument {
                points: 0.0,
                history: vec![],
                instrument: Instrument::CadHkd,
            },
            cad_jpy: ComboInstrument {
                points: 0.0,
                history: vec![],
                instrument: Instrument::CadJpy,
            },
            cad_sgd: ComboInstrument {
                points: 0.0,
                history: vec![],
                instrument: Instrument::CadSgd,
            },
            chf_hkd: ComboInstrument {
                points: 0.0,
                history: vec![],
                instrument: Instrument::ChfHkd,
            },
            chf_jpy: ComboInstrument {
                points: 0.0,
                history: vec![],
                instrument: Instrument::ChfJpy,
            },
            eur_aud: ComboInstrument {
                points: 0.0,
                history: vec![],
                instrument: Instrument::EurAud,
            },
            eur_cad: ComboInstrument {
                points: 0.0,
                history: vec![],
                instrument: Instrument::EurCad,
            },
            eur_chf: ComboInstrument {
                points: 0.0,
                history: vec![],
                instrument: Instrument::EurChf,
            },
            eur_gbp: ComboInstrument {
                points: 0.0,
                history: vec![],
                instrument: Instrument::EurGbp,
            },
            eur_hkd: ComboInstrument {
                points: 0.0,
                history: vec![],
                instrument: Instrument::EurHkd,
            },
            eur_jpy: ComboInstrument {
                points: 0.0,
                history: vec![],
                instrument: Instrument::EurJpy,
            },
            eur_nzd: ComboInstrument {
                points: 0.0,
                history: vec![],
                instrument: Instrument::EurNzd,
            },
            eur_sgd: ComboInstrument {
                points: 0.0,
                history: vec![],
                instrument: Instrument::EurSgd,
            },
            eur_usd: ComboInstrument {
                points: 0.0,
                history: vec![],
                instrument: Instrument::EurUsd,
            },
            gbp_aud: ComboInstrument {
                points: 0.0,
                history: vec![],
                instrument: Instrument::GbpAud,
            },
            gbp_cad: ComboInstrument {
                points: 0.0,
                history: vec![],
                instrument: Instrument::GbpCad,
            },
            gbp_chf: ComboInstrument {
                points: 0.0,
                history: vec![],
                instrument: Instrument::GbpChf,
            },
            gbp_hkd: ComboInstrument {
                points: 0.0,
                history: vec![],
                instrument: Instrument::GbpHkd,
            },
            gbp_jpy: ComboInstrument {
                points: 0.0,
                history: vec![],
                instrument: Instrument::GbpJpy,
            },
            gbp_nzd: ComboInstrument {
                points: 0.0,
                history: vec![],
                instrument: Instrument::GbpNzd,
            },
            gbp_sgd: ComboInstrument {
                points: 0.0,
                history: vec![],
                instrument: Instrument::GbpSgd,
            },
            gbp_usd: ComboInstrument {
                points: 0.0,
                history: vec![],
                instrument: Instrument::GbpUsd,
            },
            hkd_jpy: ComboInstrument {
                points: 0.0,
                history: vec![],
                instrument: Instrument::HkdJpy,
            },
            nzd_cad: ComboInstrument {
                points: 0.0,
                history: vec![],
                instrument: Instrument::NzdCad,
            },
            nzd_chf: ComboInstrument {
                points: 0.0,
                history: vec![],
                instrument: Instrument::NzdChf,
            },
            nzd_hkd: ComboInstrument {
                points: 0.0,
                history: vec![],
                instrument: Instrument::NzdHkd,
            },
            nzd_jpy: ComboInstrument {
                points: 0.0,
                history: vec![],
                instrument: Instrument::NzdJpy,
            },
            nzd_sgd: ComboInstrument {
                points: 0.0,
                history: vec![],
                instrument: Instrument::NzdSgd,
            },
            nzd_usd: ComboInstrument {
                points: 0.0,
                history: vec![],
                instrument: Instrument::NzdUsd,
            },
            sgd_chf: ComboInstrument {
                points: 0.0,
                history: vec![],
                instrument: Instrument::SgdChf,
            },
            sgd_hkd: ComboInstrument {
                points: 0.0,
                history: vec![],
                instrument: Instrument::SgdHkd,
            },
            sgd_jpy: ComboInstrument {
                points: 0.0,
                history: vec![],
                instrument: Instrument::SgdJpy,
            },
            usd_cad: ComboInstrument {
                points: 0.0,
                history: vec![],
                instrument: Instrument::UsdCad,
            },
            usd_chf: ComboInstrument {
                points: 0.0,
                history: vec![],
                instrument: Instrument::UsdChf,
            },
            usd_hkd: ComboInstrument {
                points: 0.0,
                history: vec![],
                instrument: Instrument::UsdHkd,
            },
            usd_jpy: ComboInstrument {
                points: 0.0,
                history: vec![],
                instrument: Instrument::UsdJpy,
            },
            usd_sgd: ComboInstrument {
                points: 0.0,
                history: vec![],
                instrument: Instrument::UsdSgd,
            },
        }
    }
}

impl fmt::Display for crate::YorexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            YorexError::BadInput(info) => write!(f, "bad input {}", info),
        }
    }
}

impl YorexInstrument2 {
    pub fn top10(&self) -> Vec<ComboGroup2> {
        let instruments = &self.instruments;
        let mut cg = self
            .combos
            .iter()
            .enumerate()
            .map(|(u, x)| {
                let mut combos = x.combos.iter().map(|f| f).collect::<Vec<_>>();
                combos.sort_by(|a, b| {
                    let cib: &ComboInstrument = b.top_ci(&instruments[u]);
                    let cia: &ComboInstrument = a.top_ci(&instruments[u]);
                    cib.points.partial_cmp(&cia.points).unwrap()
                });
                ComboGroup2 {
                    name: x.name.clone(),
                    combos,
                    id: x.id,
                }
            })
            .collect::<Vec<_>>();
        cg.sort_by(|a, b| {
            let cib: &ComboInstrument = &b.combos[0].top_ci(&instruments[b.id]);
            let cia: &ComboInstrument = &a.combos[0].top_ci(&instruments[a.id]);
            cib.points.partial_cmp(&cia.points).unwrap()
        });
        cg.iter_mut().for_each(|c| {
            c.name = c.combos[0]
                .top_ci(&instruments[c.id])
                .instrument
                .identity_self()
        });
        cg.into_iter().take(10).collect::<Vec<_>>()
    }
    pub fn top_each_ins(&mut self) -> Vec<ComboGroup2> {
        let cos = &self.combos;
        let mut instruments = self.instruments.iter().flatten().collect::<Vec<_>>();
        instruments.sort();
        instruments.dedup();
        instruments
            .iter()
            .enumerate()
            .map(|(u, &&x)| {
                let mut cg2 = ComboGroup2 {
                    name: "".to_string(),
                    combos: vec![],
                    id: 0,
                };
                cos.iter().fold(cg2, |a, b| {
                    let bp = b
                        .combos
                        .iter()
                        .map(|c| Instrument::ci(x, c).points)
                        .sum::<f64>();
                    let ap = a
                        .combos
                        .iter()
                        .map(|&c| Instrument::ci(x, c).points)
                        .sum::<f64>();
                    if bp > ap {
                        ComboGroup2 {
                            name: x.identity_self(),
                            combos: b.combos.iter().collect::<Vec<_>>(),
                            id: b.id,
                        }
                    } else {
                        a
                    }
                })
            })
            .collect::<Vec<_>>()
    }
}

impl Default for YorexInstrument2 {
    fn default() -> Self {
        YorexInstrument2 {
            instruments: vec![],
            candles: CandleInstrument {
                aud_cad: vec![],
                aud_chf: vec![],
                aud_hkd: vec![],
                aud_jpy: vec![],
                aud_nzd: vec![],
                aud_sgd: vec![],
                aud_usd: vec![],
                cad_chf: vec![],
                cad_hkd: vec![],
                cad_jpy: vec![],
                cad_sgd: vec![],
                chf_hkd: vec![],
                chf_jpy: vec![],
                eur_aud: vec![],
                eur_cad: vec![],
                eur_chf: vec![],
                eur_gbp: vec![],
                eur_hkd: vec![],
                eur_jpy: vec![],
                eur_nzd: vec![],
                eur_sgd: vec![],
                eur_usd: vec![],
                gbp_aud: vec![],
                gbp_cad: vec![],
                gbp_chf: vec![],
                gbp_hkd: vec![],
                gbp_jpy: vec![],
                gbp_nzd: vec![],
                gbp_sgd: vec![],
                gbp_usd: vec![],
                hkd_jpy: vec![],
                nzd_cad: vec![],
                nzd_chf: vec![],
                nzd_hkd: vec![],
                nzd_jpy: vec![],
                nzd_sgd: vec![],
                nzd_usd: vec![],
                sgd_chf: vec![],
                sgd_hkd: vec![],
                sgd_jpy: vec![],
                usd_cad: vec![],
                usd_chf: vec![],
                usd_hkd: vec![],
                usd_jpy: vec![],
                usd_sgd: vec![],
            },
            hlocv: Default::default(),
            combos: vec![],
            has_order: false,
            _ig_cst: "".to_string(),
            _ig_xst: "".to_string(),
            _deal_reference: "".to_string(),
            html_single: true,
        }
    }
}

impl Granularity {
    pub fn last_trade_time(&self) -> NaiveDateTime {
        let mut now = offset::Utc::now().naive_utc();
        match self {
            Granularity::H4 => {
                now = Utc
                    .ymd(now.year(), now.month(), now.day())
                    .and_hms_nano(now.hour(), 0, 0, 0)
                    .naive_utc();
                now
            }
        }
    }
    pub fn duration(&self) -> Duration {
        match self {
            Granularity::H4 => Duration::hours(4),
        }
    }
    pub fn index_step(&self) -> usize {
        match self {
            Granularity::H4 => 60,
        }
    }
    pub fn expire(&self) -> Duration {
        match self {
            Granularity::H4 => Duration::weeks(2),
        }
    }
}

impl From<&str> for Granularity {
    fn from(item: &str) -> Self {
        match item.to_lowercase().as_str() {
            "h4" => Granularity::H4,
            _ => panic!("unknown granularity"),
        }
    }
}
