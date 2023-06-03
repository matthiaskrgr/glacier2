mod foo {
    pub trait MyTrait<'a> {
        type MyItem: ?Sized;
    }

    pub struct Inner<'a, Q, R: ?Sized> {
        field: Q,
        field3: &'a u8,
        my_foo: Foo<Q>,
        field2: R,
    }

    pub struct Outer<'a, T, K: ?Sized> {
        my_inner: Inner<'a, T, K>,
    }

    pub struct Foo<T> {
        myfield: T,
    }
}

pub use foo::{Foo, Inner as NotInner, MyTrait as NotMyTrait};

unsafe impl<'a, Q, R: ?Sized> Send for NotInner<'static, Q, R>
where
    Q: NotMyTrait<'a>,
    <Q as NotMyTrait<'a>>::MyItem: Copy,
    R:,
    Foo<Q>: Send,
{
}
