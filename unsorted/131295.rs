#![feature(generic_const_exprs)]
trait MyTrait<T, U: From<T>> {}

impl<'a, 'b, T, U> MyTrait<T> for U {
    async fn foo(
        _: T,
    ) -> (
        &'a U,
        &'static dyn MyTrait<
            [(); {
                |x: &'a u32| x;
                4
            }],
        >,
    ) {
    }
}
