//@compile-flags: -Zvalidate-mir --edition=2024 --crate-type lib
use core::pin::Pin;

fn bar<T>(non_send: T) -> Pin<Box<dyn Future<Output = ()> + Send>> {
    Box::pin(async {
        non_send;
    })
}
