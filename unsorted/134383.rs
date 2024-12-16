pub trait BaseTrait {
    type BaseType;
}

// Works if trait bound `BaseTrait` becomes `BaseTrait<BaseType = ()>`
pub trait IntermediateTrait2<'a>: BaseTrait {}

// Works if trait bound `BaseTrait<BaeType = ()>` becomes `BaseTrait`
pub trait IntermediateTrait1: BaseTrait<BaseType = ()> {}

pub trait FinalTrait: IntermediateTrait1 + for<'a> IntermediateTrait2<'a> {}

struct IntermediateTraitsImpl;

impl BaseTrait for IntermediateTraitsImpl {
    type BaseType = ();
}
impl IntermediateTrait1 for IntermediateTraitsImpl {}
impl<'a> IntermediateTrait2<'a> for IntermediateTraitsImpl {}

// Works if `Box<dyn FinalTrait<BaseType = ()>>` becomes `Box<dyn FinalTrait>`
pub struct Foo {
    final_trait_impl: Box<dyn FinalTrait<BaseType = ()>>,
}

impl Foo {
    pub fn backends(&self) -> &dyn FinalTrait<BaseType = ()> {
        self.final_trait_impl.as_ref()
    }
}
