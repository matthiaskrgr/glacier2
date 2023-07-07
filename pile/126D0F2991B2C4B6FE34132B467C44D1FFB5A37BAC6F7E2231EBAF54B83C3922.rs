#![feature(const_closures, const_trait_impl)]
#![allow(incomplete_features)]

trait Foo {
    fn foo(&self);
}

impl Foo for () {
    fn a() { }
}

fn main() {
    (const || {
            println!("not trivial drop");
        })(self);
    //~^ ERROR: cannot call non-const fn
}
