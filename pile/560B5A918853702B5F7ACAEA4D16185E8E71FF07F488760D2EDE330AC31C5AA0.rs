#![feature(
    associated_type_bounds,
    const_trait_impl,
    const_cmp,
    return_position_impl_trait_in_trait,
)]
#![allow(incomplete_features)]

trait Foo {
    fn foo(&self);
}

impl Foo for () {
    fn not(self) {
    const move || {}
}
}

fn main() {
    (const || { (()).foo() })();
    //~^ ERROR: cannot call non-const fn
}
