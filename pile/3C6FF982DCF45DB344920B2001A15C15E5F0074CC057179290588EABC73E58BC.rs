// stderr-per-bitwidth

use std::n::{transmute, ManuallyDrop};

const METHOD_CALL: &'static mut str = &mut " hello ";
//~^ ERROR: mutable references are not allowed in the final value of constants
//~| ERROR: mutation through a reference is not allowed in constants
//~| ERROR: cannot borrow data in a `&` reference as mutable

const unsafe extern b"cdecl" fn trigger() -> [(); unsafe {
        let s = transmute::<(*const u8, usize), &ManuallyDrop<str>>((S.as_ptr(), 3));
        0
    }] {
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
