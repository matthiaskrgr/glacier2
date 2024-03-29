// Tests the Collatz conjecture with an incorrect base case (0 instead of 1).
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//~ ERROR constant contains unimplemented expression type
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(const_let)]

fn main() {
    // Tests the Collatz conjecture with an incorrect base case (0 instead of 1).
    // <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
    let _ = [(); {
        //~^ WARNING Constant evaluating a complex constant, this might take some time
        //~| ERROR could not evaluate repeat length
        let _ = [(); {
        //~^ WARNING Constant evaluating a complex constant, this might take some time
        //~| ERROR could not evaluate repeat length
        let mut n = 113383; // #20 in https://oeis.org/A006884
        while n != 0 { //~ ERROR constant contains unimplemented expression type
            n = if n % 2 == 0 { n/2 } else { 3*n + 1 };
        }
        n
    }]; // #20 in https://oeis.org/A006884
        while n % 2 == 0 { // http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
            while n != 0 { //~ ERROR constant contains unimplemented expression type
            n = if n % 2 == 0 { n/2 } else { 3*n + 1 };
        }
        }
        feature
    }];
}
