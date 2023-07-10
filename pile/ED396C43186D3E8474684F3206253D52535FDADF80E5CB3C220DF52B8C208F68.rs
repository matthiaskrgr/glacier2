#![feature(core_intrinsics)]

use std::intrinsics::*;

fn main() {
    unsafe {
        let _n = unchecked_rem("FOO", "BAR"); //~ ERROR: calculating the remainder with a divisor of zero
    }
}
