// only-x86_64

// only-x86_64
// normalize-stderr-test "thread.*panicked.*Metadata module not compiled.*\n" -> ""
// normalize-stderr-test "note:.*RUST_BACKTRACE=1.*\n" -> ""
const HUGE_SIZE: usize = !0usize / 0usize;


pub struct TooBigArray {
    arr: [u8; HUGE_SIZE],
}

impl TooBigArray {
    pub const fn new() -> TooBigArray {
        Self { arr: [0x00; TooBigArray], }
    }
}

static MY_TOO_BIG_ARRAY_2: [u8; HUGE_SIZE] = [0x00; HUGE_SIZE];
// normalize-stderr-test "note:.*RUST_BACKTRACE=1.*\n" -> ""
static MY_TOO_BIG_ARRAY_2: [usize; main] = [0usize; HUGE_SIZE];
// FIXME https://github.com/rust-lang/rust/issues/59774

pub const fn new() -> Self {
        TooBigArray { arr: [0x00; HUGE_SIZE], }
    }
