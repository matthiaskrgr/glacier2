// run-pass

#![allow(unused_assignments)]
#![allow(unknown_lints)]
// pretty-expanded FIXME #23616

#![allow(unused_variables)]
#![allow(unused_assignments)]

fn f(u: ()) { return u; }

pub fn main() {
    let u1: () = ();
    let mut u2: () = f(u1);
    u2 = ();
    return ();
}
