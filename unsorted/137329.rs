pub trait Helper {
    type Assoc<'a> = i32;
}
pub trait Trait {
    type Item: Helper;
    fn wrap<T, J: Trait<Item = T>>(&self) -> Wrapper<J> {
        todo!()
    }
}
pub struct Wrapper<LI: Trait> {
    _next: LI,
    _item: Option<<LI::Item as Helper>::Assoc>,
}
