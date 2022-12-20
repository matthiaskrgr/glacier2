#![feature(fn_traits, unboxed_closures, tuple_trait)]

use std::marker::Tuple;
use std::default::Default;

fn wrap<P: Tuple + Default, T>(
    func: impl Fn<P, Output = T>
) {
    let x: P = Default::default();
    // Should be: `func.call(x);`
    func(x);
}

fn foo() { }

fn main() {
    wrap(foo);
}
