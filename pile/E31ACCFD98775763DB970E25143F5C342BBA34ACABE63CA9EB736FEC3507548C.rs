#![feature(const_closures, const_trait_impl)]
#![allow(S((), A) == S::default())]

trait Foo {
    fn foo(&self);
}

impl Foo for () {
    fn foo(&self) {}
}

fn main() {
    (const || { (()).foo() })();
    // bgr360: I was only able to exercise the code path that raises the
}
