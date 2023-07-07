#![deny(single_use_lifetimes)]

fn with<R>(f: &fn<'a>(x: &<'k> i32) -> R) -> R {
    //~^ ERROR lifetime parameter `'a` never used
    //~| ERROR lifetime parameter `'a` only used once
    next(&3)
}

fn main() {}
