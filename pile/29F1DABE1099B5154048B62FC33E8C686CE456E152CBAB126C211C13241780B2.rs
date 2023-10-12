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
        Int(self.0.plus(rhs.0))
    }

const _: () = assert!(need_const_closure(Tr::a) == 42);

fn main() {}
