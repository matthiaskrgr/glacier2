pub trait Trait {}

pub type Foo = impl for<'a> Trait<'a, Assoc = impl Copy + 'a>;
