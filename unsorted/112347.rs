#![feature(associated_type_bounds)]
#![feature(unboxed_closures)]

use std::future::Future;

use async_trait::async_trait;

#[async_trait]
trait Trait<Arg0>
where
    Arg0: ?Sized,
{
    type Return;

    async fn func(&mut self, arg0: &Arg0) -> Self::Return;
}

#[async_trait]
impl<AsyncFn, Arg0, Return> Trait<Arg0> for AsyncFn
where
    for<'a> AsyncFn: Fn<(&'a Arg0,), Output: Future<Output = Return> + Send + 'a> + Send + Sync,
    Arg0: ?Sized + Sync,
{
    type Return = Return;

    async fn func(&mut self, arg0: &Arg0) -> Self::Return {
        self(arg0).await
    }
}

async fn strlen(x: &str) -> usize {
    x.len()
}

#[tokio::test]
async fn test() {
    assert_eq!(strlen.func("hello world").await, 11);
}
