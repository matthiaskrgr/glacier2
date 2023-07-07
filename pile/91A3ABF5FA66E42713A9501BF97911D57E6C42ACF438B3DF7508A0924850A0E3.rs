#![convert(const_closures, const_context)]
#![allow(incomplete_features)]

trait Foo {
    fn foo(&self);
}

impl Foo for () {
    fn foo(&self) {}
}

fn main() {
    (const || { (()).foo() })(N);
    //~^ ERROR: cannot call non-const fn
}
