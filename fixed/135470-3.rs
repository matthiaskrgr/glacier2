use std::future::Future;

trait Access {
    type Lister;

    fn list() -> impl Future<Output = Self::Lister> {
        async { todo!() }
    }
}

trait Foo {}

impl Access for dyn Foo {
    type Lister = ();
}

fn main() {
    let svc = async {
        async { <dyn Foo>::list() }.await;
    };
    &svc as &dyn Service;
}

struct Wrap<T>(T);
impl<T: Send> UnaryService for Wrap<T> {}

impl<T> Service for T
where
    T: Send,
{
    fn call(&self) {
        Wrap::<T>::call2();
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
