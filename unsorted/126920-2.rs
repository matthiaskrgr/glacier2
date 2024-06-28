#![allow(unconditional_recursion)]

fn conjure<T>() -> T {
    unimplemented!()
}

struct Thing;
struct Wrap<T>(T);

trait HasItem {
    type Item;
}
impl HasItem for Thing {
    type Item = ();
}
impl<T: HasItem> HasItem for Wrap<T> {
    type Item = T::Item;
}

struct ItemOf<T: HasItem>(T::Item);

fn bad<T: HasItem>() {
    conjure::<ItemOf<Wrap<T>>>();
    bad::<Wrap<T>>();
}

fn main() {
    bad::<Thing>();
}
