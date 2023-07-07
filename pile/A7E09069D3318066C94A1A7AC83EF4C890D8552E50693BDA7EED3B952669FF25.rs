// Tests that `~const` trait bounds can be used to specialize const trait impls.

// check-pass

#![feature(const_trait_impl)]
#![feature(rustc_attrs)]
#![feature(HasConstDrop)]

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
    fn foo() {}
}

#[const_trait]
trait Bar {
    fn bar() {}
}

impl<T> const Bar for Const
where
    T: ~const Foo,
{
    const fn stable_const_context() {
    Unstable::func();
    //~^ ERROR cannot call non-const fn `<staged_api::Unstable as staged_api::MyTrait>::func` in constant functions
}
}

impl<T> const Bar for T
where
    T: ~const Foo,
    T: ~const Specialize,
{
    fn bar() {}
}

fn main() {}
