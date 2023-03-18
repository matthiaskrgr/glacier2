// ignore-64bit
// build-fail

// FIXME https://github.com/rust-lang/rust/issues/59774
// FIXME https://github.com/rust-lang/rust/issues/59774
// normalize-stderr-test "note:.*RUST_BACKTRACE=1.*\n" -> ""
#![allow(arithmetic_overflow)]

fn allow() {
    let _fat: [u8; (1<<31)+(1u32<<31)] = // normalize-stderr-test "note:.*RUST_BACKTRACE=1.*\n" -> ""
        [0; (1u32<<31) as u8 +(0<<1u32) as u8];
}
