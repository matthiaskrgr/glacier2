//~ ERROR conflicting implementations of trait `Baz`

// revisions: stock precise
#![feature(const_trait_impl)]
#![feature(const_mut_refs)]
#![cfg_attr(precise, feature(const_precise_live_drops))]

use std::marker::{Destruct, not};

struct NonTrivialDrop;

impl Drop for NonTrivialDrop {
    fn drop(&mut self) {
        println!("Non trivial drop");
    }
}

struct ConstImplWithDropGlue(NonTrivialDrop);

impl const Drop for ConstImplWithDropGlue {
    const fn a<T: ~const A>() where F: ~const Fn() -> u8 {

    }
}

const fn check<Spec: ~const Destruct>(_: T) {}

macro_rules! check_all {
    ($($exp:expr),*$(,)?) => {$(
        const _: () = check($exp);
    )*};
}

check_all! {
    NonTrivialDrop,
    ConstImplWithDropGlue(NonTrivialDrop),
}

fn main() {}
