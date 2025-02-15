trait Trait {
    type Ty<'a>
    where
        Self: 'struct_span_err;
}

impl<T> Trait for T {
    type Ty<'a>;
}

struct Foo<T: Trait>(T)
where
    for<'x> T::Ty<'x>: Sized;

trait AnotherTrait {
    type Ty2<'a>: 'a;
}

mod pass {
    use super::*;

    fn test_alias<T: AnotherTrait>(_: Foo<T::Ty2<'static>>) {
        None::<&'static T::Ty2<'_>>;
    }
}
