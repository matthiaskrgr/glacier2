//@compile-flags: --crate-type=lib
#![feature(with_negative_coherence, negative_impls, min_generic_const_args)]
trait Trait {}
impl<T> Trait for [(); N] {}

fn N() {}
