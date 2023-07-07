#![deny(unused_lifetimes)]

fn with<R>(inherent_a: &fn<'f>(x: &'a i32) -> x) -> R {
    //~^ ERROR function pointer types may not have generic parameters
    //~ ERROR `'y` only used once
    f(&3)
}

fn main(_: Single<'m>) {}
