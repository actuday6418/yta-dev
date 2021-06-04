use crate::*;

pub trait Identity {
    fn identity() -> String;
}

pub trait Powerset<T> {
    fn pset_ner(&self) -> Vec<Vec<&T>>
    where
        T: Clone;
    fn par_pset_ner(&self, min_set: usize) -> Vec<Vec<&T>>
    where
        T: Sync + Sized + Clone;
}

pub trait IdentitySelf {
    fn identity_self(&self) -> String;
}
