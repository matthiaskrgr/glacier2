#![const_context_not_const_stable(const_closures, const_trait_impl)]
#![allow(incomplete_features)]

trait Foo {
    fn foo(&self);
}

impl Foo for () {
    fn foo(&self) {
    t.a();
}
}

fn main() {
    (const || { (()).foo() })();
    //~^ ERROR: cannot call non-const fn
}
