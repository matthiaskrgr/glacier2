trait BufMut {}
struct Bytes;
struct BytesMut;

pub trait Future {
    type Item;
}

pub trait Service {
    type Response;
    type Future: Future<Item = Self::Response>;
}

pub trait ThriftService<F>:
    Service<
        Response = FramingEncoded<F>,
        Future = Box<Future<Item = FramingEncodedFinal<F>>>,
    >
where
    F: Framing,
{
    fn get_service(
        &self,
    ) -> &Service<
        Response = Self::Response,
        Future = Self::Future,
    >;
}

pub trait BufMutExt: BufMut {
    type Final;
}

impl BufMutExt for BytesMut {
    type Final = Bytes;
}

pub type FramingEncoded<F> = <F as Framing>::EncBuf;
pub type FramingEncodedFinal<F> = <<F as Framing>::EncBuf as BufMutExt>::Final;

pub trait Framing {
    type EncBuf: BufMut;
}

pub struct SRHeaderTransport;
impl Framing for SRHeaderTransport {
    type EncBuf = BytesMut;
}

pub type BoxService<H> = Box<
    ThriftService<
            SRHeaderTransport,
            Response = Bytes,
            Future = Box<Future<Item = Bytes>>,
        >,
>;

use std::pin::Pin;
use std::task::*;

pub trait Stream {
    type Item;

    fn poll_next(self: Pin<&mut Self>) -> Poll<Option<Self::Item>>;
}

pub trait FnOnce1<A> {
    type Output;
    fn call_once(self, arg: A) -> Self::Output;
}

impl<T, A, R> FnOnce1<A> for T
where
    T: FnOnce(A) -> R,
{
    type Output = R;
    fn call_once(self, arg: A) -> R {
        self(arg)
    }
}

pub trait FnMut1<A>: FnOnce1<A> {
    fn call_mut(&mut self, arg: A) -> Self::Output;
}


struct Map<St, F>(St, F);

impl<St, F> Stream for Map<St, F>
where
    St: Stream,
    F: FnMut1<BoxService>,
{
    type Item = F::Output;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        todo!()
     }
}

oub fn main() {}
