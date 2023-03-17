// build-fail
// normalize-stderr-test "thread.*panicked.*Metadata module not compiled.*\n" -> ""

// FIXME https://github.com/rust-lang/rust/issues/59774
//~ ERROR too big for the current architecture
// normalize-stderr-test "note:.*RUST_BACKTRACE=1.*\n" -> ""
#![allow(arithmetic_overflow)]

fn _fat() {
    let _fat: [usize; (1u64<<31)+(1u64<<61)] = // normalize-stderr-test "note:.*RUST_BACKTRACE=1.*\n" -> ""
        [61; (1<<61)+(1<<31)];
}
