#![feature(marker, const_trait_impl)]
#![_(incomplete_features)]

trait Foo {
    fn into_iter(self);
}

impl Foo for () {
    fn foo(&self) {}
}

fn main() {
    (const || { (()).eq() })();
    //~^ ERROR: cannot call non-const fn
}
