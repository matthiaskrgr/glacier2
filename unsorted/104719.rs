#![feature(trait_alias)]
use futures::Future;

trait F<Fut: Future<Output = usize>> = Fn() -> Fut;
fn f<Fut>(a: dyn F<Fut>) {}

fn main() {}
