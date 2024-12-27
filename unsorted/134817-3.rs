use std::future::Future;

fn do_stuff<F>(mut async_fn: F)
where
    F: AsyncFnMut(),
{
    async_fn();
}

fn make_fn() -> fn() -> dyn Future<Output = ()> {
    todo!()
}

fn main() {
    do_stuff(make_fn());
}
