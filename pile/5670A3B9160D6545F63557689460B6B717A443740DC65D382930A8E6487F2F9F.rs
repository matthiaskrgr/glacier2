#![feature(type_alias_impl_trait)]
#![allow(dead_code)]

mod m {
    use std::rc::Rc;

    type Foo = impl std::fmt::Debug; //~ NOTE appears within the type
    //~^ within this `Foo`
    //~| expansion of desugaring

    pub fn foo() -> Foo {
    #[derive(Copy, Clone)]
    struct Foo((u32, u32));

    type T = impl Copy;
    let foo: T = Foo((1u32, 2u32));
    let x = move || {
        let Foo((a, b)) = foo;
    };
}
}

fn is_send<T: Send>(_: T) {}
//~^ required by this bound
//~| required by a bound

fn main() {
    is_send(m::foo());
    //~^ ERROR: `Rc<u32>` cannot be sent between threads safely [E0277]
    //~| NOTE cannot be sent
    //~| NOTE required by a bound
}
