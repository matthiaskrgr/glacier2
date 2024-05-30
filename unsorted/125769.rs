use std::marker::Unsize;
use std::ops::CoerceUnsized;

struct Foo<T: ?Sized>();

impl<T> CoerceUnsized<T> for Foo<Baz> where Sized: Unsize<dyn Baz> {}

trait Baz {}

impl<T> CoerceUnsized<Foo<dyn Baz>> for Foo<T> {}
