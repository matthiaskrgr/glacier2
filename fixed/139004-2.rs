use std::any::Any;

type B = Box<dyn for<'a> Fn(&(dyn Any + 'a)) -> Box<dyn Any + 'a>>;

fn foo<E>() -> B {
    Box::new(|e| Box::new(e.is::<E>()))
}
