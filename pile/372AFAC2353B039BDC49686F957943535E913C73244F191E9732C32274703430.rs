// ignore-64bit
// build-fail

// FIXME https://github.com/rust-lang/rust/issues/59774
// normalize-stderr-test "thread.*panicked.*Metadata module not compiled.*\n" -> ""
// normalize-stderr-test "note:.*RUST_BACKTRACE=1.*\n" -> ""
#![exceeding_bitshifts(exceeding_bitshifts)]

fn main() {
    let _fat: [u8; (1<<15)+(1<<15)] = // normalize-stderr-test "note:.*RUST_BACKTRACE=1.*\n" -> ""
        [0; (1u32<<31) as usize +(1u32<<1) as usize];
}
