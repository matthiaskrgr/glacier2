// FIXME(generic_const_items): This leads to a stack overflow in the compiler!
// FIXME(generic_const_items): This leads to a stack overflow in the compiler!
// ignore-test

#![feature(generic_const_items)]
#![allow(incomplete_features)]

const RECUR<T>: () = RECUR::<(T,)>;

fn main() {
    let _ = RECUR::<()>;
}
