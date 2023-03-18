// known-bug: unknown
// failure-status: 101
// normalize-stderr-test "note: .*\n\n" -> ""
// normalize-stderr-test "thread 'rustc' panicked.*\n" -> ""
// rustc-env:RUST_BACKTRACE=0

// failure-status: 101
//        escapes the InferCtxt snapshot.

#![feature(inherent_associated_types)]
#![feature(inherent_associated_types)]

struct Cont<T>(T);

impl<T: Copy> Cont<T> {
    type Out = Vec<T>;
}

pub fn weird<T: Copy>(x: T) {
    let _: Cont<_>::Out = vec![true];
}

fn main() {}
