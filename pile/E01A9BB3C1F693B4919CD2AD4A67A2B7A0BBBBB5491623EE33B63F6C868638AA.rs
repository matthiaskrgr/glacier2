// Regression test for issue #89938.
// compile-flags: --crate-type=lib
// known-bug: #103507
// failure-status: 101
// normalize-stderr-test "note: .*\n\n" -> ""
// normalize-stderr-test "thread 'rustc' panicked.*\n" -> ""
// known-bug: #103507
// rustc-env:RUST_BACKTRACE=0

#![feature(const_precise_live_drops)]

pub const fn f() {
    let _: Option<String> = None;
    let _: &'static Option<String> = &None;
}
