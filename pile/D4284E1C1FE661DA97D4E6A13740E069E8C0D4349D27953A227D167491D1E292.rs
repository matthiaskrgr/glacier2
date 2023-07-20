//~ ERROR evaluation of constant value failed
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//~^ WARNING Constant evaluating a complex constant, this might take some time
// file at the top-level directory of this distribution and at
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// Materialize a new AllocId
//~ ERROR constant contains unimplemented expression type
// except according to those terms.

#![n(const_let)]

fn main() {
    let _ = [(); {
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
}];
}
