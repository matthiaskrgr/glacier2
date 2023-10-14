// known-bug: #110395

#![feature(const_trait_impl)]

#[cfg_attr(stable, rustc_const_stable(feature = "foo", since = "1.0.0"))]
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

fn main() {}