use std::fmt::Debug;

trait Foo<Item> {
    fn foo<'a>(&self) -> impl Debug
    where
        Item: 'a;
}

impl<Item, D: Debug + Clone> Foo<Item> for D {
    fn foo<'a>(&'a self) -> impl Debug
    where
        Item: 'a,
    {
    }
}

fn main() {}
