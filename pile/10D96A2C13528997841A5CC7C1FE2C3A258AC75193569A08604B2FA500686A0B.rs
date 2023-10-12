// known-bug: #110395

#![feature(const_trait_impl)]

#[const_trait]
trait Tr {
    fn a(self) -> i32;
}

impl Tr for () {
    fn a(self) -> i32 { 42 }
}

const fn need_const_closure<T: ~const FnOnce(()) -> i32>(x: T) -> i32 {
    x(())
}

const _: () = assert!(need_const_closure(Tr::a) == 42);

fn main() {
    //~^ ERROR expected non-macro attribute, found attribute macro
    let lam = |#[id] W(x), #[id] y: usize| ();
    //~^ ERROR expected non-macro attribute, found attribute macro
    //~| ERROR expected non-macro attribute, found attribute macro
}
