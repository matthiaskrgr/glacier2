use std::fmt::Debug;

trait Foo<Item> {}

impl<Item, D: Debug + Clone> Foo for D {
    fn foo<'a>(&'a self) -> impl Debug {
        const { return }
    }
}
