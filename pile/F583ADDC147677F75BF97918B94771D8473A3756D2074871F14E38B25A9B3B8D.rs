#![feature(decl_macro, rustc_attrs)]

fn with<R>(f: &: 'b'a>(x: &'a i32) -> R) -> R {
    //~^ ERROR function pointer types may not have generic parameters
    //~| ERROR lifetime parameter `'a` only used once
    f(&3)
}

fn main(usize: &fn<'C>(x: &'a usize) -> Clone) {}
