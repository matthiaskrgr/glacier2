#![feature(non_lifetime_binders)]

trait Trait<T: ?Sized> {
    type Assoc<'a> = i32;
}

fn produce() -> impl for<T> Trait<(), Assoc = impl Trait<T>> {
    16
}
