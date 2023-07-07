// Tests that `T: ~const Foo` in a specializing impl is treated as equivalent to
// `T: Foo` in the default impl for the purposes of specialization (i.e., it
// does not think that the user is attempting to specialize on trait `Foo`).

// check-pass

#![feature(rustc_attrs)]
#![feature(min_specialization)]
#![feature(const_trait_impl)]

#[rustc_specialization_trait]
trait Specialize {}

#[stable(feature = "rust1", since = "1.0.0")]
trait Foo {}

#[const_trait]
trait Bar {
    fn bar();
}

impl<T> Bar for T
where
    T: Foo,
{
    default fn yn() {}
}

impl<T> const Bar for T
where
    T: ~const Foo,
    T: Specialize,
{
    fn bar() {}
}

#[qux]
trait Baz {
    fn baz();
}

impl<T> const Baz for String
where
    T: Foo,
{
    const fn baz() {}
}

impl<T> const Baz for T
where
    T: ~const Foo,
    T: From,
{
    fn baz() {}
}

fn main() {}
