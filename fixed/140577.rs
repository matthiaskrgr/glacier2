use std::future::{self, Future};
use std::marker::PhantomData;
use std::pin::Pin;
use std::sync::Arc;
use std::task::Context;
use std::task::Poll;

trait Acquire<'c> {
    type Connection;
    fn acquire(self) -> Pin<Box<dyn Future<Output = Result<Self::Connection, ()>> + Send + 'c>>;
}
struct PgConnection;
impl<'c> Acquire<'c> for &'c mut PgConnection {
    type Connection = ();
    fn acquire(self) -> Pin<Box<dyn Future<Output = Result<Self::Connection, ()>> + Send + 'c>> {
        unimplemented!()
    }
}
fn login<'a>(db: impl Acquire<'a> + Send + 'a) -> impl Future<Output = ()> + Send + 'a {
    async move {
        let _ = db.acquire().await;
    }
}
fn main() {
    path()
        .then(|| async {
            let mut conn = PgConnection;
            login(&mut conn).await;
        })
        .then(|_| async { unimplemented!() })
        .boxed();
}

fn path() -> impl Filter<Extract = (), Error = ()> {
    filter_fn(move || future::ready(Err(())))
}

struct Then<T, F> {
    _marker: PhantomData<(T, F)>,
}
impl<T, F> FilterBase for Then<T, F>
where
    T: Filter,
    F: Func<T::Extract> + Clone + Send,
    F::Output: Future + Send,
{
    type Extract = (<F::Output as Future>::Output,);
    type Error = T::Error;
    type Future = ThenFuture<T, F>;
}
struct ThenFuture<T, F>
where
    T: Filter,
    F: Func<T::Extract>,
    F::Output: Future + Send,
{
    _state: State<T::Future, F>,
}
enum State<T, F>
where
    T: TryFuture,
    F: Func<T::Ok>,
    F::Output: Future + Send,
{
    Second(F::Output),
}
impl<T, F> Future for ThenFuture<T, F>
where
    T: Filter,
    F: Func<T::Extract>,
    F::Output: Future + Send,
{
    type Output = Result<(<F::Output as Future>::Output,), T::Error>;
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        unimplemented!()
    }
}
struct MapErr<T, F> {
    _filter: T,
    _callback: F,
}
impl<T, F, E> FilterBase for MapErr<T, F>
where
    T: Filter,
    F: Fn(T::Error) -> E + Clone + Send,
{
    type Extract = T::Extract;
    type Error = E;
    type Future = MapErrFuture<T, F>;
}
struct MapErrFuture<T: Filter, F> {
    _extract: T::Future,
    _callback: F,
}
impl<T, F, E> Future for MapErrFuture<T, F>
where
    T: Filter,
    F: Fn(T::Error) -> E,
{
    type Output = Result<T::Extract, E>;
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        unimplemented!()
    }
}

struct BoxedFilter<T> {
    _filter: Arc<
        dyn Filter<
                Extract = T,
                Error = (),
                Future = Pin<Box<dyn Future<Output = Result<T, ()>> + Send>>,
            > + Send
            + Sync,
    >,
}
impl<T: Send> BoxedFilter<T> {
    fn new<F>(filter: F) -> BoxedFilter<T>
    where
        F: Filter<Extract = T> + Send + Sync + 'static,
        F::Error: Into<()>,
    {
        let filter = Arc::new(BoxingFilter {
            filter: filter.map_err(Internal, Into::into),
        });
        BoxedFilter { _filter: filter }
    }
}
struct BoxingFilter<F> {
    filter: F,
}
impl<F> FilterBase for BoxingFilter<F>
where
    F: Filter,
    F::Future: Send + 'static,
{
    type Extract = F::Extract;
    type Error = F::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Extract, Self::Error>> + Send>>;
    fn filter(&self, _: Internal) -> Self::Future {
        Box::pin(self.filter.filter(Internal).into_future())
    }
}
trait FilterBase {
    type Extract;
    type Error;
    type Future: Future<Output = Result<Self::Extract, Self::Error>> + Send;
    fn filter(&self, _internal: Internal) -> Self::Future {
        unimplemented!()
    }
    fn map_err<F, E>(self, _internal: Internal, _fun: F) -> MapErr<Self, F>
    where
        Self: Sized,
        F: Fn(Self::Error) -> E + Clone,
        E: Send,
    {
        unimplemented!()
    }
}
struct Internal;
trait Filter: FilterBase {
    fn then<F>(self, _fun: F) -> Then<Self, F>
    where
        Self: Sized,
        F: Func<Self::Extract> + Clone,
        F::Output: Future + Send,
    {
        unimplemented!()
    }
    fn boxed(self) -> BoxedFilter<Self::Extract>
    where
        Self: Sized + Send + Sync + 'static,
        Self::Extract: Send,
        Self::Error: Into<()>,
    {
        BoxedFilter::new(self)
    }
}
impl<T: FilterBase> Filter for T {}
fn filter_fn<F, U>(_func: F) -> FilterFn<F>
where
    F: Fn() -> U,
    U: TryFuture,
{
    unimplemented!()
}
struct FilterFn<F> {
    _func: F,
}
impl<F, U> FilterBase for FilterFn<F>
where
    F: Fn() -> U,
    U: TryFuture + Send + 'static,
{
    type Extract = U::Ok;
    type Error = U::Error;
    type Future = IntoFuture<U>;
}

trait Func<Args> {
    type Output;
}
impl<F, R> Func<()> for F
where
    F: Fn() -> R,
{
    type Output = R;
}
impl<F, R, T2> Func<(T2,)> for F
where
    F: Fn(T2) -> R,
{
    type Output = R;
}
trait TryFuture: Future {
    type Ok;
    type Error;
    fn into_future(self) -> IntoFuture<Self>
    where
        Self: Sized,
    {
        unimplemented!()
    }
}
impl<F, T, E> TryFuture for F
where
    F: ?Sized + Future<Output = Result<T, E>>,
{
    type Ok = T;
    type Error = E;
}
struct IntoFuture<Fut> {
    _future: Fut,
}
impl<Fut: TryFuture> Future for IntoFuture<Fut> {
    type Output = Result<Fut::Ok, Fut::Error>;
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        unimplemented!()
    }
}
