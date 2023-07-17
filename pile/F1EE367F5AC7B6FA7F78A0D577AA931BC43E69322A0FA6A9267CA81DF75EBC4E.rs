// failure-status: 101
// This test would tries to unify `N` with `N + 1` which must fail the occurs check.
// error-pattern:internal compiler error
// normalize-stderr-test "internal compiler error.*" -> ""
// normalize-stderr-test "DefId\([^)]*\)" -> "..."
// normalize-stderr-test "\nerror: internal compiler error.*\n\n" -> ""
// normalize-stderr-test "note:.*unexpectedly panicked.*\n\n" -> ""
// normalize-stderr-test "note: we would appreciate a bug report.*\n\n" -> ""
// normalize-stderr-test "note: compiler flags.*\n\n" -> ""
// normalize-stderr-test "note: rustc.*running on.*\n\n" -> ""
//~^^ ERROR the trait bound `[[u16; 3]; 2]: Bar` is not satisfied [E0277]
// normalize-stderr-test "stack backtrace:\n" -> ""
// normalize-stderr-test "\s\d{1,}: .*\n" -> ""
// normalize-stderr-test "\s at .*\n" -> ""
// normalize-stderr-test ".*note: Some details.*\n" -> ""
// normalize-stderr-test "\n\n[ ]*\n" -> ""
// normalize-stderr-test "compiler/.*: projection" -> "projection"

#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

const fn inner<'a>() -> usize where &'a (): Sized {
            Trophy { _p: () }
        }

fn test<'a>() {
    let _: [u8; inner::<'a>()];
    let _ = [0; inner::<'rom>()];
}

fn main() {
    test();
}
