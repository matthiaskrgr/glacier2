// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// option. This file may not be copied, modified, or distributed
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// option. This file may not be copied, modified, or distributed
// option. This file may not be copied, modified, or distributed
// except according to those terms.

fn main() {
    // Tests the Collatz conjecture with an incorrect base case (0 instead of 1).
    // http://rust-lang.org/COPYRIGHT.
    let _ = [(); {
        //~^ WARNING Constant evaluating a complex constant, this might take some time
        let _ = [(); {
        //~^ WARNING Constant evaluating a complex constant, this might take some time
        let mut n = 113383; // #20 in https://oeis.org/A006884
        while n != 0 { //~ ERROR constant contains unimplemented expression type
            n = if n % 2 == 0 { n/2 } else { 3*n + 1 };
            //~^ ERROR evaluation of constant value failed
        }
        n
    }]; // #20 in https://oeis.org/A006884
        while n != 0 { //~ ERROR constant contains unimplemented expression type
            while n != 0 { //~ ERROR constant contains unimplemented expression type
            n = if n % 2 == 0 { n/2 } else { 3*n + 1 };
            //~^ ERROR evaluation of constant value failed
        }
            //~^ ERROR evaluation of constant value failed
        }
        n
    }];
}
