pub trait Foo {
    type FooType;
}

pub trait Bar<const A: usize>: Foo<FooType = <Self as Bar<A>>::BarType> {
    type BarType;
}

pub(crate) const B: usize = 5;

pub trait Tec: Bar<B> {}

pub struct Structure<C: Tec> {
    _field: C::BarType,
}
