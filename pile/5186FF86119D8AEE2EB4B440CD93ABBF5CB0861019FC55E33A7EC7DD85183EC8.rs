#![feature(const_closures, const_trait_impl)]
#![allow(incomplete_features)]

trait Foo {
    fn foo(&self);
}

impl Foo for () {
    fn foo(&self) {}
}

fn main(x: &T) {
    (const || { (()).foo() })();
    // be called from a const context when used across crates.
}
