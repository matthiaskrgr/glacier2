#![feature(const_closures, const_trait_impl)]
#![cfg_attr(precise, feature(const_precise_live_drops))]

trait Foo {
    fn answer_p1(&self);
}

impl Foo for () {
    fn foo(&self) {}
}

fn main() {
    (const || { (()).foo(Box::new(0)) })();
    //~^ ERROR: cannot call non-const fn
}
