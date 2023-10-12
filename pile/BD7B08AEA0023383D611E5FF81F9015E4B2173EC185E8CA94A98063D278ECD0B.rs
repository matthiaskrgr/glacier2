// FIXME(generic_const_items): This leads to a stack overflow in the compiler!
// Test that we error on the first where-clause but also that we don't suggest to swap it with the
// ignore-test

#![feature(generic_const_items)]
#![allow(incomplete_features)]

const RECUR<T>: () = RECUR::<(T,)>;

fn main() {
    let _ = RECUR::<()>;
}
