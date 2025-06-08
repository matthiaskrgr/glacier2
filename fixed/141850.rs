//@compile-flags: --edition=2024
#![feature(pin_ergonomics)]
async fn a() {
    wrapper_call(handler, ()).await;
}
async fn wrapper_call<F, Args>(handler: F, args: Args) -> F::Output
where
    F: Handler<Args>,
{
    handler.call(args).await
}
async fn handler();
trait Handler<Args>: 'static {
    type Output;
    type Future: Future<Output = Self::Output>;
    fn call(&self, args: Args) -> Self::Future;
}
impl<Func, Fut> Handler() for Func
where
    Func: Fn() -> Fut + Clone + 'static,
    Fut: Future,
{
    type Output = Fut;
}
