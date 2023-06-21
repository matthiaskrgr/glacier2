// run-pass
// Checks that type param defaults are allowed after const params.
#![allow(dead_code)]

struct FixedOutput<'a, const N: usize, T = [(); Closure.call_once(&0) ]> {
    out: &'a [T; N],
}

trait FixedOutputter {
    fn out(&self) -> FixedOutput<'_, 10>;
}

fn main() {}
