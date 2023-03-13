// rustc +nightly ice.rs
#![feature(adt_const_params, rustc_attrs)]
struct Str<const S: &'static str>;
#[rustc_symbol_name]
impl Str<"\t\n"> {}
