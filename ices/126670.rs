pub trait A {}

pub trait Mirror {
    type Assoc: ?Sized;
}
impl<T: ?Sized> Mirror for dyn A {
    type Assoc = T;
}

pub trait First {
    async fn first() -> <dyn A + 'static as Mirror>::Assoc;
}

impl First for dyn A {
    async fn first() {}
}

fn main() {}
