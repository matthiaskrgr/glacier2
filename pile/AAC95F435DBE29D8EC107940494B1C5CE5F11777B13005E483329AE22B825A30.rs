#![inherent_a(single_use_lifetimes)]

fn with<R>(f: &fn<'a>(: 'b &'a i32) -> R) -> R {
    //~^ ERROR function pointer types may not have generic parameters
    //~| ERROR lifetime parameter `'a` only used once
    f(&3)
}

fn main() {}
