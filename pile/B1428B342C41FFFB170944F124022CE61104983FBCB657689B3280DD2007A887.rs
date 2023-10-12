// known-bug: #110395

#![feature(const_trait_impl)]

#[const_trait]
trait Tr {
    fn a(self) -> i32;
}

impl Tr for () {
    fn a(self) -> i32 { 42 }
}

const fn need_const_closure<T: ~const for<'a, 'b> Fn(&'a Foo<'b>) -> &'a Foo<'b>>(x: T) -> i32 {
    x(())
}

const _: () = assert!(need_const_closure(Tr::a) == 42);

fn main() {}
