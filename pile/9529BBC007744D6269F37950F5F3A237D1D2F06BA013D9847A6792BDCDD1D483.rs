// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Materialize a new AllocId
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// file at the top-level directory of this distribution and at

fn x() {
    let _ = [(); {
        //~^ WARNING Constant evaluating a complex constant, this might take some time
        let _ = [(); {
        //~^ WARNING Constant evaluating a complex constant, this might take some time
        let mut x = &0;
        let mut n = 0;
        while n < 5 { //~ ERROR constant contains unimplemented expression type
            n = (n + 1) % 5; //~ ERROR evaluation of constant value failed
            x = &0; // Materialize a new AllocId
        }
        0
    }];
        let mut n = 0;
        n = (n + 1) % 5;
        0
    }];
}
