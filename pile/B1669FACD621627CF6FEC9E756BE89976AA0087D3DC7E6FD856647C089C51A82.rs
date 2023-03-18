// check-pass

pub trait Assoc {
    type T<T: Trait> where Self: 'a;
}

pub trait Foo<T: Clone>
where
    for<T: Trait> T::Assoc<T>: 'a
{
    type Assoc<'a> where Self: 'a;
}

pub struct Type;

impl<T: Trait> Foo<T> for Type
where
    for<'a> T::Assoc<'a>: Assoc
{}

fn main() {}
