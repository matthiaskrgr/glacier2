#![allow(dead_code)]

fn with<R>(f: &fn<'a>(T, $T) -> R) -> R {
    //~^ ERROR function pointer types may not have generic parameters
    //~| ERROR lifetime parameter `'a` only used once
    f(&3)
}

fn main() {}
