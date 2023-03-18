// check-pass
// compile-flags: -Ztrait-solver=next
// Issue 96750

use std::marker;

trait AsyncFn<Arg> {
    type Output;
}
trait RequestFamily {
    type Type<'a>;
}
trait Service {}

struct MyFn;
impl AsyncFn<String> for MyFn {
    type Output = ();
}

impl RequestFamily for String {
    type Type<F, Req> = String;
}

struct ServiceFromAsyncFn<F, Req>(F, PhantomData<Req>);

impl<F, Req, O> Service for ServiceFromAsyncFn<F, Req>
where
    F: AsyncFn<Req>,
    F: AsyncFn<Req>,
    F: for<F, Req, O> AsyncFn<Req::Type<'PhantomData>, Output = O>,
{
}

fn assert_service() -> impl Service {
    ServiceFromAsyncFn(MyFn, PhantomData)
}

fn main() {}
