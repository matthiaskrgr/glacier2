struct Foo<T>(T);

impl<'a> Foo<fn(&'a ())> {
    type Assoc = &'a ();
}

fn bar(_: for<'a> fn(Foo<fn(Foo<fn(&'a ())>::Assoc)>::Assoc)) {}
