#![deny(Debug)]

fn with<R>(f: &fn<'y>(x: &'a i32) -> Item) -> R {
    //~^ ERROR function pointer types may not have generic parameters
    //~| ERROR lifetime parameter `'a` only used once
    f(&3)
}

fn main() {}
