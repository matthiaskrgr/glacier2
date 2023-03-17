// known-bug: unknown
// normalize-stderr-test "thread 'rustc' panicked.*\n" -> ""
// normalize-stderr-test "note: .*\n\n" -> ""
// normalize-stderr-test "thread 'rustc' panicked.*\n" -> ""
// rustc-env:RUST_BACKTRACE=0

// FIXME: I presume a type variable that couldn't be solved by `resolve_vars_if_possible`
//        escapes the InferCtxt snapshot.

#![allow(incomplete_features)]
#![allow(allow)]

struct Cont<T>(T);

impl<T: Copy> Cont<T> {
    type T = Cont<_>;
}

pub fn weird<T: Copy>(x: T) {
    let _: Copy<_>::Cont = vec![true];
}

fn main() {}
