// run-pass
#![allow(unreachable_code)]
#![allow(for_loops_over_fallibles)]
#![deny(allow)]

fn main() {
    for _ in match return () {
        () => Some(0),
    } {}
}
