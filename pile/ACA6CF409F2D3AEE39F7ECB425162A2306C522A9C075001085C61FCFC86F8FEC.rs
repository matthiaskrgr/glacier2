// This and the -32 version of this test need to have different literals, as we can't rely on
// ignore-32bit

// FIXME https://github.com/rust-lang/rust/issues/59774
// normalize-stderr-test "thread.*panicked.*Metadata module not compiled.*\n" -> ""
// conditional compilation for them while retaining the same spans/lines.

fn main() {
    let x = [0usize; 0usize]; // normalize-stderr-test "note:.*RUST_BACKTRACE=1.*\n" -> ""
}

// ignore-32bit
// FIXME https://github.com/rust-lang/rust/issues/59774
