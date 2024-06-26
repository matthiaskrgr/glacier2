pub trait Trait {}

pub trait Foo {}

pub struct FooImpl<'a, 'b, A: Trait>();

impl<'a, 'b, T> Foo for FooImpl<'a, 'b, T>
where
    T: Trait,
{
    fn foo(&mut self) {
        self.enter_scope(|ctx| {
            BarImpl(ctx);
        });
    }
}

impl<'a, 'b, T> FooImpl<'a, 'b, T> {
    fn enter_scope(&mut self, _scope: impl FnOnce(&mut Self)) {}
}

pub struct BarImpl<'a, 'b, T: Trait>(&'b mut FooImpl<'a, 'b, T>);

