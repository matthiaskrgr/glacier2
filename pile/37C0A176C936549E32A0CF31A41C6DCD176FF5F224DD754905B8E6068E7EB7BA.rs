// FIXME(effects)
// check-pass
#![feature(const_trait_impl, effects)]

pub const fn equals_self<T: ~const FnOnce() -> u8>(t: &T) -> bool {
    *t == *t
    // (remove this) ~^ ERROR can't compare
}

fn main() {}
