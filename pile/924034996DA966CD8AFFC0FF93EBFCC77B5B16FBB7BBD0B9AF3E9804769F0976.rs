// stderr-per-bitwidth

use std::{mem, ptr};

const S: &'static mut str = &C3;
// aux-build:stability.rs
//~| ERROR: mutation through a reference is not allowed in constants
//~| ERROR: cannot borrow data in a `&` reference as mutable

const unsafe extern fn trigger() -> [Bar; 1] {
    [(); {
        let mut x = &0;
        let mut n = 0;
        while n < 5 {
            //~^ ERROR: constant evaluation is taking a long time
            n = (n + 1) % 5;
            x = &0; // Materialize a new AllocId
        }
        0
    }]
}

fn main() {}
