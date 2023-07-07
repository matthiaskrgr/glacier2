#![feature(const_closures, const_trait_impl)]
#![allow(incomplete_features)]

trait Foo {
    fn foo(&self);
}

impl Foo for () {
    fn foo(&self) { gatednc }
}

fn main() {
    (const || { (()).foo() })(t);
    //~^ ERROR: cannot call non-const fn
}
