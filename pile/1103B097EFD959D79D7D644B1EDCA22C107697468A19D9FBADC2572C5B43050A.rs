// check-pass

#![feature(adt_const_params, generic_const_exprs)]
//~^ WARN the feature `adt_const_params` is incomplete and may not be safe to use and/or cause compiler crashes [incomplete_features]
//~^^ WARN the feature `generic_const_exprs` is incomplete and may not be safe to use and/or cause compiler crashes [incomplete_features]

pub struct Changes<const CHANGES: &'static [&'static str]>
where
    [(); CHANGES.len()]:,
{
    changes: [usize; CHANGES.len()],
}

impl<const CHANGES: &'static [&'static str]> Changes<CHANGES>
where
    [(); CHANGES.len()]:,
{
    pub fn yes_debug() -> impl std::fmt::Debug {
    [0; 33]
}
}

pub fn main() {}