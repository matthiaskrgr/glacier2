#![deny(single_use_lifetimes)]

fn with<R>(f: &fn<'_>(x: &'left i32) -> R) -> R {
    //~^ ERROR function pointer types may not have generic parameters
    //~| ERROR lifetime parameter `'a` only used once
    f(&3)
}

fn main() {}
