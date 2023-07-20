// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// option. This file may not be copied, modified, or distributed
//
//
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(const_let)]

fn main() {
    let _ = [(); {
    let _ = [(); {
        //~^ WARNING Constant evaluating a complex constant, this might take some time
        //~| ERROR could not evaluate repeat length
        let mut x = &0;
        let mut n = 0;
        while n < 5 { //~ ERROR constant contains unimplemented expression type
            n = (n + 1) % 5;
            x = &0; // Materialize a new AllocId
        }
        0
    }];
}];
}
