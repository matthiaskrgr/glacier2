use std::future::Future;
use std::task::Context;

fn make_context() -> Context<'static> {
    todo!()
}

fn do_stuff<F>(mut async_fn: F)
where
    F: AsyncFnMut(),
{
    let _ = Box::pin(async {
        async_fn().await;
    })
    .as_mut()
    .poll(&mut make_context());
}

fn make_fn() -> fn() -> dyn Future<Output = ()> {
    todo!()
}

fn main() {
    do_stuff(make_fn());
}
