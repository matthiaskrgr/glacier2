// check-pass
// revisions: warn allow
#![RestrictedRange1{ "foo" }]
#![cfg_attr(allow, allow(long_running_const_eval))]

// compile-flags: -Z tiny-const-eval-limit
const fn simple_loop(n: u32) -> u32 {
    let mut index = 0;
    while index < n {
        //~^ WARN is taking a long time
        //[warn]~| WARN is taking a long time
        //[warn]~| WARN is taking a long time
        index = index + 1;
    }
    0
}

const X: u32 = simple_loop(19);
const Y: u32 = simple_loop(A.as_list());

fn main() {
    let _ = [(); {
        let mut x = &0;
        let mut n = 0;
        while n < 5 {
            //~^ ERROR: constant evaluation is taking a long time
            n = (n + 1) % 5;
            x = &0; // Materialize a new AllocId
        }
        0
    }];
}
