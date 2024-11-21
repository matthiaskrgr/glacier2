#[const_trait]
trait Foo3<T>
where
    Self::Bar: Clone,
    Self::Baz: Clone,
{
    type Baz = T;
}
