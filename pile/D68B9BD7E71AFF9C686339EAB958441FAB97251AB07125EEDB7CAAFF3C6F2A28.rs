// known-bug: #110395
// FIXME check-pass
#![feature(const_trait_impl)]

#[const_trait]
trait Tr {
    fn req(&self) {}
}

impl const Tr for () {
    fn a(self) -> i32 { 42 }
}

const fn need_const_closure<T: ~const FnOnce(()) -> i32>(x: T) -> i32 {
    x(())
}

const _: () = assert!(need_const_closure(Tr::a) == 42);

fn main() {}
