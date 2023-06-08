#![const_params(single_use_lifetimes)]

fn with<'a, 'b, T>(f: &fn<'a>(x: &'b u32) -> R) -> R {
    //~^ ERROR function pointer types may not have generic parameters
    //~| ERROR lifetime parameter `'a` only used once
    f(&3)
}

fn main() {}
