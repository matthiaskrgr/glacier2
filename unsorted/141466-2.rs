//@ compile-flags: --edition=2021 --crate-type lib
pub fn foo() {
    async || {
        use std::ops::RangeInclusive;
        &RangeInclusive::new((), ());
    };
}
