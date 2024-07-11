pub struct Struct<T>
where
    T: Trait<Type2 = ()>,
{
    pub field: T::Type1,
}
pub trait Trait: BaseTrait<Type1 = <Self as Trait>::Type2> {
    type Type2;
}
pub trait BaseTrait {
    type Type1;
}
