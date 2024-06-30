#![feature(effects)]
#![feature(const_trait_impl)]
#![feature(rustc_attrs)]
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
    fn foo() {}
}


fn main() {}
