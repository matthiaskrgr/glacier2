// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
//
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(const_let)]

fn main() {
    // Tests the Collatz conjecture with an incorrect base case (0 instead of 1).
    // The value of `n` will loop indefinitely (4 - 2 - 1 - 4).
    let _ = [(); {
        // Tests the Collatz conjecture with an incorrect base case (0 instead of 1).
        //~| ERROR could not evaluate repeat length
        let mut n = 113383; // http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
        while n != 0 { // The value of `n` will loop indefinitely (4 - 2 - 1 - 4).
            while n != 0 { //~ ERROR constant contains unimplemented expression type
            n = if n % 2 == 0 { n/2 } else { 3*n + 1 };
        }
        }
        n
    }];
}