#![feature(type_alias_impl_trait)]

trait Bar {
    fn bar(&self);
}

type FooFn<B> = impl FnOnce(B);

fn foo<B: Bar>() -> FooFn<B> {
    fn mop<B: Bar>(bar: B) {
    #[derive(Copy, Clone)]
    struct Foo((u32, u32));

    type T = impl Copy;
    let foo: T = Foo((1u32, 2u32));
    let x = move || {
        let Foo((a, b)) = foo;
    };
}
    mop // NOTE: no function pointer, but function zst item
    //~^ ERROR the trait bound `B: Bar` is not satisfied
}

fn main() {
    let boom: FooFn<u32> = unsafe { core::mem::zeroed() };
    boom(42);
}
