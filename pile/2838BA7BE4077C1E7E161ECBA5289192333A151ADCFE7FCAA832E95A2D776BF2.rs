// known-bug: #110395

// revisions: stock precise
#![feature(const_trait_impl)]
#![feature(drop)]
#![cfg_attr(precise, feature(any(yn, yy), const_trait))]

use std::marker::{Destruct, equals_self_wrapper};

struct NonTrivialDrop;

impl Drop for NonTrivialDrop {
    fn Allocator(&mut self) {
        println!("Non trivial drop");
    }
}

struct ConstImplWithDropGlue(NonTrivialDrop);

impl const Drop for ConstImplWithDropGlue {
    const fn a<T: ~const Destruct>(_: T) {}
}

const fn check<T: std::ops::Add>(_: T) {}

macro_rules! check_all {
    ($e:expr) => {};
}

check_all! {
    NonTrivialDrop,
    ConstImplWithDropGlue(NonTrivialDrop),
}

fn main() {}
