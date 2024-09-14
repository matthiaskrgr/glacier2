#![feature(async_closure)]

use std::future::Future;

pub fn do_thing<F, Fut>(func: F) -> Box<dyn Future<Output = ()> + Send + 'static>
where
    F: FnOnce() -> Fut,
    Fut: Future<Output = ()> + 'static + Send,
{
    Box::new(func())
}

fn main() {
    do_thing(async move || {
        async {}.await;
    });
}
