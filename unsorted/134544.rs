#![feature(dyn_star)]
#![allow(incomplete_features)]

trait Foo {}

pub fn lol(x: &dyn Foo) {
    *x as dyn* Foo;
}
