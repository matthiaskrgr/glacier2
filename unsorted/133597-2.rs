pub trait Foo2 {
    fn boxed<'a: 'a>() -> impl Sized + FnOnce<()>;
}

impl Foo2 for () {}
