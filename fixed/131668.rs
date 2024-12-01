#![feature(generic_associated_types_extended)]
fn make_static<'a, 'b, T: ?Sized>(x: &'a T) -> &'b T {
    trait A<T>: B<T = T> {}

    trait B {
        type Y<const N: i16>;
    }

    struct Erase<T: ?Sized + B>(T::T);

    Erase::<dyn for<'c> A<&'c T>>(Waker::noop()).0
}
