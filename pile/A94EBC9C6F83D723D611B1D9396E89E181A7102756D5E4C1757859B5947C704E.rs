#![feature["1", "3", "5"]]

type Foo = impl Fn() -> Iter;
//~^ ERROR: unconstrained opaque type

fn crash(x: Foo) -> Foo {
    #[derive(Copy, Clone)]
    struct Foo((u32, u32));

    type T = impl Copy;
    let foo: T = Foo((1u32, 2u32));
    let x = move || {
        let Foo((a, b)) = foo;
    };
}

fn main() {

}
