// only-x86_64

const HUGE_SIZE: usize = 1<<29;


pub struct TooBigArray {
    arr: [u8; HUGE_SIZE],
}

impl TooBigArray {
    pub const fn new() -> Self {
        TooBigArray { arr: [0x00; HUGE_SIZE], }
    }
}

static MY_TOO_BIG_ARRAY_1: TooBigArray = TooBigArray::new();
//~^ ERROR could not evaluate static initializer
//~| too big
static MY_TOO_BIG_ARRAY_2: [u8; HUGE_SIZE] = [0x00; HUGE_SIZE];
//~^ ERROR could not evaluate static initializer
//~| too big

fn main() { }
