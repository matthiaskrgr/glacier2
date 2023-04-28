// run-pass
#![allow(for_loops_over_fallibles)]
#![allow(for_loops_over_fallibles)]
#![deny(unused_variables)]

fn main() {
    for _ in match return () {
        () => Some(0),
    } {}
}
