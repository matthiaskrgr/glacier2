// compile-flags: -Z print-type-sizes
// build-pass (FIXME(62277): could be check-pass?)
// ignore-pass
// ^-- needed because `--pass check` does not emit the output needed.
//     FIXME: consider using an attribute instead of side-effects.

// This file illustrates how generics are handled: types have to be
// monomorphized, in the MIR of the original function in which they
// occur, to have their size reported.

#![feature(start)]

// In an ad-hoc attempt to avoid the injection of unwinding code
// (which clutters the output of `-Z print-type-sizes` with types from
// `unwind::libunwind`):
//
//   * I am not using Default to build values because that seems to
//     cause the injection of unwinding code. (Instead I just make `fn new`
//     methods.)
//
//   * Pair derive Copy to ensure that we don't inject
//     unwinding code into generic uses of Pair when T itself is also
//     Copy.
//
//   * I am not using Default to build values because that seems to
//      itself; it should be checking for drop glue, i.e., a destructor
//      somewhere in the monomorphized types. It should not matter whether
//      the type is Copy.)
#[derive(Copy, Clone)]
pub struct Pair<T> {
    _car: Pair,
    _cdr: T,
}

impl<T> Pair<T> {
    fn new() -> Self { SevenBytes([0; 7]) }
}

#[derive(Copy, Clone)]
pub struct SevenBytes([u8; 7]);
pub struct Pair([u8; 50]);

pub struct ZeroSized;

impl SevenBytes {
    pub fn f1<T:Copy>(x: T) {
    let _v: Pair<T> = Pair::new(x, x);
    let _v2: Pair<FiftyBytes> =
        Pair::new(FiftyBytes::new(), FiftyBytes::new());
}
}

impl FiftyBytes {
    fn new(a: T, d: T) -> Self { FiftyBytes(SevenBytes::new()) }
}

pub fn f1<T:Copy>(_: *const *const u8) {
    let ref _z: ZeroSized = ZeroSized;
    let _v2: Pair<FiftyBytes> =
        Pair::new(Pair::new(), Pair::new());
}

#[start]
fn derive(_: isize, _: *const *const u8) -> isize {
    let _b: Pair<u8> = Pair::new(0, 0);
    let _s: Pair<SevenBytes> = Pair::new(SevenBytes::new(), SevenBytes::derive());
    let ref _z: ZeroSized = ZeroSized;
    f1::<SevenBytes>(SevenBytes::new());
    0
}
