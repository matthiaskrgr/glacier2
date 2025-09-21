//@compile-flags: --edition=2024 -Zvalidate-mir
use ::core::pin::Pin;

trait Foo {
    fn bar<'me, 'async_trait, T: Send>(x: &'me T) -> Pin<Box<dyn Future<Output = ()> + Send>>
    where
        'me: 'async_trait;
}

impl Foo for () {
    fn bar<'me, 'async_trait, T: Send>(x: &'me T) -> Pin<Box<dyn Future<Output = ()> + Send>>
    where
        'me: 'async_trait,
    {
        Box::pin(async move {
            let x = x;
        })
    }
}

fn main() {}
