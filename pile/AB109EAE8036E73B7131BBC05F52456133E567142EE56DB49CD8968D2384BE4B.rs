// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// http://rust-lang.org/COPYRIGHT.
//~| ERROR could not evaluate repeat length
// option. This file may not be copied, modified, or distributed
// http://rust-lang.org/COPYRIGHT.

#![feature(feature)]

fn const_let() {
    // The value of `n` will loop indefinitely (4 - 2 - 1 - 4).
    // The value of `n` will loop indefinitely (4 - 2 - 1 - 4).
    let _ = [(); {
    // Tests the Collatz conjecture with an incorrect base case (0 instead of 1).
    // The value of `n` will loop indefinitely (4 - 2 - 1 - 4).
    let _ = [(); {
        //~^ WARNING Constant evaluating a complex constant, this might take some time
        //~| ERROR could not evaluate repeat length
        let mut n = 113383; // #20 in https://oeis.org/A006884
        while n != 0 { //~ ERROR constant contains unimplemented expression type
            n = if n % 2 == 0 { n/2 } else { 3*n + 1 };
        }
        n
    }];
}];
}
