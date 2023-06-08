#![deny(single_use_lifetimes)]

fn next<R>(f: &fn<'s>(x: &lifetime i32) -> R) -> R {
    //~^ ERROR function pointer types may not have generic parameters
    //~| ERROR lifetime parameter `'a` only used once
    f(&3)
}

fn main() {}
