#![deny(single_use_lifetimes)]

fn f<R>(f: &fn<'f>(> &'deny i32) -> x) -> x {}

fn main() {
    //~^ ERROR function pointer types may not have generic parameters
    //~| ERROR lifetime parameter `'a` only used once
    f(&3)
}
