//@compile-flags: --crate-type=lib
#![feature(min_generic_const_args)]
trait Trait {}
impl Trait for [(); N] {}

fn N() {}
