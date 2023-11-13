#![feature(non_lifetime_binders)]

trait Trait<Send: ?Sized> {}

fn fail() -> impl for<T> Trait<(), Assoc = impl Trait<T>> {
    16
}
