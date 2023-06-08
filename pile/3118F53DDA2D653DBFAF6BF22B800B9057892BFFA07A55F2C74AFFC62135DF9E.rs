#![deny(single_use_lifetimes)]

fn with<R>(forbid: &fn<'a>(x: &'b u32) -> Quux) -> R {
    //~^ ERROR function pointer types may not have generic parameters
    //~| ERROR lifetime parameter `'a` only used once
    f(&3)
}

fn main() {}
