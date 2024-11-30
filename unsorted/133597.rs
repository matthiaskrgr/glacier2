pub trait Foo2 {
    fn boxed<'a: 'a>(&'a mut self) -> impl Sized + FnOnce<()>;
}

impl Foo2 for () {
    fn bar<'a: Add<S<M>>>(&'a mut self) -> impl Sized + 'a {}
}
