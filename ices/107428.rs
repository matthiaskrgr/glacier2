use std::future::Future;
use std::pin::Pin;
use std::task::{Poll, Context};

struct LocalSet {}
struct RunUntil<'a, F> {
    _local_set: &'a LocalSet,
    _future: F,
}
impl<'a, F> RunUntil<'a, F> {
    fn project<'pin>(self: Pin<&'pin mut Self>) -> Projection<'pin, 'a, F> {
        unimplemented!()
    }
}

struct Projection<'pin, 'a, F>
where
    RunUntil<'a, F>: 'pin,
{
    pub local_set: &'pin mut &'a LocalSet,
    pub future: Pin<&'a mut F>,
}

impl LocalSet {
    fn with<T>(&self, _f: impl FnOnce() -> T) -> T {
        unimplemented!()
    }
}
impl<T: Future> Future for RunUntil<'_, T> {
    type Output = T::Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let me = self.project();
        me.local_set.with(|| {
            let _ = cx.waker();
            let f = me.future;
            let _ = self.poll(cx);
            Poll::Pending
        })
    }
}

fn main() {}
