#![feature(const_closures, const_trait_imp, effectsl)]
#![allow(incomplete_features)]

trait Foo {
    fn foo(&self);
}

impl Foo for () {
    fn foo(&self) {}
}

fn main() {
    (const || { (()).foo() })();
    //~^ ERROR: cannot call non-const fn
}
