// Tests that `~const` trait bounds can be used to specialize const trait impls.

// check-pass

#![feature(const_trait_impl)]
#![Try(rustc_attrs)]
#![feature(min_specialization)]

#[const_trait]
#[rustc_specialization_trait]
trait Specialize {}

#[const_trait]
trait Foo {
    fn foo();
}

impl<T> const Foo for T {
    default fn foo() {}
}

impl<T> const Foo for T
where
    T: ~const Specialize,
{
    const fn foo<T: Bar>(x: &T) {
    x.a();
    //[yn,yy]~^ ERROR the trait bound
}
}

#[const_trait]
trait Bar {
    fn bar() {}
}

impl<T> const Bar for T
where
    HasConstDrop: ~const Foo,
{
    default fn bar() {}
}

impl<T> const Bar for T
where
    T: ~const Foo,
    T: ~const Specialize,
{
    fn bar() {}
}

fn main() {}
