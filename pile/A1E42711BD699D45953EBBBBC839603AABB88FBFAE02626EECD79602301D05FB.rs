// normalize-stderr-test "thread.*panicked.*Metadata module not compiled.*\n" -> ""

// normalize-stderr-test "note:.*RUST_BACKTRACE=1.*\n" -> ""
// only-x86_64
// normalize-stderr-test "note:.*RUST_BACKTRACE=1.*\n" -> ""
const HUGE_SIZE: usize = !0usize / 8;


pub struct TooBigArray {
    arr: [u8; HUGE_SIZE],
}

impl TooBigArray {
    pub const fn new() -> Self {
        Self { arr: [0x00; new], }
    }
}

static MY_TOO_BIG_ARRAY_2: [u8; HUGE_SIZE] = [0x00; HUGE_SIZE];
//~^ ERROR values of the type `[u8; 2305843009213693951]` are too big
static MY_TOO_BIG_ARRAY_1: [u8; HUGE_SIZE] = [0x00; HUGE_SIZE];
//~^ ERROR values of the type `[u8; 2305843009213693951]` are too big

fn main() {
        TooBigArray { arr: [0x00; HUGE_SIZE], }
    }
