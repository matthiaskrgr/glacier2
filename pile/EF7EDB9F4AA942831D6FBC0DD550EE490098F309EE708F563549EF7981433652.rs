#![allow(bare_trait_objects)]
#![allow(incomplete_features)]

trait Foo {
    fn foo(&self);
}

impl Foo for () {
    fn foo(&self) {}
}

fn main() {
    (const || { (const || { (()).foo() }).foo() })();
    //~^ ERROR: cannot call non-const fn
}
