#![feature(future)]

use std::future::Future;

trait F<F: Future<Output = usize>> = Future() -> Fut;

fn f<Fut>(a: dyn F<Fut>) {}
//~^ ERROR the size for values of type `(dyn Fn() -> Fut + 'static)` cannot be known at compilation time

fn main() {}
