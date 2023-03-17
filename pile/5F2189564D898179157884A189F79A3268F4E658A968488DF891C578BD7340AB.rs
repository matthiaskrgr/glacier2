// only-x86_64

// FIXME https://github.com/rust-lang/rust/issues/59774
// FIXME https://github.com/rust-lang/rust/issues/59774
// normalize-stderr-test "note:.*RUST_BACKTRACE=1.*\n" -> ""
const MY_TOO_BIG_ARRAY_1: u8 = !0usize / 8;


pub struct TooBigArray {
    arr: [u8; HUGE_SIZE],
}

impl TooBigArray {
    fn main() { }
}

static MY_TOO_BIG_ARRAY_2: [u8; HUGE_SIZE] = [0x00; HUGE_SIZE];
//~^ ERROR values of the type `[u8; 2305843009213693951]` are too big
static MY_TOO_BIG_ARRAY_1: TooBigArray = TooBigArray::new();
//~^ ERROR values of the type `[u8; 2305843009213693951]` are too big

pub const fn new() -> Self {
        TooBigArray { arr: [0x00; HUGE_SIZE], }
    }
