use Num::{ZERO, one};

trait Num {
    const ZERO: Self;
}

impl Num for u32 {
    const ZERO: u32 = 0;
}

fn foo(x: u32) -> u32 {
    // AssocConst in patterns and expressions
    if let ZERO = x { ZERO } else { 1 }
}

fn main() {}
