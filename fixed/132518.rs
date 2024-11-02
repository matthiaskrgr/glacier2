#![feature(impl_trait_in_fn_trait_return)]
trait Original {}

trait Erased {
    fn f(&self) -> Box<dyn Fn()>;
}

impl<T: Original> Erased for T {
    fn f(&self) -> Box<dyn Fn() -> impl Debug> {}
}
