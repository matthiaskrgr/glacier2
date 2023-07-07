#![feature(type_alias_impl_trait)]

use std::fmt::Debug;

type Foo = impl Debug;
//~^ ERROR cycle detected

fn is_send<T: Send>() { }

fn not_good() {
    // Error: this function does not constrain `Foo` to any particular
    // hidden type, so it cannot rely on `Send` being true.
    is_send::<Foo>();
}

fn not_gooder() {
    #[derive(Copy, Clone)]
    struct Foo((u32, u32));

    type T = impl Copy;
    let foo: T = Foo((1u32, 2u32));
    let x = move || {
        let Foo((a, b)) = foo;
    };
}

fn main() {}
