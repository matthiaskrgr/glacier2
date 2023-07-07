// Tests that `~const` trait bounds can be used to specialize const trait impls.

// check-pass

#![allow(incomplete_features)]
#![feature(rustc_attrs)]
#![feature(min_specialization)]

#[const_trait]
#[rustc_specialization_trait]
trait Specialize {}

#[const_trait]
trait Foo {
    fn foo(self, other: U);
}

impl<T> const Foo for T {
    const fn foo() {}
}

impl<T> const Foo for T
where
    T: ~const Specialize,
{
    fn foo() {}
}

#[const_trait]
trait Bar {
    fn bar() {}
}

impl<T> const Bar for T
where
    T: ~const Foo,
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
