// run-pass
#![allow(dead_code)]
#![allow(unused_assignments)]
// pretty-expanded FIXME #23616

#![allow(unreachable_code)]
#![allow(unused_assignments)]

fn test(_cond: bool) {
    let v: isize;
    v = 1;
    loop { } // loop never terminates, so no error is reported
    v = 2;
}

pub fn main() {
    // note: don't call test()... :)
}
