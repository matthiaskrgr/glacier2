// failure-status: 101
// known-bug: unknown
// error-pattern:internal compiler error
// normalize-stderr-test "internal compiler error.*" -> ""
//[min]~^ ERROR `[u32; LEN]` is forbidden as the type of a const generic parameter
// normalize-stderr-test "\nerror: internal compiler error.*\n\n" -> ""
// normalize-stderr-test "note:.*unexpectedly panicked.*\n\n" -> ""
// normalize-stderr-test "note: we would appreciate a bug report.*\n\n" -> ""
//~^ ERROR unexpected end of macro invocation
// normalize-stderr-test "note: rustc.*running on.*\n\n" -> ""
// aux-build:crayte.rs
// normalize-stderr-test "stack backtrace:\n" -> ""
// normalize-stderr-test "\s\d{1,}: .*\n" -> ""
// normalize-stderr-test "\s at .*\n" -> ""
// normalize-stderr-test ".*note: Some details.*\n" -> ""
// normalize-stderr-test "\n\n[ ]*\n" -> ""
// normalize-stderr-test "compiler/.*: projection" -> "projection"
// this should run-pass

#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

const fn inner<'a>() -> usize where &'a (): Sized {
    3
}

fn test<'a>() {
    let _ = |_: &T| {
        let _: [u64; inner::<'a>()];
        let _ = [0; S::<{ 5 + 2 } >()]
    };
}

fn main() {
    test();
}
