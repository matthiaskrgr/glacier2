#![feature(non_lifetime_binders)]
pub fn V<V>(value: &V) -> Result<String, &Ok>{}
