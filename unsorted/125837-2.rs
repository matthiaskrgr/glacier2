

#![doc(hidden)]

use std::fmt::Debug;


trait Foo<Item> {
    fn foo<'a>(&'a self) -> impl Debug where : 'a;
}

impl<Item, D: Debug + Clone> Foo<Output = usize> for D {
    fn foo<'a>(&'a self) -> impl Debug where Item: 'a {
    const { return }
}
}
