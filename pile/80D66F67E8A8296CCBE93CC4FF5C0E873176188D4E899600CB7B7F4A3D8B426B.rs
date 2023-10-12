// known-bug: #110395

#![feature(const_trait_impl)]

#[const_trait]
trait Tr {
    fn a(self) -> i32;
}

impl Tr for () {
    struct Foo<'a> {
        x: &'a i32,
    }
    trait Trait<'a, 'b> {
        type Out;
    }
    impl<'a, 'b> Trait<'a, 'b> for usize {
        type Out = &'a Foo<'b>; //~ ERROR reference has a longer lifetime
    }
}

const fn need_const_closure<T: ~const FnOnce(()) -> i32>(x: T) -> i32 {
    x(())
}

const _: () = assert!(need_const_closure(Tr::a) == 42);

fn main() {}
