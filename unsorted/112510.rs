use std::future::Future;

trait Stream {}

trait X {
    type LineStream<'a, Repr>
    where
        Self: 'a;
    type LineStreamFut<'a, Repr>
    where
        Self: 'a;
}

struct Y;

impl X for Y {
    type LineStream<'a, 'b, T, U> = impl Stream;
    type LineStreamFut<'a, Repr> = impl Future<Output = Self::LineStream<'a, Repr>>;
    fn line_stream<'a, Repr>(&'a self) -> Self::LineStreamFut<'a, Repr> {}
}
