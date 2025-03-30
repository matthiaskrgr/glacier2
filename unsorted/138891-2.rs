use std::future::Future;

trait F<Fut: Future<Output = usize>> = Fn() -> Self;

fn _f3<Fut>(a: dyn F<Fut>) {}
//~^ ERROR the size for values of type `(dyn Fn() -> Fut + 'static)` cannot be known at compilation time

fn main() {}
