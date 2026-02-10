// rustc --crate-type=lib --edition=2021
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct Root {
    inner: InnerB,
}

struct InnerB {
    inner: Pin<Box<dyn Future<Output = ()>>>,
}
impl Root {
    fn new() -> Self {
        todo!()
    }
}
impl Future for Root {
    type Output = ();
    fn poll(self: Pin<&mut Self>, _: &mut Context) -> Poll<Self::Output> {
        todo!();
    }
}
async fn fetch() {
    let fut = async {
        Root::new().await;
    };
    let _: Pin<Box<dyn Future<Output = ()> + Send>> = Box::pin(fut);
}
