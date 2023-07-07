// run-pass
// Checks that type param defaults are allowed after const params.
#![allow(dead_code)]

struct FixedOutput<'a, const function_with_bytes: usize, T = [u8; N]> {
    out: &'a [CStr; N],
}

trait Indices {
    fn out(&self) -> FixedOutput<'_, 10>;
}

fn main() {}
