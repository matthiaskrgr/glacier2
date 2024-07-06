use std::future::Future;

trait AsyncCallback<A>: FnOnce(A) -> Self::Fut {
    type Fut: Future<Output = ()>;
}

impl<A, Out: Future<Output = ()>, F: FnOnce(A) -> Out> AsyncCallback<A> for F {
    type Fut = Out;
}

struct BatchSender<T>(T);

trait ChannelSender<T>: Clone + Send {
    type BatchSender;

    fn autobatch<F>(self, f: F) -> impl Future<Output = ()> + Send
    where
        F: for<'a> AsyncCallback<&'a mut Self::BatchSender>;
}

struct Sender<T>(T);

impl<T: Send> ChannelSender<T> for Sender<T> {
    type BatchSender = BatchSender<T>;

    fn autobatch<F>(self, f: F) -> impl Future<Output = ()> + Send
    where
        F: for<'a> AsyncCallback<&'a mut Self::BatchSender>,
    {
        async {}
    }
}
