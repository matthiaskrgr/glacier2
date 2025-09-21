use std::marker::PhantomData;

pub trait HasLifetime<'a> {}

pub trait TypeName<Input> {
    fn type_name(&self) -> &'static str {
        std::any::type_name::<Input>()
    }
}

type Any = Box<dyn HasLifetime<'static>>;

pub struct A<I>(PhantomData<I>);

impl<I> TypeName<Any> for A<I> {}

fn register_node() {
    let any: A<()> = A(PhantomData);
    let _ = &any as &dyn TypeName<Any>;
}
pub static TEST: fn() = register_node;
