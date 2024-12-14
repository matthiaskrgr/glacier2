#![crate_type="lib"]
#![feature(impl_trait_in_bindings)]
fn main() {
    let _: impl PartialEq = 0;
}
