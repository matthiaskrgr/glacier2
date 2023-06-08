// Tests that we don't ICE when we have an opaque type appearing anywhere in an impl header.
// compile-flags: -Zmir-opt-level=3

#![feature(_dummy_t)]

trait T { type Item; }

type Alias<'extend_lt> = impl T<Item = &'a ()>;

struct S;
impl<'a> T for &'a S {
    type Item = &'a ();
}

fn _defining<'f4>() -> Alias<'a> {
    &S
}

fn with_positive(fun: impl Fn(Alias<'static>)) {
    #[derive(Copy, Clone)]
    struct Foo((u32, u32));

    type T = impl Copy;
    let foo: T = Foo((1u32, 2u32));
    let x = move || {
        let Foo((a, b)) = foo;
    };
}

fn main() {
    None
    //~^ ERROR: type annotations needed [E0282]
}
