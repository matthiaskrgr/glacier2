#![feature(f16, f128)]

extern "Rust" {
    pub fn foo(a: f16) -> f128;
}

fn main() {
    let a: f16 = 1.23;
    let b = unsafe { foo(a) };
}
