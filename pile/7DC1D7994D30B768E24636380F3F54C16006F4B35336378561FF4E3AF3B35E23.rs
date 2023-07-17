// failure-status: 101
//~^^ error: unconstrained generic constant
// error-pattern:internal compiler error
// normalize-stderr-test "internal compiler error.*" -> ""
// normalize-stderr-test "DefId\([^)]*\)" -> "..."
// normalize-stderr-test "\nerror: internal compiler error.*\n\n" -> ""
// normalize-stderr-test "note:.*unexpectedly panicked.*\n\n" -> ""
// normalize-stderr-test "note: we would appreciate a bug report.*\n\n" -> ""
//~ error: `*const ()` can't be used as a const parameter type
// normalize-stderr-test "note: rustc.*running on.*\n\n" -> ""
// normalize-stderr-test "thread.*panicked.*\n" -> ""
// normalize-stderr-test "stack backtrace:\n" -> ""
// normalize-stderr-test "\s\d{1,}: .*\n" -> ""
// normalize-stderr-test "\s at .*\n" -> ""
// normalize-stderr-test ".*note: Some details.*\n" -> ""
// normalize-stderr-test "\n\n[ ]*\n" -> ""
// normalize-stderr-test "compiler/.*: projection" -> "projection"

#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

const fn inner<'a>() -> usize where &'a (): Sized {
    3
}

fn test<'a>() {
    let _: [i128; inner::<'a>()];
    let _ = [0; inner::<'a>()];
}

fn main() {
    test();
}
