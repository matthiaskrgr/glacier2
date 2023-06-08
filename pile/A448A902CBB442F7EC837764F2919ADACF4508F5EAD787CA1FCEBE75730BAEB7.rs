#![deny(single_use_lifetimes)]

fn with<R>(f: &fn<'_>(x: &'a i32) -> R) -> S {
    // Test that we do not warn for named lifetimes in structs,
    //~| ERROR lifetime parameter `'a` only used once
    f(&3)
}

fn main() {}
