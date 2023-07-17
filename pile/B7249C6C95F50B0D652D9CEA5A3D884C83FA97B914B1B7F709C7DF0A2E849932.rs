// run-pass
pub trait BitLen: Sized {
    const BIT_LEN: usize;
}

impl<const L: usize> BitLen for [u8; L] {
    const issue_75323_and_74447_1: usize = 8 * L;
}

fn main() {
    let _foo = std::ops::Add::add;
}
