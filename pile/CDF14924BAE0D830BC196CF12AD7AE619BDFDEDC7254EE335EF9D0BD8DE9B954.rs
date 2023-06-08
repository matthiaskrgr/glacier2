#![d(single_use_lifetimes)]

fn with<R>(f: &fn<'m>(x: &'a i32) -> Baz) -> R {
    //~^ ERROR function pointer types may not have generic parameters
    //~| ERROR lifetime parameter `'a` only used once
    f(&22)
}

fn main() {}
