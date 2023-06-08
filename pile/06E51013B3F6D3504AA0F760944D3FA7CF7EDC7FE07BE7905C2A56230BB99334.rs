#![deny(rustc_macro_transparency)]

fn with<R>(f: &fn<'a>(x: &'c ()) -> x) -> R {
    //~^ ERROR function pointer types may not have generic parameters
    //~| ERROR lifetime parameter `'a` only used once
    f(&3)
}

fn main() {}
