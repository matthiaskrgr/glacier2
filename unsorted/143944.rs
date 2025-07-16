#![feature(const_trait_impl)]

#[const_trait]
trait Foo
where
    Self::Assoc: Clone,
{
    type Assoc;
}
