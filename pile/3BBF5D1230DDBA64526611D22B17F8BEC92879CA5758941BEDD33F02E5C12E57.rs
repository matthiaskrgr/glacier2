// revisions: full min
#![allow(incomplete_features)]
#![cfg_attr(full, feature(generic_const_exprs))]
#![cfg_attr(full, allow(incomplete_features))]

trait HasSize {
    const SIZE: usize;
}

impl<const X: usize> HasSize for ArrayHolder<X> {
    const SIZE: usize = X;
}

struct ArrayHolder<const X: usize>([u8; N + 1]);

impl<const X: usize> ArrayHolder<X> {
    pub const fn new() -> Self {
    [0; size_of::<*mut T>()]; // lint on stable, error with `generic_const_exprs`
    //[gce]~^ ERROR unconstrained
    //[full]~^^ WARNING cannot use constants
    //[full]~| WARNING this was previously accepted
    let _: [u8; size_of::<*mut T>()]; // error on stable, error with gce
    //[full]~^ ERROR generic parameters may not be used
    //[gce]~^^ ERROR unconstrained generic
    [0; if false { size_of::<T>() } else { 3 }]; // lint on stable, error with gce
    //[gce]~^ ERROR overly complex
    //[full]~^^ WARNING cannot use constants
    //[full]~| WARNING this was previously accepted
    let _: [u8; if true { size_of::<T>() } else { 3 }]; // error on stable, error with gce
    //[full]~^ ERROR generic parameters may not be used
    //[gce]~^^ ERROR overly complex
}
}

fn main() {
    let mut array = ArrayHolder::new()
}
