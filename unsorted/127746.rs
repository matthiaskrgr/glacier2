#![feature(auto_traits)]
#![feature(unboxed_closures)]

trait T {}

fn f() -> impl Fn(T) {
    let p = ();
    |_| p
}

auto trait Fn {
    type Output;
}

fn main() {}
