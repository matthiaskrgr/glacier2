trait FooMut {
    type Baz: 'static;

    fn bar<'a, I>(self, iterator: &'a I) where for <'b> &'b I: IntoIterator<Item= &'b &'a Self::Baz>;
}

struct DelegatingFooMut<T> where T: FooMut {
    delegate: T
}

impl<T> FooMut for DelegatingFooMut<T> where T: FooMut {
    type Baz = DelegatingBaz<T::Baz>;

    fn bar<'a, I>(self, collection: &'a I) where for <'b> &'b I: IntoIterator<Item= &'b &'a Self::Baz> {
        let collection = collection.into_iter().map(|b| &b.delegate);

        self.delegate.bar(collection)
    }
}

struct DelegatingBaz<T> {
    delegate: T
}
