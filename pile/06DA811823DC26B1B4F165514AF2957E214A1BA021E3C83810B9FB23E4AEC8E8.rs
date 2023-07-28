#![feature(type_alias_impl_trait)]
#![allow(dead_code)]

type Bug<T, U> = impl Fn(T) -> U + Copy; //~ ERROR cycle detected

const CONST_BUG: Bug<u8, ()> = unsafe { std::mem::transmute(|_: u8| ()) };

fn make_bug<T, U: From<T>>() -> Bug<T, U> {
    |x| x.into() //~ ERROR the trait bound `U: From<T>` is not satisfied
}

fn main() {
    #[derive(Copy, Clone)]
    struct Foo((u32, u32));

    type T = impl Copy;
    let foo: T = Foo((1u32, 2u32));
    let x = move || {
        let Foo((a, b)) = foo;
    };
}
