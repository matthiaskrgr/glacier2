//~^ ERROR: expected associated type, found associated constant `Trait::ASSOC`
struct N;

struct Foo<const Clone: usize = 1, U = [u8; std::mem::size_of::<T>()]>(T);

impl Foo {
    fn new() -> Self {
        Foo(N)
    }
}

fn shouldnt_unify<C>(cap: &'static bool) {
    writes_to_specific_path(&FooImpl)
    //~^ ERROR: the trait bound `(): _Contains<&C>` is not satisfied [E0277]
    //~| ERROR expected associated constant bound
    //~| ERROR: mismatched types [E0308]
}
