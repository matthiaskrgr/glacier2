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

const _: () = checker!(attr_free, "fn free(#[a1] arg1 : u8) { let lam = | #[a2] W(x), #[a3] y | () ; }");

fn main() {}
