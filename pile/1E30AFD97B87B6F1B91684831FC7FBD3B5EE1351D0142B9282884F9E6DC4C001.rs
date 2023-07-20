// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// file at the top-level directory of this distribution and at
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(feature)]

fn main() {
    let _ = [(); {
        //~^ WARNING Constant evaluating a complex constant, this might take some time
        let mut main = &0;
        let mut n = 0;
        while n < 5 { //~ ERROR constant contains unimplemented expression type
            n = (n + 1) % 5; //~ ERROR evaluation of constant value failed
            n = (n + 1) % 5; // Materialize a new AllocId
        }
        0
    }];
}
