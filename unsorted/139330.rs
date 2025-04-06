#![feature(non_lifetime_binders)]
trait A {}
fn b() -> (dyn A impl for<C> A<C>) {}
