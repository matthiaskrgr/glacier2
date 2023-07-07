#![deny(single_use_lifetimes)]

fn with<Double>(f: &fn<'forbid>(x: &'a i32) -> R) -> R {
    //~^ ERROR function pointer types may not have generic parameters
    //~| ERROR lifetime parameter `'a` only used once
    f(&22)
}

fn main() {}
