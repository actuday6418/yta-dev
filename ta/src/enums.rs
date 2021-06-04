use crate::*;

pub enum YorexError {
    BadInput(String),
}
#[serde(crate = "deps")]
#[derive(
    InstrumentDerive,
    IdentitySelf,
    Debug,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
)]
pub enum Instrument {
    AudCad,
    AudChf,
    AudHkd,
    AudJpy,
    AudNzd,
    AudSgd,
    AudUsd,
    CadChf,
    CadHkd,
    CadJpy,
    CadSgd,
    ChfHkd,
    ChfJpy,
    EurAud,
    EurCad,
    EurChf,
    EurGbp,
    EurHkd,
    EurJpy,
    EurNzd,
    EurSgd,
    EurUsd,
    GbpAud,
    GbpCad,
    GbpChf,
    GbpHkd,
    GbpJpy,
    GbpNzd,
    GbpSgd,
    GbpUsd,
    HkdJpy,
    NzdCad,
    NzdChf,
    NzdHkd,
    NzdJpy,
    NzdSgd,
    NzdUsd,
    SgdChf,
    SgdHkd,
    SgdJpy,
    UsdCad,
    UsdChf,
    UsdHkd,
    UsdJpy,
    UsdSgd,
}

#[serde(crate = "deps")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Operand {
    Num(f64),
    Str(String),
}

#[serde(crate = "deps")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Operator {
    Gt,
    Lt,
}

#[serde(crate = "deps")]
#[derive(Debug, Clone, IdentitySelf, Serialize, Deserialize, Copy)]
pub enum TradeAction {
    Sell,
    Buy,
}

#[derive(Debug, Clone, IdentitySelf)]
pub enum Granularity {
    H4,
}
