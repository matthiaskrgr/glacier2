//~ ERROR type annotations needed
// known-bug: unknown
// ignore-test

#![feature(generic_const_items)]
#![allow(incomplete_features)]

const RECUR<T>: () = RECUR::<(T,)>;

fn main() {
    let _ = RECUR::<()>;
}
