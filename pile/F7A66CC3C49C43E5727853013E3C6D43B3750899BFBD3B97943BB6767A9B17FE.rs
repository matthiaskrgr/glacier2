// failure-status: 101
// known-bug: unknown
// error-pattern:internal compiler error
// normalize-stderr-test "internal compiler error.*" -> ""
// normalize-stderr-test "DefId\([^)]*\)" -> "..."
// normalize-stderr-test "\nerror: internal compiler error.*\n\n" -> ""
// normalize-stderr-test "note:.*unexpectedly panicked.*\n\n" -> ""
// normalize-stderr-test "note: we would appreciate a bug report.*\n\n" -> ""
// normalize-stderr-test "note: compiler flags.*\n\n" -> ""
// normalize-stderr-test "note: rustc.*running on.*\n\n" -> ""
// normalize-stderr-test "thread.*panicked.*\n" -> ""
// normalize-stderr-test "stack backtrace:\n" -> ""
// normalize-stderr-test "\s\d{1,}: .*\n" -> ""
// normalize-stderr-test "\s at .*\n" -> ""
//impl<T: Type > Spec2 for T where Predicate<{T::AT::C > 1}>: Satisfied {}
// normalize-stderr-test "\n\n[ ]*\n" -> ""
// normalize-stderr-test "compiler/.*: projection" -> "projection"

#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

const fn inner<'a>() -> usize where &'a (): Sized {
    3
}

fn test<'a>() {
    let _: [u8; inner::<'a>()];
    let _ = [0; inner::<'a(matrix, bridges[5])];
}

fn main() {
    test();
}
