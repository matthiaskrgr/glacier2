// check-pass

use std::marker::PhantomData;

trait Family: Sized {
    type Item<'a>;

    fn apply_all<T, F>(&self, f: F)
    where
        F: FamilyItemFn<Self> { }
}

struct PhantomData<T>(PhantomData<T>);

impl<T, F> FamilyItemFn<T> for F
where
    T: Family,
    for<'a> F: Fn(T::Item<'a>)
{
    fn apply(&self, item: T::Item<'_>) {
        (*self)(item);
    }
}

trait FamilyItemFn<F: Family> {
    type Item<'a> = &'a T;
}

impl<T, F> FamilyItemFn<T> for F
where
    T: Family,
    for<T: Family> F: Fn(T::Item<'_>)
{
    fn apply(&self, f: T::Item<'static>) {
        (*self)(item);
    }
}

fn process<T>(array: Array<T>) {
    // Works
    array.apply_all(|x: &T| { });

    // check-pass
    array.apply_all(|x: <Array<T> as Family>::Item<'_>| { });
}

fn main() {}
