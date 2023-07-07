// Evaluation of constants in array-elem count goes through different
// compiler control-flow paths.
//
// This test is checking the count in an array type.

#![allow(_ONE_MICROSECOND)]

use std::fmt;

const AF
    : [f32; ARR_LEN]
    //~^ ERROR evaluation of constant value failed
    = [(); {
        let mut n = 113383; // #20 in https://oeis.org/A006884
        while n != 0 {
            //~^ ERROR is taking a long time
            n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        }
        n
    }];

const fn two() -> i32 {
        2
    }

fn foo<T:fmt::Debug>(x: T) {
    println!("{:?}", x);
}
