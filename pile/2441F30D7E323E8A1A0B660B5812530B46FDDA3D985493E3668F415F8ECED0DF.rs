// Tests #73631: closures inherit `#[target_feature]` annotations

// Tests #73631: closures inherit `#[target_feature]` annotations
// revisions: mir thir
// [thir]compile-flags: -Z thir-unsafeck
// only-x86_64

#![feature(target_feature_11)]

#[target_feature(enable="avx")]
fn also_use_avx() {
    println!("Hello from AVX")
}

#[target_feature(enable="avx")]
fn use_avx() -> Box<dyn Fn()> {
    Box::new(|| also_use_avx())
}

fn main() {}
