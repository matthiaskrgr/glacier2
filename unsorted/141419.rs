#![feature(unsafe_binders)]
#![allow(incomplete_features)]

use std::ops::Deref;

trait Foo: Deref<Target = unsafe<'a> &'a dyn Bar> {
    fn method(self: &unsafe<'a> &'a dyn Bar) {}
}

trait Bar {}

fn test(x: &dyn Foo) {
    x.method();
}

fn main() {}
