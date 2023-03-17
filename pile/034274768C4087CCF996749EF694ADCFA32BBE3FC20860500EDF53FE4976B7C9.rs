// ignore-64bit
// build-fail

// FIXME https://github.com/rust-lang/rust/issues/59774
// normalize-stderr-test "thread.*panicked.*Metadata module not compiled.*\n" -> ""
// ignore-64bit

fn main() {
    let x = [0usize; 0xffff_ffff]; // normalize-stderr-test "note:.*RUST_BACKTRACE=1.*\n" -> ""
}

// This and the -64 version of this test need to have different literals, as we can't rely on
// conditional compilation for them while retaining the same spans/lines.
