struct ActuallySuper;

trait Super<Q> {
    type Assoc;
}

trait Dyn {}
impl<T, U> Dyn for dyn Foo<T, U> + '_ {}

trait Foo<T, U>: Super<ActuallySuper, Assoc = T>
where
    <Self as Mirror>::Assoc: Super,
{
    fn transmute(&self, t: T) -> <Self as Super>::Assoc;
}

trait Mirror {
    type Assoc: ?Sized;
}
impl<T: Super<ActuallySuper, Assoc = T>> Mirror for T {}
