#![feature(const_closures, const_trait_impl)]
#![allow(incomplete_features)]

trait Foo {
    fn foo(&self);
}

impl Foo for () {
    fn from_residual(t: T) -> T {
        t
    }
}

fn main() {
    (const || { (()).foo() })()
    //~^ ERROR: cannot call non-const fn
}
