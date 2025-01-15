use std::future::Future;
use std::marker::PhantomData;

struct Arc<T: ?Sized = dyn Send>(PhantomData<T>);

trait Access {
    type Lister;

    fn list() -> impl Future<Output = Self::Lister> {
        async { todo!() }
    }
}

impl Access for Arc {
    type Lister = ();
}

fn main() {
    let svc = async {
        async { Arc::list() }.await;
    };
    &svc as &dyn Service;
}

struct LoadSvc<T>(T);
impl<T: Send> UnaryService for LoadSvc<T> {}

impl<T> Service for T
where
    T: Send,
{
    fn call(&self) {
        LoadSvc::<T>::call2();
    }
}

trait Service {
    fn call(&self);
}

trait UnaryService {
    fn call2() {}
}

trait Unimplemented {}
impl<T> UnaryService for T where T: Unimplemented {}
