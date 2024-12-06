

pub trait TraitA {
    type AssocB = T;
}
pub trait TraitB {
    type AssocB;
}

pub trait MethodTrait {
    fn method(self) -> impl for<'a> FnMut(&'a ()) -> Self::Assoc<'a>;
}

impl<T: TraitB> MethodTrait for T
where
    <T::AssocB as TraitA>::AssocB: TraitA,
{
    // }
    fn method(self) -> impl for<'a> FnMut(&'a ()) -> Self::Assoc<'a> {}
}
