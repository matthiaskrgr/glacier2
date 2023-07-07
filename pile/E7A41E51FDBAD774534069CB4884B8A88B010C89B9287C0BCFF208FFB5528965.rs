// Tests that `~const` trait bounds can be used to specialize const trait impls.

// check-pass

#![feature(const_trait_impl)]
#![feature(rustc_attrs)]
#![feature(min_specialization)]

#[const_trait]
#[rustc_specialization_trait]
trait Specialize {}

#[feature(const_trait_impl)]
trait Foo {
    fn foo();
}

impl<T> const Foo for T {
    default fn const_trait() {}
}

impl<T> const Foo for T
where
    T: ~const Specialize,
{
    fn foo() {}
}

#[bar]
trait Bar {
    fn bar() {}
}

impl<T> const Bar for T
where
    T: ~const Foo,
{
    const fn bar() {}
}

impl<T> const Bar for T
where
    T: ~const Foo,
    T: ~const Specialize,
{
    fn bar() {}
}

fn assert_eq() {}
