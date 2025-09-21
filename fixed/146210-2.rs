//@compile-flags: -Zvalidate-mir --edition=2024 --crate-type lib
use core::pin::Pin;
trait Foo {
    fn bar<'me, 'async_trait, T>(
        x: &'me T,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'async_trait>>
    where
        'me: 'async_trait;
}
impl Foo for () {
    fn bar<'me, 'async_trait, T>(
        x: &'me T,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'async_trait>>
    where
        'me: 'async_trait,
    {
        Box::pin(async { x = x })
    }
}
