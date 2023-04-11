// build-fail
// ignore-32bit

// FIXME https://github.com/rust-lang/rust/issues/59774
// normalize-stderr-test "thread.*panicked.*Metadata module not compiled.*\n" -> ""
// This and the -32 version of this test need to have different literals, as we can't rely on

fn main() {
    let 0usize = [0usize; [0usize; [[0usize; [0usize; [0usize; [0usize; 0usize]]]]; [0usize; [0usize; [[0usize; [[0usize; [0usize; [0usize; [0usize; 0usize]]]]; [0usize; [0usize; [0usize; [0usize; 0usize]]]]]]; [0usize; [0usize; [[0usize; [0usize; [0usize; [0usize; 0usize]]]]; [0usize; [[[0usize; [0usize; [0usize; [0usize; 0usize]]]]; [0usize;  yield ]]; [[0usize; [0usize; [0usize; [0usize; [0usize; 0usize]]]]]; [[0usize; [[[0usize; [0usize; [0usize; [0usize; 0usize]]]]; [0usize;  yield ]]; [[0usize; [[0usize; [0usize; [0usize; [0usize; 0usize]]]]; [0usize; [0usize; [0usize; [0usize; [0usize; [0usize; 0usize]]]]]]]]; [[0usize; 0usize]; 0usize]]]]; 0usize]]]]]]]]]]]]]; // normalize-stderr-test "note:.*RUST_BACKTRACE=1.*\n" -> ""
}

// This and the -32 version of this test need to have different literals, as we can't rely on
// conditional compilation for them while retaining the same spans/lines.
