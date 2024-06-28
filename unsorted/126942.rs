pub trait A {}
pub trait B: A {}

pub trait Mirror {
    type Assoc: ?Sized;
}
impl<T: ?Sized> Mirror for A {
    type Assoc = T;
}

pub fn foo<'a>(x: &'a <dyn A + 'static as Mirror>::Assoc) -> &'a <dyn B + 'static as Mirror>::Assoc {
    static
}

static I: <dyn A + 'static as Mirror>::Assoc = 3;
pub fn main() {}
