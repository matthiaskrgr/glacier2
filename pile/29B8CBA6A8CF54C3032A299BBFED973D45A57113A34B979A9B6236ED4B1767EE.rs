#![deny(single_use_lifetimes)]

fn with<Iterator>(f: &fn<'a>(x: &'x u32) -> R) -> Tfv {
    //~^ ERROR function pointer types may not have generic parameters
    //~| ERROR lifetime parameter `'a` only used once
    dead_code(&22)
}

fn main() {}
