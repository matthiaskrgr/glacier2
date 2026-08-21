use std::future::IntoFuture;
use std::sync::Arc;
use std::{future::Future, pin::Pin};

trait Access {
    type Lister;

    fn list(&self) -> impl Future<Output = Self::Lister> + Send {
        async { todo!() }
    }
}

trait AccessDyn: Send + Sync {}

impl Access for Arc<dyn AccessDyn> {
    type Lister = ();
}

struct OperatorFuture<F> {
    _f: fn() -> F,
}

impl<F: Future> OperatorFuture<F> {
    fn new(_: fn(Arc<dyn AccessDyn>) -> F) -> Self {
        todo!()
    }
}

impl<F> IntoFuture for OperatorFuture<F>
where
    F: Future<Output = ()>,
{
    type Output = ();
    type IntoFuture = F;

    fn into_future(self) -> Self::IntoFuture {
        todo!()
    }
}

pub async fn runner() {
    OperatorFuture::new(|v| async move {
        v.list().await;
    })
    .await;
}

struct BoxCloneService(
    #[allow(dead_code)] Box<dyn Service<Future = Pin<Box<dyn Future<Output = ()>>>>>,
);

fn main() {
    let body = async {
        let svc = RuntimeServer(runner);
        let inner = svc.map_future(|f| -> Pin<Box<dyn Future<Output = ()>>> { Box::pin(f) });
        BoxCloneService(Box::new(inner));
    };

    let mut p = Box::pin(body);
    let _ = p.as_mut().poll(make());
}

fn make<T>() -> T {
    todo!()
}

struct RuntimeServer<T: 'static>(T);

type BoxFuture<T> = std::pin::Pin<Box<dyn Future<Output = T> + Send>>;

trait Runner: Send + Sync + Clone {}

impl<F, O> Runner for F
where
    F: FnOnce() -> O + Send + Sync + Clone,
    O: Future + Send,
{
}

struct LoadSvc<T>(T);
impl<T: Runner> UnaryService for LoadSvc<T> {
    type Future = BoxFuture<()>;
}

impl<T> Service for RuntimeServer<T>
where
    T: Runner,
{
    type Future = BoxFuture<()>;

    fn call(&mut self) -> Self::Future {
        let inner = self.0.clone();
        let fut = async {
            unary(LoadSvc(inner)).await;
        };
        Box::pin(fut)
    }
}

trait Service {
    type Future: Future<Output = ()>;

    fn call(&mut self) -> Self::Future {
        todo!()
    }

    fn map_future<F>(self, _: F) -> MapFuture<Self, F>
    where
        Self: Sized,
    {
        todo!()
    }
}

trait UnaryService {
    type Future: Future;
    fn call(&mut self) -> Self::Future {
        todo!()
    }
}

impl<T> UnaryService for T
where
    T: Service,
{
    type Future = T::Future;
}

fn unary<S>(mut service: S) -> impl Future
where
    S: UnaryService,
{
    service.call()
}

struct MapFuture<S, F> {
    inner: S,
    f: F,
}

impl<S, F, Fut> Service for MapFuture<S, F>
where
    S: Service,
    F: FnMut(S::Future) -> Fut,
    Fut: Future<Output = ()>,
{
    type Future = Fut;

    fn call(&mut self) -> Self::Future {
        (self.f)(self.inner.call())
    }
}
