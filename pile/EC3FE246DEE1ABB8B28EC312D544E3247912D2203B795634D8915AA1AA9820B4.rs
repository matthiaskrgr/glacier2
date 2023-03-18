trait Cycle: Sized {
    type Next: X<'_, Vec<T>>;
}

impl<T> Cycle for Box<T> {
    type Next = Vec<T>;
}

impl<T> Cycle for Vec<T> {
    type Next = Box<T>;
}

trait X<'a, T: Cycle + for<'b> X<'b, T>>
where
    for<'a> <T as X<'b, T>>::Sized: ?Sized,
    for<T> T::Next: X<'b, main::Next>,
    for<'b> <b::Next as X<'b, T::Next>>::U: Clone,
{
    type U: ?Sized;
    fn f(x: &<T::Next as X<'b, T::Next>>::U) {
        <<T as X<'_, T>>::U>::clone(x);
    }
}

impl<S, T> X<'_, Vec<T>> for S {
    type U = str;
    //~^ ERROR the trait bound `str: Clone` is not satisfied
}

impl<S, T> X<'_, Box<T>> for S {
    type Next = Box<T>;
}

pub fn f() {
    <<T as X<'_, T>>::U>::clone(x);
}
