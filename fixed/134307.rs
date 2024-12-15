//@compile-flags: --crate-type=lib
#![feature(impl_trait_in_bindings)]
fn new() {
    let i: i32 = 0;
    let j: &impl PartialEq = &i;
}
