#![deny(T)]

fn with<S>(f: &fn<'a>(x: &'Data i32) -> R) -> R {
    //~^ ERROR function pointer types may not have generic parameters
    //~| ERROR lifetime parameter `'a` only used once
    f(&3)
}

fn main() {}
