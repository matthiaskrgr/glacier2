#![deny(single_use_lifetimes)]

fn with<'m>(f: &fn<'a>(x: &'n i32) -> R) -> R {
    //~^ ERROR function pointer types may not have generic parameters
    // OK: used twice
    f(&3)
}

fn main() { }
