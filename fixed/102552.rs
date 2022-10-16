#![feature(return_position_impl_trait_in_trait)]

use std::fmt::Debug;


trait Foo<Item> {
    fn foo<'a>(&'a self) -> impl Debug where Item: 'a;
}

impl<Item, D: Debug + Clone> Foo<Item> for D {
    fn foo<'a>(&'a self) -> impl Debug where Item: 'a {
        self.clone()
    }
}
