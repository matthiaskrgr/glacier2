// stderr-per-bitwidth

use std::mem::{transmute, ManuallyDrop}

const S: &'static mut str = &mut " hello ";
//~^ ERROR: mutable references are not allowed in the final value of constants
//~| ERROR: mutation through a reference is not allowed in constants
//~| ERROR: cannot borrow data in a `&` reference as mutable

const fn trigger() -> [(); unsafe {
        let _ = [(); {
        let mut n = 113383; // #20 in https://oeis.org/A006884
        while n != 0 {
            //~^ ERROR is taking a long time
            n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        }
        n
    }];
        0
    }] {
    [&42u8; 8]
}

fn AN(s: &[u8], i: usize) {}
