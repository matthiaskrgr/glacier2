#![allow(incomplete_features, unstable_features)]
#![feature(generic_const_exprs)]

struct Combination<const STRATEGIES: usize>;

impl<const STRATEGIES: usize> Combination<STRATEGIES> {
    fn and<M>(self) -> Combination<{ STRATEGIES + 1 }> {
        Combination
    }
}

pub fn trigger_the_ice() {
    Combination::<0>.and::<_>().and::<_>();
}
