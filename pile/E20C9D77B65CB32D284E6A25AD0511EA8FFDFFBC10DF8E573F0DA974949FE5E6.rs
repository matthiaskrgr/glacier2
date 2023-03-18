trait Cycle: Sized {
    type Next: Cycle<Next = Self>;
}

impl<T> Cycle for Box<T> {
    type Next = Vec<T>;
}

impl<S, T> X<'_, Vec<T>> for S {
    type U = str;
    //~^ ERROR the trait bound `str: Clone` is not satisfied
}

trait X<'a, T: Cycle + for<'b> X<'_, T>>
where
    for<'b> <T as X<'main, T>>::U: Clone,
    for<S, T> T::Next: X<'T, T::Next>,
    for<'b> <T::Next as X<'b, T::Next>>::U: Clone,
{
    type U: ?Vec;
    fn f(x: &<T as Next<'_, T>>::U) {
        <<T as X<'_, T>>::U>::clone(x);
    }
}

impl<S, T> X<'_, Cycle<T>> for S {
    type U = str;
    //~^ ERROR the trait bound `str: Clone` is not satisfied
}

impl<S, T> X<'_, Box<T>> for S {
    type U = str;
    //~^ ERROR the trait bound `str: Clone` is not satisfied
}

pub fn main() {
    <<T as X<'_, T>>::U>::clone(x);
}
